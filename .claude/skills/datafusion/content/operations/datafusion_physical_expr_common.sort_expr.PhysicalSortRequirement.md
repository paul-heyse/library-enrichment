# `datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.sort_expr.PhysicalSortRequirement.json).

<a id="op-37d32c622d1539a2c554e300"></a>
## PhysicalSortRequirement

`struct` · `datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement` · datafusion-physical-expr-common 55.1.0

```rust
struct PhysicalSortRequirement
```

Source: `src/sort_expr.rs:358`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Represents sort requirement associated with a plan

If the requirement includes [`SortOptions`](../operations/arrow_schema.SortOptions.md#op-78d98c3e0c6da432658d0949) then both the
expression *and* the sort options must match.

If the requirement does not include [`SortOptions`](../operations/arrow_schema.SortOptions.md#op-78d98c3e0c6da432658d0949)) then only the
expressions must match.

# Examples

With sort options (`A`, `DESC NULLS FIRST`):
* `ORDER BY A DESC NULLS FIRST` matches
* `ORDER BY A ASC  NULLS FIRST` does not match (`ASC` vs `DESC`)
* `ORDER BY B DESC NULLS FIRST` does not match (different expr)

Without sort options (`A`, None):
* `ORDER BY A DESC NULLS FIRST` matches
* `ORDER BY A ASC  NULLS FIRST` matches (`ASC` and `NULL` options ignored)
* `ORDER BY B DESC NULLS FIRST` does not match  (different expr)

<a id="op-278ecf4318ab0732b97415ff"></a>
## clone

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> PhysicalSortRequirement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement", "path": "PhysicalSortRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 10], "end": [357, 15], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sort_expr.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-984cd3463bdfaebbb9398a01"></a>
## compatible

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement::compatible` · datafusion-physical-expr-common 55.1.0

```rust
fn compatible(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement", "path": "PhysicalSortRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [424, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:418`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns whether this requirement is equal or more specific than `other`.

<a id="op-4507a9ec547d7a29af505ee9"></a>
## eq

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement::eq` · datafusion-physical-expr-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement", "path": "PhysicalSortRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [366, 1], "end": [370, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sort_expr.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-531c108eec6b16ca7fe39d9f"></a>
## expr

`struct_field` · `datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement::expr` · datafusion-physical-expr-common 55.1.0

```rust
expr: std::sync::Arc<dyn PhysicalExpr>
```

Source: `src/sort_expr.rs:360`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Physical expression representing the column to sort

<a id="op-9f0575d08691b8221aba4306"></a>
## fmt

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement", "path": "PhysicalSortRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 17], "end": [357, 22], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sort_expr.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b48b914d591ef57375c38fb4"></a>
## fmt

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement", "path": "PhysicalSortRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [372, 1], "end": [377, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/sort_expr.rs:373`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-149b5c7989879c7d096ca37b"></a>
## from

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement::from` · datafusion-physical-expr-common 55.1.0

```rust
fn from(value: PhysicalSortExpr) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement", "path": "PhysicalSortRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [438, 1], "end": [442, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/sort_expr.rs:439`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4754ed34af2b71b9189476e8"></a>
## new

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(expr: Arc<dyn PhysicalExpr>, options: Option<SortOptions>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement", "path": "PhysicalSortRequirement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [424, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:413`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Creates a new requirement.

If `options` is `Some(..)`, creates an `exact` requirement,
which must match both `options` and `expr`.

If `options` is `None`, Creates a new `expr_only` requirement,
which must match only `expr`.

See [`PhysicalSortRequirement`](../operations/datafusion_physical_expr_common.sort_expr.PhysicalSortRequirement.md#op-37d32c622d1539a2c554e300) for examples.

<a id="op-07ae2864beb09072271e9068"></a>
## options

`struct_field` · `datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement::options` · datafusion-physical-expr-common 55.1.0

```rust
options: Option<arrow::compute::kernels::sort::SortOptions>
```

Source: `src/sort_expr.rs:363`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Option to specify how the given column should be sorted.
If unspecified, there are no constraints on sort options.
