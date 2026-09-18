# `datafusion_expr::expr::BinaryExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.BinaryExpr.json).

<a id="op-1eb3eb15f0a982d97788c161"></a>
## BinaryExpr

`struct` · `datafusion_expr::expr::BinaryExpr` · datafusion-expr 55.1.0

```rust
struct BinaryExpr
```

Source: `src/expr.rs:793`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Binary expression for [`Expr::BinaryExpr`](../operations/datafusion_expr.expr.Expr.md#op-5fea7a03414f3ba72127e37d)

<a id="op-1b8eb53cecb6aa4b0618033d"></a>
## clone

`function` · `datafusion_expr::expr::BinaryExpr::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> BinaryExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [792, 10], "end": [792, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:792`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d31087875766fe374cbfe93e"></a>
## eq

`function` · `datafusion_expr::expr::BinaryExpr::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &BinaryExpr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [792, 17], "end": [792, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:792`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-409c974f411bcf64270c1e45"></a>
## fmt

`function` · `datafusion_expr::expr::BinaryExpr::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [809, 1], "end": [840, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expr.rs:810`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51999c084d693ac750c48465"></a>
## fmt

`function` · `datafusion_expr::expr::BinaryExpr::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [792, 50], "end": [792, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:792`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbb887e1d6105bc55319827e"></a>
## hash

`function` · `datafusion_expr::expr::BinaryExpr::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [792, 44], "end": [792, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:792`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57a49bf81e039a6c6ffcc05a"></a>
## left

`struct_field` · `datafusion_expr::expr::BinaryExpr::left` · datafusion-expr 55.1.0

```rust
left: Box<Expr>
```

Source: `src/expr.rs:795`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Left-hand side of the expression

<a id="op-17212d8d32d84933e78c8699"></a>
## new

`function` · `datafusion_expr::expr::BinaryExpr::new` · datafusion-expr 55.1.0

```rust
fn new(left: Box<Expr>, op: Operator, right: Box<Expr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 1], "end": [807, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:804`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new binary expression

<a id="op-3ff9942f2f4185feaf533e25"></a>
## op

`struct_field` · `datafusion_expr::expr::BinaryExpr::op` · datafusion-expr 55.1.0

```rust
op: Operator
```

Source: `src/expr.rs:797`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The comparison operator

<a id="op-c9b9b4eda0eff64ac8d1f431"></a>
## partial_cmp

`function` · `datafusion_expr::expr::BinaryExpr::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &BinaryExpr) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [792, 32], "end": [792, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:792`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-910bf2b29cbb19dcd63ca662"></a>
## right

`struct_field` · `datafusion_expr::expr::BinaryExpr::right` · datafusion-expr 55.1.0

```rust
right: Box<Expr>
```

Source: `src/expr.rs:799`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Right-hand side of the expression
