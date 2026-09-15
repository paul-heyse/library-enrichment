# `datafusion_common::utils`

Crate `datafusion-common` · 35 public items · structured records in [`model/datafusion_common.utils.json`](../model/datafusion_common.utils.json)

## ListCoercion

`enum` · `datafusion_common::utils::ListCoercion`

```rust
enum ListCoercion
```

**Variants**: `FixedSizedListToList`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

Information about how to coerce lists.

---

## adjust_offsets_for_slice

`function` · `datafusion_common::utils::adjust_offsets_for_slice`

```rust
fn adjust_offsets_for_slice<O: OffsetSizeTrait>(list: &arrow::array::GenericListArray<O>) -> arrow::buffer::OffsetBuffer<O>
```

If `list` is sliced, returns an adjusted offset buffer so that
it points to the sliced portion of the list values, and not the whole list values

---

## arrays_into_list_array

`function` · `datafusion_common::utils::arrays_into_list_array`

```rust
fn arrays_into_list_array(arr: impl IntoIterator<Item = arrow::array::ArrayRef>) -> Result<arrow::array::ListArray>
```

Wrap arrays into a single element `ListArray`.

Example:
```
use arrow::array::{Int32Array, ListArray, ArrayRef};
use arrow::datatypes::{Int32Type, Field};
use std::sync::Arc;

let arr1 = Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef;
let arr2 = Arc::new(Int32Array::from(vec![4, 5, 6])) as ArrayRef;

let list_arr = datafusion_common::utils::arrays_into_list_array([arr1, arr2]).unwrap();

let expected = ListArray::from_iter_primitive::<Int32Type, _, _>(
   vec![
    Some(vec![Some(1), Some(2), Some(3)]),
    Some(vec![Some(4), Some(5), Some(6)]),
   ]
);

assert_eq!(list_arr, expected);
```

---

## base_type

`function` · `datafusion_common::utils::base_type`

```rust
fn base_type(data_type: &arrow::datatypes::DataType) -> arrow::datatypes::DataType
```

Get the base type of a data type.

Example
```
use arrow::datatypes::{DataType, Field};
use datafusion_common::utils::base_type;
use std::sync::Arc;

let data_type =
    DataType::List(Arc::new(Field::new_list_field(DataType::Int32, true)));
assert_eq!(base_type(&data_type), DataType::Int32);

let data_type = DataType::Int32;
assert_eq!(base_type(&data_type), DataType::Int32);
```

---

## bisect

`function` · `datafusion_common::utils::bisect`

```rust
fn bisect<const SIDE: bool>(item_columns: &[arrow::array::ArrayRef], target: &[ScalarValue], sort_options: &[arrow::compute::SortOptions]) -> Result<usize>
```

This function searches for a tuple of given values (`target`) among the given
rows (`item_columns`) using the bisection algorithm. It assumes that `item_columns`
is sorted according to `sort_options` and returns the insertion index of `target`.
Template argument `SIDE` being `true`/`false` means left/right insertion.

---

## coerced_fixed_size_list_to_list

`function` · `datafusion_common::utils::coerced_fixed_size_list_to_list`

```rust
fn coerced_fixed_size_list_to_list(data_type: &arrow::datatypes::DataType) -> arrow::datatypes::DataType
```

Recursively coerce and `FixedSizeList` elements to `List`

---

## coerced_type_with_base_type_only

`function` · `datafusion_common::utils::coerced_type_with_base_type_only`

```rust
fn coerced_type_with_base_type_only(data_type: &arrow::datatypes::DataType, base_type: &arrow::datatypes::DataType, array_coercion: Option<&ListCoercion>) -> arrow::datatypes::DataType
```

A helper function to coerce base type in List.

Example
```
use arrow::datatypes::{DataType, Field};
use datafusion_common::utils::coerced_type_with_base_type_only;
use std::sync::Arc;

let data_type = DataType::List(Arc::new(Field::new_list_field(DataType::Int32, true)));
let base_type = DataType::Float64;
let coerced_type = coerced_type_with_base_type_only(&data_type, &base_type, None);
assert_eq!(coerced_type, DataType::List(Arc::new(Field::new_list_field(DataType::Float64, true))));
```

