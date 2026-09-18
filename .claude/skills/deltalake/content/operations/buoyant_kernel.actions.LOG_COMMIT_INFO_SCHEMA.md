# `buoyant_kernel::actions::LOG_COMMIT_INFO_SCHEMA`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.LOG_COMMIT_INFO_SCHEMA.json).

<a id="op-980d8db4e5f83d9cf6bea2e4"></a>
## LOG_COMMIT_INFO_SCHEMA

`static` · `buoyant_kernel::actions::LOG_COMMIT_INFO_SCHEMA` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
static LOG_COMMIT_INFO_SCHEMA: std::sync::LazyLock<schema::SchemaRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L153).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:153`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Schema for CommitInfo actions in the Delta log.
Wraps the CommitInfo schema in a top-level struct with "commitInfo" field name.
