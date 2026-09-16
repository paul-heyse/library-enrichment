# `ruff_linter::settings::types`

Crate `ruff_linter` · 23 public items · structured records in [`model/ruff_linter.settings.types.json`](../model/ruff_linter.settings.types.json)

## FilePattern

`enum` · `ruff_linter::settings::types::FilePattern`

```rust
enum FilePattern
```

**Variants**: `Builtin`, `Config`, `User`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn add_to(self, builder: &mut GlobSetBuilder) -> Result<()>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---

## IdentifierPattern

`enum` · `ruff_linter::settings::types::IdentifierPattern`

```rust
enum IdentifierPattern
```

**Variants**: `Literal`, `Glob`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(pattern: &str) -> Result<Self, glob::PatternError>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(pattern: &str) -> Result<Self, Self::Err>
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

Pattern to match an identifier.

# Notes

[`glob::Pattern`] matches a little differently than we ideally want to.
Specifically it uses `**` to match an arbitrary number of subdirectories,
luckily this not relevant since identifiers don't contains slashes.

For reference pep8-naming uses
[`fnmatch`](https://docs.python.org/3/library/fnmatch.html) for
pattern matching.

Literal patterns without glob metacharacters fall back on string
comparison to avoid the overhead of constructing a [`glob::Pattern`].

---

## Language

`enum` · `ruff_linter::settings::types::Language`

```rust
enum Language
```

**Variants**: `Python`, `Pyi`, `Ipynb`, `Markdown`

**Implements**: `core::str::traits::FromStr`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `strum::IntoEnumIterator`

**Derives**: Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
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

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> LanguageIter
```

---

## OutputFormat

`enum` · `ruff_linter::settings::types::OutputFormat`

```rust
enum OutputFormat
```

**Variants**: `Concise`, `Full`, `Json`, `JsonLines`, `Junit`, `Grouped`, `Github`, `Gitlab`, `Pylint`, `Rdjson`, `Azure`, `Sarif`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn is_human_readable(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
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

## PreviewMode

`enum` · `ruff_linter::settings::types::PreviewMode`

```rust
enum PreviewMode
```

**Variants**: `Disabled`, `Enabled`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
const fn is_disabled(&self) -> bool
const fn is_enabled(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(version: bool) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---

## PythonVersion

`enum` · `ruff_linter::settings::types::PythonVersion`

```rust
enum PythonVersion
```

**Variants**: `Py37`, `Py38`, `Py39`, `Py310`, `Py311`, `Py312`, `Py313`, `Py314`, `Py315`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `strum::IntoEnumIterator`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn as_tuple(&self) -> (u8, u8)
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: ast::PythonVersion) -> Result<Self, Self::Error>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> PythonVersionIter
```

---

## RuffOutputFormat

`enum` · `ruff_linter::settings::types::RuffOutputFormat`

```rust
enum RuffOutputFormat
```

**Variants**: `Grouped`, `Sarif`

The subset of output formats only implemented in Ruff, not in `ruff_db` via `DisplayDiagnostics`.

---

## UnsafeFixes

`enum` · `ruff_linter::settings::types::UnsafeFixes`

```rust
enum UnsafeFixes
```

**Variants**: `Hint`, `Disabled`, `Enabled`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
const fn is_disabled(&self) -> bool
const fn is_enabled(&self) -> bool
const fn is_hint(&self) -> bool
fn required_applicability(&self) -> Applicability
```

**via `core::convert::From`**

```rust
fn from(value: bool) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

Toggle for unsafe fixes.
`Hint` will not apply unsafe fixes but a message will be shown when they are available.
`Disabled` will not apply unsafe fixes or show a message.
`Enabled` will apply unsafe fixes.

---

## CompiledPerFile

`struct` · `ruff_linter::settings::types::CompiledPerFile`

```rust
struct CompiledPerFile<T>
```

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, state: &mut CacheKeyHasher)
```

Like [`PerFile`] but with string globs compiled to [`GlobMatcher`]s for more efficient usage.

---

## CompiledPerFileIgnoreList

`struct` · `ruff_linter::settings::types::CompiledPerFileIgnoreList`

```rust
struct CompiledPerFileIgnoreList
```

**Implements**: `core::fmt::Display`, `core::ops::deref::Deref`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn resolve(per_file_ignores: Vec<PerFileIgnore>, preview: PreviewMode) -> Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---

## CompiledPerFileList

`struct` · `ruff_linter::settings::types::CompiledPerFileList`

```rust
struct CompiledPerFileList<T>
```

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, state: &mut CacheKeyHasher)
```

