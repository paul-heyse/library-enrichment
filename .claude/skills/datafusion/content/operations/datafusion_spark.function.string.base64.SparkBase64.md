# `datafusion_spark::function::string::base64::SparkBase64`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.base64.SparkBase64.json).

<a id="op-bad4d5e6301273a7f904e22d"></a>
## SparkBase64

`struct` · `datafusion_spark::function::string::base64::SparkBase64` · datafusion-spark 55.1.0

```rust
struct SparkBase64
```

Source: `src/function/string/base64.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Apache Spark base64 uses padded base64 encoding.
<https://spark.apache.org/docs/latest/api/sql/index.html#base64>

<a id="op-ea138bf9fab577e9a06f1f9d"></a>
## default

`function` · `datafusion_spark::function::string::base64::SparkBase64::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkBase64", "path": "SparkBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [43, 2], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/base64.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-baafb72053dc778b2f4bc5f9"></a>
## eq

`function` · `datafusion_spark::function::string::base64::SparkBase64::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkBase64) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkBase64", "path": "SparkBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 26], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/base64.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0957810954142cce8a3280a"></a>
## fmt

`function` · `datafusion_spark::function::string::base64::SparkBase64::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkBase64", "path": "SparkBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/base64.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8f5bb06f03e8127552dcf9c"></a>
## hash

`function` · `datafusion_spark::function::string::base64::SparkBase64::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkBase64", "path": "SparkBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 32], "end": [34, 36], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/base64.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1802045bff57f55a84ca4287"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::base64::SparkBase64::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkBase64", "path": "SparkBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [104, 2], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/base64.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d81cdf309d520b0510d15ec8"></a>
## name

`function` · `datafusion_spark::function::string::base64::SparkBase64::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkBase64", "path": "SparkBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [104, 2], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/base64.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2317256c5695eb473c923dbe"></a>
## new

`function` · `datafusion_spark::function::string::base64::SparkBase64::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkBase64", "path": "SparkBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [58, 2], "filename": "src/function/string/base64.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/base64.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8279956c2a7253198162fa59"></a>
## return_field_from_args

`function` · `datafusion_spark::function::string::base64::SparkBase64::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkBase64", "path": "SparkBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [104, 2], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/base64.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abed21419dd08d0c4613dd27"></a>
## return_type

`function` · `datafusion_spark::function::string::base64::SparkBase64::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkBase64", "path": "SparkBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [104, 2], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/base64.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8a7cf2b5f37e1a7fea6be65"></a>
## signature

`function` · `datafusion_spark::function::string::base64::SparkBase64::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkBase64", "path": "SparkBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [104, 2], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/base64.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25532963fc8d178017e9d4cd"></a>
## simplify

`function` · `datafusion_spark::function::string::base64::SparkBase64::simplify` · datafusion-spark 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, _info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkBase64", "path": "SparkBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [104, 2], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/base64.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
