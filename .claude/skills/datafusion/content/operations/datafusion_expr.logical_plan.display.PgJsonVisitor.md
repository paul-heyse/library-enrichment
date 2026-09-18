# `datafusion_expr::logical_plan::display::PgJsonVisitor`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.display.PgJsonVisitor.json).

<a id="op-4bf15b8ce9a443e1b766b5b4"></a>
## PgJsonVisitor

`struct` · `datafusion_expr::logical_plan::display::PgJsonVisitor` · datafusion-expr 55.1.0

```rust
struct PgJsonVisitor<'a, 'b>
```

Source: `src/logical_plan/display.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Formats plans to display as postgresql plan json format.

There are already many existing visualizer for this format, for example [dalibo](https://explain.dalibo.com/).
Unfortunately, there is no formal spec for this format, but it is widely used in the PostgreSQL community.

Here is an example of the format:

```json
[
    {
        "Plan": {
            "Node Type": "Sort",
            "Output": [
                "question_1.id",
                "question_1.title",
                "question_1.text",
                "question_1.file",
                "question_1.type",
                "question_1.source",
                "question_1.exam_id"
            ],
            "Sort Key": [
                "question_1.id"
            ],
            "Plans": [
                {
                    "Node Type": "Seq Scan",
                    "Parent Relationship": "Left",
                    "Relation Name": "question",
                    "Schema": "public",
                    "Alias": "question_1",
                    "Output": [
                       "question_1.id",
                        "question_1.title",
                       "question_1.text",
                        "question_1.file",
                        "question_1.type",
                        "question_1.source",
                        "question_1.exam_id"
                    ],
                    "Filter": "(question_1.exam_id = 1)"
                }
            ]
        }
    }
]
```

<a id="op-e1aec7a275953a9d36b8403f"></a>
## Node

`assoc_type` · `datafusion_expr::logical_plan::display::PgJsonVisitor::Node` · datafusion-expr 55.1.0

```rust
Node
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::PgJsonVisitor", "path": "PgJsonVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'n"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [718, 2], "filename": "src/logical_plan/display.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'n"}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeVisitor", "path": "TreeNodeVisitor"}, "trait_path": "datafusion_common::tree_node::TreeNodeVisitor"}`

Source: `src/logical_plan/display.rs:655`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-682b6f247c3be0cd578a393e"></a>
## f_down

`function` · `datafusion_expr::logical_plan::display::PgJsonVisitor::f_down` · datafusion-expr 55.1.0

```rust
fn f_down(&mut self, node: &'n LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::PgJsonVisitor", "path": "PgJsonVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'n"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [718, 2], "filename": "src/logical_plan/display.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'n"}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeVisitor", "path": "TreeNodeVisitor"}, "trait_path": "datafusion_common::tree_node::TreeNodeVisitor"}`

Source: `src/logical_plan/display.rs:657`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b817d2da28b01bb19beddf33"></a>
## f_up

`function` · `datafusion_expr::logical_plan::display::PgJsonVisitor::f_up` · datafusion-expr 55.1.0

```rust
fn f_up(&mut self, _node: &Self::Node) -> datafusion_common::Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::PgJsonVisitor", "path": "PgJsonVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'n"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [718, 2], "filename": "src/logical_plan/display.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'n"}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeVisitor", "path": "TreeNodeVisitor"}, "trait_path": "datafusion_common::tree_node::TreeNodeVisitor"}`

Source: `src/logical_plan/display.rs:683`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a916e3c0c6f624ae79306948"></a>
## new

`function` · `datafusion_expr::logical_plan::display::PgJsonVisitor::new` · datafusion-expr 55.1.0

```rust
fn new(f: &'a mut fmt::Formatter<'b>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::PgJsonVisitor", "path": "PgJsonVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [652, 2], "filename": "src/logical_plan/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/display.rs:287`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-850e002a41f84cfebbe87695"></a>
## with_schema

`function` · `datafusion_expr::logical_plan::display::PgJsonVisitor::with_schema` · datafusion-expr 55.1.0

```rust
fn with_schema(&mut self, with_schema: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}], "constraints": []}}, "id": "datafusion_expr::logical_plan::display::PgJsonVisitor", "path": "PgJsonVisitor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [652, 2], "filename": "src/logical_plan/display.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/display.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Sets a flag which controls if the output schema is displayed
