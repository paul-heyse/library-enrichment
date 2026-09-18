# `datafusion_spark::function::datetime::weekday::SparkWeekDay`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.weekday.SparkWeekDay.json).

<a id="op-6f8291792d92af29b9f9c65e"></a>
## SparkWeekDay

`struct` · `datafusion_spark::function::datetime::weekday::SparkWeekDay` · datafusion-spark 55.1.0

```rust
struct SparkWeekDay
```

Source: `src/function/datetime/weekday.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `weekday` expression.
Returns the day of the week for a date or timestamp as an integer index where
Monday = 0, Tuesday = 1, ..., Sunday = 6.

Note: this differs from `dayofweek`, which is 1-indexed with Sunday = 1.

<https://spark.apache.org/docs/latest/api/sql/index.html#weekday>

<a id="op-2d721619cce2368189607ec5"></a>
## default

`function` · `datafusion_spark::function::datetime::weekday::SparkWeekDay::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::weekday::SparkWeekDay", "path": "SparkWeekDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [47, 2], "filename": "src/function/datetime/weekday.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/weekday.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17d26ab850605da59f2d6f60"></a>
## eq

`function` · `datafusion_spark::function::datetime::weekday::SparkWeekDay::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkWeekDay) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::weekday::SparkWeekDay", "path": "SparkWeekDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 17], "end": [38, 26], "filename": "src/function/datetime/weekday.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/weekday.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c075e1132d63500be3b0bce1"></a>
## fmt

`function` · `datafusion_spark::function::datetime::weekday::SparkWeekDay::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::weekday::SparkWeekDay", "path": "SparkWeekDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/function/datetime/weekday.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/weekday.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1986e05992e16a7ae976329"></a>
## hash

`function` · `datafusion_spark::function::datetime::weekday::SparkWeekDay::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::weekday::SparkWeekDay", "path": "SparkWeekDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 32], "end": [38, 36], "filename": "src/function/datetime/weekday.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/weekday.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a3b4b6f749e549453204122"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::weekday::SparkWeekDay::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::weekday::SparkWeekDay", "path": "SparkWeekDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [102, 2], "filename": "src/function/datetime/weekday.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/weekday.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b485c75f905c6d53554201d"></a>
## name

`function` · `datafusion_spark::function::datetime::weekday::SparkWeekDay::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::weekday::SparkWeekDay", "path": "SparkWeekDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [102, 2], "filename": "src/function/datetime/weekday.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/weekday.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f04cadafcca093d5a69fc4bf"></a>
## new

`function` · `datafusion_spark::function::datetime::weekday::SparkWeekDay::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::weekday::SparkWeekDay", "path": "SparkWeekDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [62, 2], "filename": "src/function/datetime/weekday.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/weekday.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc21a0f0dab0449dc5480ff9"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::weekday::SparkWeekDay::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::weekday::SparkWeekDay", "path": "SparkWeekDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [102, 2], "filename": "src/function/datetime/weekday.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/weekday.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b56ffc1030ed1f43f253e7c"></a>
## return_type

`function` · `datafusion_spark::function::datetime::weekday::SparkWeekDay::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::weekday::SparkWeekDay", "path": "SparkWeekDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [102, 2], "filename": "src/function/datetime/weekday.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/weekday.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6ce3b4bfae65c3941c17b4b"></a>
## signature

`function` · `datafusion_spark::function::datetime::weekday::SparkWeekDay::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::weekday::SparkWeekDay", "path": "SparkWeekDay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [102, 2], "filename": "src/function/datetime/weekday.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/weekday.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
