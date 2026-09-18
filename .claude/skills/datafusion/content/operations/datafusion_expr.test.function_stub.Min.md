# `datafusion_expr::test::function_stub::Min`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.test.function_stub.Min.json).

<a id="op-bea1fba13fa5488f345611d1"></a>
## Min

`struct` · `datafusion_expr::test::function_stub::Min` · datafusion-expr 55.1.0

```rust
struct Min
```

Source: `src/test/function_stub.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Testing stub implementation of Min aggregate

<a id="op-d9b58a712c410a93d5f77534"></a>
## accumulator

`function` · `datafusion_expr::test::function_stub::Min::accumulator` · datafusion-expr 55.1.0

```rust
fn accumulator(&self, _acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 1], "end": [359, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-969615891c8a1fd7321f7ae1"></a>
## create_groups_accumulator

`function` · `datafusion_expr::test::function_stub::Min::create_groups_accumulator` · datafusion-expr 55.1.0

```rust
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 1], "end": [359, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:346`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c08b84873bcb01adcb839dcc"></a>
## default

`function` · `datafusion_expr::test::function_stub::Min::default` · datafusion-expr 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [315, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/test/function_stub.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd370704a7a980c2717643cb"></a>
## eq

`function` · `datafusion_expr::test::function_stub::Min::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Min) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [297, 10], "end": [297, 19], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/test/function_stub.rs:297`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-180e7cdb5e1cce994f77cf8b"></a>
## fmt

`function` · `datafusion_expr::test::function_stub::Min::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 1], "end": [309, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/function_stub.rs:303`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cd183f11202523ee7dea2a8"></a>
## hash

`function` · `datafusion_expr::test::function_stub::Min::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [297, 25], "end": [297, 29], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/test/function_stub.rs:297`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18265f6e6105b8eaad3857b9"></a>
## is_descending

`function` · `datafusion_expr::test::function_stub::Min::is_descending` · datafusion-expr 55.1.0

```rust
fn is_descending(&self) -> Option<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 1], "end": [359, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:356`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69c308aac4cb4cc506340401"></a>
## name

`function` · `datafusion_expr::test::function_stub::Min::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 1], "end": [359, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f40336420e2ab08b5fa65a1"></a>
## new

`function` · `datafusion_expr::test::function_stub::Min::new` · datafusion-expr 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [317, 1], "end": [323, 2], "filename": "src/test/function_stub.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/function_stub.rs:318`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e897934fe2b759458d97c0c4"></a>
## return_type

`function` · `datafusion_expr::test::function_stub::Min::return_type` · datafusion-expr 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 1], "end": [359, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a67e190bde4f060bb00b8539"></a>
## reverse_expr

`function` · `datafusion_expr::test::function_stub::Min::reverse_expr` · datafusion-expr 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 1], "end": [359, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:353`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b43438fc98a785e6069f207d"></a>
## signature

`function` · `datafusion_expr::test::function_stub::Min::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 1], "end": [359, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efa1c9a25059eda49a701f26"></a>
## state_fields

`function` · `datafusion_expr::test::function_stub::Min::state_fields` · datafusion-expr 55.1.0

```rust
fn state_fields(&self, _args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 1], "end": [359, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:338`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
