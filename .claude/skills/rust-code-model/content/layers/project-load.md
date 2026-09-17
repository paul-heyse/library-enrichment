# ra_ap_load-cargo + ra_ap_project_model

**Reach for it when** you need hir or syntax to see a real crate rather than a loose file.

## What it is for

how a real Cargo project becomes an analysis database: workspace discovery, the crate graph, roots, dependencies, sysroot, the selected configuration.

## What it cannot answer

any fact about code -- it makes the other layers able to answer, and answers nothing itself.

## Getting it

|  |  |
|---|---|
| Obtained by | `ra_ap_load_cargo::load_workspace_at` |
| Entry point | `ra_ap_load_cargo::load_workspace_at` |
| Needs a build | yes -- build scripts and proc macros are compiled unless disabled |
| Needs a network | yes, unless dependencies are already fetched |
| Stability | 0.0.x weekly, and explicitly NOT an API boundary upstream, so the most volatile of the three |
| Crates pinned here | ra_ap_base_db 0.0.352, ra_ap_load-cargo 0.0.352, ra_ap_paths 0.0.352, ra_ap_project_model 0.0.352, ra_ap_vfs 0.0.352 |
| Indexed symbols | 128 |

## Questions routed here

- How do I load a real Cargo project into an analysis database? — `ra_ap_load_cargo::load_workspace_at`