---

## combine_limit

`function` · `datafusion_common::utils::combine_limit`

```rust
fn combine_limit(parent_skip: usize, parent_fetch: Option<usize>, child_skip: usize, child_fetch: Option<usize>) -> (usize, Option<usize>)
```

Computes the `skip` and `fetch` parameters of a single limit that would be
equivalent to two consecutive limits with the given `skip`/`fetch` parameters.

There are multiple cases to consider:

# Case 0: Parent and child are disjoint (`child_fetch <= skip`).

```text
  Before merging:
                    |........skip........|---fetch-->|     Parent limit
   |...child_skip...|---child_fetch-->|                    Child limit
```

  After merging:
```text
   |.........(child_skip + skip).........|
```

# Case 1: Parent is beyond child's range (`skip < child_fetch <= skip + fetch`).

  Before merging:
```text
                    |...skip...|------------fetch------------>|   Parent limit
   |...child_skip...|-------------child_fetch------------>|       Child limit
```

  After merging:
```text
   |....(child_skip + skip)....|---(child_fetch - skip)-->|
```

 # Case 2: Parent is within child's range (`skip + fetch < child_fetch`).

  Before merging:
```text
                    |...skip...|---fetch-->|                   Parent limit
   |...child_skip...|-------------child_fetch------------>|    Child limit
```

  After merging:
```text
   |....(child_skip + skip)....|---fetch-->|
```

---

## compare_rows

`function` · `datafusion_common::utils::compare_rows`

```rust
fn compare_rows(x: &[ScalarValue], y: &[ScalarValue], sort_options: &[arrow::compute::SortOptions]) -> Result<std::cmp::Ordering>
```

This function compares two tuples depending on the given sort options.

---

## evaluate_partition_ranges

`function` · `datafusion_common::utils::evaluate_partition_ranges`

```rust
fn evaluate_partition_ranges(num_rows: usize, partition_columns: &[arrow::compute::SortColumn]) -> Result<Vec<std::ops::Range<usize>>>
```

Given a list of 0 or more already sorted columns, finds the
partition ranges that would partition equally across columns.

See [`partition`] for more details.

---

## extract_row_at_idx_to_buf

`function` · `datafusion_common::utils::extract_row_at_idx_to_buf`

```rust
fn extract_row_at_idx_to_buf(columns: &[arrow::array::ArrayRef], idx: usize, buf: &mut Vec<ScalarValue>) -> Result<()>
```

Extracts a row at the specified index from a set of columns and stores it in the provided buffer.

---

## find_bisect_point

`function` · `datafusion_common::utils::find_bisect_point`

```rust
fn find_bisect_point<F>(item_columns: &[arrow::array::ArrayRef], target: &[ScalarValue], compare_fn: F, low: usize, high: usize) -> Result<usize> where F: Fn(&[ScalarValue], &[ScalarValue]) -> Result<bool>
```

This function searches for a tuple of given values (`target`) among a slice of
the given rows (`item_columns`) using the bisection algorithm. The slice starts
at the index `low` and ends at the index `high`. The boolean-valued function
`compare_fn` specifies whether we bisect on the left (by returning `false`),
or on the right (by returning `true`) when we compare the target value with
the current value as we iteratively bisect the input.

---

## find_indices

`function` · `datafusion_common::utils::find_indices`

```rust
fn find_indices<T: PartialEq, S: Borrow<T>>(items: &[T], targets: impl IntoIterator<Item = S>) -> Result<Vec<usize>>
```

Find indices of each element in `targets` inside `items`. If one of the
elements is absent in `items`, returns an error.

---

## fixed_size_list_to_arrays

`function` · `datafusion_common::utils::fixed_size_list_to_arrays`

```rust
fn fixed_size_list_to_arrays(a: &arrow::array::ArrayRef) -> Vec<arrow::array::ArrayRef>
```

