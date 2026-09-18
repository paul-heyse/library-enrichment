# `datafusion_ffi::tests::make_test_statistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.tests.make_test_statistics.json).

<a id="op-58981632c31081a2adfb3e74"></a>
## make_test_statistics

`function` · `datafusion_ffi::tests::make_test_statistics` · datafusion-ffi 55.1.0

```rust
fn make_test_statistics() -> datafusion_common::Statistics
```

Source: `src/tests/mod.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Returns canonical statistics used by both the producer and consumer sides of
the integration tests so round-trips can be asserted without hard-coding
the values in two places.
