# `buoyant_kernel::transforms::carrier`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.transforms.carrier.json`](../model/buoyant_kernel.transforms.carrier.json)

## Carrier

`trait` · `buoyant_kernel::transforms::carrier::Carrier`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.transforms.carrier.Carrier.md)

Also reachable as `buoyant_kernel::transforms::Carrier`, `delta_kernel::transforms::carrier::Carrier`

```rust
trait Carrier<'a, T: ToOwned + ?Sized>: Sized
```

**Implementors** (3)

- `alloc::borrow::Cow`
- `core::option::Option`
- `core::result::Result`

**Methods** (4)

```rust
fn from_inner(inner: Cow<'a, T>) -> Self
fn from_residual(residual: Self::Residual) -> Self
fn into_inner(self) -> Result<Cow<'a, T>, Self::Residual>
fn into_inner_opt(self) -> Result<Option<Cow<'a, T>>, Self::Residual>
```

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

---
