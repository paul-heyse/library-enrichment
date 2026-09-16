# `ruff_python_parser::semantic_errors`

Crate `ruff_python_parser` · 11 public items · structured records in [`model/ruff_python_parser.semantic_errors.json`](../model/ruff_python_parser.semantic_errors.json)

## AwaitOutsideAsyncFunctionKind

`enum` · `ruff_python_parser::semantic_errors::AwaitOutsideAsyncFunctionKind`

```rust
enum AwaitOutsideAsyncFunctionKind
```

**Variants**: `Await`, `AsyncFor`, `AsyncWith`, `AsyncComprehension`

**Implements**: `core::fmt::Display`, `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

---

## InvalidExpressionKind

`enum` · `ruff_python_parser::semantic_errors::InvalidExpressionKind`

```rust
enum InvalidExpressionKind
```

**Variants**: `Yield`, `NamedExpr`, `Await`

**Implements**: `core::fmt::Display`, `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

---

## InvalidExpressionPosition

`enum` · `ruff_python_parser::semantic_errors::InvalidExpressionPosition`

```rust
enum InvalidExpressionPosition
```

**Variants**: `TypeVarBound`, `TypeVarDefault`, `TypeVarTupleDefault`, `ParamSpecDefault`, `TypeAnnotation`, `GenericDefinition`, `TypeAlias`

**Implements**: `core::fmt::Display`, `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

---

## LazyImportContext

`enum` · `ruff_python_parser::semantic_errors::LazyImportContext`

```rust
enum LazyImportContext
```

**Variants**: `Function`, `Class`, `TryExceptBlocks`

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

---

## LazyImportKind

`enum` · `ruff_python_parser::semantic_errors::LazyImportKind`

```rust
enum LazyImportKind
```

**Variants**: `Import`, `ImportFrom`

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

---

## SemanticSyntaxErrorKind

`enum` · `ruff_python_parser::semantic_errors::SemanticSyntaxErrorKind`

```rust
enum SemanticSyntaxErrorKind
```

**Variants**: `LazyImportNotAllowed`, `LazyImportStar`, `LazyFutureImport`, `LateFutureImport`, `NamedExpressionInComprehensionIterable`, `NamedExpressionInClassBodyComprehension`, `ReboundComprehensionVariable`, `DuplicateTypeParameter`, `MultipleCaseAssignment`, `MultipleStarredNamesInSequencePattern`, `IrrefutableCasePattern`, `SingleStarredAssignment`, `WriteToDebug`, `InvalidExpression`, `DuplicateMatchKey`, `DuplicateMatchClassAttribute`, `LoadBeforeGlobalDeclaration`, `LoadBeforeNonlocalDeclaration`, `InvalidStarExpression`, `AsyncComprehensionInSyncComprehension`, `YieldOutsideFunction`, `ReturnOutsideFunction`, `AwaitOutsideAsyncFunction`, `DuplicateParameter`, `DuplicateKeywordArgument`, `NonlocalDeclarationAtModuleLevel`, `NonlocalAndGlobal`, `AnnotatedGlobal`, `AnnotatedNonlocal`, `YieldFromInAsyncFunction`, `NonModuleImportStar`, `MultipleStarredExpressions`, `FutureFeatureNotDefined`, `BreakOutsideLoop`, `ContinueOutsideLoop`, `GlobalParameter`, `NonlocalParameter`, `DifferentMatchPatternBindings`, `NonlocalWithoutBinding`, `TypeParameterDefaultOrder`, `ReturnInGenerator`

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

---

## WriteToDebugKind

`enum` · `ruff_python_parser::semantic_errors::WriteToDebugKind`

```rust
enum WriteToDebugKind
```

**Variants**: `Store`, `Delete`

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

---

## YieldOutsideFunctionKind

`enum` · `ruff_python_parser::semantic_errors::YieldOutsideFunctionKind`

```rust
enum YieldOutsideFunctionKind
```

**Variants**: `Yield`, `YieldFrom`, `Await`

**Implements**: `core::fmt::Display`, `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_await(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

---

## SemanticSyntaxChecker

`struct` · `ruff_python_parser::semantic_errors::SemanticSyntaxChecker`

```rust
struct SemanticSyntaxChecker
```

**Derives**: Debug, Default

**Methods** (3)

```rust
fn new() -> Self
fn visit_expr<Ctx: SemanticSyntaxContext>(&mut self, expr: &Expr, ctx: &Ctx)
fn visit_stmt<Ctx: SemanticSyntaxContext>(&mut self, stmt: &ast::Stmt, ctx: &Ctx)
```

---

## SemanticSyntaxError

`struct` · `ruff_python_parser::semantic_errors::SemanticSyntaxError`

```rust
struct SemanticSyntaxError
```

**Fields**: `kind`, `range`, `python_version`

**Implements**: `core::fmt::Display`, `get_size2::GetSize`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## SemanticSyntaxContext

`trait` · `ruff_python_parser::semantic_errors::SemanticSyntaxContext`

```rust
trait SemanticSyntaxContext
```

**Implementors** (1)

- `pyrefly::binding::bindings::BindingsBuilder`

**Methods** (18)

```rust
fn future_annotations_or_stub(&self) -> bool
fn global(&self, name: &str) -> Option<TextRange>
fn has_nonlocal_binding(&self, name: &str) -> bool
fn in_async_context(&self) -> bool
fn in_await_allowed_context(&self) -> bool
fn in_class_body_comprehension(&self) -> bool
fn in_function_scope(&self) -> bool
fn in_generator_context(&self) -> bool
fn in_loop_context(&self) -> bool
fn in_module_scope(&self) -> bool
fn in_notebook(&self) -> bool
fn in_sync_comprehension(&self) -> bool
fn in_yield_allowed_context(&self) -> bool
fn is_bound_parameter(&self, name: &str) -> bool
fn lazy_import_context(&self) -> Option<LazyImportContext>
fn python_version(&self) -> PythonVersion
fn report_semantic_error(&self, error: SemanticSyntaxError)
fn source(&self) -> &str
```

Information needed from a parent visitor to emit semantic syntax errors.

Note that the `in_*_scope` methods should refer to the immediately-enclosing scope. For example,
`in_function_scope` should return true for this case:

```python
def f():
    x  # here
```

but not for this case:

```python
def f():
    class C:
        x  # here
```

In contrast, the `in_*_context` methods should traverse parent scopes. For example,
`in_function_context` should return true for this case:

```python
def f():
    [x  # here
        for x in range(3)]
```

but not here:

```python
def f():
    class C:
        x  # here, classes break function scopes
```

---
