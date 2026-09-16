# `ruff_python_parser`

Crate `ruff_python_parser` · 13 public items · structured records in [`model/ruff_python_parser.json`](../model/ruff_python_parser.json)

## Mode

`enum` · `ruff_python_parser::Mode`

```rust
enum Mode
```

**Variants**: `Module`, `Expression`, `ParenthesizedExpression`, `Ipython`

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, ModeParseError>
```

Control in the different modes by which a source file can be parsed.

The mode argument specifies in what way code must be parsed.

---

## parse

`function` · `ruff_python_parser::parse`

```rust
fn parse(source: &str, options: ParseOptions) -> Result<Parsed<ruff_python_ast::Mod>, ParseError>
```

Parse the given Python source code using the specified [`ParseOptions`].

This function is the most general function to parse Python code. Based on the [`Mode`] supplied
via the [`ParseOptions`], it can be used to parse a single expression, a full Python program,
an interactive expression or a Python program containing IPython escape commands.

# Example

If we want to parse a simple expression, we can use the [`Mode::Expression`] mode during
parsing:

```
use ruff_python_parser::{parse, Mode, ParseOptions};

let parsed = parse("1 + 2", ParseOptions::from(Mode::Expression));
assert!(parsed.is_ok());
```

Alternatively, we can parse a full Python program consisting of multiple lines:

```
use ruff_python_parser::{parse, Mode, ParseOptions};

let source = r#"
class Greeter:

  def greet(self):
   print("Hello, world!")
"#;
let parsed = parse(source, ParseOptions::from(Mode::Module));
assert!(parsed.is_ok());
```

Additionally, we can parse a Python program containing IPython escapes:

```
use ruff_python_parser::{parse, Mode, ParseOptions};

let source = r#"
%timeit 1 + 2
?str.replace
!ls
"#;
let parsed = parse(source, ParseOptions::from(Mode::Ipython));
assert!(parsed.is_ok());
```

---

## parse_cells_unchecked

`function` · `ruff_python_parser::parse_cells_unchecked`

```rust
fn parse_cells_unchecked(source: &str, ranges: impl IntoIterator<Item = ruff_text_size::TextRange>, options: &ParseOptions) -> Parsed<ruff_python_ast::ModModule>
```

Parses each `range` of `source` as an independent module and concatenates the results into a
single [`Parsed<ModModule>`] whose nodes keep their offsets into `source`.

This validates sources such as Jupyter notebooks, where each cell must be syntactically valid on
its own while later cells can still reference earlier definitions.
The `ranges` must be ordered and non-overlapping.

Consecutive ranges must be separated by a single-byte `\n`: each range ends just before the
separator and the next range starts just after it, as Ruff's notebook cells are. A syntax error
anchored at a cell's trailing offset then lands on that separator, the cell's own last line, so
it is attributed to that cell rather than to the following one.

---

## parse_expression

`function` · `ruff_python_parser::parse_expression`

```rust
fn parse_expression(source: &str) -> Result<Parsed<ruff_python_ast::ModExpression>, ParseError>
```

Parses a single Python expression.

This convenience function can be used to parse a single expression without having to
specify the Mode or the location.

# Example

For example, parsing a single expression denoting the addition of two numbers:

```
use ruff_python_parser::parse_expression;

let expr = parse_expression("1 + 2");
assert!(expr.is_ok());
```

---

## parse_expression_range

`function` · `ruff_python_parser::parse_expression_range`

```rust
fn parse_expression_range(source: &str, range: ruff_text_size::TextRange) -> Result<Parsed<ruff_python_ast::ModExpression>, ParseError>
```

Parses a Python expression for the given range in the source.

This function allows to specify the range of the expression in the source code, other than
that, it behaves exactly like [`parse_expression`].

# Example

Parsing one of the numeric literal which is part of an addition expression:

```
use ruff_python_parser::parse_expression_range;
# use ruff_text_size::{TextRange, TextSize};

let parsed = parse_expression_range("11 + 22 + 33", TextRange::new(TextSize::new(5), TextSize::new(7)));
assert!(parsed.is_ok());
```

---

## parse_module

`function` · `ruff_python_parser::parse_module`

```rust
fn parse_module(source: &str) -> Result<Parsed<ruff_python_ast::ModModule>, ParseError>
```

Parse a full Python module usually consisting of multiple lines.

This is a convenience function that can be used to parse a full Python program without having to
specify the [`Mode`] or the location. It is probably what you want to use most of the time.

# Example

For example, parsing a simple function definition and a call to that function:

```
use ruff_python_parser::parse_module;

