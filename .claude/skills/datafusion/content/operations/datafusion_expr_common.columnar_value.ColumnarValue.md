# `datafusion_expr_common::columnar_value::ColumnarValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.columnar_value.ColumnarValue.json).

<a id="op-d5fec5b0e8fcb4446aea8c7f"></a>
## ColumnarValue

`enum` · `datafusion_expr_common::columnar_value::ColumnarValue` · datafusion-expr-common 55.1.0

```rust
enum ColumnarValue
```

Source: `src/columnar_value.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

The result of evaluating an expression.

[`ColumnarValue::Scalar`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-d6c26dcb617515197f47859d) represents a single value repeated any number of
times. This is an important performance optimization for handling values
that do not change across rows.

[`ColumnarValue::Array`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-dde33518db6b4654b48e85d4) represents a column of data, stored as an  Arrow
[`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1)

A slice of `ColumnarValue`s logically represents a table, with each column
having the same number of rows. This means that all `Array`s are the same
length.

# Example

A `ColumnarValue::Array` with an array of 5 elements and a
`ColumnarValue::Scalar` with the value 100

```text
┌──────────────┐
│ ┌──────────┐ │
│ │   "A"    │ │
│ ├──────────┤ │
│ │   "B"    │ │
│ ├──────────┤ │
│ │   "C"    │ │
│ ├──────────┤ │
│ │   "D"    │ │        ┌──────────────┐
│ ├──────────┤ │        │ ┌──────────┐ │
│ │   "E"    │ │        │ │   100    │ │
│ └──────────┘ │        │ └──────────┘ │
└──────────────┘        └──────────────┘

 ColumnarValue::        ColumnarValue::
      Array                 Scalar
```

Logically represents the following table:

| Column 1| Column 2 |
| ------- | -------- |
| A | 100 |
| B | 100 |
| C | 100 |
| D | 100 |
| E | 100 |

# Performance Notes

When implementing functions or operators, it is important to consider the
performance implications of handling scalar values.

Because all functions must handle [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1), it is
convenient to convert [`ColumnarValue::Scalar`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-d6c26dcb617515197f47859d)s using
[`Self::into_array`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-6e86167b7e5d4ece961c8a83). For example,  [`ColumnarValue::values_to_arrays`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-ec4c901c17836f47ccf68670)
converts multiple columnar values into arrays of the same length.

However, it is often much more performant to provide a different,
implementation that handles scalar values differently

<a id="op-dde33518db6b4654b48e85d4"></a>
## Array

`variant` · `datafusion_expr_common::columnar_value::ColumnarValue::Array` · datafusion-expr-common 55.1.0

```rust
Array
```

Source: `src/columnar_value.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Array of values

<a id="op-d6c26dcb617515197f47859d"></a>
## Scalar

`variant` · `datafusion_expr_common::columnar_value::ColumnarValue::Scalar` · datafusion-expr-common 55.1.0

```rust
Scalar
```

Source: `src/columnar_value.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A single value

<a id="op-45343bf5101f2e9d49d814dc"></a>
## cast_to

`function` · `datafusion_expr_common::columnar_value::ColumnarValue::cast_to` · datafusion-expr-common 55.1.0

```rust
fn cast_to(&self, cast_type: &DataType, cast_options: Option<&CastOptions<'static>>) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [310, 2], "filename": "src/columnar_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/columnar_value.rs:294`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Cast this [ColumnarValue](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-d5fec5b0e8fcb4446aea8c7f) to the specified `DataType`

# Struct Casting Behavior

When casting struct types, fields are matched **by name** rather than position:
- Source fields are matched to target fields using case-sensitive name comparison
- Fields are reordered to match the target schema
- Missing target fields are filled with null arrays
- Extra source fields are ignored

For non-struct types, uses Arrow's standard positional casting.

<a id="op-786fca7e590a1f884b521f47"></a>
## clone

`function` · `datafusion_expr_common::columnar_value::ColumnarValue::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> ColumnarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 10], "end": [101, 15], "filename": "src/columnar_value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/columnar_value.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18884955685528c27eaaae19"></a>
## create_null_array

`function` · `datafusion_expr_common::columnar_value::ColumnarValue::create_null_array` · datafusion-expr-common 55.1.0

```rust
fn create_null_array(num_rows: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [310, 2], "filename": "src/columnar_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/columnar_value.rs:231`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Null columnar values are implemented as a null array in order to pass batch
num_rows

<a id="op-f0488ef234358866deb9e4c5"></a>
## data_type

`function` · `datafusion_expr_common::columnar_value::ColumnarValue::data_type` · datafusion-expr-common 55.1.0

```rust
fn data_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [310, 2], "filename": "src/columnar_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/columnar_value.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35425f8454650a5c3d8bd099"></a>
## fmt

`function` · `datafusion_expr_common::columnar_value::ColumnarValue::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 17], "end": [101, 22], "filename": "src/columnar_value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/columnar_value.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b4e0abb150c541863a9af8a"></a>
## fmt

`function` · `datafusion_expr_common::columnar_value::ColumnarValue::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [443, 1], "end": [464, 2], "filename": "src/columnar_value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/columnar_value.rs:444`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfc38f4a160a12ff037b04e3"></a>
## from

