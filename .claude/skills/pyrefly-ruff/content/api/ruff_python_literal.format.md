# `ruff_python_literal::format`

Crate `ruff_python_literal` · 17 public items · structured records in [`model/ruff_python_literal.format.json`](../model/ruff_python_literal.format.json)

## AllowPlaceholderNesting

`enum` · `ruff_python_literal::format::AllowPlaceholderNesting`

```rust
enum AllowPlaceholderNesting
```

**Variants**: `Yes`, `No`, `AllowPlaceholderNesting`

**Derives**: Clone, Copy, Debug, Default, PartialEq, StructuralPartialEq

---

## FieldNamePart

`enum` · `ruff_python_literal::format::FieldNamePart`

```rust
enum FieldNamePart
```

**Variants**: `Attribute`, `Index`, `StringIndex`

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## FieldType

`enum` · `ruff_python_literal::format::FieldType`

```rust
enum FieldType
```

**Variants**: `Auto`, `Index`, `Keyword`

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## FormatAlign

`enum` · `ruff_python_literal::format::FormatAlign`

```rust
enum FormatAlign
```

**Variants**: `Left`, `Right`, `AfterSign`, `Center`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

---

## FormatConversion

`enum` · `ruff_python_literal::format::FormatConversion`

```rust
enum FormatConversion
```

**Variants**: `Str`, `Repr`, `Ascii`, `Bytes`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

---

## FormatGrouping

`enum` · `ruff_python_literal::format::FormatGrouping`

```rust
enum FormatGrouping
```

**Variants**: `Comma`, `Underscore`

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## FormatParseError

`enum` · `ruff_python_literal::format::FormatParseError`

```rust
enum FormatParseError
```

**Variants**: `UnmatchedBracket`, `MissingStartBracket`, `UnescapedStartBracketInLiteral`, `PlaceholderRecursionExceeded`, `UnknownConversion`, `EmptyAttribute`, `MissingRightBracket`, `InvalidCharacterAfterRightBracket`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## FormatPart

`enum` · `ruff_python_literal::format::FormatPart`

```rust
enum FormatPart
```

**Variants**: `Field`, `Literal`

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## FormatSign

`enum` · `ruff_python_literal::format::FormatSign`

```rust
enum FormatSign
```

**Variants**: `Plus`, `Minus`, `MinusOrSpace`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

---

## FormatSpec

`enum` · `ruff_python_literal::format::FormatSpec`

```rust
enum FormatSpec
```

**Variants**: `Static`, `Dynamic`

**Implements**: `core::str::traits::FromStr`

**Derives**: Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn parse(text: &str) -> Result<Self, FormatSpecError>
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

The format specification component of a format field

For example the content would be parsed from `<20` in:
```python
"hello {name:<20}".format(name="test")
```

Format specifications allow nested placeholders for dynamic formatting.
For example, the following statements are equivalent:
```python
"hello {name:{fmt}}".format(name="test", fmt="<20")
"hello {name:{align}{width}}".format(name="test", align="<", width="20")
"hello {name:<20{empty}>}".format(name="test", empty="")
```

Nested placeholders can include additional format specifiers.
```python
"hello {name:{fmt:*>}}".format(name="test", fmt="<20")
```

However, placeholders can only be singly nested (preserving our sanity).
A [`FormatSpecError::PlaceholderRecursionExceeded`] will be raised while parsing in this case.
```python
"hello {name:{fmt:{not_allowed}}}".format(name="test", fmt="<20")  # Syntax error
```

When placeholders are present in a format specification, parsing will return a [`DynamicFormatSpec`]
and avoid attempting to parse any of the clauses. Otherwise, a [`StaticFormatSpec`] will be used.

---

## FormatSpecError

`enum` · `ruff_python_literal::format::FormatSpecError`

```rust
enum FormatSpecError
```

**Variants**: `DecimalDigitsTooMany`, `PrecisionTooBig`, `InvalidFormatSpecifier`, `InvalidFormatType`, `InvalidPlaceholder`, `PlaceholderRecursionExceeded`, `UnspecifiedFormat`, `UnknownFormatCode`, `PrecisionNotAllowed`, `NotAllowed`, `UnableToConvert`, `CodeNotInRange`, `NotImplemented`

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## FormatType

`enum` · `ruff_python_literal::format::FormatType`

```rust
enum FormatType
```

**Variants**: `String`, `Binary`, `Character`, `Decimal`, `Octal`, `Number`, `Hex`, `Exponent`, `GeneralFormat`, `FixedPoint`, `Percentage`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

---

## DynamicFormatSpec

`struct` · `ruff_python_literal::format::DynamicFormatSpec`

```rust
struct DynamicFormatSpec
```

**Fields**: `placeholders`

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## FieldName

`struct` · `ruff_python_literal::format::FieldName`

```rust
struct FieldName
```

**Fields**: `field_type`, `parts`

**Derives**: Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn parse(text: &str) -> Result<FieldName, FormatParseError>
```

---

## FormatString

`struct` · `ruff_python_literal::format::FormatString`

```rust
struct FormatString
```

**Fields**: `format_parts`

**Implements**: `ruff_python_literal::format::FromTemplate`

**Derives**: Debug, PartialEq, StructuralPartialEq

**via `ruff_python_literal::format::FromTemplate`**

```rust
fn from_raw_str(text: &'a str) -> Result<Self, Self::Err>
fn from_str(text: &'a str) -> Result<Self, Self::Err>
```

---

## StaticFormatSpec

`struct` · `ruff_python_literal::format::StaticFormatSpec`

```rust
struct StaticFormatSpec
```

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## FromTemplate

`trait` · `ruff_python_literal::format::FromTemplate`

```rust
trait FromTemplate<'a>: Sized
```

**Implementors** (1)

- `ruff_python_literal::format::FormatString`

**Methods** (2)

```rust
fn from_raw_str(s: &'a str) -> Result<Self, Self::Err>
fn from_str(s: &'a str) -> Result<Self, Self::Err>
```

---
