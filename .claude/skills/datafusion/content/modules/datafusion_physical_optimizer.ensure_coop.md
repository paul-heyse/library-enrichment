# `datafusion_physical_optimizer::ensure_coop`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_coop.json).

<a id="op-c818c84dc107a050f11e18b2"></a>
## ensure_coop

`module` · `datafusion_physical_optimizer::ensure_coop` · datafusion-physical-optimizer 55.1.0

```rust
mod ensure_coop
```

Source: `src/ensure_coop.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

The [`EnsureCooperative`](../operations/datafusion_physical_optimizer.ensure_coop.EnsureCooperative.md#op-fe53237dbfc32d9e59d026ba) optimizer rule inspects the physical plan to find all
portions of the plan that will not yield cooperatively.
It will insert `CooperativeExec` nodes where appropriate to ensure execution plans
always yield cooperatively.
