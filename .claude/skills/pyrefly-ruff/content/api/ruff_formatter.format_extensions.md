# `ruff_formatter::format_extensions`

Crate `ruff_formatter` · 2 public items · structured records in [`model/ruff_formatter.format_extensions.json`](../model/ruff_formatter.format_extensions.json)

## Memoized

`struct` · `ruff_formatter::format_extensions::Memoized`

Also reachable as `ruff_formatter::prelude::Memoized`

```rust
struct Memoized<F, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Debug

**Methods** (1)

```rust
fn inspect(&self, f: &mut Formatter<'_, Context>) -> FormatResult<&[FormatElement]>
```

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

Memoizes the output of its inner [`Format`] to avoid re-formatting a potential expensive object.

---

## MemoizeFormat

`trait` · `ruff_formatter::format_extensions::MemoizeFormat`

Also reachable as `ruff_formatter::prelude::MemoizeFormat`

```rust
trait MemoizeFormat<Context>
```

**Methods** (1)

```rust
fn memoized(self) -> Memoized<Self, Context> where Self: Sized + Format<Context>
```

Utility trait that allows memorizing the output of a [`Format`].
Useful to avoid re-formatting the same object twice.

---
