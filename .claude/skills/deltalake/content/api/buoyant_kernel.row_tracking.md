# `buoyant_kernel::row_tracking`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.row_tracking.json`](../model/buoyant_kernel.row_tracking.json)

## RowTrackingDomainMetadata

`struct` · `buoyant_kernel::row_tracking::RowTrackingDomainMetadata`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.row_tracking.RowTrackingDomainMetadata.md)

Also reachable as `delta_kernel::row_tracking::RowTrackingDomainMetadata`

```rust
struct RowTrackingDomainMetadata
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug

**Methods** (1)

```rust
fn get_high_water_mark(snapshot: &Snapshot, engine: &dyn Engine) -> DeltaResult<Option<i64>>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---
