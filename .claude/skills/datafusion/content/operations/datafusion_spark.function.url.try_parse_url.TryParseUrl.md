# `datafusion_spark::function::url::try_parse_url::TryParseUrl`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.url.try_parse_url.TryParseUrl.json).

<a id="op-7bd0948a762a807c45fcfb4c"></a>
## TryParseUrl

`struct` · `datafusion_spark::function::url::try_parse_url::TryParseUrl` · datafusion-spark 55.1.0

```rust
struct TryParseUrl
```

Source: `src/function/url/try_parse_url.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

TRY_PARSE_URL function for tolerant URL component extraction (never errors; returns NULL on invalid or missing parts).
<https://spark.apache.org/docs/latest/api/sql/index.html#try_parse_url>

<a id="op-b372eb8ad5e28373676d2cca"></a>
## default

`function` · `datafusion_spark::function::url::try_parse_url::TryParseUrl::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::url::try_parse_url::TryParseUrl", "path": "TryParseUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [39, 2], "filename": "src/function/url/try_parse_url.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/url/try_parse_url.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-259712faf64bb11243a5b6c3"></a>
## eq

`function` · `datafusion_spark::function::url::try_parse_url::TryParseUrl::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &TryParseUrl) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::url::try_parse_url::TryParseUrl", "path": "TryParseUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 17], "end": [30, 26], "filename": "src/function/url/try_parse_url.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/url/try_parse_url.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-510ea00576743686107353b9"></a>
## fmt

`function` · `datafusion_spark::function::url::try_parse_url::TryParseUrl::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::url::try_parse_url::TryParseUrl", "path": "TryParseUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/function/url/try_parse_url.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/url/try_parse_url.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d916786d2846f506cbb941e"></a>
## hash

`function` · `datafusion_spark::function::url::try_parse_url::TryParseUrl::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::url::try_parse_url::TryParseUrl", "path": "TryParseUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 32], "end": [30, 36], "filename": "src/function/url/try_parse_url.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/url/try_parse_url.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e108a4e57d7f14cfe328bd0"></a>
## invoke_with_args

`function` · `datafusion_spark::function::url::try_parse_url::TryParseUrl::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::url::try_parse_url::TryParseUrl", "path": "TryParseUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [70, 2], "filename": "src/function/url/try_parse_url.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/url/try_parse_url.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed01f42648e7c510ebaeca38"></a>
## name

`function` · `datafusion_spark::function::url::try_parse_url::TryParseUrl::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::url::try_parse_url::TryParseUrl", "path": "TryParseUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [70, 2], "filename": "src/function/url/try_parse_url.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/url/try_parse_url.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21dd6b19e194a1dab087bb79"></a>
## new

`function` · `datafusion_spark::function::url::try_parse_url::TryParseUrl::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::url::try_parse_url::TryParseUrl", "path": "TryParseUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [50, 2], "filename": "src/function/url/try_parse_url.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/url/try_parse_url.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ab468cf3d86763fe94081f0"></a>
## return_type

`function` · `datafusion_spark::function::url::try_parse_url::TryParseUrl::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::url::try_parse_url::TryParseUrl", "path": "TryParseUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [70, 2], "filename": "src/function/url/try_parse_url.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/url/try_parse_url.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-877f3f4f14262e0f6286549a"></a>
## signature

`function` · `datafusion_spark::function::url::try_parse_url::TryParseUrl::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::url::try_parse_url::TryParseUrl", "path": "TryParseUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [70, 2], "filename": "src/function/url/try_parse_url.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/url/try_parse_url.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
