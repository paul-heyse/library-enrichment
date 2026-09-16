# `ruff_formatter::builders`

Crate `ruff_formatter` · 55 public items · structured records in [`model/ruff_formatter.builders.json`](../model/ruff_formatter.builders.json)

## align

`function` · `ruff_formatter::builders::align`

Also reachable as `ruff_formatter::prelude::align`

```rust
fn align<Content, Context>(count: u8, content: &Content) -> Align<'_, Context> where Content: Format<Context>
```

Aligns its content by indenting the content by `count` spaces.

[align] is a variant of `[indent]` that indents its content by a specified number of spaces rather than
using the configured indent character (tab or a specified number of spaces).

You should use [align] when you want to indent a content by a specific number of spaces.
Using [indent] is preferred in all other situations as it respects the users preferred indent character.

# Examples

## Tab indentation

```
use std::num::NonZeroU8;
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let block = format!(SimpleFormatContext::default(), [
    token("a"),
    hard_line_break(),
    token("?"),
    space(),
    align(2, &format_args![
        token("function () {"),
        hard_line_break(),
        token("}"),
    ]),
    hard_line_break(),
    token(":"),
    space(),
    align(2, &format_args![
        token("function () {"),
        block_indent(&token("console.log('test');")),
        token("}"),
    ]),
    token(";")
])?;

assert_eq!(
    "a\n? function () {\n  }\n: function () {\n\t\tconsole.log('test');\n  };",
    block.print()?.as_code()
);
# Ok(())
# }
```

You can see that:

- the printer indents the function's `}` by two spaces because it is inside of an `align`.
- the block `console.log` gets indented by two tabs.
  This is because `align` increases the indentation level by one (same as `indent`)
  if you nest an `indent` inside an `align`.
  Meaning that, `align > ... > indent` results in the same indentation as `indent > ... > indent`.

## Spaces indentation

```
use std::num::NonZeroU8;
use ruff_formatter::{format, format_args, IndentStyle, SimpleFormatOptions};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
use ruff_formatter::IndentWidth;
let context = SimpleFormatContext::new(SimpleFormatOptions {
    indent_style: IndentStyle::Space,
    indent_width: IndentWidth::try_from(4).unwrap(),
    ..SimpleFormatOptions::default()
});

let block = format!(context, [
    token("a"),
    hard_line_break(),
    token("?"),
    space(),
    align(2, &format_args![
        token("function () {"),
        hard_line_break(),
        token("}"),
    ]),
    hard_line_break(),
    token(":"),
    space(),
    align(2, &format_args![
        token("function () {"),
        block_indent(&token("console.log('test');")),
        token("}"),
    ]),
    token(";")
])?;

assert_eq!(
    "a\n? function () {\n  }\n: function () {\n      console.log('test');\n  };",
    block.print()?.as_code()
);
# Ok(())
# }
```

The printing of `align` differs if using spaces as indentation sequence *and* it contains an `indent`.
You can see the difference when comparing the indentation of the `console.log(...)` expression to the previous example:

- tab indentation: Printer indents the expression with two tabs because the `align` increases the indentation level.
- space indentation: Printer indents the expression by 4 spaces (one indentation level) **and** 2 spaces for the align.

---

## best_fit_parenthesize

`function` · `ruff_formatter::builders::best_fit_parenthesize`

Also reachable as `ruff_formatter::prelude::best_fit_parenthesize`

```rust
fn best_fit_parenthesize<Context>(content: &impl Format<Context>) -> BestFitParenthesize<'_, Context>
```

Content that may get parenthesized if it exceeds the configured line width but only if the parenthesized
layout doesn't exceed the line width too, in which case it falls back to the flat layout.

This IR is identical to the following [`best_fitting`] layout but is implemented as custom IR for
better performance.

