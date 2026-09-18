# `buoyant_kernel::history_manager::error::LogHistoryError::Internal`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.history_manager.error.LogHistoryError.Internal.json).

<a id="op-1e3007376b89f14284a7e8c8"></a>
## context

`struct_field` · `buoyant_kernel::history_manager::error::LogHistoryError::Internal::context` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
context: &'static str
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L99).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:99`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Description of the operation that failed.

<a id="op-0d9883dcebc4fd271fe60224"></a>
## source

`struct_field` · `buoyant_kernel::history_manager::error::LogHistoryError::Internal::source` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: Option<Box<Error>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L102).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:102`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The underlying error, if any.
