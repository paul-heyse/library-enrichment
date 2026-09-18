# `datafusion_physical_expr::physical_expr::create_ordering`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.physical_expr.create_ordering.json).

<a id="op-f3183477f63973f00a478fa3"></a>
## create_ordering

`function` · `datafusion_physical_expr::physical_expr::create_ordering` · datafusion-physical-expr 55.1.0

```rust
fn create_ordering(schema: &arrow::datatypes::Schema, sort_order: &[Vec<datafusion_expr::SortExpr>]) -> datafusion_common::Result<Vec<LexOrdering>>
```

Source: `src/physical_expr.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Converts logical sort expressions to physical sort expressions.

This function transforms a collection of logical sort expressions into their
physical representation that can be used during query execution.

# Arguments

* `schema` - The schema containing column definitions.
* `sort_order` - A collection of logical sort expressions grouped into
  lexicographic orderings.

# Returns

A vector of lexicographic orderings for physical execution, or an error if
the transformation fails.

# Examples

```
// Create orderings from columns "id" and "name"
# use arrow::datatypes::{Schema, Field, DataType};
# use datafusion_physical_expr::create_ordering;
# use datafusion_common::Column;
# use datafusion_expr::{Expr, SortExpr};
#
// Create a schema with two fields
let schema = Schema::new(vec![
    Field::new("id", DataType::Int32, false),
    Field::new("name", DataType::Utf8, false),
]);

let sort_exprs = vec![
    vec![SortExpr {
        expr: Expr::Column(Column::new(Some("t"), "id")),
        asc: true,
        nulls_first: false,
    }],
    vec![SortExpr {
        expr: Expr::Column(Column::new(Some("t"), "name")),
        asc: false,
        nulls_first: true,
    }],
];
let result = create_ordering(&schema, &sort_exprs).unwrap();
```
