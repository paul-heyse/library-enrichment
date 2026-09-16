# `ruff_formatter`

Crate `ruff_formatter` · 25 public items · structured records in [`model/ruff_formatter.json`](../model/ruff_formatter.json)

## IndentStyle

`enum` · `ruff_formatter::IndentStyle`

```rust
enum IndentStyle
```

**Variants**: `Tab`, `Space`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn as_str(&self) -> &'static str
const fn is_space(&self) -> bool
const fn is_tab(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---

## write

`function` · `ruff_formatter::write`

Also reachable as `ruff_formatter::prelude::write`

```rust
fn write<Context>(output: &mut dyn Buffer<Context = Context>, args: Arguments<'_, Context>) -> FormatResult<()>
```

The `write` function takes a target buffer and an `Arguments` struct that can be precompiled with the `format_args!` macro.

The arguments will be formatted in-order into the output buffer provided.

# Examples

```
use ruff_formatter::prelude::*;
use ruff_formatter::{VecBuffer, format_args, FormatState, write, Formatted};

# fn main() -> FormatResult<()> {
let mut state = FormatState::new(SimpleFormatContext::default());
let mut buffer = VecBuffer::new(&mut state);

write!(&mut buffer, [format_args!(token("Hello World"))])?;

let formatted = Formatted::new(Document::from(buffer.into_vec()), SimpleFormatContext::default());

assert_eq!("Hello World", formatted.print()?.as_code());
# Ok(())
# }
```

Please note that using [`write!`] might be preferable. Example:

```
use ruff_formatter::prelude::*;
use ruff_formatter::{VecBuffer, format_args, FormatState, write, Formatted};

# fn main() -> FormatResult<()> {
let mut state = FormatState::new(SimpleFormatContext::default());
let mut buffer = VecBuffer::new(&mut state);

write!(&mut buffer, [token("Hello World")])?;

let formatted = Formatted::new(Document::from(buffer.into_vec()), SimpleFormatContext::default());

assert_eq!("Hello World", formatted.print()?.as_code());
# Ok(())
# }
```

---

## best_fitting

`macro` · `ruff_formatter::best_fitting`

Also reachable as `ruff_formatter::prelude::best_fitting`

```rust
macro_rules! best_fitting
```

Provides multiple different alternatives and the printer picks the first one that fits.
Use this as last resort because it requires that the printer must try all variants in the worst case.
The passed variants must be in the following order:
- First: The variant that takes up most space horizontally
- Last: The variant that takes up the least space horizontally by splitting the content over multiple lines.

## Examples

```
use ruff_formatter::{Formatted, LineWidth, format, format_args, SimpleFormatOptions};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let formatted = format!(
    SimpleFormatContext::default(),
    [
        token("aVeryLongIdentifier"),
        best_fitting!(
            // Everything fits on a single line
            format_args!(
                token("("),
                group(&format_args![
                    token("["),
                        soft_block_indent(&format_args![
                        token("1,"),
                        soft_line_break_or_space(),
                        token("2,"),
                        soft_line_break_or_space(),
                        token("3"),
                    ]),
                    token("]")
                ]),
                token(")")
            ),

            // Breaks after `[`, but prints all elements on a single line
            format_args!(
                token("("),
                token("["),
                block_indent(&token("1, 2, 3")),
                token("]"),
                token(")"),
            ),

            // Breaks after `[` and prints each element on a single line
            format_args!(
                token("("),
                block_indent(&format_args![
                    token("["),
                    block_indent(&format_args![
                        token("1,"),
                        hard_line_break(),
                        token("2,"),
                        hard_line_break(),
                        token("3"),
                    ]),
                    token("]"),
                ]),
                token(")")
            )
        )
    ]
)?;

let document = formatted.into_document();

// Takes the first variant if everything fits on a single line
assert_eq!(
    "aVeryLongIdentifier([1, 2, 3])",
    Formatted::new(document.clone(), SimpleFormatContext::default())
        .print()?
        .as_code()
);

// It takes the second if the first variant doesn't fit on a single line. The second variant
// has some additional line breaks to make sure inner groups don't break
assert_eq!(
    "aVeryLongIdentifier([\n\t1, 2, 3\n])",
    Formatted::new(document.clone(), SimpleFormatContext::new(SimpleFormatOptions { line_width: 21.try_into().unwrap(), ..SimpleFormatOptions::default() }))
        .print()?
        .as_code()
);

// Prints the last option as last resort
assert_eq!(
    "aVeryLongIdentifier(\n\t[\n\t\t1,\n\t\t2,\n\t\t3\n\t]\n)",
    Formatted::new(document.clone(), SimpleFormatContext::new(SimpleFormatOptions { line_width: 20.try_into().unwrap(), ..SimpleFormatOptions::default() }))
        .print()?
        .as_code()
);
# Ok(())
# }
```

### Enclosing group with `should_expand: true`

```
use ruff_formatter::{Formatted, LineWidth, format, format_args, SimpleFormatOptions};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let formatted = format!(
    SimpleFormatContext::default(),
    [
        best_fitting!(
            // Prints the method call on the line but breaks the array.
            format_args!(
                token("expect(a).toMatch("),
                group(&format_args![
                    token("["),
                    soft_block_indent(&format_args![
                        token("1,"),
                        soft_line_break_or_space(),
                        token("2,"),
                        soft_line_break_or_space(),
                        token("3"),
                    ]),
                    token("]")
                ]).should_expand(true),
                token(")")
            ),

            // Breaks after `(`
           format_args!(
                token("expect(a).toMatch("),
                group(&soft_block_indent(
                    &group(&format_args![
                        token("["),
                        soft_block_indent(&format_args![
                            token("1,"),
                            soft_line_break_or_space(),
                            token("2,"),
                            soft_line_break_or_space(),
                            token("3"),
                        ]),
                        token("]")
                    ]).should_expand(true),
                )).should_expand(true),
                token(")")
            ),
        )
    ]
)?;

let document = formatted.into_document();

assert_eq!(
    "expect(a).toMatch([\n\t1,\n\t2,\n\t3\n])",
    Formatted::new(document.clone(), SimpleFormatContext::default())
        .print()?
        .as_code()
);

# Ok(())
# }
```

The first variant fits because all its content up to the first line break fit on the line without exceeding
the configured print width.

## Complexity
Be mindful of using this IR element as it has a considerable performance penalty:
- There are multiple representation for the same content. This results in increased memory usage
  and traversal time in the printer.
- The worst case complexity is that the printer tires each variant. This can result in quadratic
  complexity if used in nested structures.

## Behavior
This IR is similar to Prettier's `conditionalGroup`. The printer measures each variant, except the [`MostExpanded`], in [`Flat`] mode
to find the first variant that fits and prints this variant in [`Flat`] mode. If no variant fits, then
the printer falls back to printing the [`MostExpanded`] variant in [`Expanded`] mode.

The definition of *fits* differs to groups in that the printer only tests if it is possible to print
the content up to the first non-soft line break without exceeding the configured print width.
This definition differs from groups as that non-soft line breaks make group expand.

[`crate::BestFitting`] acts as a "break" boundary, meaning that it is considered to fit


[`Flat`]: crate::format_element::PrintMode::Flat
[`Expanded`]: crate::format_element::PrintMode::Expanded
[`MostExpanded`]: crate::format_element::BestFittingVariants::most_expanded

---

## dbg_write

`macro` · `ruff_formatter::dbg_write`

Also reachable as `ruff_formatter::prelude::dbg_write`

```rust
macro_rules! dbg_write
```

Writes formatted data into the given buffer and prints all written elements for a quick and dirty debugging.

An example:

```rust
use ruff_formatter::prelude::*;
use ruff_formatter::{FormatState, VecBuffer};

# fn main() -> FormatResult<()> {
let mut state = FormatState::new(SimpleFormatContext::default());
let mut buffer = VecBuffer::new(&mut state);

dbg_write!(buffer, [token("Hello")])?;
// ^-- prints: [src/main.rs:7][0] = StaticToken("Hello")

assert_eq!(buffer.into_vec(), vec![FormatElement::Token { text: "Hello" }]);
# Ok(())
# }
```

Note that the macro is intended as debugging tool and therefore you should avoid having
uses of it in version control for long periods (other than in tests and similar). Format output
from production code is better done with `[write!]`

---

## format

`macro` · `ruff_formatter::format`

Also reachable as `ruff_formatter::prelude::format`

```rust
macro_rules! format
```

Creates the Format IR for a value.

The first argument `format!` receives is the [`crate::FormatContext`] that specify how elements must be formatted.
Additional parameters passed get formatted by using their [`crate::Format`] implementation.


## Examples

```
use ruff_formatter::prelude::*;
use ruff_formatter::format;

let formatted = format!(SimpleFormatContext::default(), [token("("), token("a"), token(")")]).unwrap();

assert_eq!(
    formatted.into_document(),
    Document::from(vec![
        FormatElement::Token { text: "(" },
        FormatElement::Token { text: "a" },
        FormatElement::Token { text: ")" },
    ])
);
```

---

## format_args

`macro` · `ruff_formatter::format_args`

Also reachable as `ruff_formatter::prelude::format_args`

```rust
macro_rules! format_args
```

Constructs the parameters for other formatting macros.

This macro functions by taking a list of objects implementing [`crate::Format`]. It will canonicalize the
arguments into a single type.

This macro produces a value of type [`crate::Arguments`]. This value can be passed to
the macros within [crate]. All other formatting macros ([`format!`](crate::format!),
[`write!`](crate::write!)) are proxied through this one. This macro avoids heap allocations.

You can use the [`Arguments`] value that `format_args!` returns in  `Format` contexts
as seen below.

```rust
use ruff_formatter::{SimpleFormatContext, format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let formatted = format!(SimpleFormatContext::default(), [
    format_args!(token("Hello World"))
])?;

assert_eq!("Hello World", formatted.print()?.as_code());
# Ok(())
# }
```

[`Format`]: crate::Format
[`Arguments`]: crate::Arguments

---

## FormatOwnedWithRule

`struct` · `ruff_formatter::FormatOwnedWithRule`

```rust
struct FormatOwnedWithRule<T, R, C> where R: FormatRule<T, C>
```

**Implements**: `ruff_formatter::Format`, `ruff_formatter::FormatWithRule`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn new(item: T, rule: R) -> Self
fn with_item(self, item: T) -> Self
fn with_options(self, options: O) -> Self
```

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, C>) -> FormatResult<()>
```

**via `ruff_formatter::FormatWithRule`**

```rust
fn item(&self) -> &Self::Item
```

Formats the `item` with the specified rule.

---

## FormatRefWithRule

`struct` · `ruff_formatter::FormatRefWithRule`

```rust
struct FormatRefWithRule<'a, T, R, C> where R: FormatRule<T, C>
```

**Implements**: `ruff_formatter::Format`, `ruff_formatter::FormatWithRule`

**Derives**: Clone, Copy, Debug

**Methods** (3)

```rust
fn new(item: &'a T, rule: R) -> Self
fn rule(&self) -> &R
fn with_options(self, options: O) -> Self
```

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, C>) -> FormatResult<()>
```

**via `ruff_formatter::FormatWithRule`**

```rust
fn item(&self) -> &Self::Item
```

Formats the referenced `item` with the specified rule.

---

## FormatState

`struct` · `ruff_formatter::FormatState`

```rust
struct FormatState<Context>
```

**Derives**: Debug

**Methods** (2)

```rust
fn context_mut(&mut self) -> &mut Context
fn new(context: Context) -> Self
```

This structure stores the state that is relevant for the formatting of the whole document.

This structure is different from [`crate::Formatter`] in that the formatting infrastructure
creates a new [`crate::Formatter`] for every [`crate::write`!] call, whereas this structure stays alive
for the whole process of formatting a root with [`crate::format`!].

---

## Formatted

`struct` · `ruff_formatter::Formatted`

```rust
struct Formatted<Context>
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn context(&self) -> &Context
fn document(&self) -> &Document
fn into_document(self) -> Document
fn new(document: Document, context: Context) -> Self
fn print(&self) -> PrintResult<Printed>
fn print_with_indent(&self, indent: u16) -> PrintResult<Printed>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## IndentWidth

`struct` · `ruff_formatter::IndentWidth`

```rust
struct IndentWidth
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn value(&self) -> u32
```

**via `core::convert::From`**

```rust
fn from(value: NonZeroU8) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: u8) -> Result<Self, Self::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

The visual width of an indentation.

Determines the visual width of a tab character (`\t`) and the number of
spaces per indent when using [`IndentStyle::Space`].

---

## LineWidth

`struct` · `ruff_formatter::LineWidth`

```rust
struct LineWidth
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn value(&self) -> u16
```

**via `core::convert::From`**

```rust
fn from(value: NonZeroU16) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: u16) -> Result<LineWidth, Self::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

The maximum visual width to which the formatter should try to limit a line.

---

## Printed

`struct` · `ruff_formatter::Printed`

```rust
struct Printed
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (11)

```rust
fn as_code(&self) -> &str
fn into_code(self) -> String
fn into_sourcemap(self) -> Vec<SourceMarker>
fn new_empty() -> Self
fn range(&self) -> Option<TextRange>
fn slice_range(self, source_range: TextRange, source: &str) -> PrintedRange
fn sourcemap(&self) -> &[SourceMarker]
fn take_sourcemap(&mut self) -> Vec<SourceMarker>
fn take_verbatim_ranges(&mut self) -> Vec<TextRange>
fn verbatim(&self) -> impl Iterator<Item = (TextRange, &str)>
fn verbatim_ranges(&self) -> &[TextRange]
```

---

## PrintedRange

`struct` · `ruff_formatter::PrintedRange`

```rust
struct PrintedRange
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn as_code(&self) -> &str
fn empty() -> Self
fn into_code(self) -> String
fn new(code: String, source_range: TextRange) -> Self
fn source_range(&self) -> TextRange
```

---

## SimpleFormatContext

`struct` · `ruff_formatter::SimpleFormatContext`

Also reachable as `ruff_formatter::prelude::SimpleFormatContext`

```rust
struct SimpleFormatContext
```

**Implements**: `ruff_formatter::FormatContext`

**Derives**: Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new(options: SimpleFormatOptions) -> Self
fn with_source_code(self, code: &str) -> Self
```

**via `ruff_formatter::FormatContext`**

```rust
fn options(&self) -> &Self::Options
fn source_code(&self) -> SourceCode<'_>
```

---

## SimpleFormatOptions

`struct` · `ruff_formatter::SimpleFormatOptions`

```rust
struct SimpleFormatOptions
```

**Fields**: `indent_style`, `indent_width`, `line_width`

**Implements**: `ruff_formatter::FormatOptions`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `ruff_formatter::FormatOptions`**

```rust
fn as_print_options(&self) -> PrinterOptions
fn indent_style(&self) -> IndentStyle
fn indent_width(&self) -> IndentWidth
fn line_width(&self) -> LineWidth
```

---

## SourceMarker

`struct` · `ruff_formatter::SourceMarker`

```rust
struct SourceMarker
```

**Fields**: `source`, `dest`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Lightweight sourcemap marker between source and output tokens

---

## Format

`trait` · `ruff_formatter::Format`

Also reachable as `ruff_formatter::prelude::Format`

```rust
trait Format<Context>
```

**Implementors** (29)

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

**Methods** (1)

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

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

---

## FormatContext

`trait` · `ruff_formatter::FormatContext`

```rust
trait FormatContext
```

**Implementors** (2)

- `ruff_formatter::SimpleFormatContext`
- `ruff_python_formatter::context::PyFormatContext`

**Methods** (2)

```rust
fn options(&self) -> &Self::Options
fn source_code(&self) -> SourceCode<'_>
```

Context object storing data relevant when formatting an object.

---

## FormatOptions

`trait` · `ruff_formatter::FormatOptions`

```rust
trait FormatOptions
```

**Implementors** (2)

- `ruff_formatter::SimpleFormatOptions`
- `ruff_python_formatter::options::PyFormatOptions`

**Methods** (4)

```rust
fn as_print_options(&self) -> PrinterOptions
fn indent_style(&self) -> IndentStyle
fn indent_width(&self) -> IndentWidth
fn line_width(&self) -> LineWidth
```

Options customizing how the source code should be formatted.

---

## FormatRule

`trait` · `ruff_formatter::FormatRule`

Also reachable as `ruff_formatter::prelude::FormatRule`

```rust
trait FormatRule<T, C>
```

**Methods** (1)

```rust
fn fmt(&self, item: &T, f: &mut Formatter<'_, C>) -> FormatResult<()>
```

Rule that knows how to format an object of type `T`.

Implementing [Format] on the object itself is preferred over implementing [`FormatRule`] but
this isn't possible inside of a dependent crate for external type.

For example, the `ruff_js_formatter` crate isn't able to implement [Format] on `JsIfStatement`
because both the [Format] trait and `JsIfStatement` are external types (Rust's orphan rule).

That's why the `ruff_js_formatter` crate must define a new-type that implements the formatting
of `JsIfStatement`.

---

## FormatRuleWithOptions

`trait` · `ruff_formatter::FormatRuleWithOptions`

```rust
trait FormatRuleWithOptions<T, C>: FormatRule<T, C>
```

**Methods** (1)

```rust
fn with_options(self, options: Self::Options) -> Self
```

Rule that supports customizing how it formats an object of type `T`.

---

## FormatWithRule

`trait` · `ruff_formatter::FormatWithRule`

Also reachable as `ruff_formatter::prelude::_`

```rust
trait FormatWithRule<Context>: Format<Context>
```

**Implementors** (2)

- `ruff_formatter::FormatOwnedWithRule`
- `ruff_formatter::FormatRefWithRule`

**Methods** (1)

```rust
fn item(&self) -> &Self::Item
```

Trait for an object that formats an object with a specified rule.

Gives access to the underlying item.

Useful in situation where a type itself doesn't implement [Format] (e.g. because of Rust's orphan rule)
but you want to implement some common formatting logic.

## Examples

This can be useful if you want to format a `SyntaxNode` inside `ruff_formatter`.. `SyntaxNode` doesn't implement [Format]
itself but the language specific crate implements `AsFormat` and `IntoFormat` for it and the returned [Format]
implement [`FormatWithRule`].

```ignore
use ruff_formatter::prelude::*;
use ruff_formatter::{format, Formatted, FormatWithRule};
use ruff_rowan::{Language, SyntaxNode};
fn format_node<L: Language, F: FormatWithRule<SimpleFormatContext, Item=SyntaxNode<L>>>(node: F) -> FormatResult<Formatted<SimpleFormatContext>> {
    let formatted = format!(SimpleFormatContext::default(), [node]);
    let syntax = node.item();
    // Do something with syntax
    formatted;
}
```

---

## FormatResult

`type_alias` · `ruff_formatter::FormatResult`

Also reachable as `ruff_formatter::prelude::FormatResult`

```rust
type FormatResult<F> = Result<F, FormatError>
```

Public return type of the formatter

---

## PrintResult

`type_alias` · `ruff_formatter::PrintResult`

```rust
type PrintResult<T> = Result<T, PrintError>
```

---
