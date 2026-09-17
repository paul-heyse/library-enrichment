# `opentelemetry::baggage`

Crate `opentelemetry` · 5 public items · structured records in [`model/opentelemetry.baggage.json`](../model/opentelemetry.baggage.json)

## Baggage

`struct` · `opentelemetry::baggage::Baggage`

```rust
struct Baggage
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::iter::traits::collect::FromIterator`

**Derives**: Debug, Default

**Methods** (9)

```rust
fn get<K: AsRef<str>>(&self, key: K) -> Option<&StringValue>
fn get_with_metadata<K: AsRef<str>>(&self, key: K) -> Option<&(StringValue, BaggageMetadata)>
fn insert<K, V>(&mut self, key: K, value: V) -> Option<StringValue> where K: Into<Key>, V: Into<StringValue>
fn insert_with_metadata<K, V, S>(&mut self, key: K, value: V, metadata: S) -> Option<(StringValue, BaggageMetadata)> where K: Into<Key>, V: Into<StringValue>, S: Into<BaggageMetadata>
fn is_empty(&self) -> bool
fn iter(&self) -> Iter<'_>
fn len(&self) -> usize
fn new() -> Self
fn remove<K: AsRef<str>>(&mut self, key: K) -> Option<(StringValue, BaggageMetadata)>
```

**via `core::convert::From`**

```rust
fn from(value: I) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = (Key, (StringValue, BaggageMetadata))>>(iter: I) -> Self
fn from_iter<I: IntoIterator<Item = KeyValue>>(iter: I) -> Self
fn from_iter<I: IntoIterator<Item = KeyValueMetadata>>(iter: I) -> Self
```

A set of name/value pairs describing user-defined properties.

### Baggage Names

* ASCII strings according to the token format, defined in [RFC2616, Section 2.2]

### Baggage Values

* URL encoded UTF-8 strings.

### Baggage Value Metadata

Additional metadata can be added to values in the form of a property set,
represented as semi-colon `;` delimited list of names and/or name/value pairs,
e.g. `;k1=v1;k2;k3=v3`.

### Limits

* Maximum number of name/value pairs: `64`.
* Maximum total length of all name/value pairs: `8192`.

<https://www.w3.org/TR/baggage/#limits>

---

## BaggageMetadata

`struct` · `opentelemetry::baggage::BaggageMetadata`

```rust
struct BaggageMetadata
```

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug, Default, Eq, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn as_str(&self) -> &str
```

**via `core::convert::From`**

```rust
fn from(s: String) -> BaggageMetadata
fn from(s: &str) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

An optional property set that can be added to [`Baggage`] values.

`BaggageMetadata` can be added to values in the form of a property set,
represented as semi-colon `;` delimited list of names and/or name/value
pairs, e.g. `;k1=v1;k2;k3=v3`.

---

## Iter

`struct` · `opentelemetry::baggage::Iter`

```rust
struct Iter<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

An iterator over the entries of a [`Baggage`].

---

## KeyValueMetadata

`struct` · `opentelemetry::baggage::KeyValueMetadata`

```rust
struct KeyValueMetadata
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new<K, V, S>(key: K, value: V, metadata: S) -> Self where K: Into<Key>, V: Into<StringValue>, S: Into<BaggageMetadata>
```

**via `core::convert::From`**

```rust
fn from(kv: KeyValue) -> Self
```

[`Baggage`] name/value pairs with their associated metadata.

---

## BaggageExt

`trait` · `opentelemetry::baggage::BaggageExt`

```rust
trait BaggageExt
```

**Implementors** (1)

- `opentelemetry::context::Context`

**Methods** (4)

```rust
fn baggage(&self) -> &Baggage
fn current_with_baggage<T: Into<Baggage>>(baggage: T) -> Self
fn with_baggage<T: Into<Baggage>>(&self, baggage: T) -> Self
fn with_cleared_baggage(&self) -> Self
```

Methods for sorting and retrieving baggage data in a context.

---
