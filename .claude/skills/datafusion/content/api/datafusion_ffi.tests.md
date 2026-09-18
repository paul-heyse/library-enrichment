# `datafusion_ffi::tests`

Crate `datafusion-ffi` · 5 public items · structured records in [`model/datafusion_ffi.tests.json`](../model/datafusion_ffi.tests.json)

## create_record_batch

`function` · `datafusion_ffi::tests::create_record_batch`

```rust
fn create_record_batch(start_value: i32, num_values: usize) -> arrow::array::RecordBatch
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.tests.create_record_batch.md).


---

## create_test_schema

`function` · `datafusion_ffi::tests::create_test_schema`

```rust
fn create_test_schema() -> std::sync::Arc<arrow_schema::Schema>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.tests.create_test_schema.md).


---

## datafusion_ffi_get_module

`function` · `datafusion_ffi::tests::datafusion_ffi_get_module`

```rust
extern "C" fn datafusion_ffi_get_module() -> ForeignLibraryModule
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.tests.datafusion_ffi_get_module.md).


This defines the entry point for using the module.

---

## make_test_statistics

`function` · `datafusion_ffi::tests::make_test_statistics`

```rust
fn make_test_statistics() -> datafusion_common::Statistics
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.tests.make_test_statistics.md).


Returns canonical statistics used by both the producer and consumer sides of
the integration tests so round-trips can be asserted without hard-coding
the values in two places.

---

## ForeignLibraryModule

`struct` · `datafusion_ffi::tests::ForeignLibraryModule`

```rust
struct ForeignLibraryModule
```

**Fields**: `create_catalog`, `create_catalog_list`, `create_table`, `create_table_factory`, `create_scalar_udf`, `create_nullary_udf`, `create_timezone_udf`, `create_placement_udf`, `create_table_function`, `create_sum_udaf`, `create_stddev_udaf`, `create_rank_udwf`, `create_extension_options`, `create_empty_exec`, `create_exec_with_expressions`, `create_exec_with_dynamic_expressions`, `create_exec_with_statistics`, `create_table_with_statistics`, `create_physical_optimizer_rule`, `create_context_aware_optimizer_rule`, `create_query_planner`, `version`, `create_first_value_udaf`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.tests.ForeignLibraryModule.md).


This struct defines the module interfaces. It is to be shared by
both the module loading program and library that implements the
module.

---
