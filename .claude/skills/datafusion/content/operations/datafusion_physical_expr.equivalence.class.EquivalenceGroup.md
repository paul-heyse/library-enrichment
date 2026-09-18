# `datafusion_physical_expr::equivalence::class::EquivalenceGroup`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.equivalence.class.EquivalenceGroup.json).

<a id="op-94b5e9660effa79ac8debd32"></a>
## EquivalenceGroup

`struct` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup` · datafusion-physical-expr 55.1.0

```rust
struct EquivalenceGroup
```

Source: `src/equivalence/class.rs:305`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

A collection of distinct `EquivalenceClass`es. This object supports fast
lookups of expressions and their equivalence classes.

<a id="op-487e94daa3ce77a6f762aab7"></a>
## IntoIter

`assoc_type` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::IntoIter` · datafusion-physical-expr 55.1.0

```rust
IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [882, 1], "end": [889, 2], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/equivalence/class.rs:884`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42af60befb5f413a1b9fea78"></a>
## Item

`assoc_type` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::Item` · datafusion-physical-expr 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [882, 1], "end": [889, 2], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/equivalence/class.rs:883`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-744d6cbc417666919b701b6f"></a>
## Target

`assoc_type` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::Target` · datafusion-physical-expr 55.1.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [874, 1], "end": [880, 2], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/equivalence/class.rs:875`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-733c635235dc9ec82a275593"></a>
## add_constant

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::add_constant` · datafusion-physical-expr 55.1.0

```rust
fn add_constant(&mut self, const_expr: ConstExpr)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:319`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Adds `expr` as a constant expression to this equivalence group.

<a id="op-b80a74a8f1dcb25ad417a7ca"></a>
## add_equal_conditions

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::add_equal_conditions` · datafusion-physical-expr 55.1.0

```rust
fn add_equal_conditions(&mut self, left: Arc<dyn PhysicalExpr>, right: Arc<dyn PhysicalExpr>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:384`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Adds the equality `left` = `right` to this equivalence group. New
equality conditions often arise after steps like `Filter(a = b)`,
`Alias(a, a as b)` etc. Returns whether the given equality defines
a new equivalence class.

<a id="op-d25dfcd220a592fa510395ed"></a>
## clear_per_partition_constants

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::clear_per_partition_constants` · datafusion-physical-expr 55.1.0

```rust
fn clear_per_partition_constants(&mut self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Removes constant expressions that may change across partitions.
This method should be used when merging data from different partitions.
Returns whether any change was made to the equivalence group.

<a id="op-e7f128af0a576b5986a3c365"></a>
## clone

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> EquivalenceGroup
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 10], "end": [304, 15], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/equivalence/class.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25f44e9b6775805646f43b78"></a>
## default

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::default` · datafusion-physical-expr 55.1.0

```rust
fn default() -> EquivalenceGroup
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 24], "end": [304, 31], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/equivalence/class.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c40a787896bde9c7b2a56412"></a>
## deref

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::deref` · datafusion-physical-expr 55.1.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [874, 1], "end": [880, 2], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/equivalence/class.rs:877`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ac589076b80d83a190dd059"></a>
## exprs_equal

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::exprs_equal` · datafusion-physical-expr 55.1.0

```rust
fn exprs_equal(&self, left: &Arc<dyn PhysicalExpr>, right: &Arc<dyn PhysicalExpr>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:823`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Checks if two expressions are equal directly or through equivalence
classes. For complex expressions (e.g. `a + b`), checks that the
expression trees are structurally identical and their leaf nodes are
equivalent either directly or through equivalence classes.

<a id="op-30e2b548813fbe3ac7cd0e61"></a>
## extend

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::extend` · datafusion-physical-expr 55.1.0

```rust
fn extend(&mut self, other: Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:501`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Extends this equivalence group with the `other` equivalence group.
Returns whether any equivalence classes were unified/bridged as a
result of the extension process.

<a id="op-d63a9d1f8146a728ac50ec2f"></a>
## fmt

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 17], "end": [304, 22], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/equivalence/class.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e57ea35d1c6b382a932b2213"></a>
## fmt

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 1], "end": [903, 2], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/equivalence/class.rs:892`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-330ad4d5cf8b2992daf169ad"></a>
## from

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::from` · datafusion-physical-expr 55.1.0

```rust
fn from(classes: Vec<EquivalenceClass>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [905, 1], "end": [920, 2], "filename": "src/equivalence/class.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceClass", "path": "EquivalenceClass"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/equivalence/class.rs:906`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52749f269d90d83acb2daa42"></a>
## get_equivalence_class

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::get_equivalence_class` · datafusion-physical-expr 55.1.0

```rust
fn get_equivalence_class(&self, expr: &Arc<dyn PhysicalExpr>) -> Option<&EquivalenceClass>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:773`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the equivalence class containing `expr`. If no equivalence class
contains `expr`, returns `None`.

<a id="op-45156bd40e3e353ee04be0b4"></a>
## into_iter

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::into_iter` · datafusion-physical-expr 55.1.0

```rust
fn into_iter(self) -> Self::IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [882, 1], "end": [889, 2], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/equivalence/class.rs:886`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-062aacdce0957157ab262996"></a>
## is_expr_constant

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::is_expr_constant` · datafusion-physical-expr 55.1.0

```rust
fn is_expr_constant(&self, expr: &Arc<dyn PhysicalExpr>) -> Option<AcrossPartitions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:746`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns a `Some` value if the expression is constant according to
equivalence group, and `None` otherwise. The `Some` variant contains
an `AcrossPartitions` value indicating whether the expression is
constant across partitions, and its actual value (if available).

<a id="op-b1cdc8cae1b32132de3bb775"></a>
## join

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::join` · datafusion-physical-expr 55.1.0

