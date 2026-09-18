# `datafusion_spark::function::aggregate::collect::SparkCollectSet`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.aggregate.collect.SparkCollectSet.json).

<a id="op-b9e9a42a3b0b369b25c74838"></a>
## SparkCollectSet

`struct` · `datafusion_spark::function::aggregate::collect::SparkCollectSet` · datafusion-spark 55.1.0

```rust
struct SparkCollectSet
```

Source: `src/function/aggregate/collect.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-088d9f1ddfd6059d7482a1b1"></a>
## accumulator

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectSet::accumulator` · datafusion-spark 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectSet", "path": "SparkCollectSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [169, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/collect.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b4483e272872b4cacc528b8"></a>
## default

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectSet::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectSet", "path": "SparkCollectSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [120, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/aggregate/collect.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0bc8096b36ce692c6ce4eda"></a>
## default_value

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectSet::default_value` · datafusion-spark 55.1.0

```rust
fn default_value(&self, data_type: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectSet", "path": "SparkCollectSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [169, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/collect.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4e7b2801ea4598be1ffc729"></a>
## eq

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectSet::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkCollectSet) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectSet", "path": "SparkCollectSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 17], "end": [111, 26], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/aggregate/collect.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08e316b58fba218bb0182a64"></a>
## fmt

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectSet::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectSet", "path": "SparkCollectSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 10], "end": [111, 15], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/aggregate/collect.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-116e81f790d5f8a8a378341b"></a>
## hash

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectSet::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectSet", "path": "SparkCollectSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 32], "end": [111, 36], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/aggregate/collect.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-051ef5e1c3fd43a3904a0cad"></a>
## name

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectSet::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectSet", "path": "SparkCollectSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [169, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/collect.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ab060c72a5593402c43446f"></a>
## new

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectSet::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectSet", "path": "SparkCollectSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [128, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/aggregate/collect.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9782562215698654bc43e87"></a>
## return_type

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectSet::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectSet", "path": "SparkCollectSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [169, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/collect.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dc05e3d2d61fe44a698349c"></a>
## signature

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectSet::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectSet", "path": "SparkCollectSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [169, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/collect.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a48bd9e9cfdc878a2979d43"></a>
## state_fields

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectSet::state_fields` · datafusion-spark 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectSet", "path": "SparkCollectSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [169, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/collect.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
