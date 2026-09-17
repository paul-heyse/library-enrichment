# `grep_printer::hyperlink`

Crate `grep-printer` · 6 public items · structured records in [`model/grep_printer.hyperlink.json`](../model/grep_printer.hyperlink.json)

## hyperlink_aliases

`function` · `grep_printer::hyperlink::hyperlink_aliases`

Also reachable as `grep_printer::hyperlink_aliases`

```rust
fn hyperlink_aliases() -> Vec<HyperlinkAlias>
```

Returns the set of hyperlink aliases supported by this crate.

Aliases are supported by the `FromStr` trait implementation of a
[`HyperlinkFormat`]. That is, if an alias is seen, then it is automatically
replaced with the corresponding format. For example, the `vscode` alias
maps to `vscode://file{path}:{line}:{column}`.

This is exposed to allow callers to include hyperlink aliases in
documentation in a way that is guaranteed to match what is actually
supported.

The list returned is guaranteed to be sorted lexicographically
by the alias name. Callers may want to re-sort the list using
[`HyperlinkAlias::display_priority`] via a stable sort when showing the
list to users. This will cause special aliases like `none` and `default` to
appear first.

---

## HyperlinkAlias

`struct` · `grep_printer::hyperlink::HyperlinkAlias`

Also reachable as `grep_printer::HyperlinkAlias`

```rust
struct HyperlinkAlias
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
const fn description(&self) -> &str
const fn display_priority(&self) -> Option<i16>
const fn name(&self) -> &str
```

An alias for a hyperlink format.

Hyperlink aliases are built-in formats, therefore they hold static values.
Some of their features are usable in const blocks.

---

## HyperlinkConfig

`struct` · `grep_printer::hyperlink::HyperlinkConfig`

Also reachable as `grep_printer::HyperlinkConfig`

```rust
struct HyperlinkConfig
```

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(env: HyperlinkEnvironment, format: HyperlinkFormat) -> HyperlinkConfig
```

Hyperlink configuration.

This configuration specifies both the [hyperlink format](HyperlinkFormat)
and an [environment](HyperlinkConfig) for interpolating a subset of
variables. The specific subset includes variables that are intended to
be invariant throughout the lifetime of a process, such as a machine's
hostname.

A hyperlink configuration can be provided to printer builders such as
[`StandardBuilder::hyperlink`](crate::StandardBuilder::hyperlink).

---

## HyperlinkEnvironment

`struct` · `grep_printer::hyperlink::HyperlinkEnvironment`

Also reachable as `grep_printer::HyperlinkEnvironment`

```rust
struct HyperlinkEnvironment
```

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn host(&mut self, host: Option<String>) -> &mut HyperlinkEnvironment
fn new() -> HyperlinkEnvironment
fn wsl_prefix(&mut self, wsl_prefix: Option<String>) -> &mut HyperlinkEnvironment
```

A static environment for hyperlink interpolation.

This environment permits setting the values of variables used in hyperlink
interpolation that are not expected to change for the lifetime of a program.
That is, these values are invariant.

Currently, this includes the hostname and a WSL distro prefix.

---

## HyperlinkFormat

`struct` · `grep_printer::hyperlink::HyperlinkFormat`

Also reachable as `grep_printer::HyperlinkFormat`

```rust
struct HyperlinkFormat
```

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn empty() -> HyperlinkFormat
fn into_config(self, env: HyperlinkEnvironment) -> HyperlinkConfig
fn is_empty(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<HyperlinkFormat, HyperlinkFormatError>
```

A hyperlink format with variables.

This can be created by parsing a string using `HyperlinkFormat::from_str`.

The default format is empty. An empty format is valid and effectively
disables hyperlinks.

# Example

```
use grep_printer::HyperlinkFormat;

let fmt = "vscode".parse::<HyperlinkFormat>()?;
assert_eq!(fmt.to_string(), "vscode://file{path}:{line}:{column}");

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## HyperlinkFormatError

`struct` · `grep_printer::hyperlink::HyperlinkFormatError`

Also reachable as `grep_printer::HyperlinkFormatError`

```rust
struct HyperlinkFormatError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

An error that can occur when parsing a hyperlink format.

---
