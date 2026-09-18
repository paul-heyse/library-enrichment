# `datafusion_physical_optimizer::join_selection::PipelineFixerSubrule`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.join_selection.PipelineFixerSubrule.json).

<a id="op-fa5d788dfee178a0bfbe2975"></a>
## PipelineFixerSubrule

`type_alias` · `datafusion_physical_optimizer::join_selection::PipelineFixerSubrule` · datafusion-physical-optimizer 55.1.0

```rust
type PipelineFixerSubrule = dyn Fn(std::sync::Arc<dyn ExecutionPlan>, &datafusion_common::config::ConfigOptions) -> datafusion_common::error::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Source: `src/join_selection.rs:376`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Pipeline-fixing join selection subrule.
