# `ruff_formatter::format_element::tag`

Crate `ruff_formatter` · 12 public items · structured records in [`model/ruff_formatter.format_element.tag.json`](../model/ruff_formatter.format_element.tag.json)

## DedentMode

`enum` · `ruff_formatter::format_element::tag::DedentMode`

Also reachable as `ruff_formatter::prelude::tag::DedentMode`

```rust
enum DedentMode
```

**Variants**: `Level`, `Root`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## GroupMode

`enum` · `ruff_formatter::format_element::tag::GroupMode`

Also reachable as `ruff_formatter::prelude::tag::GroupMode`

```rust
enum GroupMode
```

**Variants**: `Flat`, `Expand`, `Propagated`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn is_flat(&self) -> bool
```

---

## Tag

`enum` · `ruff_formatter::format_element::tag::Tag`

Also reachable as `ruff_formatter::prelude::Tag`, `ruff_formatter::prelude::tag::Tag`

```rust
enum Tag
```

**Variants**: `StartIndent`, `EndIndent`, `StartAlign`, `EndAlign`, `StartDedent`, `EndDedent`, `StartGroup`, `EndGroup`, `StartConditionalGroup`, `EndConditionalGroup`, `StartConditionalContent`, `EndConditionalContent`, `StartIndentIfGroupBreaks`, `EndIndentIfGroupBreaks`, `StartFill`, `EndFill`, `StartEntry`, `EndEntry`, `StartLineSuffix`, `EndLineSuffix`, `StartVerbatim`, `EndVerbatim`, `StartLabelled`, `EndLabelled`, `StartFitsExpanded`, `EndFitsExpanded`, `StartBestFittingEntry`, `EndBestFittingEntry`, `StartBestFitParenthesize`, `EndBestFitParenthesize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

A Tag marking the start and end of some content to which some special formatting should be applied.

Tags always come in pairs of a start and an end tag and the styling defined by this tag
will be applied to all elements in between the start/end tags.

---

## TagKind

`enum` · `ruff_formatter::format_element::tag::TagKind`

Also reachable as `ruff_formatter::prelude::TagKind`, `ruff_formatter::prelude::tag::TagKind`

```rust
enum TagKind
```

**Variants**: `Indent`, `Align`, `Dedent`, `Group`, `ConditionalGroup`, `ConditionalContent`, `IndentIfGroupBreaks`, `Fill`, `Entry`, `LineSuffix`, `Verbatim`, `Labelled`, `FitsExpanded`, `BestFittingEntry`, `BestFitParenthesize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

The kind of a [Tag].

Each start end tag pair has its own [tag kind](TagKind).

---

## VerbatimKind

`enum` · `ruff_formatter::format_element::tag::VerbatimKind`

Also reachable as `ruff_formatter::prelude::tag::VerbatimKind`

```rust
enum VerbatimKind
```

**Variants**: `Bogus`, `Suppressed`, `Verbatim`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn is_bogus(&self) -> bool
```

---

## Align

`struct` · `ruff_formatter::format_element::tag::Align`

Also reachable as `ruff_formatter::prelude::tag::Align`

```rust
struct Align
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## Condition

`struct` · `ruff_formatter::format_element::tag::Condition`

Also reachable as `ruff_formatter::prelude::tag::Condition`

```rust
struct Condition
```

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn if_breaks() -> Self
fn if_fits_on_line() -> Self
fn if_group_breaks(group_id: GroupId) -> Self
fn if_group_fits_on_line(group_id: GroupId) -> Self
```

---

## ConditionalGroup

`struct` · `ruff_formatter::format_element::tag::ConditionalGroup`

Also reachable as `ruff_formatter::prelude::tag::ConditionalGroup`

```rust
struct ConditionalGroup
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(condition: Condition) -> Self
```

---

## FitsExpanded

`struct` · `ruff_formatter::format_element::tag::FitsExpanded`

Also reachable as `ruff_formatter::prelude::tag::FitsExpanded`

```rust
struct FitsExpanded
```

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

---

## Group

`struct` · `ruff_formatter::format_element::tag::Group`

Also reachable as `ruff_formatter::prelude::tag::Group`

```rust
struct Group
```

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

---

## LabelId

`struct` · `ruff_formatter::format_element::tag::LabelId`

Also reachable as `ruff_formatter::prelude::LabelId`, `ruff_formatter::prelude::tag::LabelId`

```rust
struct LabelId
```

**Derives**: Clone, Copy, Debug, Eq, PartialEq

**Methods** (1)

```rust
fn of<T: LabelDefinition>(label: T) -> Self
```

---

## LabelDefinition

`trait` · `ruff_formatter::format_element::tag::LabelDefinition`

Also reachable as `ruff_formatter::prelude::tag::LabelDefinition`

```rust
trait LabelDefinition
```

**Methods** (2)

```rust
fn name(&self) -> &'static str
fn value(&self) -> u64
```

Defines the valid labels of a language. You want to have at most one implementation per formatter
project.

---
