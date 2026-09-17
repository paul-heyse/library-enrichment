# `bstr::ext_vec`

Crate `bstr` · 5 public items · structured records in [`model/bstr.ext_vec.json`](../model/bstr.ext_vec.json)

## concat

`function` · `bstr::ext_vec::concat`

Also reachable as `bstr::concat`

```rust
fn concat<T, I>(elements: I) -> alloc::vec::Vec<u8> where T: AsRef<[u8]>, I: IntoIterator<Item = T>
```

Concatenate the elements given by the iterator together into a single
`Vec<u8>`.

The elements may be any type that can be cheaply converted into an `&[u8]`.
This includes, but is not limited to, `&str`, `&BStr` and `&[u8]` itself.

# Examples

Basic usage:

```
use bstr;

let s = bstr::concat(&["foo", "bar", "baz"]);
assert_eq!(s, "foobarbaz".as_bytes());
```

---

## join

`function` · `bstr::ext_vec::join`

Also reachable as `bstr::join`

```rust
fn join<B, T, I>(separator: B, elements: I) -> alloc::vec::Vec<u8> where B: AsRef<[u8]>, T: AsRef<[u8]>, I: IntoIterator<Item = T>
```

Join the elements given by the iterator with the given separator into a
single `Vec<u8>`.

Both the separator and the elements may be any type that can be cheaply
converted into an `&[u8]`. This includes, but is not limited to,
`&str`, `&BStr` and `&[u8]` itself.

# Examples

Basic usage:

```
use bstr;

let s = bstr::join(",", &["foo", "bar", "baz"]);
assert_eq!(s, "foo,bar,baz".as_bytes());
```

---

## DrainBytes

`struct` · `bstr::ext_vec::DrainBytes`

Also reachable as `bstr::DrainBytes`

```rust
struct DrainBytes<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<u8>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<u8>
```

A draining byte oriented iterator for `Vec<u8>`.

This iterator is created by
[`ByteVec::drain_bytes`](trait.ByteVec.html#method.drain_bytes).

# Examples

Basic usage:

```
use bstr::ByteVec;

let mut s = Vec::from("foobar");
{
    let mut drainer = s.drain_bytes(2..4);
    assert_eq!(drainer.next(), Some(b'o'));
    assert_eq!(drainer.next(), Some(b'b'));
    assert_eq!(drainer.next(), None);
}
assert_eq!(s, "foar".as_bytes());
```

---

## FromUtf8Error

`struct` · `bstr::ext_vec::FromUtf8Error`

Also reachable as `bstr::FromUtf8Error`

```rust
struct FromUtf8Error
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn as_bytes(&self) -> &[u8]
fn into_vec(self) -> Vec<u8>
fn utf8_error(&self) -> &Utf8Error
```

**via `core::error::Error`**

```rust
fn description(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

An error that may occur when converting a `Vec<u8>` to a `String`.

This error includes the original `Vec<u8>` that failed to convert to a
`String`. This permits callers to recover the allocation used even if it
it not valid UTF-8.

# Examples

Basic usage:

```
use bstr::{B, ByteVec};

let bytes = Vec::from_slice(b"foo\xFFbar");
let err = bytes.into_string().unwrap_err();

assert_eq!(err.utf8_error().valid_up_to(), 3);
assert_eq!(err.utf8_error().error_len(), Some(1));

// At no point in this example is an allocation performed.
let bytes = Vec::from(err.into_vec());
assert_eq!(bytes, B(b"foo\xFFbar"));
```

---

## ByteVec

`trait` · `bstr::ext_vec::ByteVec`

Also reachable as `bstr::ByteVec`

```rust
trait ByteVec: private::Sealed
```

**Implementors** (1)

- `alloc::vec::Vec`

**Methods** (26)

```rust
fn as_bstring(&self) -> &BString
fn as_bstring_mut(&mut self) -> &mut BString
fn drain_bytes<R>(&mut self, range: R) -> DrainBytes<'_> where R: ops::RangeBounds<usize>
fn from_os_str_lossy(os_str: &OsStr) -> Cow<'_, [u8]>
fn from_os_string(os_str: OsString) -> Result<Vec<u8>, OsString>
fn from_path_buf(path: PathBuf) -> Result<Vec<u8>, PathBuf>
fn from_path_lossy(path: &Path) -> Cow<'_, [u8]>
fn from_slice<B: AsRef<[u8]>>(bytes: B) -> Vec<u8>
fn insert_char(&mut self, at: usize, ch: char)
fn insert_str<B: AsRef<[u8]>>(&mut self, at: usize, bytes: B)
fn into_os_string(self) -> Result<OsString, FromUtf8Error> where Self: Sized
fn into_os_string_lossy(self) -> OsString where Self: Sized
fn into_path_buf(self) -> Result<PathBuf, FromUtf8Error> where Self: Sized
fn into_path_buf_lossy(self) -> PathBuf where Self: Sized
fn into_string(self) -> Result<String, FromUtf8Error> where Self: Sized
fn into_string_lossy(self) -> String where Self: Sized
unsafe fn into_string_unchecked(self) -> String where Self: Sized
fn pop_byte(&mut self) -> Option<u8>
fn pop_char(&mut self) -> Option<char>
fn push_byte(&mut self, byte: u8)
fn push_char(&mut self, ch: char)
fn push_str<B: AsRef<[u8]>>(&mut self, bytes: B)
fn remove_char(&mut self, at: usize) -> char
fn replace_range<R, B>(&mut self, range: R, replace_with: B) where R: ops::RangeBounds<usize>, B: AsRef<[u8]>
fn to_bstring(self) -> BString where Self: Sized
fn unescape_bytes<S: AsRef<str>>(escaped: S) -> Vec<u8>
```

A trait that extends `Vec<u8>` with string oriented methods.

Note that when using the constructor methods, such as
`ByteVec::from_slice`, one should actually call them using the concrete
type. For example:

```
use bstr::{B, ByteVec};

let s = Vec::from_slice(b"abc"); // NOT ByteVec::from_slice("...")
assert_eq!(s, B("abc"));
```

This trait is sealed and cannot be implemented outside of `bstr`.

---
