# `datafusion_expr::logical_plan::display::IndentVisitor`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.display.IndentVisitor.json).

<a id="op-ddad485ed99833fe8a4e9687"></a>
## IndentVisitor

`struct` · `datafusion_expr::logical_plan::display::IndentVisitor` · datafusion-expr 55.1.0

```rust
struct IndentVisitor<'a, 'b>
```

Source: `src/logical_plan/display.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Formats plans with a single line per node. For example:

Projection: id
   Filter: state Eq Utf8(\"CO\")\
      CsvScan: employee.csv projection=Some([0, 3])";

<a id="op-75c06f81d6e2c433d4896f23"></a>
## Node

`assoc_type` · `datafusion_expr::logical_plan::display::IndentVisitor::Node` · datafusion-expr 55.1.0

```rust
Node
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::IndentVisitor", "path": "IndentVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'n"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [89, 2], "filename": "src/logical_plan/display.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'n"}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeVisitor", "path": "TreeNodeVisitor"}, "trait_path": "datafusion_common::tree_node::TreeNodeVisitor"}`

Source: `src/logical_plan/display.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-deda2f11a2a3e07a4251b042"></a>
## f_down

`function` · `datafusion_expr::logical_plan::display::IndentVisitor::f_down` · datafusion-expr 55.1.0

```rust
fn f_down(&mut self, plan: &'n LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::IndentVisitor", "path": "IndentVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'n"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [89, 2], "filename": "src/logical_plan/display.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'n"}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeVisitor", "path": "TreeNodeVisitor"}, "trait_path": "datafusion_common::tree_node::TreeNodeVisitor"}`

Source: `src/logical_plan/display.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba43347002508de96ce12fae"></a>
## f_up

`function` · `datafusion_expr::logical_plan::display::IndentVisitor::f_up` · datafusion-expr 55.1.0

```rust
fn f_up(&mut self, _plan: &'n LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::IndentVisitor", "path": "IndentVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'n"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [89, 2], "filename": "src/logical_plan/display.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'n"}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeVisitor", "path": "TreeNodeVisitor"}, "trait_path": "datafusion_common::tree_node::TreeNodeVisitor"}`

Source: `src/logical_plan/display.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc1064e840e6288769cacf69"></a>
## new

`function` · `datafusion_expr::logical_plan::display::IndentVisitor::new` · datafusion-expr 55.1.0

```rust
fn new(f: &'a mut fmt::Formatter<'b>, with_schema: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::IndentVisitor", "path": "IndentVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [60, 2], "filename": "src/logical_plan/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/display.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a visitor that will write a formatted LogicalPlan to f. If `with_schema` is
true, includes schema information on each line.
