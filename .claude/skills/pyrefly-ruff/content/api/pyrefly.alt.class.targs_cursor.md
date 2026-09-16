# `pyrefly::alt::class::targs_cursor`

Crate `pyrefly` · 1 public items · structured records in [`model/pyrefly.alt.class.targs_cursor.json`](../model/pyrefly.alt.class.targs_cursor.json)

## TArgsCursor

`struct` · `pyrefly::alt::class::targs_cursor::TArgsCursor`

```rust
struct TArgsCursor
```

**Methods** (8)

```rust
fn consume_for_paramspec_arg(&mut self) -> &Type
fn consume_for_paramspec_value(&mut self) -> &[Type]
fn consume_for_typevar_arg(&mut self) -> &Type
fn consume_for_typevartuple_arg(&mut self, param_idx: usize, tparams: &TParams) -> &[Type]
fn nargs(&self) -> usize
fn nargs_unconsumed(&self, stop: usize) -> usize
fn new(targs: Vec<Type>) -> Self
fn peek(&self) -> Option<&Type>
```

Tracks the current position in the type argument list while matching
arguments to type parameters in `create_targs`.

---
