# `datafusion_spark::function::string::format_string::FormatStringFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.format_string.FormatStringFunc.json).

<a id="op-1e44c88b3f1f5f15043d2c16"></a>
## FormatStringFunc

`struct` · `datafusion_spark::function::string::format_string::FormatStringFunc` · datafusion-spark 55.1.0

```rust
struct FormatStringFunc
```

Source: `src/function/string/format_string.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `format_string` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#format_string>

<a id="op-2a648c0190917c6596a077a9"></a>
## aliases

`function` · `datafusion_spark::function::string::format_string::FormatStringFunc::aliases` · datafusion-spark 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::FormatStringFunc", "path": "FormatStringFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [192, 2], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/format_string.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edc65d43bb4c97804dac4f9e"></a>
## default

`function` · `datafusion_spark::function::string::format_string::FormatStringFunc::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::FormatStringFunc", "path": "FormatStringFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [52, 2], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/format_string.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c80a248ff9e2a8116e06c6ee"></a>
## eq

`function` · `datafusion_spark::function::string::format_string::FormatStringFunc::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &FormatStringFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::FormatStringFunc", "path": "FormatStringFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 17], "end": [42, 26], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/format_string.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a39736f5c088a4d060998d66"></a>
## fmt

`function` · `datafusion_spark::function::string::format_string::FormatStringFunc::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::FormatStringFunc", "path": "FormatStringFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 10], "end": [42, 15], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/format_string.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2372dee227f963adf064034f"></a>
## hash

`function` · `datafusion_spark::function::string::format_string::FormatStringFunc::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::FormatStringFunc", "path": "FormatStringFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 32], "end": [42, 36], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/format_string.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-994ec2a0069b9b199f2bd99c"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::format_string::FormatStringFunc::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::FormatStringFunc", "path": "FormatStringFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [192, 2], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/format_string.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ddfa362aa01d21713b22423"></a>
## name

`function` · `datafusion_spark::function::string::format_string::FormatStringFunc::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::FormatStringFunc", "path": "FormatStringFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [192, 2], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/format_string.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5930afb92ab8a4113a13702"></a>
## new

`function` · `datafusion_spark::function::string::format_string::FormatStringFunc::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::FormatStringFunc", "path": "FormatStringFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [61, 2], "filename": "src/function/string/format_string.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/format_string.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c60f4b72c58fd219bd7a00b"></a>
## return_field_from_args

`function` · `datafusion_spark::function::string::format_string::FormatStringFunc::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::FormatStringFunc", "path": "FormatStringFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [192, 2], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/format_string.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a439ecf878ccf769dee50b50"></a>
## return_type

`function` · `datafusion_spark::function::string::format_string::FormatStringFunc::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::FormatStringFunc", "path": "FormatStringFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [192, 2], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/format_string.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06c64ad760335ca70d4600c1"></a>
## signature

`function` · `datafusion_spark::function::string::format_string::FormatStringFunc::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::FormatStringFunc", "path": "FormatStringFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [192, 2], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/format_string.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
