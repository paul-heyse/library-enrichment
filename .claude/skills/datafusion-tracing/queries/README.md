# The rule corpus

These rules are **questions**, not prohibitions. They carry `severity: hint` and no `fix`. A
match means "here is something worth knowing", never "here is a violation".

Always pass the config explicitly:

```bash
ast-grep scan -c queries/sgconfig.yml --filter '^project-' <path to the repo you are editing>
```

It is deliberately not auto-discovered. Mixing these into a repository's own ast-grep corpus
would turn hint-level findings into review noise in a project that never asked for them.

Upstream reference: <https://ast-grep.github.io/llms-full.txt>.

## Families

| Prefix | Language | Runs against |
|---|---|---|
| `model-` | json | `content/model` — questions about the indexed API |
| `corpus-` | rust | `content/corpus` — how upstream actually writes it |
| `project-` | rust | the repository you are editing |
| `generated/` | rust | regenerated from the model on every build |

The prefix is functional, not cosmetic: `--filter '^project-'` is how a family is selected.

## Escalation when writing a rule

`kind` → pattern → atomic rule → relational → composite. Stop at the first that works.

## Five traps, each one paid for here

**A duplicate key disables everything.** `project-instrument-rule-not-last` was first written
with two sibling `has:` conditions. YAML keeps the last, ast-grep exits **8**, and *no rule in
the corpus loads* — all ten silently stopped running. Two conditions go under `all:`.
`verify.py` treats exit 8 as a failure because of this.

**ast-grep resolves no bindings, and some questions need them.** Whether the instrumentation
rule is registered last is not decidable from syntax: `.with_physical_optimizer_rule(rule)` looks
identical whichever rule `rule` holds. That rule was narrowed to the two cases syntax *can*
decide — the macro invoked inline, and an argument whose identifier contains `instrument`, which
is the name upstream's own README uses — and the limit is written into its `note`. The
unanchored version matched every multi-rule chain, all false.

**A macro's contents are one unparsed token tree.** `vec![a, b, instrument_rule]` does not expose
its elements as sibling nodes, so `follows` never fires inside it and the vector form of the same
check silently matched nothing. Matching the `token_tree` node's text is the decidable form.

**ast-grep cannot parse TOML.** Measured: `ast-grep run --lang toml` is rejected by 0.45.3 with
*`toml is not supported!`*. The opentelemetry version-skew check is therefore a ripgrep recipe in
`catalogs/compatibility.md`, not a rule. A rule would have been the wrong tier.

**The model rules must not anchor on `json-record`.** That util requires a `"path"` member, which
item records have and method records do not. Anchored on it,
`model-undocumented-but-reachable` reported the two builder structs and hid all sixteen methods
inside them — which are the findings.

## Testing

Every rule needs both `valid` and `invalid` fixtures in `rule-tests/<id>-test.yml`, with the `id`
matching exactly. `ast-grep test --update-all` is an authoring step and can never be a gate.

The exit codes cannot be trusted on their own: `ast-grep test` returns **0** while reporting
`0 passed; 4 failed`, and `--filter` matching nothing also exits 0 — so a typo in a rule id
deletes the test rather than failing it. `verify.py` parses the counts and asserts
`passed >= number of rule files` for that reason.

## Adding a rule

1. Write it under the right family, with `metadata.record` naming its class.
2. Write `rule-tests/<id>-test.yml`; the `id` must match the rule's exactly.
3. `ast-grep test -c queries/sgconfig.yml --update-all` once, to record snapshots.
4. Run it against real code and count the matches. A rule nobody ran is a liability.
5. `python3 build/verify.py`.
