# `ruff_python_semantic::analyze::typing`

Crate `ruff_python_semantic` · 37 public items · structured records in [`model/ruff_python_semantic.analyze.typing.json`](../model/ruff_python_semantic.analyze.typing.json)

## Callable

`enum` · `ruff_python_semantic::analyze::typing::Callable`

```rust
enum Callable
```

**Variants**: `Bool`, `Cast`, `NewType`, `TypeVar`, `NamedTuple`, `TypedDict`, `MypyExtension`, `TypeAliasType`

**Derives**: Clone, Copy, Debug

---

## ModuleMember

`enum` · `ruff_python_semantic::analyze::typing::ModuleMember`

```rust
enum ModuleMember
```

**Variants**: `BuiltIn`, `Member`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## Pep604Operator

`enum` · `ruff_python_semantic::analyze::typing::Pep604Operator`

```rust
enum Pep604Operator
```

**Variants**: `Union`, `Optional`

**Derives**: Clone, Copy, Debug

---

## SubscriptKind

`enum` · `ruff_python_semantic::analyze::typing::SubscriptKind`

```rust
enum SubscriptKind
```

**Variants**: `Literal`, `Generic`, `PEP593Annotation`, `TypedDict`

**Derives**: Clone, Copy, Debug

---

## check_type

`function` · `ruff_python_semantic::analyze::typing::check_type`

```rust
fn check_type<T: TypeChecker>(binding: &Binding<'_>, semantic: &model::SemanticModel<'_>) -> bool
```

Check if the type checker accepts the given binding with the given name.

NOTE: this function doesn't perform more serious type inference, so it won't be able
      to understand if the value gets initialized from a call to a function always returning
      lists. This also implies no interfile analysis.

---

## find_assigned_value

`function` · `ruff_python_semantic::analyze::typing::find_assigned_value`

```rust
fn find_assigned_value<'a>(symbol: &str, semantic: &'a model::SemanticModel<'a>) -> Option<&'a ruff_python_ast::Expr>
```

Find the assigned [`Expr`] for a given symbol, if any.

For example given:
```python
 foo = 42
 (bar, bla) = 1, "str"
```

This function will return a `NumberLiteral` with value `Int(42)` when called with `foo` and a
`StringLiteral` with value `"str"` when called with `bla`.

---

## find_binding_value

`function` · `ruff_python_semantic::analyze::typing::find_binding_value`

```rust
fn find_binding_value<'a>(binding: &Binding<'_>, semantic: &'a model::SemanticModel<'_>) -> Option<&'a ruff_python_ast::Expr>
```

Find the assigned [`Expr`] for a given [`Binding`], if any.

For example given:
```python
 foo = 42
 (bar, bla) = 1, "str"
```

This function will return a `NumberLiteral` with value `Int(42)` when called with `foo` and a
`StringLiteral` with value `"str"` when called with `bla`.

---

## is_bytes

`function` · `ruff_python_semantic::analyze::typing::is_bytes`

```rust
fn is_bytes(binding: &Binding<'_>, semantic: &model::SemanticModel<'_>) -> bool
```

Test whether the given binding can be considered an instance of `bytes`.

---

## is_dict

`function` · `ruff_python_semantic::analyze::typing::is_dict`

```rust
fn is_dict(binding: &Binding<'_>, semantic: &model::SemanticModel<'_>) -> bool
```

Test whether the given binding can be considered a dictionary.

For this, we check what value might be associated with it through it's initialization,
what annotation it has (we consider `dict` and `typing.Dict`), and if it is a variadic keyword
argument parameter.

---

## is_fastapi_route

`function` · `ruff_python_semantic::analyze::typing::is_fastapi_route`

```rust
fn is_fastapi_route(binding: &Binding<'_>, semantic: &model::SemanticModel<'_>) -> bool
```

---

## is_float

`function` · `ruff_python_semantic::analyze::typing::is_float`

```rust
fn is_float(binding: &Binding<'_>, semantic: &model::SemanticModel<'_>) -> bool
```

Test whether the given binding can be considered an instance of `float`.

---

## is_immutable_annotation

`function` · `ruff_python_semantic::analyze::typing::is_immutable_annotation`

```rust
fn is_immutable_annotation(expr: &ruff_python_ast::Expr, semantic: &model::SemanticModel<'_>, extend_immutable_calls: &[ruff_python_ast::name::QualifiedName<'_>]) -> bool
```

Return `true` if `Expr` represents a reference to a type annotation that resolves to an
immutable type.

---

## is_immutable_func

`function` · `ruff_python_semantic::analyze::typing::is_immutable_func`

```rust
fn is_immutable_func(func: &ruff_python_ast::Expr, semantic: &model::SemanticModel<'_>, extend_immutable_calls: &[ruff_python_ast::name::QualifiedName<'_>]) -> bool
```

Return `true` if `func` is a function that returns an immutable value.

---

## is_immutable_newtype_call

