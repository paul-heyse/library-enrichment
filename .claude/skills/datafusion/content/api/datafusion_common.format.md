# `datafusion_common::format`

Crate `datafusion-common` · 7 public items · structured records in [`model/datafusion_common.format.json`](../model/datafusion_common.format.json)

## DEFAULT_CAST_OPTIONS

`constant` · `datafusion_common::format::DEFAULT_CAST_OPTIONS`

```rust
const DEFAULT_CAST_OPTIONS: arrow::compute::CastOptions<'static> = _
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.format.DEFAULT_CAST_OPTIONS.md).


The default [`CastOptions`] to use within DataFusion

---

## DEFAULT_FORMAT_OPTIONS

`constant` · `datafusion_common::format::DEFAULT_FORMAT_OPTIONS`

```rust
const DEFAULT_FORMAT_OPTIONS: arrow::util::display::FormatOptions<'static> = _
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.format.DEFAULT_FORMAT_OPTIONS.md).


The default [`FormatOptions`] to use within DataFusion
Also see [`crate::config::FormatOptions`]

---

## ExplainAnalyzeCategories

`enum` · `datafusion_common::format::ExplainAnalyzeCategories`

```rust
enum ExplainAnalyzeCategories
```

**Variants**: `All`, `Only`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.format.ExplainAnalyzeCategories.md).


Controls which [`MetricCategory`] values are shown in `EXPLAIN ANALYZE`.

Set via `SET datafusion.explain.analyze_categories = '...'`.

See [`MetricCategory`] for the determinism properties that motivate
this filter.

---

## ExplainFormat

`enum` · `datafusion_common::format::ExplainFormat`

Also reachable as `datafusion::logical_expr::ExplainFormat`, `datafusion_expr::ExplainFormat`, `datafusion_expr::logical_plan::ExplainFormat`

```rust
enum ExplainFormat
```

**Variants**: `Indent`, `Tree`, `PostgresJSON`, `Graphviz`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(format: &str) -> Result<Self, Self::Err>
```

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.format.ExplainFormat.md).


Output formats for controlling for Explain plans

---

## MetricCategory

`enum` · `datafusion_common::format::MetricCategory`

Also reachable as `datafusion_physical_expr_common::metrics::MetricCategory`, `datafusion_physical_plan::metrics::MetricCategory`

```rust
enum MetricCategory
```

**Variants**: `Rows`, `Bytes`, `Timing`, `Uncategorized`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.format.MetricCategory.md).


Classifies a metric by what it measures.

This is orthogonal to [`MetricType`] (Summary / Dev), which controls
*verbosity*. `MetricCategory` controls *what kind of value* is shown,
so that `EXPLAIN ANALYZE` output can be narrowed to only the categories
that are useful in a given context.

In particular this is useful for testing since metrics differ in their stability across runs:
- [`Rows`](Self::Rows) and [`Bytes`](Self::Bytes) depend only on the plan
  and the data, so they are mostly deterministic across runs (given the same
  input). Variations can existing e.g. because of non-deterministic ordering
  of evaluation between threads.
  Running with a single target partition often makes these metrics stable enough to assert on in tests.
- [`Timing`](Self::Timing) depends on hardware, system load, scheduling,
  etc., so it varies from run to run even on the same machine.

[`MetricCategory`] is especially useful in sqllogictest (`.slt`) files:
setting `datafusion.explain.analyze_categories = 'rows'` lets a test
assert on row-count metrics without sprinkling `<slt:ignore>` over every
timing value.

Metrics that do not declare a category (the default for custom
`Count` / `Gauge` metrics) are treated as
[`Uncategorized`](Self::Uncategorized) for filtering purposes.

---

## MetricType

`enum` · `datafusion_common::format::MetricType`

Also reachable as `datafusion_physical_expr_common::metrics::MetricType`, `datafusion_physical_plan::metrics::MetricType`

```rust
enum MetricType
```

**Variants**: `Summary`, `Dev`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`, `datafusion_common::config::ConfigField`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn included_types(self) -> Vec<MetricType>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.format.MetricType.md).


Categorizes metrics so the display layer can choose the desired verbosity.

The `datafusion.explain.analyze_level` configuration controls which
type is shown:
- `"dev"` (the default): all metrics are shown.
- `"summary"`: only metrics tagged as `Summary` are shown.

This is orthogonal to [`MetricCategory`], which filters by *what kind*
of value a metric represents (rows / bytes / timing).

# Difference from `EXPLAIN ANALYZE VERBOSE`

The `VERBOSE` keyword controls whether per-partition metrics are shown
(when specified) or aggregated metrics are displayed (when omitted).
In contrast, `MetricType` determines which *levels* of metrics are
displayed.

---

## ExplainStatementOptions

`struct` · `datafusion_common::format::ExplainStatementOptions`

```rust
struct ExplainStatementOptions
```

**Fields**: `analyze`, `verbose`, `format`, `analyze_level`, `analyze_categories`, `show_statistics`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn from_utility_options(opts: &[UtilityOption]) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.format.ExplainStatementOptions.md).


Normalized options for a single `EXPLAIN` statement.

This collects the knobs that can be set per-statement from either the
legacy keyword form (`EXPLAIN ANALYZE VERBOSE FORMAT tree ...`) or the
Postgres-style `EXPLAIN (option [arg], ...) ...` form supported on
dialects whose
[`Dialect::supports_explain_with_utility_options`](https://docs.rs/sqlparser/latest/sqlparser/dialect/trait.Dialect.html#method.supports_explain_with_utility_options)
returns `true`.

Fields that are `None` / `false` mean "not set at the statement level" —
the physical planner falls back to the corresponding session config
value.

---
