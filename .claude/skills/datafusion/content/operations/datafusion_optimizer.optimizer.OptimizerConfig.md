# `datafusion_optimizer::optimizer::OptimizerConfig`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.optimizer.OptimizerConfig.json).

<a id="op-c99ba442ba62b2c8360b9201"></a>
## OptimizerConfig

`trait` · `datafusion_optimizer::optimizer::OptimizerConfig` · datafusion-optimizer 55.1.0

```rust
trait OptimizerConfig
```

Source: `src/optimizer.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Options to control the DataFusion Optimizer.

<a id="op-69fd491e0ebaa721d8c71eac"></a>
## alias_generator

`function` · `datafusion_optimizer::optimizer::OptimizerConfig::alias_generator` · datafusion-optimizer 55.1.0

```rust
fn alias_generator(&self) -> &Arc<AliasGenerator>
```

Source: `src/optimizer.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Return alias generator used to generate unique aliases for subqueries

<a id="op-58eefccb1240bbe49fddbbcf"></a>
## function_registry

`function` · `datafusion_optimizer::optimizer::OptimizerConfig::function_registry` · datafusion-optimizer 55.1.0

```rust
fn function_registry(&self) -> Option<&dyn FunctionRegistry>
```

Source: `src/optimizer.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b6cf9cc4ecabff19a133452"></a>
## options

`function` · `datafusion_optimizer::optimizer::OptimizerConfig::options` · datafusion-optimizer 55.1.0

```rust
fn options(&self) -> Arc<ConfigOptions>
```

Source: `src/optimizer.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce35594f596b26d5e269fdf9"></a>
## query_execution_start_time

`function` · `datafusion_optimizer::optimizer::OptimizerConfig::query_execution_start_time` · datafusion-optimizer 55.1.0

```rust
fn query_execution_start_time(&self) -> Option<DateTime<Utc>>
```

Source: `src/optimizer.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Return the time at which the query execution started. This
time is used as the value for `now()`. If `None`, time-dependent
functions like `now()` will not be simplified during optimization.
