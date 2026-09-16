# The generator

Two entry points, and the separation between them is load-bearing.

```bash
python3 acquire.py                     # network + git + cargo + a dated nightly + uv
python3 acquire.py --stage oracles     # just the tool catalogs; seconds, needs only uv
python3 acquire.py --check             # has the pinned tag moved? reports, never re-pins
python3 build.py                       # offline: stdlib + the ast-grep binary
python3 verify.py                      # all five checks
```

`build.py` never imports `acquire.py`. `verify.py` re-runs `build.py` to prove determinism, so if
acquisition were reachable the determinism check would start compiling Rust and installing wheels.

## Stages

| Stage | Owner | Needs |
|---|---|---|
| Hosted rustdoc JSON for the 31 ruff crates | `fetch.py` | network |
| Local rustdoc JSON for the 11 pyrefly crates | `acquire.py` | git, cargo, `nightly-2026-08-18` |
| Tool catalogs from both binaries | `oracles.py` | uv |
| Corpus tarballs | `fetch.py` | network |
| Model, index, pages, catalogs, topics, rules | `build.py` | stdlib + `ast-grep` |
| Checks | `verify.py` | `ast-grep` |

`acquired/` is git-tracked; `.cache/` is not. The distinction is whether anyone else could
re-serve the bytes. docs.rs and GitHub can, so their downloads are a cache. A rustdoc JSON that
only exists because someone ran a specific nightly against a specific capsule, and a catalog that
only exists because someone ran a specific binary, cannot be re-served — so they are committed.

## Four things that were easy to get wrong

**The newest nightly does not work.** `config/toolchains.toml` names `nightly-2026-09-13` as the
current producer, and pyrefly cannot be documented with it: `allocative 0.3.6` fails with `E0119`
because its `build.rs` detects nightly by string-matching `rustc --version`, turns on
`feature(never_type)`, and adds `impl Allocative for !` which now conflicts with its own
`impl for Infallible`. The break landed between 2026-08-18 and 2026-09-13. Use the older pin; it
still emits `format_version` 61, so nothing downstream changes.

**A proc-macro crate needs `"kind": "proc-macro"` in the manifest.** rustdoc writes it to
`target/doc` rather than `target/<triple>/doc`, and without the declaration the run fails at
collection with "no rustdoc JSON produced" after the whole workspace has compiled.

**The host's tools are the wrong versions.** PATH here carries ruff 0.14.4 and pyrefly 0.51.1
against pins of 0.16.7 and 1.3.1. The 0.14.4 binary reports 934 rules where 0.16.7 reports 970, so
a catalog captured from PATH loses 36 rules and looks entirely plausible. `oracles.py` installs
into the capsule and asserts the reported version before recording anything.

**The rule-to-variant join is case-folded, not PascalCased.** Naive kebab-to-Pascal joins 937 of
970 and silently drops 33, all acronyms: `blanket-noqa` is `BlanketNOQA`, `io-error` is `IOError`.
`catalogs.fold` normalises both sides and the build raises if two variants ever collide under it.

## Re-pinning

`acquire.py --check` reports whether `refs/tags/1.3.1` still resolves to the recorded commit. It
never re-pins — a moved pin is a decision, not a build step.

To move to a new release: update `version` in each `crate_sets` entry and the matching `oracles`
pins, confirm the two ruff lines still correspond (`ruff_linter`'s dependency requirement on the
library crates is the oracle), re-run `acquire.py`, then `build.py` and `verify.py`. If the rule
count and the `Rule` variant count diverge, the pins are inconsistent — that is the check doing
its job, not a bug.

A different library needs a different manifest, not a different builder.
