# `ruff_python_ast::str_prefix`

Crate `ruff_python_ast` · 5 public items · structured records in [`model/ruff_python_ast.str_prefix.json`](../model/ruff_python_ast.str_prefix.json)

## AnyStringPrefix

`enum` · `ruff_python_ast::str_prefix::AnyStringPrefix`

```rust
enum AnyStringPrefix
```

**Variants**: `Bytes`, `Format`, `Template`, `Regular`

**Implements**: `core::fmt::Display`, `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (23)

```rust
fn as_bytes(&self) -> Option<&ByteStringPrefix>
fn as_format(&self) -> Option<&FStringPrefix>
fn as_mut_bytes(&mut self) -> Option<&mut ByteStringPrefix>
fn as_mut_format(&mut self) -> Option<&mut FStringPrefix>
fn as_mut_regular(&mut self) -> Option<&mut StringLiteralPrefix>
fn as_mut_template(&mut self) -> Option<&mut TStringPrefix>
fn as_regular(&self) -> Option<&StringLiteralPrefix>
const fn as_str(self) -> &'static str
fn as_template(&self) -> Option<&TStringPrefix>
fn bytes(self) -> Option<ByteStringPrefix>
fn expect_bytes(self) -> ByteStringPrefix where Self: ::std::fmt::Debug
fn expect_format(self) -> FStringPrefix where Self: ::std::fmt::Debug
fn expect_regular(self) -> StringLiteralPrefix where Self: ::std::fmt::Debug
fn expect_template(self) -> TStringPrefix where Self: ::std::fmt::Debug
fn format(self) -> Option<FStringPrefix>
const fn is_bytes(&self) -> bool
const fn is_format(&self) -> bool
const fn is_raw(self) -> bool
const fn is_regular(&self) -> bool
const fn is_template(&self) -> bool
fn regular(self) -> Option<StringLiteralPrefix>
fn template(self) -> Option<TStringPrefix>
const fn text_len(self) -> TextSize
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Enumeration of all the possible valid prefixes
prior to a Python string literal.

Using the `as_flags()` method on variants of this enum
is the recommended way to set `*_PREFIX` flags from the
`StringFlags` bitflag, as it means that you cannot accidentally
set a combination of `*_PREFIX` flags that would be invalid
at runtime in Python.

[String and Bytes literals]: https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
[PEP 701]: https://peps.python.org/pep-0701/

---

## ByteStringPrefix

`enum` · `ruff_python_ast::str_prefix::ByteStringPrefix`

```rust
enum ByteStringPrefix
```

**Variants**: `Regular`, `Raw`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn as_str(self) -> &'static str
const fn is_raw(self) -> bool
const fn text_len(self) -> TextSize
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Enumeration of the valid prefixes a bytestring literal can have.

---

## FStringPrefix

`enum` · `ruff_python_ast::str_prefix::FStringPrefix`

```rust
enum FStringPrefix
```

**Variants**: `Regular`, `Raw`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn as_str(self) -> &'static str
const fn is_raw(self) -> bool
const fn text_len(self) -> TextSize
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Enumeration of the valid prefixes an f-string literal can have.

---

## StringLiteralPrefix

`enum` · `ruff_python_ast::str_prefix::StringLiteralPrefix`

```rust
enum StringLiteralPrefix
```

**Variants**: `Empty`, `Unicode`, `Raw`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
const fn as_str(self) -> &'static str
const fn is_empty(&self) -> bool
const fn is_raw(&self) -> bool
const fn is_unicode(&self) -> bool
const fn text_len(self) -> TextSize
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Enumerations of the valid prefixes a string literal can have.

Bytestrings and f-strings are excluded from this enumeration,
as they are represented by different AST nodes.

---

## TStringPrefix

`enum` · `ruff_python_ast::str_prefix::TStringPrefix`

```rust
enum TStringPrefix
```

**Variants**: `Regular`, `Raw`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn as_str(self) -> &'static str
const fn is_raw(self) -> bool
const fn text_len(self) -> TextSize
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Enumeration of the valid prefixes a t-string literal can have.

---
