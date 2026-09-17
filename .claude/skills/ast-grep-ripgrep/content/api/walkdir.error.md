# `walkdir::error`

Crate `walkdir` · 1 public items · structured records in [`model/walkdir.error.json`](../model/walkdir.error.json)

## Error

`struct` · `walkdir::error::Error`

Also reachable as `walkdir::Error`

```rust
struct Error
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**Methods** (5)

```rust
fn depth(&self) -> usize
fn into_io_error(self) -> Option<io::Error>
fn io_error(&self) -> Option<&io::Error>
fn loop_ancestor(&self) -> Option<&Path>
fn path(&self) -> Option<&Path>
```

**via `core::error::Error`**

```rust
fn cause(&self) -> Option<&dyn error::Error>
fn description(&self) -> &str
fn source(&self) -> Option<&dyn error::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

An error produced by recursively walking a directory.

This error type is a light wrapper around [`std::io::Error`]. In
particular, it adds the following information:

* The depth at which the error occurred in the file tree, relative to the
root.
* The path, if any, associated with the IO error.
* An indication that a loop occurred when following symbolic links. In this
case, there is no underlying IO error.

To maintain good ergonomics, this type has a
[`impl From<Error> for std::io::Error`][impl] defined which preserves the original context.
This allows you to use an [`io::Result`] with methods in this crate if you don't care about
accessing the underlying error data in a structured form.

[`std::io::Error`]: https://doc.rust-lang.org/stable/std/io/struct.Error.html
[`io::Result`]: https://doc.rust-lang.org/stable/std/io/type.Result.html
[impl]: struct.Error.html#impl-From%3CError%3E

---
