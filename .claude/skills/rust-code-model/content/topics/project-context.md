# Packages, features and the crate graph

Package identity, targets, workspace structure, dependency identities and renames, resolved graphs, declared features -- and how a real project becomes an analysis database.

## Mental model

Two layers that sound alike and are not. `cargo metadata` is a stable JSON description of the project and contains no code facts at all. `ra_ap_load-cargo` turns that project into something the semantic layers can query, and contributes no new category of fact -- its whole value is making the others operate on a real resolved crate instead of a loose file.

## Questions this covers

| Question | Start at | Instead of |
|---|---|---|
| What is the crate graph and which features are on? | `cargo_metadata::Metadata::resolve` | cargo-metadata, project-load |
| How do I load a real Cargo project into an analysis database? | `ra_ap_load_cargo::load_workspace_at` | hir, cargo-metadata |

## Why not the neighbouring layer

- **What is the crate graph and which features are on?** -- not `cargo-metadata`: with --no-deps the resolve field is null, which is not the same as no dependencies
- **What is the crate graph and which features are on?** -- not `project-load`: builds its own graph for analysis, which is not the Cargo view
- **How do I load a real Cargo project into an analysis database?** -- not `hir`: operates on a database; it does not create one
- **How do I load a real Cargo project into an analysis database?** -- not `cargo-metadata`: describes the project without loading anything

## What was executed

Each row ran against the pinned toolchain. A `confirmed` verdict means the probe
held *and* its control came out the other way; `recorded` means there was nothing
for a control to falsify.

| Probe | Question | Verdict |
|---|---|---|
| PL001 | Does --no-deps omit the resolved dependency graph? | confirmed |
| PL002 | What schema version does cargo metadata report? | recorded |
| PL003 | Does the metadata report a feature that is declared but not enabled? | confirmed |

**PL001** — `resolve` is the whole dependency graph and it is null under --no-deps. Reading a field that is null for a reason as 'this project has no dependencies' is the characteristic error at this layer.

**PL002** — It has read 1 for years while the envelope beneath it gained keys -- `build_directory` is recent. The version is a weak guarantee: new fields and new enum values arrive without a bump, so a deserializer must tolerate unknown keys.

**PL003** — This layer reports the feature table as DECLARED, which is not the same as what a build resolved. It is the complement of RD005: rustdoc drops what was not enabled, cargo lists what could be.

