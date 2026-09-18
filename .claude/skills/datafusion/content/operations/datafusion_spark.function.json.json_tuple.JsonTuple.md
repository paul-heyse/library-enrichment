# `datafusion_spark::function::json::json_tuple::JsonTuple`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.json.json_tuple.JsonTuple.json).

<a id="op-527c53a8910f05ae27502348"></a>
## JsonTuple

`struct` · `datafusion_spark::function::json::json_tuple::JsonTuple` · datafusion-spark 55.1.0

```rust
struct JsonTuple
```

Source: `src/function/json/json_tuple.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `json_tuple` expression

<https://spark.apache.org/docs/latest/api/sql/index.html#json_tuple>

Extracts top-level fields from a JSON string and returns them as a struct.

`json_tuple(json_string, field1, field2, ...) -> Struct<c0: Utf8, c1: Utf8, ...>`

Note: In Spark, `json_tuple` is a Generator that produces multiple columns directly.
In DataFusion, a ScalarUDF can only return one value per row, so the result is wrapped
in a Struct. The caller (e.g. Comet) is expected to destructure the struct fields.

- Returns NULL for each field that is missing from the JSON object
- Returns NULL for all fields if the input is NULL or not valid JSON
- Non-string JSON values are converted to their JSON string representation
- JSON `null` values are returned as NULL (not the string "null")

<a id="op-98563ca415aa20b05d62cce7"></a>
## default

`function` · `datafusion_spark::function::json::json_tuple::JsonTuple::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::json::json_tuple::JsonTuple", "path": "JsonTuple"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [54, 2], "filename": "src/function/json/json_tuple.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/json/json_tuple.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54609985810b6eab45dcc785"></a>
## eq

`function` · `datafusion_spark::function::json::json_tuple::JsonTuple::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &JsonTuple) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::json::json_tuple::JsonTuple", "path": "JsonTuple"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 17], "end": [45, 26], "filename": "src/function/json/json_tuple.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/json/json_tuple.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9517b165f5fe9f484c3cbab6"></a>
## fmt

`function` · `datafusion_spark::function::json::json_tuple::JsonTuple::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::json::json_tuple::JsonTuple", "path": "JsonTuple"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/function/json/json_tuple.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/json/json_tuple.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f451ff45d9e989674e26ee97"></a>
## hash

`function` · `datafusion_spark::function::json::json_tuple::JsonTuple::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::json::json_tuple::JsonTuple", "path": "JsonTuple"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 32], "end": [45, 36], "filename": "src/function/json/json_tuple.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/json/json_tuple.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fec4876a3a784e704d334bf"></a>
## invoke_with_args

`function` · `datafusion_spark::function::json::json_tuple::JsonTuple::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::json::json_tuple::JsonTuple", "path": "JsonTuple"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [109, 2], "filename": "src/function/json/json_tuple.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/json/json_tuple.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47f9df44e510b98191206669"></a>
## name

`function` · `datafusion_spark::function::json::json_tuple::JsonTuple::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::json::json_tuple::JsonTuple", "path": "JsonTuple"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [109, 2], "filename": "src/function/json/json_tuple.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/json/json_tuple.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1076dafaacae8b5daa9b093"></a>
## new

`function` · `datafusion_spark::function::json::json_tuple::JsonTuple::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::json::json_tuple::JsonTuple", "path": "JsonTuple"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [62, 2], "filename": "src/function/json/json_tuple.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/json/json_tuple.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-133ad0f098ffe2cfbed8c13e"></a>
## return_field_from_args

`function` · `datafusion_spark::function::json::json_tuple::JsonTuple::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::json::json_tuple::JsonTuple", "path": "JsonTuple"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [109, 2], "filename": "src/function/json/json_tuple.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/json/json_tuple.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7dcf0d557b55274cf9d843ec"></a>
## return_type

`function` · `datafusion_spark::function::json::json_tuple::JsonTuple::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::json::json_tuple::JsonTuple", "path": "JsonTuple"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [109, 2], "filename": "src/function/json/json_tuple.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/json/json_tuple.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c58a1127d75fa52b3d80bf60"></a>
## signature

`function` · `datafusion_spark::function::json::json_tuple::JsonTuple::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::json::json_tuple::JsonTuple", "path": "JsonTuple"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [109, 2], "filename": "src/function/json/json_tuple.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/json/json_tuple.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
