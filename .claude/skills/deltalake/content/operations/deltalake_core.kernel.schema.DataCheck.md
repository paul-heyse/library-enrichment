# `deltalake_core::kernel::schema::DataCheck`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.schema.DataCheck.json).

<a id="op-db637476f32308dc05ec126b"></a>
## DataCheck

`trait` · `deltalake_core::kernel::schema::DataCheck` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait DataCheck
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/mod.rs#L12).

Source: `crates/core/src/kernel/schema/mod.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A trait for all kernel types that are used as part of data checking

<a id="op-4051d67ca13b9b8ad9152cbc"></a>
## as_any

`function` · `deltalake_core::kernel::schema::DataCheck::as_any` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_any(&self) -> &dyn Any
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/mod.rs#L19).

Source: `crates/core/src/kernel/schema/mod.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Upcast to [`Any`] so callers can downcast to the concrete check implementation.

Unresolved upstream links (retained, not inferred): ``Any``.

<a id="op-edf1c42a510dd59442ce587e"></a>
## get_expression

`function` · `deltalake_core::kernel::schema::DataCheck::get_expression` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_expression(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/mod.rs#L16).

Source: `crates/core/src/kernel/schema/mod.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The SQL expression to use for the check

<a id="op-077469f8a63122c317af9d74"></a>
## get_name

`function` · `deltalake_core::kernel::schema::DataCheck::get_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_name(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/mod.rs#L14).

Source: `crates/core/src/kernel/schema/mod.rs:14`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The name of the specific check
