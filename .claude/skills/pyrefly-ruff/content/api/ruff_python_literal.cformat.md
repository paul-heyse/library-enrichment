# `ruff_python_literal::cformat`

Crate `ruff_python_literal` · 17 public items · structured records in [`model/ruff_python_literal.cformat.json`](../model/ruff_python_literal.cformat.json)

## CFloatType

`enum` · `ruff_python_literal::cformat::CFloatType`

```rust
enum CFloatType
```

**Variants**: `Exponent`, `PointDecimal`, `General`

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## CFormatContext

`enum` · `ruff_python_literal::cformat::CFormatContext`

```rust
enum CFormatContext
```

**Variants**: `Str`, `Bytes`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

---

## CFormatErrorType

`enum` · `ruff_python_literal::cformat::CFormatErrorType`

```rust
enum CFormatErrorType
```

**Variants**: `UnmatchedKeyParentheses`, `MissingModuloSign`, `UnsupportedFormatChar`, `IncompleteFormat`, `IntTooBig`

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## CFormatPart

`enum` · `ruff_python_literal::cformat::CFormatPart`

```rust
enum CFormatPart<T>
```

**Variants**: `Literal`, `Spec`

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## CFormatPrecision

`enum` · `ruff_python_literal::cformat::CFormatPrecision`

```rust
enum CFormatPrecision
```

**Variants**: `Quantity`, `Dot`

**Implements**: `core::convert::From`

**Derives**: Debug, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(quantity: CFormatQuantity) -> Self
```

---

## CFormatQuantity

`enum` · `ruff_python_literal::cformat::CFormatQuantity`

```rust
enum CFormatQuantity
```

**Variants**: `Amount`, `FromValuesTuple`

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## CFormatType

`enum` · `ruff_python_literal::cformat::CFormatType`

```rust
enum CFormatType
```

**Variants**: `Number`, `Float`, `Character`, `String`

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## CNumberType

`enum` · `ruff_python_literal::cformat::CNumberType`

```rust
enum CNumberType
```

**Variants**: `Decimal`, `Octal`, `Hex`

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## CConversionFlags

`struct` · `ruff_python_literal::cformat::CConversionFlags`

```rust
struct CConversionFlags
```

**Implements**: `bitflags::traits::Flags`, `bitflags::traits::PublicFlags`, `core::fmt::Binary`, `core::fmt::LowerHex`, `core::fmt::Octal`, `core::fmt::UpperHex`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `core::ops::bit::BitAnd`, `core::ops::bit::BitAndAssign`, `core::ops::bit::BitOr`, `core::ops::bit::BitOrAssign`, `core::ops::bit::BitXor`, `core::ops::bit::BitXorAssign`, `core::ops::bit::Not`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (22)

```rust
const fn all() -> Self
const fn bits(&self) -> u32
const fn complement(self) -> Self
const fn contains(&self, other: Self) -> bool
const fn difference(self, other: Self) -> Self
const fn empty() -> Self
const fn from_bits(bits: u32) -> __private::core::option::Option<Self>
const fn from_bits_retain(bits: u32) -> Self
const fn from_bits_truncate(bits: u32) -> Self
fn from_name(name: &str) -> __private::core::option::Option<Self>
fn insert(&mut self, other: Self)
const fn intersection(self, other: Self) -> Self
const fn intersects(&self, other: Self) -> bool
const fn is_all(&self) -> bool
const fn is_empty(&self) -> bool
const fn iter(&self) -> iter::Iter<CConversionFlags>
const fn iter_names(&self) -> iter::IterNames<CConversionFlags>
fn remove(&mut self, other: Self)
fn set(&mut self, other: Self, value: bool)
const fn symmetric_difference(self, other: Self) -> Self
fn toggle(&mut self, other: Self)
const fn union(self, other: Self) -> Self
```

**via `bitflags::traits::Flags`**

```rust
fn all_named() -> CConversionFlags
fn bits(&self) -> u32
fn from_bits_retain(bits: u32) -> CConversionFlags
```

**via `core::fmt::Binary`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::LowerHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::Octal`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::UpperHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, other: Self) -> Self
```

**via `core::ops::arith::SubAssign`**

```rust
fn sub_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitAnd`**

```rust
fn bitand(self, other: Self) -> Self
```

**via `core::ops::bit::BitAndAssign`**

```rust
fn bitand_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, other: CConversionFlags) -> Self
```

**via `core::ops::bit::BitOrAssign`**

```rust
fn bitor_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitXor`**

```rust
fn bitxor(self, other: Self) -> Self
```

**via `core::ops::bit::BitXorAssign`**

```rust
fn bitxor_assign(&mut self, other: Self)
```

**via `core::ops::bit::Not`**

```rust
fn not(self) -> Self
```

---

## CFormatError

`struct` · `ruff_python_literal::cformat::CFormatError`

```rust
struct CFormatError
```

**Fields**: `typ`, `index`

**Implements**: `core::fmt::Display`

**Derives**: Debug, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## CFormatSpec

`struct` · `ruff_python_literal::cformat::CFormatSpec`

```rust
struct CFormatSpec
```

**Fields**: `mapping_key`, `flags`, `min_field_width`, `precision`, `format_type`, `format_char`

**Implements**: `core::str::traits::FromStr`

**Derives**: Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn parse<T, I>(iter: &mut ParseIter<I>, context: CFormatContext) -> Result<Self, ParsingError> where T: Into<char> + Copy, I: Iterator<Item = T>
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(text: &str) -> Result<Self, Self::Err>
```

---

## CFormatStrOrBytes

`struct` · `ruff_python_literal::cformat::CFormatStrOrBytes`

```rust
struct CFormatStrOrBytes<S>
```

**Derives**: Debug, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn iter(&self) -> impl Iterator<Item = &(usize, CFormatPart<S>)>
fn iter_mut(&mut self) -> impl Iterator<Item = &mut (usize, CFormatPart<S>)>
fn parse<I: Iterator<Item = u8>>(iter: &mut ParseIter<I>) -> Result<Self, CFormatError>
fn parse<I: Iterator<Item = char>>(iter: &mut ParseIter<I>) -> Result<Self, CFormatError>
```

---

## CFormatBytes

`type_alias` · `ruff_python_literal::cformat::CFormatBytes`

```rust
type CFormatBytes = CFormatStrOrBytes<Vec<u8>>
```

---

## CFormatConversion

`type_alias` · `ruff_python_literal::cformat::CFormatConversion`

```rust
type CFormatConversion = super::format::FormatConversion
```

---

## CFormatString

`type_alias` · `ruff_python_literal::cformat::CFormatString`

```rust
type CFormatString = CFormatStrOrBytes<String>
```

---

## ParseIter

`type_alias` · `ruff_python_literal::cformat::ParseIter`

```rust
type ParseIter<I> = std::iter::Peekable<std::iter::Enumerate<I>>
```

---

## ParsingError

`type_alias` · `ruff_python_literal::cformat::ParsingError`

```rust
type ParsingError = (CFormatErrorType, usize)
```

---
