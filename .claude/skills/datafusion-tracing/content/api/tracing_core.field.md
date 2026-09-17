# `tracing_core::field`

Crate `tracing-core` · 12 public items · structured records in [`model/tracing_core.field.json`](../model/tracing_core.field.json)

## debug

`function` · `tracing_core::field::debug`

Also reachable as `tracing::field::debug`

```rust
fn debug<T>(t: T) -> DebugValue<T> where T: fmt::Debug
```

Wraps a type implementing `fmt::Debug` as a `Value` that can be
recorded using its `Debug` implementation.

---

## display

`function` · `tracing_core::field::display`

Also reachable as `tracing::field::display`

```rust
fn display<T>(t: T) -> DisplayValue<T> where T: fmt::Display
```

Wraps a type implementing `fmt::Display` as a `Value` that can be
recorded using its `Display` implementation.

---

## valuable

`function` · `tracing_core::field::valuable`

Also reachable as `tracing::field::valuable`

```rust
fn valuable<T>(t: &T) -> valuable::Value<'_> where T: valuable::Valuable
```

Wraps a type implementing [`Valuable`] as a `Value` that
can be recorded using its `Valuable` implementation.

[`Valuable`]: https://docs.rs/valuable/latest/valuable/trait.Valuable.html

---

## DebugValue

`struct` · `tracing_core::field::DebugValue`

Also reachable as `tracing::field::DebugValue`

```rust
struct DebugValue<T: fmt::Debug>
```

**Implements**: `tracing_core::field::Value`

**Derives**: Clone, Debug

**via `tracing_core::field::Value`**

```rust
fn record(&self, key: &Field, visitor: &mut dyn Visit)
```

A `Value` which serializes as a string using `fmt::Debug`.

---

## DisplayValue

`struct` · `tracing_core::field::DisplayValue`

Also reachable as `tracing::field::DisplayValue`

```rust
struct DisplayValue<T: fmt::Display>
```

**Implements**: `core::fmt::Display`, `tracing_core::field::Value`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `tracing_core::field::Value`**

```rust
fn record(&self, key: &Field, visitor: &mut dyn Visit)
```

A `Value` which serializes using `fmt::Display`.

Uses `record_debug` in the `Value` implementation to
avoid an unnecessary evaluation.

---

## Empty

`struct` · `tracing_core::field::Empty`

Also reachable as `tracing::field::Empty`

```rust
struct Empty
```

**Implements**: `tracing_core::field::Value`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**via `tracing_core::field::Value`**

```rust
fn record(&self, _: &Field, _: &mut dyn Visit)
```

An empty field.

This can be used to indicate that the value of a field is not currently
present but will be recorded later.

When a field's value is `Empty`. it will not be recorded.

---

## Field

`struct` · `tracing_core::field::Field`

Also reachable as `tracing::field::Field`, `tracing_core::Field`

```rust
struct Field
```

**Implements**: `core::convert::AsRef`, `core::fmt::Display`, `tracing::field::AsField`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn callsite(&self) -> callsite::Identifier
fn index(&self) -> usize
fn name(&self) -> &'static str
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

An opaque key allowing _O_(1) access to a field in a `Span`'s key-value
data.

As keys are defined by the _metadata_ of a span, rather than by an
individual instance of a span, a key may be used to access the same field
across all instances of a given span with the same metadata. Thus, when a
subscriber observes a new span, it need only access a field by name _once_,
and use the key for that name for all other accesses.

---

## FieldSet

`struct` · `tracing_core::field::FieldSet`

Also reachable as `tracing::field::FieldSet`

```rust
struct FieldSet
```

**Implements**: `core::fmt::Display`

**Derives**: Debug, Eq, PartialEq

**Methods** (6)

```rust
fn contains(&self, field: &Field) -> bool
fn field<Q: Borrow<str> + ?Sized>(&self, name: &Q) -> Option<Field>
fn is_empty(&self) -> bool
fn iter(&self) -> Iter
fn len(&self) -> usize
const fn new(names: &'static [&'static str], callsite: callsite::Identifier) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Describes the fields present on a span.

## Equality

In well-behaved applications, two `FieldSet`s [initialized] with equal
[callsite identifiers] will have identical fields. Consequently, in release
builds, [`FieldSet::eq`] *only* checks that its arguments have equal
callsites. However, the equality of field names is checked in debug builds.

[initialized]: Self::new
[callsite identifiers]: callsite::Identifier

---

## Iter

`struct` · `tracing_core::field::Iter`

Also reachable as `tracing::field::Iter`

```rust
struct Iter
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Field>
```

An iterator over a set of fields.

---

## ValueSet

`struct` · `tracing_core::field::ValueSet`

Also reachable as `tracing::field::ValueSet`

```rust
struct ValueSet<'a>
```

**Implements**: `core::fmt::Display`

**Derives**: Debug

**Methods** (4)

```rust
fn callsite(&self) -> callsite::Identifier
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn record(&self, visitor: &mut dyn Visit)
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A set of fields and values for a span.

---

## Value

`trait` · `tracing_core::field::Value`

Also reachable as `tracing::Value`, `tracing::field::Value`

```rust
trait Value: sealed::Sealed
```

**Implementors** (21)

