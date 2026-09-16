# `pyrefly::alt::traits`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.alt.traits.json`](../model/pyrefly.alt.traits.json)

## SolveResult

`enum` · `pyrefly::alt::traits::SolveResult`

```rust
enum SolveResult<K: Keyed>
```

**Variants**: `Answer`, `Alias`

---

## Solve

`trait` · `pyrefly::alt::traits::Solve`

```rust
trait Solve<Ans: LookupAnswer>: Keyed
```

**Implementors** (25)

- `pyrefly::binding::binding::Key`
- `pyrefly::binding::binding::KeyAbstractClassCheck`
- `pyrefly::binding::binding::KeyAnnotation`
- `pyrefly::binding::binding::KeyClass`
- `pyrefly::binding::binding::KeyClassBaseType`
- `pyrefly::binding::binding::KeyClassChecks`
- `pyrefly::binding::binding::KeyClassDisjointBase`
- `pyrefly::binding::binding::KeyClassField`
- `pyrefly::binding::binding::KeyClassMetadata`
- `pyrefly::binding::binding::KeyClassMro`
- `pyrefly::binding::binding::KeyClassSubscriptSymmetry`
- `pyrefly::binding::binding::KeyClassSynthesizedFields`
- `pyrefly::binding::binding::KeyDecoratedFunction`
- `pyrefly::binding::binding::KeyDecorator`
- `pyrefly::binding::binding::KeyDjangoRelations`
- `pyrefly::binding::binding::KeyExpect`
- `pyrefly::binding::binding::KeyExport`
- `pyrefly::binding::binding::KeyLegacyTypeParam`
- `pyrefly::binding::binding::KeyTParams`
- `pyrefly::binding::binding::KeyTypeAlias`
- `pyrefly::binding::binding::KeyUndecoratedFunction`
- `pyrefly::binding::binding::KeyUndecoratedFunctionRange`
- `pyrefly::binding::binding::KeyVariance`
- `pyrefly::binding::binding::KeyYield`
- `pyrefly::binding::binding::KeyYieldFrom`

**Methods** (5)

```rust
fn check_shortcut(_answers: &AnswersSolver<'_, '_, Ans>, _binding: &Self::Value) -> Option<Self::Answer>
fn create_recursive(_answers: &AnswersSolver<'_, '_, Ans>, _binding: &Self::Value) -> Var
fn promote_recursive(heap: &TypeHeap, x: Var) -> Self::Answer
fn record_recursive(_answers: &AnswersSolver<'_, '_, Ans>, answer: Self::Answer, _recursive: Var) -> Self::Answer
fn solve(answers: &AnswersSolver<'_, '_, Ans>, binding: &Self::Value, range: TextRange, errors: &ErrorCollector) -> SolveResult<Self>
```

---
