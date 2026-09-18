# `datafusion_physical_expr_common::sort_expr::LexRequirement`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.sort_expr.LexRequirement.json).

<a id="op-dbccd31ff3eaea7309fbb53a"></a>
## LexRequirement

`struct` · `datafusion_physical_expr_common::sort_expr::LexRequirement` · datafusion-physical-expr-common 55.1.0

```rust
struct LexRequirement
```

Source: `src/sort_expr.rs:710`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

This object represents a lexicographical ordering requirement and contains
a vector of `PhysicalSortRequirement` objects.

For example, a `vec![a Some(ASC), b None]` represents a lexicographical
requirement that firsts imposes an ordering by column `a` in ascending
order, then by column `b` in *any* (ascending or descending) order. The
ordering is non-degenerate, meaning it contains at least one element, and
it is duplicate-free, meaning it does not contain multiple entries for the
same column.

Note that a `LexRequirement` need not enforce the uniqueness of its sort
expressions after construction like a `LexOrdering` does, because it provides
no mutation methods. If such methods become necessary, we will need to
enforce uniqueness like the latter object.

<a id="op-a44b55cac2b7e9e20a7072b6"></a>
## IntoIter

`assoc_type` · `datafusion_physical_expr_common::sort_expr::LexRequirement::IntoIter` · datafusion-physical-expr-common 55.1.0

```rust
IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexRequirement", "path": "LexRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [764, 1], "end": [771, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/sort_expr.rs:766`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7fdd695048762983ddf212c"></a>
## Item

`assoc_type` · `datafusion_physical_expr_common::sort_expr::LexRequirement::Item` · datafusion-physical-expr-common 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexRequirement", "path": "LexRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [764, 1], "end": [771, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/sort_expr.rs:765`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-667d622ec50f9b24f6b1be8a"></a>
## Target

`assoc_type` · `datafusion_physical_expr_common::sort_expr::LexRequirement::Target` · datafusion-physical-expr-common 55.1.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexRequirement", "path": "LexRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [756, 1], "end": [762, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/sort_expr.rs:757`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb79dc81c14dd06a5c8f420e"></a>
## clone

`function` · `datafusion_physical_expr_common::sort_expr::LexRequirement::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> LexRequirement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexRequirement", "path": "LexRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [709, 17], "end": [709, 22], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sort_expr.rs:709`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eccf6af1eb6fec567025e57f"></a>
## deref

`function` · `datafusion_physical_expr_common::sort_expr::LexRequirement::deref` · datafusion-physical-expr-common 55.1.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexRequirement", "path": "LexRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [756, 1], "end": [762, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/sort_expr.rs:759`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6dd58cdcc17003d585c3d31c"></a>
## eq

`function` · `datafusion_physical_expr_common::sort_expr::LexRequirement::eq` · datafusion-physical-expr-common 55.1.0

```rust
fn eq(&self, other: &LexRequirement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexRequirement", "path": "LexRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [709, 24], "end": [709, 33], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sort_expr.rs:709`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-838dfe0592b4bef5a14aaaa0"></a>
## first

`function` · `datafusion_physical_expr_common::sort_expr::LexRequirement::first` · datafusion-physical-expr-common 55.1.0

```rust
fn first(&self) -> &PhysicalSortRequirement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexRequirement", "path": "LexRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [714, 1], "end": [743, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:725`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns the leading `PhysicalSortRequirement` of the `LexRequirement`.
Note that this function does not return an `Option`, as a `LexRequirement`
is always non-degenerate (i.e. it contains at least one element).

<a id="op-33cc671c719c5a0aceb8f11b"></a>
## fmt

`function` · `datafusion_physical_expr_common::sort_expr::LexRequirement::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexRequirement", "path": "LexRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [709, 10], "end": [709, 15], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sort_expr.rs:709`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0aae00fd1d3c6f8fabb359d7"></a>
## from

`function` · `datafusion_physical_expr_common::sort_expr::LexRequirement::from` · datafusion-physical-expr-common 55.1.0

```rust
fn from(value: LexOrdering) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexRequirement", "path": "LexRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [789, 1], "end": [797, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/sort_expr.rs:790`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2edde240c0fc3495c1da4dd9"></a>
## from

`function` · `datafusion_physical_expr_common::sort_expr::LexRequirement::from` · datafusion-physical-expr-common 55.1.0

```rust
fn from(value: [PhysicalSortRequirement; N]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexRequirement", "path": "LexRequirement"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "usize"}}}, "name": "N"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [745, 1], "end": [754, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"array": {"len": "N", "type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement", "path": "PhysicalSortRequirement"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/sort_expr.rs:746`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-989e0647edcbd09a5a43febf"></a>
## into_iter

`function` · `datafusion_physical_expr_common::sort_expr::LexRequirement::into_iter` · datafusion-physical-expr-common 55.1.0

```rust
fn into_iter(self) -> Self::IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexRequirement", "path": "LexRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [764, 1], "end": [771, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/sort_expr.rs:768`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-452af51ae6f153f592032064"></a>
## new

`function` · `datafusion_physical_expr_common::sort_expr::LexRequirement::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(reqs: impl IntoIterator<Item = PhysicalSortRequirement>) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexRequirement", "path": "LexRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [714, 1], "end": [743, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:717`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Creates a new [`LexRequirement`](../operations/datafusion_physical_expr_common.sort_expr.LexRequirement.md#op-dbccd31ff3eaea7309fbb53a) from the given vector of sort expressions.
If the vector is empty, returns `None`.
