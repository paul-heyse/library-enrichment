# `deltalake_core::kernel::schema::schema::StructTypeExt`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.schema.schema.StructTypeExt.json).

<a id="op-31f430dd51887c26e6e6f14a"></a>
## StructTypeExt

`trait` · `deltalake_core::kernel::schema::schema::StructTypeExt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait StructTypeExt
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/schema.rs#L55).

Source: `crates/core/src/kernel/schema/schema.rs:55`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Trait to add convenience functions to struct type

<a id="op-3e9b2ad05fb6197fc062ff45"></a>
## get_generated_columns

`function` · `deltalake_core::kernel::schema::schema::StructTypeExt::get_generated_columns` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_generated_columns(&self) -> Result<Vec<GeneratedColumn>, Error>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/schema.rs#L60).

Source: `crates/core/src/kernel/schema/schema.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get all generated column expressions

<a id="op-97b673d011917189ba83eba5"></a>
## get_invariants

`function` · `deltalake_core::kernel::schema::schema::StructTypeExt::get_invariants` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_invariants(&self) -> Result<Vec<Invariant>, Error>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/schema.rs#L57).

Source: `crates/core/src/kernel/schema/schema.rs:57`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get all invariants in the schemas
