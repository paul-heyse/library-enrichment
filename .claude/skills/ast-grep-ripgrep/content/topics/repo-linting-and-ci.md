# Running rules in CI

**These rules should gate a pipeline.**

`scan` runs a project's rules and reports findings with severity. Severity is deployment policy, not a property of the rule: the same rule can be a hint locally and an error in a release pipeline, set with `--error=<id>` and friends, or filtered with `--min-severity`.

CI output has dedicated formats -- `--format github` for inline annotations, `--format sarif` for a code-scanning pipeline, `--json` for anything else.

The exit codes deserve care. `ast-grep test` exits 3 when `--filter` matched no rule, so a typo in a filter looks like a clean run; and `--update-all` always exits 0, which means it can never be part of a gate. Both are in `exit-codes.tsv`.

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| A007 | confirmed | Does outline exit nonzero when it finds nothing? | `ast-grep outline --match ZZZ_no_such_symbol src/lib.rs` |

## Decision rules

- Severity at deployment, not baked into the rule.
- `--format sarif` for code scanning, `--format github` for annotations.
- Assert on the number of test cases executed, not merely on 'nothing failed'.
- Pass `-c` explicitly in CI rather than relying on discovery from the working directory.

## Anti-patterns

- Failing a build on any match regardless of severity.
- `--update-all` in a gate.
- Not noticing exit 3 from a mistyped `--filter`.
- Config discovery that depends on which directory CI happened to start in.

## Checklist

- Is severity set by policy?
- Is the config path explicit?
- Are exit 3 and exit 0-with-zero-tests both caught?
