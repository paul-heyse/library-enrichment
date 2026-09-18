# `datafusion_expr::expr_schema::ExprSchemable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_schema.ExprSchemable.json).

<a id="op-7ac274f9ddd864e7c1033522"></a>
## ExprSchemable

`trait` · `datafusion_expr::expr_schema::ExprSchemable` · datafusion-expr 55.1.0

```rust
trait ExprSchemable
```

Source: `src/expr_schema.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Trait to allow expr to typable with respect to a schema

<a id="op-ce9a423c8bd0ec0b16c9ec82"></a>
## cast_to

`function` · `datafusion_expr::expr_schema::ExprSchemable::cast_to` · datafusion-expr 55.1.0

```rust
fn cast_to(self, cast_to_type: &DataType, schema: &dyn ExprSchema) -> Result<Expr>
```

Source: `src/expr_schema.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Cast to a type with respect to a schema

<a id="op-a54320f1df8be1f09d0079f4"></a>
## data_type_and_nullable

`function` · `datafusion_expr::expr_schema::ExprSchemable::data_type_and_nullable` · datafusion-expr 55.1.0

```rust
fn data_type_and_nullable(&self, schema: &dyn ExprSchema) -> Result<(DataType, bool)>
```

Source: `src/expr_schema.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Given a schema, return the type and nullability of the expr

<a id="op-1fe6f24ccb408cc1caaba5aa"></a>
## get_type

`function` · `datafusion_expr::expr_schema::ExprSchemable::get_type` · datafusion-expr 55.1.0

```rust
fn get_type(&self, schema: &dyn ExprSchema) -> Result<DataType>
```

Source: `src/expr_schema.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Given a schema, return the type of the expr

<a id="op-7569d020d1afce6c7de5dd55"></a>
## metadata

`function` · `datafusion_expr::expr_schema::ExprSchemable::metadata` · datafusion-expr 55.1.0

```rust
fn metadata(&self, schema: &dyn ExprSchema) -> Result<FieldMetadata>
```

Source: `src/expr_schema.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Given a schema, return the expr's optional metadata

<a id="op-1459642b39be797beb2fe496"></a>
## nullable

`function` · `datafusion_expr::expr_schema::ExprSchemable::nullable` · datafusion-expr 55.1.0

```rust
fn nullable(&self, input_schema: &dyn ExprSchema) -> Result<bool>
```

Source: `src/expr_schema.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Given a schema, return the nullability of the expr

<a id="op-f799024329b7cc4de1f89e79"></a>
## to_field

`function` · `datafusion_expr::expr_schema::ExprSchemable::to_field` · datafusion-expr 55.1.0

```rust
fn to_field(&self, input_schema: &dyn ExprSchema) -> Result<(Option<TableReference>, Arc<Field>)>
```

Source: `src/expr_schema.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Convert to a field with respect to a schema
