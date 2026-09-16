# `pyrefly::alt::expr`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.alt.expr.json`](../model/pyrefly.alt.expr.json)

## TypeOrExpr

`enum` · `pyrefly::alt::expr::TypeOrExpr`

```rust
enum TypeOrExpr<'a>
```

**Variants**: `Type`, `Expr`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug

**Methods** (2)

```rust
fn infer<Ans: LookupAnswer>(self, solver: &AnswersSolver<'_, '_, Ans>, errors: &ErrorCollector) -> Type
fn transform<Ans: LookupAnswer>(&self, solver: &AnswersSolver<'_, '_, Ans>, errors: &ErrorCollector, owner: &'a Owner<Type>, transformation: impl Fn(&Type) -> Type) -> (Self, bool)
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## ExprOptions

`struct` · `pyrefly::alt::expr::ExprOptions`

```rust
struct ExprOptions<'a, 'b, 'subset>
```

**Methods** (2)

```rust
fn check(want: &'b Type, errors: &'a ErrorCollector, check_errors: &'a ErrorCollector, context: &'a dyn Fn() -> TypeCheckContext, call_context: Option<&'a CallContext<'subset>>) -> Self
fn infer(errors: &'a ErrorCollector, hint: Option<HintRef<'a, 'b>>) -> Self
```

---
