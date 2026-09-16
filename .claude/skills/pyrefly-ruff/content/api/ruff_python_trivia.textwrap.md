# `ruff_python_trivia::textwrap`

Crate `ruff_python_trivia` · 4 public items · structured records in [`model/ruff_python_trivia.textwrap.json`](../model/ruff_python_trivia.textwrap.json)

## dedent

`function` · `ruff_python_trivia::textwrap::dedent`

```rust
fn dedent(text: &str) -> std::borrow::Cow<'_, str>
```

Removes common leading whitespace from each line.

This function will look at each non-empty line and determine the
maximum amount of whitespace that can be removed from all lines.

Lines that consist solely of whitespace are trimmed to a blank line.

```
# use ruff_python_trivia::textwrap::dedent;

assert_eq!(dedent("
    1st line
      2nd line
    3rd line
"), "
1st line
  2nd line
3rd line
");
```

---

## dedent_to

`function` · `ruff_python_trivia::textwrap::dedent_to`

```rust
fn dedent_to(text: &str, indent: &str) -> Option<String>
```

Reduce a block's indentation to match the provided indentation.

This function looks at the first line in the block to determine the
current indentation, then removes whitespace from each line to
match the provided indentation.

Leading comments are ignored unless the block is only composed of comments.

Lines that are indented by _less_ than the indent of the first line
are left unchanged.

Lines that consist solely of whitespace are trimmed to a blank line.

Lines that start with formfeeds have the indentation after the formfeeds
removed and the formfeeds reinstated

# Panics
If the first line is indented by less than the provided indent.

---

## indent

`function` · `ruff_python_trivia::textwrap::indent`

```rust
fn indent<'a>(text: &'a str, prefix: &str) -> std::borrow::Cow<'a, str>
```

Indent each line by the given prefix.

# Examples

```
# use ruff_python_trivia::textwrap::indent;

assert_eq!(indent("First line.\nSecond line.\n", "  "),
           "  First line.\n  Second line.\n");
```

When indenting, trailing whitespace is stripped from the prefix.
This means that empty lines remain empty afterwards:

```
# use ruff_python_trivia::textwrap::indent;

assert_eq!(indent("First line.\n\n\nSecond line.\n", "  "),
           "  First line.\n\n\n  Second line.\n");
```

Notice how `"\n\n\n"` remained as `"\n\n\n"`.

This feature is useful when you want to indent text and have a
space between your prefix and the text. In this case, you _don't_
want a trailing space on empty lines:

```
# use ruff_python_trivia::textwrap::indent;

assert_eq!(indent("foo = 123\n\nprint(foo)\n", "# "),
           "# foo = 123\n#\n# print(foo)\n");
```

Notice how `"\n\n"` became `"\n#\n"` instead of `"\n# \n"` which
would have trailing whitespace.

Leading and trailing whitespace coming from the text itself is
kept unchanged:

```
# use ruff_python_trivia::textwrap::indent;

assert_eq!(indent(" \t  Foo   ", "->"), "-> \t  Foo   ");
```

---

## indent_first_line

`function` · `ruff_python_trivia::textwrap::indent_first_line`

```rust
fn indent_first_line<'a>(text: &'a str, prefix: &str) -> std::borrow::Cow<'a, str>
```

Indent only the first line by the given prefix.

This function is useful when you want to indent the first line of a multi-line
expression while preserving the relative indentation of subsequent lines.

# Examples

```
# use ruff_python_trivia::textwrap::indent_first_line;

assert_eq!(indent_first_line("First line.\nSecond line.\n", "  "),
           "  First line.\nSecond line.\n");
```

When indenting, trailing whitespace is stripped from the prefix.
This means that empty lines remain empty afterwards:

```
# use ruff_python_trivia::textwrap::indent_first_line;

assert_eq!(indent_first_line("\n\n\nSecond line.\n", "  "),
           "\n\n\nSecond line.\n");
```

Leading and trailing whitespace coming from the text itself is
kept unchanged:

```
# use ruff_python_trivia::textwrap::indent_first_line;

assert_eq!(indent_first_line(" \t  Foo   ", "->"), "-> \t  Foo   ");
```

---