- `alloc::boxed::Box`
- `alloc::string::String`
- `core::fmt::Arguments`
- `core::num::nonzero::NonZeroI128`
- `core::num::nonzero::NonZeroI16`
- `core::num::nonzero::NonZeroI32`
- `core::num::nonzero::NonZeroI64`
- `core::num::nonzero::NonZeroI8`
- `core::num::nonzero::NonZeroIsize`
- `core::num::nonzero::NonZeroU128`
- `core::num::nonzero::NonZeroU16`
- `core::num::nonzero::NonZeroU32`
- `core::num::nonzero::NonZeroU64`
- `core::num::nonzero::NonZeroU8`
- `core::num::nonzero::NonZeroUsize`
- `core::num::wrapping::Wrapping`
- `core::option::Option`
- `tracing_core::field::DebugValue`
- `tracing_core::field::DisplayValue`
- `tracing_core::field::Empty`
- `valuable::value::Value`

**Methods** (1)

```rust
fn record(&self, key: &Field, visitor: &mut dyn Visit)
```

A field value of an erased type.

Implementors of `Value` may call the appropriate typed recording methods on
the [visitor] passed to their `record` method in order to indicate how
their data should be recorded.

[visitor]: Visit

---

## Visit

`trait` · `tracing_core::field::Visit`

Also reachable as `tracing::field::Visit`, `tracing_subscriber::field::Visit`

```rust
trait Visit
```

**Implementors** (9)

- `core::fmt::builders::DebugMap`
- `core::fmt::builders::DebugStruct`
- `tracing_subscriber::field::debug::Alt`
- `tracing_subscriber::field::delimited::VisitDelimited`
- `tracing_subscriber::field::display::Messages`
- `tracing_subscriber::fmt::format::DefaultVisitor`
- `tracing_subscriber::fmt::format::FieldFnVisitor`
- `tracing_subscriber::fmt::format::json::JsonVisitor`
- `tracing_subscriber::fmt::format::pretty::PrettyVisitor`

**Methods** (11)

```rust
fn record_bool(&mut self, field: &Field, value: bool)
fn record_bytes(&mut self, field: &Field, value: &[u8])
fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug)
fn record_error(&mut self, field: &Field, value: &dyn std::error::Error + 'static)
fn record_f64(&mut self, field: &Field, value: f64)
fn record_i128(&mut self, field: &Field, value: i128)
fn record_i64(&mut self, field: &Field, value: i64)
fn record_str(&mut self, field: &Field, value: &str)
fn record_u128(&mut self, field: &Field, value: u128)
fn record_u64(&mut self, field: &Field, value: u64)
fn record_value(&mut self, field: &Field, value: valuable::Value<'_>)
```

Visits typed values.

An instance of `Visit` ("a visitor") represents the logic necessary to
record field values of various types. When an implementor of [`Value`] is
[recorded], it calls the appropriate method on the provided visitor to
indicate the type that value should be recorded as.

When a [`Subscriber`] implementation [records an `Event`] or a
[set of `Value`s added to a `Span`], it can pass an `&mut Visit` to the
`record` method on the provided [`ValueSet`] or [`Event`]. This visitor
will then be used to record all the field-value pairs present on that
`Event` or `ValueSet`.

# Examples

A simple visitor that writes to a string might be implemented like so:
```
# extern crate tracing_core as tracing;
use std::fmt::{self, Write};
use tracing::field::{Value, Visit, Field};
pub struct StringVisitor<'a> {
    string: &'a mut String,
}

impl<'a> Visit for StringVisitor<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        write!(self.string, "{} = {:?}; ", field.name(), value).unwrap();
    }
}
```
This visitor will format each recorded value using `fmt::Debug`, and
append the field name and formatted value to the provided string,
regardless of the type of the recorded value. When all the values have
been recorded, the `StringVisitor` may be dropped, allowing the string
to be printed or stored in some other data structure.

The `Visit` trait provides default implementations for `record_i64`,
`record_u64`, `record_bool`, `record_str`, and `record_error`, which simply
forward the recorded value to `record_debug`. Thus, `record_debug` is the
only method which a `Visit` implementation *must* implement. However,
visitors may override the default implementations of these functions in
order to implement type-specific behavior.

Additionally, when a visitor receives a value of a type it does not care
about, it is free to ignore those values completely. For example, a
visitor which only records numeric data might look like this:

```
# extern crate tracing_core as tracing;
# use std::fmt::{self, Write};
# use tracing::field::{Value, Visit, Field};
pub struct SumVisitor {
    sum: i64,
}

impl Visit for SumVisitor {
    fn record_i64(&mut self, _field: &Field, value: i64) {
       self.sum += value;
    }

    fn record_u64(&mut self, _field: &Field, value: u64) {
        self.sum += value as i64;
    }

    fn record_debug(&mut self, _field: &Field, _value: &dyn fmt::Debug) {
        // Do nothing
    }
}
```

This visitor (which is probably not particularly useful) keeps a running
sum of all the numeric values it records, and ignores all other values. A
more practical example of recording typed values is presented in
`examples/counters.rs`, which demonstrates a very simple metrics system
implemented using `tracing`.

<div class="example-wrap" style="display:inline-block">
<pre class="ignore" style="white-space:normal;font:inherit;">
<strong>Note</strong>: The <code>record_error</code> trait method is only
available when the Rust standard library is present, as it requires the
<code>std::error::Error</code> trait.
</pre></div>

[recorded]: Value::record
[`Subscriber`]: super::subscriber::Subscriber
[records an `Event`]: super::subscriber::Subscriber::event
[set of `Value`s added to a `Span`]: super::subscriber::Subscriber::record
[`Event`]: super::event::Event

---
