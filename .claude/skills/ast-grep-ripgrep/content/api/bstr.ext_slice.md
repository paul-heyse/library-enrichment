# `bstr::ext_slice`

Crate `bstr` · 15 public items · structured records in [`model/bstr.ext_slice.json`](../model/bstr.ext_slice.json)

## B

`function` · `bstr::ext_slice::B`

Also reachable as `bstr::B`

```rust
fn B<B: ?Sized + AsRef<[u8]>>(bytes: &B) -> &[u8]
```

A short-hand constructor for building a `&[u8]`.

This idiosyncratic constructor is useful for concisely building byte string
slices. Its primary utility is in conveniently writing byte string literals
in a uniform way. For example, consider this code that does not compile:

```ignore
let strs = vec![b"a", b"xy"];
```

The above code doesn't compile because the type of the byte string literal
`b"a"` is `&'static [u8; 1]`, and the type of `b"xy"` is
`&'static [u8; 2]`. Since their types aren't the same, they can't be stored
in the same `Vec`. (This is dissimilar from normal Unicode string slices,
where both `"a"` and `"xy"` have the same type of `&'static str`.)

One way of getting the above code to compile is to convert byte strings to
slices. You might try this:

```ignore
let strs = vec![&b"a", &b"xy"];
```

But this just creates values with type `& &'static [u8; 1]` and
`& &'static [u8; 2]`. Instead, you need to force the issue like so:

```
let strs = vec![&b"a"[..], &b"xy"[..]];
// or
let strs = vec![b"a".as_ref(), b"xy".as_ref()];
```

But neither of these are particularly convenient to type, especially when
it's something as common as a string literal. Thus, this constructor
permits writing the following instead:

```
use bstr::B;

let strs = vec![B("a"), B(b"xy")];
```

Notice that this also lets you mix and match both string literals and byte
string literals. This can be quite convenient!

---

## Bytes

`struct` · `bstr::ext_slice::Bytes`

Also reachable as `bstr::Bytes`

```rust
struct Bytes<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn as_bytes(&self) -> &'a [u8]
```

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
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator over the bytes in a byte string.

`'a` is the lifetime of the byte string being traversed.

---

## Fields

`struct` · `bstr::ext_slice::Fields`

Also reachable as `bstr::Fields`

```rust
struct Fields<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'a [u8]>
```

An iterator over the fields in a byte string, separated by whitespace.

Whitespace for this iterator is defined by the Unicode property
`White_Space`.

This iterator splits on contiguous runs of whitespace, such that the fields
in `foo\t\t\n  \nbar` are `foo` and `bar`.

`'a` is the lifetime of the byte string being split.

---

## FieldsWith

`struct` · `bstr::ext_slice::FieldsWith`

Also reachable as `bstr::FieldsWith`

```rust
struct FieldsWith<'a, F>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'a [u8]>
```

An iterator over fields in the byte string, separated by a predicate over
codepoints.

This iterator splits a byte string based on its predicate function such
that the elements returned are separated by contiguous runs of codepoints
for which the predicate returns true.

`'a` is the lifetime of the byte string being split, while `F` is the type
of the predicate, i.e., `FnMut(char) -> bool`.

---

## Find

`struct` · `bstr::ext_slice::Find`

Also reachable as `bstr::Find`

```rust
struct Find<'h, 'n>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<usize>
```

An iterator over non-overlapping substring matches.

Matches are reported by the byte offset at which they begin.

`'h` is the lifetime of the haystack while `'n` is the lifetime of the
needle.

---

## FindReverse

`struct` · `bstr::ext_slice::FindReverse`

Also reachable as `bstr::FindReverse`

```rust
struct FindReverse<'h, 'n>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<usize>
```

An iterator over non-overlapping substring matches in reverse.

Matches are reported by the byte offset at which they begin.

`'h` is the lifetime of the haystack while `'n` is the lifetime of the
needle.

---

## Finder

`struct` · `bstr::ext_slice::Finder`

Also reachable as `bstr::Finder`

