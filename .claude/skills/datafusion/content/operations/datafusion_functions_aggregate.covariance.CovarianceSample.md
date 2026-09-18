# `datafusion_functions_aggregate::covariance::CovarianceSample`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.covariance.CovarianceSample.json).

<a id="op-e5fa762d69a4fae51b9b03bb"></a>
## CovarianceSample

`struct` · `datafusion_functions_aggregate::covariance::CovarianceSample` · datafusion-functions-aggregate 55.1.0

```rust
struct CovarianceSample
```

Source: `src/covariance.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cbe40410af8ede8c8d5ea3c"></a>
## accumulator

`function` · `datafusion_functions_aggregate::covariance::CovarianceSample::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, _acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceSample", "path": "CovarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [131, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/covariance.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70be37e22248bfc78e239036"></a>
## aliases

`function` · `datafusion_functions_aggregate::covariance::CovarianceSample::aliases` · datafusion-functions-aggregate 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceSample", "path": "CovarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [131, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/covariance.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c349e624d04ad26661b74bd"></a>
## default

`function` · `datafusion_functions_aggregate::covariance::CovarianceSample::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceSample", "path": "CovarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [76, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/covariance.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c89895c6f6809af22e38bb1"></a>
## documentation

`function` · `datafusion_functions_aggregate::covariance::CovarianceSample::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceSample", "path": "CovarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [131, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/covariance.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d49ee58a5bd5cc652394291d"></a>
## eq

`function` · `datafusion_functions_aggregate::covariance::CovarianceSample::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &CovarianceSample) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceSample", "path": "CovarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 10], "end": [66, 19], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/covariance.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3cb230f8fc2214862081cda"></a>
## fmt

`function` · `datafusion_functions_aggregate::covariance::CovarianceSample::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceSample", "path": "CovarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 31], "end": [66, 36], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/covariance.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66c4f4cfb551f01c0cf9f2ac"></a>
## hash

`function` · `datafusion_functions_aggregate::covariance::CovarianceSample::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceSample", "path": "CovarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 25], "end": [66, 29], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/covariance.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85a6b7e3ce3aa45e3bdfbe20"></a>
## name

`function` · `datafusion_functions_aggregate::covariance::CovarianceSample::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceSample", "path": "CovarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [131, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/covariance.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e68d281707be6fb04248fa15"></a>
## new

`function` · `datafusion_functions_aggregate::covariance::CovarianceSample::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceSample", "path": "CovarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [88, 2], "filename": "src/covariance.rs"}, "trait": null, "trait_path": null}`

Source: `src/covariance.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3986b3f9be2a30f3659028b8"></a>
## return_type

`function` · `datafusion_functions_aggregate::covariance::CovarianceSample::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceSample", "path": "CovarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [131, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/covariance.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cc07909b2d059411565c2c2"></a>
## signature

`function` · `datafusion_functions_aggregate::covariance::CovarianceSample::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceSample", "path": "CovarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [131, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/covariance.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d71f0497518139fa5c4d67cb"></a>
## state_fields

`function` · `datafusion_functions_aggregate::covariance::CovarianceSample::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceSample", "path": "CovarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [131, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/covariance.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
