# `datafusion_functions_aggregate_common::tdigest`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.tdigest.json).

<a id="op-f4b8fac84cf5d1c82809b884"></a>
## tdigest

`module` · `datafusion_functions_aggregate_common::tdigest` · datafusion-functions-aggregate-common 55.1.0

```rust
mod tdigest
```

Source: `src/tdigest.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

An implementation of the [TDigest sketch algorithm] providing approximate
quantile calculations.

The TDigest code in this module is modified from
<https://github.com/MnO2/t-digest>, itself a rust reimplementation of
[Facebook's Folly TDigest] implementation.

Alterations include reduction of runtime heap allocations, broader type
support, (de-)serialization support, reduced type conversions and null value
tolerance.

[TDigest sketch algorithm]: https://arxiv.org/abs/1902.04023
[Facebook's Folly TDigest]: https://github.com/facebook/folly/blob/main/folly/stats/TDigest.h
