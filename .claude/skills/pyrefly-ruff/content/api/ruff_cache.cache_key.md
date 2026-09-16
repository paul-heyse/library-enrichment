# `ruff_cache::cache_key`

Crate `ruff_cache` · 2 public items · structured records in [`model/ruff_cache.cache_key.json`](../model/ruff_cache.cache_key.json)

## CacheKeyHasher

`struct` · `ruff_cache::cache_key::CacheKeyHasher`

Also reachable as `ruff_cache::CacheKeyHasher`

```rust
struct CacheKeyHasher
```

**Implements**: `core::hash::Hasher`

**Derives**: Clone, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `core::hash::Hasher`**

```rust
fn finish(&self) -> u64
fn write(&mut self, bytes: &[u8])
fn write_i128(&mut self, i: i128)
fn write_i16(&mut self, i: i16)
fn write_i32(&mut self, i: i32)
fn write_i64(&mut self, i: i64)
fn write_i8(&mut self, i: i8)
fn write_isize(&mut self, i: isize)
fn write_u128(&mut self, i: u128)
fn write_u16(&mut self, i: u16)
fn write_u32(&mut self, i: u32)
fn write_u64(&mut self, i: u64)
fn write_u8(&mut self, i: u8)
fn write_usize(&mut self, i: usize)
```

---

## CacheKey

`trait` · `ruff_cache::cache_key::CacheKey`

Also reachable as `ruff_cache::CacheKey`

```rust
trait CacheKey
```

**Implementors** (110)

- `alloc::borrow::Cow`
- `alloc::collections::btree::map::BTreeMap`
- `alloc::collections::btree::set::BTreeSet`
- `alloc::string::String`
- `alloc::vec::Vec`
- `char_str::char_str::CharStr`
- `core::num::nonzero::NonZeroI128`
- `core::num::nonzero::NonZeroI16`
- `core::num::nonzero::NonZeroI32`
- `core::num::nonzero::NonZeroI64`
- `core::num::nonzero::NonZeroI8`
- `core::num::nonzero::NonZeroU128`
- `core::num::nonzero::NonZeroU16`
- `core::num::nonzero::NonZeroU32`
- `core::num::nonzero::NonZeroU64`
- `core::num::nonzero::NonZeroU8`
- `core::option::Option`
- `filetime::FileTime`
- `glob::Pattern`
- `globset::glob::Glob`
- `globset::glob::GlobMatcher`
- `regex::regex::string::Regex`
- `ruff_formatter::IndentStyle`
- `ruff_formatter::IndentWidth`
- `ruff_formatter::LineWidth`
- `ruff_graph::settings::AnalyzeSettings`
- `ruff_graph::settings::Direction`
- `ruff_graph::settings::StringImports`
- `ruff_linter::line_width::IndentWidth`
- `ruff_linter::line_width::LineLength`
- `ruff_linter::registry::rule_set::RuleSet`
- `ruff_linter::rules::flake8_annotations::settings::Settings`
- `ruff_linter::rules::flake8_bandit::settings::Settings`
- `ruff_linter::rules::flake8_boolean_trap::settings::Settings`
- `ruff_linter::rules::flake8_bugbear::settings::Settings`
- `ruff_linter::rules::flake8_builtins::settings::Settings`
- `ruff_linter::rules::flake8_comprehensions::settings::Settings`
- `ruff_linter::rules::flake8_copyright::settings::Settings`
- `ruff_linter::rules::flake8_errmsg::settings::Settings`
- `ruff_linter::rules::flake8_gettext::settings::Settings`
- `ruff_linter::rules::flake8_implicit_str_concat::settings::Settings`
- `ruff_linter::rules::flake8_import_conventions::settings::BannedAliases`
- `ruff_linter::rules::flake8_import_conventions::settings::Settings`
- `ruff_linter::rules::flake8_pytest_style::settings::Settings`
- `ruff_linter::rules::flake8_pytest_style::types::ParametrizeNameType`
- `ruff_linter::rules::flake8_pytest_style::types::ParametrizeValuesRowType`
- `ruff_linter::rules::flake8_pytest_style::types::ParametrizeValuesType`
- `ruff_linter::rules::flake8_quotes::settings::Quote`
- `ruff_linter::rules::flake8_quotes::settings::Settings`
- `ruff_linter::rules::flake8_self::settings::Settings`
- `ruff_linter::rules::flake8_tidy_imports::settings::AllImports`
- `ruff_linter::rules::flake8_tidy_imports::settings::ApiBan`
- `ruff_linter::rules::flake8_tidy_imports::settings::ImportSelection`
- `ruff_linter::rules::flake8_tidy_imports::settings::ImportSelector`
- `ruff_linter::rules::flake8_tidy_imports::settings::ImportSelectorSettings`
- `ruff_linter::rules::flake8_tidy_imports::settings::Settings`
- `ruff_linter::rules::flake8_tidy_imports::settings::Strictness`
- `ruff_linter::rules::flake8_type_checking::settings::Settings`
- `ruff_linter::rules::flake8_unused_arguments::settings::Settings`
- `ruff_linter::rules::isort::categorize::ImportSection`
- `ruff_linter::rules::isort::categorize::ImportType`
- `ruff_linter::rules::isort::categorize::KnownModules`
- `ruff_linter::rules::isort::settings::RelativeImportsOrder`
- `ruff_linter::rules::isort::settings::Settings`
- `ruff_linter::rules::mccabe::settings::Settings`
- `ruff_linter::rules::pep8_naming::settings::IgnoreNames`
- `ruff_linter::rules::pep8_naming::settings::Settings`
- `ruff_linter::rules::pycodestyle::settings::Settings`
- `ruff_linter::rules::pydoclint::settings::Settings`
- `ruff_linter::rules::pydocstyle::settings::Convention`
- `ruff_linter::rules::pydocstyle::settings::Settings`
- `ruff_linter::rules::pyflakes::settings::Settings`
- `ruff_linter::rules::pylint::settings::ConstantType`
- `ruff_linter::rules::pylint::settings::Settings`
- `ruff_linter::rules::pyupgrade::settings::Settings`
- `ruff_linter::rules::ruff::settings::Settings`
- `ruff_linter::settings::LinterSettings`
- `ruff_linter::settings::TargetVersion`
- `ruff_linter::settings::fix_safety_table::FixSafetyTable`
- `ruff_linter::settings::rule_table::RuleTable`
- `ruff_linter::settings::types::CompiledPerFile`
- `ruff_linter::settings::types::CompiledPerFileIgnoreList`
- `ruff_linter::settings::types::CompiledPerFileList`
- `ruff_linter::settings::types::CompiledPerFileTargetVersionList`
- `ruff_linter::settings::types::ExtensionMapping`
- `ruff_linter::settings::types::FilePattern`
- `ruff_linter::settings::types::FilePatternSet`
- `ruff_linter::settings::types::GlobPath`
- `ruff_linter::settings::types::IdentifierPattern`
- `ruff_linter::settings::types::Language`
- `ruff_linter::settings::types::PreviewMode`
- `ruff_linter::settings::types::UnsafeFixes`
- `ruff_python_formatter::options::DocstringCode`
- `ruff_python_formatter::options::DocstringCodeLineWidth`
- `ruff_python_formatter::options::MagicTrailingComma`
- `ruff_python_formatter::options::NestedStringQuoteStyle`
- `ruff_python_formatter::options::PreviewMode`
- `ruff_python_formatter::options::QuoteStyle`
- `ruff_python_semantic::imports::Alias`
- `ruff_python_semantic::imports::MemberNameImport`
- `ruff_python_semantic::imports::ModuleNameImport`
- `ruff_python_semantic::imports::NameImport`
- `ruff_python_semantic::imports::NameImports`
- `ruff_workspace::settings::FileResolverSettings`
- `ruff_workspace::settings::FormatterSettings`
- `ruff_workspace::settings::Settings`
- `std::collections::hash::map::HashMap`
- `std::collections::hash::set::HashSet`
- `std::path::Path`
- `std::path::PathBuf`

