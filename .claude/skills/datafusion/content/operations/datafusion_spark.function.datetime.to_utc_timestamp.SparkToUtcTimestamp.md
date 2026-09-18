# `datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.to_utc_timestamp.SparkToUtcTimestamp.json).

<a id="op-3de069dc63740ac850a199d0"></a>
## SparkToUtcTimestamp

`struct` · `datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp` · datafusion-spark 55.1.0

```rust
struct SparkToUtcTimestamp
```

Source: `src/function/datetime/to_utc_timestamp.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Apache Spark `to_utc_timestamp` function.

Interprets the given timestamp in the provided timezone and then converts it to UTC.

Timestamp in Apache Spark represents number of microseconds from the Unix epoch, which is not
timezone-agnostic. So in Apache Spark this function just shift the timestamp value from the given
timezone to UTC timezone.

See <https://spark.apache.org/docs/latest/api/sql/index.html#to_utc_timestamp>

<a id="op-d192aaa2d4db12bb268b6fe8"></a>
## default

`function` · `datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp", "path": "SparkToUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [57, 2], "filename": "src/function/datetime/to_utc_timestamp.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/to_utc_timestamp.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66a38c1c86f4321d6863145c"></a>
## eq

`function` · `datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkToUtcTimestamp) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp", "path": "SparkToUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 17], "end": [48, 26], "filename": "src/function/datetime/to_utc_timestamp.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/to_utc_timestamp.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-029e22b2c55574731c88b6f9"></a>
## fmt

`function` · `datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp", "path": "SparkToUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 10], "end": [48, 15], "filename": "src/function/datetime/to_utc_timestamp.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/to_utc_timestamp.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a14c5bdc22b6d3b93deeadaa"></a>
## hash

`function` · `datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp", "path": "SparkToUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 32], "end": [48, 36], "filename": "src/function/datetime/to_utc_timestamp.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/to_utc_timestamp.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-badef63c44a9b9ebcc68aa87"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp", "path": "SparkToUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [103, 2], "filename": "src/function/datetime/to_utc_timestamp.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/to_utc_timestamp.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfa4640cb3e5939ff783c4d3"></a>
## name

`function` · `datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp", "path": "SparkToUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [103, 2], "filename": "src/function/datetime/to_utc_timestamp.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/to_utc_timestamp.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8658f9a2a10cf7ba5ece3739"></a>
## new

`function` · `datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp", "path": "SparkToUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [75, 2], "filename": "src/function/datetime/to_utc_timestamp.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/to_utc_timestamp.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-accc8675e8cdffe7e682c77b"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp", "path": "SparkToUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [103, 2], "filename": "src/function/datetime/to_utc_timestamp.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/to_utc_timestamp.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cc18215ac203863031d8892"></a>
## return_type

`function` · `datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp", "path": "SparkToUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [103, 2], "filename": "src/function/datetime/to_utc_timestamp.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/to_utc_timestamp.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86a53efa3b33f7613564d4af"></a>
## signature

`function` · `datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp", "path": "SparkToUtcTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [103, 2], "filename": "src/function/datetime/to_utc_timestamp.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/to_utc_timestamp.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
