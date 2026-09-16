# `ruff_workspace::options`

Crate `ruff_workspace` · 36 public items · structured records in [`model/ruff_workspace.options.json`](../model/ruff_workspace.options.json)

## Alias

`struct` · `ruff_workspace::options::Alias`

```rust
struct Alias
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## AnalyzeOptions

`struct` · `ruff_workspace::options::AnalyzeOptions`

```rust
struct AnalyzeOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Configures Ruff's `analyze` command.

---

## DeprecatedTopLevelLintOptions

`struct` · `ruff_workspace::options::DeprecatedTopLevelLintOptions`

```rust
struct DeprecatedTopLevelLintOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn record(_visit: &mut dyn Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Newtype wrapper for [`LintCommonOptions`] that allows customizing the JSON schema and omitting the fields from the [`OptionsMetadata`].

---

## Flake8AnnotationsOptions

`struct` · `ruff_workspace::options::Flake8AnnotationsOptions`

```rust
struct Flake8AnnotationsOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-annotations` plugin.

---

## Flake8BanditOptions

`struct` · `ruff_workspace::options::Flake8BanditOptions`

```rust
struct Flake8BanditOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-bandit` plugin.

---

## Flake8BooleanTrapOptions

`struct` · `ruff_workspace::options::Flake8BooleanTrapOptions`

```rust
struct Flake8BooleanTrapOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-boolean-trap` plugin

---

## Flake8BugbearOptions

`struct` · `ruff_workspace::options::Flake8BugbearOptions`

```rust
struct Flake8BugbearOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-bugbear` plugin.

---

## Flake8BuiltinsOptions

`struct` · `ruff_workspace::options::Flake8BuiltinsOptions`

```rust
struct Flake8BuiltinsOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-builtins` plugin.

---

## Flake8ComprehensionsOptions

`struct` · `ruff_workspace::options::Flake8ComprehensionsOptions`

```rust
struct Flake8ComprehensionsOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-comprehensions` plugin.

---

## Flake8CopyrightOptions

`struct` · `ruff_workspace::options::Flake8CopyrightOptions`

```rust
struct Flake8CopyrightOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-copyright` plugin.

---

## Flake8ErrMsgOptions

`struct` · `ruff_workspace::options::Flake8ErrMsgOptions`

```rust
struct Flake8ErrMsgOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-errmsg` plugin.

---

## Flake8GetTextOptions

`struct` · `ruff_workspace::options::Flake8GetTextOptions`

```rust
struct Flake8GetTextOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-gettext` plugin.

---

## Flake8ImplicitStrConcatOptions

`struct` · `ruff_workspace::options::Flake8ImplicitStrConcatOptions`

```rust
struct Flake8ImplicitStrConcatOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-implicit-str-concat` plugin

---

## Flake8ImportConventionsOptions

`struct` · `ruff_workspace::options::Flake8ImportConventionsOptions`

```rust
struct Flake8ImportConventionsOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-import-conventions` plugin

---

## Flake8PytestStyleOptions

`struct` · `ruff_workspace::options::Flake8PytestStyleOptions`

```rust
struct Flake8PytestStyleOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-pytest-style` plugin

---

## Flake8QuotesOptions

`struct` · `ruff_workspace::options::Flake8QuotesOptions`

```rust
struct Flake8QuotesOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-quotes` plugin.

---

## Flake8SelfOptions

`struct` · `ruff_workspace::options::Flake8SelfOptions`

```rust
struct Flake8SelfOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8_self` plugin.

---

## Flake8TidyImportsOptions

`struct` · `ruff_workspace::options::Flake8TidyImportsOptions`

```rust
struct Flake8TidyImportsOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-tidy-imports` plugin

---

## Flake8TypeCheckingOptions

`struct` · `ruff_workspace::options::Flake8TypeCheckingOptions`

```rust
struct Flake8TypeCheckingOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-type-checking` plugin

---

## Flake8UnusedArgumentsOptions

`struct` · `ruff_workspace::options::Flake8UnusedArgumentsOptions`

```rust
struct Flake8UnusedArgumentsOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `flake8-unused-arguments` plugin

---

## FormatOptions

`struct` · `ruff_workspace::options::FormatOptions`

```rust
struct FormatOptions
```

**Fields**: `exclude`, `preview`, `indent_style`, `quote_style`, `nested_string_quote_style`, `skip_magic_trailing_comma`, `line_ending`, `docstring_code_format`, `docstring_code_line_length`

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Configures the way Ruff formats your code.

---

## IsortOptions

`struct` · `ruff_workspace::options::IsortOptions`

