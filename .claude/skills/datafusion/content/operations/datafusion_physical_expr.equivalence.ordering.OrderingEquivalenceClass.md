# `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.equivalence.ordering.OrderingEquivalenceClass.json).

<a id="op-dc5b63b5b53d499bc6f0b33a"></a>
## OrderingEquivalenceClass

`struct` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass` · datafusion-physical-expr 55.1.0

```rust
struct OrderingEquivalenceClass
```

Source: `src/equivalence/ordering.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

An `OrderingEquivalenceClass` keeps track of distinct alternative orderings
than can describe a table. For example, consider the following table:

```text
┌───┬───┬───┬───┐
│ a │ b │ c │ d │
├───┼───┼───┼───┤
│ 1 │ 4 │ 3 │ 1 │
│ 2 │ 3 │ 3 │ 2 │
│ 3 │ 1 │ 2 │ 2 │
│ 3 │ 2 │ 1 │ 3 │
└───┴───┴───┴───┘
```

Here, both `[a ASC, b ASC]` and `[c DESC, d ASC]` describe the table
ordering. In this case, we say that these orderings are equivalent.

An `OrderingEquivalenceClass` is a set of such equivalent orderings, which
is represented by a vector of `LexOrdering`s. The set does not store any
redundant information by enforcing the invariant that no suffix of an
ordering in the equivalence class is a prefix of another ordering in the
equivalence class. The set can be empty, which means that there are no
orderings that describe the table.

<a id="op-bc863b1550133e7145bf52e6"></a>
## IntoIter

`assoc_type` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::IntoIter` · datafusion-physical-expr 55.1.0

```rust
IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [294, 1], "end": [301, 2], "filename": "src/equivalence/ordering.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/equivalence/ordering.rs:296`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f32b7276fbcec1846d67f7ad"></a>
## Item

`assoc_type` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::Item` · datafusion-physical-expr 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [294, 1], "end": [301, 2], "filename": "src/equivalence/ordering.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/equivalence/ordering.rs:295`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5067bfb15e2c22ffbf60cff5"></a>
## Target

`assoc_type` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::Target` · datafusion-physical-expr 55.1.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [277, 1], "end": [283, 2], "filename": "src/equivalence/ordering.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/equivalence/ordering.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fefb02b8031b2dc3192d2380"></a>
## add_offset

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::add_offset` · datafusion-physical-expr 55.1.0

```rust
fn add_offset(&mut self, offset: isize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [275, 2], "filename": "src/equivalence/ordering.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/ordering.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Adds `offset` value to the index of each expression inside this
ordering equivalence class.

<a id="op-a6724b73790fc19d4581bad4"></a>
## add_orderings

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::add_orderings` · datafusion-physical-expr 55.1.0

```rust
fn add_orderings(&mut self, sort_exprs: impl IntoIterator<Item = impl IntoIterator<Item = PhysicalSortExpr>>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [275, 2], "filename": "src/equivalence/ordering.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/ordering.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Adds new orderings into this ordering equivalence class.

<a id="op-67b32b396b89ccd661b4a0bb"></a>
## clear

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::clear` · datafusion-physical-expr 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [275, 2], "filename": "src/equivalence/ordering.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/ordering.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Clears (empties) this ordering equivalence class.

<a id="op-0bb502f79dc1cd6ec9d7cde2"></a>
## clone

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> OrderingEquivalenceClass
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 10], "end": [54, 15], "filename": "src/equivalence/ordering.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/equivalence/ordering.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36dd8a2e32f2c998cb5cf1df"></a>
## default

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::default` · datafusion-physical-expr 55.1.0

```rust
fn default() -> OrderingEquivalenceClass
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 24], "end": [54, 31], "filename": "src/equivalence/ordering.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/equivalence/ordering.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b08abd1cfb1d162092fd624d"></a>
## deref

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::deref` · datafusion-physical-expr 55.1.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [277, 1], "end": [283, 2], "filename": "src/equivalence/ordering.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/equivalence/ordering.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a51640971abcc081c463104b"></a>
## eq

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &OrderingEquivalenceClass) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 37], "end": [54, 46], "filename": "src/equivalence/ordering.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/equivalence/ordering.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29548baeeb26c4776be972b4"></a>
## extend

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::extend` · datafusion-physical-expr 55.1.0

```rust
fn extend(&mut self, orderings: impl IntoIterator<Item = LexOrdering>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [275, 2], "filename": "src/equivalence/ordering.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/ordering.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Extend this ordering equivalence class with the given orderings.

<a id="op-a0b82ee61c3f0414e6feca4e"></a>
## fmt

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [303, 1], "end": [315, 2], "filename": "src/equivalence/ordering.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/equivalence/ordering.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd654d0dc2b55b1de476e1ee"></a>
## fmt

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 17], "end": [54, 22], "filename": "src/equivalence/ordering.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/equivalence/ordering.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-113ef73351504fb0f5014655"></a>
## from

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::from` · datafusion-physical-expr 55.1.0

