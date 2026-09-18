# `datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.from_utc_timestamp.SparkFromUtcTimestamp.json).

<a id="op-8f8c689d9c9a9984f7583827"></a>
## SparkFromUtcTimestamp

`struct` · `datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp` · datafusion-spark 55.1.0

```rust
struct SparkFromUtcTimestamp
```

Source: `src/function/datetime/from_utc_timestamp.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Apache Spark `from_utc_timestamp` function.

Interprets the given timestamp as UTC and converts it to the given timezone.

Timestamp in Apache Spark represents number of microseconds from the Unix epoch, which is not
timezone-agnostic. So in Apache Spark this function just shift the timestamp value from UTC timezone to
the given timezone.

See <https://spark.apache.org/docs/latest/api/sql/index.html#from_utc_timestamp>

<a id="op-c7e97dc6ac8b2645bf769f11"></a>
## default

`function` · `datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp", "path": "SparkFromUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [55, 2], "filename": "src/function/datetime/from_utc_timestamp.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/from_utc_timestamp.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cab6fba9786cec6fba3c87a5"></a>
## eq

`function` · `datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkFromUtcTimestamp) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp", "path": "SparkFromUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 17], "end": [46, 26], "filename": "src/function/datetime/from_utc_timestamp.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/from_utc_timestamp.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5058dfeb7a579b9b2ba2d8f8"></a>
## fmt

`function` · `datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp", "path": "SparkFromUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 10], "end": [46, 15], "filename": "src/function/datetime/from_utc_timestamp.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/from_utc_timestamp.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0efa37c9c951961d6e6e3737"></a>
## hash

`function` · `datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp", "path": "SparkFromUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 32], "end": [46, 36], "filename": "src/function/datetime/from_utc_timestamp.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/from_utc_timestamp.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c039db305cafc2f2517e162"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp", "path": "SparkFromUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [101, 2], "filename": "src/function/datetime/from_utc_timestamp.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/from_utc_timestamp.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea926f4ab10ae7bbbbfa2cac"></a>
## name

`function` · `datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp", "path": "SparkFromUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [101, 2], "filename": "src/function/datetime/from_utc_timestamp.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/from_utc_timestamp.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a2de1113fb95b9fc8651226"></a>
## new

`function` · `datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp", "path": "SparkFromUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [73, 2], "filename": "src/function/datetime/from_utc_timestamp.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/from_utc_timestamp.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23d4c0cbe87e222e734f2f50"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp", "path": "SparkFromUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [101, 2], "filename": "src/function/datetime/from_utc_timestamp.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/from_utc_timestamp.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-116ee84af07f252ee4377f38"></a>
## return_type

`function` · `datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp", "path": "SparkFromUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [101, 2], "filename": "src/function/datetime/from_utc_timestamp.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/from_utc_timestamp.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c379635adf89f254031bb803"></a>
## signature

`function` · `datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp", "path": "SparkFromUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [101, 2], "filename": "src/function/datetime/from_utc_timestamp.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/from_utc_timestamp.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
