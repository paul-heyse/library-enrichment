# Knowing what you are actually running

Which version each subject reports, what that reading covers, and where it misleads.

## Mental model

This is the failure mode these subjects share. Three of them answer a question adjacent to the one you asked: `rust-analyzer --version` reports a rustc release, not an ra_ap version; a toolchain's rustdoc format version says nothing about what docs.rs will serve; `cargo metadata` reports schema 1 while the envelope grows new keys. Ask the artifact in front of you, not the tool that might have produced it.

## Questions this covers

| Question | Start at | Instead of |
|---|---|---|
| Can I script these tools and trust the exit code? | `-` | hir |
| Which layer should I use at all? | `content/index/layers.tsv` | - |

## Why not the neighbouring layer

- **Can I script these tools and trust the exit code?** -- not `hir`: sometimes: ssr exits 1 on a refused rule, but parse exits 0 on source it could not parse

## What was executed

Each row ran against the pinned toolchain. A `confirmed` verdict means the probe
held *and* its control came out the other way; `recorded` means there was nothing
for a control to falsify.

| Probe | Question | Verdict |
|---|---|---|
| XL001 | Does the syntax layer know the type of an expression? | confirmed |
| XL002 | Does MIR retain the name of a local variable? | confirmed |

**XL001** — The control finds the return type as written -- RET_TYPE is a syntax node. What is absent is any inferred type. The tree records what the source says, never what it means, which is the single most common layer confusion.

**XL002** — Partly. Locals are numbered, and the original names survive only as `debug` annotations for the debugger. Code that matches on variable names will not find them where it expects.

