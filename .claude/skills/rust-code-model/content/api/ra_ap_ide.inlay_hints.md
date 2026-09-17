# `ra_ap_ide::inlay_hints`

Crate `ra_ap_ide` · 16 public items · structured records in [`model/ra_ap_ide.inlay_hints.json`](../model/ra_ap_ide.inlay_hints.json)

## AdjustmentHints

`enum` · `ra_ap_ide::inlay_hints::AdjustmentHints`

Also reachable as `ra_ap_ide::AdjustmentHints`

```rust
enum AdjustmentHints
```

**Variants**: `Always`, `BorrowsOnly`, `Never`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## AdjustmentHintsMode

`enum` · `ra_ap_ide::inlay_hints::AdjustmentHintsMode`

Also reachable as `ra_ap_ide::AdjustmentHintsMode`

```rust
enum AdjustmentHintsMode
```

**Variants**: `Prefix`, `Postfix`, `PreferPrefix`, `PreferPostfix`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## ClosureReturnTypeHints

`enum` · `ra_ap_ide::inlay_hints::ClosureReturnTypeHints`

Also reachable as `ra_ap_ide::ClosureReturnTypeHints`

```rust
enum ClosureReturnTypeHints
```

**Variants**: `Always`, `WithBlock`, `Never`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## DiscriminantHints

`enum` · `ra_ap_ide::inlay_hints::DiscriminantHints`

Also reachable as `ra_ap_ide::DiscriminantHints`

```rust
enum DiscriminantHints
```

**Variants**: `Always`, `Never`, `Fieldless`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## InlayHintPosition

`enum` · `ra_ap_ide::inlay_hints::InlayHintPosition`

Also reachable as `ra_ap_ide::InlayHintPosition`

```rust
enum InlayHintPosition
```

**Variants**: `Before`, `After`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Debug, Hash

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, _analysis: &ra_fixture::RaFixtureAnalysis, _virtual_file_id: ra_fixture::FileId, _real_file_id: ra_fixture::FileId) -> Result<Self, ()>
```

---

## InlayKind

`enum` · `ra_ap_ide::inlay_hints::InlayKind`

Also reachable as `ra_ap_ide::InlayKind`

```rust
enum InlayKind
```

**Variants**: `Adjustment`, `BindingMode`, `Chaining`, `ClosingBrace`, `ClosureCapture`, `Discriminant`, `GenericParamList`, `Lifetime`, `Parameter`, `GenericParameter`, `Type`, `Dyn`, `Drop`, `RangeExclusive`, `ExternUnsafety`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, _analysis: &ra_fixture::RaFixtureAnalysis, _virtual_file_id: ra_fixture::FileId, _real_file_id: ra_fixture::FileId) -> Result<Self, ()>
```

---

## InlayTooltip

`enum` · `ra_ap_ide::inlay_hints::InlayTooltip`

Also reachable as `ra_ap_ide::InlayTooltip`

```rust
enum InlayTooltip
```

**Variants**: `String`, `Markdown`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Debug, Hash

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, _analysis: &ra_fixture::RaFixtureAnalysis, _virtual_file_id: ra_fixture::FileId, _real_file_id: ra_fixture::FileId) -> Result<Self, ()>
```

---

## LazyProperty

`enum` · `ra_ap_ide::inlay_hints::LazyProperty`

Also reachable as `ra_ap_ide::LazyProperty`

```rust
enum LazyProperty<T>
```

**Variants**: `Computed`, `Lazy`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn computed(self) -> Option<T>
fn is_lazy(&self) -> bool
```

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, __analysis: &::ide_db::ra_fixture::RaFixtureAnalysis, __virtual_file_id: ::ide_db::ra_fixture::FileId, __real_file_id: ::ide_db::ra_fixture::FileId) -> Result<Self, ()>
```

A type signaling that a value is either computed, or is available for computation.

---

## LifetimeElisionHints

`enum` · `ra_ap_ide::inlay_hints::LifetimeElisionHints`

Also reachable as `ra_ap_ide::LifetimeElisionHints`

```rust
enum LifetimeElisionHints
```

**Variants**: `Always`, `SkipTrivial`, `Never`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## TypeHintsPlacement

`enum` · `ra_ap_ide::inlay_hints::TypeHintsPlacement`

Also reachable as `ra_ap_ide::TypeHintsPlacement`

```rust
enum TypeHintsPlacement
```

**Variants**: `Inline`, `EndOfLine`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## GenericParameterHints

`struct` · `ra_ap_ide::inlay_hints::GenericParameterHints`

Also reachable as `ra_ap_ide::GenericParameterHints`

```rust
struct GenericParameterHints
```

**Fields**: `type_hints`, `lifetime_hints`, `const_hints`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## InlayFieldsToResolve

`struct` · `ra_ap_ide::inlay_hints::InlayFieldsToResolve`

Also reachable as `ra_ap_ide::InlayFieldsToResolve`

```rust
struct InlayFieldsToResolve
```

**Fields**: `resolve_text_edits`, `resolve_hint_tooltip`, `resolve_label_tooltip`, `resolve_label_location`, `resolve_label_command`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
const fn empty() -> Self
fn from_client_capabilities(client_capability_fields: &FxHashSet<&str>) -> Self
```