`function` · `ruff_python_semantic::analyze::typing::is_immutable_newtype_call`

```rust
fn is_immutable_newtype_call(name: &ast::ExprName, semantic: &model::SemanticModel<'_>, extend_immutable_calls: &[ruff_python_ast::name::QualifiedName<'_>]) -> bool
```

Return `true` if `name` is bound to the `typing.NewType` call where the original type is
immutable.

For example:
```python
from typing import NewType

UserId = NewType("UserId", int)
```

Here, `name` would be `UserId`.

---

## is_int

`function` · `ruff_python_semantic::analyze::typing::is_int`

```rust
fn is_int(binding: &Binding<'_>, semantic: &model::SemanticModel<'_>) -> bool
```

Test whether the given binding can be considered an integer.

---

## is_io_base

`function` · `ruff_python_semantic::analyze::typing::is_io_base`

```rust
fn is_io_base(binding: &Binding<'_>, semantic: &model::SemanticModel<'_>) -> bool
```

Test whether the given binding can be considered a file-like object (i.e., a type that
implements `io.IOBase`).

---

## is_io_base_expr

`function` · `ruff_python_semantic::analyze::typing::is_io_base_expr`

```rust
fn is_io_base_expr(expr: &ruff_python_ast::Expr, semantic: &model::SemanticModel<'_>) -> bool
```

Test whether the given expression can be considered a file-like object (i.e., a type that
implements `io.IOBase`).

---

## is_known_to_be_of_type_dict

`function` · `ruff_python_semantic::analyze::typing::is_known_to_be_of_type_dict`

```rust
fn is_known_to_be_of_type_dict(semantic: &model::SemanticModel<'_>, expr: &ruff_python_ast::ExprName) -> bool
```

---

## is_list

`function` · `ruff_python_semantic::analyze::typing::is_list`

```rust
fn is_list(binding: &Binding<'_>, semantic: &model::SemanticModel<'_>) -> bool
```

Test whether the given binding can be considered a list.

For this, we check what value might be associated with it through it's initialization and
what annotation it has (we consider `list` and `typing.List`)

---

## is_mutable_expr

`function` · `ruff_python_semantic::analyze::typing::is_mutable_expr`

```rust
fn is_mutable_expr(expr: &ruff_python_ast::Expr, semantic: &model::SemanticModel<'_>) -> bool
```

Return `true` if `expr` is an expression that resolves to a mutable value.

---

## is_mutable_func

`function` · `ruff_python_semantic::analyze::typing::is_mutable_func`

```rust
fn is_mutable_func(func: &ruff_python_ast::Expr, semantic: &model::SemanticModel<'_>) -> bool
```

Return `true` if `func` is a function that returns a mutable value.

---

## is_pathlib_path

`function` · `ruff_python_semantic::analyze::typing::is_pathlib_path`

```rust
fn is_pathlib_path(binding: &Binding<'_>, semantic: &model::SemanticModel<'_>) -> bool
```

Test whether the given binding can be considered a `pathlib.PurePath`
or an instance of a subclass thereof.

---

## is_pep585_generic

`function` · `ruff_python_semantic::analyze::typing::is_pep585_generic`

```rust
fn is_pep585_generic(expr: &ruff_python_ast::Expr, semantic: &model::SemanticModel<'_>) -> bool
```

Return whether a given expression uses a PEP 585 standard library generic.

---

## is_set

`function` · `ruff_python_semantic::analyze::typing::is_set`

```rust
fn is_set(binding: &Binding<'_>, semantic: &model::SemanticModel<'_>) -> bool
```

Test whether the given binding can be considered a set.

For this, we check what value might be associated with it through it's initialization and
what annotation it has (we consider `set` and `typing.Set`).

---

## is_string

`function` · `ruff_python_semantic::analyze::typing::is_string`

```rust
fn is_string(binding: &Binding<'_>, semantic: &model::SemanticModel<'_>) -> bool
```

Test whether the given binding can be considered an instance of `str`.

---

## is_sys_version_block

`function` · `ruff_python_semantic::analyze::typing::is_sys_version_block`

```rust
fn is_sys_version_block(stmt: &ast::StmtIf, semantic: &model::SemanticModel<'_>) -> bool
```

Returns `true` if the [`ast::StmtIf`] is a version-checking block (e.g., `if sys.version_info >= ...:`).

---

## is_tuple

`function` · `ruff_python_semantic::analyze::typing::is_tuple`

```rust
fn is_tuple(binding: &Binding<'_>, semantic: &model::SemanticModel<'_>) -> bool
```

Test whether the given binding can be considered a tuple.

For this, we check what value might be associated with it through it's initialization, what
annotation it has (we consider `tuple` and `typing.Tuple`), and if it is a variadic positional
argument.

---

## is_type_checking_block

`function` · `ruff_python_semantic::analyze::typing::is_type_checking_block`

