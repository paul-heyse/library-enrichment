# `deltalake_core::kernel::scalars::ScalarExt`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.scalars.ScalarExt.json).

<a id="op-f9d338becc50c96202828938"></a>
## ScalarExt

`trait` · `deltalake_core::kernel::scalars::ScalarExt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait ScalarExt: Sized
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/scalars.rs#L56).

Source: `crates/core/src/kernel/scalars.rs:56`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Auxiliary methods for dealing with kernel scalars

<a id="op-c6752bbbe6f10a0d8b075db4"></a>
## from_array

`function` · `deltalake_core::kernel::scalars::ScalarExt::from_array` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_array(arr: &dyn Array, index: usize) -> Option<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/scalars.rs#L62).

Source: `crates/core/src/kernel/scalars.rs:62`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a [`Scalar`](../operations/buoyant_kernel.expressions.scalars.Scalar.md#op-8dd45baeed3da91aa357441a) from an arrow array row

<a id="op-a70b61d46a560516604629ad"></a>
## serialize

`function` · `deltalake_core::kernel::scalars::ScalarExt::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize(&self) -> String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/scalars.rs#L58).

Source: `crates/core/src/kernel/scalars.rs:58`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Serialize to string

<a id="op-107c2d3db0c10cafa5234c00"></a>
## serialize_encoded

`function` · `deltalake_core::kernel::scalars::ScalarExt::serialize_encoded` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize_encoded(&self) -> String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/scalars.rs#L60).

Source: `crates/core/src/kernel/scalars.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Serialize to string for use in hive partition file names

<a id="op-ded368877cc1931f17aab2c2"></a>
## to_json

`function` · `deltalake_core::kernel::scalars::ScalarExt::to_json` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_json(&self) -> Value
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/scalars.rs#L64).

Source: `crates/core/src/kernel/scalars.rs:64`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Serialize as serde_json::Value
