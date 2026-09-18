# `datafusion_optimizer::extract_equijoin_predicate`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.extract_equijoin_predicate.json`](../model/datafusion_optimizer.extract_equijoin_predicate.json)

## ExtractEquijoinPredicate

`struct` · `datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate`

```rust
struct ExtractEquijoinPredicate
```

**Implements**: `datafusion_optimizer::optimizer::OptimizerRule`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_optimizer::optimizer::OptimizerRule`**

```rust
fn apply_order(&self) -> Option<ApplyOrder>
fn name(&self) -> &str
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
fn supports_rewrite(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.extract_equijoin_predicate.ExtractEquijoinPredicate.md).


Optimizer that splits conjunctive join predicates into equijoin
predicates and (other) filter predicates.

Join algorithms are often highly optimized for equality predicates such as `x = y`,
often called `equijoin` predicates, so it is important to locate such predicates
and treat them specially.

For example, `SELECT ... FROM A JOIN B ON (A.x = B.y AND B.z > 50)`
has one equijoin predicate (`A.x = B.y`) and one filter predicate (`B.z > 50`).
See [find_valid_equijoin_key_pair] for more information on what predicates
are considered equijoins.

---
