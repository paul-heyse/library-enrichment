# `buoyant_kernel::actions::schema_contains_file_actions`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.schema_contains_file_actions.json).

<a id="op-1a4c69300000f893d4ab94d3"></a>
## schema_contains_file_actions

`function` · `buoyant_kernel::actions::schema_contains_file_actions` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn schema_contains_file_actions(schema: &schema::SchemaRef) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L183).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:183`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns true if the schema contains file actions (add or remove)
columns.