```rust
struct IsortOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `isort` plugin.

---

## LintCommonOptions

`struct` · `ruff_workspace::options::LintCommonOptions`

```rust
struct LintCommonOptions
```

**Fields**: `allowed_confusables`, `dummy_variable_rgx`, `extend_ignore`, `extend_select`, `extend_fixable`, `extend_unfixable`, `external`, `fixable`, `ignore`, `extend_safe_fixes`, `extend_unsafe_fixes`, `ignore_init_module_imports`, `logger_objects`, `select`, `explicit_preview_rules`, `task_tags`, `typing_modules`, `unfixable`, `flake8_annotations`, `flake8_bandit`, `flake8_boolean_trap`, `flake8_bugbear`, `flake8_builtins`, `flake8_comprehensions`, `flake8_copyright`, `flake8_errmsg`, `flake8_quotes`, `flake8_self`, `flake8_tidy_imports`, `flake8_type_checking`, `flake8_gettext`, `flake8_implicit_str_concat`, `flake8_import_conventions`, `flake8_pytest_style`, `flake8_unused_arguments`, `isort`, `mccabe`, `pep8_naming`, `pycodestyle`, `pydocstyle`, `pyflakes`, `pylint`, `pyupgrade`, `per_file_ignores`, `extend_per_file_ignores`

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn record(visit: &mut dyn ruff_options_metadata::Visit)
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

## LintOptions

`struct` · `ruff_workspace::options::LintOptions`

```rust
struct LintOptions
```

**Fields**: `common`, `exclude`, `pydoclint`, `ruff`, `preview`, `typing_extensions`, `future_annotations`

**Implements**: `core::convert::From`, `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: LintOptionsWire) -> LintOptions
```

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Configures how Ruff checks your code.

Options specified in the `lint` section take precedence over the deprecated top-level settings.

---

## LintOptionsWire

`struct` · `ruff_workspace::options::LintOptionsWire`

```rust
struct LintOptionsWire
```

**Implements**: `serde_core::de::Deserialize`

**Derives**: Clone, Debug, Default

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Like [`LintCommonOptions`], but with any `#[serde(flatten)]` fields inlined. This leads to far,
far better error messages when deserializing.

---

## McCabeOptions

`struct` · `ruff_workspace::options::McCabeOptions`

```rust
struct McCabeOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `mccabe` plugin.

---

## ModuleName

`struct` · `ruff_workspace::options::ModuleName`

```rust
struct ModuleName
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Options

`struct` · `ruff_workspace::options::Options`

```rust
struct Options
```

**Fields**: `cache_dir`, `extend`, `output_format`, `output_prefer_rule_codes`, `fix`, `unsafe_fixes`, `fix_only`, `show_fixes`, `required_version`, `preview`, `exclude`, `extend_exclude`, `extend_include`, `force_exclude`, `include`, `respect_gitignore`, `extension`, `builtins`, `namespace_packages`, `target_version`, `per_file_target_version`, `src`, `line_length`, `indent_width`, `lint`, `lint_top_level`, `format`, `analyze`

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn from_toml_table(table: toml::Table) -> Result<Self, toml::de::Error>
```

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn record(visit: &mut dyn ruff_options_metadata::Visit)
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

## Pep8NamingOptions

`struct` · `ruff_workspace::options::Pep8NamingOptions`

```rust
struct Pep8NamingOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `pep8-naming` plugin.

---

## PyUpgradeOptions

`struct` · `ruff_workspace::options::PyUpgradeOptions`

```rust
struct PyUpgradeOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `pyupgrade` plugin.

---

## PycodestyleOptions

`struct` · `ruff_workspace::options::PycodestyleOptions`

```rust
struct PycodestyleOptions
```

**Fields**: `max_line_length`, `max_doc_length`, `ignore_overlong_task_comments`

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `pycodestyle` plugin.

---

## PydoclintOptions

`struct` · `ruff_workspace::options::PydoclintOptions`

```rust
struct PydoclintOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `pydoclint` plugin.

---

## PydocstyleOptions

`struct` · `ruff_workspace::options::PydocstyleOptions`

```rust
struct PydocstyleOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `pydocstyle` plugin.

---

## PyflakesOptions

`struct` · `ruff_workspace::options::PyflakesOptions`

```rust
struct PyflakesOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `pyflakes` plugin.

---

## PylintOptions

`struct` · `ruff_workspace::options::PylintOptions`

```rust
struct PylintOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `pylint` plugin.

---

## RuffOptions

`struct` · `ruff_workspace::options::RuffOptions`

```rust
struct RuffOptions
```

**Implements**: `ruff_options_metadata::OptionsMetadata`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_options_metadata::OptionsMetadata`**

```rust
fn documentation() -> Option<&'static str>
fn record(visit: &mut dyn ruff_options_metadata::Visit)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for the `ruff` plugin

---
