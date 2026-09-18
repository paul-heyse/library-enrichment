# `arrow_array::array::string_array`

Crate `arrow-array` · 3 public items · structured records in [`model/arrow_array.array.string_array.json`](../model/arrow_array.array.string_array.json)

## GenericStringArray

`type_alias` · `arrow_array::array::string_array::GenericStringArray`

```rust
type GenericStringArray<OffsetSize> = GenericByteArray<types::GenericStringType<OffsetSize>>
```

**Implements**: `core::convert::From`

**via `core::convert::From`**

```rust
fn from(v: GenericBinaryArray<OffsetSize>) -> Self
fn from(v: GenericListArray<OffsetSize>) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_array.array.string_array.GenericStringArray.md).


A [`GenericByteArray`] for storing `str`

---

## LargeStringArray

`type_alias` · `arrow_array::array::string_array::LargeStringArray`

```rust
type LargeStringArray = GenericStringArray<i64>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.array.string_array.LargeStringArray.md).


A [`GenericStringArray`] of `str` using `i64` offsets

# Examples

Construction

```
# use arrow_array::LargeStringArray;
// Create from Vec<Option<&str>>
let arr = LargeStringArray::from(vec![Some("foo"), Some("bar"), None, Some("baz")]);
// Create from Vec<&str>
let arr = LargeStringArray::from(vec!["foo", "bar", "baz"]);
// Create from iter/collect (requires Option<&str>)
let arr: LargeStringArray = std::iter::repeat(Some("foo")).take(10).collect();
```

Construction and Access

```
use arrow_array::LargeStringArray;
let array = LargeStringArray::from(vec![Some("foo"), None, Some("bar")]);
assert_eq!(array.value(2), "bar");
```

See [`GenericByteArray`] for more information and examples

---

## StringArray

`type_alias` · `arrow_array::array::string_array::StringArray`

```rust
type StringArray = GenericStringArray<i32>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.array.string_array.StringArray.md).


A [`GenericStringArray`] of `str` using `i32` offsets

# Examples

Construction

```
# use arrow_array::StringArray;
// Create from Vec<Option<&str>>
let arr = StringArray::from(vec![Some("foo"), Some("bar"), None, Some("baz")]);
// Create from Vec<&str>
let arr = StringArray::from(vec!["foo", "bar", "baz"]);
// Create from iter/collect (requires Option<&str>)
let arr: StringArray = std::iter::repeat(Some("foo")).take(10).collect();
```

Construction and Access

```
# use arrow_array::StringArray;
let array = StringArray::from(vec![Some("foo"), None, Some("bar")]);
assert_eq!(array.value(0), "foo");
```

See [`GenericByteArray`] for more information and examples

---
