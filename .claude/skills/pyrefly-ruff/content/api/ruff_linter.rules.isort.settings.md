# `ruff_linter::rules::isort::settings`

Crate `ruff_linter` · 3 public items · structured records in [`model/ruff_linter.rules.isort.settings.json`](../model/ruff_linter.rules.isort.settings.json)

## RelativeImportsOrder

`enum` · `ruff_linter::rules::isort::settings::RelativeImportsOrder`

```rust
enum RelativeImportsOrder
```

**Variants**: `ClosestToFurthest`, `FurthestToClosest`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## SettingsError

`enum` · `ruff_linter::rules::isort::settings::SettingsError`

```rust
enum SettingsError
```

**Variants**: `InvalidKnownFirstParty`, `InvalidKnownThirdParty`, `InvalidKnownLocalFolder`, `InvalidExtraStandardLibrary`, `InvalidUserDefinedSection`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Error returned by the [`TryFrom`] implementation of [`Settings`].

---

## Settings

`struct` · `ruff_linter::rules::isort::settings::Settings`

```rust
struct Settings
```

**Fields**: `required_imports`, `combine_as_imports`, `force_single_line`, `force_sort_within_sections`, `case_sensitive`, `force_wrap_aliases`, `force_to_top`, `known_modules`, `detect_same_package`, `order_by_type`, `relative_imports_order`, `single_line_exclusions`, `split_on_trailing_comma`, `classes`, `constants`, `variables`, `no_lines_before`, `import_headings`, `lines_after_imports`, `lines_between_types`, `forced_separate`, `section_order`, `default_section`, `no_sections`, `from_first`, `length_sort`, `length_sort_straight`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---
