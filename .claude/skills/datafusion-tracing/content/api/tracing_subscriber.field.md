# `tracing_subscriber::field`

Crate `tracing-subscriber` · 7 public items · structured records in [`model/tracing_subscriber.field.json`](../model/tracing_subscriber.field.json)

## MakeExt

`trait` · `tracing_subscriber::field::MakeExt`

Also reachable as `tracing_subscriber::prelude::__tracing_subscriber_field_MakeExt`

```rust
trait MakeExt<T> where Self: MakeVisitor<T> + Sized + sealed::Sealed<MakeExtMarker<T>>
```

**Methods** (3)

```rust
fn debug_alt(self) -> debug::Alt<Self>
fn delimited<D>(self, delimiter: D) -> delimited::Delimited<D, Self> where D: AsRef<str> + Clone, Self::Visitor: VisitFmt
fn display_messages(self) -> display::Messages<Self>
```

Extension trait providing `MakeVisitor` combinators.

---

## MakeOutput

`trait` · `tracing_subscriber::field::MakeOutput`

```rust
trait MakeOutput<T, Out> where Self: MakeVisitor<T> + sealed::Sealed<(T, Out)>, Self::Visitor: VisitOutput<Out>
```

**Methods** (1)

```rust
fn visit_with<F>(&self, target: T, fields: &F) -> Out where F: RecordFields
```

Extension trait implemented for all `MakeVisitor` implementations that
produce a visitor implementing `VisitOutput`.

---

## MakeVisitor

`trait` · `tracing_subscriber::field::MakeVisitor`

```rust
trait MakeVisitor<T>
```

**Implementors** (6)

- `tracing_subscriber::field::debug::Alt`
- `tracing_subscriber::field::delimited::Delimited`
- `tracing_subscriber::field::display::Messages`
- `tracing_subscriber::fmt::format::DefaultFields`
- `tracing_subscriber::fmt::format::FieldFn`
- `tracing_subscriber::fmt::format::pretty::PrettyFields`

**Methods** (1)

```rust
fn make_visitor(&self, target: T) -> Self::Visitor
```

Creates new [visitors].

A type implementing `MakeVisitor` represents a composable factory for types
implementing the [`Visit` trait][visitors]. The `MakeVisitor` trait defines
a single function, `make_visitor`, which takes in a `T`-typed `target` and
returns a type implementing `Visit` configured for that target. A target may
be a string, output stream, or data structure that the visitor will record
data to, configuration variables that determine the visitor's behavior, or
`()` when no input is required to produce a visitor.

[visitors]: tracing_core::field::Visit

---

## RecordFields

`trait` · `tracing_subscriber::field::RecordFields`

Also reachable as `tracing_subscriber::prelude::__tracing_subscriber_field_RecordFields`

```rust
trait RecordFields: sealed::Sealed<RecordFieldsMarker>
```

**Implementors** (3)

- `tracing_core::event::Event`
- `tracing_core::span::Attributes`
- `tracing_core::span::Record`

**Methods** (1)

```rust
fn record(&self, visitor: &mut dyn Visit)
```

Extension trait implemented by types which can be recorded by a [visitor].

This allows writing code that is generic over `tracing_core`'s
[`span::Attributes`][attr], [`span::Record`][rec], and [`Event`]
types. These types all provide inherent `record` methods that allow a
visitor to record their fields, but there is no common trait representing this.

With `RecordFields`, we can write code like this:
```
use tracing_core::field::Visit;
# use tracing_core::field::Field;
use tracing_subscriber::field::RecordFields;

struct MyVisitor {
    // ...
}
# impl MyVisitor { fn new() -> Self { Self{} } }
impl Visit for MyVisitor {
    // ...
# fn record_debug(&mut self, _: &Field, _: &dyn std::fmt::Debug) {}
}

fn record_with_my_visitor<R>(r: R)
where
    R: RecordFields,
{
    let mut visitor = MyVisitor::new();
    r.record(&mut visitor);
}
```
[visitor]: tracing_core::field::Visit
[attr]: tracing_core::span::Attributes
[rec]: tracing_core::span::Record

---

## VisitFmt

`trait` · `tracing_subscriber::field::VisitFmt`

```rust
trait VisitFmt: VisitOutput<fmt::Result>
```

**Implementors** (7)

- `tracing_subscriber::field::debug::Alt`
- `tracing_subscriber::field::delimited::VisitDelimited`
- `tracing_subscriber::field::display::Messages`
- `tracing_subscriber::fmt::format::DefaultVisitor`
- `tracing_subscriber::fmt::format::FieldFnVisitor`
- `tracing_subscriber::fmt::format::json::JsonVisitor`
- `tracing_subscriber::fmt::format::pretty::PrettyVisitor`

**Methods** (1)

```rust
fn writer(&mut self) -> &mut dyn fmt::Write
```

Extension trait implemented by visitors to indicate that they write to a
`fmt::Write` instance, and allow access to that writer.

---

## VisitOutput

`trait` · `tracing_subscriber::field::VisitOutput`

```rust
trait VisitOutput<Out>: Visit
```

**Implementors** (7)

- `tracing_subscriber::field::debug::Alt`
- `tracing_subscriber::field::delimited::VisitDelimited`
- `tracing_subscriber::field::display::Messages`
- `tracing_subscriber::fmt::format::DefaultVisitor`
- `tracing_subscriber::fmt::format::FieldFnVisitor`
- `tracing_subscriber::fmt::format::json::JsonVisitor`
- `tracing_subscriber::fmt::format::pretty::PrettyVisitor`

**Methods** (2)

```rust
fn finish(self) -> Out
fn visit<R>(self, fields: &R) -> Out where R: RecordFields, Self: Sized
```

A [visitor] that produces output once it has visited a set of fields.

[visitor]: tracing_core::field::Visit

---

## VisitWrite

`trait` · `tracing_subscriber::field::VisitWrite`

```rust
trait VisitWrite: VisitOutput<Result<(), io::Error>>
```

**Implementors** (2)

- `tracing_subscriber::field::debug::Alt`
- `tracing_subscriber::field::display::Messages`

**Methods** (1)

```rust
fn writer(&mut self) -> &mut dyn io::Write
```

Extension trait implemented by visitors to indicate that they write to an
`io::Write` instance, and allow access to that writer.

---
