# `ruff_linter::registry`

Crate `ruff_linter` · 6 public items · structured records in [`model/ruff_linter.registry.json`](../model/ruff_linter.registry.json)

## INCOMPATIBLE_CODES

`constant` · `ruff_linter::registry::INCOMPATIBLE_CODES`

```rust
const INCOMPATIBLE_CODES: &[(Rule, Rule, &str); 2] = _
```

Pairs of checks that shouldn't be enabled together.

---

## FromCodeError

`enum` · `ruff_linter::registry::FromCodeError`

```rust
enum FromCodeError
```

**Variants**: `Unknown`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## LintSource

`enum` · `ruff_linter::registry::LintSource`

```rust
enum LintSource
```

**Variants**: `Ast`, `Io`, `PhysicalLines`, `LogicalLines`, `Tokens`, `Imports`, `Noqa`, `Filesystem`, `Toml`

**Derives**: Clone, Copy

**Methods** (9)

```rust
const fn is_ast(&self) -> bool
const fn is_filesystem(&self) -> bool
const fn is_imports(&self) -> bool
const fn is_io(&self) -> bool
const fn is_logical_lines(&self) -> bool
const fn is_noqa(&self) -> bool
const fn is_physical_lines(&self) -> bool
const fn is_tokens(&self) -> bool
const fn is_toml(&self) -> bool
```

---

## Linter

`enum` · `ruff_linter::registry::Linter`

```rust
enum Linter
```

**Variants**: `Airflow`, `Eradicate`, `FastApi`, `Flake82020`, `Flake8Annotations`, `Flake8Async`, `Flake8Bandit`, `Flake8BlindExcept`, `Flake8BooleanTrap`, `Flake8Bugbear`, `Flake8Builtins`, `Flake8Commas`, `Flake8Comprehensions`, `Flake8Copyright`, `Flake8Datetimez`, `Flake8Debugger`, `Flake8Django`, `Flake8ErrMsg`, `Flake8Executable`, `Flake8Fixme`, `Flake8FutureAnnotations`, `Flake8GetText`, `Flake8ImplicitStrConcat`, `Flake8ImportConventions`, `Flake8Logging`, `Flake8LoggingFormat`, `Flake8NoPep420`, `Flake8Pie`, `Flake8Print`, `Flake8Pyi`, `Flake8PytestStyle`, `Flake8Quotes`, `Flake8Raise`, `Flake8Return`, `Flake8Self`, `Flake8Simplify`, `Flake8Slots`, `Flake8TidyImports`, `Flake8Todos`, `Flake8TypeChecking`, `Flake8UnusedArguments`, `Flake8UsePathlib`, `Flynt`, `Isort`, `McCabe`, `Numpy`, `PandasVet`, `PEP8Naming`, `Perflint`, `Pycodestyle`, `Pydoclint`, `Pydocstyle`, `Pyflakes`, `PygrepHooks`, `Pylint`, `Pyupgrade`, `Refurb`, `Ruff`, `Tryceratops`

**Implements**: `ruff_linter::registry::RuleNamespace`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn all_rules(&Linter) -> ::std::vec::IntoIter<Rule>
fn code_for_rule(&self, rule: Rule) -> Option<&'static str>
const fn upstream_categories(&self) -> Option<&'static [UpstreamCategoryAndPrefix]>
```

**via `ruff_linter::registry::RuleNamespace`**

```rust
fn common_prefix(&self) -> &'static str
fn name(&self) -> &'static str
fn parse_code(code: &str) -> Option<(Self, &str)>
fn url(&self) -> Option<&'static str>
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> LinterIter
```

---

## LinterIter

`struct` · `ruff_linter::registry::LinterIter`

```rust
struct LinterIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [Linter]

---

## RuleNamespace

`trait` · `ruff_linter::registry::RuleNamespace`

```rust
trait RuleNamespace: Sized
```

**Implementors** (1)

- `ruff_linter::registry::Linter`

**Methods** (4)

```rust
fn common_prefix(&self) -> &'static str
fn name(&self) -> &'static str
fn parse_code(code: &str) -> Option<(Self, &str)>
fn url(&self) -> Option<&'static str>
```

---
