# `pyrefly::alt::unwrap`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.alt.unwrap.json`](../model/pyrefly.alt.unwrap.json)

## MAX_HINT_WIDTH

`constant` · `pyrefly::alt::unwrap::MAX_HINT_WIDTH`

```rust
const MAX_HINT_WIDTH: usize = 32
```

Maximum size for a union hint. Hints wider than this are not tried
individually, as doing so would be prohibitively expensive.

---

## HintRef

`struct` · `pyrefly::alt::unwrap::HintRef`

```rust
struct HintRef<'a, 'b>
```

**Derives**: Clone, Copy, Debug

**Methods** (7)

```rust
fn errors(&self) -> Option<&ErrorCollector>
fn filter_for_call(hint: Option<Self>, tparams: Option<&TParams>) -> Option<Self>
fn filter_for_constructor(hint: Option<Self>, targs: &TArgs) -> Option<Self>
fn new(hint: &'b Type, errors: Option<&'a ErrorCollector>) -> Self
fn soft(hint: &'b Type) -> Self
fn types(&self) -> &'b [Type]
fn with_ty_opt(hint: Option<Self>, ty: Option<&'b Type>) -> Option<Self>
```

---
