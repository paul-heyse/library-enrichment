# `datafusion_physical_optimizer::optimizer`

Crate `datafusion-physical-optimizer` · 2 public items · structured records in [`model/datafusion_physical_optimizer.optimizer.json`](../model/datafusion_physical_optimizer.optimizer.json)

## ConfigOnlyContext

`struct` · `datafusion_physical_optimizer::optimizer::ConfigOnlyContext`

Also reachable as `datafusion::physical_optimizer::ConfigOnlyContext`, `datafusion_physical_optimizer::ConfigOnlyContext`

```rust
struct ConfigOnlyContext<'a>
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerContext`

**Methods** (1)

```rust
fn new(config: &'a ConfigOptions) -> Self
```

**via `datafusion_session::physical_optimizer::PhysicalOptimizerContext`**

```rust
fn config_options(&self) -> &ConfigOptions
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.optimizer.ConfigOnlyContext.md).


Simple context wrapping [`ConfigOptions`] for backward compatibility.

This struct provides a minimal implementation of [`PhysicalOptimizerContext`]
that only supplies configuration options. Used when no statistics registry
is available or needed.

---

## PhysicalOptimizer

`struct` · `datafusion_physical_optimizer::optimizer::PhysicalOptimizer`

```rust
struct PhysicalOptimizer
```

**Fields**: `rules`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn new() -> Self
fn with_rules(rules: Vec<Arc<dyn PhysicalOptimizerRule + Send + Sync>>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.optimizer.PhysicalOptimizer.md).


A rule-based physical optimizer.

---
