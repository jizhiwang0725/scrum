# Request validation and JSON errors

## Purpose and module responsibilities

This change adds strict request validation to the current Axum backend.
The current state::AppState contains recipe_names, not complete recipe records.

| Module | Responsibility |
| --- | --- |
| main.rs | Initialize state and logging, call app::build_app and http::with_middleware, start the server. |
| app.rs | Register HTTP routes, extract requests, invoke query.rs functions and return responses. |
| query.rs | Decode and validate search parameters, normalize text and check feature availability. |
| error.rs | Build ApiError and convert it into the documented JSON error envelope. |
| state.rs | Define the shared AppState read by app::inspect_state. |
| http.rs | Apply CORS and request logging; convert successful CORS preflights to 204. |
| config.rs | Parse and validate configured frontend origins. |
| query_tests.rs | Test parsing, normalization, limits and feature-support rules. |
| app_tests.rs | Test HTTP responses with the existing middleware installed. |

## Current endpoints

- GET /api/health returns status=ok and rejects nonempty query strings.
- GET /api/debug/query returns a marked debug echo of supported, validated parameters.
  Its response is not the public recipe-list response.
- GET /api/debug/state reports the number of names in state::AppState.
- HEAD uses the corresponding GET status and headers without a response body.
- Known routes accept OPTIONS. Existing CORS configuration and logging remain active.
- Router-level 404 and 405 responses use the JSON error envelope.
- 405 responses include Allow: GET, HEAD, OPTIONS.

The public /api/recipes route is not registered yet. Complete recipe loading,
search, ranking and the RecipeList response still require integration.

## Parameter processing

app::echo_query obtains the original RawQuery and calls:

~~~rust
let params = crate::query::parse_search_query(raw.as_deref())?;
params.ensure_supported(false)?;
~~~

The false argument is intentional: ingredient filtering is not connected to the
current handler. Pass true only when the calling handler actually applies the
validated ingredient conditions.

parse_search_query performs these checks before any availability check:

1. Split raw pairs, then decode each key and value exactly once. A plus means a
   space; %2B means a literal plus. Reject malformed escapes and invalid UTF-8.
2. Reject unknown and duplicate decoded parameter names.
3. Limit decoded q to 100 codepoints before normalization. Missing or empty q
   means no name restriction, not a default recipe name.
4. Apply NFKC, lowercase, trimming and whitespace collapsing. Keep q as one phrase.
5. Limit decoded ingredients to 1000 codepoints. Apply NFKC, split on ASCII or
   Chinese commas and Unicode whitespace, discard empty tokens and allow at most
   20 nonempty tokens before deduplication. Each normalized item has at most 50
   codepoints. Canonicalize the two documented potato/tomato aliases and deduplicate.
6. Accept only the exact sort values relevance, time_asc and time_desc.
   Missing sort defaults to relevance; an empty value is invalid.
7. Limit decoded dietary_tags to 100 codepoints. Split on ASCII commas, trim,
   discard empties, validate the four enum values and deduplicate to at most four tags.

SearchParams::ensure_supported then rejects time sorting and nonempty dietary
tags. It also rejects nonempty ingredient conditions unless ingredient_search
is true. Invalid parameters take precedence over unavailable features.

The parser ignores empty ampersand-separated segments. A bare known key is
treated as an empty value. Health treats an empty raw query as no parameters.

## Matching integration

A future recipe handler should call query::parse_search_query, then
SearchParams::ensure_supported, then the actual filtering and ranking functions.

Normalize dataset titles with query::normalize_text and ingredient names with
query::normalize_ingredient. Preserve original recipe text in response objects.
An empty q or ingredients list means no restriction. Search results must contain
complete recipe records in items, with total equal to items.len().

No recipe-search or recipe-loading module exists in the supplied backend snapshot;
this document does not invent names for those future modules.

## Errors

error::ApiError implements IntoResponse. Parameter errors use HTTP 400 and this
shape:

~~~json
{
  "error": {
    "code": "INVALID_QUERY",
    "message": "Invalid query parameters.",
    "details": [
      {"field": "sort", "message": "Must be relevance, time_asc or time_desc."}
    ]
  }
}
~~~

All 400 responses contain at least one detail. Entire-query decoding errors use
field=query. Router-level 404 and 405 responses use an empty details list.
When data-loading or search handlers are added, their internal failures must map
to a generic 500 INTERNAL_ERROR envelope without exposing internal paths or secrets.

Error messages and explanatory comments are in English. Unicode search fixtures,
alias keys and sample recipe names use Rust Unicode escapes in source code so
their runtime values and test meaning remain unchanged.

## Verification

All 24 automated tests passed locally: 14 query-parser tests and 10 HTTP tests.

Command:
```sh
cargo test --manifest-path backend/Cargo.toml
```

Result: 24 passed, 0 failed.


## Manual checks

Start the team backend on its default port 3000:

~~~sh
cargo run --manifest-path backend/Cargo.toml
~~~

In another terminal:

~~~sh
curl -i 'http://localhost:3000/api/health'
curl -i 'http://localhost:3000/api/health?q=x'
curl -i 'http://localhost:3000/api/debug/query?q='
curl -i 'http://localhost:3000/api/debug/query?q=+TOMATO++Soup+'
curl -i 'http://localhost:3000/api/debug/query?q=a&%71=b'
curl -i 'http://localhost:3000/api/debug/query?q=%FF'
curl -i 'http://localhost:3000/api/debug/query?sort=time_asc'
curl -i 'http://localhost:3000/api/debug/query?sort=time_asc&dietary_tags=unknown'
curl -i -X POST 'http://localhost:3000/api/health'
curl -I 'http://localhost:3000/api/health'
~~~

Expected results: 200; 400 INVALID_QUERY; 200 with empty params.q; 200 with
params.q="tomato soup"; 400 INVALID_QUERY for q; 400 INVALID_QUERY for query;
400 UNSUPPORTED_PARAMETER for sort; 400 INVALID_QUERY for dietary_tags;
405 JSON with Allow; and 200 with no body.

## Existing CORS behavior

http::with_middleware installs CorsLayer, which can handle OPTIONS before route
handlers. http::preflight_no_content converts successful preflights to 204.
Ordinary OPTIONS may return 200 with no body. Unknown-path OPTIONS can also be
intercepted by the existing CORS layer. Public-route integration should decide
whether preflights need to be restricted to known paths.

This change does not claim to finish recipe search, data integration or that
CORS routing policy.
