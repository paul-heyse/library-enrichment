---
name: rust-code-model
description: Pinned, offline index of the seven layers of Rust program knowledge -- rustdoc JSON and rustdoc-types, ra_ap_syntax, ra_ap_hir, ra_ap_load-cargo and ra_ap_project_model, MIR through rustc, rustc_mir_dataflow, and cargo metadata -- with a router that says which layer answers which question and 33 executed probes. Use it when deciding how to extract facts about Rust code: public API shape, name resolution, inferred types, source structure, control flow, value state, or package and feature context. Do not use it as a general Rust language reference, or for crates outside the eleven indexed here.
---

# The seven layers of Rust program knowledge

There are seven genuinely different ways to know something about Rust code, and they overlap
just enough to look interchangeable.

Ask rustdoc JSON what a function does and it will tell you the function exists, describe its
signature precisely, and say nothing about its behaviour -- because the format contains **no
function bodies at all**. That is not a gap to work around; it is what the format is. An agent
that does not know it will read a confident, complete-looking document and conclude the
function is trivial.

**The failure this repository exists to prevent is not "I could not find the answer". It is a
confident answer from the wrong layer.** Rustdoc JSON has no bodies. The syntax tree parses
anything and resolves nothing. HIR knows what every name means and has thrown away the
comments. MIR knows exactly which branch runs and has forgotten the variable was called `s`.
Each is right about its own question and quietly wrong about its neighbour's.

`content/` is prebuilt and pinned. Nothing here queries the network or a service.

## What is pinned

| Subject | Pin |
|---|---|
| `ra_ap_*` (9 crates) | 0.0.352, published 2026-09-14 -- the whole family in lockstep |
| `rustdoc-types` | 0.61.0, which defines `FORMAT_VERSION = 61` |
| `cargo_metadata` | 0.23.1 |
| toolchain | `nightly-2026-09-13`, rustc 1.100.0-nightly `809936eac` |
| rustc source | `rust-lang/rust` at `809936eac` -- the same commit the probes run |
| rust-analyzer source | tag `2026-09-14` (matched to the crate release **by publication date**) |

1,026 canonical items, 4,378 methods, 331 `SyntaxKind` variants, 188 grammar nodes, 105 MIR
vocabulary entries, 28 routed questions, **33 executed probes**. `content/PROVENANCE.json` is
authoritative.

## Escalation ladder

Stop at the first rung that answers the question.

**0. Which layer is this question for?** → `content/topics/00-map.md`, then the topic pages.
This rung is not optional here. Six of the eight known limits below are consequences of asking
the wrong layer.

```
rg -P '\tsyntax\t' content/index/questions.tsv
rg -P '^mir\t' content/index/layers.tsv
```

**1. Does it exist, how is it spelled?** → `rg` over `content/index/*.tsv`

```
rg -P '^ra_ap_hir::semantics::Semantics\t' content/index/methods.tsv
rg -P '\tnode\t' content/index/syntax-kinds.tsv
rg -P '^TerminatorKind\t' content/index/mir-vocabulary.tsv
```

**2. A direct lookup.** → `content/catalogs/` -- compiler views, version oracles, what is not
reachable, and every executed behaviour.

**3. What does this layer refuse to answer?** → `content/layers/<layer>.md`. Read this before
concluding a capability is missing; the `cannot_answer` field is the point of the page.

**4. Can I call it as a library?** → `content/api/<crate>.<module>.md`, indexed by
`content/index/symbols.tsv`. **The path is a rule, not a lookup**: `ra_ap_hir::Semantics` is an
access path, and the canonical item is `ra_ap_hir::semantics::Semantics`. Check
`content/index/aliases.tsv` before concluding a name is absent.

**5. How does it really behave?** → `content/index/behaviors.tsv`, and the pinned upstream
source in `content/corpus/`. For the two compiler layers the corpus is not a convenience:
there is no rustdoc JSON for any compiler-internal crate anywhere, so those files are the
reference.

**6. What is my own code getting wrong?** → the rule corpus, against your repository:

```
ast-grep scan -c queries/sgconfig.yml --filter '^project-' content
```

## Rules that keep answers correct

**Route before you reach.** The question "what does this function do" and the question "what is
this function's signature" go to different layers, and only one of them has an answer. Start at
`questions.tsv`; its `rejected` column names the layer you were about to misuse and why.

**Absence has three causes in rustdoc JSON, and they look identical.** An item can be missing
because it is not public, because a `cfg(feature)` was off, or because it lives in a private
module. The document records none of the three (RD003, RD005). `features.tsv` is built from
`Cargo.toml` because the manifest is the only evidence a gate exists.

**Never cache a rustdoc `Id`.** It is a position within one document, not an identity. Adding
an unrelated struct *before* a type moves that type's `Id`; rebuilding the same source does not
(RD004). An `Id` reused across two versions of a crate silently refers to something else.

**Ask the artifact, not the tool that might have produced it.** `rust-analyzer --version`
reports a rustc release, not an `ra_ap` version, and no published mapping relates them. A
toolchain's rustdoc format version says nothing about what docs.rs will serve -- `rustdoc-types`
0.61.0, the crate that *defines* format 61, is itself served at format 60. Read
`content/index/version-oracles.tsv` before trusting any version string.

**`recorded` is weaker than `confirmed`, and the difference is a control.** A probe whose
control did not come out the other way demonstrates nothing and is never counted as evidence.
26 of 33 probes here are confirmed; the other 7 say so.

**These crates are 0.0.x and ship weekly.** Every `ra_ap` release is semver-breaking by cargo's
rules and there is no changelog. Pin exactly. This index is true of 0.0.352 and says so.

## Reporting

Cite the file you read -- an index row, a probe id, a corpus path. When you rejected a layer,
say which and why; that is usually more useful than the answer. If the index is silent, report
silence rather than absence: `content/index/unreachable.tsv` lists what is genuinely out of
reach and what to use instead, and the known limits in `reference.md` state what this
repository deliberately does not claim.

## Additional references

- `reference.md` -- layout, index schemas, query recipes, and eight known limits
- `queries/README.md` -- the rule corpus and how to extend it
- `build/README.md` -- how the repository is acquired, built and verified
