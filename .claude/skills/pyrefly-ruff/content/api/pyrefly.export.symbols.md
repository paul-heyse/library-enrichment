# `pyrefly::export::symbols`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.export.symbols.json`](../model/pyrefly.export.symbols.json)

## FlatSymbol

`struct` · `pyrefly::export::symbols::FlatSymbol`

```rust
struct FlatSymbol
```

**Fields**: `name`, `kind`

**Derives**: Debug

---

## FlatSymbols

`struct` · `pyrefly::export::symbols::FlatSymbols`

```rust
struct FlatSymbols
```

**Derives**: Debug

**Methods** (2)

```rust
fn iter(&self) -> impl Iterator<Item = (&FlatSymbol, Option<&FlatSymbol>)>
fn new(body: &[Stmt]) -> Self
```

A compact, source-ordered table of declarations and their parent links.

---
