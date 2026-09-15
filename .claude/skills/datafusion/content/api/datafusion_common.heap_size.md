# `datafusion_common::heap_size`

Crate `datafusion-common` · 2 public items · structured records in [`model/datafusion_common.heap_size.json`](../model/datafusion_common.heap_size.json)

## DFHeapSizeCtx

`struct` · `datafusion_common::heap_size::DFHeapSizeCtx`

```rust
struct DFHeapSizeCtx
```

**Derives**: Default

---

## DFHeapSize

`trait` · `datafusion_common::heap_size::DFHeapSize`

```rust
trait DFHeapSize
```

**Implementors** (33)

- `alloc::boxed::Box`
- `alloc::string::String`
- `alloc::sync::Arc`
- `alloc::vec::Vec`
- `arrow_array::array::fixed_size_list_array::FixedSizeListArray`
- `arrow_array::array::list_array::LargeListArray`
- `arrow_array::array::list_array::ListArray`
- `arrow_array::array::list_view_array::LargeListViewArray`
- `arrow_array::array::list_view_array::ListViewArray`
- `arrow_array::array::map_array::MapArray`
- `arrow_array::array::struct_array::StructArray`
- `arrow_buffer::bigint::i256`
- `arrow_buffer::interval::IntervalDayTime`
- `arrow_buffer::interval::IntervalMonthDayNano`
- `arrow_schema::datatype::DataType`
- `arrow_schema::datatype::IntervalUnit`
- `arrow_schema::datatype::TimeUnit`
- `arrow_schema::datatype::UnionMode`
- `arrow_schema::field::Field`
- `arrow_schema::fields::Fields`
- `arrow_schema::fields::UnionFields`
- `chrono::datetime::DateTime`
- `core::option::Option`
- `datafusion_common::scalar::ScalarValue`
- `datafusion_common::stats::ColumnStatistics`
- `datafusion_common::stats::Precision`
- `datafusion_common::stats::Statistics`
- `datafusion_common::table_reference::TableReference`
- `datafusion_execution::cache::SchemaFingerprint`
- `datafusion_execution::cache::TableScopedPath`
- `datafusion_execution::cache::cache_manager::CachedFileMetadata`
- `half::binary16::f16`
- `std::collections::hash::map::HashMap`

**Methods** (1)

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

Trait for computing how many bytes a value has allocated on the heap.

Implementations need to use [`DFHeapSizeCtx`] that is pushed through every
nested call. The context records which allocations have already been measured
so they are only counted once.

---