```rust
struct Finder<'a>
```

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn find<B: AsRef<[u8]>>(&self, haystack: B) -> Option<usize>
fn into_owned(self) -> Finder<'static>
fn needle(&self) -> &[u8]
fn new<B: ?Sized + AsRef<[u8]>>(needle: &'a B) -> Finder<'a>
```

A single substring searcher fixed to a particular needle.

The purpose of this type is to permit callers to construct a substring
searcher that can be used to search haystacks without the overhead of
constructing the searcher in the first place. This is a somewhat niche
concern when it's necessary to reuse the same needle to search multiple
different haystacks with as little overhead as possible. In general, using
[`ByteSlice::find`](trait.ByteSlice.html#method.find)
or
[`ByteSlice::find_iter`](trait.ByteSlice.html#method.find_iter)
is good enough, but `Finder` is useful when you can meaningfully observe
searcher construction time in a profile.

When the `std` feature is enabled, then this type has an `into_owned`
version which permits building a `Finder` that is not connected to the
lifetime of its needle.

---

## FinderReverse

`struct` · `bstr::ext_slice::FinderReverse`

Also reachable as `bstr::FinderReverse`

```rust
struct FinderReverse<'a>
```

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn into_owned(self) -> FinderReverse<'static>
fn needle(&self) -> &[u8]
fn new<B: ?Sized + AsRef<[u8]>>(needle: &'a B) -> FinderReverse<'a>
fn rfind<B: AsRef<[u8]>>(&self, haystack: B) -> Option<usize>
```

A single substring reverse searcher fixed to a particular needle.

The purpose of this type is to permit callers to construct a substring
searcher that can be used to search haystacks without the overhead of
constructing the searcher in the first place. This is a somewhat niche
concern when it's necessary to re-use the same needle to search multiple
different haystacks with as little overhead as possible. In general, using
[`ByteSlice::rfind`](trait.ByteSlice.html#method.rfind)
or
[`ByteSlice::rfind_iter`](trait.ByteSlice.html#method.rfind_iter)
is good enough, but `FinderReverse` is useful when you can meaningfully
observe searcher construction time in a profile.

When the `std` feature is enabled, then this type has an `into_owned`
version which permits building a `FinderReverse` that is not connected to
the lifetime of its needle.

---

## Lines

`struct` · `bstr::ext_slice::Lines`

Also reachable as `bstr::Lines`

```rust
struct Lines<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn as_bytes(&self) -> &'a [u8]
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'a [u8]>
```

An iterator over all lines in a byte string, without their terminators.

For this iterator, the only line terminators recognized are `\r\n` and
`\n`.

`'a` is the lifetime of the byte string being iterated over.

---

## LinesWithTerminator

`struct` · `bstr::ext_slice::LinesWithTerminator`

Also reachable as `bstr::LinesWithTerminator`

```rust
struct LinesWithTerminator<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn as_bytes(&self) -> &'a [u8]
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'a [u8]>
```

An iterator over all lines in a byte string, including their terminators.

For this iterator, the only line terminator recognized is `\n`. (Since
line terminators are included, this also handles `\r\n` line endings.)

Line terminators are only included if they are present in the original
byte string. For example, the last line in a byte string may not end with
a line terminator.

Concatenating all elements yielded by this iterator is guaranteed to yield
the original byte string.

`'a` is the lifetime of the byte string being iterated over.

---

## Split

`struct` · `bstr::ext_slice::Split`

Also reachable as `bstr::Split`

```rust
struct Split<'h, 's>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'h [u8]>
```

An iterator over substrings in a byte string, split by a separator.

`'h` is the lifetime of the byte string being split (the haystack), while
`'s` is the lifetime of the byte string doing the splitting.

---

## SplitN

`struct` · `bstr::ext_slice::SplitN`

Also reachable as `bstr::SplitN`

```rust
struct SplitN<'h, 's>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'h [u8]>
```

An iterator over at most `n` substrings in a byte string, split by a
separator.

`'h` is the lifetime of the byte string being split (the haystack), while
`'s` is the lifetime of the byte string doing the splitting.

---

## SplitNReverse

`struct` · `bstr::ext_slice::SplitNReverse`

Also reachable as `bstr::SplitNReverse`

```rust
struct SplitNReverse<'h, 's>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'h [u8]>
```

An iterator over at most `n` substrings in a byte string, split by a
separator, in reverse.

`'h` is the lifetime of the byte string being split (the haystack), while
`'s` is the lifetime of the byte string doing the splitting.

---

## SplitReverse

`struct` · `bstr::ext_slice::SplitReverse`

Also reachable as `bstr::SplitReverse`

```rust
struct SplitReverse<'h, 's>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'h [u8]>
```

An iterator over substrings in a byte string, split by a separator, in
reverse.

`'h` is the lifetime of the byte string being split (the haystack), while
`'s` is the lifetime of the byte string doing the splitting.

---

## ByteSlice

`trait` · `bstr::ext_slice::ByteSlice`

Also reachable as `bstr::ByteSlice`

```rust
trait ByteSlice: private::Sealed
```

**Methods** (76)

```rust
fn as_bstr(&self) -> &BStr
fn as_bstr_mut(&mut self) -> &mut BStr
fn bytes(&self) -> Bytes<'_>
fn char_indices(&self) -> CharIndices<'_>
fn chars(&self) -> Chars<'_>
fn contains_str<B: AsRef<[u8]>>(&self, needle: B) -> bool
fn ends_with_str<B: AsRef<[u8]>>(&self, suffix: B) -> bool
fn escape_bytes(&self) -> EscapeBytes<'_>
fn fields(&self) -> Fields<'_>
fn fields_with<F: FnMut(char) -> bool>(&self, f: F) -> FieldsWith<'_, F>
fn find<B: AsRef<[u8]>>(&self, needle: B) -> Option<usize>
fn find_byte(&self, byte: u8) -> Option<usize>
fn find_byteset<B: AsRef<[u8]>>(&self, byteset: B) -> Option<usize>
fn find_char(&self, ch: char) -> Option<usize>
fn find_iter<'h, 'n, B: ?Sized + AsRef<[u8]>>(&'h self, needle: &'n B) -> Find<'h, 'n>
fn find_non_ascii_byte(&self) -> Option<usize>
fn find_not_byteset<B: AsRef<[u8]>>(&self, byteset: B) -> Option<usize>
fn from_os_str(os_str: &OsStr) -> Option<&[u8]>
fn from_path(path: &Path) -> Option<&[u8]>
fn grapheme_indices(&self) -> GraphemeIndices<'_>
fn graphemes(&self) -> Graphemes<'_>
fn is_ascii(&self) -> bool
fn is_utf8(&self) -> bool
fn last_byte(&self) -> Option<u8>
fn lines(&self) -> Lines<'_>
fn lines_with_terminator(&self) -> LinesWithTerminator<'_>
fn make_ascii_lowercase(&mut self)
fn make_ascii_uppercase(&mut self)
fn repeatn(&self, n: usize) -> Vec<u8>
fn replace<N: AsRef<[u8]>, R: AsRef<[u8]>>(&self, needle: N, replacement: R) -> Vec<u8>
fn replace_into<N: AsRef<[u8]>, R: AsRef<[u8]>>(&self, needle: N, replacement: R, dest: &mut Vec<u8>)
fn replacen<N: AsRef<[u8]>, R: AsRef<[u8]>>(&self, needle: N, replacement: R, limit: usize) -> Vec<u8>
fn replacen_into<N: AsRef<[u8]>, R: AsRef<[u8]>>(&self, needle: N, replacement: R, limit: usize, dest: &mut Vec<u8>)
fn reverse_bytes(&mut self)
fn reverse_chars(&mut self)
fn reverse_graphemes(&mut self)
fn rfind<B: AsRef<[u8]>>(&self, needle: B) -> Option<usize>
fn rfind_byte(&self, byte: u8) -> Option<usize>
fn rfind_byteset<B: AsRef<[u8]>>(&self, byteset: B) -> Option<usize>
fn rfind_char(&self, ch: char) -> Option<usize>
fn rfind_iter<'h, 'n, B: ?Sized + AsRef<[u8]>>(&'h self, needle: &'n B) -> FindReverse<'h, 'n>
fn rfind_not_byteset<B: AsRef<[u8]>>(&self, byteset: B) -> Option<usize>
fn rsplit_once_str<'a, B: ?Sized + AsRef<[u8]>>(&'a self, splitter: &B) -> Option<(&'a [u8], &'a [u8])>
fn rsplit_str<'h, 's, B: ?Sized + AsRef<[u8]>>(&'h self, splitter: &'s B) -> SplitReverse<'h, 's>
fn rsplitn_str<'h, 's, B: ?Sized + AsRef<[u8]>>(&'h self, limit: usize, splitter: &'s B) -> SplitNReverse<'h, 's>
fn sentence_indices(&self) -> SentenceIndices<'_>
fn sentences(&self) -> Sentences<'_>
fn split_once_str<'a, B: ?Sized + AsRef<[u8]>>(&'a self, splitter: &B) -> Option<(&'a [u8], &'a [u8])>
fn split_str<'h, 's, B: ?Sized + AsRef<[u8]>>(&'h self, splitter: &'s B) -> Split<'h, 's>
fn splitn_str<'h, 's, B: ?Sized + AsRef<[u8]>>(&'h self, limit: usize, splitter: &'s B) -> SplitN<'h, 's>
fn starts_with_str<B: AsRef<[u8]>>(&self, prefix: B) -> bool
fn to_ascii_lowercase(&self) -> Vec<u8>
fn to_ascii_uppercase(&self) -> Vec<u8>
fn to_lowercase(&self) -> Vec<u8>
fn to_lowercase_into(&self, buf: &mut Vec<u8>)
fn to_os_str(&self) -> Result<&OsStr, Utf8Error>
fn to_os_str_lossy(&self) -> Cow<'_, OsStr>
fn to_path(&self) -> Result<&Path, Utf8Error>
fn to_path_lossy(&self) -> Cow<'_, Path>
fn to_str(&self) -> Result<&str, Utf8Error>
fn to_str_lossy(&self) -> Cow<'_, str>
fn to_str_lossy_into(&self, dest: &mut String)
unsafe fn to_str_unchecked(&self) -> &str
fn to_uppercase(&self) -> Vec<u8>
fn to_uppercase_into(&self, buf: &mut Vec<u8>)
fn trim(&self) -> &[u8]
fn trim_end(&self) -> &[u8]
fn trim_end_with<F: FnMut(char) -> bool>(&self, trim: F) -> &[u8]
fn trim_start(&self) -> &[u8]
fn trim_start_with<F: FnMut(char) -> bool>(&self, trim: F) -> &[u8]
fn trim_with<F: FnMut(char) -> bool>(&self, trim: F) -> &[u8]
fn utf8_chunks(&self) -> Utf8Chunks<'_>
fn word_indices(&self) -> WordIndices<'_>
fn words(&self) -> Words<'_>
fn words_with_break_indices(&self) -> WordsWithBreakIndices<'_>
fn words_with_breaks(&self) -> WordsWithBreaks<'_>
```

A trait that extends `&[u8]` with string oriented methods.

This trait is sealed and cannot be implemented outside of `bstr`.

---
