use super::*;

fn parse(raw: &str) -> SearchParams {
    parse_search_query(Some(raw)).expect("valid query")
}

fn assert_invalid(raw: &str, field: &str) {
    let error = parse_search_query(Some(raw)).unwrap_err();
    assert_eq!(error.status, axum::http::StatusCode::BAD_REQUEST);
    assert_eq!(error.body.code, "INVALID_QUERY");
    assert_eq!(error.body.details[0].field, field);
}

#[test]
fn absent_and_empty_conditions_are_unrestricted() {
    for raw in [
        None,
        Some(""),
        Some("q=&ingredients=&dietary_tags="),
        Some("q=+++"),
    ] {
        let params = parse_search_query(raw).unwrap();
        assert_eq!(params.q, "");
        assert!(params.ingredients.is_empty());
        assert!(params.dietary_tags.is_empty());
        assert_eq!(params.sort, Sort::Relevance);
        params.ensure_supported(false).unwrap();
    }
}

#[test]
fn normalization_keeps_the_query_as_one_phrase() {
    assert_eq!(parse("q=　ＴＯＭＡＴＯ++Soup　").q, "tomato soup");
}

#[test]
fn form_decoding_is_strict_and_happens_once() {
    assert_eq!(parse("q=a+b%2Bc").q, "a b+c");
    assert_eq!(parse("q=%2520").q, "%20");
    assert_eq!(parse("q=a%26b%3Dc").q, "a&b=c");
    assert_eq!(parse("q=%E7%95%AA%E8%8C%84").q, "番茄");
    for raw in [
        "q=%",
        "q=%2",
        "q=%GG",
        "q=%FF",
        "q=%C0%AF",
        "q=%ED%A0%80",
        "q=%F4%90%80%80",
        "%FF=x",
        "q=ok&sort=time_asc&dietary_tags=%",
    ] {
        assert_invalid(raw, "query");
    }
}

#[test]
fn unknown_and_case_changed_keys_are_invalid() {
    for (raw, field) in [("Q=x", "Q"), ("extra=1", "extra"), ("=x", "query")] {
        assert_invalid(raw, field);
    }
}

#[test]
fn every_parameter_rejects_duplicates_including_encoded_names() {
    for key in ["q", "ingredients", "sort", "dietary_tags"] {
        assert_invalid(&format!("{key}=&{key}="), key);
    }
    assert_invalid("q=a&%71=b", "q");
}

#[test]
fn q_length_counts_decoded_codepoints_before_normalization() {
    assert_eq!(
        parse(&format!("q={}", "番".repeat(100))).q.chars().count(),
        100
    );
    assert_invalid(&format!("q={}", "番".repeat(101)), "q");
    assert_invalid(&format!("q={}a", " ".repeat(100)), "q");
    // 100 decoded ligatures become 300 letters, but the input meets the limit.
    assert_eq!(parse(&format!("q={}", "ﬃ".repeat(100))).q.len(), 300);
}

#[test]
fn ingredients_split_on_commas_and_unicode_whitespace_then_alias_and_deduplicate() {
    let params = parse("ingredients=土豆，马铃薯,%E3%80%80西红柿+番茄,,ＳＡＬＴ");
    assert_eq!(params.ingredients, vec!["马铃薯", "番茄", "salt"]);
    assert_eq!(normalize_ingredient("　西红柿　"), "番茄");
}

#[test]
fn ingredient_total_length_is_checked_before_removing_empty_tokens() {
    assert!(
        parse(&format!("ingredients={}", ",".repeat(1000)))
            .ingredients
            .is_empty()
    );
    assert_invalid(&format!("ingredients={}", ",".repeat(1001)), "ingredients");
}

#[test]
fn ingredient_token_count_is_checked_before_deduplication() {
    assert_eq!(
        parse(&format!("ingredients={}", vec!["盐"; 20].join(","))).ingredients,
        vec!["盐"]
    );
    assert_invalid(
        &format!("ingredients={}", vec!["盐"; 21].join(",")),
        "ingredients",
    );
}

#[test]
fn each_ingredient_has_a_normalized_codepoint_limit() {
    assert!(parse_search_query(Some(&format!("ingredients={}", "番".repeat(50)))).is_ok());
    assert_invalid(&format!("ingredients={}", "番".repeat(51)), "ingredients");
    // NFKC expands each ligature to three letters.
    assert_invalid(&format!("ingredients={}", "ﬃ".repeat(17)), "ingredients");
}

#[test]
fn sort_is_exact_and_unsupported_sort_values_are_still_valid_inputs() {
    assert_eq!(parse("sort=relevance").sort, Sort::Relevance);
    for raw in ["sort=", "sort=RELEVANCE", "sort=+relevance", "sort=fastest"] {
        assert_invalid(raw, "sort");
    }
    for raw in ["sort=time_asc", "sort=time_desc"] {
        let error = parse(raw).ensure_supported(true).unwrap_err();
        assert_eq!(error.body.code, "UNSUPPORTED_PARAMETER");
        assert_eq!(error.body.details[0].field, "sort");
    }
}

#[test]
fn dietary_tags_trim_remove_empty_and_deduplicate() {
    assert!(parse("dietary_tags=,+,").dietary_tags.is_empty());
    assert_eq!(
        parse("dietary_tags=vegan,+vegan,,keto").dietary_tags,
        vec!["vegan", "keto"]
    );
    assert_eq!(
        parse("dietary_tags=vegetarian,vegan,gluten_free,keto")
            .dietary_tags
            .len(),
        4
    );
    for raw in [
        "dietary_tags=VEGAN",
        "dietary_tags=unknown",
        "dietary_tags=vegan，keto",
    ] {
        assert_invalid(raw, "dietary_tags");
    }
    assert!(
        parse(&format!("dietary_tags={}", " ".repeat(100)))
            .dietary_tags
            .is_empty()
    );
    assert_invalid(&format!("dietary_tags={}", " ".repeat(101)), "dietary_tags");
}

#[test]
fn validation_precedes_support_checks() {
    assert_invalid("sort=time_asc&dietary_tags=unknown", "dietary_tags");
    assert_invalid(&format!("sort=time_desc&q={}", "a".repeat(101)), "q");
    assert_invalid("ingredients=盐&sort=fastest", "sort");
}

#[test]
fn support_gate_rejects_unavailable_features_without_ignoring_them() {
    let params = parse("ingredients=盐");
    let error = params.ensure_supported(false).unwrap_err();
    assert_eq!(error.body.code, "UNSUPPORTED_PARAMETER");
    assert_eq!(error.body.details[0].field, "ingredients");
    params.ensure_supported(true).unwrap();

    let error = parse("dietary_tags=vegan")
        .ensure_supported(true)
        .unwrap_err();
    assert_eq!(error.body.code, "UNSUPPORTED_PARAMETER");
    assert_eq!(error.body.details[0].field, "dietary_tags");
}