`function` · `datafusion_expr_common::columnar_value::ColumnarValue::from` · datafusion-expr-common 55.1.0

```rust
fn from(value: ScalarValue) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [119, 2], "filename": "src/columnar_value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/columnar_value.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c575347bc8643b3d317f7a58"></a>
## from

`function` · `datafusion_expr_common::columnar_value::ColumnarValue::from` · datafusion-expr-common 55.1.0

```rust
fn from(value: ArrayRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [113, 2], "filename": "src/columnar_value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/columnar_value.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e86167b7e5d4ece961c8a83"></a>
## into_array

`function` · `datafusion_expr_common::columnar_value::ColumnarValue::into_array` · datafusion-expr-common 55.1.0

```rust
fn into_array(self, num_rows: usize) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [310, 2], "filename": "src/columnar_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/columnar_value.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Convert any [`Self::Scalar`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-d6c26dcb617515197f47859d) into an Arrow [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) with the specified
number of rows  by repeating the same scalar multiple times,
which is not as efficient as handling the scalar directly.
[`Self::Array`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-dde33518db6b4654b48e85d4) will just be returned as is.

See [`Self::into_array_of_size`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-40e7599a5316a4a3fa567f4a) if you need to validate the length of the output array.

See [`Self::values_to_arrays`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-ec4c901c17836f47ccf68670) to convert multiple columnar values into
arrays of the same length.

# Errors

Errors if `self` is a Scalar that fails to be converted into an array of size

<a id="op-40e7599a5316a4a3fa567f4a"></a>
## into_array_of_size

`function` · `datafusion_expr_common::columnar_value::ColumnarValue::into_array_of_size` · datafusion-expr-common 55.1.0

```rust
fn into_array_of_size(self, num_rows: usize) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [310, 2], "filename": "src/columnar_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/columnar_value.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Convert a columnar value into an Arrow [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) with the specified
number of rows. [`Self::Scalar`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-d6c26dcb617515197f47859d) is converted by repeating the same
scalar multiple times which is not as efficient as handling the scalar
directly.
This validates that if this is [`Self::Array`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-dde33518db6b4654b48e85d4), it has the expected length.

See [`Self::values_to_arrays`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-ec4c901c17836f47ccf68670) to convert multiple columnar values into
arrays of the same length.

# Errors

Errors if `self` is a Scalar that fails to be converted into an array of size or
if the array length does not match the expected length

<a id="op-c524cc0bd43fc1fc943c2310"></a>
## to_array

`function` · `datafusion_expr_common::columnar_value::ColumnarValue::to_array` · datafusion-expr-common 55.1.0

```rust
fn to_array(&self, num_rows: usize) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [310, 2], "filename": "src/columnar_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/columnar_value.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Convert any [`Self::Scalar`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-d6c26dcb617515197f47859d) into an Arrow [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) with the specified
number of rows  by repeating the same scalar multiple times,
which is not as efficient as handling the scalar directly.
[`Self::Array`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-dde33518db6b4654b48e85d4) will just be returned as is.

See [`Self::to_array_of_size`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-27ce41647f6fa6ec7299abae) if you need to validate the length of the output array.

See [`Self::values_to_arrays`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-ec4c901c17836f47ccf68670) to convert multiple columnar values into
arrays of the same length.

# Errors

Errors if `self` is a Scalar that fails to be converted into an array of size

<a id="op-27ce41647f6fa6ec7299abae"></a>
## to_array_of_size

`function` · `datafusion_expr_common::columnar_value::ColumnarValue::to_array_of_size` · datafusion-expr-common 55.1.0

```rust
fn to_array_of_size(&self, num_rows: usize) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [310, 2], "filename": "src/columnar_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/columnar_value.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Convert a columnar value into an Arrow [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) with the specified
number of rows. [`Self::Scalar`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-d6c26dcb617515197f47859d) is converted by repeating the same
scalar multiple times which is not as efficient as handling the scalar
directly.
This validates that if this is [`Self::Array`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-dde33518db6b4654b48e85d4), it has the expected length.

See [`Self::values_to_arrays`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-ec4c901c17836f47ccf68670) to convert multiple columnar values into
arrays of the same length.

# Errors

Errors if `self` is a Scalar that fails to be converted into an array of size or
if the array length does not match the expected length

<a id="op-ec4c901c17836f47ccf68670"></a>
## values_to_arrays

`function` · `datafusion_expr_common::columnar_value::ColumnarValue::values_to_arrays` · datafusion-expr-common 55.1.0

```rust
fn values_to_arrays(args: &[ColumnarValue]) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [310, 2], "filename": "src/columnar_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/columnar_value.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Converts  [`ColumnarValue`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-d5fec5b0e8fcb4446aea8c7f)s to [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1)s with the same length.

# Performance Note

This function expands any [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) to an array. This expansion
permits using a single function in terms of arrays, but it can be
inefficient compared to handling the scalar value directly.

Thus, it is recommended to provide specialized implementations for
scalar values if performance is a concern.

# Errors

If there are multiple array arguments that have different lengths
