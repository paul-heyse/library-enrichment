# `datafusion_expr::logical_plan::statement`

Crate `datafusion-expr` · 11 public items · structured records in [`model/datafusion_expr.logical_plan.statement.json`](../model/datafusion_expr.logical_plan.statement.json)

## Statement

`enum` · `datafusion_expr::logical_plan::statement::Statement`

Also reachable as `datafusion::logical_expr::Statement`, `datafusion_expr::Statement`, `datafusion_expr::logical_plan::Statement`

```rust
enum Statement
```

**Variants**: `TransactionStart`, `TransactionEnd`, `SetVariable`, `ResetVariable`, `Prepare`, `Execute`, `Deallocate`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn display(&self) -> impl Display + '_
fn name(&self) -> &str
fn schema(&self) -> &DFSchemaRef
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.statement.Statement.md).


Various types of Statements.

# Transactions:

While DataFusion does not offer support transactions, it provides
[`LogicalPlan`] support to assist building database systems
using DataFusion

---

## TransactionAccessMode

`enum` · `datafusion_expr::logical_plan::statement::TransactionAccessMode`

Also reachable as `datafusion::logical_expr::TransactionAccessMode`, `datafusion_expr::TransactionAccessMode`, `datafusion_expr::logical_plan::TransactionAccessMode`

```rust
enum TransactionAccessMode
```

**Variants**: `ReadOnly`, `ReadWrite`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.statement.TransactionAccessMode.md).


Indicates if this transaction is allowed to write

---

## TransactionConclusion

`enum` · `datafusion_expr::logical_plan::statement::TransactionConclusion`

Also reachable as `datafusion::logical_expr::TransactionConclusion`, `datafusion_expr::TransactionConclusion`, `datafusion_expr::logical_plan::TransactionConclusion`

```rust
enum TransactionConclusion
```

**Variants**: `Commit`, `Rollback`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.statement.TransactionConclusion.md).


Indicates if a transaction was committed or aborted

---

## TransactionIsolationLevel

`enum` · `datafusion_expr::logical_plan::statement::TransactionIsolationLevel`

Also reachable as `datafusion::logical_expr::TransactionIsolationLevel`, `datafusion_expr::TransactionIsolationLevel`, `datafusion_expr::logical_plan::TransactionIsolationLevel`

```rust
enum TransactionIsolationLevel
```

**Variants**: `ReadUncommitted`, `ReadCommitted`, `RepeatableRead`, `Serializable`, `Snapshot`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.statement.TransactionIsolationLevel.md).


Indicates ANSI transaction isolation level

---

## Deallocate

`struct` · `datafusion_expr::logical_plan::statement::Deallocate`

Also reachable as `datafusion::logical_expr::Deallocate`, `datafusion_expr::Deallocate`, `datafusion_expr::logical_plan::Deallocate`

```rust
struct Deallocate
```

**Fields**: `name`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.statement.Deallocate.md).


Deallocate a prepared statement.

---

## Execute

`struct` · `datafusion_expr::logical_plan::statement::Execute`

Also reachable as `datafusion::logical_expr::Execute`, `datafusion_expr::Execute`, `datafusion_expr::logical_plan::Execute`

```rust
struct Execute
```

**Fields**: `name`, `parameters`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.statement.Execute.md).


Execute a prepared statement.

---

## Prepare

`struct` · `datafusion_expr::logical_plan::statement::Prepare`

Also reachable as `datafusion::logical_expr::Prepare`, `datafusion_expr::Prepare`, `datafusion_expr::logical_plan::Prepare`

```rust
struct Prepare
```

**Fields**: `name`, `fields`, `input`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.statement.Prepare.md).


Prepare a statement but do not execute it. Prepare statements can have 0 or more
`Expr::Placeholder` expressions that are filled in during execution

---

## ResetVariable

`struct` · `datafusion_expr::logical_plan::statement::ResetVariable`

Also reachable as `datafusion::logical_expr::ResetVariable`, `datafusion_expr::ResetVariable`, `datafusion_expr::logical_plan::ResetVariable`

```rust
struct ResetVariable
```

**Fields**: `variable`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.statement.ResetVariable.md).


Reset a configuration variable to its default

---

## SetVariable

`struct` · `datafusion_expr::logical_plan::statement::SetVariable`

Also reachable as `datafusion::logical_expr::SetVariable`, `datafusion_expr::SetVariable`, `datafusion_expr::logical_plan::SetVariable`

```rust
struct SetVariable
```

**Fields**: `variable`, `value`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.statement.SetVariable.md).


Set a Variable's value -- value in
[`ConfigOptions`](datafusion_common::config::ConfigOptions)

---

## TransactionEnd

`struct` · `datafusion_expr::logical_plan::statement::TransactionEnd`

Also reachable as `datafusion::logical_expr::TransactionEnd`, `datafusion_expr::TransactionEnd`, `datafusion_expr::logical_plan::TransactionEnd`

```rust
struct TransactionEnd
```

**Fields**: `conclusion`, `chain`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.statement.TransactionEnd.md).


Indicator that any current transaction should be terminated

---

## TransactionStart

`struct` · `datafusion_expr::logical_plan::statement::TransactionStart`

Also reachable as `datafusion::logical_expr::TransactionStart`, `datafusion_expr::TransactionStart`, `datafusion_expr::logical_plan::TransactionStart`

```rust
struct TransactionStart
```

**Fields**: `access_mode`, `isolation_level`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.logical_plan.statement.TransactionStart.md).


Indicator that the following statements should be committed or rolled back atomically

---
