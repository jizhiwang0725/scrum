use crate::error::ApiError;
use serde::Serialize;
use std::collections::BTreeMap;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Sort {
    Relevance,
    TimeAsc,
    TimeDesc,
}

/// Validated and normalized query. Empty q / ingredients mean no restriction.
#[derive(Debug, Serialize)]
pub struct SearchParams {
    pub q: String,
    pub ingredients: Vec<String>,
    pub sort: Sort,
    pub dietary_tags: Vec<String>,
}

/// Shared with #28: apply this to recipe titles before matching.
pub fn normalize_text(value: &str) -> String {
    value
        .nfkc()
        .collect::<String>()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Shared with #28: apply this to ingredient names, keeping original data for responses.
pub fn normalize_ingredient(value: &str) -> String {
    let value = normalize_text(value);
    match value.as_str() {
        "土豆" => "马铃薯".to_owned(),
        "西红柿" => "番茄".to_owned(),
        _ => value,
    }
}

fn check_length(value: &str, maximum: usize, field: &str) -> Result<(), ApiError> {
    if value.chars().count() > maximum {
        return Err(ApiError::invalid(
            field,
            &format!("最多允许 {maximum} 个 Unicode 码点。"),
        ));
    }
    Ok(())
}

fn hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

/// Strict form decoding: reject bad escapes / UTF-8, decode once, '+' means space.
/// form_urlencoded's permissive replacement behavior is unsuitable for this contract.
fn decode_component(raw: &str) -> Result<String, ApiError> {
    let invalid = || ApiError::invalid("query", "URL 编码或 UTF-8 不合法。");
    let bytes = raw.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            b'%' => {
                if index + 2 >= bytes.len() {
                    return Err(invalid());
                }
                let high = hex(bytes[index + 1]).ok_or_else(invalid)?;
                let low = hex(bytes[index + 2]).ok_or_else(invalid)?;
                decoded.push(high * 16 + low);
                index += 3;
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }

    String::from_utf8(decoded).map_err(|_| invalid())
}

/// Parse and validate every parameter before checking whether features are supported.
pub fn parse_search_query(raw: Option<&str>) -> Result<SearchParams, ApiError> {
    // Decode everything first, so encoding failures always use field "query".
    let mut pairs = Vec::new();
    for pair in raw.unwrap_or("").split('&').filter(|pair| !pair.is_empty()) {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        pairs.push((decode_component(key)?, decode_component(value)?));
    }

    let mut fields = BTreeMap::new();
    for (key, value) in pairs {
        if !matches!(key.as_str(), "q" | "ingredients" | "sort" | "dietary_tags") {
            let field = if key.is_empty() { "query" } else { &key };
            return Err(ApiError::invalid(field, "未知查询参数。"));
        }
        if fields.insert(key.clone(), value).is_some() {
            return Err(ApiError::invalid(&key, "参数不能重复出现。"));
        }
    }

    let q = fields.get("q").map(String::as_str).unwrap_or("");
    // The contract limits the decoded value BEFORE normalization.
    check_length(q, 100, "q")?;
    let q = normalize_text(q);

    let raw_ingredients = fields.get("ingredients").map(String::as_str).unwrap_or("");
    check_length(raw_ingredients, 1000, "ingredients")?;
    let normalized_list: String = raw_ingredients.nfkc().collect();
    let parts: Vec<_> = normalized_list
        .split(|c: char| c == ',' || c == '，' || c.is_whitespace())
        .filter(|part| !part.is_empty())
        .collect();

    // Count nonempty tokens BEFORE alias substitution and deduplication.
    if parts.len() > 20 {
        return Err(ApiError::invalid(
            "ingredients",
            "去掉空项后最多允许 20 项。",
        ));
    }

    let mut ingredients = Vec::new();
    for part in parts {
        let normalized = normalize_text(part);
        check_length(&normalized, 50, "ingredients")?;
        let name = normalize_ingredient(part);
        if !ingredients.contains(&name) {
            ingredients.push(name);
        }
    }

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

    let tags = fields.get("dietary_tags").map(String::as_str).unwrap_or("");
    check_length(tags, 100, "dietary_tags")?;
    let mut dietary_tags = Vec::new();

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
    /// Call only AFTER parse_search_query has validated the whole request.
    /// A/#28 must pass true only when ingredient filtering is actually connected.
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
        if !ingredient_search && !self.ingredients.is_empty() {
            return Err(ApiError::unsupported("ingredients", "食材搜索尚未接入。"));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "query_tests.rs"]
mod tests;
