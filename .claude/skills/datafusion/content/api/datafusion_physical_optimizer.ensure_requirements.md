# `datafusion_physical_optimizer::ensure_requirements`

Crate `datafusion-physical-optimizer` · 1 public items · structured records in [`model/datafusion_physical_optimizer.ensure_requirements.json`](../model/datafusion_physical_optimizer.ensure_requirements.json)

## EnsureRequirements

`struct` · `datafusion_physical_optimizer::ensure_requirements::EnsureRequirements`

```rust
struct EnsureRequirements
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_session::physical_optimizer::PhysicalOptimizerRule`**

```rust
fn name(&self) -> &str
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

Optimizer rule that enforces both distribution and sorting requirements.

This rule combines the functionality of `EnforceDistribution` and
`EnforceSorting` into a coordinated sequence where distribution is
always settled before sorting for each operator, preventing the
non-idempotent interactions between the two separate rules.

See [module level documentation](self) for more details.

---
