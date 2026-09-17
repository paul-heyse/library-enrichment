# What a crate exposes

The shape of a public contract: what exists, how it is spelled, what it takes and returns, what it implements, and what is deprecated.

## Mental model

Rustdoc JSON is a model of *documentation*, not of code. It is the most normalized view of a crate's exposed API and it contains no function bodies whatsoever. Every question of the form 'what does this do' has to leave this layer immediately. Three different causes make an item absent -- not public, gated off by a feature, or in a private module -- and the document distinguishes none of them.

## Questions this covers

| Question | Start at | Instead of |
|---|---|---|
| What does this crate expose publicly? | `rustdoc_types::Crate::index` | hir, cargo-metadata |
| What is the exact signature, generics and bounds of an item? | `rustdoc_types::ItemEnum` | syntax |
| Is this item deprecated, or gated behind a feature? | `rustdoc_types::Item::deprecation` | rustdoc-json |
| What traits does this type implement? | `rustdoc_types::ItemEnum::Impl` | hir |
| Which rustdoc format version am I parsing? | `rustdoc_types::FORMAT_VERSION` | rustdoc-json |
| Can I cache a rustdoc Id and use it later? | `rustdoc_types::Id` | rustdoc-json |
| Why is this item missing from the documentation I generated? | `--document-private-items` | rustdoc-json |

## Why not the neighbouring layer

- **What does this crate expose publicly?** -- not `hir`: sees every item whether public or not, and has no notion of the documented surface
- **What does this crate expose publicly?** -- not `cargo-metadata`: knows the package, not a single item inside it
- **What is the exact signature, generics and bounds of an item?** -- not `syntax`: has the signature as written, but not the resolved paths in it
- **Is this item deprecated, or gated behind a feature?** -- not `rustdoc-json`: for the feature half: a gated-off item is absent entirely, with no record that it exists
- **What traits does this type implement?** -- not `hir`: also answers, and is the right choice when the type is local or private rather than a published API
- **Which rustdoc format version am I parsing?** -- not `rustdoc-json`: the toolchain's own version does not tell you: docs.rs serves whatever its fleet built, and rustdoc-types 0.61.0 is itself served at format 60
- **Can I cache a rustdoc Id and use it later?** -- not `rustdoc-json`: no: an Id is a position in one document. Adding an unrelated item shifts it
- **Why is this item missing from the documentation I generated?** -- not `rustdoc-json`: three different causes look identical: not public, cfg-gated off, or in a private module

## What was executed

Each row ran against the pinned toolchain. A `confirmed` verdict means the probe
held *and* its control came out the other way; `recorded` means there was nothing
for a control to falsify.

| Probe | Question | Verdict |
|---|---|---|
| RD001 | Does rustdoc JSON contain function bodies? | recorded |
| RD002 | Is a function's body present anywhere in its rustdoc JSON? | confirmed |
| RD003 | Does a type in a private module appear without --document-private-items? | confirmed |
| RD004 | Is an item's Id stable when an unrelated item is added before it? | confirmed |
| RD005 | Does rustdoc JSON record a cfg(feature) gate for an item it excluded? | confirmed |
| RD006 | Does the locally emitted format version match the pinned one? | recorded |

**RD001** — Emission step for RD002 and RD003. Kept separate so a failure to emit is not misread as a fact about content.

**RD002** — The single most important negative fact about this layer. The control is what makes it mean anything: the same document DOES contain the function's name, so the absent body is absent by design rather than because the probe read the wrong file. Any question about what code does must go to another layer.

**RD003** — Absence from a default rustdoc document means 'not public', not 'does not exist'. The two documents differ on exactly this type.

**RD004** — An Id is a position within one document, not an identity for an item. Adding a struct BEFORE Target moves Target's Id; rebuilding the same source does not. The control rules out nondeterminism, which isolates the real hazard: an Id cached across two versions of a crate silently refers to something else.

**RD005** — A feature-gated item that was not enabled is simply not in the document -- there is no record that it exists or what would enable it. This is why features.tsv is built from Cargo.toml instead: the manifest is the only evidence of the gate.

**RD006** — What this toolchain emits. Deliberately not compared with what docs.rs served -- see PROVENANCE `versions`, where rustdoc-types 0.61.0 is itself served at format 60.

