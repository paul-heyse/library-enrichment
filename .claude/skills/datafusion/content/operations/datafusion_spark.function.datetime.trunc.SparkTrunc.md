# `datafusion_spark::function::datetime::trunc::SparkTrunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.trunc.SparkTrunc.json).

<a id="op-f6bb66e7acec30eff6f7d1a1"></a>
## SparkTrunc

`struct` · `datafusion_spark::function::datetime::trunc::SparkTrunc` · datafusion-spark 55.1.0

```rust
struct SparkTrunc
```

Source: `src/function/datetime/trunc.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark trunc supports date inputs only and extra format aliases.
Also spark trunc's argument order is (date, format).
<https://spark.apache.org/docs/latest/api/sql/index.html#trunc>

<a id="op-568d34117a10897cc00120ac"></a>
## default

`function` · `datafusion_spark::function::datetime::trunc::SparkTrunc::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::trunc::SparkTrunc", "path": "SparkTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [43, 2], "filename": "src/function/datetime/trunc.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/trunc.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6a33ddc4321989204f2d75b"></a>
## eq

`function` · `datafusion_spark::function::datetime::trunc::SparkTrunc::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkTrunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::trunc::SparkTrunc", "path": "SparkTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 26], "filename": "src/function/datetime/trunc.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/trunc.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1f78b5f8fa54fd1fe6b5b66"></a>
## fmt

`function` · `datafusion_spark::function::datetime::trunc::SparkTrunc::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::trunc::SparkTrunc", "path": "SparkTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/function/datetime/trunc.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/trunc.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9d91fca41e50b25bfe91fd8"></a>
## hash

`function` · `datafusion_spark::function::datetime::trunc::SparkTrunc::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::trunc::SparkTrunc", "path": "SparkTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 32], "end": [34, 36], "filename": "src/function/datetime/trunc.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/trunc.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4e05a28cea068db2548e334"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::trunc::SparkTrunc::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::trunc::SparkTrunc", "path": "SparkTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [138, 2], "filename": "src/function/datetime/trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/trunc.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b863b000f8d038efa4c65759"></a>
## name

`function` · `datafusion_spark::function::datetime::trunc::SparkTrunc::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::trunc::SparkTrunc", "path": "SparkTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [138, 2], "filename": "src/function/datetime/trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/trunc.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04eeb150416aa5bdbfc1b034"></a>
## new

`function` · `datafusion_spark::function::datetime::trunc::SparkTrunc::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::trunc::SparkTrunc", "path": "SparkTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [61, 2], "filename": "src/function/datetime/trunc.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/trunc.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-730c4f0913c1a60a4c6391b2"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::trunc::SparkTrunc::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::trunc::SparkTrunc", "path": "SparkTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [138, 2], "filename": "src/function/datetime/trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/trunc.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3204ba1688c8fbe13806c36a"></a>
## return_type

`function` · `datafusion_spark::function::datetime::trunc::SparkTrunc::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::trunc::SparkTrunc", "path": "SparkTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [138, 2], "filename": "src/function/datetime/trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/trunc.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2c4167903942d14dedf1752"></a>
## signature

`function` · `datafusion_spark::function::datetime::trunc::SparkTrunc::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::trunc::SparkTrunc", "path": "SparkTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [138, 2], "filename": "src/function/datetime/trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/trunc.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-288c66dddaa64746eb6d40d4"></a>
## simplify

`function` · `datafusion_spark::function::datetime::trunc::SparkTrunc::simplify` · datafusion-spark 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::trunc::SparkTrunc", "path": "SparkTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [138, 2], "filename": "src/function/datetime/trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/trunc.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
