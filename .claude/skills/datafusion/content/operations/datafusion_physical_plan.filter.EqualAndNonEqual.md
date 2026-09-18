# `datafusion_physical_plan::filter::EqualAndNonEqual`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.filter.EqualAndNonEqual.json).

<a id="op-39a24e4f58be32d99bc1b02e"></a>
## EqualAndNonEqual

`type_alias` · `datafusion_physical_plan::filter::EqualAndNonEqual` · datafusion-physical-plan 55.1.0

```rust
type EqualAndNonEqual<'a> = (Vec<PhysicalExprPairRef<'a>>, Vec<PhysicalExprPairRef<'a>>)
```

Source: `src/filter.rs:1407`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The equals Column-Pairs and Non-equals Column-Pairs in the Predicates
