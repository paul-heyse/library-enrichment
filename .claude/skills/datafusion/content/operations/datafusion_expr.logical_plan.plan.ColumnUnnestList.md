# `datafusion_expr::logical_plan::plan::ColumnUnnestList`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.ColumnUnnestList.json).

<a id="op-7a5857553a708c71d2fe8e36"></a>
## ColumnUnnestList

`struct` · `datafusion_expr::logical_plan::plan::ColumnUnnestList` · datafusion-expr 55.1.0

```rust
struct ColumnUnnestList
```

Source: `src/logical_plan/plan.rs:4611`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Represent the unnesting operation on a list column, such as the recursion depth and
the output column name after unnesting

Example: given `ColumnUnnestList { output_column: "output_name", depth: 2 }`

```text
  input             output_name
 ┌─────────┐      ┌─────────┐
 │{{1,2}}  │      │ 1       │
 ├─────────┼─────►├─────────┤
 │{{3}}    │      │ 2       │
 ├─────────┤      ├─────────┤
 │{{4},{5}}│      │ 3       │
 └─────────┘      ├─────────┤
                  │ 4       │
                  ├─────────┤
                  │ 5       │
                  └─────────┘
```

<a id="op-52120ae5c69f8d411b510f08"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::ColumnUnnestList::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> ColumnUnnestList
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ColumnUnnestList", "path": "ColumnUnnestList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4610, 17], "end": [4610, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:4610`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04389448f25b9f32a8695c17"></a>
## depth

`struct_field` · `datafusion_expr::logical_plan::plan::ColumnUnnestList::depth` · datafusion-expr 55.1.0

```rust
depth: usize
```

Source: `src/logical_plan/plan.rs:4613`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bd30cb0f6e917487e422a5e"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::ColumnUnnestList::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &ColumnUnnestList) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ColumnUnnestList", "path": "ColumnUnnestList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4610, 24], "end": [4610, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:4610`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5344eef2c035f8b70b8b5704"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::ColumnUnnestList::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ColumnUnnestList", "path": "ColumnUnnestList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4610, 10], "end": [4610, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:4610`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-884ca627ee7d89def1ffe7f7"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::ColumnUnnestList::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ColumnUnnestList", "path": "ColumnUnnestList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4616, 1], "end": [4620, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/logical_plan/plan.rs:4617`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da1ec612a1bfa688f08ad70a"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::ColumnUnnestList::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ColumnUnnestList", "path": "ColumnUnnestList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4610, 39], "end": [4610, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:4610`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b41952a3107ddd75960b3a39"></a>
## output_column

`struct_field` · `datafusion_expr::logical_plan::plan::ColumnUnnestList::output_column` · datafusion-expr 55.1.0

```rust
output_column: datafusion_common::Column
```

Source: `src/logical_plan/plan.rs:4612`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e67c0a5c40f86539e06ba1d6"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::ColumnUnnestList::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &ColumnUnnestList) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ColumnUnnestList", "path": "ColumnUnnestList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4610, 45], "end": [4610, 55], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:4610`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
