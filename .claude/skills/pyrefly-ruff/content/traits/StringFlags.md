# StringFlags

`ruff_python_ast::nodes::StringFlags`

```rust
trait StringFlags: Copy
```

Also reachable as `ruff_python_ast::StringFlags`

Prose: [`api/ruff_python_ast.nodes.md`](../api/ruff_python_ast.nodes.md#stringflags) · records: [`model/ruff_python_ast.nodes.json`](../model/ruff_python_ast.nodes.json)

## Required

Every implementation must supply these.

```rust
fn is_unclosed(self) -> bool
fn prefix(self) -> AnyStringPrefix
fn quote_style(self) -> Quote
fn triple_quotes(self) -> TripleQuotes
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn as_any_string_flags(self) -> AnyStringFlags
fn closer_len(self) -> TextSize
fn display_contents(self, contents: &str) -> DisplayFlags<'_>
fn is_triple_quoted(self) -> bool
fn opener_len(self) -> TextSize
fn quote_len(self) -> TextSize
fn quote_str(self) -> &'static str
```

## Implementors (6)

Read one before writing your own.

- `ruff_python_ast::nodes::AnyStringFlags`
- `ruff_python_ast::nodes::BytesLiteralFlags`
- `ruff_python_ast::nodes::FStringFlags`
- `ruff_python_ast::nodes::StringLiteralFlags`
- `ruff_python_ast::nodes::TStringFlags`
- `ruff_python_ast::token::TokenFlags`
