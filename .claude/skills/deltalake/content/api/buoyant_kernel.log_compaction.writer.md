# `buoyant_kernel::log_compaction::writer`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.log_compaction.writer.json`](../model/buoyant_kernel.log_compaction.writer.json)

## should_compact

`function` · `buoyant_kernel::log_compaction::writer::should_compact`

Also reachable as `buoyant_kernel::should_compact`, `delta_kernel::log_compaction::writer::should_compact`

```rust
fn should_compact(commit_version: Version, compaction_interval: Version) -> bool
```

Determine if log compaction should be performed based on the commit version and
compaction interval.

Always returns `false` because log compaction is currently disabled.

---

## LogCompactionWriter

`struct` · `buoyant_kernel::log_compaction::writer::LogCompactionWriter`

Also reachable as `buoyant_kernel::LogCompactionWriter`, `delta_kernel::log_compaction::writer::LogCompactionWriter`

```rust
struct LogCompactionWriter
```

**Implements**: `buoyant_kernel::action_reconciliation::RetentionCalculator`

**Derives**: Debug

**Methods** (2)

```rust
fn compaction_data(&mut self, engine: &dyn Engine) -> DeltaResult<ActionReconciliationIterator>
fn compaction_path(&self) -> &url::Url
```

**via `buoyant_kernel::action_reconciliation::RetentionCalculator`**

```rust
fn table_properties(&self) -> &TableProperties
```

Writer for log compaction files

This writer provides an API for creating log compaction files that aggregate actions
from multiple commit files.

---
