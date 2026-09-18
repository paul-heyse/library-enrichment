# `datafusion_ffi::tests::catalog`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.tests.catalog.json).

<a id="op-0a968e21add36f7b0fb491a2"></a>
## catalog

`module` · `datafusion_ffi::tests::catalog` · datafusion-ffi 55.1.0

```rust
mod catalog
```

Source: `src/tests/catalog.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This is an example of an async table provider that will call functions on
the tokio runtime of the library providing the function. Since we cannot
share a tokio runtime across the ffi boundary and the producer and consumer
may have different runtimes, we need to store a reference to the runtime
and enter it during streaming calls. The entering of the runtime will
occur by the datafusion_ffi crate during the streaming calls. This code
serves as an integration test of this feature. If we do not correctly
access the runtime, then you will get a panic when trying to do operations
such as spawning a tokio task.
