# `ruff_db::display`

Crate `ruff_db` · 2 public items · structured records in [`model/ruff_db.display.json`](../model/ruff_db.display.json)

## Join

`struct` · `ruff_db::display::Join`

```rust
struct Join<'a, 'b>
```

**Methods** (2)

```rust
fn entry(&mut self, item: &dyn Display) -> &mut Self
fn finish(&mut self) -> fmt::Result
```

---

## FormatterJoinExtension

`trait` · `ruff_db::display::FormatterJoinExtension`

```rust
trait FormatterJoinExtension<'b>
```

**Implementors** (1)

- `core::fmt::Formatter`

**Methods** (1)

```rust
fn join<'a>(&'a mut self, separator: &'static str) -> Join<'a, 'b>
```

---
