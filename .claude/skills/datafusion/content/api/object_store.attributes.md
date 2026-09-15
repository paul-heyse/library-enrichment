# `object_store::attributes`

Crate `object_store` · 4 public items · structured records in [`model/object_store.attributes.json`](../model/object_store.attributes.json)

## Attribute

`enum` · `object_store::attributes::Attribute`

```rust
enum Attribute
```

**Variants**: `ContentDisposition`, `ContentEncoding`, `ContentLanguage`, `ContentType`, `CacheControl`, `StorageClass`, `Metadata`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

Additional object attribute types

---

## AttributeValue

`struct` · `object_store::attributes::AttributeValue`

```rust
struct AttributeValue
```

**Implements**: `core::convert::AsRef`, `core::convert::From`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::convert::From`**

```rust
fn from(value: String) -> Self
fn from(value: &'static str) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

The value of an [`Attribute`]

Provides efficient conversion from both static and owned strings

```
# use object_store::AttributeValue;
// Can use static strings without needing an allocation
let value = AttributeValue::from("bar");
// Can also store owned strings
let value = AttributeValue::from("foo".to_string());
```

---

## Attributes

`struct` · `object_store::attributes::Attributes`

```rust
struct Attributes
```

**Implements**: `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn get(&self, key: &Attribute) -> Option<&AttributeValue>
fn insert(&mut self, key: Attribute, value: AttributeValue) -> Option<AttributeValue>
fn is_empty(&self) -> bool
fn iter(&self) -> AttributesIter<'_>
fn len(&self) -> usize
fn new() -> Self
fn remove(&mut self, key: &Attribute) -> Option<AttributeValue>
fn with_capacity(capacity: usize) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self
```

Additional attributes of an object

Attributes can be specified in [PutOptions](crate::PutOptions) and retrieved
from APIs returning [GetResult](crate::GetResult).

Unlike [`ObjectMeta`](crate::ObjectMeta), [`Attributes`] are not returned by
listing APIs

---

## AttributesIter

`struct` · `object_store::attributes::AttributesIter`

```rust
struct AttributesIter<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

Iterator over [`Attributes`]

---
