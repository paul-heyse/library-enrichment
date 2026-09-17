# `tracing_subscriber::fmt::writer`

Crate `tracing-subscriber` · 12 public items · structured records in [`model/tracing_subscriber.fmt.writer.json`](../model/tracing_subscriber.fmt.writer.json)

## EitherWriter

`enum` · `tracing_subscriber::fmt::writer::EitherWriter`

```rust
enum EitherWriter<A, B>
```

**Variants**: `A`, `B`

**Implements**: `std::io::Write`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn none() -> Self
fn some(t: T) -> Self
```

**via `std::io::Write`**

```rust
fn flush(&mut self) -> io::Result<()>
fn write(&mut self, buf: &[u8]) -> io::Result<usize>
fn write_all(&mut self, buf: &[u8]) -> io::Result<()>
fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> io::Result<()>
fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize>
```

A [writer] that is one of two types implementing [`io::Write`].

This may be used by [`MakeWriter`] implementations that may conditionally
return one of two writers.

[writer]: std::io::Write

---

## BoxMakeWriter

`struct` · `tracing_subscriber::fmt::writer::BoxMakeWriter`

```rust
struct BoxMakeWriter
```

**Implements**: `tracing_subscriber::fmt::writer::MakeWriter`

**Derives**: Debug

**Methods** (1)

```rust
fn new<M>(make_writer: M) -> Self where M: for<'a> MakeWriter<'a> + Send + Sync + 'static
```

**via `tracing_subscriber::fmt::writer::MakeWriter`**

```rust
fn make_writer(&'a self) -> Self::Writer
fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer
```

A writer that erases the specific [`io::Write`] and [`MakeWriter`] types being used.

This is useful in cases where the concrete type of the writer cannot be known
until runtime.

# Examples

A function that returns a [`Subscriber`] that will write to either stdout or stderr:

```rust
# use tracing::Subscriber;
# use tracing_subscriber::fmt::writer::BoxMakeWriter;

fn dynamic_writer(use_stderr: bool) -> impl Subscriber {
    let writer = if use_stderr {
        BoxMakeWriter::new(std::io::stderr)
    } else {
        BoxMakeWriter::new(std::io::stdout)
    };

    tracing_subscriber::fmt().with_writer(writer).finish()
}
```

[`Subscriber`]: tracing::Subscriber
[`io::Write`]: std::io::Write

---

## MutexGuardWriter

`struct` · `tracing_subscriber::fmt::writer::MutexGuardWriter`

```rust
struct MutexGuardWriter<'a, W>
```

**Implements**: `std::io::Write`

**Derives**: Debug

**via `std::io::Write`**

```rust
fn flush(&mut self) -> io::Result<()>
fn write(&mut self, buf: &[u8]) -> io::Result<usize>
fn write_all(&mut self, buf: &[u8]) -> io::Result<()>
fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> io::Result<()>
fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize>
```

A type implementing [`io::Write`] for a [`MutexGuard`] where the type
inside the [`Mutex`] implements [`io::Write`].

This is used by the [`MakeWriter`] implementation for [`Mutex`], because
[`MutexGuard`] itself will not implement [`io::Write`] — instead, it
_dereferences_ to a type implementing [`io::Write`]. Because [`MakeWriter`]
requires the `Writer` type to implement [`io::Write`], it's necessary to add
a newtype that forwards the trait implementation.

[`io::Write`]: std::io::Write
[`MutexGuard`]: std::sync::MutexGuard
[`Mutex`]: std::sync::Mutex

---

## OrElse

`struct` · `tracing_subscriber::fmt::writer::OrElse`

```rust
struct OrElse<A, B>
```

**Implements**: `tracing_subscriber::fmt::writer::MakeWriter`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new<'a, W>(inner: A, or_else: B) -> Self where A: MakeWriter<'a, Writer = OptionalWriter<W>>, B: MakeWriter<'a>, W: Write
```

**via `tracing_subscriber::fmt::writer::MakeWriter`**

```rust
fn make_writer(&'a self) -> Self::Writer
fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer
```

Combines a [`MakeWriter`] that returns an [`OptionalWriter`] with another
[`MakeWriter`], so that the second [`MakeWriter`] is used when the first
[`MakeWriter`] returns [`OptionalWriter::none`][own].

This is returned by the [`MakeWriterExt::or_else] method. See the
method documentation for details.

[own]: EitherWriter::none

---

## Tee

`struct` · `tracing_subscriber::fmt::writer::Tee`

```rust
struct Tee<A, B>
```

**Implements**: `std::io::Write`, `tracing_subscriber::fmt::writer::MakeWriter`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(a: A, b: B) -> Self
```

**via `std::io::Write`**

```rust
fn flush(&mut self) -> io::Result<()>
fn write(&mut self, buf: &[u8]) -> io::Result<usize>
fn write_all(&mut self, buf: &[u8]) -> io::Result<()>
fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> io::Result<()>
fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize>
```

**via `tracing_subscriber::fmt::writer::MakeWriter`**

```rust
fn make_writer(&'a self) -> Self::Writer
fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer
```

Combines two types implementing [`MakeWriter`] (or [`std::io::Write`]) to
produce a writer that writes to both [`MakeWriter`]'s returned writers.

This is returned by the [`MakeWriterExt::and`] method. See the method
documentation for details.

---

## TestWriter

`struct` · `tracing_subscriber::fmt::writer::TestWriter`

Also reachable as `tracing_subscriber::fmt::TestWriter`

```rust
struct TestWriter
```

**Implements**: `std::io::Write`, `tracing_subscriber::fmt::writer::MakeWriter`

**Derives**: Debug, Default

**Methods** (2)

```rust
fn new() -> Self
fn with_stderr() -> Self
```

**via `std::io::Write`**

```rust
fn flush(&mut self) -> io::Result<()>
fn write(&mut self, buf: &[u8]) -> io::Result<usize>
```

**via `tracing_subscriber::fmt::writer::MakeWriter`**

```rust
fn make_writer(&'a self) -> Self::Writer
```

A writer intended to support [`libtest`'s output capturing][capturing] for use in unit tests.

`TestWriter` is used by [`fmt::Subscriber`] or [`fmt::Layer`] to enable capturing support.

`cargo test` can only capture output from the standard library's [`print!`] and [`eprint!`]
macros. See [`libtest`'s output capturing][capturing] and
[rust-lang/rust#90785](https://github.com/rust-lang/rust/issues/90785) for more details about
output capturing.

Writing to [`io::stdout`] and [`io::stderr`] produces the same results as using
[`libtest`'s `--nocapture` option][nocapture] which may make the results look unreadable.

[`fmt::Subscriber`]: super::Subscriber
[`fmt::Layer`]: super::Layer
[capturing]: https://doc.rust-lang.org/book/ch11-02-running-tests.html#showing-function-output
[nocapture]: https://doc.rust-lang.org/cargo/commands/cargo-test.html
[`io::stdout`]: std::io::stdout
[`io::stderr`]: std::io::stderr
[`print!`]: std::print!

---

## WithFilter

`struct` · `tracing_subscriber::fmt::writer::WithFilter`

```rust
struct WithFilter<M, F>
```

**Implements**: `tracing_subscriber::fmt::writer::MakeWriter`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(make: M, filter: F) -> Self where F: Fn(&Metadata<'_>) -> bool
```

**via `tracing_subscriber::fmt::writer::MakeWriter`**

```rust
fn make_writer(&'a self) -> Self::Writer
fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer
```

A [`MakeWriter`] combinator that wraps a [`MakeWriter`] with a predicate for
span and event [`Metadata`], so that the [`MakeWriter::make_writer_for`]
method returns [`OptionalWriter::some`][ows] when the predicate returns `true`,
and [`OptionalWriter::none`][own] when the predicate returns `false`.

This is returned by the [`MakeWriterExt::with_filter`] method. See the
method documentation for details.

[`Metadata`]: tracing_core::Metadata
[ows]: EitherWriter::some
[own]: EitherWriter::none

---

## WithMaxLevel

`struct` · `tracing_subscriber::fmt::writer::WithMaxLevel`

```rust
struct WithMaxLevel<M>
```

**Implements**: `tracing_subscriber::fmt::writer::MakeWriter`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(make: M, level: tracing_core::Level) -> Self
```

**via `tracing_subscriber::fmt::writer::MakeWriter`**

```rust
fn make_writer(&'a self) -> Self::Writer
fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer
```

A [`MakeWriter`] combinator that only returns an enabled [writer] for spans
and events with metadata at or below a specified verbosity [`Level`].

This is returned by the [`MakeWriterExt::with_max_level`] method. See the
method documentation for details.

[writer]: std::io::Write
[`Level`]: tracing_core::Level

---

## WithMinLevel

`struct` · `tracing_subscriber::fmt::writer::WithMinLevel`

```rust
struct WithMinLevel<M>
```

**Implements**: `tracing_subscriber::fmt::writer::MakeWriter`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(make: M, level: tracing_core::Level) -> Self
```

**via `tracing_subscriber::fmt::writer::MakeWriter`**

```rust
fn make_writer(&'a self) -> Self::Writer
fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer
```

A [`MakeWriter`] combinator that only returns an enabled [writer] for spans
and events with metadata at or above a specified verbosity [`Level`].

This is returned by the [`MakeWriterExt::with_min_level`] method. See the
method documentation for details.

[writer]: std::io::Write
[`Level`]: tracing_core::Level

---

## MakeWriter

`trait` · `tracing_subscriber::fmt::writer::MakeWriter`

Also reachable as `tracing_subscriber::fmt::MakeWriter`

```rust
trait MakeWriter<'a>
```

**Implementors** (10)

- `alloc::sync::Arc`
- `std::fs::File`
- `std::sync::poison::mutex::Mutex`
- `tracing_subscriber::fmt::writer::BoxMakeWriter`
- `tracing_subscriber::fmt::writer::OrElse`
- `tracing_subscriber::fmt::writer::Tee`
- `tracing_subscriber::fmt::writer::TestWriter`
- `tracing_subscriber::fmt::writer::WithFilter`
- `tracing_subscriber::fmt::writer::WithMaxLevel`
- `tracing_subscriber::fmt::writer::WithMinLevel`

**Methods** (2)

```rust
fn make_writer(&'a self) -> Self::Writer
fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer
```

A type that can create [`io::Write`] instances.

`MakeWriter` is used by [`fmt::Layer`] or [`fmt::Subscriber`] to print
formatted text representations of [`Event`]s.

This trait is already implemented for function pointers and
immutably-borrowing closures that return an instance of [`io::Write`], such
as [`io::stdout`] and [`io::stderr`]. Additionally, it is implemented for
[`std::sync::Mutex`] when the type inside the mutex implements
[`io::Write`].

# Examples

The simplest usage is to pass in a named function that returns a writer. For
example, to log all events to stderr, we could write:
```
let subscriber = tracing_subscriber::fmt()
    .with_writer(std::io::stderr)
    .finish();
# drop(subscriber);
```

Any function that returns a writer can be used:

```
fn make_my_great_writer() -> impl std::io::Write {
    // ...
    # std::io::stdout()
}

let subscriber = tracing_subscriber::fmt()
    .with_writer(make_my_great_writer)
    .finish();
# drop(subscriber);
```

A closure can be used to introduce arbitrary logic into how the writer is
created. Consider the (admittedly rather silly) example of sending every 5th
event to stderr, and all other events to stdout:

```
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

let n = AtomicUsize::new(0);
let subscriber = tracing_subscriber::fmt()
    .with_writer(move || -> Box<dyn io::Write> {
        if n.fetch_add(1, Relaxed) % 5 == 0 {
            Box::new(io::stderr())
        } else {
            Box::new(io::stdout())
       }
    })
    .finish();
# drop(subscriber);
```

A single instance of a type implementing [`io::Write`] may be used as a
`MakeWriter` by wrapping it in a [`Mutex`]. For example, we could
write to a file like so:

```
use std::{fs::File, sync::Mutex};

# fn docs() -> Result<(), Box<dyn std::error::Error>> {
let log_file = File::create("my_cool_trace.log")?;
let subscriber = tracing_subscriber::fmt()
    .with_writer(Mutex::new(log_file))
    .finish();
# drop(subscriber);
# Ok(())
# }
```

[`io::Write`]: std::io::Write
[`fmt::Layer`]: crate::fmt::Layer
[`fmt::Subscriber`]: crate::fmt::Subscriber
[`Event`]: tracing_core::event::Event
[`io::stdout`]: std::io::stdout()
[`io::stderr`]: std::io::stderr()
[`MakeWriter::make_writer_for`]: MakeWriter::make_writer_for
[`Metadata`]: tracing_core::Metadata
[levels]: tracing_core::Level
[targets]: tracing_core::Metadata::target

---

## MakeWriterExt

`trait` · `tracing_subscriber::fmt::writer::MakeWriterExt`

Also reachable as `tracing_subscriber::prelude::_`

```rust
trait MakeWriterExt<'a>: MakeWriter<'a>
```

**Methods** (5)

```rust
fn and<B>(self, other: B) -> Tee<Self, B> where Self: Sized, B: MakeWriter<'a> + Sized
fn or_else<W, B>(self, other: B) -> OrElse<Self, B> where Self: MakeWriter<'a, Writer = OptionalWriter<W>> + Sized, B: MakeWriter<'a> + Sized, W: Write
fn with_filter<F>(self, filter: F) -> WithFilter<Self, F> where Self: Sized, F: Fn(&Metadata<'_>) -> bool
fn with_max_level(self, level: tracing_core::Level) -> WithMaxLevel<Self> where Self: Sized
fn with_min_level(self, level: tracing_core::Level) -> WithMinLevel<Self> where Self: Sized
```

Extension trait adding combinators for working with types implementing
[`MakeWriter`].

This is not intended to be implemented directly for user-defined
[`MakeWriter`]s; instead, it should be imported when the desired methods are
used.

---

## OptionalWriter

`type_alias` · `tracing_subscriber::fmt::writer::OptionalWriter`

```rust
type OptionalWriter<T> = EitherWriter<T, std::io::Sink>
```

A [writer] which may or may not be enabled.

This may be used by [`MakeWriter`] implementations that wish to
conditionally enable or disable the returned writer based on a span or
event's [`Metadata`].

[writer]: std::io::Write

---
