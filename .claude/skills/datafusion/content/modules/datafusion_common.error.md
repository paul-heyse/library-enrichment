# `datafusion_common::error`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.error.json).

<a id="op-75f74b3fbecf28572a19063c"></a>
## error

`module` · `datafusion_common::error` · datafusion-common 55.1.0

```rust
mod error
```

Source: `src/error.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

# Error Handling in DataFusion

In DataFusion, there are two types of errors that can be raised:

1. Expected errors – These indicate invalid operations performed by the caller,
   such as attempting to open a non-existent file. Different categories exist to
   distinguish their sources (e.g., [`DataFusionError::ArrowError`](../operations/datafusion_common.error.DataFusionError.md#op-5ea8745e5c5bfce4154f3662),
   [`DataFusionError::IoError`](../operations/datafusion_common.error.DataFusionError.md#op-ed9d0eb2b7b6eb692654571f), etc.).

2. Unexpected errors – Represented by [`DataFusionError::Internal`](../operations/datafusion_common.error.DataFusionError.md#op-bdc3e1c5a07020f01c5a1dc4), these
   indicate that an internal invariant has been broken, suggesting a potential
   bug in the system.

There are several convenient macros for throwing errors. For example, use
`exec_err!` for expected errors.
For invariant checks, you can use `assert_or_internal_err!`,
`assert_eq_or_internal_err!`, `assert_ne_or_internal_err!` for easier assertions.
On the performance-critical path, use `debug_assert!` instead to reduce overhead.
