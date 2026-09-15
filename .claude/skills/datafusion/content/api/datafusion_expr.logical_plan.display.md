# `datafusion_expr::logical_plan::display`

Crate `datafusion-expr` · 4 public items · structured records in [`model/datafusion_expr.logical_plan.display.json`](../model/datafusion_expr.logical_plan.display.json)

## display_schema

`function` · `datafusion_expr::logical_plan::display::display_schema`

Also reachable as `datafusion::logical_expr::display_schema`, `datafusion_expr::display_schema`, `datafusion_expr::logical_plan::display_schema`

```rust
fn display_schema(schema: &arrow::datatypes::Schema) -> impl fmt::Display + '_
```

Print the schema in a compact representation to `buf`

For example: `foo:Utf8` if `foo` can not be null, and
`foo:Utf8;N` if `foo` is nullable.

```
use arrow::datatypes::{DataType, Field, Schema};
# use datafusion_expr::logical_plan::display_schema;
let schema = Schema::new(vec![
    Field::new("id", DataType::Int32, false),
    Field::new("first_name", DataType::Utf8, true),
]);

assert_eq!(
    "[id:Int32, first_name:Utf8;N]",
    format!("{}", display_schema(&schema))
);
```

---

## GraphvizVisitor

`struct` · `datafusion_expr::logical_plan::display::GraphvizVisitor`

```rust
struct GraphvizVisitor<'a, 'b>
```

**Implements**: `datafusion_common::tree_node::TreeNodeVisitor`

**Methods** (6)

```rust
fn end_graph(&mut self) -> fmt::Result
fn new(f: &'a mut fmt::Formatter<'b>) -> Self
fn post_visit_plan(&mut self) -> fmt::Result
fn pre_visit_plan(&mut self, label: &str) -> fmt::Result
fn set_with_schema(&mut self, with_schema: bool)
fn start_graph(&mut self) -> fmt::Result
```

**via `datafusion_common::tree_node::TreeNodeVisitor`**

```rust
fn f_down(&mut self, plan: &'n LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion>
fn f_up(&mut self, _plan: &LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion>
```

Formats plans for graphical display using the `DOT` language. This
format can be visualized using software from
[`graphviz`](https://graphviz.org/)

---

## IndentVisitor

`struct` · `datafusion_expr::logical_plan::display::IndentVisitor`

```rust
struct IndentVisitor<'a, 'b>
```

**Implements**: `datafusion_common::tree_node::TreeNodeVisitor`

**Methods** (1)

```rust
fn new(f: &'a mut fmt::Formatter<'b>, with_schema: bool) -> Self
```

**via `datafusion_common::tree_node::TreeNodeVisitor`**

```rust
fn f_down(&mut self, plan: &'n LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion>
fn f_up(&mut self, _plan: &'n LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion>
```

Formats plans with a single line per node. For example:

Projection: id
   Filter: state Eq Utf8(\"CO\")\
      CsvScan: employee.csv projection=Some([0, 3])";

---

## PgJsonVisitor

`struct` · `datafusion_expr::logical_plan::display::PgJsonVisitor`

```rust
struct PgJsonVisitor<'a, 'b>
```

**Implements**: `datafusion_common::tree_node::TreeNodeVisitor`

**Methods** (2)

```rust
fn new(f: &'a mut fmt::Formatter<'b>) -> Self
fn with_schema(&mut self, with_schema: bool)
```

**via `datafusion_common::tree_node::TreeNodeVisitor`**

```rust
fn f_down(&mut self, node: &'n LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion>
fn f_up(&mut self, _node: &Self::Node) -> datafusion_common::Result<TreeNodeRecursion>
```

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

---
