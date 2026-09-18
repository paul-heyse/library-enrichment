# `datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.is_valid_utf8.SparkIsValidUtf8.json).

<a id="op-9dc74547f7a0508ad404f6c4"></a>
## SparkIsValidUtf8

`struct` · `datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8` · datafusion-spark 55.1.0

```rust
struct SparkIsValidUtf8
```

Source: `src/function/string/is_valid_utf8.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `is_valid_utf8` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#is_valid_utf8>

<a id="op-20d655a66c147056536deb43"></a>
## default

`function` · `datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8", "path": "SparkIsValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [46, 2], "filename": "src/function/string/is_valid_utf8.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/is_valid_utf8.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72e49bb1cfb23acd5163f99e"></a>
## eq

`function` · `datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkIsValidUtf8) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8", "path": "SparkIsValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 17], "end": [37, 26], "filename": "src/function/string/is_valid_utf8.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/is_valid_utf8.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea1bb9ed34db3c707922e20b"></a>
## fmt

`function` · `datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8", "path": "SparkIsValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "src/function/string/is_valid_utf8.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/is_valid_utf8.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a75e9a64b105582b090508cb"></a>
## hash

`function` · `datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8", "path": "SparkIsValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 32], "end": [37, 36], "filename": "src/function/string/is_valid_utf8.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/is_valid_utf8.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d8c16e929276fb03d67d218"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8", "path": "SparkIsValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [87, 2], "filename": "src/function/string/is_valid_utf8.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/is_valid_utf8.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7d130ee65645792a9e1ffbc"></a>
## name

`function` · `datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8", "path": "SparkIsValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [87, 2], "filename": "src/function/string/is_valid_utf8.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/is_valid_utf8.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a08417091657a9bdb250524a"></a>
## new

`function` · `datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8", "path": "SparkIsValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [65, 2], "filename": "src/function/string/is_valid_utf8.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/is_valid_utf8.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c4c87faa1156b33f12fcf2a"></a>
## return_field_from_args

`function` · `datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, _args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8", "path": "SparkIsValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [87, 2], "filename": "src/function/string/is_valid_utf8.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/is_valid_utf8.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28904147bf1a9d455901da0d"></a>
## return_type

`function` · `datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8", "path": "SparkIsValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [87, 2], "filename": "src/function/string/is_valid_utf8.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/is_valid_utf8.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9659b7a5d99bf1ebf02a317"></a>
## signature

`function` · `datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8", "path": "SparkIsValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [87, 2], "filename": "src/function/string/is_valid_utf8.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/is_valid_utf8.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