```rust
fn join(&self, right_equivalences: &Self, join_type: &JoinType, left_size: usize, on: &[(PhysicalExprRef, PhysicalExprRef)]) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:781`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Combine equivalence groups of the given join children.

<a id="op-4949085fd34b35887a1ddece"></a>
## new

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::new` · datafusion-physical-expr 55.1.0

```rust
fn new(classes: impl IntoIterator<Item = EquivalenceClass>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:314`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Creates an equivalence group from the given equivalence classes.

<a id="op-0c07497fa712273f13897daf"></a>
## normalize_expr

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::normalize_expr` · datafusion-physical-expr 55.1.0

```rust
fn normalize_expr(&self, expr: Arc<dyn PhysicalExpr>) -> Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:513`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Normalizes the given physical expression according to this group. The
expression is replaced with the first (canonical) expression in the
equivalence class it matches with (if any).

<a id="op-88913f8c10c89a79ed691015"></a>
## normalize_sort_expr

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::normalize_sort_expr` · datafusion-physical-expr 55.1.0

```rust
fn normalize_sort_expr(&self, sort_expr: PhysicalSortExpr) -> PhysicalSortExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:531`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Normalizes the given sort expression according to this group. The
underlying physical expression is replaced with the first expression in
the equivalence class it matches with (if any). If the underlying
expression does not belong to any equivalence class in this group,
returns the sort expression as is.

<a id="op-bd759896b3c2a5466d04ab77"></a>
## normalize_sort_exprs

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::normalize_sort_exprs` · datafusion-physical-expr 55.1.0

```rust
fn normalize_sort_exprs<'a>(&'a self, sort_exprs: impl IntoIterator<Item = PhysicalSortExpr> + 'a) -> impl Iterator<Item = PhysicalSortExpr> + 'a
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:547`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Normalizes the given sort expressions (i.e. `sort_exprs`) by:
- Replacing sections that belong to some equivalence class in the
  with the first entry in the matching equivalence class.
- Removing expressions that have a constant value.

If columns `a` and `b` are known to be equal, `d` is known to be a
constant, and `sort_exprs` is `[b ASC, d DESC, c ASC, a ASC]`, this
function would return `[a ASC, c ASC, a ASC]`.

<a id="op-9dd36e8073d7d2f417d06d00"></a>
## normalize_sort_requirement

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::normalize_sort_requirement` · datafusion-physical-expr 55.1.0

```rust
fn normalize_sort_requirement(&self, sort_requirement: PhysicalSortRequirement) -> PhysicalSortRequirement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:574`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Normalizes the given sort requirement according to this group. The
underlying physical expression is replaced with the first expression in
the equivalence class it matches with (if any). If the underlying
expression does not belong to any equivalence class in this group,
returns the given sort requirement as is.

<a id="op-4afc65123b88da4e3295e095"></a>
## normalize_sort_requirements

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::normalize_sort_requirements` · datafusion-physical-expr 55.1.0

```rust
fn normalize_sort_requirements<'a>(&'a self, sort_reqs: impl IntoIterator<Item = PhysicalSortRequirement> + 'a) -> impl Iterator<Item = PhysicalSortRequirement> + 'a
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:590`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Normalizes the given sort requirements (i.e. `sort_reqs`) by:
- Replacing sections that belong to some equivalence class in the
  with the first entry in the matching equivalence class.
- Removing expressions that have a constant value.

If columns `a` and `b` are known to be equal, `d` is known to be a
constant, and `sort_reqs` is `[b ASC, d DESC, c ASC, a ASC]`, this
function would return `[a ASC, c ASC, a ASC]`.

<a id="op-50e65ec05bfd993ee2d685c3"></a>
## project

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::project` · datafusion-physical-expr 55.1.0

```rust
fn project(&self, mapping: &ProjectionMapping) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:700`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Projects this equivalence group according to the given projection mapping.

<a id="op-aca8c1468f4f000bcedc77ef"></a>
## project_expr

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::project_expr` · datafusion-physical-expr 55.1.0

```rust
fn project_expr(&self, mapping: &ProjectionMapping, expr: &Arc<dyn PhysicalExpr>) -> Option<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:661`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Projects `expr` according to the given projection mapping.
If the resulting expression is invalid after projection, returns `None`.

<a id="op-3d67c340972c47f04c9a5766"></a>
## project_expressions

`function` · `datafusion_physical_expr::equivalence::class::EquivalenceGroup::project_expressions` · datafusion-physical-expr 55.1.0

```rust
fn project_expressions<'a>(&'a self, mapping: &'a ProjectionMapping, expressions: impl IntoIterator<Item = &'a Arc<dyn PhysicalExpr>> + 'a) -> impl Iterator<Item = Option<Arc<dyn PhysicalExpr>>> + 'a
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::EquivalenceGroup", "path": "EquivalenceGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [872, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:680`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Projects `expressions` according to the given projection mapping.
This function is similar to [`Self::project_expr`](../operations/datafusion_physical_expr.equivalence.class.EquivalenceGroup.md#op-aca8c1468f4f000bcedc77ef), but projects multiple
expressions at once more efficiently than calling `project_expr` for each
expression.
