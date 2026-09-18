# `datafusion_substrait::logical_plan::consumer::expr::ExprContainer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.ExprContainer.json).

<a id="op-077cc0fee9367ada2915462a"></a>
## ExprContainer

`struct` · `datafusion_substrait::logical_plan::consumer::expr::ExprContainer` · datafusion-substrait 55.1.0

```rust
struct ExprContainer
```

Source: `src/logical_plan/consumer/expr/mod.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

An ExprContainer is a container for a collection of expressions with a common input schema

In addition, each expression is associated with a field, which defines the
expression's output.  The data type and nullability of the field are calculated from the
expression and the input schema.  However the names of the field (and its nested fields) are
derived from the Substrait message.

<a id="op-1b36927ac3790d953edaf3e1"></a>
## exprs

`struct_field` · `datafusion_substrait::logical_plan::consumer::expr::ExprContainer::exprs` · datafusion-substrait 55.1.0

```rust
exprs: Vec<(datafusion::logical_expr::Expr, datafusion::arrow::datatypes::Field)>
```

Source: `src/logical_plan/consumer/expr/mod.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

The expressions

Each item contains an expression and the field that defines the expected nullability and name of the expr's output

<a id="op-4d39dbc3f8650aac021cefff"></a>
## input_schema

`struct_field` · `datafusion_substrait::logical_plan::consumer::expr::ExprContainer::input_schema` · datafusion-substrait 55.1.0

```rust
input_schema: datafusion::common::DFSchemaRef
```

Source: `src/logical_plan/consumer/expr/mod.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

The input schema for the expressions