```rust
fn is_type_checking_block(stmt: &ast::StmtIf, semantic: &model::SemanticModel<'_>) -> bool
```

Return `true` if [`ast::StmtIf`] is a guard for a type-checking block.

---

## match_annotated_subscript

`function` · `ruff_python_semantic::analyze::typing::match_annotated_subscript`

```rust
fn match_annotated_subscript<'a>(expr: &ruff_python_ast::Expr, semantic: &model::SemanticModel<'_>, typing_modules: impl Iterator<Item = &'a str>, extend_generics: &[String]) -> Option<SubscriptKind>
```

---

## resolve_assignment

`function` · `ruff_python_semantic::analyze::typing::resolve_assignment`

```rust
fn resolve_assignment<'a>(expr: &'a ruff_python_ast::Expr, semantic: &'a model::SemanticModel<'a>) -> Option<ruff_python_ast::name::QualifiedName<'a>>
```

Return the [`QualifiedName`] of the value to which the given [`Expr`] is assigned, if any.

For example, given:
```python
import asyncio

loop = asyncio.get_running_loop()
loop.create_task(...)
```

This function will return `["asyncio", "get_running_loop"]` for the `loop` binding.

This function will also automatically expand attribute accesses, so given:
```python
from module import AppContainer

container = AppContainer()
container.app.get(...)
```

This function will return `["module", "AppContainer", "app", "get"]` for the
attribute access `container.app.get`.

---

## to_pep585_generic

`function` · `ruff_python_semantic::analyze::typing::to_pep585_generic`

```rust
fn to_pep585_generic(expr: &ruff_python_ast::Expr, semantic: &model::SemanticModel<'_>) -> Option<ModuleMember>
```

Returns the PEP 585 standard library generic variant for a `typing` module reference, if such
a variant exists.

---

## to_pep604_operator

`function` · `ruff_python_semantic::analyze::typing::to_pep604_operator`

```rust
fn to_pep604_operator(value: &ruff_python_ast::Expr, slice: &ruff_python_ast::Expr, semantic: &model::SemanticModel<'_>) -> Option<Pep604Operator>
```

Return the PEP 604 operator variant to which the given subscript [`Expr`] corresponds, if any.

---

## traverse_literal

`function` · `ruff_python_semantic::analyze::typing::traverse_literal`

```rust
fn traverse_literal<'a, F>(func: &mut F, semantic: &model::SemanticModel<'_>, expr: &'a ruff_python_ast::Expr) where F: FnMut(&'a ruff_python_ast::Expr, &'a ruff_python_ast::Expr)
```

Traverse a "literal" type annotation, applying `func` to each literal member.

The function is called with each expression in the literal (excluding declarations of nested
literals) and the parent expression.

---

## traverse_union

`function` · `ruff_python_semantic::analyze::typing::traverse_union`

```rust
fn traverse_union<'a, F>(func: &mut F, semantic: &model::SemanticModel<'_>, expr: &'a ruff_python_ast::Expr) where F: FnMut(&'a ruff_python_ast::Expr, &'a ruff_python_ast::Expr)
```

Traverse a "union" type annotation, applying `func` to each union member.

Supports traversal of `Union` and `|` union expressions.

The function is called with each expression in the union (excluding declarations of nested
unions) and the parent expression.

---

## traverse_union_and_optional

`function` · `ruff_python_semantic::analyze::typing::traverse_union_and_optional`

```rust
fn traverse_union_and_optional<'a, F>(func: &mut F, semantic: &model::SemanticModel<'_>, expr: &'a ruff_python_ast::Expr) where F: FnMut(&'a ruff_python_ast::Expr, &'a ruff_python_ast::Expr)
```

Traverse a "union" type annotation, applying `func` to each union member.

Supports traversal of `Union`, `|`, and `Optional` union expressions.

The function is called with each expression in the union (excluding declarations of nested
unions) and the parent expression.

---

## PathlibPathChecker

`struct` · `ruff_python_semantic::analyze::typing::PathlibPathChecker`

```rust
struct PathlibPathChecker
```

**Implements**: `ruff_python_semantic::analyze::typing::TypeChecker`

**via `ruff_python_semantic::analyze::typing::TypeChecker`**

```rust
fn match_annotation(annotation: &Expr, semantic: &SemanticModel<'_>) -> bool
fn match_initializer(initializer: &Expr, semantic: &SemanticModel<'_>) -> bool
```

---

## TypeChecker

`trait` · `ruff_python_semantic::analyze::typing::TypeChecker`

```rust
trait TypeChecker
```

**Implementors** (1)

- `ruff_python_semantic::analyze::typing::PathlibPathChecker`

**Methods** (2)

```rust
fn match_annotation(annotation: &Expr, semantic: &SemanticModel<'_>) -> bool
fn match_initializer(initializer: &Expr, semantic: &SemanticModel<'_>) -> bool
```

Abstraction for a type checker, conservatively checks for the intended type(s).

---
