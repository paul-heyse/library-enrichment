# `ruff_formatter::arguments`

Crate `ruff_formatter` · 2 public items · structured records in [`model/ruff_formatter.arguments.json`](../model/ruff_formatter.arguments.json)

## Argument

`struct` · `ruff_formatter::arguments::Argument`

Also reachable as `ruff_formatter::Argument`

```rust
struct Argument<'fmt, Context>
```

**Derives**: Clone, Copy

A convenience wrapper for representing a formattable argument.

---

## Arguments

`struct` · `ruff_formatter::arguments::Arguments`

Also reachable as `ruff_formatter::Arguments`

```rust
struct Arguments<'fmt, Context>
```

**Implements**: `core::convert::From`, `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug

**via `core::convert::From`**

```rust
fn from(argument: &'fmt Argument<'fmt, Context>) -> Self
```

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, formatter: &mut Formatter<'_, Context>) -> FormatResult<()>
```

Sequence of objects that should be formatted in the specified order.

The [`format_args!`] macro will safely create an instance of this structure.

You can use the `Arguments<a>` that [`format_args!`] return in `Format` context as seen below.
It will call the `format` function for each of its objects.

```rust
use ruff_formatter::prelude::*;
use ruff_formatter::{format, format_args};

# fn main() -> FormatResult<()> {
let formatted = format!(SimpleFormatContext::default(), [
    format_args!(token("a"), space(), token("b"))
])?;

assert_eq!("a b", formatted.print()?.as_code());
# Ok(())
# }
```

---
