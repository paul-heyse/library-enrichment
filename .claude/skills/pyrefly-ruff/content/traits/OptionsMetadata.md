# OptionsMetadata

`ruff_options_metadata::OptionsMetadata`

```rust
trait OptionsMetadata
```

Prose: [`api/ruff_options_metadata.md`](../api/ruff_options_metadata.md#optionsmetadata) · records: [`model/ruff_options_metadata.json`](../model/ruff_options_metadata.json)

## Required

Every implementation must supply these.

```rust
fn record(visit: &mut dyn Visit)
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn documentation() -> Option<&'static str>
fn metadata() -> OptionSet where Self: Sized + 'static
```

## Implementors (34)

Read one before writing your own.

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

## Documentation

Returns metadata for its options.
