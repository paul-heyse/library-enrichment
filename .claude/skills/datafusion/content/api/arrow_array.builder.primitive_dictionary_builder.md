# `arrow_array::builder::primitive_dictionary_builder`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.builder.primitive_dictionary_builder.json`](../model/arrow_array.builder.primitive_dictionary_builder.json)

## PrimitiveDictionaryBuilder

`struct` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder`

```rust
struct PrimitiveDictionaryBuilder<K, V> where K: ArrowPrimitiveType, V: ArrowPrimitiveType
```

**Implements**: `arrow_array::builder::ArrayBuilder`, `core::iter::traits::collect::Extend`

**Derives**: Debug, Default

**Methods** (20)

```rust
fn append(&mut self, value: V::Native) -> Result<K::Native, ArrowError>
fn append_n(&mut self, value: V::Native, count: usize) -> Result<K::Native, ArrowError>
fn append_null(&mut self)
fn append_nulls(&mut self, n: usize)
fn append_option(&mut self, value: Option<V::Native>)
fn append_options(&mut self, value: Option<V::Native>, count: usize)
fn append_value(&mut self, value: V::Native)
fn append_values(&mut self, value: V::Native, count: usize)
fn extend_dictionary(&mut self, dictionary: &TypedDictionaryArray<'_, K, PrimitiveArray<V>>) -> Result<(), ArrowError>
fn finish(&mut self) -> DictionaryArray<K>
fn finish_cloned(&self) -> DictionaryArray<K>
fn finish_preserve_values(&mut self) -> DictionaryArray<K>
fn new() -> Self
unsafe fn new_from_builders(keys_builder: PrimitiveBuilder<K>, values_builder: PrimitiveBuilder<V>) -> Self
fn new_from_empty_builders(keys_builder: PrimitiveBuilder<K>, values_builder: PrimitiveBuilder<V>) -> Self
fn try_new_from_builder<K2>(source: PrimitiveDictionaryBuilder<K2, V>) -> Result<Self, ArrowError> where K::Native: NumCast, K2: ArrowDictionaryKeyType, K2::Native: NumCast
fn validity_slice(&self) -> Option<&[u8]>
fn values_slice(&self) -> &[V::Native]
fn values_slice_mut(&mut self) -> &mut [V::Native]
fn with_capacity(keys_capacity: usize, values_capacity: usize) -> Self
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
fn extend<T: IntoIterator<Item = Option<P::Native>>>(&mut self, iter: T)
```

Builder for [`DictionaryArray`] of [`PrimitiveArray`]

# Example:

```

# use arrow_array::builder::PrimitiveDictionaryBuilder;
# use arrow_array::types::{UInt32Type, UInt8Type};
# use arrow_array::{Array, UInt32Array, UInt8Array};

let mut builder = PrimitiveDictionaryBuilder::<UInt8Type, UInt32Type>::new();
 builder.append(12345678).unwrap();
 builder.append_null();
 builder.append(22345678).unwrap();
 let array = builder.finish();

 assert_eq!(
     array.keys(),
     &UInt8Array::from(vec![Some(0), None, Some(1)])
 );

 // Values are polymorphic and so require a downcast.
 let av = array.values();
 let ava: &UInt32Array = av.as_any().downcast_ref::<UInt32Array>().unwrap();
 let avs: &[u32] = ava.values();

 assert!(!array.is_null(0));
 assert!(array.is_null(1));
 assert!(!array.is_null(2));

 assert_eq!(avs, &[12345678, 22345678]);
```

---
