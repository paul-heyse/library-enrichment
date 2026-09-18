# `datafusion_spark::function::string::length::SparkLengthFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.length.SparkLengthFunc.json).

<a id="op-f1dcfec475c4f25d1dcc99e6"></a>
## SparkLengthFunc

`struct` · `datafusion_spark::function::string::length::SparkLengthFunc` · datafusion-spark 55.1.0

```rust
struct SparkLengthFunc
```

Source: `src/function/string/length.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `length` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#length>

<a id="op-5c78c79d66183af177db0c64"></a>
## aliases

`function` · `datafusion_spark::function::string::length::SparkLengthFunc::aliases` · datafusion-spark 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::length::SparkLengthFunc", "path": "SparkLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [102, 2], "filename": "src/function/string/length.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/length.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-803c9fa14548f3c8b10b7052"></a>
## default

`function` · `datafusion_spark::function::string::length::SparkLengthFunc::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::length::SparkLengthFunc", "path": "SparkLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [42, 2], "filename": "src/function/string/length.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/length.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87ff365aaad4b9948dc265e1"></a>
## eq

`function` · `datafusion_spark::function::string::length::SparkLengthFunc::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkLengthFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::length::SparkLengthFunc", "path": "SparkLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 26], "filename": "src/function/string/length.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/length.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7511eedcec4da7eb4c75126"></a>
## fmt

`function` · `datafusion_spark::function::string::length::SparkLengthFunc::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::length::SparkLengthFunc", "path": "SparkLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/function/string/length.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/length.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b31f44189a0e14d72b8309f"></a>
## hash

`function` · `datafusion_spark::function::string::length::SparkLengthFunc::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::length::SparkLengthFunc", "path": "SparkLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 32], "end": [32, 36], "filename": "src/function/string/length.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/length.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74aa592e0ee0731321bc9149"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::length::SparkLengthFunc::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> datafusion_common::Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::length::SparkLengthFunc", "path": "SparkLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [102, 2], "filename": "src/function/string/length.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/length.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58e4c5d178f5fc096f498cf7"></a>
## name

`function` · `datafusion_spark::function::string::length::SparkLengthFunc::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::length::SparkLengthFunc", "path": "SparkLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [102, 2], "filename": "src/function/string/length.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/length.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16c2856f9b4dbeef7c614947"></a>
## new

`function` · `datafusion_spark::function::string::length::SparkLengthFunc::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::length::SparkLengthFunc", "path": "SparkLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [66, 2], "filename": "src/function/string/length.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/length.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0d75dc807554b503f87f9f2"></a>
## return_field_from_args

`function` · `datafusion_spark::function::string::length::SparkLengthFunc::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> datafusion_common::Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::length::SparkLengthFunc", "path": "SparkLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [102, 2], "filename": "src/function/string/length.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/length.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cdd82e35c7bb426e9b3fb0e"></a>
## return_type

`function` · `datafusion_spark::function::string::length::SparkLengthFunc::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _args: &[DataType]) -> datafusion_common::Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::length::SparkLengthFunc", "path": "SparkLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [102, 2], "filename": "src/function/string/length.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/length.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcf1cc5133e35ee60718770e"></a>
## signature

`function` · `datafusion_spark::function::string::length::SparkLengthFunc::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::length::SparkLengthFunc", "path": "SparkLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [102, 2], "filename": "src/function/string/length.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/length.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
