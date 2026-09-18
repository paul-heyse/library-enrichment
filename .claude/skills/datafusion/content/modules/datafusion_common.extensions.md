# `datafusion_common::extensions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.extensions.json).

<a id="op-a0d5f66bae93763f5731dfd1"></a>
## extensions

`module` · `datafusion_common::extensions` · datafusion-common 55.1.0

```rust
mod extensions
```

Source: `src/extensions.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A type-keyed map of opaque, `Arc`'d objects.

Used as the backing store for the various `extensions` fields throughout
DataFusion (e.g. [`SessionConfig`], [`ExtendedStatistics`],
[`PartitionedFile`]) so that independent components can each attach
their own data without conflict, each keyed by its concrete Rust type.

[`SessionConfig`]: https://docs.rs/datafusion-execution/latest/datafusion_execution/config/struct.SessionConfig.html
[`ExtendedStatistics`]: https://docs.rs/datafusion-physical-plan/latest/datafusion_physical_plan/operator_statistics/struct.ExtendedStatistics.html
[`PartitionedFile`]: https://docs.rs/datafusion-datasource/latest/datafusion_datasource/struct.PartitionedFile.html
