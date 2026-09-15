# `arrow_string::concat_elements`

Crate `arrow-string` · 8 public items · structured records in [`model/arrow_string.concat_elements.json`](../model/arrow_string.concat_elements.json)

## concat_element_binary

`function` · `arrow_string::concat_elements::concat_element_binary`

Also reachable as `arrow::compute::kernels::concat_elements::concat_element_binary`

```rust
fn concat_element_binary<Offset: OffsetSizeTrait>(left: &GenericBinaryArray<Offset>, right: &GenericBinaryArray<Offset>) -> Result<GenericBinaryArray<Offset>, arrow_schema::ArrowError>
```

Returns the elementwise concatenation of a [`GenericBinaryArray`].

---

## concat_elements_binary_view_array

`function` · `arrow_string::concat_elements::concat_elements_binary_view_array`

Also reachable as `arrow::compute::kernels::concat_elements::concat_elements_binary_view_array`

```rust
fn concat_elements_binary_view_array(left: &BinaryViewArray, right: &BinaryViewArray) -> Result<BinaryViewArray, arrow_schema::ArrowError>
```

Concatenates two `BinaryViewArray`s element-wise.
If either element is `Null`, the result element is also `Null`.

# Errors
- Returns an error if the input arrays have different lengths.
- Returns an error if any concatenated value exceeds `u32::MAX` in length.

---

## concat_elements_bytes

`function` · `arrow_string::concat_elements::concat_elements_bytes`

Also reachable as `arrow::compute::kernels::concat_elements::concat_elements_bytes`

```rust
fn concat_elements_bytes<T: ByteArrayType>(left: &GenericByteArray<T>, right: &GenericByteArray<T>) -> Result<GenericByteArray<T>, arrow_schema::ArrowError>
```

Returns the elementwise concatenation of a [`GenericByteArray`].

---

## concat_elements_dyn

`function` · `arrow_string::concat_elements::concat_elements_dyn`

Also reachable as `arrow::compute::kernels::concat_elements::concat_elements_dyn`

```rust
fn concat_elements_dyn(left: &dyn Array, right: &dyn Array) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Returns the elementwise concatenation of [`Array`]s.

The output array will have the same type as the input arrays (which must have the same type).

Concatenation of `FixedSizeBinaryArray` instances with different sizes is supported. The output
type is `FixedSizeBinaryArray` with the sum of the sizes of the two input arrays as size.

# Errors

This function errors if the arrays are of different types.

---

## concat_elements_fixed_size_binary

`function` · `arrow_string::concat_elements::concat_elements_fixed_size_binary`

Also reachable as `arrow::compute::kernels::concat_elements::concat_elements_fixed_size_binary`

```rust
fn concat_elements_fixed_size_binary(left: &FixedSizeBinaryArray, right: &FixedSizeBinaryArray) -> Result<FixedSizeBinaryArray, arrow_schema::ArrowError>
```

Returns the elementwise concatenation of a [`FixedSizeBinaryArray`].

The result has `value_length = left.value_length() + right.value_length()`.
An index is null if either input is null at that position.

An error will be returned if `left` and `right` have different lengths.

---

## concat_elements_string_view_array

`function` · `arrow_string::concat_elements::concat_elements_string_view_array`

Also reachable as `arrow::compute::kernels::concat_elements::concat_elements_string_view_array`

```rust
fn concat_elements_string_view_array(left: &StringViewArray, right: &StringViewArray) -> Result<StringViewArray, arrow_schema::ArrowError>
```

Concatenates two `StringViewArray`s element-wise.
If either element is `Null`, the result element is also `Null`.

# Errors
- Returns an error if the input arrays have different lengths.
- Returns an error if any concatenated value exceeds `u32::MAX` in length.
- Returns an error if concatenated strings do not result in a proper UTF-8 string

---

## concat_elements_utf8

`function` · `arrow_string::concat_elements::concat_elements_utf8`

Also reachable as `arrow::compute::kernels::concat_elements::concat_elements_utf8`

```rust
fn concat_elements_utf8<Offset: OffsetSizeTrait>(left: &GenericStringArray<Offset>, right: &GenericStringArray<Offset>) -> Result<GenericStringArray<Offset>, arrow_schema::ArrowError>
```

Returns the elementwise concatenation of a [`GenericStringArray`].

An index of the resulting [`GenericStringArray`] is null if any of
`StringArray` are null at that location.

```text
e.g:

  ["Hello"] + ["World"] = ["HelloWorld"]

  ["a", "b"] + [None, "c"] = [None, "bc"]
```

An error will be returned if `left` and `right` have different lengths

---

## concat_elements_utf8_many

`function` · `arrow_string::concat_elements::concat_elements_utf8_many`

Also reachable as `arrow::compute::kernels::concat_elements::concat_elements_utf8_many`

```rust
fn concat_elements_utf8_many<Offset: OffsetSizeTrait>(arrays: &[&GenericStringArray<Offset>]) -> Result<GenericStringArray<Offset>, arrow_schema::ArrowError>
```

Returns the elementwise concatenation of [`StringArray`].
```text
e.g:
  ["a", "b"] + [None, "c"] + [None, "d"] = [None, "bcd"]
```

An error will be returned if the [`StringArray`] are of different lengths

---
