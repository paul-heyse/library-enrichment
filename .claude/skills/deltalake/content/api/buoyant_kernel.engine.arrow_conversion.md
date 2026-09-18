# `buoyant_kernel::engine::arrow_conversion`

Crate `buoyant_kernel` · 4 public items · structured records in [`model/buoyant_kernel.engine.arrow_conversion.json`](../model/buoyant_kernel.engine.arrow_conversion.json)

## TryFromArrow

`trait` · `buoyant_kernel::engine::arrow_conversion::TryFromArrow`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_conversion.TryFromArrow.md)

Also reachable as `delta_kernel::engine::arrow_conversion::TryFromArrow`

```rust
trait TryFromArrow<ArrowType>: Sized
```

**Implementors** (3)

- `buoyant_kernel::schema::DataType`
- `buoyant_kernel::schema::StructField`
- `buoyant_kernel::schema::StructType`

**Methods** (1)

```rust
fn try_from_arrow(t: ArrowType) -> Result<Self, ArrowError>
```

Convert an arrow type into a kernel type (a similar [`TryIntoKernel`] trait is automatically
implemented for all types that implement [`TryFromArrow`])

---

## TryFromKernel

`trait` · `buoyant_kernel::engine::arrow_conversion::TryFromKernel`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_conversion.TryFromKernel.md)

Also reachable as `delta_kernel::engine::arrow_conversion::TryFromKernel`

```rust
trait TryFromKernel<KernelType>: Sized
```

**Implementors** (3)

- `arrow_schema::datatype::DataType`
- `arrow_schema::field::Field`
- `arrow_schema::schema::Schema`

**Methods** (1)

```rust
fn try_from_kernel(t: KernelType) -> Result<Self, ArrowError>
```

Convert a kernel type into an arrow type (a similar [`TryIntoArrow`] trait is automatically
implemented for all types that implement [`TryFromKernel`])

---

## TryIntoArrow

`trait` · `buoyant_kernel::engine::arrow_conversion::TryIntoArrow`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_conversion.TryIntoArrow.md)

Also reachable as `delta_kernel::engine::arrow_conversion::TryIntoArrow`

```rust
trait TryIntoArrow<ArrowType>
```

**Methods** (1)

```rust
fn try_into_arrow(self) -> Result<ArrowType, ArrowError>
```

Convert a kernel type into an arrow type (automatically implemented for all types that
implement [`TryFromKernel`])

---

## TryIntoKernel

`trait` · `buoyant_kernel::engine::arrow_conversion::TryIntoKernel`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_conversion.TryIntoKernel.md)

Also reachable as `delta_kernel::engine::arrow_conversion::TryIntoKernel`

```rust
trait TryIntoKernel<KernelType>
```

**Methods** (1)

```rust
fn try_into_kernel(self) -> Result<KernelType, ArrowError>
```

Convert an arrow type into a kernel type (automatically implemented for all types that
implement [`TryFromArrow`])

---
