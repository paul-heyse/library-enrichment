# `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.primitive_dictionary_builder.PrimitiveDictionaryBuilder.json).

<a id="op-a12b3c003516f0ffac7bf241"></a>
## PrimitiveDictionaryBuilder

`struct` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder` · arrow-array 59.3.0

```rust
struct PrimitiveDictionaryBuilder<K, V> where K: ArrowPrimitiveType, V: ArrowPrimitiveType
```

Source: `src/builder/primitive_dictionary_builder.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) of [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814)

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

<a id="op-64c014e4ca96be5085afa965"></a>
## append

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::append` · arrow-array 59.3.0

```rust
fn append(&mut self, value: V::Native) -> Result<K::Native, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:301`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append a primitive value to the array. Return an existing index
if already present in the values array or a new index if the
value is appended to the values array.

<a id="op-955a6cb1cef8ff0c6b5dc51e"></a>
## append_n

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::append_n` · arrow-array 59.3.0

```rust
fn append_n(&mut self, value: V::Native, count: usize) -> Result<K::Native, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:311`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append a value multiple times to the array.
This is the same as `append` but allows to append the same value multiple times without doing multiple lookups.

Returns an error if the new index would overflow the key type.

<a id="op-ebb9ce8b84a622318cb55b3b"></a>
## append_null

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:340`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a null slot into the builder

<a id="op-37ddef5729e60ee0ae82561f"></a>
## append_nulls

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::append_nulls` · arrow-array 59.3.0

```rust
fn append_nulls(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:346`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append `n` null slots into the builder

<a id="op-747103ece176cd4002e90bb6"></a>
## append_option

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::append_option` · arrow-array 59.3.0

```rust
fn append_option(&mut self, value: Option<V::Native>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:356`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append an `Option` value into the builder

# Panics

Panics if the resulting length of the dictionary values array would exceed `T::Native::MAX`

<a id="op-ccb1434d68970eef1df93aff"></a>
## append_options

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::append_options` · arrow-array 59.3.0

```rust
fn append_options(&mut self, value: Option<V::Native>, count: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:369`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Append an `Option` value into the builder repeatedly `count` times.
This is the same as `append_option` but allows to append the same value multiple times without doing multiple lookups.

# Panics

Panics if the resulting length of the dictionary values array would exceed `T::Native::MAX`

<a id="op-758fea0118271a8899b85748"></a>
## append_value

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, value: V::Native)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:323`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Infallibly append a value to this builder

# Panics

Panics if the resulting length of the dictionary values array would exceed `T::Native::MAX`

<a id="op-29e608c63c8b91440719e664"></a>
## append_values

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::append_values` · arrow-array 59.3.0

```rust
fn append_values(&mut self, value: V::Native, count: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:333`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Infallibly append a value to this builder repeatedly `count` times.
This is the same as `append_value` but allows to append the same value multiple times without doing multiple lookups.

# Panics

Panics if the resulting length of the dictionary values array would exceed `T::Native::MAX`

