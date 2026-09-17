# Patterns and repositories you do not control

**The pattern or the corpus comes from someone else.**

Two different risks, often confused. A hostile **pattern** is a resource-consumption problem: PCRE2 backtracks, and recursion, nested quantifiers and backreferences can be made to cost enormously. A hostile **repository** is a configuration problem: ast-grep's `sgconfig.yml` can register custom languages that load native parser libraries.

The default regex engine is the safer request language, because its linear-time guarantee is exactly the property a hostile pattern attacks. Prefer it for anything user-supplied and escalate to `-P` only for trusted patterns.

Ripgrep's `--regex-size-limit` and `--dfa-size-limit` do **not** bound PCRE2 -- they constrain the default engine only. Under `-P` the available limits are in-pattern (`(*LIMIT_MATCH=n)`, `(*LIMIT_DEPTH=n)`, `(*LIMIT_HEAP=n)`) plus whatever the operating system enforces. `unreachable.tsv` records this explicitly, because assuming otherwise is a plausible and wrong inference.

## Constructs

`default` is ripgrep's Rust engine; `reachable` says whether the ripgrep CLI can actually get at it, which is narrower than whether PCRE2 supports it. `observed` is the verdict of an executed probe, not a reading of a version string.

| construct | syntax | default | reachable | since pcre2 | observed |
|---|---|---|---|---|---|
| resource-limits | `(*LIMIT_MATCH=n) (*LIMIT_DEPTH=n) (*LIMIT_HEAP=n)` | no | requires -P | - | unknown |
| no-jit | `(*NO_JIT)` | no | requires -P | - | unknown |

## Decision rules

- Untrusted pattern: default engine, and a timeout owned by the parent process.
- Untrusted repository: do not let its `sgconfig.yml` be discovered. Pass `-c` explicitly.
- `--pre` runs an arbitrary command per file. Treat enabling it as granting execution.
- Relaxing ignore layers can pull in secrets that were ignored deliberately.

## Anti-patterns

- Believing `--regex-size-limit` protects a PCRE2 search.
- Running `scan` on a cloned repository with its own config discovered implicitly.
- Treating in-pattern limits as a sandbox. They are defence in depth, not isolation.

## Checklist

- Who wrote the pattern?
- Who wrote the config that will be discovered?
- Is there a timeout, and does the parent own it?
