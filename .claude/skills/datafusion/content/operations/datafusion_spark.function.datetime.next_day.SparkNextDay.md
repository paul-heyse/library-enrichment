# `datafusion_spark::function::datetime::next_day::SparkNextDay`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.next_day.SparkNextDay.json).

<a id="op-0ee160333f01901da40bb3a6"></a>
## SparkNextDay

`struct` · `datafusion_spark::function::datetime::next_day::SparkNextDay` · datafusion-spark 55.1.0

```rust
struct SparkNextDay
```

Source: `src/function/datetime/next_day.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

<https://spark.apache.org/docs/latest/api/sql/index.html#next_day>

<a id="op-1da9aee2850ed719dfdffa75"></a>
## default

`function` · `datafusion_spark::function::datetime::next_day::SparkNextDay::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::next_day::SparkNextDay", "path": "SparkNextDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [39, 2], "filename": "src/function/datetime/next_day.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/next_day.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d618f692be75d7038ed2263"></a>
## eq

`function` · `datafusion_spark::function::datetime::next_day::SparkNextDay::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkNextDay) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::next_day::SparkNextDay", "path": "SparkNextDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 17], "end": [30, 26], "filename": "src/function/datetime/next_day.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/next_day.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdda1f8ddea5dc41ac8423c1"></a>
## fmt

`function` · `datafusion_spark::function::datetime::next_day::SparkNextDay::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::next_day::SparkNextDay", "path": "SparkNextDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/function/datetime/next_day.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/next_day.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-699855a798d6d3b1ff1ff664"></a>
## hash

`function` · `datafusion_spark::function::datetime::next_day::SparkNextDay::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::next_day::SparkNextDay", "path": "SparkNextDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 32], "end": [30, 36], "filename": "src/function/datetime/next_day.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/next_day.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-580b29b56003c4685ab40870"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::next_day::SparkNextDay::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::next_day::SparkNextDay", "path": "SparkNextDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [181, 2], "filename": "src/function/datetime/next_day.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/next_day.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80c9e5ea6d60c2e23a1c61c2"></a>
## name

`function` · `datafusion_spark::function::datetime::next_day::SparkNextDay::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::next_day::SparkNextDay", "path": "SparkNextDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [181, 2], "filename": "src/function/datetime/next_day.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/next_day.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8da071fe7474997017fe8ef"></a>
## new

`function` · `datafusion_spark::function::datetime::next_day::SparkNextDay::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::next_day::SparkNextDay", "path": "SparkNextDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [50, 2], "filename": "src/function/datetime/next_day.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/next_day.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e274bbc528c59ffc08d2c574"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::next_day::SparkNextDay::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, _args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::next_day::SparkNextDay", "path": "SparkNextDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [181, 2], "filename": "src/function/datetime/next_day.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/next_day.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aad35656a09f4d2b8bcb1a64"></a>
## return_type

`function` · `datafusion_spark::function::datetime::next_day::SparkNextDay::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::next_day::SparkNextDay", "path": "SparkNextDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [181, 2], "filename": "src/function/datetime/next_day.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/next_day.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-142fc6ee4b63417c3d001674"></a>
## signature

`function` · `datafusion_spark::function::datetime::next_day::SparkNextDay::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::next_day::SparkNextDay", "path": "SparkNextDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [181, 2], "filename": "src/function/datetime/next_day.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/next_day.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