A sequence of [`CompiledPerFile<T>`].

---

## CompiledPerFileTargetVersionList

`struct` · `ruff_linter::settings::types::CompiledPerFileTargetVersionList`

```rust
struct CompiledPerFileTargetVersionList
```

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn is_match(&self, path: &Path) -> Option<ast::PythonVersion>
fn resolve(per_file_versions: Vec<PerFileTargetVersion>) -> Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---

## ExtensionMapping

`struct` · `ruff_linter::settings::types::ExtensionMapping`

```rust
struct ExtensionMapping
```

**Implements**: `core::convert::From`, `core::iter::traits::collect::FromIterator`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**Methods** (3)

```rust
fn extensions(&self) -> impl Iterator<Item = &String>
fn get_source_type(&self, path: &Path) -> SourceType
fn get_source_type_by_extension(&self, ext: &str) -> SourceType
```

**via `core::convert::From`**

```rust
fn from(value: FxHashMap<String, Language>) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = ExtensionPair>>(iter: T) -> Self
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---

## ExtensionPair

`struct` · `ruff_linter::settings::types::ExtensionPair`

```rust
struct ExtensionPair
```

**Fields**: `extension`, `language`

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> std::result::Result<Self, Self::Err>
```

---

## FilePatternSet

`struct` · `ruff_linter::settings::types::FilePatternSet`

```rust
struct FilePatternSet
```

**Implements**: `core::fmt::Display`, `core::ops::deref::Deref`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn try_from_iter<I>(patterns: I) -> Result<Self, anyhow::Error> where I: IntoIterator<Item = FilePattern>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, state: &mut CacheKeyHasher)
```

---

## GlobPath

`struct` · `ruff_linter::settings::types::GlobPath`

```rust
struct GlobPath
```

**Implements**: `core::ops::deref::Deref`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn normalize(path: impl AsRef<Path>, root: impl AsRef<Path>) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

Represents a path to be passed to [`Glob::new`].

---

## LanguageIter

`struct` · `ruff_linter::settings::types::LanguageIter`

```rust
struct LanguageIter
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

An iterator over the variants of [Language]

---

## PatternPrefixPair

`struct` · `ruff_linter::settings::types::PatternPrefixPair`

```rust
struct PatternPrefixPair
```

**Fields**: `pattern`, `prefix`

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

---

## PerFile

`struct` · `ruff_linter::settings::types::PerFile`

```rust
struct PerFile<T>
```

**Derives**: Clone, Debug

A glob pattern and associated data for matching file paths.

---

## PerFileIgnore

`struct` · `ruff_linter::settings::types::PerFileIgnore`

```rust
struct PerFileIgnore
```

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(pattern: String, selectors: Vec<UnresolvedRuleSelector>, project_root: Option<&Path>) -> Self
```

Per-file ignored linting rules.

See [`PerFile`] for details of the representation.

---

## PerFileTargetVersion

`struct` · `ruff_linter::settings::types::PerFileTargetVersion`

```rust
struct PerFileTargetVersion
```

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(pattern: String, version: ast::PythonVersion, project_root: Option<&Path>) -> Self
```

Contains the target Python version for a given glob pattern.

See [`PerFile`] for details of the representation.

---

## PythonVersionIter

`struct` · `ruff_linter::settings::types::PythonVersionIter`

```rust
struct PythonVersionIter
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

An iterator over the variants of [PythonVersion]

---

## RequiredVersion

`struct` · `ruff_linter::settings::types::RequiredVersion`

```rust
struct RequiredVersion
```

**Implements**: `core::convert::TryFrom`, `core::fmt::Display`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn contains(&self, version: &pep440_rs::Version) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: String) -> Result<Self, Self::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(value: &str) -> Result<Self, Self::Err>
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
