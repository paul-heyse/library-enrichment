# `ruff_python_codegen::generator`

Crate `ruff_python_codegen` · 2 public items · structured records in [`model/ruff_python_codegen.generator.json`](../model/ruff_python_codegen.generator.json)

## Mode

`enum` · `ruff_python_codegen::generator::Mode`

Also reachable as `ruff_python_codegen::Mode`

```rust
enum Mode
```

**Variants**: `Default`, `AstUnparse`

**Derives**: Default

---

## Generator

`struct` · `ruff_python_codegen::generator::Generator`

Also reachable as `ruff_python_codegen::Generator`

```rust
struct Generator<'a>
```

**Implements**: `core::convert::From`

**Methods** (5)

```rust
fn expr(self, expr: &Expr) -> String
const fn new(indent: &'a Indentation, line_ending: LineEnding) -> Self
fn stmt(self, stmt: &Stmt) -> String
fn unparse_suite(&mut self, suite: &Suite)
fn with_mode(self, mode: Mode) -> Self
```

**via `core::convert::From`**

```rust
fn from(stylist: &'a Stylist<'a>) -> Self
```

---
