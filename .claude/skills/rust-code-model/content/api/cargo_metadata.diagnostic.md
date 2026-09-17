# `cargo_metadata::diagnostic`

Crate `cargo_metadata` · 17 public items · structured records in [`model/cargo_metadata.diagnostic.json`](../model/cargo_metadata.diagnostic.json)

## Applicability

`enum` · `cargo_metadata::diagnostic::Applicability`

```rust
enum Applicability
```

**Variants**: `MachineApplicable`, `HasPlaceholders`, `MaybeIncorrect`, `Unspecified`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Whether a suggestion can be safely applied.

---

## DiagnosticBuilderError

`enum` · `cargo_metadata::diagnostic::DiagnosticBuilderError`

```rust
enum DiagnosticBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(s: ::derive_builder::export::core::string::String) -> Self
fn from(s: ::derive_builder::UninitializedFieldError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::derive_builder::export::core::fmt::Formatter<'_>) -> ::derive_builder::export::core::fmt::Result
```

Error type for DiagnosticBuilder

---

## DiagnosticCodeBuilderError

`enum` · `cargo_metadata::diagnostic::DiagnosticCodeBuilderError`

```rust
enum DiagnosticCodeBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(s: ::derive_builder::export::core::string::String) -> Self
fn from(s: ::derive_builder::UninitializedFieldError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::derive_builder::export::core::fmt::Formatter<'_>) -> ::derive_builder::export::core::fmt::Result
```

Error type for DiagnosticCodeBuilder

---

## DiagnosticLevel

`enum` · `cargo_metadata::diagnostic::DiagnosticLevel`

```rust
enum DiagnosticLevel
```

**Variants**: `Ice`, `Error`, `Warning`, `FailureNote`, `Note`, `Help`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The diagnostic level

---

## DiagnosticSpanBuilderError

`enum` · `cargo_metadata::diagnostic::DiagnosticSpanBuilderError`

```rust
enum DiagnosticSpanBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(s: ::derive_builder::export::core::string::String) -> Self
fn from(s: ::derive_builder::UninitializedFieldError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::derive_builder::export::core::fmt::Formatter<'_>) -> ::derive_builder::export::core::fmt::Result
```

Error type for DiagnosticSpanBuilder

---

## DiagnosticSpanLineBuilderError

`enum` · `cargo_metadata::diagnostic::DiagnosticSpanLineBuilderError`

```rust
enum DiagnosticSpanLineBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(s: ::derive_builder::export::core::string::String) -> Self
fn from(s: ::derive_builder::UninitializedFieldError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::derive_builder::export::core::fmt::Formatter<'_>) -> ::derive_builder::export::core::fmt::Result
```

Error type for DiagnosticSpanLineBuilder

---

## DiagnosticSpanMacroExpansionBuilderError

`enum` · `cargo_metadata::diagnostic::DiagnosticSpanMacroExpansionBuilderError`

```rust
enum DiagnosticSpanMacroExpansionBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(s: ::derive_builder::UninitializedFieldError) -> Self
fn from(s: ::derive_builder::export::core::string::String) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::derive_builder::export::core::fmt::Formatter<'_>) -> ::derive_builder::export::core::fmt::Result
```

Error type for DiagnosticSpanMacroExpansionBuilder

---

## Diagnostic

`struct` · `cargo_metadata::diagnostic::Diagnostic`

```rust
struct Diagnostic
```

**Fields**: `message`, `code`, `level`, `spans`, `children`, `rendered`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

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

A diagnostic message generated by rustc

---

## DiagnosticBuilder

`struct` · `cargo_metadata::diagnostic::DiagnosticBuilder`

```rust
struct DiagnosticBuilder
```

**Derives**: Default

**Methods** (7)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<Diagnostic, DiagnosticBuilderError>
fn children<VALUE: ::derive_builder::export::core::convert::Into<Vec<Diagnostic>>>(self, value: VALUE) -> Self
fn code<VALUE: ::derive_builder::export::core::convert::Into<Option<DiagnosticCode>>>(self, value: VALUE) -> Self
fn level<VALUE: ::derive_builder::export::core::convert::Into<DiagnosticLevel>>(self, value: VALUE) -> Self
fn message<VALUE: ::derive_builder::export::core::convert::Into<String>>(self, value: VALUE) -> Self
fn rendered<VALUE: ::derive_builder::export::core::convert::Into<Option<String>>>(self, value: VALUE) -> Self
fn spans<VALUE: ::derive_builder::export::core::convert::Into<Vec<DiagnosticSpan>>>(self, value: VALUE) -> Self
```

Builder for [`Diagnostic`](struct.Diagnostic.html).

---

## DiagnosticCode

`struct` · `cargo_metadata::diagnostic::DiagnosticCode`

```rust
struct DiagnosticCode
```

**Fields**: `code`, `explanation`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The error code associated to this diagnostic.

---

## DiagnosticCodeBuilder

`struct` · `cargo_metadata::diagnostic::DiagnosticCodeBuilder`

```rust
struct DiagnosticCodeBuilder
```

**Derives**: Default

**Methods** (3)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<DiagnosticCode, DiagnosticCodeBuilderError>
fn code<VALUE: ::derive_builder::export::core::convert::Into<String>>(self, value: VALUE) -> Self
fn explanation<VALUE: ::derive_builder::export::core::convert::Into<Option<String>>>(self, value: VALUE) -> Self
```

