# `datafusion_common::alias`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.alias.json`](../model/datafusion_common.alias.json)

## AliasGenerator

`struct` · `datafusion_common::alias::AliasGenerator`

```rust
struct AliasGenerator
```

**Derives**: Debug, Default

**Methods** (3)

```rust
fn new() -> Self
fn next(&self, prefix: &str) -> String
fn update_min_id(&self, min_id: usize)
```

A utility struct that can be used to generate unique aliases when optimizing queries

---
