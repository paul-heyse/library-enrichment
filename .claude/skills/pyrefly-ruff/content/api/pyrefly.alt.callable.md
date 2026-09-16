# `pyrefly::alt::callable`

Crate `pyrefly` · 6 public items · structured records in [`model/pyrefly.alt.callable.json`](../model/pyrefly.alt.callable.json)

## CallArg

`enum` · `pyrefly::alt::callable::CallArg`

```rust
enum CallArg<'a>
```

**Variants**: `Arg`, `Star`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn arg(x: TypeOrExpr<'a>) -> Self
fn expr(x: &'a Expr) -> Self
fn expr_maybe_starred(x: &'a Expr) -> Self
fn materialize<Ans: LookupAnswer>(&self, solver: &AnswersSolver<'_, '_, Ans>, errors: &ErrorCollector, owner: &'a Owner<Type>) -> (Self, bool)
fn ty(ty: &'a Type, range: TextRange) -> Self
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## ReturnTypeResolutionError

`enum` · `pyrefly::alt::callable::ReturnTypeResolutionError`

```rust
enum ReturnTypeResolutionError
```

**Variants**: `TypeLevelDsl`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

An error discovered while resolving or finalizing a call candidate's return type.

These errors do not make an otherwise valid candidate ineligible during overload or
contextual-hint selection. Each candidate carries its own errors through selection, and only
the selected candidate's errors are reported. Type-level DSL evaluation is currently the only
fallible return-type operation, but other fallible return finalization belongs in this channel.

---

## ArgMap

`struct` · `pyrefly::alt::callable::ArgMap`

```rust
struct ArgMap
```

**Fields**: `range_to_param`, `unmatched_params`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new() -> Self
```

---

## CallKeyword

`struct` · `pyrefly::alt::callable::CallKeyword`

```rust
struct CallKeyword<'a>
```

**Fields**: `range`, `arg`, `value`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn materialize<Ans: LookupAnswer>(&self, solver: &AnswersSolver<'_, '_, Ans>, errors: &ErrorCollector, owner: &'a Owner<Type>) -> (Self, bool)
fn new(x: &'a Keyword) -> Self
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## CallWithTypes

`struct` · `pyrefly::alt::callable::CallWithTypes`

```rust
struct CallWithTypes
```

**Methods** (6)

```rust
fn call_arg<'a, 'b: 'a, Ans: LookupAnswer>(&'a self, x: &CallArg<'b>, solver: &AnswersSolver<'_, '_, Ans>, errors: &ErrorCollector) -> CallArg<'a>
fn call_keyword<'a, 'b: 'a, Ans: LookupAnswer>(&'a self, x: &CallKeyword<'b>, solver: &AnswersSolver<'_, '_, Ans>, errors: &ErrorCollector) -> CallKeyword<'a>
fn new() -> Self
fn type_or_expr<'a, 'b: 'a, Ans: LookupAnswer>(&'a self, x: TypeOrExpr<'b>, solver: &AnswersSolver<'_, '_, Ans>, errors: &ErrorCollector) -> TypeOrExpr<'a>
fn vec_call_arg<'a, 'b: 'a, Ans: LookupAnswer>(&'a self, xs: &[CallArg<'b>], solver: &AnswersSolver<'_, '_, Ans>, errors: &ErrorCollector) -> Vec<CallArg<'a>>
fn vec_call_keyword<'a, 'b: 'a, Ans: LookupAnswer>(&'a self, xs: &[CallKeyword<'b>], solver: &AnswersSolver<'_, '_, Ans>, errors: &ErrorCollector) -> Vec<CallKeyword<'a>>
```

Structure to turn TypeOrExprs into Types.
This is used to avoid re-inferring types for arguments multiple times.

Implemented by keeping an `Owner` to hand out references to `Type`.

---

## MatchedParam

`struct` · `pyrefly::alt::callable::MatchedParam`

```rust
struct MatchedParam
```

**Fields**: `ty`, `name`

**Derives**: Clone, Debug

The parameter an argument was matched against.

---