Helper function to convert a FixedSizeListArray into a vector of ArrayRefs.

---

## get_at_indices

`function` · `datafusion_common::utils::get_at_indices`

```rust
fn get_at_indices<T: Clone, I: Borrow<usize>>(items: &[T], indices: impl IntoIterator<Item = I>) -> Result<Vec<T>>
```

This function "takes" the elements at `indices` from the slice `items`.

---

## get_available_parallelism

`function` · `datafusion_common::utils::get_available_parallelism`

```rust
fn get_available_parallelism() -> usize
```

Returns the estimated number of threads available for parallel execution.

This is a wrapper around `std::thread::available_parallelism`, providing a default value
of `1` if the system's parallelism cannot be determined.

The result is cached after the first call.

---

## get_row_at_idx

`function` · `datafusion_common::utils::get_row_at_idx`

```rust
fn get_row_at_idx(columns: &[arrow::array::ArrayRef], idx: usize) -> Result<Vec<ScalarValue>>
```

Given column vectors, returns row at `idx`.

---

## linear_search

`function` · `datafusion_common::utils::linear_search`

```rust
fn linear_search<const SIDE: bool>(item_columns: &[arrow::array::ArrayRef], target: &[ScalarValue], sort_options: &[arrow::compute::SortOptions]) -> Result<usize>
```

This function searches for a tuple of given values (`target`) among the given
rows (`item_columns`) via a linear scan. It assumes that `item_columns` is sorted
according to `sort_options` and returns the insertion index of `target`.
Template argument `SIDE` being `true`/`false` means left/right insertion.

---

## list_ndims

`function` · `datafusion_common::utils::list_ndims`

```rust
fn list_ndims(data_type: &arrow::datatypes::DataType) -> u64
```

Compute the number of dimensions in a list data type.

---

## list_to_arrays

`function` · `datafusion_common::utils::list_to_arrays`

```rust
fn list_to_arrays<O: OffsetSizeTrait>(a: &arrow::array::ArrayRef) -> Vec<arrow::array::ArrayRef>
```

Helper function to convert a ListArray into a vector of ArrayRefs.

---

## list_values

`function` · `datafusion_common::utils::list_values`

```rust
fn list_values(array: &dyn Array) -> Result<arrow::array::ArrayRef>
```

Returns the inner values of a list, or an error otherwise
For [`ListArray`] and [`LargeListArray`], if it's sliced, it returns a
sliced array too. Therefore, too reconstruct a list using it,
you must adjust the offsets using [`adjust_offsets_for_slice`]

---

## list_values_row_number

`function` · `datafusion_common::utils::list_values_row_number`

```rust
fn list_values_row_number(array: &dyn Array) -> Result<arrow::array::ArrayRef>
```

If `array` is a list or a map, returns a new array of the same length as it's inner values
where each value is the 1-based index of the sublist it's contained. Example:

`[[1], [2, 3], [4, 5, 6]] =>  [1, 2, 2, 3, 3, 3]`

Otherwise returns an error

---

## longest_consecutive_prefix

`function` · `datafusion_common::utils::longest_consecutive_prefix`

```rust
fn longest_consecutive_prefix<T: Borrow<usize>>(sequence: impl IntoIterator<Item = T>) -> usize
```

This function finds the longest prefix of the form 0, 1, 2, ... within the
collection `sequence`. Examples:
- For 0, 1, 2, 4, 5; we would produce 3, meaning 0, 1, 2 is the longest satisfying
  prefix.
- For 1, 2, 3, 4; we would produce 0, meaning there is no such prefix.

---

## merge_and_order_indices

`function` · `datafusion_common::utils::merge_and_order_indices`

```rust
fn merge_and_order_indices<T: Borrow<usize>, S: Borrow<usize>>(first: impl IntoIterator<Item = T>, second: impl IntoIterator<Item = S>) -> Vec<usize>
```

Merges collections `first` and `second`, removes duplicates and sorts the
result, returning it as a [`Vec`].

---

## normalize_float_zero

