# `buoyant_kernel::actions::get_commit_schema`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.get_commit_schema.json).

<a id="op-86256d3113b68849ac13e381"></a>
## get_commit_schema

`function` · `buoyant_kernel::actions::get_commit_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_commit_schema() -> &'static schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L169).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:169`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Gets the schema for all actions that can appear in commits
logs.  This excludes actions that can only appear in checkpoints.
