# `arrow_array::builder::generic_byte_run_builder`

Crate `arrow-array` · 5 public items · structured records in [`model/arrow_array.builder.generic_byte_run_builder.json`](../model/arrow_array.builder.generic_byte_run_builder.json)

## GenericByteRunBuilder

`struct` · `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder`

```rust
struct GenericByteRunBuilder<R, V> where R: ArrowPrimitiveType, V: ByteArrayType
```

**Implements**: `arrow_array::builder::ArrayBuilder`, `core::iter::traits::collect::Extend`

**Derives**: Debug, Default

**Methods** (7)

```rust
fn append_null(&mut self)
fn append_option(&mut self, input_value: Option<impl AsRef<V::Native>>)
fn append_value(&mut self, input_value: impl AsRef<V::Native>)
fn finish(&mut self) -> RunArray<R>
fn finish_cloned(&self) -> RunArray<R>
fn new() -> Self
fn with_capacity(capacity: usize, data_capacity: usize) -> Self
```

**via `arrow_array::builder::ArrayBuilder`**

```rust
fn as_any(&self) -> &dyn Any
fn as_any_mut(&mut self) -> &mut dyn Any
fn finish(&mut self) -> ArrayRef
fn finish_cloned(&self) -> ArrayRef
fn into_box_any(Box<self>) -> Box<dyn Any>
fn len(&self) -> usize
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: IntoIterator<Item = Option<S>>>(&mut self, iter: T)
```

Builder for [`RunArray`] of [`GenericByteArray`](crate::array::GenericByteArray)

# Example:

```

# use arrow_array::builder::GenericByteRunBuilder;
# use arrow_array::{GenericByteArray, BinaryArray};
# use arrow_array::types::{BinaryType, Int16Type};
# use arrow_array::{Array, Int16Array};
# use arrow_array::cast::AsArray;

let mut builder =
GenericByteRunBuilder::<Int16Type, BinaryType>::new();
builder.extend([Some(b"abc"), Some(b"abc"), None, Some(b"def")].into_iter());
builder.append_value(b"def");
builder.append_null();
let array = builder.finish();

assert_eq!(array.run_ends().values(), &[2, 3, 5, 6]);

let av = array.values();

assert!(!av.is_null(0));
assert!(av.is_null(1));
assert!(!av.is_null(2));
assert!(av.is_null(3));

// Values are polymorphic and so require a downcast.
let ava: &BinaryArray = av.as_binary();

assert_eq!(ava.value(0), b"abc");
assert_eq!(ava.value(2), b"def");
```

---

## BinaryRunBuilder

`type_alias` · `arrow_array::builder::generic_byte_run_builder::BinaryRunBuilder`

```rust
type BinaryRunBuilder<K> = GenericByteRunBuilder<K, types::BinaryType>
```

Builder for [`RunArray`] of [`BinaryArray`](crate::array::BinaryArray)

```
// Create a run-end encoded array with run-end indexes data type as `i16`.
// The encoded data is binary values.

# use arrow_array::builder::BinaryRunBuilder;
# use arrow_array::{BinaryArray, Int16Array};
# use arrow_array::cast::AsArray;
# use arrow_array::types::Int16Type;

let mut builder = BinaryRunBuilder::<Int16Type>::new();

// The builder builds the dictionary value by value
builder.append_value(b"abc");
builder.append_null();
builder.extend([Some(b"def"), Some(b"def"), Some(b"abc")]);
let array = builder.finish();

assert_eq!(array.run_ends().values(), &[1, 2, 4, 5]);

// Values are polymorphic and so require a downcast.
let av = array.values();
let ava: &BinaryArray = av.as_binary();

assert_eq!(ava.value(0), b"abc");
assert!(av.is_null(1));
assert_eq!(ava.value(2), b"def");
assert_eq!(ava.value(3), b"abc");

```

---

## LargeBinaryRunBuilder

`type_alias` · `arrow_array::builder::generic_byte_run_builder::LargeBinaryRunBuilder`

```rust
type LargeBinaryRunBuilder<K> = GenericByteRunBuilder<K, types::LargeBinaryType>
```

Builder for [`RunArray`] of [`LargeBinaryArray`](crate::array::LargeBinaryArray)

---

## LargeStringRunBuilder

`type_alias` · `arrow_array::builder::generic_byte_run_builder::LargeStringRunBuilder`

```rust
type LargeStringRunBuilder<K> = GenericByteRunBuilder<K, types::LargeUtf8Type>
```

Builder for [`RunArray`] of [`LargeStringArray`](crate::array::LargeStringArray)

---

## StringRunBuilder

`type_alias` · `arrow_array::builder::generic_byte_run_builder::StringRunBuilder`

```rust
type StringRunBuilder<K> = GenericByteRunBuilder<K, types::Utf8Type>
```

Builder for [`RunArray`] of [`StringArray`](crate::array::StringArray)

```
// Create a run-end encoded array with run-end indexes data type as `i16`.
// The encoded values are Strings.

# use arrow_array::builder::StringRunBuilder;
# use arrow_array::{Int16Array, StringArray};
# use arrow_array::types::Int16Type;
# use arrow_array::cast::AsArray;
#
let mut builder = StringRunBuilder::<Int16Type>::new();

// The builder builds the dictionary value by value
builder.append_value("abc");
builder.append_null();
builder.extend([Some("def"), Some("def"), Some("abc")]);
let array = builder.finish();

assert_eq!(array.run_ends().values(), &[1, 2, 4, 5]);

// Values are polymorphic and so require a downcast.
let av = array.values();
let ava: &StringArray = av.as_string::<i32>();

assert_eq!(ava.value(0), "abc");
assert!(av.is_null(1));
assert_eq!(ava.value(2), "def");
assert_eq!(ava.value(3), "abc");

```

---
