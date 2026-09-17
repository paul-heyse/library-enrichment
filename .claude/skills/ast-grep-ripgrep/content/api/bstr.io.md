# `bstr::io`

Crate `bstr` · 3 public items · structured records in [`model/bstr.io.json`](../model/bstr.io.json)

## ByteLines

`struct` · `bstr::io::ByteLines`

```rust
struct ByteLines<B>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<io::Result<Vec<u8>>>
```

An iterator over lines from an instance of
[`std::io::BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html).

This iterator is generally created by calling the
[`byte_lines`](trait.BufReadExt.html#method.byte_lines)
method on the
[`BufReadExt`](trait.BufReadExt.html)
trait.

---

## ByteRecords

`struct` · `bstr::io::ByteRecords`

```rust
struct ByteRecords<B>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<io::Result<Vec<u8>>>
```

An iterator over records from an instance of
[`std::io::BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html).

A byte record is any sequence of bytes terminated by a particular byte
chosen by the caller. For example, NUL separated byte strings are said to
be NUL-terminated byte records.

This iterator is generally created by calling the
[`byte_records`](trait.BufReadExt.html#method.byte_records)
method on the
[`BufReadExt`](trait.BufReadExt.html)
trait.

---

## BufReadExt

`trait` · `bstr::io::BufReadExt`

```rust
trait BufReadExt: io::BufRead
```

**Methods** (6)

```rust
fn byte_lines(self) -> ByteLines<Self> where Self: Sized
fn byte_records(self, terminator: u8) -> ByteRecords<Self> where Self: Sized
fn for_byte_line<F>(&mut self, for_each_line: F) -> io::Result<()> where Self: Sized, F: FnMut(&[u8]) -> io::Result<bool>
fn for_byte_line_with_terminator<F>(&mut self, for_each_line: F) -> io::Result<()> where Self: Sized, F: FnMut(&[u8]) -> io::Result<bool>
fn for_byte_record<F>(&mut self, terminator: u8, for_each_record: F) -> io::Result<()> where Self: Sized, F: FnMut(&[u8]) -> io::Result<bool>
fn for_byte_record_with_terminator<F>(&mut self, terminator: u8, for_each_record: F) -> io::Result<()> where Self: Sized, F: FnMut(&[u8]) -> io::Result<bool>
```

An extension trait for
[`std::io::BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html)
which provides convenience APIs for dealing with byte strings.

---
