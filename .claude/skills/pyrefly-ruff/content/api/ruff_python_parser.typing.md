# `ruff_python_parser::typing`

Crate `ruff_python_parser` · 3 public items · structured records in [`model/ruff_python_parser.typing.json`](../model/ruff_python_parser.typing.json)

## AnnotationKind

`enum` · `ruff_python_parser::typing::AnnotationKind`

```rust
enum AnnotationKind
```

**Variants**: `Simple`, `Complex`

**Derives**: Clone, Copy, Debug

**Methods** (1)

```rust
const fn is_simple(self) -> bool
```

---

## parse_type_annotation

`function` · `ruff_python_parser::typing::parse_type_annotation`

```rust
fn parse_type_annotation(string_expr: &ruff_python_ast::ExprStringLiteral, source: &str) -> Result<ParsedAnnotation, ParseError>
```

Parses the given string expression node as a type annotation. The given `source` is the entire
source code.

---

## ParsedAnnotation

`struct` · `ruff_python_parser::typing::ParsedAnnotation`

```rust
struct ParsedAnnotation
```

**Derives**: Debug

**Methods** (3)

```rust
fn expression(&self) -> &Expr
fn kind(&self) -> AnnotationKind
fn parsed(&self) -> &Parsed<ModExpression>
```

---
