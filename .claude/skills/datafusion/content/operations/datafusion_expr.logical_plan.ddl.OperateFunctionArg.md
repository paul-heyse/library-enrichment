# `datafusion_expr::logical_plan::ddl::OperateFunctionArg`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.ddl.OperateFunctionArg.json).

<a id="op-95a41cf47f93bc3266227b20"></a>
## OperateFunctionArg

`struct` · `datafusion_expr::logical_plan::ddl::OperateFunctionArg` · datafusion-expr 55.1.0

```rust
struct OperateFunctionArg
```

Source: `src/logical_plan/ddl.rs:693`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Part of the `CREATE FUNCTION` statement

See [`CreateFunction`](../operations/datafusion_expr.logical_plan.ddl.CreateFunction.md#op-3668c6565f5901a6967c489f) for details

<a id="op-ee39ccb34fad7747b482c4b6"></a>
## apply_elements

`function` · `datafusion_expr::logical_plan::ddl::OperateFunctionArg::apply_elements` · datafusion-expr 55.1.0

```rust
fn apply_elements<F: FnMut(&'a Expr) -> Result<TreeNodeRecursion>>(&'a self, f: F) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [701, 1], "end": [720, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeContainer", "path": "TreeNodeContainer"}, "trait_path": "datafusion_common::tree_node::TreeNodeContainer"}`

Source: `src/logical_plan/ddl.rs:702`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92aacd4828a535846a67fda0"></a>
## clone

`function` · `datafusion_expr::logical_plan::ddl::OperateFunctionArg::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> OperateFunctionArg
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [692, 10], "end": [692, 15], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/ddl.rs:692`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6edc7bc1e80ba1e3fcff84c"></a>
## data_type

`struct_field` · `datafusion_expr::logical_plan::ddl::OperateFunctionArg::data_type` · datafusion-expr 55.1.0

```rust
data_type: arrow::datatypes::DataType
```

Source: `src/logical_plan/ddl.rs:697`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2498163e986efeef13e70122"></a>
## default_expr

`struct_field` · `datafusion_expr::logical_plan::ddl::OperateFunctionArg::default_expr` · datafusion-expr 55.1.0

```rust
default_expr: Option<Expr>
```

Source: `src/logical_plan/ddl.rs:698`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e08f05cb4e0b400ebd3b501d"></a>
## eq

`function` · `datafusion_expr::logical_plan::ddl::OperateFunctionArg::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &OperateFunctionArg) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [692, 17], "end": [692, 26], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/ddl.rs:692`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3237cb11e34e6cb4468c9e1d"></a>
## fmt

`function` · `datafusion_expr::logical_plan::ddl::OperateFunctionArg::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [692, 50], "end": [692, 55], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/ddl.rs:692`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-937b4aa73c420f6a28f87c64"></a>
## hash

`function` · `datafusion_expr::logical_plan::ddl::OperateFunctionArg::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [692, 44], "end": [692, 48], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/ddl.rs:692`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfc06aeaecfddcefc560cf06"></a>
## map_elements

`function` · `datafusion_expr::logical_plan::ddl::OperateFunctionArg::map_elements` · datafusion-expr 55.1.0

```rust
fn map_elements<F: FnMut(Expr) -> Result<Transformed<Expr>>>(self, f: F) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [701, 1], "end": [720, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeContainer", "path": "TreeNodeContainer"}, "trait_path": "datafusion_common::tree_node::TreeNodeContainer"}`

Source: `src/logical_plan/ddl.rs:709`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b76590cec3bc452ecc617f3f"></a>
## name

`struct_field` · `datafusion_expr::logical_plan::ddl::OperateFunctionArg::name` · datafusion-expr 55.1.0

```rust
name: Option<sqlparser::ast::Ident>
```

Source: `src/logical_plan/ddl.rs:696`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a3761ab4eb701f8ee93baed"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::ddl::OperateFunctionArg::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &OperateFunctionArg) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::OperateFunctionArg", "path": "OperateFunctionArg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [692, 32], "end": [692, 42], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/ddl.rs:692`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
