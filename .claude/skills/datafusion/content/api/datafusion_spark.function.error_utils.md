# `datafusion_spark::function::error_utils`

Crate `datafusion-spark` · 5 public items · structured records in [`model/datafusion_spark.function.error_utils.json`](../model/datafusion_spark.function.error_utils.json)

## generic_exec_err

`function` · `datafusion_spark::function::error_utils::generic_exec_err`

```rust
fn generic_exec_err(function_name: &str, message: &str) -> datafusion_common::DataFusionError
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.error_utils.generic_exec_err.md).


---

## generic_internal_err

`function` · `datafusion_spark::function::error_utils::generic_internal_err`

```rust
fn generic_internal_err(function_name: &str, message: &str) -> datafusion_common::DataFusionError
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.error_utils.generic_internal_err.md).


---

## invalid_arg_count_exec_err

`function` · `datafusion_spark::function::error_utils::invalid_arg_count_exec_err`

```rust
fn invalid_arg_count_exec_err(function_name: &str, required_range: (i32, i32), provided: usize) -> datafusion_common::DataFusionError
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.error_utils.invalid_arg_count_exec_err.md).


---

## unsupported_data_type_exec_err

`function` · `datafusion_spark::function::error_utils::unsupported_data_type_exec_err`

```rust
fn unsupported_data_type_exec_err(function_name: &str, required: &str, provided: &arrow::datatypes::DataType) -> datafusion_common::DataFusionError
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.error_utils.unsupported_data_type_exec_err.md).


---

## unsupported_data_types_exec_err

`function` · `datafusion_spark::function::error_utils::unsupported_data_types_exec_err`

```rust
fn unsupported_data_types_exec_err(function_name: &str, required: &str, provided: &[arrow::datatypes::DataType]) -> datafusion_common::DataFusionError
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.error_utils.unsupported_data_types_exec_err.md).


---
