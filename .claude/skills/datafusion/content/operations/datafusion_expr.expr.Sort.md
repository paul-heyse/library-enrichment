# `datafusion_expr::expr::Sort`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.Sort.json).

<a id="op-db804608f982bce14d75f784"></a>
## Sort

`struct` · `datafusion_expr::expr::Sort` · datafusion-expr 55.1.0

```rust
struct Sort
```

Source: `src/expr.rs:1035`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

SORT expression

<a id="op-374c70ab375e85b23f19acd8"></a>
## apply_elements

`function` · `datafusion_expr::expr::Sort::apply_elements` · datafusion-expr 55.1.0

```rust
fn apply_elements<F: FnMut(&'a Expr) -> Result<TreeNodeRecursion>>(&'a self, f: F) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Sort", "path": "Sort"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1090, 1], "end": [1106, 2], "filename": "src/expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeContainer", "path": "TreeNodeContainer"}, "trait_path": "datafusion_common::tree_node::TreeNodeContainer"}`

Source: `src/expr.rs:1091`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5306e187e2b8c1afe1bd5ae"></a>
## asc

`struct_field` · `datafusion_expr::expr::Sort::asc` · datafusion-expr 55.1.0

```rust
asc: bool
```

Source: `src/expr.rs:1039`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The direction of the sort

<a id="op-7818c8ce1578d80f19e8ff42"></a>
## clone

`function` · `datafusion_expr::expr::Sort::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Sort
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Sort", "path": "Sort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1034, 10], "end": [1034, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1034`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e301db4792fe297f92321022"></a>
## eq

`function` · `datafusion_expr::expr::Sort::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Sort) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Sort", "path": "Sort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1034, 17], "end": [1034, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1034`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d164a271c585581e3f0bb05"></a>
## expr

`struct_field` · `datafusion_expr::expr::Sort::expr` · datafusion-expr 55.1.0

```rust
expr: Expr
```

Source: `src/expr.rs:1037`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The expression to sort on

<a id="op-78e3a79cb5b9e642c76f0f20"></a>
## fmt

`function` · `datafusion_expr::expr::Sort::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Sort", "path": "Sort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1073, 1], "end": [1088, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expr.rs:1074`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df7db25dcbd83069b5d085a0"></a>
## fmt

`function` · `datafusion_expr::expr::Sort::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Sort", "path": "Sort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1034, 50], "end": [1034, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1034`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ec1bfc2c5b5560c340a14cf"></a>
## hash

`function` · `datafusion_expr::expr::Sort::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Sort", "path": "Sort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1034, 44], "end": [1034, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1034`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70b29a2bf93c077ae05a63c7"></a>
## map_elements

`function` · `datafusion_expr::expr::Sort::map_elements` · datafusion-expr 55.1.0

```rust
fn map_elements<F: FnMut(Expr) -> Result<Transformed<Expr>>>(self, f: F) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Sort", "path": "Sort"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1090, 1], "end": [1106, 2], "filename": "src/expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeContainer", "path": "TreeNodeContainer"}, "trait_path": "datafusion_common::tree_node::TreeNodeContainer"}`

Source: `src/expr.rs:1098`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a984239f07a080ca512513f"></a>
## new

`function` · `datafusion_expr::expr::Sort::new` · datafusion-expr 55.1.0

```rust
fn new(expr: Expr, asc: bool, nulls_first: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Sort", "path": "Sort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1044, 1], "end": [1071, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1046`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Sort expression

<a id="op-28a9d4e1a3be2691d5bb13be"></a>
## nulls_first

`struct_field` · `datafusion_expr::expr::Sort::nulls_first` · datafusion-expr 55.1.0

```rust
nulls_first: bool
```

Source: `src/expr.rs:1041`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether to put Nulls before all other data values

<a id="op-ee429887366977a230c2a755"></a>
## partial_cmp

`function` · `datafusion_expr::expr::Sort::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Sort) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Sort", "path": "Sort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1034, 32], "end": [1034, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1034`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d41360aac9e5e30df5149b0"></a>
## reverse

`function` · `datafusion_expr::expr::Sort::reverse` · datafusion-expr 55.1.0

```rust
fn reverse(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Sort", "path": "Sort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1044, 1], "end": [1071, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1055`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Sort expression with the opposite sort direction

<a id="op-d952d0cda5a089b0b58ac33a"></a>
## with_expr

`function` · `datafusion_expr::expr::Sort::with_expr` · datafusion-expr 55.1.0

```rust
fn with_expr(&self, expr: Expr) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Sort", "path": "Sort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1044, 1], "end": [1071, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1064`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Replaces the Sort expressions with `expr`
