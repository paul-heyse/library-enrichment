# `datafusion_proto::logical_plan::AsLogicalPlan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.AsLogicalPlan.json).

<a id="op-a1e9538edd19e13dc513d59a"></a>
## AsLogicalPlan

`trait` · `datafusion_proto::logical_plan::AsLogicalPlan` · datafusion-proto 55.1.0

```rust
trait AsLogicalPlan: Debug + Send + Sync + Clone
```

Source: `src/logical_plan/mod.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30e19d5e9ce3819f41b5f2e7"></a>
## try_decode

`function` · `datafusion_proto::logical_plan::AsLogicalPlan::try_decode` · datafusion-proto 55.1.0

```rust
fn try_decode(buf: &[u8]) -> Result<Self> where Self: Sized
```

Source: `src/logical_plan/mod.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3862244fb1f9fe7da6b99814"></a>
## try_encode

`function` · `datafusion_proto::logical_plan::AsLogicalPlan::try_encode` · datafusion-proto 55.1.0

```rust
fn try_encode<B>(&self, buf: &mut B) -> Result<()> where B: BufMut, Self: Sized
```

Source: `src/logical_plan/mod.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1003ff778d24eee94529be40"></a>
## try_from_logical_plan

`function` · `datafusion_proto::logical_plan::AsLogicalPlan::try_from_logical_plan` · datafusion-proto 55.1.0

```rust
fn try_from_logical_plan(plan: &LogicalPlan, extension_codec: &dyn LogicalExtensionCodec) -> Result<Self> where Self: Sized
```

Source: `src/logical_plan/mod.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fff51e9ffcf46c9b135001b6"></a>
## try_into_logical_plan

`function` · `datafusion_proto::logical_plan::AsLogicalPlan::try_into_logical_plan` · datafusion-proto 55.1.0

```rust
fn try_into_logical_plan(&self, ctx: &TaskContext, extension_codec: &dyn LogicalExtensionCodec) -> Result<LogicalPlan>
```

Source: `src/logical_plan/mod.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
