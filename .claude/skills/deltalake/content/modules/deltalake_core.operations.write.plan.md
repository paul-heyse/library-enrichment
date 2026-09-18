# `deltalake_core::operations::write::plan`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.write.plan.json).

<a id="op-85deed4ff230c9f5b9a771cf"></a>
## plan

`module` · `deltalake_core::operations::write::plan` · deltalake-core 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod plan
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/plan.rs#L1).

Source: `crates/core/src/operations/write/plan.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Write planning has two stages.
`prepare_write` normalizes incoming rows into table shaped insert data and resolves exact
validations against that prepared schema.
`plan_overwrite_rewrite` builds a typed overwrite rewrite plan that keeps matched existing
files, rescue planning, and CDC composition explicit until commit assembly.