**Methods** (2)

```rust
fn cache_key(&self, state: &mut CacheKeyHasher)
fn cache_key_slice(data: &[Self], state: &mut CacheKeyHasher) where Self: Sized
```

A type that be used as part of a cache key.

A cache looks up artefacts by a cache key. Many cache keys are composed of sub-keys. For example,
caching the lint results of a file depend at least on the file content, the user settings, and linter version.
Types implementing the [`CacheKey`] trait can be used as part of a cache key by which artefacts are queried.

## Implementing `CacheKey`

You can derive [`CacheKey`] with `#[derive(CacheKey)]` if all fields implement [`CacheKey`]. The resulting
cache key will be the combination of the values from calling `cache_key` on each field.

```
# use ruff_macros::CacheKey;

#[derive(CacheKey)]
struct Test {
    name: String,
    version: u32,
}
```

If you need more control over computing the cache key, you can of course implement the [`CacheKey]` yourself:

```
 use ruff_cache::{CacheKey, CacheKeyHasher};

struct Test {
    name: String,
    version: u32,
    other: String
}

impl CacheKey for Test {
    fn cache_key(&self, state: &mut CacheKeyHasher) {
        self.name.cache_key(state);
        self.version.cache_key(state);
    }
}
```

## Portability

Ideally, the cache key is portable across platforms but this is not yet a strict requirement.

## Using [`Hash`]

You can defer to the [`Hash`] implementation for non-composite types.
Be aware, that the [`Hash`] implementation may not be portable.

## Why a new trait rather than reusing [`Hash`]?
The main reason is that hashes and cache keys have different constraints:

* Cache keys are less performance sensitive: Hashes must be super fast to compute for performant hashed-collections. That's
  why some standard types don't implement [`Hash`] where it would be safe to implement [`CacheKey`], e.g. `HashSet`
* Cache keys must be deterministic where hash keys do not have this constraint. That's why pointers don't implement [`CacheKey`] but they implement [`Hash`].
* Ideally, cache keys are portable

[`Hash`](Hash)

---
