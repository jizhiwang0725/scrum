//! 请求参数解析与校验：先验证整个请求，再由调用方检查当前支持的功能。
use crate::error::ApiError;
use serde::Serialize;
use std::collections::BTreeMap;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
// 排序参数枚举；Serialize 使用 snake_case，例如 TimeAsc 输出为 "time_asc"。
// 解析时仍严格区分大小写，不对 sort 去空格或转小写。
pub enum Sort {
    Relevance,
    TimeAsc,
    TimeDesc,
}

/// Validated and normalized query. Empty q / ingredients mean no restriction.
#[derive(Debug, Serialize)]
// 传给后续业务逻辑的参数：q 和 ingredients 已完成标准化。
// 空字符串或空列表表示不限制条件；不设置默认菜名。
pub struct SearchParams {
    pub q: String,
    pub ingredients: Vec<String>,
    pub sort: Sort,
    pub dietary_tags: Vec<String>,
}

/// Shared with #28: apply this to recipe titles before matching.
// 文本标准化顺序：NFKC → 转小写 → 按 Unicode 空白切分 → 用单空格连接。
// 最后两步同时去掉首尾空白并合并连续空白；q 仍作为一个完整词组匹配。
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
// 食材先做相同的文本标准化，再把约定别名转换成统一名称。
// #28 也需要对数据中的食材名称调用此函数；响应仍保留原始菜谱文字。
pub fn normalize_ingredient(value: &str) -> String {
    let value = normalize_text(value);
    match value.as_str() {
        "土豆" => "马铃薯".to_owned(),
        "西红柿" => "番茄".to_owned(),
        _ => value,
    }
}

// chars().count() 统计 Unicode 标量值，对合法 UTF-8 对应码点数量。
// 它不是 UTF-8 字节数，也不等同于用户看到的复杂 emoji 数量。
fn check_length(value: &str, maximum: usize, field: &str) -> Result<(), ApiError> {
    if value.chars().count() > maximum {
        return Err(ApiError::invalid(
            field,
            &format!("最多允许 {maximum} 个 Unicode 码点。"),
        ));
    }
    Ok(())
}

// 将一个十六进制字符转换成 0～15；其他字符返回 None，供 %HH 解码使用。
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
// 手动执行严格的表单 URL 解码，不使用宽松解码器自动替换非法输入。
// 只扫描原始字节一次，解码后得到的百分号不会再次被当作转义处理。
fn decode_component(raw: &str) -> Result<String, ApiError> {
    let invalid = || ApiError::invalid("query", "URL 编码或 UTF-8 不合法。");
    let bytes = raw.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            // 原始加号代表空格；字面加号必须使用 %2B。
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            // 百分号后必须恰好能读取两个十六进制字符，否则返回编码错误。
            b'%' => {
                if index + 2 >= bytes.len() {
                    return Err(invalid());
                }
                let high = hex(bytes[index + 1]).ok_or_else(invalid)?;
                let low = hex(bytes[index + 2]).ok_or_else(invalid)?;
                // 两个十六进制数字合成一个字节；例如 %2B 合成为加号字节。
                decoded.push(high * 16 + low);
                index += 3;
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }

    // 所有转义解码完后再检查 UTF-8；禁止将非法字节替换成替代字符后接受。
    String::from_utf8(decoded).map_err(|_| invalid())
}

/// Parse and validate every parameter before checking whether features are supported.
// 输入是原始查询字符串，可为 None；输出是校验后的参数或 ApiError。
// 本函数只判断参数是否合法，不在解析中提前拒绝尚未支持的合法功能。
pub fn parse_search_query(raw: Option<&str>) -> Result<SearchParams, ApiError> {
    // Decode everything first, so encoding failures always use field "query".
    // 先按原始 & 和第一个 = 切分，再分别解码键和值。
    // 因此值中的 %26、%3D 不会被误当成参数分隔符。
    let mut pairs = Vec::new();
    for pair in raw.unwrap_or("").split('&').filter(|pair| !pair.is_empty()) {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        pairs.push((decode_component(key)?, decode_component(value)?));
    }

    // 按解码后的键检查名称及重复情况；例如 q 与 %71 是同一个参数名。
    let mut fields = BTreeMap::new();
    for (key, value) in pairs {
        if !matches!(key.as_str(), "q" | "ingredients" | "sort" | "dietary_tags") {
            let field = if key.is_empty() { "query" } else { &key };
            return Err(ApiError::invalid(field, "未知查询参数。"));
        }
        // insert 返回旧值；出现旧值说明这个键已经提供过，必须拒绝。
        if fields.insert(key.clone(), value).is_some() {
            return Err(ApiError::invalid(&key, "参数不能重复出现。"));
        }
    }

    let q = fields.get("q").map(String::as_str).unwrap_or("");
    // The contract limits the decoded value BEFORE normalization.
    // q 的 100 码点上限针对解码后的原文，必须在 trim 和 NFKC 之前检查。
    check_length(q, 100, "q")?;
    let q = normalize_text(q);

    let raw_ingredients = fields.get("ingredients").map(String::as_str).unwrap_or("");
    // 食材参数总长度同样先按解码后的原文检查，不能先删除空项再计算。
    check_length(raw_ingredients, 1000, "ingredients")?;
    // 先做 NFKC，再识别英文逗号、中文逗号和 Unicode 空白分隔符。
    let normalized_list: String = raw_ingredients.nfkc().collect();
    let parts: Vec<_> = normalized_list
        .split(|c: char| c == ',' || c == '，' || c.is_whitespace())
        .filter(|part| !part.is_empty())
        .collect();

    // Count nonempty tokens BEFORE alias substitution and deduplication.
    // 20 项限制在去空项后、去重前检查；重复 21 次同一食材也属于超限。
    if parts.len() > 20 {
        return Err(ApiError::invalid(
            "ingredients",
            "去掉空项后最多允许 20 项。",
        ));
    }

    // 依次校验每个食材标准化后的长度，再替换别名并去重。
    // 保留首次出现的顺序；业务匹配时同一食材只计一次。
    let mut ingredients = Vec::new();
    for part in parts {
        let normalized = normalize_text(part);
        check_length(&normalized, 50, "ingredients")?;
        let name = normalize_ingredient(part);
        if !ingredients.contains(&name) {
            ingredients.push(name);
        }
    }

    // 只接受三个精确的枚举值；缺少 sort 使用 relevance，但 sort= 不合法。
    // 此处允许解析 time_asc/time_desc，是否支持留到全部校验结束后判断。
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

    // 饮食标签只按英文逗号切分，不做 NFKC 或大小写转换。
    let tags = fields.get("dietary_tags").map(String::as_str).unwrap_or("");
    check_length(tags, 100, "dietary_tags")?;
    let mut dietary_tags = Vec::new();

    // 去掉标签两端空白及空项，验证枚举，然后去重。
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
    // 仅对已经完成全部校验的 SearchParams 调用此方法。
    // 即使 sort=time_asc 暂不支持，只要另一个参数非法，也应先报 INVALID_QUERY。
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
        // 只有真实食材过滤逻辑已接入时，调用方才应传 true。
        if !ingredient_search && !self.ingredients.is_empty() {
            return Err(ApiError::unsupported("ingredients", "食材搜索尚未接入。"));
        }
        Ok(())
    }
}

// 测试覆盖严格解码、标准化、长度边界、去重和校验顺序。
#[cfg(test)]
#[path = "query_tests.rs"]
mod tests;
