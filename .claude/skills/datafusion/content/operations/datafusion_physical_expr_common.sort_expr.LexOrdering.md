# `datafusion_physical_expr_common::sort_expr::LexOrdering`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.sort_expr.LexOrdering.json).

<a id="op-d19b6df9e9a4be59c4b4dfd1"></a>
## LexOrdering

`struct` · `datafusion_physical_expr_common::sort_expr::LexOrdering` · datafusion-physical-expr-common 55.1.0

```rust
struct LexOrdering
```

Source: `src/sort_expr.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

This object represents a lexicographical ordering and contains a vector
of `PhysicalSortExpr` objects.

For example, a `vec![a ASC, b DESC]` represents a lexicographical ordering
that first sorts by column `a` in ascending order, then by column `b` in
descending order.

# Invariants

The following always hold true for a `LexOrdering`:

1. It is non-degenerate, meaning it contains at least one element.
2. It is duplicate-free, meaning it does not contain multiple entries for
   the same column.

<a id="op-54bb8c60146856d3a7229aeb"></a>
## IntoIter

`assoc_type` · `datafusion_physical_expr_common::sort_expr::LexOrdering::IntoIter` · datafusion-physical-expr-common 55.1.0

```rust
IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [671, 1], "end": [678, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/sort_expr.rs:673`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2035eb2ba135f7047eba53c"></a>
## Item

`assoc_type` · `datafusion_physical_expr_common::sort_expr::LexOrdering::Item` · datafusion-physical-expr-common 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [671, 1], "end": [678, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/sort_expr.rs:672`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6e27839c2b22784b74ebe37"></a>
## Target

`assoc_type` · `datafusion_physical_expr_common::sort_expr::LexOrdering::Target` · datafusion-physical-expr-common 55.1.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [648, 1], "end": [654, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/sort_expr.rs:649`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e852b0b544fa64e1bac07e44"></a>
## capacity

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::capacity` · datafusion-physical-expr-common 55.1.0

```rust
fn capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [586, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:525`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns the number of elements that can be stored in the `LexOrdering`
without reallocating.

<a id="op-c61de88f13618b79932c9e78"></a>
## clone

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> LexOrdering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 10], "end": [471, 15], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sort_expr.rs:471`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e51a94664d8c2ed6e444a8cb"></a>
## deref

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::deref` · datafusion-physical-expr-common 55.1.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [648, 1], "end": [654, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/sort_expr.rs:651`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef989ffaefd492251d1fcda4"></a>
## eq

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::eq` · datafusion-physical-expr-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [622, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sort_expr.rs:614`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d7b8409e8262ec615a6f830"></a>
## extend

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::extend` · datafusion-physical-expr-common 55.1.0

```rust
fn extend(&mut self, sort_exprs: impl IntoIterator<Item = PhysicalSortExpr>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [586, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:509`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add all elements from `iter` to the `LexOrdering`.

<a id="op-6d3a4da93744ba2ab024cfc9"></a>
## first

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::first` · datafusion-physical-expr-common 55.1.0

```rust
fn first(&self) -> &PhysicalSortExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [586, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:518`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns the leading `PhysicalSortExpr` of the `LexOrdering`. Note that
this function does not return an `Option`, as a `LexOrdering` is always
non-degenerate (i.e. it contains at least one element).

<a id="op-1baf376d98f5a8d52b57faa9"></a>
## fmt

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 17], "end": [471, 22], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sort_expr.rs:471`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ae0f52d48c2b57c370f1747"></a>
## fmt

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [656, 1], "end": [669, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/sort_expr.rs:657`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-530b06025a1a6c82ba7d188f"></a>
## from

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::from` · datafusion-physical-expr-common 55.1.0

```rust
fn from(value: [PhysicalSortExpr; N]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "usize"}}}, "name": "N"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [638, 1], "end": [646, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"array": {"len": "N", "type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/sort_expr.rs:639`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb035bba40210dccdea0bada"></a>
## from

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::from` · datafusion-physical-expr-common 55.1.0

```rust
fn from(value: LexRequirement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [799, 1], "end": [805, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexRequirement", "path": "LexRequirement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/sort_expr.rs:800`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eab77e7529315b16c7dc0e34"></a>
## get_sort_options

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::get_sort_options` · datafusion-physical-expr-common 55.1.0

```rust
fn get_sort_options(&self, expr: &dyn PhysicalExpr) -> Option<SortOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [586, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:577`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns the sort options for the given expression if one is defined in this `LexOrdering`.

<a id="op-ff9e949f0635822ee013356d"></a>
## into_iter

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::into_iter` · datafusion-physical-expr-common 55.1.0

```rust
fn into_iter(self) -> Self::IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [671, 1], "end": [678, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/sort_expr.rs:675`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f928852e6f4f65a00a74f11"></a>
## is_reverse

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::is_reverse` · datafusion-physical-expr-common 55.1.0

```rust
fn is_reverse(&self, other: &LexOrdering) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [586, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:563`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Check if reversing this ordering would satisfy another ordering requirement.

This supports **prefix matching**: if this ordering is `[A DESC, B ASC]`
and `other` is `[A ASC]`, reversing this gives `[A ASC, B DESC]`, which
satisfies `other` since `[A ASC]` is a prefix.

# Arguments
* `other` - The ordering requirement to check against

# Returns
`true` if reversing this ordering would satisfy `other`

# Example
```text
self:  [number DESC, letter ASC]
other: [number ASC]
After reversing self: [number ASC, letter DESC]  ✓ Prefix match!
```

<a id="op-62b4c227fcd3efc3f842a866"></a>
## new

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(exprs: impl IntoIterator<Item = PhysicalSortExpr>) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [586, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Creates a new [`LexOrdering`](../operations/datafusion_physical_expr_common.sort_expr.LexOrdering.md#op-d19b6df9e9a4be59c4b4dfd1) from the given vector of sort expressions.
If the vector is empty, returns `None`.

<a id="op-497283cb1c2dd8253e77bea8"></a>
## partial_cmp

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::partial_cmp` · datafusion-physical-expr-common 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [624, 1], "end": [636, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sort_expr.rs:628`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

There is a partial ordering among `LexOrdering` objects. For example, the
ordering `[a ASC]` is coarser (less) than ordering `[a ASC, b ASC]`.
If two orderings do not share a prefix, they are incomparable.

<a id="op-659186d3f28c0c140bb38be0"></a>
## push

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::push` · datafusion-physical-expr-common 55.1.0

```rust
fn push(&mut self, sort_expr: PhysicalSortExpr)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [586, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:502`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Appends an element to the back of the `LexOrdering`.

<a id="op-4f78be7a34d9cfa597eef34f"></a>
## truncate

`function` · `datafusion_physical_expr_common::sort_expr::LexOrdering::truncate` · datafusion-physical-expr-common 55.1.0

```rust
fn truncate(&mut self, len: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [586, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Truncates the `LexOrdering`, keeping only the first `len` elements.
Returns `true` if truncation made a change, `false` otherwise. Negative
cases happen in two scenarios: (1) When `len` is greater than or equal
to the number of expressions inside this `LexOrdering`, making truncation
a no-op, or (2) when `len` is `0`, making truncation impossible.
