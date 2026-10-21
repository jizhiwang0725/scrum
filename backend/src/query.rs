//! Strict query parsing and normalization called by app::echo_query.
use crate::error::ApiError;
use serde::Serialize;
use std::collections::BTreeMap;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
// Serialize variants as snake_case, for example TimeAsc becomes "time_asc".
// Input values are matched exactly: do not trim or lowercase the sort parameter.
pub enum Sort {
    Relevance,
    TimeAsc,
    TimeDesc,
}

#[derive(Debug, Serialize)]
// Validated parameters returned by parse_search_query.
// An empty q or ingredients list means no restriction; no default recipe name is inserted.
pub struct SearchParams {
    pub q: String,
    pub ingredients: Vec<String>,
    pub sort: Sort,
    pub dietary_tags: Vec<String>,
}

// Apply NFKC, lowercase, trim and collapse Unicode whitespace to single spaces.
// Keep the normalized q as one phrase. Recipe-title matching should use this same helper.
pub fn normalize_text(value: &str) -> String {
    value
        .nfkc()
        .collect::<String>()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

// Normalize a name with normalize_text, then replace the documented aliases.
// Ingredient matching should use this helper on dataset names and preserve original response text.
pub fn normalize_ingredient(value: &str) -> String {
    let value = normalize_text(value);
    match value.as_str() {
        "土豆" => "马铃薯".to_owned(),
        "西红柿" => "番茄".to_owned(),
        _ => value,
    }
}

// Count Unicode scalar values with chars().count(), not UTF-8 bytes.
// This matches codepoint counting for valid UTF-8, not the number of displayed emoji.
fn check_length(value: &str, maximum: usize, field: &str) -> Result<(), ApiError> {
    if value.chars().count() > maximum {
        return Err(ApiError::invalid(
            field,
            &format!("最多允许 {maximum} 个 Unicode 码点。"),
        ));
    }
    Ok(())
}

// Decode one hexadecimal digit into 0..15; None indicates an invalid percent escape.
fn hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

// Strictly decode one form-query key or value without replacing invalid bytes.
// Scan original bytes once; a decoded percent sign is not processed a second time.
fn decode_component(raw: &str) -> Result<String, ApiError> {
    let invalid = || ApiError::invalid("query", "URL 编码或 UTF-8 不合法。");
    let bytes = raw.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            // An original plus sign represents a space. A literal plus must be encoded as %2B.
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            // Require two hexadecimal digits after each percent sign.
            b'%' => {
                if index + 2 >= bytes.len() {
                    return Err(invalid());
                }
                let high = hex(bytes[index + 1]).ok_or_else(invalid)?;
                let low = hex(bytes[index + 2]).ok_or_else(invalid)?;
                // Combine the two hexadecimal digits into one decoded byte.
                decoded.push(high * 16 + low);
                index += 3;
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }

    // Reject invalid UTF-8 after percent decoding instead of inserting replacement characters.
    String::from_utf8(decoded).map_err(|_| invalid())
}

// Parse RawQuery input supplied by app::echo_query into SearchParams.
// Validate all fields here; call SearchParams::ensure_supported only after this succeeds.
pub fn parse_search_query(raw: Option<&str>) -> Result<SearchParams, ApiError> {
    // Split raw pairs on & and the first = before decoding their keys and values.
    // Encoded separators such as %26 and %3D therefore remain part of a value.
    let mut pairs = Vec::new();
    for pair in raw.unwrap_or("").split('&').filter(|pair| !pair.is_empty()) {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        pairs.push((decode_component(key)?, decode_component(value)?));
    }

    // Check decoded parameter names and reject duplicates, including q together with %71.
    let mut fields = BTreeMap::new();
    for (key, value) in pairs {
        if !matches!(key.as_str(), "q" | "ingredients" | "sort" | "dietary_tags") {
            let field = if key.is_empty() { "query" } else { &key };
            return Err(ApiError::invalid(field, "未知查询参数。"));
        }
        // insert returns the previous value if this decoded key has already appeared.
        if fields.insert(key.clone(), value).is_some() {
            return Err(ApiError::invalid(&key, "参数不能重复出现。"));
        }
    }

    let q = fields.get("q").map(String::as_str).unwrap_or("");
    // Enforce the 100-codepoint limit on the decoded input BEFORE normalization or trimming.
    check_length(q, 100, "q")?;
    let q = normalize_text(q);

    let raw_ingredients = fields.get("ingredients").map(String::as_str).unwrap_or("");
    // Check the total decoded ingredient input length before removing empty entries.
    check_length(raw_ingredients, 1000, "ingredients")?;
    // Apply NFKC before splitting on ASCII commas, Chinese commas or Unicode whitespace.
    let normalized_list: String = raw_ingredients.nfkc().collect();
    let parts: Vec<_> = normalized_list
        .split(|c: char| c == ',' || c == '，' || c.is_whitespace())
        .filter(|part| !part.is_empty())
        .collect();

    // Count nonempty tokens before alias replacement and deduplication.
    // Twenty-one repetitions of the same ingredient still exceed the input limit.
    if parts.len() > 20 {
        return Err(ApiError::invalid(
            "ingredients",
            "去掉空项后最多允许 20 项。",
        ));
    }

    // Check each normalized item, map aliases and deduplicate while preserving first occurrence.
    // Matching must count each canonical ingredient only once.
    let mut ingredients = Vec::new();
    for part in parts {
        let normalized = normalize_text(part);
        check_length(&normalized, 50, "ingredients")?;
        let name = normalize_ingredient(part);
        if !ingredients.contains(&name) {
            ingredients.push(name);
        }
    }

    // Missing sort defaults to relevance; an empty value is invalid.
    // Parse valid time-sort values here and defer availability checks until all fields are validated.
    let sort = match fields.get("sort").map(String::as_str) {
        None | Some("relevance") => Sort::Relevance,
        Some("time_asc") => Sort::TimeAsc,
        Some("time_desc") => Sort::TimeDesc,
        _ => {
            return Err(ApiError::invalid(
                "sort",
                "必须为 relevance、time_asc 或 time_desc。",
            ));
        }
    };

    // Tags use ASCII comma splitting; do not apply NFKC or lowercase their enum values.
    let tags = fields.get("dietary_tags").map(String::as_str).unwrap_or("");
    check_length(tags, 100, "dietary_tags")?;
    let mut dietary_tags = Vec::new();

    // Trim each tag, discard empty entries, validate the enum and deduplicate.
    for tag in tags.split(',').map(str::trim).filter(|tag| !tag.is_empty()) {
        if !matches!(tag, "vegetarian" | "vegan" | "gluten_free" | "keto") {
            return Err(ApiError::invalid(
                "dietary_tags",
                "必须为 vegetarian、vegan、gluten_free 或 keto。",
            ));
        }
        let tag = tag.to_owned();
        if !dietary_tags.contains(&tag) {
            dietary_tags.push(tag);
        }
    }

    if dietary_tags.len() > 4 {
        return Err(ApiError::invalid("dietary_tags", "最多允许 4 项。"));
    }

    Ok(SearchParams {
        q,
        ingredients,
        sort,
        dietary_tags,
    })
}

impl SearchParams {
    // app::echo_query calls this after parse_search_query validates the entire request.
    // Invalid input must take precedence over unavailable features in the same request.
    pub fn ensure_supported(&self, ingredient_search: bool) -> Result<(), ApiError> {
        if self.sort != Sort::Relevance {
            return Err(ApiError::unsupported(
                "sort",
                "尚未提供耗时数据，暂不支持耗时排序。",
            ));
        }
        if !self.dietary_tags.is_empty() {
            return Err(ApiError::unsupported(
                "dietary_tags",
                "尚未提供饮食标签数据。",
            ));
        }
        // Only pass true when the calling handler actually applies ingredient filtering.
        if !ingredient_search && !self.ingredients.is_empty() {
            return Err(ApiError::unsupported("ingredients", "食材搜索尚未接入。"));
        }
        Ok(())
    }
}

// Compile query_tests.rs to check decoding, normalization, limits and validation order.
#[cfg(test)]
#[path = "query_tests.rs"]
mod tests;
