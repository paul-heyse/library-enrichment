# `datafusion_common::utils::get_available_parallelism`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.get_available_parallelism.json).

<a id="op-e27670a06d385fef807fa464"></a>
## get_available_parallelism

`function` · `datafusion_common::utils::get_available_parallelism` · datafusion-common 55.1.0

```rust
fn get_available_parallelism() -> usize
```

Source: `src/utils/mod.rs:1151`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the estimated number of threads available for parallel execution.

This is a wrapper around `std::thread::available_parallelism`, providing a default value
of `1` if the system's parallelism cannot be determined.

The result is cached after the first call.
