# Rules

One config, always passed explicitly:

```bash
ast-grep scan -c queries/sgconfig.yml --filter '^model-'   content/model
ast-grep scan -c queries/sgconfig.yml --filter '^corpus-'  content/corpus
ast-grep scan -c queries/sgconfig.yml --filter '^project-' <the repo you are editing>
ast-grep test -c queries/sgconfig.yml
```

Three families, by what a hit means:

| Family | Target | A hit is |
|---|---|---|
| `model-*` | `content/model/*.json` | a fact about the indexed libraries |
| `corpus-*` | `content/corpus/` | a location in vendored upstream material |
| `project-*` | the repository you are editing | a question about your code |

Keeping them separate matters because they mean opposite things. `corpus-assert-type` finds the
576 places the typing conformance suite states a required type — evidence, not a problem.
`project-ruff-check-without-output-format` finds a call whose output you will have to parse as
text — a question, not a defect.

Every rule is `severity: hint` and none carries a `fix`. These are questions, not prohibitions,
and they must never be merged into a repository's own enforcement corpus where a match means a
violation. That is what the separate `sgconfig.yml` is for.

## Traps paid for here

**ast-grep 0.45 has no TOML grammar.** `ast-grep run -l toml` is rejected outright. So
`project-removed-rule-code` matches a removed ruff code in a Python string and cannot see the same
code in a `pyproject.toml`, which is where it is most likely to be. The rule says so in its own
`note`, and `rg -P '\tRemoved\t' content/index/rules.tsv` is the answer for config files. Stating
the gap is the point: a rule that quietly covers half its domain is worse than one that says which
half.

**Match `string_content`, not `string`.** In the Python grammar a `string` node includes its
quotes, so a regex has to carry `["']` on both ends — and a single quote inside a single-quoted
YAML scalar is a parse error, which is how the first version of the generated rule failed to load
at all. `string_content` is the inner text and needs no quote handling.

**A test whose `id` does not exactly match its rule is silently skipped.** `ast-grep test` exits 0
reporting `0 passed; 0 failed`, which reads as success. `verify.check_rule_tests` therefore asserts
that cases ran *and* that the count covers every rule file, not merely that nothing failed.

**Snapshots are an authoring artifact, never a gate.** Record with `--update-all` when a rule's
reported span legitimately changes; the gate is `ast-grep test` with no flags.

**Generated rules are never hand-edited.** `rules/generated/` is written by `build/queries.py` from
the pinned index on every build, together with its own test fixture. Editing one is undone by the
next rebuild; change the generator instead.

## Adding a rule

1. Prototype with `ast-grep run -p` and check the parse with `--debug-query=cst` before adding
   relational logic. A complex YAML rule is not more precise than a clear pattern.
2. Put it in `rules/model/` or `rules/project/` by what a hit means, not by what it matches.
3. Give it a test at `rule-tests/<rule-id>-test.yml` with the `id` copied exactly, and at least one
   `valid:` and one `invalid:` case. `invalid:` means *should match* — the rule is a question and a
   match is a finding.
4. Run `ast-grep test -c queries/sgconfig.yml`, then `python3 build/verify.py`.
