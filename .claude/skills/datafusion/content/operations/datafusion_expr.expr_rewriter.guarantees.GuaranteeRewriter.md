# `datafusion_expr::expr_rewriter::guarantees::GuaranteeRewriter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.guarantees.GuaranteeRewriter.json).

<a id="op-90a79aeb50b958ff8a2cf30f"></a>
## GuaranteeRewriter

`struct` · `datafusion_expr::expr_rewriter::guarantees::GuaranteeRewriter` · datafusion-expr 55.1.0

```rust
struct GuaranteeRewriter<'a>
```

Source: `src/expr_rewriter/guarantees.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Rewrite expressions to incorporate guarantees.

See [`rewrite_with_guarantees`](../operations/datafusion_expr.expr_rewriter.guarantees.rewrite_with_guarantees.md#op-7ce773c056f3dc96ed4f991b) for more information

<a id="op-cd5c20b0d2645025c91b7c8d"></a>
## Node

`assoc_type` · `datafusion_expr::expr_rewriter::guarantees::GuaranteeRewriter::Node` · datafusion-expr 55.1.0

```rust
Node
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_expr::expr_rewriter::guarantees::GuaranteeRewriter", "path": "GuaranteeRewriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [98, 2], "filename": "src/expr_rewriter/guarantees.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRewriter", "path": "TreeNodeRewriter"}, "trait_path": "datafusion_common::tree_node::TreeNodeRewriter"}`

Source: `src/expr_rewriter/guarantees.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea1b1e053d51db1abf458ef0"></a>
## f_up

`function` · `datafusion_expr::expr_rewriter::guarantees::GuaranteeRewriter::f_up` · datafusion-expr 55.1.0

```rust
fn f_up(&mut self, expr: Expr) -> Result<Transformed<Expr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_expr::expr_rewriter::guarantees::GuaranteeRewriter", "path": "GuaranteeRewriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [98, 2], "filename": "src/expr_rewriter/guarantees.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRewriter", "path": "TreeNodeRewriter"}, "trait_path": "datafusion_common::tree_node::TreeNodeRewriter"}`

Source: `src/expr_rewriter/guarantees.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bd078dc1f46481e68d68e43"></a>
## new

`function` · `datafusion_expr::expr_rewriter::guarantees::GuaranteeRewriter::new` · datafusion-expr 55.1.0

```rust
fn new(guarantees: impl IntoIterator<Item = &'a (Expr, NullableInterval)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_expr::expr_rewriter::guarantees::GuaranteeRewriter", "path": "GuaranteeRewriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [41, 2], "filename": "src/expr_rewriter/guarantees.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr_rewriter/guarantees.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
