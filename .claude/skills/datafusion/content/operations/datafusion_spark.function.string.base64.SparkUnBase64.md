# `datafusion_spark::function::string::base64::SparkUnBase64`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.base64.SparkUnBase64.json).

<a id="op-b335fd81f13826953831b639"></a>
## SparkUnBase64

`struct` · `datafusion_spark::function::string::base64::SparkUnBase64` · datafusion-spark 55.1.0

```rust
struct SparkUnBase64
```

Source: `src/function/string/base64.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

<https://spark.apache.org/docs/latest/api/sql/index.html#unbase64>

<a id="op-1b31ac38bc1af685736725ae"></a>
## default

`function` · `datafusion_spark::function::string::base64::SparkUnBase64::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkUnBase64", "path": "SparkUnBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [116, 2], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/base64.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-677643fb39cd8a043884a8bd"></a>
## eq

`function` · `datafusion_spark::function::string::base64::SparkUnBase64::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkUnBase64) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkUnBase64", "path": "SparkUnBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 17], "end": [107, 26], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/base64.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57c7bc50a05769ef51c507d9"></a>
## fmt

`function` · `datafusion_spark::function::string::base64::SparkUnBase64::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkUnBase64", "path": "SparkUnBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 10], "end": [107, 15], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/base64.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de03d17c445f23faba96a706"></a>
## hash

`function` · `datafusion_spark::function::string::base64::SparkUnBase64::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkUnBase64", "path": "SparkUnBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 32], "end": [107, 36], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/base64.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61ceb1f2436aff33df7ecfe6"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::base64::SparkUnBase64::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkUnBase64", "path": "SparkUnBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [174, 2], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/base64.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb6b1eb19e269e3d60648622"></a>
## name

`function` · `datafusion_spark::function::string::base64::SparkUnBase64::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkUnBase64", "path": "SparkUnBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [174, 2], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/base64.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d7e38efb95a17fea720e43b"></a>
## new

`function` · `datafusion_spark::function::string::base64::SparkUnBase64::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkUnBase64", "path": "SparkUnBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [131, 2], "filename": "src/function/string/base64.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/base64.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a82ac31ea12bfba0c595e099"></a>
## return_field_from_args

`function` · `datafusion_spark::function::string::base64::SparkUnBase64::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkUnBase64", "path": "SparkUnBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [174, 2], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/base64.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eae9945ea2d62fe906ccf5b1"></a>
## return_type

`function` · `datafusion_spark::function::string::base64::SparkUnBase64::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkUnBase64", "path": "SparkUnBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [174, 2], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/base64.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e2e61437361034fce65d168"></a>
## signature

`function` · `datafusion_spark::function::string::base64::SparkUnBase64::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkUnBase64", "path": "SparkUnBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [174, 2], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/base64.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46ff999052e419679f7d1a2d"></a>
## simplify

`function` · `datafusion_spark::function::string::base64::SparkUnBase64::simplify` · datafusion-spark 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, _info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::base64::SparkUnBase64", "path": "SparkUnBase64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [174, 2], "filename": "src/function/string/base64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/base64.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
