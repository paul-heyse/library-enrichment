# `sqlparser::ast::value`

Crate `sqlparser` · 10 public items · structured records in [`model/sqlparser.ast.value.json`](../model/sqlparser.ast.value.json)

## DateTimeField

`enum` · `sqlparser::ast::value::DateTimeField`

Also reachable as `sqlparser::ast::DateTimeField`

```rust
enum DateTimeField
```

**Variants**: `Year`, `Years`, `Month`, `Months`, `Week`, `Weeks`, `Day`, `DayOfWeek`, `DayOfYear`, `Days`, `Date`, `Datetime`, `Hour`, `Hours`, `Minute`, `Minutes`, `Second`, `Seconds`, `Century`, `Decade`, `Dow`, `Doy`, `Epoch`, `Isodow`, `Isoyear`, `IsoWeek`, `Julian`, `Microsecond`, `Microseconds`, `Millenium`, `Millennium`, `Millisecond`, `Milliseconds`, `Nanosecond`, `Nanoseconds`, `Quarter`, `Time`, `Timezone`, `TimezoneAbbr`, `TimezoneHour`, `TimezoneMinute`, `TimezoneRegion`, `NoDateTime`, `Custom`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Represents the date/time fields used by functions like `EXTRACT`.

Each variant corresponds to a supported date/time part (for example
`YEAR`, `MONTH`, `DAY`, etc.). The `Custom` variant allows arbitrary
identifiers (e.g. dialect-specific abbreviations).

---

## NormalizationForm

`enum` · `sqlparser::ast::value::NormalizationForm`

Also reachable as `sqlparser::ast::NormalizationForm`

```rust
enum NormalizationForm
```

**Variants**: `NFC`, `NFD`, `NFKC`, `NFKD`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

The Unicode Standard defines four normalization forms, which are intended to eliminate
certain distinctions between visually or functionally identical characters.

See [Unicode Normalization Forms](https://unicode.org/reports/tr15/) for details.

---

## TrimWhereField

`enum` · `sqlparser::ast::value::TrimWhereField`

Also reachable as `sqlparser::ast::TrimWhereField`

```rust
enum TrimWhereField
```

**Variants**: `Both`, `Leading`, `Trailing`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

The side on which `TRIM` should be applied.

Corresponds to `TRIM(BOTH|LEADING|TRAILING)` SQL syntax.

---

## Value

`enum` · `sqlparser::ast::value::Value`

Also reachable as `sqlparser::ast::Value`

```rust
enum Value
```

**Variants**: `Number`, `SingleQuotedString`, `DollarQuotedString`, `TripleSingleQuotedString`, `TripleDoubleQuotedString`, `EscapedStringLiteral`, `UnicodeStringLiteral`, `SingleQuotedByteStringLiteral`, `DoubleQuotedByteStringLiteral`, `TripleSingleQuotedByteStringLiteral`, `TripleDoubleQuotedByteStringLiteral`, `SingleQuotedRawStringLiteral`, `DoubleQuotedRawStringLiteral`, `TripleSingleQuotedRawStringLiteral`, `TripleDoubleQuotedRawStringLiteral`, `NationalStringLiteral`, `QuoteDelimitedStringLiteral`, `NationalQuoteDelimitedStringLiteral`, `HexStringLiteral`, `DoubleQuotedString`, `Boolean`, `Null`, `Placeholder`

**Implements**: `core::convert::From`, `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn into_string(self) -> Option<String>
fn with_empty_span(self) -> ValueWithSpan
fn with_span(self, span: Span) -> ValueWithSpan
```

**via `core::convert::From`**

```rust
fn from(value: ValueWithSpan) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Primitive SQL values such as number and string

---

## escape_double_quote_string

`function` · `sqlparser::ast::value::escape_double_quote_string`

Also reachable as `sqlparser::ast::escape_double_quote_string`

```rust
fn escape_double_quote_string(s: &str) -> EscapeQuotedString<'_>
```

Convenience wrapper for escaping strings for double-quoted literals (`").`

---

## escape_quoted_string

`function` · `sqlparser::ast::value::escape_quoted_string`

Also reachable as `sqlparser::ast::escape_quoted_string`

```rust
fn escape_quoted_string(string: &str, quote: char) -> EscapeQuotedString<'_>
```

Return a helper which formats `string` for inclusion inside a quoted
literal that uses `quote` as the delimiter.

---

## DollarQuotedString

`struct` · `sqlparser::ast::value::DollarQuotedString`

Also reachable as `sqlparser::ast::DollarQuotedString`

```rust
struct DollarQuotedString
```

**Fields**: `value`, `tag`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

A dollar-quoted string literal, e.g. `$$...$$` or `$tag$...$tag$`.

---

## EscapeQuotedString

`struct` · `sqlparser::ast::value::EscapeQuotedString`

```rust
struct EscapeQuotedString<'a>
```

---

## QuoteDelimitedString

`struct` · `sqlparser::ast::value::QuoteDelimitedString`

Also reachable as `sqlparser::ast::QuoteDelimitedString`

```rust
struct QuoteDelimitedString
```

**Fields**: `start_quote`, `value`, `end_quote`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

A quote delimited string literal, e.g. `Q'_abc_'`.

See [Value::QuoteDelimitedStringLiteral] and/or
[Value::NationalQuoteDelimitedStringLiteral].

---

## ValueWithSpan

`struct` · `sqlparser::ast::value::ValueWithSpan`

Also reachable as `sqlparser::ast::ValueWithSpan`

```rust
struct ValueWithSpan
```

**Fields**: `value`, `span`

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::ops::deref::Deref`, `core::ops::deref::DerefMut`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::spans::Spanned`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (1)

```rust
fn into_string(self) -> Option<String>
```

**via `core::convert::From`**

```rust
fn from(value: Value) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::spans::Spanned`**

```rust
fn span(&self) -> Span
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Wraps a primitive SQL [`Value`]  with its [`Span`] location

# Example: create a `ValueWithSpan` from a `Value`
```
# use sqlparser::ast::{Value, ValueWithSpan};
# use sqlparser::tokenizer::{Location, Span};
let value = Value::SingleQuotedString(String::from("endpoint"));
// from line 1, column 1 to line 1, column 7
let span = Span::new(Location::new(1, 1), Location::new(1, 7));
let value_with_span = value.with_span(span);
```

# Example: create a `ValueWithSpan` from a `Value` with an empty span

You can call [`Value::with_empty_span`] to create a `ValueWithSpan` with an empty span
```
# use sqlparser::ast::{Value, ValueWithSpan};
# use sqlparser::tokenizer::{Location, Span};
let value = Value::SingleQuotedString(String::from("endpoint"));
let value_with_span = value.with_empty_span();
assert_eq!(value_with_span.span, Span::empty());
```

You can also use the [`From`] trait to convert  `ValueWithSpan` to/from `Value`s
```
# use sqlparser::ast::{Value, ValueWithSpan};
# use sqlparser::tokenizer::{Location, Span};
let value = Value::SingleQuotedString(String::from("endpoint"));
// converting `Value` to `ValueWithSpan` results in an empty span
let value_with_span: ValueWithSpan = value.into();
assert_eq!(value_with_span.span, Span::empty());
// convert back to `Value`
let value: Value = value_with_span.into();
```
A `Value` paired with its source `Span` location.

---
