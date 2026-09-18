# `datafusion_physical_expr_common::sort_expr::OrderingRequirements`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.sort_expr.OrderingRequirements.json).

<a id="op-b346497aa70b044d92555431"></a>
## OrderingRequirements

`enum` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements` · datafusion-physical-expr-common 55.1.0

```rust
enum OrderingRequirements
```

Source: `src/sort_expr.rs:820`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Represents a plan's input ordering requirements. Vector elements represent
alternative ordering requirements in the order of preference. The list of
alternatives can be either hard or soft, depending on whether the operator
can work without an input ordering.

# Invariants

The following always hold true for a `OrderingRequirements`:

1. It is non-degenerate, meaning it contains at least one ordering. The
   absence of an input ordering requirement is represented by a `None` value
   in `ExecutionPlan` APIs, which return an `Option<OrderingRequirements>`.

<a id="op-66a8801a64a298cce9ccc4bd"></a>
## Hard

`variant` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::Hard` · datafusion-physical-expr-common 55.1.0

```rust
Hard
```

Source: `src/sort_expr.rs:822`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

The operator is not able to work without one of these requirements.

<a id="op-ee59de8e1fc1b4a4a3cc095c"></a>
## Soft

`variant` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::Soft` · datafusion-physical-expr-common 55.1.0

```rust
Soft
```

Source: `src/sort_expr.rs:825`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

The operator can benefit from these input orderings when available,
but can still work in the absence of any input ordering.

<a id="op-22581d3520c4e85fb205f7de"></a>
## Target

`assoc_type` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::Target` · datafusion-physical-expr-common 55.1.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [900, 1], "end": [908, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/sort_expr.rs:901`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61c68feb6d4526012363000a"></a>
## add_alternative

`function` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::add_alternative` · datafusion-physical-expr-common 55.1.0

```rust
fn add_alternative(&mut self, requirement: LexRequirement)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [828, 1], "end": [886, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:856`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Adds an alternative requirement to the list of alternatives.

<a id="op-79779a332e431509ab073292"></a>
## clone

`function` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> OrderingRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [819, 17], "end": [819, 22], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sort_expr.rs:819`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c013330c6cabdf95189094c5"></a>
## deref

`function` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::deref` · datafusion-physical-expr-common 55.1.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [900, 1], "end": [908, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/sort_expr.rs:903`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7d2b2a78fffd0476f8d8f3c"></a>
## deref_mut

`function` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::deref_mut` · datafusion-physical-expr-common 55.1.0

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [910, 1], "end": [916, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::ops::deref::DerefMut", "path": "DerefMut"}, "trait_path": "core::ops::deref::DerefMut"}`

Source: `src/sort_expr.rs:911`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88d3f2c504ec975468f83871"></a>
## eq

`function` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::eq` · datafusion-physical-expr-common 55.1.0

```rust
fn eq(&self, other: &OrderingRequirements) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [819, 24], "end": [819, 33], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sort_expr.rs:819`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-298223cda3e8059cca50c557"></a>
## first

`function` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::first` · datafusion-physical-expr-common 55.1.0

```rust
fn first(&self) -> &LexRequirement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [828, 1], "end": [886, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:872`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns a reference to the first (i.e. most preferred) `LexRequirement`
among alternative requirements.

<a id="op-230921972211799ab4f563c1"></a>
## fmt

`function` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [819, 10], "end": [819, 15], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sort_expr.rs:819`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5001908dcf7008818f27751c"></a>
## from

`function` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::from` · datafusion-physical-expr-common 55.1.0

```rust
fn from(requirement: LexRequirement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [888, 1], "end": [892, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexRequirement", "path": "LexRequirement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/sort_expr.rs:889`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92c90bdffa7d72b4b636a786"></a>
## from

`function` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::from` · datafusion-physical-expr-common 55.1.0

```rust
fn from(ordering: LexOrdering) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [894, 1], "end": [898, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/sort_expr.rs:895`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc2c12e86750bd90bf492647"></a>
## into_alternatives

`function` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::into_alternatives` · datafusion-physical-expr-common 55.1.0

```rust
fn into_alternatives(self) -> (Vec<LexRequirement>, bool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [828, 1], "end": [886, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:880`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns all alternatives as a vector of `LexRequirement` objects and a
boolean value indicating softness/hardness of the requirements.

<a id="op-e38abfb1b0015433da5ef2ee"></a>
## into_single

`function` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::into_single` · datafusion-physical-expr-common 55.1.0

```rust
fn into_single(self) -> LexRequirement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [828, 1], "end": [886, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:864`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns the first (i.e. most preferred) `LexRequirement` among
alternative requirements.

<a id="op-72cf57deb3fffcc7aeadf53f"></a>
## new

`function` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(requirement: LexRequirement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [828, 1], "end": [886, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:846`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Creates a new instance with a single hard requirement.

<a id="op-a265afd743546d5b0b27a428"></a>
## new_alternatives

`function` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::new_alternatives` · datafusion-physical-expr-common 55.1.0

```rust
fn new_alternatives(alternatives: impl IntoIterator<Item = LexRequirement>, soft: bool) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [828, 1], "end": [886, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:831`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Creates a new instance from the given alternatives. If an empty list of
alternatives are given, returns `None`.

<a id="op-87fcc61eb013e805a8e6d342"></a>
## new_soft

`function` · `datafusion_physical_expr_common::sort_expr::OrderingRequirements::new_soft` · datafusion-physical-expr-common 55.1.0

```rust
fn new_soft(requirement: LexRequirement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::OrderingRequirements", "path": "OrderingRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [828, 1], "end": [886, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:851`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Creates a new instance with a single soft requirement.
