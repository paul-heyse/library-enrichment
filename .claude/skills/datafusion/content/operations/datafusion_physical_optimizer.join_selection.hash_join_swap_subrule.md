# `datafusion_physical_optimizer::join_selection::hash_join_swap_subrule`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.join_selection.hash_join_swap_subrule.json).

<a id="op-a2c0d97eb9955a8482a25849"></a>
## hash_join_swap_subrule

`function` · `datafusion_physical_optimizer::join_selection::hash_join_swap_subrule` · datafusion-physical-optimizer 55.1.0

```rust
fn hash_join_swap_subrule(input: std::sync::Arc<dyn ExecutionPlan>, _config_options: &datafusion_common::config::ConfigOptions) -> datafusion_common::error::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Source: `src/join_selection.rs:535`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This subrule will swap build/probe sides of a hash join depending on whether
one of its inputs may produce an infinite stream of records. The rule ensures
that the left (build) side of the hash join always operates on an input stream
that will produce a finite set of records. If the left side can not be chosen
to be "finite", the join sides stay the same as the original query.
```text
For example, this rule makes the following transformation:



          +--------------+              +--------------+
          |              |  unbounded   |              |
   Left   | Infinite     |    true      | Hash         |\true
          | Data source  |--------------| Repartition  | \   +--------------+       +--------------+
          |              |              |              |  \  |              |       |              |
          +--------------+              +--------------+   - |  Hash Join   |-------| Projection   |
                                                           - |              |       |              |
          +--------------+              +--------------+  /  +--------------+       +--------------+
          |              |  unbounded   |              | /
   Right  | Finite       |    false     | Hash         |/false
          | Data Source  |--------------| Repartition  |
          |              |              |              |
          +--------------+              +--------------+



          +--------------+              +--------------+
          |              |  unbounded   |              |
   Left   | Finite       |    false     | Hash         |\false
          | Data source  |--------------| Repartition  | \   +--------------+       +--------------+
          |              |              |              |  \  |              | true  |              | true
          +--------------+              +--------------+   - |  Hash Join   |-------| Projection   |-----
                                                           - |              |       |              |
          +--------------+              +--------------+  /  +--------------+       +--------------+
          |              |  unbounded   |              | /
   Right  | Infinite     |    true      | Hash         |/true
          | Data Source  |--------------| Repartition  |
          |              |              |              |
          +--------------+              +--------------+
```
