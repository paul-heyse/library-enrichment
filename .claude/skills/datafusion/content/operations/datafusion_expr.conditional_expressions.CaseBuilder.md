# `datafusion_expr::conditional_expressions::CaseBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.conditional_expressions.CaseBuilder.json).

<a id="op-4b54fdfd9575eff869b289e4"></a>
## CaseBuilder

`struct` · `datafusion_expr::conditional_expressions::CaseBuilder` · datafusion-expr 55.1.0

```rust
struct CaseBuilder
```

Source: `src/conditional_expressions.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Helper struct for building [Expr::Case](../operations/datafusion_expr.expr.Expr.md#op-030741e7d78bd356286f24f2)

<a id="op-d5ada3fe79470314a73a6b1a"></a>
## clone

`function` · `datafusion_expr::conditional_expressions::CaseBuilder::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> CaseBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::conditional_expressions::CaseBuilder", "path": "CaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 17], "end": [26, 22], "filename": "src/conditional_expressions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/conditional_expressions.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3d482656098390ac109b4ff"></a>
## end

`function` · `datafusion_expr::conditional_expressions::CaseBuilder::end` · datafusion-expr 55.1.0

```rust
fn end(&self) -> Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::conditional_expressions::CaseBuilder", "path": "CaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [106, 2], "filename": "src/conditional_expressions.rs"}, "trait": null, "trait_path": null}`

Source: `src/conditional_expressions.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c63c183bc945e5c559656698"></a>
## fmt

`function` · `datafusion_expr::conditional_expressions::CaseBuilder::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::conditional_expressions::CaseBuilder", "path": "CaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 15], "filename": "src/conditional_expressions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/conditional_expressions.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ff9413b5b051449968f922a"></a>
## new

`function` · `datafusion_expr::conditional_expressions::CaseBuilder::new` · datafusion-expr 55.1.0

```rust
fn new(expr: Option<Box<Expr>>, when_expr: Vec<Expr>, then_expr: Vec<Expr>, else_expr: Option<Box<Expr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::conditional_expressions::CaseBuilder", "path": "CaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [106, 2], "filename": "src/conditional_expressions.rs"}, "trait": null, "trait_path": null}`

Source: `src/conditional_expressions.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7676cd23b0db94ea4dddf8a1"></a>
## otherwise

`function` · `datafusion_expr::conditional_expressions::CaseBuilder::otherwise` · datafusion-expr 55.1.0

```rust
fn otherwise(&mut self, else_expr: Expr) -> Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::conditional_expressions::CaseBuilder", "path": "CaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [106, 2], "filename": "src/conditional_expressions.rs"}, "trait": null, "trait_path": null}`

Source: `src/conditional_expressions.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-818e73837821f0bf07d977ec"></a>
## when

`function` · `datafusion_expr::conditional_expressions::CaseBuilder::when` · datafusion-expr 55.1.0

```rust
fn when(&mut self, when: Expr, then: Expr) -> CaseBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::conditional_expressions::CaseBuilder", "path": "CaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [106, 2], "filename": "src/conditional_expressions.rs"}, "trait": null, "trait_path": null}`

Source: `src/conditional_expressions.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