---

## InlayHint

`struct` · `ra_ap_ide::inlay_hints::InlayHint`

Also reachable as `ra_ap_ide::InlayHint`

```rust
struct InlayHint
```

**Fields**: `range`, `position`, `pad_left`, `pad_right`, `kind`, `label`, `text_edit`, `resolve_parent`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Debug, Hash

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, __analysis: &::ide_db::ra_fixture::RaFixtureAnalysis, __virtual_file_id: ::ide_db::ra_fixture::FileId, __real_file_id: ::ide_db::ra_fixture::FileId) -> Result<Self, ()>
```

---

## InlayHintLabel

`struct` · `ra_ap_ide::inlay_hints::InlayHintLabel`

Also reachable as `ra_ap_ide::InlayHintLabel`

```rust
struct InlayHintLabel
```

**Fields**: `parts`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Debug, Default, Hash

**Methods** (4)

```rust
fn append_part(&mut self, part: InlayHintLabelPart)
fn append_str(&mut self, s: &str)
fn prepend_str(&mut self, s: &str)
fn simple(s: impl Into<String>, tooltip: Option<LazyProperty<InlayTooltip>>, linked_location: Option<LazyProperty<FileRange>>) -> InlayHintLabel
```

**via `core::convert::From`**

```rust
fn from(s: String) -> Self
fn from(s: &str) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, __analysis: &::ide_db::ra_fixture::RaFixtureAnalysis, __virtual_file_id: ::ide_db::ra_fixture::FileId, __real_file_id: ::ide_db::ra_fixture::FileId) -> Result<Self, ()>
```

---

## InlayHintLabelPart

`struct` · `ra_ap_ide::inlay_hints::InlayHintLabelPart`

Also reachable as `ra_ap_ide::InlayHintLabelPart`

```rust
struct InlayHintLabelPart
```

**Fields**: `text`, `linked_location`, `tooltip`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Debug, Hash

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, __analysis: &::ide_db::ra_fixture::RaFixtureAnalysis, __virtual_file_id: ::ide_db::ra_fixture::FileId, __real_file_id: ::ide_db::ra_fixture::FileId) -> Result<Self, ()>
```

---

## InlayHintsConfig

`struct` · `ra_ap_ide::inlay_hints::InlayHintsConfig`

Also reachable as `ra_ap_ide::InlayHintsConfig`

```rust
struct InlayHintsConfig<'a>
```

**Fields**: `render_colons`, `type_hints`, `type_hints_placement`, `sized_bound`, `discriminant_hints`, `parameter_hints`, `parameter_hints_for_missing_arguments`, `generic_parameter_hints`, `chaining_hints`, `adjustment_hints`, `adjustment_hints_disable_reborrows`, `adjustment_hints_mode`, `adjustment_hints_hide_outside_unsafe`, `closure_return_type_hints`, `closure_capture_hints`, `binding_mode_hints`, `implicit_drop_hints`, `implied_dyn_trait_hints`, `lifetime_elision_hints`, `param_names_for_lifetime_elision_hints`, `hide_inferred_type_hints`, `hide_named_constructor_hints`, `hide_closure_initialization_hints`, `hide_closure_parameter_hints`, `range_exclusive_hints`, `closure_style`, `max_length`, `closing_brace_hints_min_lines`, `fields_to_resolve`, `ra_fixture`

**Derives**: Clone, Debug

---
