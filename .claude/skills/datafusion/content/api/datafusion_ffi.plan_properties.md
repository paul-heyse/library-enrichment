# `datafusion_ffi::plan_properties`

Crate `datafusion-ffi` · 3 public items · structured records in [`model/datafusion_ffi.plan_properties.json`](../model/datafusion_ffi.plan_properties.json)

## FFI_Boundedness

`enum` · `datafusion_ffi::plan_properties::FFI_Boundedness`

```rust
enum FFI_Boundedness
```

**Variants**: `Bounded`, `Unbounded`

**Implements**: `core::convert::From`

**Derives**: Clone

**via `core::convert::From`**

```rust
fn from(value: Boundedness) -> Self
```

FFI safe version of [`Boundedness`].

---

## FFI_EmissionType

`enum` · `datafusion_ffi::plan_properties::FFI_EmissionType`

```rust
enum FFI_EmissionType
```

**Variants**: `Incremental`, `Final`, `Both`

**Implements**: `core::convert::From`

**Derives**: Clone

**via `core::convert::From`**

```rust
fn from(value: EmissionType) -> Self
```

FFI safe version of [`EmissionType`].

---

## FFI_PlanProperties

`struct` · `datafusion_ffi::plan_properties::FFI_PlanProperties`

```rust
struct FFI_PlanProperties
```

**Fields**: `output_partitioning`, `emission_type`, `boundedness`, `output_ordering`, `schema`, `release`, `private_data`, `library_marker_id`

**Implements**: `core::convert::From`, `core::ops::drop::Drop`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(props: &PlanProperties) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A stable struct for sharing [`PlanProperties`] across FFI boundaries.

---
