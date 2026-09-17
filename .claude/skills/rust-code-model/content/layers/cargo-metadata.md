# cargo metadata / cargo_metadata

**Reach for it when** the question is about packages, targets, features or dependency identity.

## What it is for

package identity, targets, workspace structure, dependency identities and renames, the resolved graph, manifest metadata, declared features.

## What it cannot answer

anything whatsoever about code; which features a particular build resolved, as opposed to which are declared (PL003).

## Getting it

|  |  |
|---|---|
| Obtained by | `cargo metadata --format-version 1` |
| Entry point | `cargo_metadata::MetadataCommand` |
| Needs a build | no -- resolution only, and --no-deps skips even that |
| Needs a network | yes unless the lockfile is complete and the registry cached |
| Stability | the wire format has been version 1 since ~2017; the crate that parses it is pre-1.0 |
| Crates pinned here | cargo_metadata 0.23.1 |
| Indexed symbols | 76 |

## Questions routed here

- What is the crate graph and which features are on? — `cargo_metadata::Metadata::resolve`

## Executed evidence

| Probe | Question | Verdict | Command |
|---|---|---|---|
| PL001 | Does --no-deps omit the resolved dependency graph? | confirmed | `sh -c cargo metadata --format-version 1 --no-deps \| python3 -c "import json,sys;print('ABS` |
| PL002 | What schema version does cargo metadata report? | recorded | `sh -c cargo metadata --format-version 1 --no-deps \| python3 -c "import json,sys;print(json` |
| PL003 | Does the metadata report a feature that is declared but not enabled? | confirmed | `sh -c cargo metadata --format-version 1 --no-deps \| python3 -c "import json,sys;print('DEC` |

