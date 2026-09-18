# `arrow_select::coalesce`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.coalesce.json).

<a id="op-52efa955cfdbb9bb2d26eae1"></a>
## coalesce

`module` · `arrow_select::coalesce` · arrow-select 59.3.0

```rust
mod coalesce
```

Source: `src/coalesce.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

[`BatchCoalescer`](../operations/arrow_select.coalesce.BatchCoalescer.md#op-76e0a8f9b55b024998e5f424)  concatenates multiple [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es after
operations such as [`filter`] and [`take`].

[`filter`]: crate::filter::filter
[`take`]: crate::take::take
