# `datafusion_ffi::version`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.version.json).

<a id="op-f2538eeb0331c6d9d71e32df"></a>
## version

`function` · `datafusion_ffi::version` · datafusion-ffi 55.1.0

```rust
extern "C" fn version() -> u64
```

Source: `src/lib.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Returns the major version of the FFI implementation. If the API evolves,
we use the major version to identify compatibility over the unsafe
boundary. This call is intended to be used by implementers to validate
they have compatible libraries.
