# `ra_ap_project_model::build_dependencies`

Crate `ra_ap_project_model` · 2 public items · structured records in [`model/ra_ap_project_model.build_dependencies.json`](../model/ra_ap_project_model.build_dependencies.json)

## ProcMacroDylibPath

`enum` · `ra_ap_project_model::build_dependencies::ProcMacroDylibPath`

Also reachable as `ra_ap_project_model::ProcMacroDylibPath`

```rust
enum ProcMacroDylibPath
```

**Variants**: `Path`, `DylibNotFound`, `NotProcMacro`, `NotBuilt`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

---

## WorkspaceBuildScripts

`struct` · `ra_ap_project_model::build_dependencies::WorkspaceBuildScripts`

Also reachable as `ra_ap_project_model::WorkspaceBuildScripts`

```rust
struct WorkspaceBuildScripts
```

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn error(&self) -> Option<&str>
```

Output of the build script and proc-macro building steps for a workspace.

---
