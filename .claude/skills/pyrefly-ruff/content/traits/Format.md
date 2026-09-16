# Format

`ruff_formatter::Format`

```rust
trait Format<Context>
```

Also reachable as `ruff_formatter::prelude::Format`

Prose: [`api/ruff_formatter.md`](../api/ruff_formatter.md#format) · records: [`model/ruff_formatter.json`](../model/ruff_formatter.json)

## Required

Every implementation must supply these.

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

## Implementors (29)

Read one before writing your own.

- `core::option::Option`
- `ruff_formatter::FormatOwnedWithRule`
- `ruff_formatter::FormatRefWithRule`
- `ruff_formatter::arguments::Arguments`
- `ruff_formatter::builders::Align`
- `ruff_formatter::builders::BestFitParenthesize`
- `ruff_formatter::builders::BestFitting`
- `ruff_formatter::builders::BlockIndent`
- `ruff_formatter::builders::ConditionalGroup`
- `ruff_formatter::builders::Dedent`
- `ruff_formatter::builders::ExpandParent`
- `ruff_formatter::builders::FitsExpanded`
- `ruff_formatter::builders::FormatLabelled`
- `ruff_formatter::builders::FormatOnce`
- `ruff_formatter::builders::FormatWith`
- `ruff_formatter::builders::Group`
- `ruff_formatter::builders::IfGroupBreaks`
- `ruff_formatter::builders::Indent`
- `ruff_formatter::builders::IndentIfGroupBreaks`
- `ruff_formatter::builders::Line`
- `ruff_formatter::builders::LineSuffix`
- `ruff_formatter::builders::LineSuffixBoundary`
- `ruff_formatter::builders::SourcePosition`
- `ruff_formatter::builders::SourceTextSliceBuilder`
- `ruff_formatter::builders::Space`
- `ruff_formatter::builders::Text`
- `ruff_formatter::builders::Token`
- `ruff_formatter::format_extensions::Memoized`
- `ruff_python_ast::str_prefix::AnyStringPrefix`

## Documentation

Formatting trait for types that can create a formatted representation. The `ruff_formatter` equivalent
to [`std::fmt::Display`].

## Example
Implementing `Format` for a custom struct

```
use ruff_formatter::{format, write, IndentStyle};
use ruff_formatter::prelude::*;
use ruff_text_size::TextSize;

struct Paragraph(String);

impl Format<SimpleFormatContext> for Paragraph {
    fn fmt(&self, f: &mut Formatter<SimpleFormatContext>) -> FormatResult<()> {
        write!(f, [
            text(&self.0),
            hard_line_break(),
        ])
    }
}

# fn main() -> FormatResult<()> {
let paragraph = Paragraph(String::from("test"));
let formatted = format!(SimpleFormatContext::default(), [paragraph])?;

assert_eq!("test\n", formatted.print()?.as_code());
# Ok(())
# }
```
