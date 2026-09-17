# `grep_printer::color`

Crate `grep-printer` · 4 public items · structured records in [`model/grep_printer.color.json`](../model/grep_printer.color.json)

## ColorError

`enum` · `grep_printer::color::ColorError`

Also reachable as `grep_printer::ColorError`

```rust
enum ColorError
```

**Variants**: `UnrecognizedOutType`, `UnrecognizedSpecType`, `UnrecognizedColor`, `UnrecognizedStyle`, `InvalidFormat`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

An error that can occur when parsing color specifications.

---

## default_color_specs

`function` · `grep_printer::color::default_color_specs`

Also reachable as `grep_printer::default_color_specs`

```rust
fn default_color_specs() -> Vec<UserColorSpec>
```

Returns a default set of color specifications.

This may change over time, but the color choices are meant to be fairly
conservative that work across terminal themes.

Additional color specifications can be added to the list returned. More
recently added specifications override previously added specifications.

---

## ColorSpecs

`struct` · `grep_printer::color::ColorSpecs`

Also reachable as `grep_printer::ColorSpecs`

```rust
struct ColorSpecs
```

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn column(&self) -> &ColorSpec
fn default_with_color() -> ColorSpecs
fn highlight(&self) -> &ColorSpec
fn line(&self) -> &ColorSpec
fn matched(&self) -> &ColorSpec
fn new(specs: &[UserColorSpec]) -> ColorSpecs
fn path(&self) -> &ColorSpec
```

A merged set of color specifications.

This set of color specifications represents the various color types that
are supported by the printers in this crate. A set of color specifications
can be created from a sequence of
[`UserColorSpec`]s.

---

## UserColorSpec

`struct` · `grep_printer::color::UserColorSpec`

Also reachable as `grep_printer::UserColorSpec`

```rust
struct UserColorSpec
```

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn to_color_spec(&self) -> ColorSpec
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<UserColorSpec, ColorError>
```

A single color specification provided by the user.

## Format

The format of a `Spec` is a triple: `{type}:{attribute}:{value}`. Each
component is defined as follows:

* `{type}` can be one of `path`, `line`, `column`, `match` or `highlight`.
* `{attribute}` can be one of `fg`, `bg` or `style`. `{attribute}` may also
  be the special value `none`, in which case, `{value}` can be omitted.
* `{value}` is either a color name (for `fg`/`bg`) or a style instruction.

`{type}` controls which part of the output should be styled.

When `{attribute}` is `none`, then this should cause any existing style
settings to be cleared for the specified `type`.

`{value}` should be a color when `{attribute}` is `fg` or `bg`, or it
should be a style instruction when `{attribute}` is `style`. When
`{attribute}` is `none`, `{value}` must be omitted.

Valid colors are `black`, `blue`, `green`, `red`, `cyan`, `magenta`,
`yellow`, `white`. Extended colors can also be specified, and are formatted
as `x` (for 256-bit colors) or `x,x,x` (for 24-bit true color), where
`x` is a number between 0 and 255 inclusive. `x` may be given as a normal
decimal number of a hexadecimal number, where the latter is prefixed by
`0x`.

Valid style instructions are `nobold`, `bold`, `intense`, `nointense`,
`underline`, `nounderline`, `italic`, `noitalic`.

## Example

The standard way to build a `UserColorSpec` is to parse it from a string.
Once multiple `UserColorSpec`s have been constructed, they can be provided
to the standard printer where they will automatically be applied to the
output.

A `UserColorSpec` can also be converted to a `termcolor::ColorSpec`:

```rust
# fn main() {
use termcolor::{Color, ColorSpec};
use grep_printer::UserColorSpec;

let user_spec1: UserColorSpec = "path:fg:blue".parse().unwrap();
let user_spec2: UserColorSpec = "match:bg:0xff,0x7f,0x00".parse().unwrap();

let spec1 = user_spec1.to_color_spec();
let spec2 = user_spec2.to_color_spec();

assert_eq!(spec1.fg(), Some(&Color::Blue));
assert_eq!(spec2.bg(), Some(&Color::Rgb(0xFF, 0x7F, 0x00)));
# }
```

---
