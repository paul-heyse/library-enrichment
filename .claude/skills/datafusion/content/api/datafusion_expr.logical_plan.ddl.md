# `datafusion_expr::logical_plan::ddl`

Crate `datafusion-expr` · 15 public items · structured records in [`model/datafusion_expr.logical_plan.ddl.json`](../model/datafusion_expr.logical_plan.ddl.json)

## DdlStatement

`enum` · `datafusion_expr::logical_plan::ddl::DdlStatement`

Also reachable as `datafusion::logical_expr::DdlStatement`, `datafusion_expr::DdlStatement`, `datafusion_expr::logical_plan::DdlStatement`

```rust
enum DdlStatement
```

**Variants**: `CreateExternalTable`, `CreateMemoryTable`, `CreateView`, `CreateCatalogSchema`, `CreateCatalog`, `CreateIndex`, `DropTable`, `DropView`, `DropCatalogSchema`, `CreateFunction`, `DropFunction`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn display(&self) -> impl Display + '_
fn inputs(&self) -> Vec<&LogicalPlan>
fn name(&self) -> &str
fn schema(&self) -> &DFSchemaRef
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.DdlStatement.md).


Various types of DDL  (CREATE / DROP) catalog manipulation

---

## CreateCatalog

`struct` · `datafusion_expr::logical_plan::ddl::CreateCatalog`

Also reachable as `datafusion::logical_expr::CreateCatalog`, `datafusion_expr::CreateCatalog`, `datafusion_expr::logical_plan::CreateCatalog`

```rust
struct CreateCatalog
```

**Fields**: `catalog_name`, `if_not_exists`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.CreateCatalog.md).


Creates a catalog (aka "Database").

---

## CreateCatalogSchema

`struct` · `datafusion_expr::logical_plan::ddl::CreateCatalogSchema`

Also reachable as `datafusion::logical_expr::CreateCatalogSchema`, `datafusion_expr::CreateCatalogSchema`, `datafusion_expr::logical_plan::CreateCatalogSchema`

```rust
struct CreateCatalogSchema
```

**Fields**: `schema_name`, `if_not_exists`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.CreateCatalogSchema.md).


Creates a schema.

---

## CreateExternalTable

`struct` · `datafusion_expr::logical_plan::ddl::CreateExternalTable`

Also reachable as `datafusion::logical_expr::CreateExternalTable`, `datafusion_expr::CreateExternalTable`, `datafusion_expr::logical_plan::CreateExternalTable`

```rust
struct CreateExternalTable
```

**Fields**: `schema`, `name`, `locations`, `file_type`, `table_partition_cols`, `if_not_exists`, `or_replace`, `temporary`, `definition`, `order_exprs`, `unbounded`, `options`, `constraints`, `column_defaults`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn builder(name: impl Into<TableReference>, location: impl Into<String>, file_type: impl Into<String>, schema: DFSchemaRef) -> CreateExternalTableBuilder
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.CreateExternalTable.md).


Creates an external table.

---

## CreateExternalTableBuilder

`struct` · `datafusion_expr::logical_plan::ddl::CreateExternalTableBuilder`

```rust
struct CreateExternalTableBuilder
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.CreateExternalTableBuilder.md).


Builder for [`CreateExternalTable`] that provides a fluent API for construction.

Created via [`CreateExternalTable::builder`].

---

## CreateFunction

`struct` · `datafusion_expr::logical_plan::ddl::CreateFunction`

Also reachable as `datafusion::logical_expr::CreateFunction`, `datafusion_expr::CreateFunction`, `datafusion_expr::logical_plan::CreateFunction`

```rust
struct CreateFunction
```

**Fields**: `or_replace`, `temporary`, `name`, `args`, `return_type`, `params`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.CreateFunction.md).


Arguments passed to the `CREATE FUNCTION` statement

These statements are turned into executable functions using [`FunctionFactory`]

# Notes

This structure purposely mirrors the structure in sqlparser's
[`sqlparser::ast::Statement::CreateFunction`], but does not use it directly
to avoid a dependency on sqlparser in the core crate.


[`FunctionFactory`]: https://docs.rs/datafusion/latest/datafusion/execution/context/trait.FunctionFactory.html

---

## CreateFunctionBody

`struct` · `datafusion_expr::logical_plan::ddl::CreateFunctionBody`

Also reachable as `datafusion::logical_expr::CreateFunctionBody`, `datafusion_expr::CreateFunctionBody`, `datafusion_expr::logical_plan::CreateFunctionBody`

```rust
struct CreateFunctionBody
```

**Fields**: `language`, `behavior`, `function_body`

**Implements**: `datafusion_common::tree_node::TreeNodeContainer`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**via `datafusion_common::tree_node::TreeNodeContainer`**

```rust
fn apply_elements<F: FnMut(&'a Expr) -> Result<TreeNodeRecursion>>(&'a self, f: F) -> Result<TreeNodeRecursion>
fn map_elements<F: FnMut(Expr) -> Result<Transformed<Expr>>>(self, f: F) -> Result<Transformed<Self>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.CreateFunctionBody.md).


