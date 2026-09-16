# `ruff_python_semantic::analyze::visibility`

Crate `ruff_python_semantic` · 15 public items · structured records in [`model/ruff_python_semantic.analyze.visibility.json`](../model/ruff_python_semantic.analyze.visibility.json)

## Visibility

`enum` · `ruff_python_semantic::analyze::visibility::Visibility`

```rust
enum Visibility
```

**Variants**: `Public`, `Private`

**Derives**: Clone, Copy, Debug

**Methods** (2)

```rust
const fn is_private(&self) -> bool
const fn is_public(&self) -> bool
```

---

## is_abstract

`function` · `ruff_python_semantic::analyze::visibility::is_abstract`

```rust
fn is_abstract(decorator_list: &[ruff_python_ast::Decorator], semantic: &model::SemanticModel<'_>) -> bool
```

Returns `true` if a function definition is an abstract method based on its decorators.

---

## is_call

`function` · `ruff_python_semantic::analyze::visibility::is_call`

```rust
fn is_call(name: &str) -> bool
```

Returns `true` if a function is a `__call__`.

---

## is_classmethod

`function` · `ruff_python_semantic::analyze::visibility::is_classmethod`

```rust
fn is_classmethod(decorator_list: &[ruff_python_ast::Decorator], semantic: &model::SemanticModel<'_>) -> bool
```

Returns `true` if a function is a "class method".

---

## is_final

`function` · `ruff_python_semantic::analyze::visibility::is_final`

```rust
fn is_final(decorator_list: &[ruff_python_ast::Decorator], semantic: &model::SemanticModel<'_>) -> bool
```

Returns `true` if a class is an `final`.

---

## is_init

`function` · `ruff_python_semantic::analyze::visibility::is_init`

```rust
fn is_init(name: &str) -> bool
```

Returns `true` if a function is an `__init__`.

---

## is_magic

`function` · `ruff_python_semantic::analyze::visibility::is_magic`

```rust
fn is_magic(name: &str) -> bool
```

Returns `true` if a function is a "magic method".

---

## is_new

`function` · `ruff_python_semantic::analyze::visibility::is_new`

```rust
fn is_new(name: &str) -> bool
```

Returns `true` if a function is a `__new__`.

---

## is_overload

`function` · `ruff_python_semantic::analyze::visibility::is_overload`

```rust
fn is_overload(decorator_list: &[ruff_python_ast::Decorator], semantic: &model::SemanticModel<'_>) -> bool
```

Returns `true` if a function definition is an `@overload`.

---

## is_override

`function` · `ruff_python_semantic::analyze::visibility::is_override`

```rust
fn is_override(decorator_list: &[ruff_python_ast::Decorator], semantic: &model::SemanticModel<'_>) -> bool
```

Returns `true` if a function definition is an `@override` (PEP 698).

---

## is_property

`function` · `ruff_python_semantic::analyze::visibility::is_property`

```rust
fn is_property<'a, P, I>(decorator_list: &[ruff_python_ast::Decorator], extra_properties: P, semantic: &model::SemanticModel<'_>) -> bool where P: IntoIterator<IntoIter = I>, I: Iterator<Item = ruff_python_ast::name::QualifiedName<'a>> + Clone
```

Returns `true` if a function definition is a `@property`.
`extra_properties` can be used to check additional non-standard
`@property`-like decorators.

---

## is_staticmethod

`function` · `ruff_python_semantic::analyze::visibility::is_staticmethod`

```rust
fn is_staticmethod(decorator_list: &[ruff_python_ast::Decorator], semantic: &model::SemanticModel<'_>) -> bool
```

Returns `true` if a function is a "static method".

---

## is_test

`function` · `ruff_python_semantic::analyze::visibility::is_test`

```rust
fn is_test(name: &str) -> bool
```

Returns `true` if a function is a test one.

---

## is_validator

`function` · `ruff_python_semantic::analyze::visibility::is_validator`

```rust
fn is_validator(decorator_list: &[ruff_python_ast::Decorator], semantic: &model::SemanticModel<'_>) -> bool
```

Returns `true` if a function definition is an `attrs`-like validator based on its decorators.

---

## method_visibility

`function` · `ruff_python_semantic::analyze::visibility::method_visibility`

```rust
fn method_visibility(function: &ast::StmtFunctionDef) -> Visibility
```

Infer the [`Visibility`] of a method from its name and decorators.

---
