# `pyrefly::alt::solve`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.alt.solve.json`](../model/pyrefly.alt.solve.json)

## Iterable

`enum` · `pyrefly::alt::solve::Iterable`

```rust
enum Iterable
```

**Variants**: `OfType`, `FixedLen`, `Unpacked`, `OfTypeVarTuple`

**Derives**: Debug

---

## TypeFormContext

`enum` · `pyrefly::alt::solve::TypeFormContext`

```rust
enum TypeFormContext<'a>
```

**Variants**: `TypeExpression`, `BaseClassList`, `ClassKeyword`, `ClassVarAnnotation`, `FunctionArgument`, `GenericBase`, `ParameterAnnotation`, `ParameterArgsAnnotation`, `ParameterKwargsAnnotation`, `ReturnAnnotation`, `TypeVarConstraint`, `TypeVarDefault`, `IntVarDefault`, `ParamSpecDefault`, `TypeVarTupleDefault`, `TypeAlias`, `VarAnnotation`, `TypeArgument`, `TypeArgumentForType`, `TypeArgumentCallableReturn`, `TypeLevelLambdaReturn`, `TypePredicateArgument`, `TupleElement`, `TupleOrCallableParam`, `UnionMember`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn is_valid_unparameterized_annotation(self, x: SpecialForm) -> bool
fn quantified_kind_default(x: QuantifiedKind) -> TypeFormContext<'static>
const fn type_argument() -> TypeFormContext<'static>
```

---