`function` · `datafusion_common::utils::normalize_float_zero`

```rust
fn normalize_float_zero(array: &arrow::array::ArrayRef) -> arrow::array::ArrayRef
```

Replace `-0.0` with `+0.0` in any `Float16`, `Float32`, or `Float64` array.
For non-float arrays returns the input unchanged. NaN payloads are
preserved.

Arrow's comparison kernels (`arrow::compute::kernels::cmp::eq` etc.) and
row-encoding (`arrow::row::RowConverter`) use IEEE 754 totalOrder
semantics, which treats `-0.0` and `+0.0` as distinct. SQL semantics
(PostgreSQL / IEEE 754 equality) require them to compare equal, so
callers normalize before invoking those kernels.

The common case - no `-0.0` present - is allocation-free: a single
read-only scan of the underlying buffer (auto-vectorizable to an
OR-reduction) decides whether to fall through to the rewriting path.
Only arrays that actually contain `-0.0` pay for a new buffer.

---

## normalize_float_zero_scalar

`function` · `datafusion_common::utils::normalize_float_zero_scalar`

```rust
fn normalize_float_zero_scalar(scalar: ScalarValue) -> ScalarValue
```

Replace `-0.0` with `+0.0` in `Float16`, `Float32`, or `Float64` scalar
values. Other variants are returned unchanged. See [`normalize_float_zero`]
for context.

---

## project_schema

`function` · `datafusion_common::utils::project_schema`

Also reachable as `datafusion::common::project_schema`, `datafusion::physical_plan::project_schema`, `datafusion_common::project_schema`, `datafusion_physical_plan::execution_plan::project_schema`, `datafusion_physical_plan::project_schema`

```rust
fn project_schema(schema: &arrow::datatypes::SchemaRef, projection: Option<&impl AsRef<[usize]>>) -> Result<arrow::datatypes::SchemaRef>
```

Applies an optional projection to a [`SchemaRef`], returning the
projected schema

Example:
```
use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use datafusion_common::project_schema;

// Schema with columns 'a', 'b', and 'c'
let schema = SchemaRef::new(Schema::new(vec![
    Field::new("a", DataType::Int32, true),
    Field::new("b", DataType::Int64, true),
    Field::new("c", DataType::Utf8, true),
]));

// Pick columns 'c' and 'b'
let projection = Some(vec![2, 1]);
let projected_schema = project_schema(&schema, projection.as_ref()).unwrap();

let expected_schema = SchemaRef::new(Schema::new(vec![
    Field::new("c", DataType::Utf8, true),
    Field::new("b", DataType::Int64, true),
]));

assert_eq!(projected_schema, expected_schema);
```

---

## quote_identifier

`function` · `datafusion_common::utils::quote_identifier`

```rust
fn quote_identifier(s: &str) -> std::borrow::Cow<'_, str>
```

Wraps identifier string in double quotes, escaping any double quotes in
the identifier by replacing it with two double quotes

e.g. identifier `tab.le"name` becomes `"tab.le""name"`

---

## remove_list_null_values

`function` · `datafusion_common::utils::remove_list_null_values`

```rust
fn remove_list_null_values(array: &arrow::array::ArrayRef) -> Result<arrow::array::ArrayRef>
```

For lists and large lists, truncates the sublist of null values
Otherwise returns an error

---

## search_in_slice

`function` · `datafusion_common::utils::search_in_slice`

```rust
fn search_in_slice<F>(item_columns: &[arrow::array::ArrayRef], target: &[ScalarValue], compare_fn: F, low: usize, high: usize) -> Result<usize> where F: Fn(&[ScalarValue], &[ScalarValue]) -> Result<bool>
```

This function searches for a tuple of given values (`target`) among a slice of
the given rows (`item_columns`) via a linear scan. The slice starts at the index
`low` and ends at the index `high`. The boolean-valued function `compare_fn`
specifies the stopping criterion.

---

## set_difference

`function` · `datafusion_common::utils::set_difference`

