# `datafusion_session::physical_optimizer::PhysicalOptimizerContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.physical_optimizer.PhysicalOptimizerContext.json).

<a id="op-3296df92ae1d4db86475371d"></a>
## PhysicalOptimizerContext

`trait` · `datafusion_session::physical_optimizer::PhysicalOptimizerContext` · datafusion-session 55.1.0

```rust
trait PhysicalOptimizerContext: Send + Sync
```

Source: `src/physical_optimizer.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Context available to physical optimizer rules.

This trait provides access to configuration options and an optional statistics
registry for enhanced statistics lookup.

<a id="op-195c114b8099c6239ad1b5d7"></a>
## config_options

`function` · `datafusion_session::physical_optimizer::PhysicalOptimizerContext::config_options` · datafusion-session 55.1.0

```rust
fn config_options(&self) -> &ConfigOptions
```

Source: `src/physical_optimizer.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Returns the configuration options.

<a id="op-049929a0d86d07b03b117542"></a>
## statistics_registry

`function` · `datafusion_session::physical_optimizer::PhysicalOptimizerContext::statistics_registry` · datafusion-session 55.1.0

```rust
fn statistics_registry(&self) -> Option<&StatisticsRegistry>
```

Source: `src/physical_optimizer.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Returns the statistics registry for enhanced statistics lookup.

Returns `None` if no registry is configured, in which case rules
should fall back to using [`ExecutionPlan::partition_statistics`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-6923c0e0a9951b41d7be05cd).
