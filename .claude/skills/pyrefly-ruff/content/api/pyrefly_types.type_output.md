# `pyrefly_types::type_output`

Crate `pyrefly_types` · 5 public items · structured records in [`model/pyrefly_types.type_output.json`](../model/pyrefly_types.type_output.json)

## AnnotationPart

`enum` · `pyrefly_types::type_output::AnnotationPart`

```rust
enum AnnotationPart
```

**Variants**: `Text`, `Reference`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

A semantic component of a type annotation.

---

## AnnotationOutput

`struct` · `pyrefly_types::type_output::AnnotationOutput`

```rust
struct AnnotationOutput<'a>
```

**Implements**: `pyrefly_types::type_output::TypeOutput`

**Methods** (2)

```rust
fn into_parts(self) -> Vec<AnnotationPart>
fn new(context: &'a TypeDisplayContext<'a>) -> Self
```

**via `pyrefly_types::type_output::TypeOutput`**

```rust
fn write_builtin(&mut self, name: &str, _qname: Option<&QName>) -> fmt::Result
fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result
fn write_lit(&mut self, lit: &Lit) -> fmt::Result
fn write_qname(&mut self, qname: &QName) -> fmt::Result
fn write_reference(&mut self, module: ModuleName, name: &str) -> fmt::Result
fn write_special_form(&mut self, name: &str, qname: Option<&QName>) -> fmt::Result
fn write_str(&mut self, s: &str) -> fmt::Result
fn write_targs(&mut self, targs: &TArgs) -> fmt::Result
fn write_type(&mut self, ty: &Type) -> fmt::Result
```

Type output that preserves references for alias and import resolution.

---

## DisplayOutput

`struct` · `pyrefly_types::type_output::DisplayOutput`

```rust
struct DisplayOutput<'a, 'b, 'f>
```

**Implements**: `pyrefly_types::type_output::TypeOutput`

**Methods** (1)

```rust
fn new(context: &'a TypeDisplayContext<'a>, formatter: &'b mut fmt::Formatter<'f>) -> Self
```

**via `pyrefly_types::type_output::TypeOutput`**

```rust
fn write_builtin(&mut self, name: &str, _qname: Option<&QName>) -> fmt::Result
fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result
fn write_lit(&mut self, lit: &Lit) -> fmt::Result
fn write_qname(&mut self, q: &QName) -> fmt::Result
fn write_reference(&mut self, module: ModuleName, name: &str) -> fmt::Result
fn write_special_form(&mut self, name: &str, qname: Option<&QName>) -> fmt::Result
fn write_str(&mut self, s: &str) -> fmt::Result
fn write_targs(&mut self, targs: &TArgs) -> fmt::Result
fn write_type(&mut self, ty: &Type) -> fmt::Result
```

Implementation of `TypeOutput` that writes formatted types to plain text.

This struct wraps a `fmt::Formatter` and delegates type formatting to the
[`TypeDisplayContext`] to produce typed formatted as plain text.

---

## OutputWithLocations

`struct` · `pyrefly_types::type_output::OutputWithLocations`

```rust
struct OutputWithLocations<'a>
```

**Implements**: `pyrefly_types::type_output::TypeOutput`

**Methods** (2)

```rust
fn new(context: &'a TypeDisplayContext<'a>) -> Self
fn parts(&self) -> &[(String, Option<TextRangeWithModule>)]
```

**via `pyrefly_types::type_output::TypeOutput`**

```rust
fn write_builtin(&mut self, name: &str, qname: Option<&QName>) -> fmt::Result
fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result
fn write_lit(&mut self, lit: &Lit) -> fmt::Result
fn write_qname(&mut self, qname: &QName) -> fmt::Result
fn write_reference(&mut self, module: ModuleName, name: &str) -> fmt::Result
fn write_special_form(&mut self, name: &str, qname: Option<&QName>) -> fmt::Result
fn write_str(&mut self, s: &str) -> fmt::Result
fn write_targs(&mut self, targs: &TArgs) -> fmt::Result
fn write_type(&mut self, ty: &Type) -> fmt::Result
```

This struct is used to collect the type to be displayed as a vector. Each element
in the vector will be tuple of (String, Option<TextRangeWithModule>).
The String the actual part of the string that will be displayed. When displaying the type
each of these will be concatenated to create the final type.
The second element of the vector is an optional location. For any part that do have
a location this will be included. For separators like '|', '[', etc. this will be None.

---

## TypeOutput

`trait` · `pyrefly_types::type_output::TypeOutput`

```rust
trait TypeOutput
```

**Implementors** (3)

- `pyrefly_types::type_output::AnnotationOutput`
- `pyrefly_types::type_output::DisplayOutput`
- `pyrefly_types::type_output::OutputWithLocations`

**Methods** (9)

```rust
fn write_builtin(&mut self, name: &str, qname: Option<&QName>) -> fmt::Result
fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result
fn write_lit(&mut self, lit: &Lit) -> fmt::Result
fn write_qname(&mut self, qname: &QName) -> fmt::Result
fn write_reference(&mut self, module: ModuleName, name: &str) -> fmt::Result
fn write_special_form(&mut self, name: &str, qname: Option<&QName>) -> fmt::Result
fn write_str(&mut self, s: &str) -> fmt::Result
fn write_targs(&mut self, targs: &TArgs) -> fmt::Result
fn write_type(&mut self, ty: &Type) -> fmt::Result
```

A trait that will be will be used for formatting types, but also allow
additional functionality. The major difference between implementations
of this trait will be the location that type information is written to
For example, this will be used to write type information to a formatter as we do now,
and also allow us to collect the same types along with their location into a vector.

---
