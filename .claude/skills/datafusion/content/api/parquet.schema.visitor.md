# `parquet::schema::visitor`

Crate `parquet` · 1 public items · structured records in [`model/parquet.schema.visitor.json`](../model/parquet.schema.visitor.json)

## TypeVisitor

`trait` · `parquet::schema::visitor::TypeVisitor`

```rust
trait TypeVisitor<R, C>
```

**Methods** (6)

```rust
fn dispatch(&mut self, cur_type: TypePtr, context: C) -> Result<R>
fn visit_list(&mut self, list_type: TypePtr, context: C) -> Result<R>
fn visit_list_with_item(&mut self, list_type: TypePtr, item_type: TypePtr, context: C) -> Result<R>
fn visit_map(&mut self, map_type: TypePtr, context: C) -> Result<R>
fn visit_primitive(&mut self, primitive_type: TypePtr, context: C) -> Result<R>
fn visit_struct(&mut self, struct_type: TypePtr, context: C) -> Result<R>
```

[Full member, field, variant and typed contracts](../operations/parquet.schema.visitor.TypeVisitor.md).


A utility trait to help user to traverse against parquet type.

---
