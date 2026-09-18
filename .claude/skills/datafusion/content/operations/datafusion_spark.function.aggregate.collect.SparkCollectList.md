# `datafusion_spark::function::aggregate::collect::SparkCollectList`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.aggregate.collect.SparkCollectList.json).

<a id="op-642fd5fa464cb65c7393ee04"></a>
## SparkCollectList

`struct` · `datafusion_spark::function::aggregate::collect::SparkCollectList` · datafusion-spark 55.1.0

```rust
struct SparkCollectList
```

Source: `src/function/aggregate/collect.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc4ccc230c8770e529cdca65"></a>
## accumulator

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectList::accumulator` · datafusion-spark 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectList", "path": "SparkCollectList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [108, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/collect.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-597790b0eecf8fe3fbfc661a"></a>
## default

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectList::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectList", "path": "SparkCollectList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [59, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/aggregate/collect.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10cb34d74502078b00b8066e"></a>
## default_value

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectList::default_value` · datafusion-spark 55.1.0

```rust
fn default_value(&self, data_type: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectList", "path": "SparkCollectList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [108, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/collect.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05eb0ec24d870975d132a560"></a>
## eq

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectList::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkCollectList) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectList", "path": "SparkCollectList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 17], "end": [50, 26], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/aggregate/collect.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cf4a5609b972a28367e4748"></a>
## fmt

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectList::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectList", "path": "SparkCollectList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 10], "end": [50, 15], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/aggregate/collect.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e398e9a056d2dc0506695d70"></a>
## hash

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectList::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectList", "path": "SparkCollectList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 32], "end": [50, 36], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/aggregate/collect.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bce7c5092efcb50291b7fd2"></a>
## name

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectList::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectList", "path": "SparkCollectList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [108, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/collect.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41fbca397ca0b6fd7b43a6e0"></a>
## new

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectList::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectList", "path": "SparkCollectList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [67, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/aggregate/collect.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c7b9d065ac60c5a0f894e4e"></a>
## return_type

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectList::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectList", "path": "SparkCollectList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [108, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/collect.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4413aa15012781ce181d357"></a>
## signature

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectList::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectList", "path": "SparkCollectList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [108, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/collect.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dfbde36c64f6636970f850c"></a>
## state_fields

`function` · `datafusion_spark::function::aggregate::collect::SparkCollectList::state_fields` · datafusion-spark 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::collect::SparkCollectList", "path": "SparkCollectList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [108, 2], "filename": "src/function/aggregate/collect.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/function/aggregate/collect.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
