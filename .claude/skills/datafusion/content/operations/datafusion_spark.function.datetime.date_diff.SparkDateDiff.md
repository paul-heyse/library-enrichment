# `datafusion_spark::function::datetime::date_diff::SparkDateDiff`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.date_diff.SparkDateDiff.json).

<a id="op-21e974a59ba876dc7e64c553"></a>
## SparkDateDiff

`struct` · `datafusion_spark::function::datetime::date_diff::SparkDateDiff` · datafusion-spark 55.1.0

```rust
struct SparkDateDiff
```

Source: `src/function/datetime/date_diff.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

<https://spark.apache.org/docs/latest/api/sql/index.html#date_diff>

<a id="op-d5d07dd872445c77f0dd7405"></a>
## aliases

`function` · `datafusion_spark::function::datetime::date_diff::SparkDateDiff::aliases` · datafusion-spark 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_diff::SparkDateDiff", "path": "SparkDateDiff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [114, 2], "filename": "src/function/datetime/date_diff.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_diff.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d175f0a17dbc36691277f38"></a>
## default

`function` · `datafusion_spark::function::datetime::date_diff::SparkDateDiff::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_diff::SparkDateDiff", "path": "SparkDateDiff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [42, 2], "filename": "src/function/datetime/date_diff.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/date_diff.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae292543b27f2e3d5690ce47"></a>
## eq

`function` · `datafusion_spark::function::datetime::date_diff::SparkDateDiff::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkDateDiff) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_diff::SparkDateDiff", "path": "SparkDateDiff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 26], "filename": "src/function/datetime/date_diff.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/date_diff.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f93b9720c71597cd2f7a392"></a>
## fmt

`function` · `datafusion_spark::function::datetime::date_diff::SparkDateDiff::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_diff::SparkDateDiff", "path": "SparkDateDiff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/function/datetime/date_diff.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/date_diff.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-497446e0b64ae0256448ae2e"></a>
## hash

`function` · `datafusion_spark::function::datetime::date_diff::SparkDateDiff::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_diff::SparkDateDiff", "path": "SparkDateDiff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 32], "end": [32, 36], "filename": "src/function/datetime/date_diff.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/date_diff.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83ff52545056eae6c82567c5"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::date_diff::SparkDateDiff::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_diff::SparkDateDiff", "path": "SparkDateDiff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [114, 2], "filename": "src/function/datetime/date_diff.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_diff.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2224c7b47b6ba2c34b47b463"></a>
## name

`function` · `datafusion_spark::function::datetime::date_diff::SparkDateDiff::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_diff::SparkDateDiff", "path": "SparkDateDiff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [114, 2], "filename": "src/function/datetime/date_diff.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_diff.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06c430791cd9d6074c8fdcc8"></a>
## new

`function` · `datafusion_spark::function::datetime::date_diff::SparkDateDiff::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_diff::SparkDateDiff", "path": "SparkDateDiff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [71, 2], "filename": "src/function/datetime/date_diff.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/date_diff.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5072f0d7523d74a51e39e9c"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::date_diff::SparkDateDiff::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_diff::SparkDateDiff", "path": "SparkDateDiff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [114, 2], "filename": "src/function/datetime/date_diff.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_diff.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-574bd58f053f98bf273985a9"></a>
## return_type

`function` · `datafusion_spark::function::datetime::date_diff::SparkDateDiff::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_diff::SparkDateDiff", "path": "SparkDateDiff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [114, 2], "filename": "src/function/datetime/date_diff.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_diff.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acc5e3eb20f497d76b99f7ba"></a>
## signature

`function` · `datafusion_spark::function::datetime::date_diff::SparkDateDiff::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_diff::SparkDateDiff", "path": "SparkDateDiff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [114, 2], "filename": "src/function/datetime/date_diff.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_diff.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12438b138a908ca35ae55150"></a>
## simplify

`function` · `datafusion_spark::function::datetime::date_diff::SparkDateDiff::simplify` · datafusion-spark 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_diff::SparkDateDiff", "path": "SparkDateDiff"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [114, 2], "filename": "src/function/datetime/date_diff.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_diff.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