Builder for [`DiagnosticCode`](struct.DiagnosticCode.html).

---

## DiagnosticSpan

`struct` · `cargo_metadata::diagnostic::DiagnosticSpan`

```rust
struct DiagnosticSpan
```

**Fields**: `file_name`, `byte_start`, `byte_end`, `line_start`, `line_end`, `column_start`, `column_end`, `is_primary`, `text`, `label`, `suggested_replacement`, `suggestion_applicability`, `expansion`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A section of the source code associated with a Diagnostic

---

## DiagnosticSpanBuilder

`struct` · `cargo_metadata::diagnostic::DiagnosticSpanBuilder`

```rust
struct DiagnosticSpanBuilder
```

**Derives**: Default

**Methods** (14)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<DiagnosticSpan, DiagnosticSpanBuilderError>
fn byte_end<VALUE: ::derive_builder::export::core::convert::Into<u32>>(self, value: VALUE) -> Self
fn byte_start<VALUE: ::derive_builder::export::core::convert::Into<u32>>(self, value: VALUE) -> Self
fn column_end<VALUE: ::derive_builder::export::core::convert::Into<usize>>(self, value: VALUE) -> Self
fn column_start<VALUE: ::derive_builder::export::core::convert::Into<usize>>(self, value: VALUE) -> Self
fn expansion<VALUE: ::derive_builder::export::core::convert::Into<Option<Box<DiagnosticSpanMacroExpansion>>>>(self, value: VALUE) -> Self
fn file_name<VALUE: ::derive_builder::export::core::convert::Into<String>>(self, value: VALUE) -> Self
fn is_primary<VALUE: ::derive_builder::export::core::convert::Into<bool>>(self, value: VALUE) -> Self
fn label<VALUE: ::derive_builder::export::core::convert::Into<Option<String>>>(self, value: VALUE) -> Self
fn line_end<VALUE: ::derive_builder::export::core::convert::Into<usize>>(self, value: VALUE) -> Self
fn line_start<VALUE: ::derive_builder::export::core::convert::Into<usize>>(self, value: VALUE) -> Self
fn suggested_replacement<VALUE: ::derive_builder::export::core::convert::Into<Option<String>>>(self, value: VALUE) -> Self
fn suggestion_applicability<VALUE: ::derive_builder::export::core::convert::Into<Option<Applicability>>>(self, value: VALUE) -> Self
fn text<VALUE: ::derive_builder::export::core::convert::Into<Vec<DiagnosticSpanLine>>>(self, value: VALUE) -> Self
```

Builder for [`DiagnosticSpan`](struct.DiagnosticSpan.html).

---

## DiagnosticSpanLine

`struct` · `cargo_metadata::diagnostic::DiagnosticSpanLine`

```rust
struct DiagnosticSpanLine
```

**Fields**: `text`, `highlight_start`, `highlight_end`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A line of code associated with the Diagnostic

---

## DiagnosticSpanLineBuilder

`struct` · `cargo_metadata::diagnostic::DiagnosticSpanLineBuilder`

```rust
struct DiagnosticSpanLineBuilder
```

**Derives**: Default

**Methods** (4)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<DiagnosticSpanLine, DiagnosticSpanLineBuilderError>
fn highlight_end<VALUE: ::derive_builder::export::core::convert::Into<usize>>(self, value: VALUE) -> Self
fn highlight_start<VALUE: ::derive_builder::export::core::convert::Into<usize>>(self, value: VALUE) -> Self
fn text<VALUE: ::derive_builder::export::core::convert::Into<String>>(self, value: VALUE) -> Self
```

Builder for [`DiagnosticSpanLine`](struct.DiagnosticSpanLine.html).

---

## DiagnosticSpanMacroExpansion

`struct` · `cargo_metadata::diagnostic::DiagnosticSpanMacroExpansion`

```rust
struct DiagnosticSpanMacroExpansion
```

**Fields**: `span`, `macro_decl_name`, `def_site_span`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Macro expansion information associated with a diagnostic.

---

## DiagnosticSpanMacroExpansionBuilder

`struct` · `cargo_metadata::diagnostic::DiagnosticSpanMacroExpansionBuilder`

```rust
struct DiagnosticSpanMacroExpansionBuilder
```

**Derives**: Default

**Methods** (4)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<DiagnosticSpanMacroExpansion, DiagnosticSpanMacroExpansionBuilderError>
fn def_site_span<VALUE: ::derive_builder::export::core::convert::Into<Option<DiagnosticSpan>>>(self, value: VALUE) -> Self
fn macro_decl_name<VALUE: ::derive_builder::export::core::convert::Into<String>>(self, value: VALUE) -> Self
fn span<VALUE: ::derive_builder::export::core::convert::Into<DiagnosticSpan>>(self, value: VALUE) -> Self
```

Builder for [`DiagnosticSpanMacroExpansion`](struct.DiagnosticSpanMacroExpansion.html).

---
