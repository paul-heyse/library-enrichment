# Writing and changing rules

These rules are **questions**, not prohibitions. They carry `severity: hint` and no `fix`. A match
means "here is something worth knowing", never "here is a violation". Keep them out of any
repository's enforcement corpus, and always pass this config explicitly:

```bash
ast-grep scan -c queries/sgconfig.yml --filter '^project-' <path>
```

Upstream reference: <https://ast-grep.github.io/llms-full.txt>. Read it rather than inferring
syntax. What follows is only what this pack depends on, plus the traps that actually bit.

## Reach for the simplest thing that expresses the invariant

`kind` → pattern → atomic rule → relational → composite. Stop at the first rung that works.
Complex YAML is not more precise than a clear structural match; it is just more assumptions to
get wrong.

## Traps, each one paid for

**Patterns are formatting-sensitive; `kind`-anchored rules are not.** A pattern for
`fn supports_filters_pushdown($$$) -> $R { $$$ }` silently missed a real declaration written
across four lines. The equivalent `kind: function_item` + `has: {field: name, regex: ...}` found
it. Use patterns for whole-item shapes like `impl $TRAIT for $TYPE`; use rules for anything that
matches a declaration.

**`stopBy: end` is required for descendant matching.** The default inspects immediate children
only, so a `has:` that looks obviously correct can match nothing.

**Every rule needs a positive matcher at every level.** `has: {field: trait}` is rejected with
"Rule must have one positive matcher" — add `regex: '.'` or a `kind:`.

**Utils resolve only through the project config.** `matches: json-record` works under
`-c sgconfig.yml` and silently matches nothing under `--inline-rules`, which does not consult
`utilDirs`. Prototype with a rule file, not an inline rule.

**ast-grep resolves no imports.** This is the big one. An unanchored identifier rule for the 116
deprecated items produced 47 hits on a real workspace, *all* false — local variables, module
names, and `tokio::sync::oneshot::channel`. The shipped version matches only qualified paths
rooted in a pinned crate, and only names nothing undeprecated shares. It therefore misses
`use arrow::x::name; name()`. That trade is deliberate: a hint rule that cries wolf gets ignored.

## Testing is not optional

Every rule has `valid` and `invalid` fixtures in `rule-tests/`. A rule with only "should match"
cases can still be unusably noisy, and one with only "should not match" cases can be silently
dead.

```bash
ast-grep test -c queries/sgconfig.yml                # the gate
ast-grep test -c queries/sgconfig.yml --update-all   # authoring only, never a gate
```

Three exit-code hazards, which `build/verify.py` handles:

- A test file naming a rule `id` that does not exist **exits 0**, reporting `0 passed; 0 failed`.
  A typo deletes the test instead of failing it — so assert on the number of cases *executed*.
- `--filter` matching nothing exits **3**, not 0.
- `--update-all` rewrites snapshots and then always exits 0. It can never be part of a gate.

`ast-grep test` never returns 1. Check zero versus nonzero, not a specific code.

## Layout

```
sgconfig.yml            ruleDirs, utilDirs, testConfigs
rules/model/            questions over content/model/*.json   (language: json)
rules/corpus/           questions over content/corpus/*.rs    (language: rust)
rules/project/          capability gaps in the repo being edited (language: rust)
rules/generated/        written by build/queries.py -- never hand-edit
utils/                  shared sub-rules, including parameterized ones
outline/deltalake.yml   custom outline extractor
rule-tests/             fixtures and __snapshots__
```

Generated rules are regenerated from the model on every build, so they cannot go stale. Editing
one by hand loses the edit on the next `python3 build/build.py`.

## Adding a rule

1. Write it under the family it belongs to, with `metadata.record` naming what it reports.
2. Write `rule-tests/<id>-test.yml` with at least one `valid` and one `invalid` case. The `id`
   must match exactly or the test silently disappears.
3. `ast-grep test -c queries/sgconfig.yml --update-all` once, to record snapshots.
4. Run it against real code and count the matches. If it fires on everything, it is noise —
   anchor it further before shipping.
5. `python3 build/verify.py` to confirm the suite still covers every rule.
