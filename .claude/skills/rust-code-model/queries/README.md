# The rule corpus

Four rules, all `severity: hint`, none carrying a `fix`. They are **questions, not
prohibitions**, and they must never be mixed into a repository's own enforcement corpus — a
hint that leaks into a gate becomes a policy nobody agreed to.

```
ast-grep scan -c queries/sgconfig.yml --filter '^project-' PATH
```

The config is deliberately not auto-discovered. It is always passed with `-c`, so nothing here
can be picked up by a host repository's own `ast-grep` run by accident.

## What each rule asks

| Rule | Language | A hit means |
|---|---|---|
| `project-rustdoc-format-unasserted` | rust | rustdoc JSON deserialized without checking `format_version` first. An unsupported version parses far enough to produce confident nonsense. |
| `project-rustdoc-format-pinned-url` | bash | a docs.rs `/json/<n>` URL. That form is an exact-match lookup against archived builds, not a minimum — it 404s for every other version, and retention is per crate. |
| `project-cargo-metadata-no-format-version` | bash | `cargo metadata` invoked without `--format-version`, which Cargo's own documentation asks for. |
| `project-floating-nightly` | bash | an undated toolchain override. Everything reached through the compiler here is unstable and unversioned, so this pins nothing. |

Every rule has valid and invalid fixtures in `rule-tests/` with recorded snapshots:

```
ast-grep test -c sgconfig.yml
```

**Assert on parsed output, never on the exit status.** `ast-grep test` returns 0 even when it
reports `0 passed; 4 failed`, and `--filter` matching no rule at all exits 3 — so a typo in a
filter reads as a clean run. `build/verify.py` parses the `N passed; N failed` line for exactly
this reason.

## Adding a rule

Prefer, in order: `kind` → `pattern` → an atomic rule → a relational rule → a composite. Stop
at the first rung that works. Patterns are formatting-sensitive; `kind`-anchored rules are not.

Two traps worth knowing before you write one:

- **A backtick cannot start a plain YAML scalar.** A `message:` beginning with one fails the
  whole scan with exit 8, not just that rule. Quote it.
- **`ast-grep` resolves no imports.** A rule matching a bare call name will match any function
  with that name, from any crate. Anchor on something that identifies the origin — a
  fully-qualified path, or a `kind` plus a regex — or expect false positives.

There is no `toml` grammar in this ast-grep build (`--lang toml` exits 2), which is why the
version-pinning rules target shell invocations rather than `Cargo.toml` dependency lines.
