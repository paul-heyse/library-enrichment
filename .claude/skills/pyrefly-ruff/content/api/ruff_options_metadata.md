# `ruff_options_metadata`

Crate `ruff_options_metadata` · 6 public items · structured records in [`model/ruff_options_metadata.json`](../model/ruff_options_metadata.json)

## OptionEntry

`enum` · `ruff_options_metadata::OptionEntry`

```rust
enum OptionEntry
```

**Variants**: `Field`, `Set`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn into_field(self) -> Option<OptionField>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Metadata of an option that can either be a [`OptionField`] or [`OptionSet`].

---

## Deprecated

`struct` · `ruff_options_metadata::Deprecated`

```rust
struct Deprecated
```

**Fields**: `since`, `message`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## OptionField

`struct` · `ruff_options_metadata::OptionField`

```rust
struct OptionField
```

**Fields**: `doc`, `default`, `value_type`, `scope`, `example`, `deprecated`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

---

## OptionSet

`struct` · `ruff_options_metadata::OptionSet`

```rust
struct OptionSet
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug

**Methods** (6)

```rust
fn collect_fields(&self) -> Vec<(String, OptionField)>
fn documentation(&self) -> Option<&'static str>
fn find(&self, name: &str) -> Option<OptionEntry>
fn has(&self, name: &str) -> bool
fn of<T>() -> Self where T: OptionsMetadata + 'static
fn record(&self, visit: &mut dyn Visit)
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

A set of options.

It extracts the options by calling the [`OptionsMetadata::record`] of a type implementing
[`OptionsMetadata`].

---

## OptionsMetadata

`trait` · `ruff_options_metadata::OptionsMetadata`

```rust
trait OptionsMetadata
```

**Implementors** (34)

- `core::option::Option`
- `ruff_workspace::options::AnalyzeOptions`
- `ruff_workspace::options::DeprecatedTopLevelLintOptions`
- `ruff_workspace::options::Flake8AnnotationsOptions`
- `ruff_workspace::options::Flake8BanditOptions`
- `ruff_workspace::options::Flake8BooleanTrapOptions`
- `ruff_workspace::options::Flake8BugbearOptions`
- `ruff_workspace::options::Flake8BuiltinsOptions`
- `ruff_workspace::options::Flake8ComprehensionsOptions`
- `ruff_workspace::options::Flake8CopyrightOptions`
- `ruff_workspace::options::Flake8ErrMsgOptions`
- `ruff_workspace::options::Flake8GetTextOptions`
- `ruff_workspace::options::Flake8ImplicitStrConcatOptions`
- `ruff_workspace::options::Flake8ImportConventionsOptions`
- `ruff_workspace::options::Flake8PytestStyleOptions`
- `ruff_workspace::options::Flake8QuotesOptions`
- `ruff_workspace::options::Flake8SelfOptions`
- `ruff_workspace::options::Flake8TidyImportsOptions`
- `ruff_workspace::options::Flake8TypeCheckingOptions`
- `ruff_workspace::options::Flake8UnusedArgumentsOptions`
- `ruff_workspace::options::FormatOptions`
- `ruff_workspace::options::IsortOptions`
- `ruff_workspace::options::LintCommonOptions`
- `ruff_workspace::options::LintOptions`
- `ruff_workspace::options::McCabeOptions`
- `ruff_workspace::options::Options`
- `ruff_workspace::options::Pep8NamingOptions`
- `ruff_workspace::options::PyUpgradeOptions`
- `ruff_workspace::options::PycodestyleOptions`
- `ruff_workspace::options::PydoclintOptions`
- `ruff_workspace::options::PydocstyleOptions`
- `ruff_workspace::options::PyflakesOptions`
- `ruff_workspace::options::PylintOptions`
- `ruff_workspace::options::RuffOptions`

**Methods** (3)

```rust
fn documentation() -> Option<&'static str>
fn metadata() -> OptionSet where Self: Sized + 'static
fn record(visit: &mut dyn Visit)
```

Returns metadata for its options.

---

## Visit

`trait` · `ruff_options_metadata::Visit`

```rust
trait Visit
```

**Methods** (2)

```rust
fn record_field(&mut self, name: &str, field: OptionField)
fn record_set(&mut self, name: &str, group: OptionSet)
```

Visits [`OptionsMetadata`].

An instance of [`Visit`] represents the logic for inspecting an object's options metadata.

---
