# `datafusion_physical_optimizer::sanity_checker::check_finiteness_requirements`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.sanity_checker.check_finiteness_requirements.json).

<a id="op-1d32939d333d359d76cd756c"></a>
## check_finiteness_requirements

`function` · `datafusion_physical_optimizer::sanity_checker::check_finiteness_requirements` · datafusion-physical-optimizer 55.1.0

```rust
fn check_finiteness_requirements(input: &dyn ExecutionPlan, optimizer_options: &datafusion_common::config::OptimizerOptions) -> datafusion_common::Result<()>
```

Source: `src/sanity_checker.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This function propagates finiteness information and rejects any plan with
pipeline-breaking operators acting on infinite inputs.
