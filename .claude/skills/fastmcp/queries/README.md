# Writing and changing rules

These are **questions, not prohibitions**. Every rule is `severity: hint`, none carries a `fix`,
and they are meant to be run deliberately with `-c` rather than mixed into an enforcement
corpus where a hint quietly becomes a gate.

```bash
ast-grep scan -c queries/sgconfig.yml --filter '^project-' <the repo you are editing>
ast-grep scan -c queries/sgconfig.yml --filter '^model-'  content/model
ast-grep test -c queries/sgconfig.yml
```

## Three traps measured on this grammar

**A bare `def` pattern matches `async def` too.** `-p 'def $M($$$P): $$$B'` matches both, so a
rule meant to find a synchronous hook would fire on correct asynchronous code. `async` is an
*unnamed* token, so `kind: async` is rejected outright. The working discriminator is a regex on
the node's own text: an async definition begins with it. See `project-sync-hook-override`.

**A decorated function is a `decorated_definition`.** `-p 'def $F(...)'` returns nothing for
`@mcp.tool def f(): ...`, because the outer node is the decorated form. Match that kind, or use
`kind: function_definition` with an `inside:` relation.

**A test naming a rule id that does not exist passes.** `ast-grep test` reports
`0 passed; 0 failed` and exits 0, so a typo in a test's `id` deletes the test rather than
failing it. `verify.py` asserts on the number of cases actually executed, and that the count is
at least the number of rule files.

## Adding a rule

1. Prototype with `ast-grep run -p ... --debug-query=cst` until the parse is what you expect.
2. Write the rule with a `note:` that says what to do, not just what is wrong.
3. Add `rule-tests/<id>-test.yml` with **both** `valid:` and `invalid:` cases. The valid cases
   are the ones that matter: a rule with no valid fixture has not been shown to be precise.
4. `ast-grep test -c sgconfig.yml --update-all` once, then read the snapshot before committing.
   `--update-all` always exits 0, so it can never be part of a gate.
5. `python3 build/verify.py`.

A rule that fires on correct code is worse than no rule. One was dropped for exactly that: a
check for unannotated `@mcp.tool` parameters, which FastMCP's own schema generation tolerates
and which would have flagged working code.
