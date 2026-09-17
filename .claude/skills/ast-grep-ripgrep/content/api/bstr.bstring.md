# `bstr::bstring`

Crate `bstr` · 1 public items · structured records in [`model/bstr.bstring.json`](../model/bstr.bstring.json)

## BString

`struct` · `bstr::bstring::BString`

Also reachable as `bstr::BString`

```rust
struct BString
```

**Implements**: `core::borrow::Borrow`, `core::borrow::BorrowMut`, `core::convert::AsMut`, `core::convert::AsRef`, `core::convert::From`, `core::fmt::Display`, `core::iter::traits::collect::FromIterator`, `core::ops::deref::Deref`, `core::ops::deref::DerefMut`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (1)

```rust
const fn new(bytes: Vec<u8>) -> BString
```

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &BStr
fn borrow(&self) -> &[u8]
```

**via `core::borrow::BorrowMut`**

```rust
fn borrow_mut(&mut self) -> &mut BStr
fn borrow_mut(&mut self) -> &mut [u8]
```

**via `core::convert::AsMut`**

```rust
fn as_mut(&mut self) -> &mut BStr
fn as_mut(&mut self) -> &mut [u8]
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &[u8]
fn as_ref(&self) -> &BStr
```

**via `core::convert::From`**

```rust
fn from(s: &'a str) -> BString
fn from(s: Vec<u8>) -> BString
fn from(s: [u8; N]) -> BString
fn from(s: &'a BStr) -> BString
fn from(s: String) -> BString
fn from(s: &'a [u8]) -> BString
fn from(s: &'a [u8; N]) -> BString
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = BString>>(iter: T) -> BString
fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> BString
fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> BString
fn from_iter<T: IntoIterator<Item = &'a BStr>>(iter: T) -> BString
fn from_iter<T: IntoIterator<Item = &'a [u8]>>(iter: T) -> BString
fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> BString
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Vec<u8>
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut Vec<u8>
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<BString, Utf8Error>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<BString, D::Error> where D: Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer
```

A wrapper for `Vec<u8>` that provides convenient string oriented trait
impls.

A `BString` has ownership over its contents and corresponds to
a growable or shrinkable buffer. Its borrowed counterpart is a
[`BStr`](struct.BStr.html), called a byte string slice.

Using a `BString` is just like using a `Vec<u8>`, since `BString`
implements `Deref` to `Vec<u8>`. So all methods available on `Vec<u8>`
are also available on `BString`.

# Examples

You can create a new `BString` from a `Vec<u8>` via a `From` impl:

```
use bstr::BString;

let s = BString::from("Hello, world!");
```

# Deref

The `BString` type implements `Deref` and `DerefMut`, where the target
types are `&Vec<u8>` and `&mut Vec<u8>`, respectively. `Deref` permits all of the
methods defined on `Vec<u8>` to be implicitly callable on any `BString`.

For more information about how deref works, see the documentation for the
[`std::ops::Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html)
trait.

# Representation

A `BString` has the same representation as a `Vec<u8>` and a `String`.
That is, it is made up of three word sized components: a pointer to a
region of memory containing the bytes, a length and a capacity.

---
