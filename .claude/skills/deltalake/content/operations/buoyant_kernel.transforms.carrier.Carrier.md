# `buoyant_kernel::transforms::carrier::Carrier`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transforms.carrier.Carrier.json).

<a id="op-329287d77829220b89182e9d"></a>
## Carrier

`trait` · `buoyant_kernel::transforms::carrier::Carrier` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait Carrier<'a, T: ToOwned + ?Sized>: Sized
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/carrier.rs#L18).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/carrier.rs:18`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Carrier abstraction for transform outputs.

A carrier can represent one of two outcomes for a transformed node:
- present value (`Cow<'a, T>`)
- residual outcome (`Self::Residual`)

Depending on carrier choice, a residual encodes filtered-out and/or error cases. A non-filtering
infallible carrier has `Residual = Infallible`.

Different carriers encode those states differently:
- `()` and `Result<(), E>` are carriers for read-only visitor transforms that return no value
- `Cow<'a, T>` and `Result<Cow<'a, T>, E>` are non-filtering carriers
- `Option<Cow<'a, T>>` and `Result<Option<Cow<'a, T>>, E>` are filtering carriers

<a id="op-cd92ec992b2cb95a18f74764"></a>
## NONE

`assoc_const` · `buoyant_kernel::transforms::carrier::Carrier::NONE` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
NONE
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/carrier.rs#L23).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/carrier.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

For filtering carriers, Some carrier containing a residual value equivalent to
`Option::None`. None for non-filtering carriers.

<a id="op-4a759990b6417539c3a6ffeb"></a>
## Residual

`assoc_type` · `buoyant_kernel::transforms::carrier::Carrier::Residual` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Residual
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/carrier.rs#L19).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/carrier.rs:19`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29ca8dffcb718d3d76787b79"></a>
## from_inner

`function` · `buoyant_kernel::transforms::carrier::Carrier::from_inner` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_inner(inner: Cow<'a, T>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/carrier.rs#L26).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/carrier.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Wrap a present transformed node.

<a id="op-e4936caf63fd3b50a4b23bb2"></a>
## from_residual

`function` · `buoyant_kernel::transforms::carrier::Carrier::from_residual` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_residual(residual: Self::Residual) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/carrier.rs#L28).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/carrier.rs:28`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Wrap a residual outcome for this carrier (e.g. None or Err).

<a id="op-200b2a4a74ef7a073bb3014b"></a>
## into_inner

`function` · `buoyant_kernel::transforms::carrier::Carrier::into_inner` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_inner(self) -> Result<Cow<'a, T>, Self::Residual>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/carrier.rs#L32).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/carrier.rs:32`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Extracts a present node or returns a residual.

Filtering and/or fallible carriers encode "filtered out" and "error" in the residual side.

<a id="op-cb26da915da8565ee647c8c7"></a>
## into_inner_opt

`function` · `buoyant_kernel::transforms::carrier::Carrier::into_inner_opt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_inner_opt(self) -> Result<Option<Cow<'a, T>>, Self::Residual>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/carrier.rs#L38).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/carrier.rs:38`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Extracts an optional node or returns a residual.

This bridges filtering and non-filtering carriers into a common shape:
- filtering carriers may return `Ok(None)` to indicate "filtered out"
- non-filtering carriers always return `Ok(Some(_))`
