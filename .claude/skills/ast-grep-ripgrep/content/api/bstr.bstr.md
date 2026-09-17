# `bstr::bstr`

Crate `bstr` · 1 public items · structured records in [`model/bstr.bstr.json`](../model/bstr.bstr.json)

## BStr

`struct` · `bstr::bstr::BStr`

Also reachable as `bstr::BStr`

```rust
struct BStr
```

**Implements**: `alloc::borrow::ToOwned`, `core::borrow::Borrow`, `core::borrow::BorrowMut`, `core::convert::AsMut`, `core::convert::AsRef`, `core::fmt::Display`, `core::ops::deref::Deref`, `core::ops::deref::DerefMut`, `core::ops::index::Index`, `core::ops::index::IndexMut`, `serde_core::ser::Serialize`

**Derives**: Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (1)

```rust
fn new<B: ?Sized + AsRef<[u8]>>(bytes: &B) -> &BStr
```

**via `alloc::borrow::ToOwned`**

```rust
fn to_owned(&self) -> BString
```

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &[u8]
```

**via `core::borrow::BorrowMut`**

```rust
fn borrow_mut(&mut self) -> &mut [u8]
```

**via `core::convert::AsMut`**

```rust
fn as_mut(&mut self) -> &mut [u8]
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &BStr
fn as_ref(&self) -> &[u8]
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &[u8]
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut [u8]
```

**via `core::ops::index::Index`**

```rust
fn index(&self, r: ops::RangeToInclusive<usize>) -> &BStr
fn index(&self, r: ops::RangeTo<usize>) -> &BStr
fn index(&self, r: ops::RangeFrom<usize>) -> &BStr
fn index(&self, r: ops::RangeInclusive<usize>) -> &BStr
fn index(&self, r: ops::Range<usize>) -> &BStr
fn index(&self, _: ops::RangeFull) -> &BStr
fn index(&self, idx: usize) -> &u8
```

**via `core::ops::index::IndexMut`**

```rust
fn index_mut(&mut self, r: ops::RangeToInclusive<usize>) -> &mut BStr
fn index_mut(&mut self, r: ops::RangeFrom<usize>) -> &mut BStr
fn index_mut(&mut self, r: ops::Range<usize>) -> &mut BStr
fn index_mut(&mut self, idx: usize) -> &mut u8
fn index_mut(&mut self, r: ops::RangeTo<usize>) -> &mut BStr
fn index_mut(&mut self, r: ops::RangeInclusive<usize>) -> &mut BStr
fn index_mut(&mut self, _: ops::RangeFull) -> &mut BStr
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer
```

A wrapper for `&[u8]` that provides convenient string oriented trait impls.

If you need ownership or a growable byte string buffer, then use
[`BString`](struct.BString.html).

Using a `&BStr` is just like using a `&[u8]`, since `BStr`
implements `Deref` to `[u8]`. So all methods available on `[u8]`
are also available on `BStr`.

# Representation

A `&BStr` has the same representation as a `&str`. That is, a `&BStr` is
a fat pointer which consists of a pointer to some bytes and a length.

# Trait implementations

The `BStr` type has a number of trait implementations, and in particular,
defines equality and ordinal comparisons between `&BStr`, `&str` and
`&[u8]` for convenience.

The `Debug` implementation for `BStr` shows its bytes as a normal string.
For invalid UTF-8, hex escape sequences are used.

The `Display` implementation behaves as if `BStr` were first lossily
converted to a `str`. Invalid UTF-8 bytes are substituted with the Unicode
replacement codepoint, which looks like this: �.

---
