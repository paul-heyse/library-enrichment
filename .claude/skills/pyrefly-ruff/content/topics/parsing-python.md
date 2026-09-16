# Parsing Python

`ruff_python_parser` is the front end for both toolchains. It parses to a concrete syntax tree with full trivia preserved and recovers from errors rather than bailing, which is what lets a formatter and a language server share one parser. A `Parsed` carries both the tree and the errors; an error does not mean you have no tree.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `ruff_python_parser::Parsed` | struct | 20 | [prose](../api/ruff_python_parser.md#parsed) | [records](../model/ruff_python_parser.json) |
| `ruff_python_parser::error::ParseError` | struct | 10 | [prose](../api/ruff_python_parser.error.md#parseerror) | [records](../model/ruff_python_parser.error.json) |
| `ruff_python_ast::generated::Mod` | enum | 17 | [prose](../api/ruff_python_ast.generated.md#mod) | [records](../model/ruff_python_ast.generated.json) |

## Decision rules

- Parsing a whole file? `parse_module`. An expression? `parse_expression`.
- Need the original text ranges? Everything is a `TextRange` into the source; keep the source.

## Anti-patterns

- Discarding the source after parsing -- every range is an offset into it.
- Assuming a parse error means an empty tree.

## Agent checklist

- Pick the target Python version explicitly; the grammar depends on it.
