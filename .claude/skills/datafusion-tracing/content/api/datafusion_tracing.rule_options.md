# `datafusion_tracing::rule_options`

Crate `datafusion-tracing` · 3 public items · structured records in [`model/datafusion_tracing.rule_options.json`](../model/datafusion_tracing.rule_options.json)

## InstrumentationLevel

`enum` · `datafusion_tracing::rule_options::InstrumentationLevel`

```rust
enum InstrumentationLevel
```

**Variants**: `Disabled`, `PhaseOnly`, `Full`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn phase_span_enabled(&self) -> bool
fn rule_spans_enabled(&self) -> bool
```

Instrumentation level for a phase (analyzer, optimizer, or physical optimizer).

Rule spans always require a parent phase span, so the levels are hierarchical:
- `Disabled`: no spans (default)
- `PhaseOnly`: only the phase span
- `Full`: phase span + individual rule spans

---

## RuleInstrumentationOptions

`struct` · `datafusion_tracing::rule_options::RuleInstrumentationOptions`

Also reachable as `datafusion_tracing::RuleInstrumentationOptions`

```rust
struct RuleInstrumentationOptions
```

**Derives**: Clone, Debug, Default

**Methods** (4)

```rust
fn builder() -> RuleInstrumentationOptionsBuilder
fn full() -> Self
fn phase_only() -> Self
fn with_plan_diff(self) -> Self
```

Configuration options for instrumented DataFusion rules (Analyzer, Optimizer, Physical Optimizer).

---

## RuleInstrumentationOptionsBuilder

`struct` · `datafusion_tracing::rule_options::RuleInstrumentationOptionsBuilder`

```rust
struct RuleInstrumentationOptionsBuilder
```

**Derives**: Default

**Methods** (10)

```rust
fn all(self) -> Self
fn all_phase_only(self) -> Self
fn analyzer(self) -> Self
fn analyzer_phase_only(self) -> Self
fn build(self) -> RuleInstrumentationOptions
fn optimizer(self) -> Self
fn optimizer_phase_only(self) -> Self
fn physical_optimizer(self) -> Self
fn physical_optimizer_phase_only(self) -> Self
fn plan_diff(self) -> Self
```

The builder for `RuleInstrumentationOptions`.

---
