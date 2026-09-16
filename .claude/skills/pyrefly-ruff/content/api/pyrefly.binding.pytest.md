# `pyrefly::binding::pytest`

Crate `pyrefly` · 4 public items · structured records in [`model/pyrefly.binding.pytest.json`](../model/pyrefly.binding.pytest.json)

## is_pytest_fixture_function

`function` · `pyrefly::binding::pytest::is_pytest_fixture_function`

```rust
fn is_pytest_fixture_function(function_def: &ruff_python_ast::StmtFunctionDef, aliases: &PytestAliases) -> bool
```

---

## PytestAliases

`struct` · `pyrefly::binding::pytest::PytestAliases`

```rust
struct PytestAliases
```

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn from_module(module: &ModModule) -> Self
fn is_empty(&self) -> bool
```

---

## PytestBindingInfo

`struct` · `pyrefly::binding::pytest::PytestBindingInfo`

```rust
struct PytestBindingInfo
```

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn add_fixture_definition(&mut self, name: Name, return_type_key: ShortIdentifier, class_key: Option<Idx<KeyClass>>)
fn aliases(&self) -> &PytestAliases
fn from_module(module: &ModModule) -> Option<Self>
fn is_fixture_definition(&self, func_name: &ruff_python_ast::Identifier, class_key: Option<&Idx<KeyClass>>) -> bool
fn visible_fixture_class_key(&self, name: &Name, class_key: Option<&Idx<KeyClass>>) -> Option<Option<Idx<KeyClass>>>
```

---

## PytestFixtureDefinitions

`struct` · `pyrefly::binding::pytest::PytestFixtureDefinitions`

```rust
struct PytestFixtureDefinitions
```

**Derives**: Clone, Debug

---
