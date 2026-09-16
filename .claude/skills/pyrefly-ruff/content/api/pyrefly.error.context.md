# `pyrefly::error::context`

Crate `pyrefly` · 3 public items · structured records in [`model/pyrefly.error.context.json`](../model/pyrefly.error.context.json)

## ErrorContext

`enum` · `pyrefly::error::context::ErrorContext`

```rust
enum ErrorContext
```

**Variants**: `BadContextManager`, `UnaryOp`, `BinaryOp`, `InplaceBinaryOp`, `Iteration`, `AsyncIteration`, `Await`, `Index`, `SetItem`, `DelItem`, `MatchPositional`, `ImportNotFound`, `ImportNotTyped`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn annotations(&self) -> Vec<(TextRange, String)>
fn as_error_kind(&self) -> ErrorKind
fn format(&self) -> String
```

General context for an error. For many errors, the root cause is some steps removed from what
the user sees. For example:
  class C:
    def __lt__(self, other: C) -> bool:
  C() < 0  # ERROR: expected C, got 0
The root cause is `C.__lt__` being called with the wrong type, but the user sees a `<`
comparison. ErrorContext stores this context that the user sees, to make it easier to connect
it back to the root cause.

Note: Types stored in the ErrorContext should be processed through `AnswersSolver::for_display`
otherwise printed type representations may be non-deterministic due to unsolved vars

---

## TypeCheckKind

`enum` · `pyrefly::error::context::TypeCheckKind`

```rust
enum TypeCheckKind
```

**Variants**: `MagicMethodReturn`, `AugmentedAssignment`, `ImplicitFunctionReturn`, `ExplicitFunctionReturn`, `TypeGuardReturn`, `CallArgument`, `CallVarArgs`, `CallKwArgs`, `CallUnpackKwArg`, `FunctionParameterDefault`, `DictKey`, `DictValue`, `TypedDictKey`, `TypedDictUnpacking`, `TypedDictOpenUnpacking`, `Attribute`, `AnnotatedName`, `IterationVariableMismatch`, `AnnAssign`, `UnpackedAssign`, `ExceptionClass`, `YieldValue`, `YieldFrom`, `UnexpectedBareYield`, `PostInit`, `OverloadReturn`, `OverloadInput`, `OverloadDefault`, `TypeVarSpecialization`, `Container`

**Derives**: Debug

**Methods** (3)

```rust
fn as_error_kind(&self) -> ErrorKind
fn format_error(&self, got: &Type, want: &Type, current_module: ModuleName) -> String
fn from_annotation_target(target: &AnnotationTarget) -> Self
```

---

## TypeCheckContext

`struct` · `pyrefly::error::context::TypeCheckContext`

```rust
struct TypeCheckContext
```

**Fields**: `kind`, `context`, `annotations`

**Derives**: Debug

**Methods** (4)

```rust
fn of_kind(kind: TypeCheckKind) -> Self
fn with_annotation(self, range: Option<TextRange>, label: String) -> Self
fn with_annotations(self, annotations: Vec<(TextRange, String)>) -> Self
fn with_context(self, context: Option<ErrorContext>) -> Self
```

The context in which a got <: want type check occurs. This differs from ErrorContext in that
TypeCheckContext applies specifically to type mismatches. For example:
  class C:
    def __lt__(self, other: C) -> bool:
  C() < 0  # ERROR: expected C, got 0
The TypeCheckContext contains a TypeCheckKind::CallArgument, recording that
the mismatch is in the `other` parameter of `C.__lt__`, and an
ErrorContext::BinaryOp, recording that the type mismatch occurs in the context of a `<` comparison.

---
