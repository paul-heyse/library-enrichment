# `datafusion_spark::function::aggregate::avg::SparkAvg`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.aggregate.avg.SparkAvg.json).

<a id="op-f6229d7d5ae56c70ca2901e4"></a>
## SparkAvg

`struct` · `datafusion_spark::function::aggregate::avg::SparkAvg` · datafusion-spark 55.1.0

```rust
struct SparkAvg
```

Source: `src/function/aggregate/avg.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

AVG aggregate expression
Spark average aggregate expression. Differs from standard DataFusion average aggregate
in that it uses an `i64` for the count (DataFusion version uses `u64`); also there is ANSI mode
support planned in the future for Spark version.

<a id="op-c162ac7299fbeb3fe28dadaa"></a>
## accumulator

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::accumulator` · datafusion-spark 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [151, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/avg.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5dc094464bd58d9b8c94957"></a>
## clone

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::clone` · datafusion-spark 55.1.0

```rust
fn clone(&self) -> SparkAvg
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 17], "end": [47, 22], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/function/aggregate/avg.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8450bf803f98c594070bca7"></a>
## create_groups_accumulator

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::create_groups_accumulator` · datafusion-spark 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [151, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/avg.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e6bdeaf679218f2ea18c477"></a>
## default

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [56, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/aggregate/avg.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa6fbc89d6fedee755f468c3"></a>
## default_value

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::default_value` · datafusion-spark 55.1.0

```rust
fn default_value(&self, _data_type: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [151, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/avg.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c908a57a30cd380f6d3008d"></a>
## eq

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkAvg) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 24], "end": [47, 33], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/aggregate/avg.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f3703ea7945aa6e9a75ad37"></a>
## fmt

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 15], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/aggregate/avg.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39fce0c3a0f4487751487839"></a>
## groups_accumulator_supported

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::groups_accumulator_supported` · datafusion-spark 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [151, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/avg.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78d52d300ecea1a835b535f1"></a>
## hash

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 39], "end": [47, 43], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/aggregate/avg.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acbb6a0286ab62823995ff8c"></a>
## name

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [151, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/avg.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f968bd838571f3cc4535304"></a>
## new

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [72, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/aggregate/avg.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Implement AVG aggregate function

<a id="op-9027059d6eea1adcc08fddb4"></a>
## return_type

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [151, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/avg.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cead743f2a61ccc3ef2593d"></a>
## reverse_expr

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::reverse_expr` · datafusion-spark 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [151, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/avg.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f33015a497afd0299c9fbe6f"></a>
## signature

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [151, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/avg.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35630077a1ec24fe9c678527"></a>
## state_fields

`function` · `datafusion_spark::function::aggregate::avg::SparkAvg::state_fields` · datafusion-spark 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::SparkAvg", "path": "SparkAvg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [151, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/avg.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
