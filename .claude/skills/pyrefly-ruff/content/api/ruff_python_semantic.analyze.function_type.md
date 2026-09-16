# `ruff_python_semantic::analyze::function_type`

Crate `ruff_python_semantic` · 5 public items · structured records in [`model/ruff_python_semantic.analyze.function_type.json`](../model/ruff_python_semantic.analyze.function_type.json)

## FunctionType

`enum` · `ruff_python_semantic::analyze::function_type::FunctionType`

```rust
enum FunctionType
```

**Variants**: `Function`, `Method`, `ClassMethod`, `StaticMethod`, `NewMethod`

**Derives**: Clone, Copy, Debug

---

## classify

`function` · `ruff_python_semantic::analyze::function_type::classify`

```rust
fn classify(name: &str, decorator_list: &[ruff_python_ast::Decorator], parent_scope: &scope::Scope<'_>, semantic: &model::SemanticModel<'_>, classmethod_decorators: &[String], staticmethod_decorators: &[String]) -> FunctionType
```

Classify a function based on its scope, name, and decorators.

---

## is_class_method

`function` · `ruff_python_semantic::analyze::function_type::is_class_method`

```rust
fn is_class_method(decorator: &ruff_python_ast::Decorator, semantic: &model::SemanticModel<'_>, classmethod_decorators: &[String]) -> bool
```

Return `true` if a [`Decorator`] is indicative of a class method.
Note: Implicit class methods like `__init_subclass__` and `__class_getitem__` are not considered.

---

## is_stub

`function` · `ruff_python_semantic::analyze::function_type::is_stub`

```rust
fn is_stub(function_def: &ruff_python_ast::StmtFunctionDef, semantic: &model::SemanticModel<'_>) -> bool
```

Returns `true` if a function has an empty body, and is therefore a stub.

A function body is considered to be empty if it contains only `pass` statements, `...` literals,
`NotImplementedError` raises, or string literal statements (docstrings).

---

## is_subject_to_liskov_substitution_principle

`function` · `ruff_python_semantic::analyze::function_type::is_subject_to_liskov_substitution_principle`

```rust
fn is_subject_to_liskov_substitution_principle(function_name: &str, decorator_list: &[ruff_python_ast::Decorator], parent_scope: &scope::Scope<'_>, semantic: &model::SemanticModel<'_>, classmethod_decorators: &[String], staticmethod_decorators: &[String]) -> bool
```

Return `true` if this function is subject to the Liskov Substitution Principle.

Type checkers will check nearly all methods for compliance with the Liskov Substitution
Principle, but some methods are exempt.

---