<a id="op-b48500143b5cf157215f519f"></a>
## as_any

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [237, 1], "end": [275, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_dictionary_builder.rs:243`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as an non-mutable `Any` reference.

<a id="op-0504b90b6a174f89cf08b4b9"></a>
## as_any_mut

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [237, 1], "end": [275, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_dictionary_builder.rs:248`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as an mutable `Any` reference.

<a id="op-e90f3f246e2e49d5a3b823d4"></a>
## default

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::default` · arrow-array 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [93, 1], "end": [101, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder/primitive_dictionary_builder.rs:98`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9a7e5d593833754bea5f099"></a>
## extend

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::extend` · arrow-array 59.3.0

```rust
fn extend<T: IntoIterator<Item = Option<P::Native>>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "P"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "P"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [515, 1], "end": [524, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "P"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/builder/primitive_dictionary_builder.rs:519`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30cec6e3ad88f5d2abd33e13"></a>
## extend_dictionary

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::extend_dictionary` · arrow-array 59.3.0

```rust
fn extend_dictionary(&mut self, dictionary: &TypedDictionaryArray<'_, K, PrimitiveArray<V>>) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:383`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Extends builder with dictionary

This is the same as [`Self::extend`](../operations/arrow_array.builder.primitive_dictionary_builder.PrimitiveDictionaryBuilder.md#op-b9a7e5d593833754bea5f099) but is faster as it translates
the dictionary values once rather than doing a lookup for each item in the iterator

when dictionary values are null (the actual mapped values) the keys are null


<a id="op-b9cc2550166749718254c063"></a>
## finish

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [237, 1], "end": [275, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_dictionary_builder.rs:263`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array and reset this builder.

<a id="op-d9a4e1b513b56e95ebd89906"></a>
## finish

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> DictionaryArray<K>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:434`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the `DictionaryArray` and reset this builder.

<a id="op-024c18bb69398b6235453f5b"></a>
## finish_cloned

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [237, 1], "end": [275, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_dictionary_builder.rs:268`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-1b8428dadc93c13871560347"></a>
## finish_cloned

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> DictionaryArray<K>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:452`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the `DictionaryArray` without resetting the builder.

<a id="op-2b1686d68565abd805186fc8"></a>
## finish_preserve_values

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::finish_preserve_values` · arrow-array 59.3.0

```rust
fn finish_preserve_values(&mut self) -> DictionaryArray<K>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:484`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the `DictionaryArray` without resetting the values builder or
the internal de-duplication map.

The advantage of doing this is that the values will represent the entire
set of what has been built so-far by this builder and ensures
consistency in the assignment of keys to values across multiple calls
to `finish_preserve_values`. This enables ipc writers to efficiently
emit delta dictionaries.

The downside to this is that building the record requires creating a
copy of the values, which can become slowly more expensive if the
dictionary grows.

Additionally, if record batches from multiple different dictionary
builders for the same column are fed into a single ipc writer, beware
that entire dictionaries are likely to be re-sent frequently even when
the majority of the values are not used by the current record batch.

<a id="op-6890007ccd4cb6fff1a0b08b"></a>
## finish_preserve_values

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::finish_preserve_values` · arrow-array 59.3.0

```rust
fn finish_preserve_values(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [237, 1], "end": [275, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_dictionary_builder.rs:272`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60bb874280889d34571ba801"></a>
## fmt

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "V"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [82, 10], "end": [82, 15], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/primitive_dictionary_builder.rs:82`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0dc7332966da3efffd40fea8"></a>
## into_box_any

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [237, 1], "end": [275, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_dictionary_builder.rs:253`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-e5c14dba3ce2bcf8256664f4"></a>
## len

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [237, 1], "end": [275, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/primitive_dictionary_builder.rs:258`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of array slots in the builder

<a id="op-c4509e1f6173920247822084"></a>
## new

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::new` · arrow-array 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [235, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:109`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `PrimitiveDictionaryBuilder`.

<a id="op-40e5e1044331b175d5e03707"></a>
## new_from_builders

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::new_from_builders` · arrow-array 59.3.0

```rust
unsafe fn new_from_builders(keys_builder: PrimitiveBuilder<K>, values_builder: PrimitiveBuilder<V>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [235, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `PrimitiveDictionaryBuilder` from existing `PrimitiveBuilder`s of keys and values.

# Safety

caller must ensure that the passed in builders are valid for DictionaryArray.

<a id="op-30efd41e2ae34d70ba1022ac"></a>
## new_from_empty_builders

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::new_from_empty_builders` · arrow-array 59.3.0

```rust
fn new_from_empty_builders(keys_builder: PrimitiveBuilder<K>, values_builder: PrimitiveBuilder<V>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [235, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:122`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `PrimitiveDictionaryBuilder` from the provided keys and values builders.

# Panics

This method panics if `keys_builder` or `values_builder` is not empty.

<a id="op-e9f12cb5b42056903d4388a7"></a>
## try_new_from_builder

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::try_new_from_builder` · arrow-array 59.3.0

```rust
fn try_new_from_builder<K2>(source: PrimitiveDictionaryBuilder<K2, V>) -> Result<Self, ArrowError> where K::Native: NumCast, K2: ArrowDictionaryKeyType, K2::Native: NumCast
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [235, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:200`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `PrimitiveDictionaryBuilder` from the existing builder with the same
keys and values, but with a new data type for the keys.

# Example
```
#
# use arrow_array::builder::PrimitiveDictionaryBuilder;
# use arrow_array::types::{UInt8Type, UInt16Type, UInt64Type};
# use arrow_array::UInt16Array;
# use arrow_schema::ArrowError;

let mut u8_keyed_builder = PrimitiveDictionaryBuilder::<UInt8Type, UInt64Type>::new();

// appending too many values causes the dictionary to overflow
for i in 0..256 {
    u8_keyed_builder.append_value(i);
}
let result = u8_keyed_builder.append(256);
assert!(matches!(result, Err(ArrowError::DictionaryKeyOverflowError{})));

// we need to upgrade to a larger key type
let mut u16_keyed_builder = PrimitiveDictionaryBuilder::<UInt16Type, UInt64Type>::try_new_from_builder(u8_keyed_builder).unwrap();
let dictionary_array = u16_keyed_builder.finish();
let keys = dictionary_array.keys();

assert_eq!(keys, &UInt16Array::from_iter(0..256));

<a id="op-c47f75a8ac85c2f9cc4d24da"></a>
## validity_slice

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::validity_slice` · arrow-array 59.3.0

```rust
fn validity_slice(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:510`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer as a slice

<a id="op-feb4c65a86d530a9d8c0ba8f"></a>
## values_slice

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::values_slice` · arrow-array 59.3.0

```rust
fn values_slice(&self) -> &[V::Native]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:500`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current dictionary values buffer as a slice

<a id="op-72a034861355332daba5b629"></a>
## values_slice_mut

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::values_slice_mut` · arrow-array 59.3.0

```rust
fn values_slice_mut(&mut self) -> &mut [V::Native]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowDictionaryKeyType", "path": "ArrowDictionaryKeyType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [277, 1], "end": [513, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:505`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current dictionary values buffer as a mutable slice

<a id="op-bac717e644ca011dbaf63403"></a>
## with_capacity

`function` · `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(keys_capacity: usize, values_capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder", "path": "PrimitiveDictionaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [235, 2], "filename": "src/builder/primitive_dictionary_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/primitive_dictionary_builder.rs:166`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `PrimitiveDictionaryBuilder` with the provided capacities

`keys_capacity`: the number of keys, i.e. length of array to build
`values_capacity`: the number of distinct dictionary values, i.e. size of dictionary
