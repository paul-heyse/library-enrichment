# The generator

Two entry points, and the separation between them is load-bearing.

```
python3 build/acquire.py     # the ONLY networked stage
python3 build/build.py       # offline; regenerates content/ from acquired/
python3 build/verify.py      # eight checks, including a byte-for-byte rebuild
```

`build.py` never imports `acquire.py`. If acquisition could run inside the build, a rebuild
could pick up different bytes from the network and still call itself reproducible.

## Where bytes live

| Directory | Committed | Why |
|---|---|---|
| `.cache/` | no | raw downloads, re-fetchable from the pin at any time |
| `acquired/` | yes | what `build.py` reads. This is what makes the determinism check mean something: a rebuild reads bytes that are in the tree, not bytes a server chose to send today |

## Stages

| Stage | Module | Output |
|---|---|---|
| assert pins | `oracles.py` | refuses to build unless the toolchain, the format version and ast-grep are the pinned ones |
| models | `model.py`, `render.py`, `emit.py` | rustdoc JSON → canonical items → `model/`, `api/` |
| indexes | `build.py` | `symbols`, `methods`, `impls`, `aliases`, `features`, `unnameable`, `foreign-impls`, `unresolved` |
| vocabulary | `vocab.py` | MIR, `SyntaxKind`, grammar nodes and the dataflow framework, parsed from pinned source |
| probes | `probes.py` | executes rustc, cargo and rust-analyzer; writes `behaviors.tsv` |
| router | `router.py` | `questions`, `layers`, `compiler-views`, `version-oracles`, `unreachable` |
| pages | `pages.py` | `topics/`, `layers/`, `catalogs/` |
| provenance | `build.py` | pins, counts, version readings, per-file sha256 |

`model.py`, `render.py`, `emit.py`, `link.py` and `fetch.py` are shared with the other Rust
capability repositories in this family and are near-verbatim copies.

## Five traps this build already fell into

**`-Zunpretty` values cannot be comma-split.** Several contain a comma — `hir,typed`,
`expanded,identified`. The backticks in rustc's error message are the delimiters.

**A trait body must be brace-matched before its members are read.** Splitting on the trait
header and taking the rest of the file folds every later trait's methods into it.

**A trait method's required/provided status must be decided from its own signature**, not from
the slice up to the next member. rustc's doc comments contain braces — the comment above
`apply_effect` mentions `{early,primary} x {statement,terminator}` — which is enough to make a
required method look like it has a default body.

**Probe output cannot be stored raw.** It carries temporary paths, timings and MIR pass
numbering, none of which is stable across runs. `probes.py` stores the *matched evidence* with
the fixture root rewritten to a placeholder, which is deterministic while the claim holds and
changes when it stops holding.

**Fixtures are a manifest, not a directory.** `fixtures/tree.json` is materialised to a
temporary directory at probe time. A checked-in directory of deliberately broken Rust and a
stray `Cargo.toml` would be found by the editors, formatters and lint corpora of whatever
repository this skill is copied into.

## Adding a probe

Add it to `probes.json` with a **control that must come out the other way**. A probe without
one cannot distinguish "the feature works" from "the pattern matched for an unrelated reason",
and is recorded as `recorded` rather than `confirmed` — a weaker verdict the index keeps
distinct on purpose. `verify.py` fails on any `divergent` probe, which is what a control that
did not discriminate produces.
