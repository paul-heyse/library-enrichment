# `datafusion_functions_aggregate::grouping::Grouping`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.grouping.Grouping.json).

<a id="op-c9b26e86af1fad6ee74f1063"></a>
## Grouping

`struct` · `datafusion_functions_aggregate::grouping::Grouping` · datafusion-functions-aggregate 55.1.0

```rust
struct Grouping
```

Source: `src/grouping.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae3e75b4ecbb6d2a055cbcb9"></a>
## accumulator

`function` · `datafusion_functions_aggregate::grouping::Grouping::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, _acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::grouping::Grouping", "path": "Grouping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [113, 2], "filename": "src/grouping.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/grouping.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c3368dae32ef5487cadbe89"></a>
## default

`function` · `datafusion_functions_aggregate::grouping::Grouping::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::grouping::Grouping", "path": "Grouping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [69, 2], "filename": "src/grouping.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/grouping.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-596a49f8c82aa104e026d872"></a>
## documentation

`function` · `datafusion_functions_aggregate::grouping::Grouping::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::grouping::Grouping", "path": "Grouping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [113, 2], "filename": "src/grouping.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/grouping.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa6934e7faadf0ead209d885"></a>
## eq

`function` · `datafusion_functions_aggregate::grouping::Grouping::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &Grouping) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::grouping::Grouping", "path": "Grouping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 10], "end": [60, 19], "filename": "src/grouping.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/grouping.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44b8986a781b0cfeb9efcf75"></a>
## fmt

`function` · `datafusion_functions_aggregate::grouping::Grouping::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::grouping::Grouping", "path": "Grouping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 31], "end": [60, 36], "filename": "src/grouping.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/grouping.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e2fafb0bcebb141a89ccfa5"></a>
## hash

`function` · `datafusion_functions_aggregate::grouping::Grouping::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::grouping::Grouping", "path": "Grouping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 25], "end": [60, 29], "filename": "src/grouping.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/grouping.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bfdec207d081cfe6db6071e"></a>
## name

`function` · `datafusion_functions_aggregate::grouping::Grouping::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::grouping::Grouping", "path": "Grouping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [113, 2], "filename": "src/grouping.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/grouping.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45c3bf501e5291618327d222"></a>
## new

`function` · `datafusion_functions_aggregate::grouping::Grouping::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::grouping::Grouping", "path": "Grouping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [78, 2], "filename": "src/grouping.rs"}, "trait": null, "trait_path": null}`

Source: `src/grouping.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Create a new GROUPING aggregate function.

<a id="op-cf91f5485bc81c5b0d39636e"></a>
## return_type

`function` · `datafusion_functions_aggregate::grouping::Grouping::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::grouping::Grouping", "path": "Grouping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [113, 2], "filename": "src/grouping.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/grouping.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-937d3de7805ee42229cc1588"></a>
## signature

`function` · `datafusion_functions_aggregate::grouping::Grouping::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::grouping::Grouping", "path": "Grouping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [113, 2], "filename": "src/grouping.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/grouping.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48530a5540b7a6f1bfc30bde"></a>
## state_fields

`function` · `datafusion_functions_aggregate::grouping::Grouping::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::grouping::Grouping", "path": "Grouping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [113, 2], "filename": "src/grouping.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/grouping.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
