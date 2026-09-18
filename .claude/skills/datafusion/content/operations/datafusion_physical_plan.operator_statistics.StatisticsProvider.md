# `datafusion_physical_plan::operator_statistics::StatisticsProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.StatisticsProvider.json).

<a id="op-b3050eb3d6b996dd9e336fb5"></a>
## StatisticsProvider

`trait` · `datafusion_physical_plan::operator_statistics::StatisticsProvider` · datafusion-physical-plan 55.1.0

```rust
trait StatisticsProvider: Debug + Send + Sync
```

Source: `src/operator_statistics/mod.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Customize statistics computation for [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) nodes.

Implementations can handle specific operator types or override default
estimation logic. The chain of providers is traversed until one returns
[`StatisticsResult::Computed`](../operations/datafusion_physical_plan.operator_statistics.StatisticsResult.md#op-8692bf67418955a391809cf5).

# Implementing a Custom Provider

```ignore
#[derive(Debug)]
struct MyStatisticsProvider;

impl StatisticsProvider for MyStatisticsProvider {
    fn compute_statistics(
        &self,
        plan: &dyn ExecutionPlan,
        child_stats: &[ExtendedStatistics],
    ) -> Result<StatisticsResult> {
        if let Some(my_exec) = plan.downcast_ref::<MyCustomExec>() {
            // Custom logic for MyCustomExec
            Ok(StatisticsResult::Computed(/* ... */))
        } else {
            // Let next provider handle it
            Ok(StatisticsResult::Delegate)
        }
    }
}
```

<a id="op-603c16aba2560c74e71c13be"></a>
## compute_statistics

`function` · `datafusion_physical_plan::operator_statistics::StatisticsProvider::compute_statistics` · datafusion-physical-plan 55.1.0

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Source: `src/operator_statistics/mod.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Compute statistics for an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) node.

# Arguments
* `plan` - The execution plan node to compute statistics for
* `child_stats` - Extended statistics already computed for child nodes,
  in the same order as `plan.children()`. Empty for leaf nodes.

# Returns
* `StatisticsResult::Computed(stats)` - Short-circuits the chain
* `StatisticsResult::Delegate` - Passes to next provider in chain