```rust
fn set_difference<T: Borrow<usize>, S: Borrow<usize>>(first: impl IntoIterator<Item = T>, second: impl IntoIterator<Item = S>) -> Vec<usize>
```

Calculates the set difference between sequences `first` and `second`,
returning the result as a [`Vec`]. Preserves the ordering of `first`.

---

## split_vec_min_alloc

`function` · `datafusion_common::utils::split_vec_min_alloc`

```rust
fn split_vec_min_alloc<T>(vec: &mut Vec<T>, n: usize) -> Vec<T>
```

Splits `vec` at index `n`, returning the first `n` elements and leaving the
remaining `vec.len() - n` elements in `vec`.

Allocates for whichever side is smaller, so the new allocation is
`min(n, vec.len() - n)` rather than always `n` (as `vec.drain(0..n).collect()`
would). This matters when the split emits a prefix under memory pressure,
where `n` can be close to `vec.len()`.

---

## take_function_args

`function` · `datafusion_common::utils::take_function_args`

```rust
fn take_function_args<const N: usize, T>(function_name: &str, args: impl IntoIterator<Item = T>) -> Result<[T; N]>
```

Converts a collection of function arguments into a fixed-size array of length N
producing a reasonable error message in case of unexpected number of arguments.

# Example
```
# use datafusion_common::Result;
# use datafusion_common::utils::take_function_args;
# use datafusion_common::ScalarValue;
fn my_function(args: &[ScalarValue]) -> Result<()> {
    // function expects 2 args, so create a 2-element array
    let [arg1, arg2] = take_function_args("my_function", args)?;
    // ... do stuff..
    Ok(())
}

// Calling the function with 1 argument produces an error:
let args = vec![ScalarValue::Int32(Some(10))];
let err = my_function(&args).unwrap_err();
assert_eq!(
    err.to_string(),
    "Execution error: my_function function requires 2 arguments, got 1"
);
// Calling the function with 2 arguments works great
let args = vec![ScalarValue::Int32(Some(10)), ScalarValue::Int32(Some(20))];
my_function(&args).unwrap();
```

---

## transpose

`function` · `datafusion_common::utils::transpose`

```rust
fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>>
```

Transposes the given vector of vectors.

---

## SingleRowListArrayBuilder

`struct` · `datafusion_common::utils::SingleRowListArrayBuilder`

```rust
struct SingleRowListArrayBuilder
```

**Derives**: Clone, Debug

**Methods** (14)

```rust
fn build_fixed_size_list_array(self, list_size: usize) -> FixedSizeListArray
fn build_fixed_size_list_scalar(self, list_size: usize) -> ScalarValue
fn build_large_list_array(self) -> LargeListArray
fn build_large_list_scalar(self) -> ScalarValue
fn build_large_list_view_array(self) -> LargeListViewArray
fn build_large_list_view_scalar(self) -> ScalarValue
fn build_list_array(self) -> ListArray
fn build_list_scalar(self) -> ScalarValue
fn build_list_view_array(self) -> ListViewArray
fn build_list_view_scalar(self) -> ScalarValue
fn new(arr: ArrayRef) -> Self
fn with_field(self, field: &Field) -> Self
fn with_field_name(self, field_name: Option<String>) -> Self
fn with_nullable(self, nullable: bool) -> Self
```

Creates single element [`ListArray`], [`LargeListArray`] and
[`FixedSizeListArray`] from other arrays

For example this builder can convert `[1, 2, 3]` into `[[1, 2, 3]]`

# Example
```
# use std::sync::Arc;
# use arrow::array::{Array, ListArray};
# use arrow::array::types::Int64Type;
# use datafusion_common::utils::SingleRowListArrayBuilder;
// Array is [1, 2, 3]
let arr = ListArray::from_iter_primitive::<Int64Type, _, _>(vec![Some(vec![
    Some(1),
    Some(2),
    Some(3),
])]);
// Wrap as a list array: [[1, 2, 3]]
let list_arr = SingleRowListArrayBuilder::new(Arc::new(arr)).build_list_array();
assert_eq!(list_arr.len(), 1);
```

---