```rust
fn from(eq_properties: EquivalenceProperties) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "crate::equivalence::OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1371, 1], "end": [1375, 2], "filename": "src/equivalence/properties/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::properties::EquivalenceProperties", "path": "EquivalenceProperties"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/equivalence/properties/mod.rs:1372`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16b2e7d88a6957170117f766"></a>
## from

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::from` · datafusion-physical-expr 55.1.0

```rust
fn from(orderings: Vec<LexOrdering>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [291, 2], "filename": "src/equivalence/ordering.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::LexOrdering", "path": "LexOrdering"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/equivalence/ordering.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7039df947e990c093aafd830"></a>
## get_options

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::get_options` · datafusion-physical-expr 55.1.0

```rust
fn get_options(&self, expr: &Arc<dyn PhysicalExpr>) -> Option<SortOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [275, 2], "filename": "src/equivalence/ordering.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/ordering.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Gets sort options associated with this expression if it is a leading
ordering expression. Otherwise, returns `None`.

<a id="op-236edb1bde987d265b3ca91d"></a>
## into_iter

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::into_iter` · datafusion-physical-expr 55.1.0

```rust
fn into_iter(self) -> Self::IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [294, 1], "end": [301, 2], "filename": "src/equivalence/ordering.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/equivalence/ordering.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8e84fdc692d3fb0d9d11c31"></a>
## is_expr_partial_const

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::is_expr_partial_const` · datafusion-physical-expr 55.1.0

```rust
fn is_expr_partial_const(&self, expr: &Arc<dyn PhysicalExpr>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [275, 2], "filename": "src/equivalence/ordering.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/ordering.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Checks whether the given expression is partially constant according to
this ordering equivalence class.

This function determines whether `expr` appears in at least one combination
of `descending` and `nulls_first` options that indicate partial constantness
in a lexicographical ordering. Specifically, an expression is considered
a partial constant in this context if its `SortOptions` satisfies either
of the following conditions:
- It is `descending` with `nulls_first` and _also_ `ascending` with
  `nulls_last`, OR
- It is `descending` with `nulls_last` and _also_ `ascending` with
  `nulls_first`.

The equivalence mechanism primarily uses `ConstExpr`s to represent globally
constant expressions. However, some expressions may only be partially
constant within a lexicographical ordering. This function helps identify
such cases. If an expression is constant within a prefix ordering, it is
added as a constant during `ordering_satisfy_requirement()` iterations
after the corresponding prefix requirement is satisfied.

### Future Improvements

This function may become unnecessary if any of the following improvements
are implemented:
1. `SortOptions` supports encoding constantness information.
2. `EquivalenceProperties` gains `FunctionalDependency` awareness, eliminating
   the need for `Constant` and `Constraints`.

<a id="op-44595fc9239066780c0c6e0e"></a>
## join_suffix

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::join_suffix` · datafusion-physical-expr 55.1.0

```rust
fn join_suffix(self, other: &Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [275, 2], "filename": "src/equivalence/ordering.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/ordering.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-165ef815bb4b5ed21a40bcb8"></a>
## new

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::new` · datafusion-physical-expr 55.1.0

```rust
fn new(orderings: impl IntoIterator<Item = impl IntoIterator<Item = PhysicalSortExpr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [275, 2], "filename": "src/equivalence/ordering.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/ordering.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Creates a new ordering equivalence class from the given orderings
and removes any redundant entries (if given).

<a id="op-ad1c1224786f5897831b74a0"></a>
## output_ordering

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::output_ordering` · datafusion-physical-expr 55.1.0

```rust
fn output_ordering(&self) -> Option<LexOrdering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [275, 2], "filename": "src/equivalence/ordering.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/ordering.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the concatenation of all the orderings. This enables merge
operations to preserve all equivalent orderings simultaneously.

<a id="op-57ebacd2c2418af6c843d375"></a>
## with_new_schema

`function` · `datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass::with_new_schema` · datafusion-physical-expr 55.1.0

```rust
fn with_new_schema(self, schema: &SchemaRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::ordering::OrderingEquivalenceClass", "path": "OrderingEquivalenceClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [275, 2], "filename": "src/equivalence/ordering.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/ordering.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Transforms this `OrderingEquivalenceClass` by mapping columns in the
original schema to columns in the new schema by index. The new schema
and the original schema needs to be aligned; i.e. they should have the
same number of columns, and fields at the same index have the same type
in both schemas.
