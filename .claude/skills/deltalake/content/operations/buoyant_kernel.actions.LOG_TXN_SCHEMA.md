# `buoyant_kernel::actions::LOG_TXN_SCHEMA`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.LOG_TXN_SCHEMA.json).

<a id="op-c55d73e50f48d071d105c0f8"></a>
## LOG_TXN_SCHEMA

`static` · `buoyant_kernel::actions::LOG_TXN_SCHEMA` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
static LOG_TXN_SCHEMA: std::sync::LazyLock<schema::SchemaRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L159).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:159`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Schema for transaction (txn) actions in the Delta log.
Wraps the SetTransaction schema in a top-level struct with "txn" field name.
