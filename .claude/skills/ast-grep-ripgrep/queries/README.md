# Rules

Questions, not prohibitions. Every rule carries `severity: hint` and no `fix`, and
`sgconfig.yml` is never auto-discovered — it is always passed with `-c`, precisely so these
hints cannot leak into a repository's own enforcement corpus and become a policy nobody agreed
to.

## Three families, split by what a hit means

| Family | Runs against | A hit is |
|---|---|---|
| `project-*` | the repository you are editing | a question about your code |
| `model-*` | `content/model/*.json` | a fact about the indexed library surface |
| `corpus-*` | `content/corpus/` | a location in vendored upstream material — evidence |

```
ast-grep scan -c queries/sgconfig.yml --filter '^project-' PATH
ast-grep scan -c queries/sgconfig.yml --filter '^model-' content/model
ast-grep scan -c queries/sgconfig.yml --filter '^corpus-' content/corpus/ripgrep/tests
```

The `project-` family is the one to run unprompted after writing a search or a rule. Each targets
a mistake that does not fail loudly: `--json` with a space silently turns the style into a path;
`-uuu` searches `.git` and every build artifact; `sg` may resolve to the setgroups binary; and
the UTS#18 class form compiles as an ordinary class rather than as set algebra, so it matches the
wrong thing instead of erroring.

## Four traps paid for here

**A backtick cannot start a plain YAML scalar.** `message: `-uuu` disables …` is a parse error,
and ast-grep reports it by failing the whole scan with exit 8 — not by skipping one rule. Quote
any message starting with a backtick or `@`.

**`ast-grep test` exits 0 while reporting failures.** A run printing `0 passed; 4 failed` still
returns 0. `verify.py` therefore parses the summary line and asserts on the counts. Never gate on
the status.

**`--filter` matching nothing exits 3.** A typo in a filter looks like a clean run with no
findings unless you check for it specifically.

**`--update-all` always exits 0**, so it can never be part of a gate. Use it to record snapshots,
then re-run without it.

## Anchor structurally, refine with regex

Every rule here anchors on a node kind first — `command` for shell, `pair` for JSON — and only
then applies `regex` to the node's text. That ordering matters: it is what stops a mention of
`--json stream` inside a comment or a heredoc from matching. Regex used *after* a structural
anchor is refinement; regex used *instead* of one is a parser rebuilt badly.

## Adding a rule

1. Prototype with `ast-grep scan --inline-rules` before creating a file.
2. Anchor on a positive `kind`. Add relations second, and bound `stopBy`.
3. Look field names up in `content/index/fields.tsv` rather than guessing — 631 of them are
   indexed, and a wrong field name matches nothing rather than erroring.
4. Write `rule-tests/<id>-test.yml` with both `valid:` and `invalid:` cases.
5. Record the snapshot with `--update-all`, then re-run without it and read the summary line.
6. Write the `note:` so it explains *why* the hit matters. A rule whose note only restates the
   pattern has told the reader nothing they could not see.
