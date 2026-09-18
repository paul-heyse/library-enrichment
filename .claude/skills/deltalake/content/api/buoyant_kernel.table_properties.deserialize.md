# `buoyant_kernel::table_properties::deserialize`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.table_properties.deserialize.json`](../model/buoyant_kernel.table_properties.deserialize.json)

## ParseIntervalError

`enum` · `buoyant_kernel::table_properties::deserialize::ParseIntervalError`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.table_properties.deserialize.ParseIntervalError.md)

Also reachable as `buoyant_kernel::table_properties::ParseIntervalError`, `delta_kernel::table_properties::deserialize::ParseIntervalError`

```rust
enum ParseIntervalError
```

**Variants**: `NotAnInterval`, `ParseIntError`, `NegativeInterval`, `UnsupportedInterval`, `UnknownUnit`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---
