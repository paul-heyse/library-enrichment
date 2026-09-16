# `ruff_python_formatter::options`

Crate `ruff_python_formatter` · 7 public items · structured records in [`model/ruff_python_formatter.options.json`](../model/ruff_python_formatter.options.json)

## DocstringCode

`enum` · `ruff_python_formatter::options::DocstringCode`

Also reachable as `ruff_python_formatter::DocstringCode`

```rust
enum DocstringCode
```

**Variants**: `Disabled`, `Enabled`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## DocstringCodeLineWidth

`enum` · `ruff_python_formatter::options::DocstringCodeLineWidth`

Also reachable as `ruff_python_formatter::DocstringCodeLineWidth`

```rust
enum DocstringCodeLineWidth
```

**Variants**: `Fixed`, `Dynamic`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## MagicTrailingComma

`enum` · `ruff_python_formatter::options::MagicTrailingComma`

Also reachable as `ruff_python_formatter::MagicTrailingComma`

```rust
enum MagicTrailingComma
```

**Variants**: `Respect`, `Ignore`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default

**Methods** (1)

```rust
const fn is_ignore(self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## NestedStringQuoteStyle

`enum` · `ruff_python_formatter::options::NestedStringQuoteStyle`

Also reachable as `ruff_python_formatter::NestedStringQuoteStyle`

```rust
enum NestedStringQuoteStyle
```

**Variants**: `Alternating`, `Preferred`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## PreviewMode

`enum` · `ruff_python_formatter::options::PreviewMode`

Also reachable as `ruff_python_formatter::PreviewMode`

```rust
enum PreviewMode
```

**Variants**: `Disabled`, `Enabled`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn is_enabled(self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## QuoteStyle

`enum` · `ruff_python_formatter::options::QuoteStyle`

Also reachable as `ruff_python_formatter::QuoteStyle`

```rust
enum QuoteStyle
```

**Variants**: `Single`, `Double`, `Preserve`

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn as_str(&self) -> &'static str
```

**via `core::convert::From`**

```rust
fn from(value: Quote) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## PyFormatOptions

`struct` · `ruff_python_formatter::options::PyFormatOptions`

Also reachable as `ruff_python_formatter::PyFormatOptions`

```rust
struct PyFormatOptions
```

**Implements**: `ruff_formatter::FormatOptions`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default

**Methods** (23)

```rust
const fn docstring_code(&self) -> DocstringCode
const fn docstring_code_line_width(&self) -> DocstringCodeLineWidth
fn from_extension(path: &Path) -> Self
fn from_source_type(source_type: PySourceType) -> Self
const fn line_ending(&self) -> LineEnding
const fn magic_trailing_comma(&self) -> MagicTrailingComma
const fn nested_string_quote_style(&self) -> NestedStringQuoteStyle
const fn preview(&self) -> PreviewMode
const fn quote_style(&self) -> QuoteStyle
const fn source_type(&self) -> PySourceType
const fn target_version(&self) -> ast::PythonVersion
fn with_docstring_code(self, docstring_code: DocstringCode) -> Self
fn with_docstring_code_line_width(self, line_width: DocstringCodeLineWidth) -> Self
fn with_indent_style(self, indent_style: IndentStyle) -> Self
fn with_indent_width(self, indent_width: IndentWidth) -> Self
fn with_line_ending(self, line_ending: LineEnding) -> Self
fn with_line_width(self, line_width: LineWidth) -> Self
fn with_magic_trailing_comma(self, trailing_comma: MagicTrailingComma) -> Self
fn with_nested_string_quote_style(self, nested_string_quote_style: NestedStringQuoteStyle) -> Self
fn with_preview(self, preview: PreviewMode) -> Self
fn with_quote_style(self, style: QuoteStyle) -> Self
fn with_source_map_generation(self, source_map: SourceMapGeneration) -> Self
fn with_target_version(self, target_version: ast::PythonVersion) -> Self
```

**via `ruff_formatter::FormatOptions`**

```rust
fn as_print_options(&self) -> PrinterOptions
fn indent_style(&self) -> IndentStyle
fn indent_width(&self) -> IndentWidth
fn line_width(&self) -> LineWidth
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Resolved options for formatting one individual file. The difference to `FormatterSettings`
is that `FormatterSettings` stores the settings for multiple files (the entire project, a subdirectory, ..)

---
