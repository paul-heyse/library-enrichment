# `datafusion_expr::logical_plan::display::GraphvizVisitor`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.display.GraphvizVisitor.json).

<a id="op-91d01ab773e18fafb84afb84"></a>
## GraphvizVisitor

`struct` · `datafusion_expr::logical_plan::display::GraphvizVisitor` · datafusion-expr 55.1.0

```rust
struct GraphvizVisitor<'a, 'b>
```

Source: `src/logical_plan/display.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Formats plans for graphical display using the `DOT` language. This
format can be visualized using software from
[`graphviz`](https://graphviz.org/)

<a id="op-e6faa574139649a5d8d52d90"></a>
## Node

`assoc_type` · `datafusion_expr::logical_plan::display::GraphvizVisitor::Node` · datafusion-expr 55.1.0

```rust
Node
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::GraphvizVisitor", "path": "GraphvizVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'n"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [221, 2], "filename": "src/logical_plan/display.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'n"}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeVisitor", "path": "TreeNodeVisitor"}, "trait_path": "datafusion_common::tree_node::TreeNodeVisitor"}`

Source: `src/logical_plan/display.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35a86ecf7e49ad54266daed2"></a>
## end_graph

`function` · `datafusion_expr::logical_plan::display::GraphvizVisitor::end_graph` · datafusion-expr 55.1.0

```rust
fn end_graph(&mut self) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::GraphvizVisitor", "path": "GraphvizVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [172, 2], "filename": "src/logical_plan/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/display.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a403e0139fe95bbec9d0a3b7"></a>
## f_down

`function` · `datafusion_expr::logical_plan::display::GraphvizVisitor::f_down` · datafusion-expr 55.1.0

```rust
fn f_down(&mut self, plan: &'n LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::GraphvizVisitor", "path": "GraphvizVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'n"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [221, 2], "filename": "src/logical_plan/display.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'n"}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeVisitor", "path": "TreeNodeVisitor"}, "trait_path": "datafusion_common::tree_node::TreeNodeVisitor"}`

Source: `src/logical_plan/display.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24ee8fe34dc5d0eed44bdc7c"></a>
## f_up

`function` · `datafusion_expr::logical_plan::display::GraphvizVisitor::f_up` · datafusion-expr 55.1.0

```rust
fn f_up(&mut self, _plan: &LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::GraphvizVisitor", "path": "GraphvizVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'n"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [221, 2], "filename": "src/logical_plan/display.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'n"}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeVisitor", "path": "TreeNodeVisitor"}, "trait_path": "datafusion_common::tree_node::TreeNodeVisitor"}`

Source: `src/logical_plan/display.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d69a77d1154c657c6ec7b4e3"></a>
## new

`function` · `datafusion_expr::logical_plan::display::GraphvizVisitor::new` · datafusion-expr 55.1.0

```rust
fn new(f: &'a mut fmt::Formatter<'b>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::GraphvizVisitor", "path": "GraphvizVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [172, 2], "filename": "src/logical_plan/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/display.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-447b59672707895442cd1504"></a>
## post_visit_plan

`function` · `datafusion_expr::logical_plan::display::GraphvizVisitor::post_visit_plan` · datafusion-expr 55.1.0

```rust
fn post_visit_plan(&mut self) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::GraphvizVisitor", "path": "GraphvizVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [172, 2], "filename": "src/logical_plan/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/display.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-342bbc021bd41eec8b2d96e4"></a>
## pre_visit_plan

`function` · `datafusion_expr::logical_plan::display::GraphvizVisitor::pre_visit_plan` · datafusion-expr 55.1.0

```rust
fn pre_visit_plan(&mut self, label: &str) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::GraphvizVisitor", "path": "GraphvizVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [172, 2], "filename": "src/logical_plan/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/display.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc08ac3d56e61bca834ec683"></a>
## set_with_schema

`function` · `datafusion_expr::logical_plan::display::GraphvizVisitor::set_with_schema` · datafusion-expr 55.1.0

```rust
fn set_with_schema(&mut self, with_schema: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::GraphvizVisitor", "path": "GraphvizVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [172, 2], "filename": "src/logical_plan/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/display.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Sets a flag which controls if the output schema is displayed

<a id="op-190724e081226e3509aef971"></a>
## start_graph

`function` · `datafusion_expr::logical_plan::display::GraphvizVisitor::start_graph` · datafusion-expr 55.1.0

```rust
fn start_graph(&mut self) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::GraphvizVisitor", "path": "GraphvizVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [172, 2], "filename": "src/logical_plan/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/display.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
