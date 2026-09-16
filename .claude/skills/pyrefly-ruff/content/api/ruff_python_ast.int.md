# `ruff_python_ast::int`

Crate `ruff_python_ast` · 1 public items · structured records in [`model/ruff_python_ast.int.json`](../model/ruff_python_ast.int.json)

## Int

`struct` · `ruff_python_ast::int::Int`

Also reachable as `ruff_python_ast::Int`

```rust
struct Int
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn as_i32(&self) -> Option<i32>
fn as_i64(&self) -> Option<i64>
fn as_i8(&self) -> Option<i8>
fn as_u16(&self) -> Option<u16>
const fn as_u64(&self) -> Option<u64>
fn as_u8(&self) -> Option<u8>
fn as_usize(&self) -> Option<usize>
fn from_str_radix(number: &str, radix: u32, token: &str) -> Result<Self, std::num::ParseIntError>
```

**via `core::convert::From`**

```rust
fn from(value: u32) -> Self
fn from(value: u16) -> Self
fn from(value: u8) -> Self
fn from(value: u64) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

A Python integer literal. Represents both small (fits in an `i64`) and large integers.

---
