# `datafusion_functions_aggregate::covariance::CovariancePopulation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.covariance.CovariancePopulation.json).

<a id="op-51f89cb4fc4bc2cae12c79c1"></a>
## CovariancePopulation

`struct` · `datafusion_functions_aggregate::covariance::CovariancePopulation` · datafusion-functions-aggregate 55.1.0

```rust
struct CovariancePopulation
```

Source: `src/covariance.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34212d40f8622ca824313b53"></a>
## accumulator

`function` · `datafusion_functions_aggregate::covariance::CovariancePopulation::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, _acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovariancePopulation", "path": "CovariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [209, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/covariance.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aab79c0dc69328e62926f5bd"></a>
## default

`function` · `datafusion_functions_aggregate::covariance::CovariancePopulation::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovariancePopulation", "path": "CovariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [157, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/covariance.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7009f9de9b94c5cd99aa13db"></a>
## documentation

`function` · `datafusion_functions_aggregate::covariance::CovariancePopulation::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovariancePopulation", "path": "CovariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [209, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/covariance.rs:206`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eda679a0fac62c42da2eb971"></a>
## eq

`function` · `datafusion_functions_aggregate::covariance::CovariancePopulation::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &CovariancePopulation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovariancePopulation", "path": "CovariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 10], "end": [148, 19], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/covariance.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d90917cece671a4ddea01f93"></a>
## fmt

`function` · `datafusion_functions_aggregate::covariance::CovariancePopulation::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovariancePopulation", "path": "CovariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 31], "end": [148, 36], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/covariance.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23ec13ec662916707d586e88"></a>
## hash

`function` · `datafusion_functions_aggregate::covariance::CovariancePopulation::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovariancePopulation", "path": "CovariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 25], "end": [148, 29], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/covariance.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-956ead6de3f194bfc53524de"></a>
## name

`function` · `datafusion_functions_aggregate::covariance::CovariancePopulation::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovariancePopulation", "path": "CovariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [209, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/covariance.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f419e7e069dd60607896b4f"></a>
## new

`function` · `datafusion_functions_aggregate::covariance::CovariancePopulation::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovariancePopulation", "path": "CovariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [159, 1], "end": [168, 2], "filename": "src/covariance.rs"}, "trait": null, "trait_path": null}`

Source: `src/covariance.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c89ed01d49a4e0a4c29ecec0"></a>
## return_type

`function` · `datafusion_functions_aggregate::covariance::CovariancePopulation::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovariancePopulation", "path": "CovariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [209, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/covariance.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00343d62d8a496b268e12f8e"></a>
## signature

`function` · `datafusion_functions_aggregate::covariance::CovariancePopulation::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovariancePopulation", "path": "CovariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [209, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/covariance.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ecc9f2ac15921fddb8083e7"></a>
## state_fields

`function` · `datafusion_functions_aggregate::covariance::CovariancePopulation::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovariancePopulation", "path": "CovariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 1], "end": [209, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/covariance.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
