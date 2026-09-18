# `datafusion_functions_aggregate::approx_median::ApproxMedian`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.approx_median.ApproxMedian.json).

<a id="op-b3c48ecad789423beae97699"></a>
## ApproxMedian

`struct` · `datafusion_functions_aggregate::approx_median::ApproxMedian` · datafusion-functions-aggregate 55.1.0

```rust
struct ApproxMedian
```

Source: `src/approx_median.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

APPROX_MEDIAN aggregate expression

<a id="op-07cf0568f4df5e3742113be2"></a>
## accumulator

`function` · `datafusion_functions_aggregate::approx_median::ApproxMedian::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_median::ApproxMedian", "path": "ApproxMedian"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [150, 2], "filename": "src/approx_median.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_median.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-388877528674445589846bcc"></a>
## default

`function` · `datafusion_functions_aggregate::approx_median::ApproxMedian::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_median::ApproxMedian", "path": "ApproxMedian"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [70, 2], "filename": "src/approx_median.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/approx_median.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c02281c6d726d49dd3c84c4"></a>
## documentation

`function` · `datafusion_functions_aggregate::approx_median::ApproxMedian::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_median::ApproxMedian", "path": "ApproxMedian"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [150, 2], "filename": "src/approx_median.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_median.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32fe5560d70fe3de1b9ff0f2"></a>
## eq

`function` · `datafusion_functions_aggregate::approx_median::ApproxMedian::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &ApproxMedian) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_median::ApproxMedian", "path": "ApproxMedian"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 17], "end": [61, 26], "filename": "src/approx_median.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/approx_median.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67f8e6647671293af0b30d0f"></a>
## fmt

`function` · `datafusion_functions_aggregate::approx_median::ApproxMedian::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_median::ApproxMedian", "path": "ApproxMedian"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 10], "end": [61, 15], "filename": "src/approx_median.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/approx_median.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11f03cbdc181aa7a658f08ba"></a>
## hash

`function` · `datafusion_functions_aggregate::approx_median::ApproxMedian::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_median::ApproxMedian", "path": "ApproxMedian"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 32], "end": [61, 36], "filename": "src/approx_median.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/approx_median.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1a00e7a57fc766071077c99"></a>
## name

`function` · `datafusion_functions_aggregate::approx_median::ApproxMedian::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_median::ApproxMedian", "path": "ApproxMedian"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [150, 2], "filename": "src/approx_median.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_median.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bc30acecb1545e5108e030c"></a>
## new

`function` · `datafusion_functions_aggregate::approx_median::ApproxMedian::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_median::ApproxMedian", "path": "ApproxMedian"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [86, 2], "filename": "src/approx_median.rs"}, "trait": null, "trait_path": null}`

Source: `src/approx_median.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Create a new APPROX_MEDIAN aggregate function

<a id="op-a5e10772e26a5309d4e01370"></a>
## return_type

`function` · `datafusion_functions_aggregate::approx_median::ApproxMedian::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_median::ApproxMedian", "path": "ApproxMedian"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [150, 2], "filename": "src/approx_median.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_median.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37e1d9002b9619278062f078"></a>
## signature

`function` · `datafusion_functions_aggregate::approx_median::ApproxMedian::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_median::ApproxMedian", "path": "ApproxMedian"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [150, 2], "filename": "src/approx_median.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_median.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fd6baad49010be7d97e7340"></a>
## state_fields

`function` · `datafusion_functions_aggregate::approx_median::ApproxMedian::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_median::ApproxMedian", "path": "ApproxMedian"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [150, 2], "filename": "src/approx_median.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_median.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
