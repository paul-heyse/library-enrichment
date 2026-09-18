# `datafusion_physical_optimizer::output_requirements`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.output_requirements.json).

<a id="op-84e65716bccc6e7af3218531"></a>
## output_requirements

`module` · `datafusion_physical_optimizer::output_requirements` · datafusion-physical-optimizer 55.1.0

```rust
mod output_requirements
```

Source: `src/output_requirements.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

The GlobalOrderRequire optimizer rule either:
- Adds an auxiliary `OutputRequirementExec` operator to keep track of global
  ordering and distribution requirement across rules, or
- Removes the auxiliary `OutputRequirementExec` operator from the physical plan.
  Since the `OutputRequirementExec` operator is only a helper operator, it
  shouldn't occur in the final plan (i.e. the executed plan).
