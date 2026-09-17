# Writing and changing rules

These are **questions, not prohibitions**. Every rule is `severity: hint`, none carries a `fix`,
and they are meant to be run deliberately with `-c` rather than mixed into an enforcement corpus
where a hint quietly becomes a gate.

```bash
ast-grep scan -c queries/sgconfig.yml --filter '^project-' queries/fixtures
ast-grep test -c queries/sgconfig.yml
```

Swap `queries/fixtures` for the repository you are editing. The fixtures exist so the scan has a
runnable target that fires, and so an agent can see what firing looks like before pointing it at
something that matters.

## The rules here are generated

All three `project-*` rules are written by `build/queries.py` from the breaking-change rows in
`build/catalogs.json`, joined to the model. That is deliberate: the hand-written rule they
replace would be exact only on the day it was typed, and a re-pin that changes the surface
changes the rule with it. Do not hand-edit anything under `rules/generated/` — a rebuild
overwrites it and the edit is lost silently.

To add a rule, add the breaking-change row and a generator beside the others. `write_all` raises
if a row names a rule nobody generates, so a named-but-missing rule cannot ship as a row that
silently does nothing.

## Three traps measured on this grammar

**A backtick cannot start a plain YAML scalar.** Every message here begins with a backtick
around a symbol name, and YAML reserves `` ` `` as an indicator character. ast-grep reports this
as "found character that cannot start any token", which reads like a rule-syntax error rather
than a quoting one. `_rule` JSON-quotes the message for this reason.

**A test naming a rule id that does not exist passes.** `ast-grep test` reports
`0 passed; 0 failed` and exits 0, so a typo in a test's `id` deletes the test rather than failing
it. `verify.py` asserts on the number of cases actually executed, and that the count is at least
the number of rule files.

**`--update-all` always exits 0.** It rewrites snapshots and reports success either way, so it can
never be part of a gate. Run it once, read the snapshot, then commit.

## Adding a rule

1. Prototype with `ast-grep run -p ... --debug-query=cst` until the parse is what you expect.
2. Write the rule with a `note:` that says what to do, not just what is wrong.
3. Add `rule-tests/<id>-test.yml` with **both** `valid:` and `invalid:` cases. The valid cases
   are the ones that matter: a rule with no valid fixture has not been shown to be precise.
4. `ast-grep test -c sgconfig.yml --update-all` once, then read the snapshot before committing.
5. `python3 build/verify.py`.

A rule that fires on correct code is worse than no rule.

## What these rules cannot do

ast-grep establishes syntax, not semantics: it resolves no imports and no types. So
`project-vendored-click-import` cannot tell a Typer project from any other project that imports
`click` — it finds the import and asks the question. That is the right shape for the finding,
because the failure it points at is one no type checker will raise: the `click` you import
type-checks perfectly and is a different library from the one Typer runs.
