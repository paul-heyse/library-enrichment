# `ruff_linter::rule_selector`

Crate `ruff_linter` · 8 public items · structured records in [`model/ruff_linter.rule_selector.json`](../model/ruff_linter.rule_selector.json)

## ParseError

`enum` · `ruff_linter::rule_selector::ParseError`

```rust
enum ParseError
```

**Variants**: `Unknown`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## RuleSelector

`enum` · `ruff_linter::rule_selector::RuleSelector`

Also reachable as `ruff_linter::RuleSelector`

```rust
enum RuleSelector
```

**Variants**: `All`, `Category`, `C`, `T`, `Linter`, `Prefix`, `Rule`

**Implements**: `core::convert::From`, `core::str::traits::FromStr`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (5)

```rust
fn all_rules(&self) -> impl Iterator<Item = Rule> + use<>
fn is_exact(&self) -> bool
fn prefix_and_code(&self) -> (&'static str, &'static str)
fn rules<'a>(&'a self, preview: &PreviewOptions) -> impl Iterator<Item = Rule> + use<{'lifetime': "'a"}>
fn specificity(&self) -> Specificity
```

**via `core::convert::From`**

```rust
fn from(linter: Eradicate) -> Self
fn from(linter: Flake8FutureAnnotations) -> Self
fn from(linter: Flake8Raise) -> Self
fn from(linter: Flynt) -> Self
fn from(linter: Flake8BlindExcept) -> Self
fn from(linter: Airflow) -> Self
fn from(linter: Flake8Logging) -> Self
fn from(linter: Numpy) -> Self
fn from(linter: Flake8Fixme) -> Self
fn from(linter: Flake8ImportConventions) -> Self
fn from(linter: Pydocstyle) -> Self
fn from(linter: Flake8Simplify) -> Self
fn from(linter: Flake8Commas) -> Self
fn from(linter: PandasVet) -> Self
fn from(linter: Flake8Comprehensions) -> Self
fn from(linter: FastApi) -> Self
fn from(linter: Flake8GetText) -> Self
fn from(linter: Pylint) -> Self
fn from(linter: Flake82020) -> Self
fn from(linter: Flake8BooleanTrap) -> Self
fn from(linter: Flake8Pie) -> Self
fn from(linter: Flake8NoPep420) -> Self
fn from(linter: Flake8Annotations) -> Self
fn from(linter: Flake8Django) -> Self
fn from(linter: Flake8TidyImports) -> Self
fn from(linter: Flake8Debugger) -> Self
fn from(linter: Tryceratops) -> Self
fn from(linter: Pydoclint) -> Self
fn from(linter: Flake8Slots) -> Self
fn from(linter: Flake8Async) -> Self
fn from(linter: Flake8Quotes) -> Self
fn from(linter: Flake8UsePathlib) -> Self
fn from(linter: Flake8Bandit) -> Self
fn from(linter: Flake8Datetimez) -> Self
fn from(linter: Flake8Executable) -> Self
fn from(linter: Flake8PytestStyle) -> Self
fn from(linter: Flake8Print) -> Self
fn from(linter: Flake8Self) -> Self
fn from(linter: Flake8UnusedArguments) -> Self
fn from(linter: Flake8ErrMsg) -> Self
fn from(linter: Flake8TypeChecking) -> Self
fn from(linter: Refurb) -> Self
fn from(linter: Flake8Bugbear) -> Self
fn from(linter: Flake8Pyi) -> Self
fn from(linter: Ruff) -> Self
fn from(linter: Pycodestyle) -> Self
fn from(linter: Flake8LoggingFormat) -> Self
fn from(linter: Flake8Copyright) -> Self
fn from(linter: McCabe) -> Self
fn from(linter: PEP8Naming) -> Self
fn from(linter: PygrepHooks) -> Self
fn from(linter: Flake8ImplicitStrConcat) -> Self
fn from(linter: Perflint) -> Self
fn from(linter: Pyflakes) -> Self
fn from(linter: Flake8Builtins) -> Self
fn from(linter: Isort) -> Self
fn from(linter: Flake8Todos) -> Self
fn from(linter: Flake8Return) -> Self
fn from(linter: Pyupgrade) -> Self
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

---

## RuleSelectorIter

`enum` · `ruff_linter::rule_selector::RuleSelectorIter`

```rust
enum RuleSelectorIter
```

**Variants**: `All`, `Chain`, `Slice`, `Once`

**Implements**: `core::iter::traits::iterator::Iterator`

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

---

## Specificity

`enum` · `ruff_linter::rule_selector::Specificity`

```rust
enum Specificity
```

**Variants**: `All`, `Category`, `LinterGroup`, `Linter`, `Prefix1Char`, `Prefix2Chars`, `Prefix3Chars`, `Prefix4Chars`, `Rule`

**Implements**: `strum::IntoEnumIterator`

**Derives**: Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> SpecificityIter
```

---

## PreviewOptions

`struct` · `ruff_linter::rule_selector::PreviewOptions`

```rust
struct PreviewOptions
```

**Fields**: `mode`, `require_explicit`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

---

## RuleResolutionError

`struct` · `ruff_linter::rule_selector::RuleResolutionError`

```rust
struct RuleResolutionError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**Methods** (2)

```rust
fn log_warning(&self)
fn with_setting(self, setting: &'static str) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## SpecificityIter

`struct` · `ruff_linter::rule_selector::SpecificityIter`

```rust
struct SpecificityIter
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

An iterator over the variants of [Specificity]

---

## UnresolvedRuleSelector

`struct` · `ruff_linter::rule_selector::UnresolvedRuleSelector`

Also reachable as `ruff_linter::UnresolvedRuleSelector`

```rust
struct UnresolvedRuleSelector
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn cli(selector: impl Into<String>) -> Self
fn new(selector: impl Into<String>, source: ValueSource) -> Self
fn resolve(&self, preview: PreviewMode) -> Result<RuleSelector, RuleResolutionError>
fn source(&self) -> &ValueSource
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A potential rule selector that has not yet been validated and tracks its source.

If you add a new field that uses this type, be sure to update `rule-codes-in-selectors`
(`RUF201`) to validate the additional selector field.

---