Part of the `CREATE FUNCTION` statement

See [`CreateFunction`] for details

---

## CreateIndex

`struct` · `datafusion_expr::logical_plan::ddl::CreateIndex`

Also reachable as `datafusion::logical_expr::CreateIndex`, `datafusion_expr::CreateIndex`, `datafusion_expr::logical_plan::CreateIndex`

```rust
struct CreateIndex
```

**Fields**: `name`, `table`, `using`, `columns`, `unique`, `if_not_exists`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.CreateIndex.md).


---

## CreateMemoryTable

`struct` · `datafusion_expr::logical_plan::ddl::CreateMemoryTable`

Also reachable as `datafusion::logical_expr::CreateMemoryTable`, `datafusion_expr::CreateMemoryTable`, `datafusion_expr::logical_plan::CreateMemoryTable`

```rust
struct CreateMemoryTable
```

**Fields**: `name`, `constraints`, `input`, `if_not_exists`, `or_replace`, `column_defaults`, `temporary`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.CreateMemoryTable.md).


Creates an in memory table.

---

## CreateView

`struct` · `datafusion_expr::logical_plan::ddl::CreateView`

Also reachable as `datafusion::logical_expr::CreateView`, `datafusion_expr::CreateView`, `datafusion_expr::logical_plan::CreateView`

```rust
struct CreateView
```

**Fields**: `name`, `input`, `or_replace`, `definition`, `temporary`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.CreateView.md).


Creates a view.

---

## DropCatalogSchema

`struct` · `datafusion_expr::logical_plan::ddl::DropCatalogSchema`

Also reachable as `datafusion::logical_expr::DropCatalogSchema`, `datafusion_expr::DropCatalogSchema`, `datafusion_expr::logical_plan::DropCatalogSchema`

```rust
struct DropCatalogSchema
```

**Fields**: `name`, `if_exists`, `cascade`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.DropCatalogSchema.md).


Drops a schema

---

## DropFunction

`struct` · `datafusion_expr::logical_plan::ddl::DropFunction`

Also reachable as `datafusion::logical_expr::DropFunction`, `datafusion_expr::DropFunction`, `datafusion_expr::logical_plan::DropFunction`

```rust
struct DropFunction
```

**Fields**: `name`, `if_exists`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.DropFunction.md).


---

## DropTable

`struct` · `datafusion_expr::logical_plan::ddl::DropTable`

Also reachable as `datafusion::logical_expr::DropTable`, `datafusion_expr::DropTable`, `datafusion_expr::logical_plan::DropTable`

```rust
struct DropTable
```

**Fields**: `name`, `if_exists`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.DropTable.md).


Drops a table.

---

## DropView

`struct` · `datafusion_expr::logical_plan::ddl::DropView`

Also reachable as `datafusion::logical_expr::DropView`, `datafusion_expr::DropView`, `datafusion_expr::logical_plan::DropView`

```rust
struct DropView
```

**Fields**: `name`, `if_exists`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.DropView.md).


Drops a view.

---

## OperateFunctionArg

`struct` · `datafusion_expr::logical_plan::ddl::OperateFunctionArg`

Also reachable as `datafusion::logical_expr::OperateFunctionArg`, `datafusion_expr::OperateFunctionArg`, `datafusion_expr::logical_plan::OperateFunctionArg`

```rust
struct OperateFunctionArg
```

**Fields**: `name`, `data_type`, `default_expr`

**Implements**: `datafusion_common::tree_node::TreeNodeContainer`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**via `datafusion_common::tree_node::TreeNodeContainer`**

```rust
fn apply_elements<F: FnMut(&'a Expr) -> Result<TreeNodeRecursion>>(&'a self, f: F) -> Result<TreeNodeRecursion>
fn map_elements<F: FnMut(Expr) -> Result<Transformed<Expr>>>(self, f: F) -> Result<Transformed<Self>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.ddl.OperateFunctionArg.md).


Part of the `CREATE FUNCTION` statement

See [`CreateFunction`] for details

---