let source = r#"
def foo():
   return 42

print(foo())
"#;

let module = parse_module(source);
assert!(module.is_ok());
```

---

## parse_parenthesized_expression_range

`function` · `ruff_python_parser::parse_parenthesized_expression_range`

```rust
fn parse_parenthesized_expression_range(source: &str, range: ruff_text_size::TextRange) -> Result<Parsed<ruff_python_ast::ModExpression>, ParseError>
```

Parses a Python expression as if it is parenthesized.

It behaves similarly to [`parse_expression_range`] but allows what would be valid within parenthesis

# Example

Parsing an expression that would be valid within parenthesis:

```
use ruff_python_parser::parse_parenthesized_expression_range;
# use ruff_text_size::{TextRange, TextSize};

let parsed = parse_parenthesized_expression_range("'''\n int | str'''", TextRange::new(TextSize::new(3), TextSize::new(14)));
assert!(parsed.is_ok());

---

## parse_string_annotation

`function` · `ruff_python_parser::parse_string_annotation`

```rust
fn parse_string_annotation(source: &str, string: &ruff_python_ast::StringLiteral) -> Result<Parsed<ruff_python_ast::ModExpression>, ParseError>
```

Parses a Python expression from a string annotation.

# Example

Parsing a string annotation:

```
use ruff_python_parser::parse_string_annotation;
use ruff_python_ast::{StringLiteral, StringLiteralFlags, AtomicNodeIndex};
use ruff_text_size::{TextRange, TextSize};

let string = StringLiteral {
    value: "'''\n int | str'''".to_string().into_boxed_str(),
    flags: StringLiteralFlags::empty(),
    range: TextRange::new(TextSize::new(0), TextSize::new(16)),
    node_index: AtomicNodeIndex::NONE
};
let parsed = parse_string_annotation("'''\n int | str'''", &string);
assert!(!parsed.is_ok());
```

---

## parse_unchecked

`function` · `ruff_python_parser::parse_unchecked`

```rust
fn parse_unchecked(source: &str, options: ParseOptions) -> Parsed<ruff_python_ast::Mod>
```

Parse the given Python source code using the specified [`ParseOptions`].

This is same as the [`parse`] function except that it doesn't check for any [`ParseError`]
and returns the [`Parsed`] as is.

---

## parse_unchecked_source

`function` · `ruff_python_parser::parse_unchecked_source`

```rust
fn parse_unchecked_source(source: &str, source_type: ruff_python_ast::PySourceType) -> Parsed<ruff_python_ast::ModModule>
```

Parse the given Python source code using the specified [`PySourceType`].

---

## ModeParseError

`struct` · `ruff_python_parser::ModeParseError`

```rust
struct ModeParseError
```

**Implements**: `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Returned when a given mode is not valid.

---

## Parsed

`struct` · `ruff_python_parser::Parsed`

```rust
struct Parsed<T>
```

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (15)

```rust
fn as_result(&self) -> Result<&Parsed<T>, &[ParseError]>
fn errors(&self) -> &[ParseError]
fn expr(&self) -> &Expr
fn has_invalid_syntax(&self) -> bool
fn has_no_syntax_errors(&self) -> bool
fn has_syntax_errors(&self) -> bool
fn has_valid_syntax(&self) -> bool
fn into_expr(self) -> Expr
fn into_suite(self) -> Suite
fn into_syntax(self) -> T
fn suite(&self) -> &Suite
fn syntax(&self) -> &T
fn tokens(&self) -> &Tokens
fn try_into_module(self) -> Option<Parsed<ModModule>>
fn unsupported_syntax_errors(&self) -> &[UnsupportedSyntaxError]
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

Represents the parsed source code.

---

## AsMode

`trait` · `ruff_python_parser::AsMode`

```rust
trait AsMode
```

**Implementors** (1)

- `ruff_python_ast::PySourceType`

**Methods** (1)

```rust
fn as_mode(&self) -> Mode
```

A type that can be represented as [Mode].

---
