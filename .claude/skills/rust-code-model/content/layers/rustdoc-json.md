# Rustdoc JSON + rustdoc-types

**Reach for it when** the question is about the shape of a public contract.

## What it is for

what a crate exposes: modules, signatures, generics, bounds, trait impls, re-exports, attributes, docs.

## What it cannot answer

anything a function does -- there are no bodies in the document at all (RD002); anything about code that is not this crate's public API; anything cfg-gated off at build time (RD005).

## Getting it

|  |  |
|---|---|
| Obtained by | `rustdoc --output-format json, or docs.rs/crate/NAME/VERSION/json` |
| Entry point | `rustdoc_types::Crate` |
| Needs a build | yes -- rustdoc must compile the crate, or docs.rs must have done so |
| Needs a network | only for the hosted form |
| Stability | unstable, nightly-only; format_version has had four breaking bumps in five weeks |
| Crates pinned here | rustdoc-types 0.61.0 |
| Indexed symbols | 58 |

## Questions routed here

- What does this crate expose publicly? — `rustdoc_types::Crate::index`
- What is the exact signature, generics and bounds of an item? — `rustdoc_types::ItemEnum`
- Is this item deprecated, or gated behind a feature? — `rustdoc_types::Item::deprecation`
- What traits does this type implement? — `rustdoc_types::ItemEnum::Impl`
- Which rustdoc format version am I parsing? — `rustdoc_types::FORMAT_VERSION`
- Can I cache a rustdoc Id and use it later? — `rustdoc_types::Id`
- Why is this item missing from the documentation I generated? — `--document-private-items`

## Executed evidence

| Probe | Question | Verdict | Command |
|---|---|---|---|
| RD001 | Does rustdoc JSON contain function bodies? | recorded | `rustdoc +nightly-2026-09-13 -Z unstable-options --output-format json --out-dir rd001 --cra` |
| RD002 | Is a function's body present anywhere in its rustdoc JSON? | confirmed | `sh -c grep -c distinctive_body_token rd001/bodies.json \|\| true` |
| RD003 | Does a type in a private module appear without --document-private-items? | confirmed | `sh -c rustdoc +nightly-2026-09-13 -Z unstable-options --output-format json --document-priv` |
| RD004 | Is an item's Id stable when an unrelated item is added before it? | confirmed | `sh -c rustdoc +nightly-2026-09-13 -Z unstable-options --output-format json --out-dir rd004` |
| RD005 | Does rustdoc JSON record a cfg(feature) gate for an item it excluded? | confirmed | `sh -c rustdoc +nightly-2026-09-13 -Z unstable-options --output-format json --out-dir rd005` |
| RD006 | Does the locally emitted format version match the pinned one? | recorded | `sh -c python3 -c "import json;print(json.load(open('rd001/bodies.json'))['format_version']` |

