# `buoyant_kernel::engine::arrow_conversion::scalar`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_conversion.scalar.json).

<a id="op-8d1c6fa555e49ff4bd37b4e4"></a>
## scalar

`module` · `buoyant_kernel::engine::arrow_conversion::scalar` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod scalar
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/scalar.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/scalar.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Extract a kernel [`Scalar`] from a single row of an Arrow array.

# Supported types

All Delta primitive types are supported: Integer, Long, Short, Byte, Float, Double,
Boolean, String, Date, Timestamp, TimestampNtz, Decimal, Binary (including `LargeUtf8`
and `LargeBinary` Arrow variants).

Complex types (Struct, Array, Map) are not supported and return an error.

[`Scalar`]: crate::expressions::Scalar
