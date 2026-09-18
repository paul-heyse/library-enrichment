# `datafusion_spark::function::datetime::date_trunc::SparkDateTrunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.date_trunc.SparkDateTrunc.json).

<a id="op-caf2e1ed1b8b4fd203fd26f1"></a>
## SparkDateTrunc

`struct` · `datafusion_spark::function::datetime::date_trunc::SparkDateTrunc` · datafusion-spark 55.1.0

```rust
struct SparkDateTrunc
```

Source: `src/function/datetime/date_trunc.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark date_trunc supports extra format aliases.
It also handles timestamps with timezones by converting to session timezone first.
<https://spark.apache.org/docs/latest/api/sql/index.html#date_trunc>

<a id="op-a04b5a8ed04e6991f66c9986"></a>
## default

`function` · `datafusion_spark::function::datetime::date_trunc::SparkDateTrunc::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_trunc::SparkDateTrunc", "path": "SparkDateTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [43, 2], "filename": "src/function/datetime/date_trunc.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/date_trunc.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a8128de863a09826addb024"></a>
## eq

`function` · `datafusion_spark::function::datetime::date_trunc::SparkDateTrunc::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkDateTrunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_trunc::SparkDateTrunc", "path": "SparkDateTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 26], "filename": "src/function/datetime/date_trunc.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/date_trunc.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-709a8d734285c4bcf15b1243"></a>
## fmt

`function` · `datafusion_spark::function::datetime::date_trunc::SparkDateTrunc::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_trunc::SparkDateTrunc", "path": "SparkDateTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/function/datetime/date_trunc.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/date_trunc.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2062282909c68307a944cd70"></a>
## hash

`function` · `datafusion_spark::function::datetime::date_trunc::SparkDateTrunc::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_trunc::SparkDateTrunc", "path": "SparkDateTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 32], "end": [34, 36], "filename": "src/function/datetime/date_trunc.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/date_trunc.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2554efcf3e5a461d6c245d58"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::date_trunc::SparkDateTrunc::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_trunc::SparkDateTrunc", "path": "SparkDateTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [167, 2], "filename": "src/function/datetime/date_trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_trunc.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a53f09d94ad12ed3cce7756d"></a>
## name

`function` · `datafusion_spark::function::datetime::date_trunc::SparkDateTrunc::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_trunc::SparkDateTrunc", "path": "SparkDateTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [167, 2], "filename": "src/function/datetime/date_trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_trunc.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ebfa4634b4babe808cd997f"></a>
## new

`function` · `datafusion_spark::function::datetime::date_trunc::SparkDateTrunc::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_trunc::SparkDateTrunc", "path": "SparkDateTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [61, 2], "filename": "src/function/datetime/date_trunc.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/date_trunc.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48cb9f7b04adba6b12e4e02f"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::date_trunc::SparkDateTrunc::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_trunc::SparkDateTrunc", "path": "SparkDateTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [167, 2], "filename": "src/function/datetime/date_trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_trunc.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d42e15f073bb0a2ec225e6f4"></a>
## return_type

`function` · `datafusion_spark::function::datetime::date_trunc::SparkDateTrunc::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_trunc::SparkDateTrunc", "path": "SparkDateTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [167, 2], "filename": "src/function/datetime/date_trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_trunc.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe3509e4f730b782e5880b28"></a>
## signature

`function` · `datafusion_spark::function::datetime::date_trunc::SparkDateTrunc::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_trunc::SparkDateTrunc", "path": "SparkDateTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [167, 2], "filename": "src/function/datetime/date_trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_trunc.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fac9096c5a2f7c8e43b4b80"></a>
## simplify

`function` · `datafusion_spark::function::datetime::date_trunc::SparkDateTrunc::simplify` · datafusion-spark 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_trunc::SparkDateTrunc", "path": "SparkDateTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [167, 2], "filename": "src/function/datetime/date_trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_trunc.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
