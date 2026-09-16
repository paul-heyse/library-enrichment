# `annotate_snippets::level`

Crate `ruff_annotate_snippets` · 6 public items · structured records in [`model/annotate_snippets.level.json`](../model/annotate_snippets.level.json)

## ERROR

`constant` · `annotate_snippets::level::ERROR`

Also reachable as `ruff_annotate_snippets::level::ERROR`

```rust
const ERROR: Level<'_> = _
```

Default `error:` [`Level`]

---

## HELP

`constant` · `annotate_snippets::level::HELP`

Also reachable as `ruff_annotate_snippets::level::HELP`

```rust
const HELP: Level<'_> = _
```

Default `help:` [`Level`]

---

## INFO

`constant` · `annotate_snippets::level::INFO`

Also reachable as `ruff_annotate_snippets::level::INFO`

```rust
const INFO: Level<'_> = _
```

Default `info:` [`Level`]

---

## NOTE

`constant` · `annotate_snippets::level::NOTE`

Also reachable as `ruff_annotate_snippets::level::NOTE`

```rust
const NOTE: Level<'_> = _
```

Default `note:` [`Level`]

---

## WARNING

`constant` · `annotate_snippets::level::WARNING`

Also reachable as `ruff_annotate_snippets::level::WARNING`

```rust
const WARNING: Level<'_> = _
```

Default `warning:` [`Level`]

---

## Level

`struct` · `annotate_snippets::level::Level`

Also reachable as `ruff_annotate_snippets::Level`, `ruff_annotate_snippets::level::Level`

```rust
struct Level<'a>
```

**Derives**: Clone, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (5)

```rust
fn message(self, text: impl Into<Cow<'a, str>>) -> Message<'a>
fn no_name(self) -> Level<'a>
fn primary_title(self, text: impl Into<Cow<'a, str>>) -> Title<'a>
fn secondary_title(self, text: impl Into<Cow<'a, str>>) -> Title<'a>
fn with_name(self, name: impl Into<OptionCow<'a>>) -> Level<'a>
```

Severity level for [`Title`]s and [`Message`]s

# Example

```rust
# use annotate_snippets::*;
let report = &[
    Level::ERROR.primary_title("mismatched types").id("E0308")
        .element(Level::NOTE.message("expected reference")),
    Group::with_title(
        Level::HELP.secondary_title("function defined here")
    ),
];
```

---
