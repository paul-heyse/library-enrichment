# `datafusion_spark::function::datetime::last_day::SparkLastDay`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.last_day.SparkLastDay.json).

<a id="op-574be6ed0ded0dfacd2b7817"></a>
## SparkLastDay

`struct` · `datafusion_spark::function::datetime::last_day::SparkLastDay` · datafusion-spark 55.1.0

```rust
struct SparkLastDay
```

Source: `src/function/datetime/last_day.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98ae0f90495bcadb0eef36a1"></a>
## default

`function` · `datafusion_spark::function::datetime::last_day::SparkLastDay::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::last_day::SparkLastDay", "path": "SparkLastDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [39, 2], "filename": "src/function/datetime/last_day.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/last_day.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58f47605e22772f1679a279b"></a>
## eq

`function` · `datafusion_spark::function::datetime::last_day::SparkLastDay::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkLastDay) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::last_day::SparkLastDay", "path": "SparkLastDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 17], "end": [30, 26], "filename": "src/function/datetime/last_day.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/last_day.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28a7b2ddaa7ddaab817e18b1"></a>
## fmt

`function` · `datafusion_spark::function::datetime::last_day::SparkLastDay::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::last_day::SparkLastDay", "path": "SparkLastDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/function/datetime/last_day.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/last_day.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7a5cece815319ffc7a5979b"></a>
## hash

`function` · `datafusion_spark::function::datetime::last_day::SparkLastDay::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::last_day::SparkLastDay", "path": "SparkLastDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 32], "end": [30, 36], "filename": "src/function/datetime/last_day.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/last_day.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b38d175615d7a534373e225"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::last_day::SparkLastDay::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::last_day::SparkLastDay", "path": "SparkLastDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [109, 2], "filename": "src/function/datetime/last_day.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/last_day.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-970eabaa7747685c819ce232"></a>
## name

`function` · `datafusion_spark::function::datetime::last_day::SparkLastDay::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::last_day::SparkLastDay", "path": "SparkLastDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [109, 2], "filename": "src/function/datetime/last_day.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/last_day.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b087d237d106a14e9b04212b"></a>
## new

`function` · `datafusion_spark::function::datetime::last_day::SparkLastDay::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::last_day::SparkLastDay", "path": "SparkLastDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [47, 2], "filename": "src/function/datetime/last_day.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/last_day.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df1ccdf91117cdba1aaf0e52"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::last_day::SparkLastDay::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::last_day::SparkLastDay", "path": "SparkLastDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [109, 2], "filename": "src/function/datetime/last_day.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/last_day.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ba401edf09df95597a5eadb"></a>
## return_type

`function` · `datafusion_spark::function::datetime::last_day::SparkLastDay::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::last_day::SparkLastDay", "path": "SparkLastDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [109, 2], "filename": "src/function/datetime/last_day.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/last_day.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1196b30c560f9fbac183af24"></a>
## signature

`function` · `datafusion_spark::function::datetime::last_day::SparkLastDay::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::last_day::SparkLastDay", "path": "SparkLastDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [109, 2], "filename": "src/function/datetime/last_day.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/last_day.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
