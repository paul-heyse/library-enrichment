# `datafusion_spark::function::aggregate::try_sum::SparkTrySum`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.aggregate.try_sum.SparkTrySum.json).

<a id="op-82bcaa663b9aaf716a43396d"></a>
## SparkTrySum

`struct` · `datafusion_spark::function::aggregate::try_sum::SparkTrySum` · datafusion-spark 55.1.0

```rust
struct SparkTrySum
```

Source: `src/function/aggregate/try_sum.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7808b90a8710b5c265a0129c"></a>
## accumulator

`function` · `datafusion_spark::function::aggregate::try_sum::SparkTrySum::accumulator` · datafusion-spark 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::try_sum::SparkTrySum", "path": "SparkTrySum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [249, 1], "end": [333, 2], "filename": "src/function/aggregate/try_sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/try_sum.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ceb5a0fc82853b7fa1ad6f39"></a>
## coerce_types

`function` · `datafusion_spark::function::aggregate::try_sum::SparkTrySum::coerce_types` · datafusion-spark 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::try_sum::SparkTrySum", "path": "SparkTrySum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [249, 1], "end": [333, 2], "filename": "src/function/aggregate/try_sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/try_sum.rs:310`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d699c5525770947a0d61a012"></a>
## default

`function` · `datafusion_spark::function::aggregate::try_sum::SparkTrySum::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::try_sum::SparkTrySum", "path": "SparkTrySum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [39, 2], "filename": "src/function/aggregate/try_sum.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/aggregate/try_sum.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efad93fec458ad54e85a9657"></a>
## default_value

`function` · `datafusion_spark::function::aggregate::try_sum::SparkTrySum::default_value` · datafusion-spark 55.1.0

```rust
fn default_value(&self, _data_type: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::try_sum::SparkTrySum", "path": "SparkTrySum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [249, 1], "end": [333, 2], "filename": "src/function/aggregate/try_sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/try_sum.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b05c9ad54fd50618ce87ec8b"></a>
## eq

`function` · `datafusion_spark::function::aggregate::try_sum::SparkTrySum::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkTrySum) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::try_sum::SparkTrySum", "path": "SparkTrySum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 19], "filename": "src/function/aggregate/try_sum.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/aggregate/try_sum.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-650959b501b43e794d7cc28f"></a>
## fmt

`function` · `datafusion_spark::function::aggregate::try_sum::SparkTrySum::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::try_sum::SparkTrySum", "path": "SparkTrySum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [55, 2], "filename": "src/function/aggregate/try_sum.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/aggregate/try_sum.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da41b881317794da17d96043"></a>
## hash

`function` · `datafusion_spark::function::aggregate::try_sum::SparkTrySum::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::try_sum::SparkTrySum", "path": "SparkTrySum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 25], "end": [30, 29], "filename": "src/function/aggregate/try_sum.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/aggregate/try_sum.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8574f85fe4b3eed7a6402146"></a>
## name

`function` · `datafusion_spark::function::aggregate::try_sum::SparkTrySum::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::try_sum::SparkTrySum", "path": "SparkTrySum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [249, 1], "end": [333, 2], "filename": "src/function/aggregate/try_sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/try_sum.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51fb39f52e639cfa59a731a3"></a>
## new

`function` · `datafusion_spark::function::aggregate::try_sum::SparkTrySum::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::try_sum::SparkTrySum", "path": "SparkTrySum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [47, 2], "filename": "src/function/aggregate/try_sum.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/aggregate/try_sum.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ed9b32d6772b46a87811883"></a>
## return_type

`function` · `datafusion_spark::function::aggregate::try_sum::SparkTrySum::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::try_sum::SparkTrySum", "path": "SparkTrySum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [249, 1], "end": [333, 2], "filename": "src/function/aggregate/try_sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/try_sum.rs:258`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bec3023617deda5f34d93de7"></a>
## signature

`function` · `datafusion_spark::function::aggregate::try_sum::SparkTrySum::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::try_sum::SparkTrySum", "path": "SparkTrySum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [249, 1], "end": [333, 2], "filename": "src/function/aggregate/try_sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/try_sum.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-208ed95faee3b0befdf18142"></a>
## state_fields

`function` · `datafusion_spark::function::aggregate::try_sum::SparkTrySum::state_fields` · datafusion-spark 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::try_sum::SparkTrySum", "path": "SparkTrySum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [249, 1], "end": [333, 2], "filename": "src/function/aggregate/try_sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/try_sum.rs:297`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
