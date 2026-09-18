# `datafusion_functions_table::generate_series::SeriesValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_table.generate_series.SeriesValue.json).

<a id="op-fb1d42b88aad21d1b65ca4f4"></a>
## SeriesValue

`trait` · `datafusion_functions_table::generate_series::SeriesValue` · datafusion-functions-table 55.1.0

```rust
trait SeriesValue: fmt::Debug + Clone + Send + Sync + 'static
```

Source: `src/generate_series.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

Trait for values that can be generated in a series

<a id="op-5dc4d9703ec652825e19191e"></a>
## StepType

`assoc_type` · `datafusion_functions_table::generate_series::SeriesValue::StepType` · datafusion-functions-table 55.1.0

```rust
StepType
```

Source: `src/generate_series.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0c11edf01990d0e551f2cc4"></a>
## ValueType

`assoc_type` · `datafusion_functions_table::generate_series::SeriesValue::ValueType` · datafusion-functions-table 55.1.0

```rust
ValueType
```

Source: `src/generate_series.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fc2c48a372fac62efa61b51"></a>
## advance

`function` · `datafusion_functions_table::generate_series::SeriesValue::advance` · datafusion-functions-table 55.1.0

```rust
fn advance(&mut self, step: &Self::StepType) -> Result<()>
```

Source: `src/generate_series.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

Advance to the next value in the series.

<a id="op-940635f2e6aa3eb3c28e3bc8"></a>
## advance_with_end

`function` · `datafusion_functions_table::generate_series::SeriesValue::advance_with_end` · datafusion-functions-table 55.1.0

```rust
fn advance_with_end(&mut self, _end: &mut Self, step: &Self::StepType) -> Result<()>
```

Source: `src/generate_series.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

Advance to the next value, adjusting the end of the series if needed.

The default implementation preserves the behavior of [`Self::advance`](../operations/datafusion_functions_table.generate_series.SeriesValue.md#op-3fc2c48a372fac62efa61b51).
Implementations can override this method when they need to handle an
overflow by terminating the series after the current value.

<a id="op-02faffb86334f2d732e4153c"></a>
## create_array

`function` · `datafusion_functions_table::generate_series::SeriesValue::create_array` · datafusion-functions-table 55.1.0

```rust
fn create_array(&self, values: Vec<Self::ValueType>) -> Result<ArrayRef>
```

Source: `src/generate_series.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

Create an Arrow array from a vector of values

<a id="op-c7c06dabc95cc820943c2bb5"></a>
## display_value

`function` · `datafusion_functions_table::generate_series::SeriesValue::display_value` · datafusion-functions-table 55.1.0

```rust
fn display_value(&self) -> String
```

Source: `src/generate_series.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

Display the value for debugging

<a id="op-658b0bb2a12d89d199a5b708"></a>
## should_stop

`function` · `datafusion_functions_table::generate_series::SeriesValue::should_stop` · datafusion-functions-table 55.1.0

```rust
fn should_stop(&self, end: Self, step: &Self::StepType, include_end: bool) -> bool
```

Source: `src/generate_series.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

Check if we've reached the end of the series

<a id="op-c0c93f9ce096182cc906f85d"></a>
## to_value_type

`function` · `datafusion_functions_table::generate_series::SeriesValue::to_value_type` · datafusion-functions-table 55.1.0

```rust
fn to_value_type(&self) -> Self::ValueType
```

Source: `src/generate_series.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

Convert self to ValueType for array creation
