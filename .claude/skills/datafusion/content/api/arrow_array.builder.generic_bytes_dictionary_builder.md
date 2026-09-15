# `arrow_array::builder::generic_bytes_dictionary_builder`

Crate `arrow-array` · 5 public items · structured records in [`model/arrow_array.builder.generic_bytes_dictionary_builder.json`](../model/arrow_array.builder.generic_bytes_dictionary_builder.json)

## GenericByteDictionaryBuilder

`struct` · `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder`

```rust
struct GenericByteDictionaryBuilder<K, T> where K: ArrowDictionaryKeyType, T: ByteArrayType
```

**Implements**: `arrow_array::builder::ArrayBuilder`, `core::iter::traits::collect::Extend`

**Derives**: Debug, Default

**Methods** (17)

```rust
fn append(&mut self, value: impl AsRef<T::Native>) -> Result<K::Native, ArrowError>
fn append_n(&mut self, value: impl AsRef<T::Native>, count: usize) -> Result<K::Native, ArrowError>
fn append_null(&mut self)
fn append_nulls(&mut self, n: usize)
fn append_option(&mut self, value: Option<impl AsRef<T::Native>>)
fn append_options(&mut self, value: Option<impl AsRef<T::Native>>, count: usize)
fn append_value(&mut self, value: impl AsRef<T::Native>)
fn append_values(&mut self, value: impl AsRef<T::Native>, count: usize)
fn extend_dictionary(&mut self, dictionary: &TypedDictionaryArray<'_, K, GenericByteArray<T>>) -> Result<(), ArrowError>
fn finish(&mut self) -> DictionaryArray<K>
fn finish_cloned(&self) -> DictionaryArray<K>
fn finish_preserve_values(&mut self) -> DictionaryArray<K>
fn new() -> Self
fn new_with_dictionary(keys_capacity: usize, dictionary_values: &GenericByteArray<T>) -> Result<Self, ArrowError>
fn try_new_from_builder<K2>(source: GenericByteDictionaryBuilder<K2, T>) -> Result<Self, ArrowError> where K::Native: NumCast, K2: ArrowDictionaryKeyType, K2::Native: NumCast
fn validity_slice(&self) -> Option<&[u8]>
fn with_capacity(keys_capacity: usize, value_capacity: usize, data_capacity: usize) -> Self
```

**via `arrow_array::builder::ArrayBuilder`**

```rust
fn as_any(&self) -> &dyn Any
fn as_any_mut(&mut self) -> &mut dyn Any
fn finish(&mut self) -> ArrayRef
fn finish_cloned(&self) -> ArrayRef
fn finish_preserve_values(&mut self) -> ArrayRef
fn into_box_any(Box<self>) -> Box<dyn Any>
fn len(&self) -> usize
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<I: IntoIterator<Item = Option<V>>>(&mut self, iter: I)
```

Builder for [`DictionaryArray`] of [`GenericByteArray`]

For example to map a set of byte indices to String values. Note that
the use of a `HashMap` here will not scale to very large arrays or
result in an ordered dictionary.

---

## BinaryDictionaryBuilder

`type_alias` · `arrow_array::builder::generic_bytes_dictionary_builder::BinaryDictionaryBuilder`

```rust
type BinaryDictionaryBuilder<K> = GenericByteDictionaryBuilder<K, types::GenericBinaryType<i32>>
```

Builder for [`DictionaryArray`] of [`BinaryArray`](crate::array::BinaryArray)

```
// Create a dictionary array indexed by bytes whose values are binary.
// It can thus hold up to 256 distinct binary values.

# use arrow_array::builder::BinaryDictionaryBuilder;
# use arrow_array::{BinaryArray, Int8Array};
# use arrow_array::types::Int8Type;

let mut builder = BinaryDictionaryBuilder::<Int8Type>::new();

// The builder builds the dictionary value by value
builder.append(b"abc").unwrap();
builder.append_null();
builder.append(b"def").unwrap();
builder.append(b"def").unwrap();
builder.append(b"abc").unwrap();
let array = builder.finish();

assert_eq!(
  array.keys(),
  &Int8Array::from(vec![Some(0), None, Some(1), Some(1), Some(0)])
);

// Values are polymorphic and so require a downcast.
let av = array.values();
let ava: &BinaryArray = av.as_any().downcast_ref::<BinaryArray>().unwrap();

assert_eq!(ava.value(0), b"abc");
assert_eq!(ava.value(1), b"def");

```

---

## LargeBinaryDictionaryBuilder

`type_alias` · `arrow_array::builder::generic_bytes_dictionary_builder::LargeBinaryDictionaryBuilder`

```rust
type LargeBinaryDictionaryBuilder<K> = GenericByteDictionaryBuilder<K, types::GenericBinaryType<i64>>
```

Builder for [`DictionaryArray`] of [`LargeBinaryArray`](crate::array::LargeBinaryArray)

---

## LargeStringDictionaryBuilder

`type_alias` · `arrow_array::builder::generic_bytes_dictionary_builder::LargeStringDictionaryBuilder`

```rust
type LargeStringDictionaryBuilder<K> = GenericByteDictionaryBuilder<K, types::GenericStringType<i64>>
```

Builder for [`DictionaryArray`] of [`LargeStringArray`](crate::array::LargeStringArray)

---

## StringDictionaryBuilder

`type_alias` · `arrow_array::builder::generic_bytes_dictionary_builder::StringDictionaryBuilder`

```rust
type StringDictionaryBuilder<K> = GenericByteDictionaryBuilder<K, types::GenericStringType<i32>>
```

Builder for [`DictionaryArray`] of [`StringArray`](crate::array::StringArray)

```
// Create a dictionary array indexed by bytes whose values are Strings.
// It can thus hold up to 256 distinct string values.

# use arrow_array::builder::StringDictionaryBuilder;
# use arrow_array::{Int8Array, StringArray};
# use arrow_array::types::Int8Type;

let mut builder = StringDictionaryBuilder::<Int8Type>::new();

// The builder builds the dictionary value by value
builder.append("abc").unwrap();
builder.append_null();
builder.append_n("def", 2).unwrap();  // appends "def" twice with a single lookup
builder.append("abc").unwrap();
let array = builder.finish();

assert_eq!(
  array.keys(),
  &Int8Array::from(vec![Some(0), None, Some(1), Some(1), Some(0)])
);

// Values are polymorphic and so require a downcast.
let av = array.values();
let ava: &StringArray = av.as_any().downcast_ref::<StringArray>().unwrap();

assert_eq!(ava.value(0), "abc");
assert_eq!(ava.value(1), "def");

```

---
