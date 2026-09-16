# `pyrefly_config::base`

Crate `pyrefly_config` · 6 public items · structured records in [`model/pyrefly_config.base.json`](../model/pyrefly_config.base.json)

## InferReturnTypes

`enum` · `pyrefly_config::base::InferReturnTypes`

```rust
enum InferReturnTypes
```

**Variants**: `Never`, `Annotated`, `Checked`

**Implements**: `clap_builder::derive::ValueEnum`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `clap_builder::derive::ValueEnum`**

```rust
fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>
fn value_variants<'a>() -> &'a [Self]
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Controls when Pyrefly infers return types for functions without explicit return annotations.

---

## Preset

`enum` · `pyrefly_config::base::Preset`

```rust
enum Preset
```

**Variants**: `Off`, `Basic`, `Legacy`, `Default`, `Strict`, `All`

**Implements**: `clap_builder::derive::ValueEnum`, `enum_iterator::Sequence`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn apply(self) -> ConfigBase
```

**via `clap_builder::derive::ValueEnum`**

```rust
fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>
fn value_variants<'a>() -> &'a [Self]
```

**via `enum_iterator::Sequence`**

```rust
fn first() -> ::core::option::Option<Self>
fn last() -> ::core::option::Option<Self>
fn next(&self) -> ::core::option::Option<Self>
fn previous(&self) -> ::core::option::Option<Self>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A named collection of error severities and behavior settings that serves as
the base configuration. User-specified settings merge on top, overriding
the preset. Explicit configuration always wins over the preset regardless
of order in the config file.

---

## RecursionOverflowHandler

`enum` · `pyrefly_config::base::RecursionOverflowHandler`

```rust
enum RecursionOverflowHandler
```

**Variants**: `BreakWithPlaceholder`, `PanicWithDebugInfo`

**Implements**: `clap_builder::derive::ValueEnum`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `clap_builder::derive::ValueEnum`**

```rust
fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>
fn value_variants<'a>() -> &'a [Self]
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

How to handle when recursion depth limit is exceeded.

---

## UntypedDefBehavior

`enum` · `pyrefly_config::base::UntypedDefBehavior`

```rust
enum UntypedDefBehavior
```

**Variants**: `CheckAndInferReturnType`, `CheckAndInferReturnAny`, `SkipAndInferReturnAny`

**Implements**: `clap_builder::derive::ValueEnum`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `clap_builder::derive::ValueEnum`**

```rust
fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>
fn value_variants<'a>() -> &'a [Self]
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

## ConfigBase

`struct` · `pyrefly_config::base::ConfigBase`

```rust
struct ConfigBase
```

**Fields**: `errors`, `permissive_ignores`, `enabled_ignores`, `type_ignore_unknown_tag_behavior`, `untyped_def_behavior`, `check_unannotated_defs`, `infer_return_types`, `disable_type_errors_in_ide`, `ignore_errors_in_generated_code`, `infer_with_first_use`, `pytorch_efficiency_lints`, `recursion_depth_limit`, `recursion_overflow_handler`, `strict_callable_subtyping`, `strict_partial_subtyping`, `spec_compliant_overloads`, `legacy_overload_expansion`, `treat_all_caps_as_final`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (16)

```rust
fn default_for_ide_without_config() -> Self
fn get_check_unannotated_defs(base: &Self) -> Option<bool>
fn get_disable_type_errors_in_ide(base: &Self) -> Option<bool>
fn get_enabled_ignores(base: &Self) -> Option<&SmallSet<Tool>>
fn get_errors(base: &Self) -> Option<&ErrorDisplayConfig>
fn get_ignore_errors_in_generated_code(base: &Self) -> Option<bool>
fn get_infer_return_types(base: &Self) -> Option<InferReturnTypes>
fn get_infer_with_first_use(base: &Self) -> Option<bool>
fn get_legacy_overload_expansion(base: &Self) -> Option<bool>
fn get_recursion_limit_config(base: &Self) -> Option<RecursionLimitConfig>
fn get_spec_compliant_overloads(base: &Self) -> Option<bool>
fn get_strict_callable_subtyping(base: &Self) -> Option<bool>
fn get_strict_partial_subtyping(base: &Self) -> Option<bool>
fn get_treat_all_caps_as_final(base: &Self) -> Option<bool>
fn get_type_ignore_unknown_tag_behavior(base: &Self) -> Option<TypeIgnoreUnknownTagBehavior>
fn resolve_legacy_settings(&mut self)
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

## RecursionLimitConfig

`struct` · `pyrefly_config::base::RecursionLimitConfig`

```rust
struct RecursionLimitConfig
```

**Fields**: `limit`, `handler`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Internal configuration struct combining depth limit and handler.
Not serialized directly - constructed from flat config fields.

---
