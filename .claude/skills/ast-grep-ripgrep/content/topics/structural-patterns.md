# Matching code by its shape

**Find this code construct, wherever and however it is formatted.**

An ast-grep pattern is **code**, parsed with the same grammar as the file it searches, then matched as a tree. It is not a wildcard syntax layered over text. That is why it is immune to formatting: a call split across four lines is the same tree as one on a single line.

Metavariables are the wildcards. `$A` captures one named node; `$$$ARGS` captures a run of them; `$_` matches without capturing. A repeated capturing name imposes **equality** -- `$A == $A` matches `x == x` and not `x == y` -- which is powerful and is still syntactic equality, not proof that both names resolve to the same declaration.

When a pattern will not match, inspect the parse before making the pattern cleverer. `--debug-query=cst` shows what the grammar actually produced, including the unnamed punctuation nodes that ordinary metavariables do not capture. Most failed patterns are a misunderstanding about which node is the root, not a missing feature.

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| A001 | confirmed | Does a code-shaped pattern match a call? | `ast-grep run -l rust -p println!($$$ARGS) src/main.rs` |
| A002 | confirmed | Does a repeated capturing metavariable enforce equality? | `ast-grep run -l python -p print($A) src/app.py` |
| A003 | recorded | Is a bare multi-metavariable accepted as the whole pattern? | `ast-grep run -l rust -p $$$A src/main.rs` |
| A004 | confirmed | Does --kind accept an ESQuery-style selector? | `ast-grep run -l rust -k function_item src/main.rs` |
| A005 | confirmed | Does the ESQuery direct-child combinator work in --kind? | `ast-grep run -l rust -k function_item > identifier src/lib.rs` |
| A006 | recorded | Does strictness change whether a comment defeats a match? | `ast-grep run -l rust -p x + 1 --strictness relaxed src/main.rs` |
| A011 | recorded | Does --debug-query show how the pattern parsed? | `ast-grep run -l rust -p helper($X) --debug-query=cst src/main.rs` |

## Decision rules

- If a node kind alone expresses it, use `-k`. It is exact and needs no pattern to parse.
- Give the pattern enough context to parse, then use `--selector` to pick the node you actually meant.
- Choose strictness from the invariants the task depends on, not from habit. Stricter is not safer; it is more brittle to formatting.
- Look up the kind in `kinds.tsv` rather than guessing -- and note the three kinds this binary rejects despite being published in `languages.json`.
- Field names for `has: {field: ...}` live in `fields.tsv`. Guessing them is the commonest reason a relational rule matches nothing.

## Anti-patterns

- Treating `$X` as `.*`. It is one node.
- A bare `$$$A` as the whole pattern.
- Lowercase metavariable names -- they are not metavariables.
- Expecting `$OP` to capture an operator. Operators are usually unnamed nodes; `$$OP` is the form.
- Adding relational clauses to fix a pattern whose root node was wrong.

## Checklist

- Would `-k` do?
- Has `--debug-query=cst` been run?
- Is repeated-metavariable equality intended?
- Zero, one and many cases all tested for `$$$`?