```rust
# use ruff_formatter::prelude::*;
# use ruff_formatter::format_args;

let format_expression = format_with(|f: &mut Formatter<SimpleFormatContext>| token("A long string").fmt(f));
let _ = best_fitting![
    // ---------------------------------------------------------------------
    // Variant 1:
    // Try to fit the expression without any parentheses
    group(&format_expression),
    // ---------------------------------------------------------------------
    // Variant 2:
    // Try to fit the expression by adding parentheses and indenting the expression.
    group(&format_args![
        token("("),
        soft_block_indent(&format_expression),
        token(")")
    ])
    .should_expand(true),
    // ---------------------------------------------------------------------
    // Variant 3: Fallback, no parentheses
    // Expression doesn't fit regardless of adding the parentheses. Remove the parentheses again.
    group(&format_expression).should_expand(true)
]
// Measure all lines, to avoid that the printer decides that this fits right after hitting
// the `(`.
.with_mode(BestFittingMode::AllLines)        ;
```

The element breaks from left-to-right because it uses the unintended version as *expanded* layout, the same as the above showed best fitting example.

## Examples

### Content that fits into the configured line width.

```rust
# use ruff_formatter::prelude::*;
# use ruff_formatter::{format, PrintResult, write};

# fn main() -> FormatResult<()> {
    let formatted = format!(SimpleFormatContext::default(), [format_with(|f| {
        write!(f, [
            token("aLongerVariableName = "),
            best_fit_parenthesize(&token("'a string that fits into the configured line width'"))
        ])
    })])?;

    assert_eq!(formatted.print()?.as_code(), "aLongerVariableName = 'a string that fits into the configured line width'");
    # Ok(())
# }
```

### Content that fits parenthesized

```rust
# use ruff_formatter::prelude::*;
# use ruff_formatter::{format, PrintResult, write};

# fn main() -> FormatResult<()> {
    let formatted = format!(SimpleFormatContext::default(), [format_with(|f| {
        write!(f, [
            token("aLongerVariableName = "),
            best_fit_parenthesize(&token("'a string that exceeds configured line width but fits parenthesized'"))
        ])
    })])?;

    assert_eq!(formatted.print()?.as_code(), "aLongerVariableName = (\n\t'a string that exceeds configured line width but fits parenthesized'\n)");
    # Ok(())
# }
```

### Content that exceeds the line width, parenthesized or not

```rust
# use ruff_formatter::prelude::*;
# use ruff_formatter::{format, PrintResult, write};

# fn main() -> FormatResult<()> {
    let formatted = format!(SimpleFormatContext::default(), [format_with(|f| {
        write!(f, [
            token("aLongerVariableName = "),
            best_fit_parenthesize(&token("'a string that exceeds the configured line width and even parenthesizing doesn't make it fit'"))
        ])
    })])?;

    assert_eq!(formatted.print()?.as_code(), "aLongerVariableName = 'a string that exceeds the configured line width and even parenthesizing doesn't make it fit'");
    # Ok(())
# }
```

---

## block_indent

`function` · `ruff_formatter::builders::block_indent`

Also reachable as `ruff_formatter::prelude::block_indent`

```rust
fn block_indent<Context>(content: &impl Format<Context>) -> BlockIndent<'_, Context>
```

Inserts a hard line break before and after the content and increases the indentation level for the content by one.

Block indents indent a block of code, such as in a function body, and therefore insert a line
break before and after the content.

Doesn't create an indentation if the passed in content is empty.

# Examples

```
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let block = format![
    SimpleFormatContext::default(),
    [
        token("{"),
        block_indent(&format_args![
            token("let a = 10;"),
            hard_line_break(),
            token("let c = a + 5;"),
        ]),
        token("}"),
    ]
]?;

assert_eq!(
    "{\n\tlet a = 10;\n\tlet c = a + 5;\n}",
    block.print()?.as_code()
);
# Ok(())
# }
```

---

## conditional_group

`function` · `ruff_formatter::builders::conditional_group`

Also reachable as `ruff_formatter::prelude::conditional_group`

```rust
fn conditional_group<Content, Context>(content: &Content, condition: format_element::tag::Condition) -> ConditionalGroup<'_, Context> where Content: Format<Context>
```

Sets the `condition` for the group. The element will behave as a regular group if `condition` is met,
and as *ungrouped* content if the condition is not met.

## Examples

Only expand before operators if the parentheses are necessary.

```
# use ruff_formatter::prelude::*;
# use ruff_formatter::{format, format_args, LineWidth, SimpleFormatOptions};

# fn main() -> FormatResult<()> {
use ruff_formatter::Formatted;
let content = format_with(|f| {
    let parentheses_id = f.group_id("parentheses");
    group(&format_args![
        if_group_breaks(&token("(")),
        indent_if_group_breaks(&format_args![
            soft_line_break(),
            conditional_group(&format_args![
                token("'aaaaaaa'"),
                soft_line_break_or_space(),
                token("+"),
                space(),
                fits_expanded(&conditional_group(&format_args![
                    token("["),
                    soft_block_indent(&format_args![
                        token("'Good morning!',"),
                        soft_line_break_or_space(),
                        token("'How are you?'"),
                    ]),
                    token("]"),
                ], tag::Condition::if_group_fits_on_line(parentheses_id))),
                soft_line_break_or_space(),
                token("+"),
                space(),
                conditional_group(&format_args![
                    token("'bbbb'"),
                    soft_line_break_or_space(),
                    token("and"),
                    space(),
                    token("'c'")
                ], tag::Condition::if_group_fits_on_line(parentheses_id))
            ], tag::Condition::if_breaks()),
        ], parentheses_id),
        soft_line_break(),
        if_group_breaks(&token(")"))
    ])
    .with_id(Some(parentheses_id))
    .fmt(f)
});

let formatted = format!(SimpleFormatContext::default(), [content])?;
let document = formatted.into_document();

// All content fits
let all_fits = Formatted::new(document.clone(), SimpleFormatContext::new(SimpleFormatOptions {
    line_width: LineWidth::try_from(65).unwrap(),
    ..SimpleFormatOptions::default()
}));

assert_eq!(
    "'aaaaaaa' + ['Good morning!', 'How are you?'] + 'bbbb' and 'c'",
    all_fits.print()?.as_code()
);

// The parentheses group fits, because it can expand the list,
let list_expanded = Formatted::new(document.clone(), SimpleFormatContext::new(SimpleFormatOptions {
    line_width: LineWidth::try_from(21).unwrap(),
    ..SimpleFormatOptions::default()
}));

assert_eq!(
    "'aaaaaaa' + [\n\t'Good morning!',\n\t'How are you?'\n] + 'bbbb' and 'c'",
    list_expanded.print()?.as_code()
);

// It is necessary to split all groups to fit the content
let all_expanded = Formatted::new(document, SimpleFormatContext::new(SimpleFormatOptions {
    line_width: LineWidth::try_from(11).unwrap(),
    ..SimpleFormatOptions::default()
}));

assert_eq!(
    "(\n\t'aaaaaaa'\n\t+ [\n\t\t'Good morning!',\n\t\t'How are you?'\n\t]\n\t+ 'bbbb'\n\tand 'c'\n)",
    all_expanded.print()?.as_code()
);
# Ok(())
# }
```

---

## dedent

`function` · `ruff_formatter::builders::dedent`

Also reachable as `ruff_formatter::prelude::dedent`

```rust
fn dedent<Content, Context>(content: &Content) -> Dedent<'_, Context> where Content: Format<Context>
```

It reduces the indentation for the given content depending on the closest [indent] or [align] parent element.
- [align] Undoes the spaces added by [align]
- [indent] Reduces the indentation level by one

This is a No-op if the indentation level is zero.

# Examples

```
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let block = format!(SimpleFormatContext::default(), [
    token("root"),
    align(2, &format_args![
        hard_line_break(),
        token("aligned"),
        dedent(&format_args![
            hard_line_break(),
            token("not aligned"),
        ]),
        dedent(&indent(&format_args![
            hard_line_break(),
            token("Indented, not aligned")
        ]))
    ]),
    dedent(&format_args![
        hard_line_break(),
        token("Dedent on root level is a no-op.")
    ])
])?;

assert_eq!(
    "root\n  aligned\nnot aligned\n\tIndented, not aligned\nDedent on root level is a no-op.",
    block.print()?.as_code()
);
# Ok(())
# }
```

---

## dedent_to_root

`function` · `ruff_formatter::builders::dedent_to_root`

Also reachable as `ruff_formatter::prelude::dedent_to_root`

```rust
fn dedent_to_root<Content, Context>(content: &Content) -> Dedent<'_, Context> where Content: Format<Context>
```

It resets the indent document so that the content will be printed at the start of the line.

# Examples

```
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let block = format!(SimpleFormatContext::default(), [
    token("root"),
    indent(&format_args![
        hard_line_break(),
        token("indent level 1"),
        indent(&format_args![
            hard_line_break(),
            token("indent level 2"),
            align(2, &format_args![
                hard_line_break(),
                token("two space align"),
                dedent_to_root(&format_args![
                    hard_line_break(),
                    token("starts at the beginning of the line")
                ]),
            ]),
            hard_line_break(),
            token("end indent level 2"),
        ])
 ]),
])?;

assert_eq!(
    "root\n\tindent level 1\n\t\tindent level 2\n\t\t  two space align\nstarts at the beginning of the line\n\t\tend indent level 2",
    block.print()?.as_code()
);
# Ok(())
# }
```

## Prettier

This resembles the behaviour of Prettier's `align(Number.NEGATIVE_INFINITY, content)` IR element.

---

## empty_line

`function` · `ruff_formatter::builders::empty_line`

Also reachable as `ruff_formatter::prelude::empty_line`

```rust
const fn empty_line() -> Line
```

A forced empty line. An empty line inserts enough line breaks in the output for
the previous and next element to be separated by an empty line.

# Examples

```
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

fn main() -> FormatResult<()> {
let elements = format!(
    SimpleFormatContext::default(), [
    group(&format_args![
        token("a,"),
        empty_line(),
        token("b"),
        empty_line()
    ])
])?;

assert_eq!(
    "a,\n\nb\n\n",
    elements.print()?.as_code()
);
# Ok(())
# }
```

---

## expand_parent

`function` · `ruff_formatter::builders::expand_parent`

Also reachable as `ruff_formatter::prelude::expand_parent`

```rust
const fn expand_parent() -> ExpandParent
```

IR element that forces the parent group to print in expanded mode.

Has no effect if used outside of a group or element that introduce implicit groups (fill element).

## Examples

```
use ruff_formatter::{format, format_args, LineWidth};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let elements = format!(SimpleFormatContext::default(), [
    group(&format_args![
        token("["),
        soft_block_indent(&format_args![
            token("'Good morning! How are you today?',"),
            soft_line_break_or_space(),
            token("2,"),
            expand_parent(), // Forces the parent to expand
            soft_line_break_or_space(),
            token("3"),
        ]),
        token("]"),
    ])
])?;

assert_eq!(
    "[\n\t'Good morning! How are you today?',\n\t2,\n\t3\n]",
    elements.print()?.as_code()
);
# Ok(())
# }
```

# Prettier
Equivalent to Prettier's `break_parent` IR element

---

## fits_expanded

`function` · `ruff_formatter::builders::fits_expanded`

Also reachable as `ruff_formatter::prelude::fits_expanded`

```rust
fn fits_expanded<Content, Context>(content: &Content) -> FitsExpanded<'_, Context> where Content: Format<Context>
```

Changes the definition of *fits* for `content`. It measures the width of all lines and allows
the content inside of the [`fits_expanded`] to exceed the configured line width. The content
coming before and after [`fits_expanded`] must fit into the configured line width.

The [`fits_expanded`] acts as a expands boundary similar to best fitting,
meaning that a [`hard_line_break`] will not cause the parent group to expand.

Useful in conjunction with a group with a condition.

## Examples
The outer group with the binary expression remains *flat* regardless of the array expression that
spans multiple lines with items exceeding the configured line width.

```
# use ruff_formatter::{format, format_args, LineWidth, SimpleFormatOptions, write};
# use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let content = format_with(|f| {
    let group_id = f.group_id("header");

    write!(f, [
        group(&format_args![
            token("a"),
            soft_line_break_or_space(),
            token("+"),
            space(),
            fits_expanded(&group(&format_args![
                token("["),
                soft_block_indent(&format_args![
                    token("a,"), space(), token("# comment"), expand_parent(), soft_line_break_or_space(),
                    token("'A very long string that exceeds the configured line width of 80 characters but the enclosing binary expression still fits.'")
                ]),
                token("]")
            ]))
        ]),
    ])
});

let formatted = format!(SimpleFormatContext::default(), [content])?;

assert_eq!(
    "a + [\n\ta, # comment\n\t'A very long string that exceeds the configured line width of 80 characters but the enclosing binary expression still fits.'\n]",
    formatted.print()?.as_code()
);
# Ok(())
# }
```

---

## format_once

`function` · `ruff_formatter::builders::format_once`

Also reachable as `ruff_formatter::prelude::format_once`

```rust
const fn format_once<T, Context>(formatter: T) -> FormatOnce<T, Context> where T: FnOnce(&mut Formatter<'_, Context>) -> FormatResult<()>
```

Creates an inline `Format` object that can only be formatted once.

This can be useful in situation where the borrow checker doesn't allow you to use [`format_with`]
because the code formatting the content consumes the value and cloning the value is too expensive.
An example of this is if you want to nest a `FormatElement` or non-cloneable `Iterator` inside of a
`block_indent` as shown can see in the examples section.

# Panics

Panics if the object gets formatted more than once.

# Example

```
use ruff_formatter::prelude::*;
use ruff_formatter::{SimpleFormatContext, format, write, Buffer};

struct MyFormat;

fn generate_values() -> impl Iterator<Item=Token> {
    vec![token("1"), token("2"), token("3"), token("4")].into_iter()
}

impl Format<SimpleFormatContext> for MyFormat {
    fn fmt(&self, f: &mut Formatter<SimpleFormatContext>) -> FormatResult<()> {
        let mut values = generate_values();

        let first = values.next();

        // Formats the first item outside of the block and all other items inside of the block,
        // separated by line breaks
        write!(f, [
            first,
            block_indent(&format_once(|f| {
                // Using format_with isn't possible here because the iterator gets consumed here
                f.join_with(&hard_line_break()).entries(values).finish()
            })),
        ])
    }
}

# fn main() -> FormatResult<()> {
let formatted = format!(SimpleFormatContext::default(), [MyFormat])?;

assert_eq!("1\n\t2\n\t3\n\t4\n", formatted.print()?.as_code());
# Ok(())
# }
```

Formatting the same value twice results in a panic.

```should_panic
use ruff_formatter::prelude::*;
use ruff_formatter::{SimpleFormatContext, format, write, Buffer};
use ruff_text_size::TextSize;

let mut count = 0;

let value = format_once(|f| {
    write!(f, [text(&std::format!("Formatted {count}."))])
});

format!(SimpleFormatContext::default(), [value]).expect("Formatting once works fine");

// Formatting the value more than once panics
format!(SimpleFormatContext::default(), [value]);
```

---

## format_with

`function` · `ruff_formatter::builders::format_with`

Also reachable as `ruff_formatter::prelude::format_with`

```rust
const fn format_with<Context, T>(formatter: T) -> FormatWith<Context, T> where T: Fn(&mut Formatter<'_, Context>) -> FormatResult<()>
```

Creates an object implementing `Format` that calls the passed closure to perform the formatting.

# Examples

```
use ruff_formatter::prelude::*;
use ruff_formatter::{SimpleFormatContext, format, write};
use ruff_text_size::TextSize;

struct MyFormat {
    items: Vec<&'static str>,
}

impl Format<SimpleFormatContext> for MyFormat {
    fn fmt(&self, f: &mut Formatter<SimpleFormatContext>) -> FormatResult<()> {
        write!(f, [
            token("("),
            block_indent(&format_with(|f| {
                let separator = space();
                let mut join = f.join_with(&separator);

                for item in &self.items {
                    join.entry(&format_with(|f| write!(f, [text(item)])));
                }
                join.finish()
            })),
            token(")")
        ])
    }
}

# fn main() -> FormatResult<()> {
let formatted = format!(SimpleFormatContext::default(), [MyFormat { items: vec!["a", "b", "c"]}])?;

assert_eq!("(\n\ta b c\n)", formatted.print()?.as_code());
# Ok(())
# }
```

---

## group

`function` · `ruff_formatter::builders::group`

Also reachable as `ruff_formatter::prelude::group`

```rust
fn group<Context>(content: &impl Format<Context>) -> Group<'_, Context>
```

Creates a logical `Group` around the content that should either consistently be printed on a single line
or broken across multiple lines.

The printer will try to print the content of the `Group` on a single line, ignoring all soft line breaks and
emitting spaces for soft line breaks or spaces. The printer tracks back if it isn't successful either
because it encountered a hard line break, or because printing the `Group` on a single line exceeds
the configured line width, and thus it must print all its content on multiple lines,
emitting line breaks for all line break kinds.

# Examples

`Group` that fits on a single line

```
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let elements = format!(SimpleFormatContext::default(), [
    group(&format_args![
        token("["),
        soft_block_indent(&format_args![
            token("1,"),
            soft_line_break_or_space(),
            token("2,"),
            soft_line_break_or_space(),
            token("3"),
        ]),
        token("]"),
    ])
])?;

assert_eq!(
    "[1, 2, 3]",
    elements.print()?.as_code()
);
# Ok(())
# }
```

The printer breaks the `Group` over multiple lines if its content doesn't fit on a single line
```
use ruff_formatter::{format, format_args, LineWidth, SimpleFormatOptions};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let context = SimpleFormatContext::new(SimpleFormatOptions {
    line_width: LineWidth::try_from(20).unwrap(),
    ..SimpleFormatOptions::default()
});

let elements = format!(context, [
    group(&format_args![
        token("["),
        soft_block_indent(&format_args![
            token("'Good morning! How are you today?',"),
            soft_line_break_or_space(),
            token("2,"),
            soft_line_break_or_space(),
            token("3"),
        ]),
        token("]"),
    ])
])?;

assert_eq!(
    "[\n\t'Good morning! How are you today?',\n\t2,\n\t3\n]",
    elements.print()?.as_code()
);
# Ok(())
# }
```

---

## hard_line_break

`function` · `ruff_formatter::builders::hard_line_break`

Also reachable as `ruff_formatter::prelude::hard_line_break`

```rust
const fn hard_line_break() -> Line
```

A forced line break that are always printed. A hard line break forces any enclosing `Group`
to be printed over multiple lines.

# Examples

It forces a line break, even if the enclosing `Group` would otherwise fit on a single line.
```
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let elements = format!(SimpleFormatContext::default(), [
    group(&format_args![
        token("a,"),
        hard_line_break(),
        token("b"),
        hard_line_break()
    ])
])?;

assert_eq!(
    "a,\nb\n",
    elements.print()?.as_code()
);
# Ok(())
# }
```

---

## if_group_breaks

`function` · `ruff_formatter::builders::if_group_breaks`

Also reachable as `ruff_formatter::prelude::if_group_breaks`

```rust
fn if_group_breaks<Content, Context>(content: &Content) -> IfGroupBreaks<'_, Context> where Content: Format<Context>
```

Adds a conditional content that is emitted only if it isn't inside an enclosing `Group` that
is printed on a single line. The element allows, for example, to insert a trailing comma after the last
array element only if the array doesn't fit on a single line.

The element has no special meaning if used outside of a `Group`. In that case, the content is always emitted.

If you're looking for a way to only print something if the `Group` fits on a single line see [`self::if_group_fits_on_line`].

# Examples

Omits the trailing comma for the last array element if the `Group` fits on a single line
```
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let elements = format!(SimpleFormatContext::default(), [
    group(&format_args![
        token("["),
        soft_block_indent(&format_args![
            token("1,"),
            soft_line_break_or_space(),
            token("2,"),
            soft_line_break_or_space(),
            token("3"),
            if_group_breaks(&token(","))
        ]),
        token("]"),
    ])
])?;

assert_eq!(
    "[1, 2, 3]",
    elements.print()?.as_code()
);
# Ok(())
# }
```

Prints the trailing comma for the last array element if the `Group` doesn't fit on a single line
```
use ruff_formatter::{format_args, format, LineWidth, SimpleFormatOptions};
use ruff_formatter::prelude::*;

fn main() -> FormatResult<()> {
let context = SimpleFormatContext::new(SimpleFormatOptions {
    line_width: LineWidth::try_from(20).unwrap(),
    ..SimpleFormatOptions::default()
});

let elements = format!(context, [
    group(&format_args![
        token("["),
        soft_block_indent(&format_args![
            token("'A somewhat longer string to force a line break',"),
            soft_line_break_or_space(),
            token("2,"),
            soft_line_break_or_space(),
            token("3"),
            if_group_breaks(&token(","))
        ]),
        token("]"),
    ])
])?;

assert_eq!(
    "[\n\t'A somewhat longer string to force a line break',\n\t2,\n\t3,\n]",
    elements.print()?.as_code()
);
# Ok(())
# }
```

---

## if_group_fits_on_line

`function` · `ruff_formatter::builders::if_group_fits_on_line`

Also reachable as `ruff_formatter::prelude::if_group_fits_on_line`

```rust
fn if_group_fits_on_line<Content, Context>(flat_content: &Content) -> IfGroupBreaks<'_, Context> where Content: Format<Context>
```

Adds a conditional content specific for `Group`s that fit on a single line. The content isn't
emitted for `Group`s spanning multiple lines.

See [`if_group_breaks`] if you're looking for a way to print content only for groups spanning multiple lines.

# Examples

Adds the trailing comma for the last array element if the `Group` fits on a single line
```
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let formatted = format!(SimpleFormatContext::default(), [
    group(&format_args![
        token("["),
        soft_block_indent(&format_args![
            token("1,"),
            soft_line_break_or_space(),
            token("2,"),
            soft_line_break_or_space(),
            token("3"),
            if_group_fits_on_line(&token(","))
        ]),
        token("]"),
    ])
])?;

assert_eq!(
    "[1, 2, 3,]",
    formatted.print()?.as_code()
);
# Ok(())
# }
```

Omits the trailing comma for the last array element if the `Group` doesn't fit on a single line
```
use ruff_formatter::{format, format_args, LineWidth, SimpleFormatOptions};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let context = SimpleFormatContext::new(SimpleFormatOptions {
    line_width: LineWidth::try_from(20).unwrap(),
    ..SimpleFormatOptions::default()
});

let formatted = format!(context, [
    group(&format_args![
        token("["),
        soft_block_indent(&format_args![
            token("'A somewhat longer string to force a line break',"),
            soft_line_break_or_space(),
            token("2,"),
            soft_line_break_or_space(),
            token("3"),
            if_group_fits_on_line(&token(","))
        ]),
        token("]"),
    ])
])?;

assert_eq!(
    "[\n\t'A somewhat longer string to force a line break',\n\t2,\n\t3\n]",
    formatted.print()?.as_code()
);
# Ok(())
# }
```

---

## indent

`function` · `ruff_formatter::builders::indent`

Also reachable as `ruff_formatter::prelude::indent`

```rust
fn indent<Content, Context>(content: &Content) -> Indent<'_, Context> where Content: Format<Context>
```

It adds a level of indentation to the given content

It doesn't add any line breaks at the edges of the content, meaning that
the line breaks have to be manually added.

This helper should be used only in rare cases, instead you should rely more on
[`block_indent`] and [`soft_block_indent`]

# Examples

```
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let block = format!(SimpleFormatContext::default(), [
    token("switch {"),
    block_indent(&format_args![
        token("default:"),
        indent(&format_args![
            // this is where we want to use a
            hard_line_break(),
            token("break;"),
        ])
    ]),
    token("}"),
])?;

assert_eq!(
    "switch {\n\tdefault:\n\t\tbreak;\n}",
    block.print()?.as_code()
);
# Ok(())
# }
```

---

## indent_if_group_breaks

`function` · `ruff_formatter::builders::indent_if_group_breaks`

Also reachable as `ruff_formatter::prelude::indent_if_group_breaks`

```rust
fn indent_if_group_breaks<Content, Context>(content: &Content, group_id: GroupId) -> IndentIfGroupBreaks<'_, Context> where Content: Format<Context>
```

Increases the indent level by one if the group with the specified id breaks.

This IR has the same semantics as using [`if_group_breaks`] and [`if_group_fits_on_line`] together.

```
# use ruff_formatter::prelude::*;
# use ruff_formatter::write;
# let format = format_with(|f: &mut Formatter<SimpleFormatContext>| {
let id = f.group_id("head");

write!(f, [
    group(&token("Head")).with_id(Some(id)),
    if_group_breaks(&indent(&token("indented"))).with_group_id(Some(id)),
    if_group_fits_on_line(&token("indented")).with_group_id(Some(id))
])

# });
```

If you want to indent some content if the enclosing group breaks, use [`indent`].

Use [`if_group_breaks`] or [`if_group_fits_on_line`] if the fitting and breaking content differs more than just the
indentation level.

# Examples

Indent the body of an arrow function if the group wrapping the signature breaks:
```
use ruff_formatter::{format, format_args, LineWidth, SimpleFormatOptions, write};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let content = format_with(|f| {
    let group_id = f.group_id("header");

    write!(f, [
        group(&token("(aLongHeaderThatBreaksForSomeReason) =>")).with_id(Some(group_id)),
        indent_if_group_breaks(&format_args![hard_line_break(), token("a => b")], group_id)
    ])
});

let context = SimpleFormatContext::new(SimpleFormatOptions {
    line_width: LineWidth::try_from(20).unwrap(),
    ..SimpleFormatOptions::default()
});

let formatted = format!(context, [content])?;

assert_eq!(
    "(aLongHeaderThatBreaksForSomeReason) =>\n\ta => b",
    formatted.print()?.as_code()
);
# Ok(())
# }
```

It doesn't add an indent if the group wrapping the signature doesn't break:
```
use ruff_formatter::{format, format_args, LineWidth, SimpleFormatOptions, write};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let content = format_with(|f| {
    let group_id = f.group_id("header");

    write!(f, [
        group(&token("(aLongHeaderThatBreaksForSomeReason) =>")).with_id(Some(group_id)),
        indent_if_group_breaks(&format_args![hard_line_break(), token("a => b")], group_id)
    ])
});

let formatted = format!(SimpleFormatContext::default(), [content])?;

assert_eq!(
    "(aLongHeaderThatBreaksForSomeReason) =>\na => b",
    formatted.print()?.as_code()
);
# Ok(())
# }
```

---

## labelled

`function` · `ruff_formatter::builders::labelled`

Also reachable as `ruff_formatter::prelude::labelled`

```rust
fn labelled<Content, Context>(label_id: prelude::tag::LabelId, content: &Content) -> FormatLabelled<'_, Context> where Content: Format<Context>
```

Marks some content with a label.

This does not directly influence how this content will be printed, but some
parts of the formatter may inspect the [labelled element](Tag::StartLabelled)
using [`FormatElements::has_label`].

## Examples

```rust
use ruff_formatter::prelude::*;
use ruff_formatter::{format, write, LineWidth};

#[derive(Debug, Copy, Clone)]
enum MyLabels {
    Main
}

impl tag::LabelDefinition for MyLabels {
    fn value(&self) -> u64 {
        *self as u64
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Main => "Main"
        }
    }
}

# fn main() -> FormatResult<()> {
let formatted = format!(
    SimpleFormatContext::default(),
    [format_with(|f| {
        let mut recording = f.start_recording();
        write!(recording, [
            labelled(
                LabelId::of(MyLabels::Main),
                &token("'I have a label'")
            )
        ])?;

        let recorded = recording.stop();

        let is_labelled = recorded.first().is_some_and( |element| element.has_label(LabelId::of(MyLabels::Main)));

        if is_labelled {
            write!(f, [token(" has label `Main`")])
        } else {
            write!(f, [token(" doesn't have label `Main`")])
        }
    })]
)?;

assert_eq!("'I have a label' has label `Main`", formatted.print()?.as_code());
# Ok(())
# }
```

## Alternatives

Use `Memoized.inspect(f)?.has_label(LabelId::of::<SomeLabelId>()` if you need to know if some content breaks that should
only be written later.

---

## line_suffix

`function` · `ruff_formatter::builders::line_suffix`

Also reachable as `ruff_formatter::prelude::line_suffix`

```rust
fn line_suffix<Content, Context>(inner: &Content, reserved_width: u32) -> LineSuffix<'_, Context> where Content: Format<Context>
```

Pushes some content to the end of the current line.

## Examples

```rust
use ruff_formatter::format;
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let elements = format!(SimpleFormatContext::default(), [
    token("a"),
    line_suffix(&token("c"), 0),
    token("b")
])?;

assert_eq!("abc", elements.print()?.as_code());
# Ok(())
# }
```

Provide reserved width for the line suffix to include it during measurement.
```rust
use ruff_formatter::{format, format_args, LineWidth, SimpleFormatContext, SimpleFormatOptions};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let context = SimpleFormatContext::new(SimpleFormatOptions {
    line_width: LineWidth::try_from(10).unwrap(),
    ..SimpleFormatOptions::default()
});

let elements = format!(context, [
    // Breaks
    group(&format_args![
        if_group_breaks(&token("(")),
        soft_block_indent(&format_args![token("a"), line_suffix(&token(" // a comment"), 13)]),
        if_group_breaks(&token(")"))
        ]),

    // Fits
    group(&format_args![
        if_group_breaks(&token("(")),
        soft_block_indent(&format_args![token("a"), line_suffix(&token(" // a comment"), 0)]),
        if_group_breaks(&token(")"))
    ]),
])?;
# assert_eq!("(\n\ta // a comment\n)a // a comment", elements.print()?.as_code());
# Ok(())
# }
```

---

## line_suffix_boundary

`function` · `ruff_formatter::builders::line_suffix_boundary`

Also reachable as `ruff_formatter::prelude::line_suffix_boundary`

```rust
const fn line_suffix_boundary() -> LineSuffixBoundary
```

Inserts a boundary for line suffixes that forces the printer to print all pending line suffixes.
Helpful if a line suffix shouldn't pass a certain point.

## Examples

Forces the line suffix "c" to be printed before the token `d`.
```
use ruff_formatter::format;
use ruff_formatter::prelude::*;

# fn  main() -> FormatResult<()> {
let elements = format!(SimpleFormatContext::default(), [
    token("a"),
    line_suffix(&token("c"), 0),
    token("b"),
    line_suffix_boundary(),
    token("d")
])?;

assert_eq!(
    "abc\nd",
    elements.print()?.as_code()
);
# Ok(())
# }
```

---

## soft_block_indent

`function` · `ruff_formatter::builders::soft_block_indent`

Also reachable as `ruff_formatter::prelude::soft_block_indent`

```rust
fn soft_block_indent<Context>(content: &impl Format<Context>) -> BlockIndent<'_, Context>
```

Indents the content by inserting a line break before and after the content and increasing
the indentation level for the content by one if the enclosing group doesn't fit on a single line.
Doesn't change the formatting if the enclosing group fits on a single line.

# Examples

Indents the content by one level and puts in new lines if the enclosing `Group` doesn't fit on a single line

```
use ruff_formatter::{format, format_args, LineWidth, SimpleFormatOptions};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let context = SimpleFormatContext::new(SimpleFormatOptions {
    line_width: LineWidth::try_from(10).unwrap(),
    ..SimpleFormatOptions::default()
});

let elements = format!(context, [
    group(&format_args![
        token("["),
        soft_block_indent(&format_args![
            token("'First string',"),
            soft_line_break_or_space(),
            token("'second string',"),
        ]),
        token("]"),
    ])
])?;

assert_eq!(
    "[\n\t'First string',\n\t'second string',\n]",
    elements.print()?.as_code()
);
# Ok(())
# }
```

Doesn't change the formatting if the enclosing `Group` fits on a single line
```
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let elements = format!(SimpleFormatContext::default(), [
    group(&format_args![
        token("["),
        soft_block_indent(&format_args![
            token("5,"),
            soft_line_break_or_space(),
            token("10"),
        ]),
        token("]"),
    ])
])?;

assert_eq!(
    "[5, 10]",
    elements.print()?.as_code()
);
# Ok(())
# }
```

---

## soft_line_break

`function` · `ruff_formatter::builders::soft_line_break`

Also reachable as `ruff_formatter::prelude::soft_line_break`

```rust
const fn soft_line_break() -> Line
```

A line break that only gets printed if the enclosing `Group` doesn't fit on a single line.
It's omitted if the enclosing `Group` fits on a single line.
A soft line break is identical to a hard line break when not enclosed inside of a `Group`.

# Examples

Soft line breaks are omitted if the enclosing `Group` fits on a single line

```
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let elements = format!(SimpleFormatContext::default(), [
    group(&format_args![token("a,"), soft_line_break(), token("b")])
])?;

assert_eq!(
    "a,b",
    elements.print()?.as_code()
);
# Ok(())
# }
```
See [`soft_line_break_or_space`] if you want to insert a space between the elements if the enclosing
`Group` fits on a single line.

Soft line breaks are emitted if the enclosing `Group` doesn't fit on a single line
```
use ruff_formatter::{format, format_args, LineWidth, SimpleFormatOptions};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let context = SimpleFormatContext::new(SimpleFormatOptions {
    line_width: LineWidth::try_from(10).unwrap(),
    ..SimpleFormatOptions::default()
});

let elements = format!(context, [
    group(&format_args![
        token("a long word,"),
        soft_line_break(),
        token("so that the group doesn't fit on a single line"),
    ])
])?;

assert_eq!(
    "a long word,\nso that the group doesn't fit on a single line",
    elements.print()?.as_code()
);
# Ok(())
# }
```

---

## soft_line_break_or_space

`function` · `ruff_formatter::builders::soft_line_break_or_space`

Also reachable as `ruff_formatter::prelude::soft_line_break_or_space`

```rust
const fn soft_line_break_or_space() -> Line
```

A line break if the enclosing `Group` doesn't fit on a single line, a space otherwise.

# Examples

The line breaks are emitted as spaces if the enclosing `Group` fits on a single line:
```
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let elements = format!(SimpleFormatContext::default(), [
    group(&format_args![
        token("a,"),
        soft_line_break_or_space(),
        token("b"),
    ])
])?;

assert_eq!(
    "a, b",
    elements.print()?.as_code()
);
# Ok(())
# }
```

The printer breaks the lines if the enclosing `Group` doesn't fit on a single line:
```
use ruff_formatter::{format_args, format, LineWidth, SimpleFormatOptions};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let context = SimpleFormatContext::new(SimpleFormatOptions {
    line_width: LineWidth::try_from(10).unwrap(),
    ..SimpleFormatOptions::default()
});

let elements = format!(context, [
    group(&format_args![
        token("a long word,"),
        soft_line_break_or_space(),
        token("so that the group doesn't fit on a single line"),
    ])
])?;

assert_eq!(
    "a long word,\nso that the group doesn't fit on a single line",
    elements.print()?.as_code()
);
# Ok(())
# }
```

---

## soft_line_indent_or_space

`function` · `ruff_formatter::builders::soft_line_indent_or_space`

Also reachable as `ruff_formatter::prelude::soft_line_indent_or_space`

```rust
fn soft_line_indent_or_space<Context>(content: &impl Format<Context>) -> BlockIndent<'_, Context>
```

If the enclosing `Group` doesn't fit on a single line, inserts a line break and indent.
Otherwise, just inserts a space.

Line indents are used to break a single line of code, and therefore only insert a line
break before the content and not after the content.

# Examples

Indents the content by one level and puts in new lines if the enclosing `Group` doesn't
fit on a single line. Otherwise, just inserts a space.

```
use ruff_formatter::{format, format_args, LineWidth, SimpleFormatOptions};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let context = SimpleFormatContext::new(SimpleFormatOptions {
    line_width: LineWidth::try_from(10).unwrap(),
    ..SimpleFormatOptions::default()
});

let elements = format!(context, [
    group(&format_args![
        token("name"),
        space(),
        token("="),
        soft_line_indent_or_space(&format_args![
            token("firstName"),
            space(),
            token("+"),
            space(),
            token("lastName"),
        ]),
    ])
])?;

assert_eq!(
    "name =\n\tfirstName + lastName",
    elements.print()?.as_code()
);
# Ok(())
# }
```

Only adds a space if the enclosing `Group` fits on a single line
```
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let elements = format!(SimpleFormatContext::default(), [
    group(&format_args![
        token("a"),
        space(),
        token("="),
        soft_line_indent_or_space(&token("10")),
    ])
])?;

assert_eq!(
    "a = 10",
    elements.print()?.as_code()
);
# Ok(())
# }
```

---

## soft_space_or_block_indent

`function` · `ruff_formatter::builders::soft_space_or_block_indent`

Also reachable as `ruff_formatter::prelude::soft_space_or_block_indent`

```rust
fn soft_space_or_block_indent<Context>(content: &impl Format<Context>) -> BlockIndent<'_, Context>
```

Adds spaces around the content if its enclosing group fits on a line, otherwise indents the content and separates it by line breaks.

# Examples

Adds line breaks and indents the content if the enclosing group doesn't fit on the line.

```
use ruff_formatter::{format, format_args, LineWidth, SimpleFormatOptions};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let context = SimpleFormatContext::new(SimpleFormatOptions {
    line_width: LineWidth::try_from(10).unwrap(),
    ..SimpleFormatOptions::default()
});

let elements = format!(context, [
    group(&format_args![
        token("{"),
        soft_space_or_block_indent(&format_args![
            token("aPropertyThatExceeds"),
            token(":"),
            space(),
            token("'line width'"),
        ]),
        token("}")
    ])
])?;

assert_eq!(
    "{\n\taPropertyThatExceeds: 'line width'\n}",
    elements.print()?.as_code()
);
# Ok(())
# }
```

Adds spaces around the content if the group fits on the line
```
use ruff_formatter::{format, format_args};
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let elements = format!(SimpleFormatContext::default(), [
    group(&format_args![
        token("{"),
        soft_space_or_block_indent(&format_args![
            token("a"),
            token(":"),
            space(),
            token("5"),
        ]),
        token("}")
    ])
])?;

assert_eq!(
    "{ a: 5 }",
    elements.print()?.as_code()
);
# Ok(())
# }
```

---

## source_position

`function` · `ruff_formatter::builders::source_position`

Also reachable as `ruff_formatter::prelude::source_position`

```rust
const fn source_position(position: ruff_text_size::TextSize) -> SourcePosition
```

Creates a source map entry from the passed source `position` to the position in the formatted output.

## Examples

```
use ruff_formatter::format;
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
// the tab must be encoded as \\t to not literally print a tab character ("Hello{tab}World" vs "Hello\tWorld")
use ruff_text_size::TextSize;
use ruff_formatter::SourceMarker;


let elements = format!(SimpleFormatContext::default(), [
    source_position(TextSize::new(0)),
    token("\"Hello "),
    source_position(TextSize::new(8)),
    token("'Ruff'"),
    source_position(TextSize::new(14)),
    token("\""),
    source_position(TextSize::new(20))
])?;

let printed = elements.print()?;

assert_eq!(printed.as_code(), r#""Hello 'Ruff'""#);
assert_eq!(printed.sourcemap(), [
    SourceMarker { source: TextSize::new(0), dest: TextSize::new(0) },
    SourceMarker { source: TextSize::new(8), dest: TextSize::new(7) },
    SourceMarker { source: TextSize::new(14), dest: TextSize::new(13) },
    SourceMarker { source: TextSize::new(20), dest: TextSize::new(14) },
]);

# Ok(())
# }
```

---

## source_text_slice

`function` · `ruff_formatter::builders::source_text_slice`

Also reachable as `ruff_formatter::prelude::source_text_slice`

```rust
const fn source_text_slice(range: ruff_text_size::TextRange) -> SourceTextSliceBuilder
```

Emits a text as it is written in the source document. Optimized to avoid allocations.

---

## space

`function` · `ruff_formatter::builders::space`

Also reachable as `ruff_formatter::prelude::space`

```rust
const fn space() -> Space
```

Inserts a single space. Allows to separate different tokens.

# Examples

```
use ruff_formatter::format;
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
// the tab must be encoded as \\t to not literally print a tab character ("Hello{tab}World" vs "Hello\tWorld")
let elements = format!(SimpleFormatContext::default(), [token("a"), space(), token("b")])?;

assert_eq!("a b", elements.print()?.as_code());
# Ok(())
# }
```

---

## text

`function` · `ruff_formatter::builders::text`

Also reachable as `ruff_formatter::prelude::text`

```rust
fn text(text: &str) -> Text<'_>
```

Creates a text from a dynamic string.

This is done by allocating a new string internally.

---

## token

`function` · `ruff_formatter::builders::token`

Also reachable as `ruff_formatter::prelude::token`

```rust
fn token(text: &'static str) -> Token
```

Creates a token that gets written as is to the output. A token must be ASCII only and is not allowed
to contain any line breaks or tab characters.

# Examples

```
use ruff_formatter::format;
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
let elements = format!(SimpleFormatContext::default(), [token("Hello World")])?;

assert_eq!(
    "Hello World",
    elements.print()?.as_code()
);
# Ok(())
# }
```

Printing a string literal as a literal requires that the string literal is properly escaped and
enclosed in quotes (depending on the target language).

```
use ruff_formatter::format;
use ruff_formatter::prelude::*;

# fn main() -> FormatResult<()> {
// the tab must be encoded as \\t to not literally print a tab character ("Hello{tab}World" vs "Hello\tWorld")
let elements = format!(SimpleFormatContext::default(), [token("\"Hello\\tWorld\"")])?;

assert_eq!(r#""Hello\tWorld""#, elements.print()?.as_code());
# Ok(())
# }
```

---

## Align

`struct` · `ruff_formatter::builders::Align`

Also reachable as `ruff_formatter::prelude::Align`

```rust
struct Align<'a, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## BestFitParenthesize

`struct` · `ruff_formatter::builders::BestFitParenthesize`

Also reachable as `ruff_formatter::prelude::BestFitParenthesize`

```rust
struct BestFitParenthesize<'a, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug

**Methods** (1)

```rust
fn with_group_id(self, group_id: Option<GroupId>) -> Self
```

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## BestFitting

`struct` · `ruff_formatter::builders::BestFitting`

Also reachable as `ruff_formatter::BestFitting`, `ruff_formatter::prelude::BestFitting`

```rust
struct BestFitting<'a, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy

**Methods** (2)

```rust
const fn from_arguments_unchecked(variants: Arguments<'a, Context>) -> Self
fn with_mode(self, mode: BestFittingMode) -> Self
```

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

The first variant is the most flat, and the last is the most expanded variant.
See [`best_fitting!`] macro for a more in-detail documentation

---

## BlockIndent

`struct` · `ruff_formatter::builders::BlockIndent`

Also reachable as `ruff_formatter::prelude::BlockIndent`

```rust
struct BlockIndent<'a, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## ConditionalGroup

`struct` · `ruff_formatter::builders::ConditionalGroup`

Also reachable as `ruff_formatter::prelude::ConditionalGroup`

```rust
struct ConditionalGroup<'content, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Debug

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## Dedent

`struct` · `ruff_formatter::builders::Dedent`

Also reachable as `ruff_formatter::prelude::Dedent`

```rust
struct Dedent<'a, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## ExpandParent

`struct` · `ruff_formatter::builders::ExpandParent`

Also reachable as `ruff_formatter::prelude::ExpandParent`

```rust
struct ExpandParent
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## FillBuilder

`struct` · `ruff_formatter::builders::FillBuilder`

Also reachable as `ruff_formatter::prelude::FillBuilder`

```rust
struct FillBuilder<'fmt, 'buf, Context>
```

**Methods** (3)

```rust
fn entries<F, I>(&mut self, separator: &dyn Format<Context>, entries: I) -> &mut Self where F: Format<Context>, I: IntoIterator<Item = F>
fn entry(&mut self, separator: &dyn Format<Context>, entry: &dyn Format<Context>) -> &mut Self
fn finish(&mut self) -> FormatResult<()>
```

Builder to fill as many elements as possible on a single line.

---

## FitsExpanded

`struct` · `ruff_formatter::builders::FitsExpanded`

Also reachable as `ruff_formatter::prelude::FitsExpanded`

```rust
struct FitsExpanded<'a, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone

**Methods** (1)

```rust
fn with_condition(self, condition: Option<Condition>) -> Self
```

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## FormatLabelled

`struct` · `ruff_formatter::builders::FormatLabelled`

Also reachable as `ruff_formatter::prelude::FormatLabelled`

```rust
struct FormatLabelled<'a, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## FormatOnce

`struct` · `ruff_formatter::builders::FormatOnce`

Also reachable as `ruff_formatter::prelude::FormatOnce`

```rust
struct FormatOnce<T, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Debug

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## FormatWith

`struct` · `ruff_formatter::builders::FormatWith`

Also reachable as `ruff_formatter::prelude::FormatWith`

```rust
struct FormatWith<Context, T>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

Utility for formatting some content with an inline lambda function.

---

## Group

`struct` · `ruff_formatter::builders::Group`

Also reachable as `ruff_formatter::prelude::Group`

```rust
struct Group<'a, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug

**Methods** (2)

```rust
fn should_expand(self, should_expand: bool) -> Self
fn with_id(self, group_id: Option<GroupId>) -> Self
```

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## IfGroupBreaks

`struct` · `ruff_formatter::builders::IfGroupBreaks`

Also reachable as `ruff_formatter::prelude::IfGroupBreaks`

```rust
struct IfGroupBreaks<'a, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug

**Methods** (1)

```rust
fn with_group_id(self, group_id: Option<GroupId>) -> Self
```

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## Indent

`struct` · `ruff_formatter::builders::Indent`

Also reachable as `ruff_formatter::prelude::Indent`

```rust
struct Indent<'a, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## IndentIfGroupBreaks

`struct` · `ruff_formatter::builders::IndentIfGroupBreaks`

Also reachable as `ruff_formatter::prelude::IndentIfGroupBreaks`

```rust
struct IndentIfGroupBreaks<'a, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## JoinBuilder

`struct` · `ruff_formatter::builders::JoinBuilder`

Also reachable as `ruff_formatter::prelude::JoinBuilder`

```rust
struct JoinBuilder<'fmt, 'buf, Separator, Context>
```

**Methods** (3)

```rust
fn entries<F, I>(&mut self, entries: I) -> &mut Self where F: Format<Context>, I: IntoIterator<Item = F>
fn entry(&mut self, entry: &dyn Format<Context>) -> &mut Self
fn finish(&mut self) -> FormatResult<()>
```

Builder to join together a sequence of content.
See [`Formatter::join`]

---

## Line

`struct` · `ruff_formatter::builders::Line`

Also reachable as `ruff_formatter::prelude::Line`

```rust
struct Line
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## LineSuffix

`struct` · `ruff_formatter::builders::LineSuffix`

Also reachable as `ruff_formatter::prelude::LineSuffix`

```rust
struct LineSuffix<'a, Context>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## LineSuffixBoundary

`struct` · `ruff_formatter::builders::LineSuffixBoundary`

Also reachable as `ruff_formatter::prelude::LineSuffixBoundary`

```rust
struct LineSuffixBoundary
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## SourcePosition

`struct` · `ruff_formatter::builders::SourcePosition`

Also reachable as `ruff_formatter::prelude::SourcePosition`

```rust
struct SourcePosition
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## SourceTextSliceBuilder

`struct` · `ruff_formatter::builders::SourceTextSliceBuilder`

Also reachable as `ruff_formatter::prelude::SourceTextSliceBuilder`

```rust
struct SourceTextSliceBuilder
```

**Implements**: `ruff_formatter::Format`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## Space

`struct` · `ruff_formatter::builders::Space`

Also reachable as `ruff_formatter::prelude::Space`

```rust
struct Space
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## Text

`struct` · `ruff_formatter::builders::Text`

Also reachable as `ruff_formatter::prelude::Text`

```rust
struct Text<'a>
```

**Implements**: `ruff_formatter::Format`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---

## Token

`struct` · `ruff_formatter::builders::Token`

Also reachable as `ruff_formatter::prelude::Token`

```rust
struct Token
```

**Implements**: `ruff_formatter::Format`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `ruff_formatter::Format`**

```rust
fn fmt(&self, f: &mut Formatter<'_, Context>) -> FormatResult<()>
```

---
