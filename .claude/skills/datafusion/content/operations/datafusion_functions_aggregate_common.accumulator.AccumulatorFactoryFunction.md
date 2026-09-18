# `datafusion_functions_aggregate_common::accumulator::AccumulatorFactoryFunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.accumulator.AccumulatorFactoryFunction.json).

<a id="op-62ddde0e4f97bc1a8dbb2223"></a>
## AccumulatorFactoryFunction

`type_alias` · `datafusion_functions_aggregate_common::accumulator::AccumulatorFactoryFunction` · datafusion-functions-aggregate-common 55.1.0

```rust
type AccumulatorFactoryFunction = std::sync::Arc<dyn Fn(AccumulatorArgs<'_>) -> datafusion_common::Result<Box<dyn Accumulator>> + Send + Sync>
```

Source: `src/accumulator.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Factory that returns an accumulator for the given aggregate function.
