# `object_store::payload`

Crate `object_store` · 4 public items · structured records in [`model/object_store.payload.json`](../model/object_store.payload.json)

## PutPayload

`struct` · `object_store::payload::PutPayload`

```rust
struct PutPayload
```

**Implements**: `core::convert::AsRef`, `core::convert::From`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`

**Derives**: Clone, Debug, Default

**Methods** (5)

```rust
fn content_length(&self) -> usize
fn from_bytes(s: Bytes) -> Self
fn from_static(s: &'static [u8]) -> Self
fn iter(&self) -> PutPayloadIter<'_>
fn new() -> Self
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &[Bytes]
```

**via `core::convert::From`**

```rust
fn from(value: &'static [u8]) -> Self
fn from(value: PutPayloadMut) -> Self
fn from(value: &'static str) -> Self
fn from(value: Vec<u8>) -> Self
fn from(value: String) -> Self
fn from(value: Bytes) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = Bytes>>(iter: T) -> Self
fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

A cheaply cloneable, ordered collection of [`Bytes`]

---

## PutPayloadIntoIter

`struct` · `object_store::payload::PutPayloadIntoIter`

```rust
struct PutPayloadIntoIter
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

An owning iterator of [`PutPayload`]

---

## PutPayloadIter

`struct` · `object_store::payload::PutPayloadIter`

```rust
struct PutPayloadIter<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator over [`PutPayload`]

---

## PutPayloadMut

`struct` · `object_store::payload::PutPayloadMut`

```rust
struct PutPayloadMut
```

**Derives**: Debug, Default

**Methods** (7)

```rust
fn content_length(&self) -> usize
fn extend_from_slice(&mut self, slice: &[u8])
fn freeze(self) -> PutPayload
fn is_empty(&self) -> bool
fn new() -> Self
fn push(&mut self, bytes: Bytes)
fn with_block_size(self, block_size: usize) -> Self
```

A builder for [`PutPayload`] that avoids reallocating memory

Data is allocated in fixed blocks, which are flushed to [`Bytes`] once full.
Unlike [`Vec`] this avoids needing to repeatedly reallocate blocks of memory,
which typically involves copying all the previously written data to a new
contiguous memory region.

---
