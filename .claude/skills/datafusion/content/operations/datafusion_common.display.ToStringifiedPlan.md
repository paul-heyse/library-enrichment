# `datafusion_common::display::ToStringifiedPlan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.display.ToStringifiedPlan.json).

<a id="op-938e6ec5ffbc89a866704e89"></a>
## ToStringifiedPlan

`trait` · `datafusion_common::display::ToStringifiedPlan` · datafusion-common 55.1.0

```rust
trait ToStringifiedPlan
```

Source: `src/display/mod.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Trait for something that can be formatted as a stringified plan

<a id="op-c58271a48ef33ef42c458836"></a>
## to_stringified

`function` · `datafusion_common::display::ToStringifiedPlan::to_stringified` · datafusion-common 55.1.0

```rust
fn to_stringified(&self, plan_type: PlanType) -> StringifiedPlan
```

Source: `src/display/mod.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a stringified plan with the specified type
