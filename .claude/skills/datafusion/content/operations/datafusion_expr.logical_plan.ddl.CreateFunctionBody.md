# `datafusion_expr::logical_plan::ddl::CreateFunctionBody`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.ddl.CreateFunctionBody.json).

<a id="op-3006162c81f6d40375fa2213"></a>
## CreateFunctionBody

`struct` · `datafusion_expr::logical_plan::ddl::CreateFunctionBody` · datafusion-expr 55.1.0

```rust
struct CreateFunctionBody
```

Source: `src/logical_plan/ddl.rs:726`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Part of the `CREATE FUNCTION` statement

See [`CreateFunction`](../operations/datafusion_expr.logical_plan.ddl.CreateFunction.md#op-3668c6565f5901a6967c489f) for details

<a id="op-3419129a2a0ab9a192f6f591"></a>
## apply_elements

`function` · `datafusion_expr::logical_plan::ddl::CreateFunctionBody::apply_elements` · datafusion-expr 55.1.0

```rust
fn apply_elements<F: FnMut(&'a Expr) -> Result<TreeNodeRecursion>>(&'a self, f: F) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [735, 1], "end": [756, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeContainer", "path": "TreeNodeContainer"}, "trait_path": "datafusion_common::tree_node::TreeNodeContainer"}`

Source: `src/logical_plan/ddl.rs:736`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04512c12244982876fc49c49"></a>
## behavior

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateFunctionBody::behavior` · datafusion-expr 55.1.0

```rust
behavior: Option<Volatility>
```

Source: `src/logical_plan/ddl.rs:730`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

IMMUTABLE | STABLE | VOLATILE

<a id="op-a487892b61f16f6c2778a5e2"></a>
## clone

`function` · `datafusion_expr::logical_plan::ddl::CreateFunctionBody::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> CreateFunctionBody
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [725, 10], "end": [725, 15], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/ddl.rs:725`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a325d19c54b62032d8abaaf"></a>
## eq

`function` · `datafusion_expr::logical_plan::ddl::CreateFunctionBody::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &CreateFunctionBody) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [725, 17], "end": [725, 26], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/ddl.rs:725`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efa6c93136fbce28438c54dd"></a>
## fmt

`function` · `datafusion_expr::logical_plan::ddl::CreateFunctionBody::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [725, 50], "end": [725, 55], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/ddl.rs:725`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d4d524d808b60d6e5ebaba1"></a>
## function_body

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateFunctionBody::function_body` · datafusion-expr 55.1.0

```rust
function_body: Option<Expr>
```

Source: `src/logical_plan/ddl.rs:732`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

RETURN or AS function body

<a id="op-5fdac0a307484c07ac5b969e"></a>
## hash

`function` · `datafusion_expr::logical_plan::ddl::CreateFunctionBody::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [725, 44], "end": [725, 48], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/ddl.rs:725`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-510066ac01ce6fcea9a32f28"></a>
## language

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateFunctionBody::language` · datafusion-expr 55.1.0

```rust
language: Option<sqlparser::ast::Ident>
```

Source: `src/logical_plan/ddl.rs:728`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

LANGUAGE lang_name

<a id="op-5c6a3221682eb98ba3b28fd7"></a>
## map_elements

`function` · `datafusion_expr::logical_plan::ddl::CreateFunctionBody::map_elements` · datafusion-expr 55.1.0

```rust
fn map_elements<F: FnMut(Expr) -> Result<Transformed<Expr>>>(self, f: F) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [735, 1], "end": [756, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeContainer", "path": "TreeNodeContainer"}, "trait_path": "datafusion_common::tree_node::TreeNodeContainer"}`

Source: `src/logical_plan/ddl.rs:743`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5369aa21193052d179c2d93c"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::ddl::CreateFunctionBody::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &CreateFunctionBody) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [725, 32], "end": [725, 42], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/ddl.rs:725`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
