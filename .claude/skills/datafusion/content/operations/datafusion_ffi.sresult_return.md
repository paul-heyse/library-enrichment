# `datafusion_ffi::sresult_return`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.sresult_return.json).

<a id="op-90427ad702f6df337167544e"></a>
## sresult_return

`macro` · `datafusion_ffi::sresult_return` · datafusion-ffi 55.1.0

```rust
macro_rules! sresult_return
```

Source: `src/util.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This macro is a helpful conversion utility to convert from a DataFusion Result to an FFI_Result
and to also call return when it is an error. Since you cannot use `?` on an FFI_Result, this is designed
to mimic the pattern.
