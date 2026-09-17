# Instrumenting planning phases

A different macro family and a different shape: `instrument_rules_with_*_spans!` takes `state:` and returns a `SessionState`, rather than returning a rule you register. It emits a `Phase` span per pass and a `Rule` span per rule inside it. On a large plan the `Rule` spans dominate trace volume by an order of magnitude, which is what `phase_only()` and the builder's `*_phase_only()` selectors exist for.

## Entry points

| Item | Visibility | Methods | Reached via |
|---|---|---:|---|
| [`RuleInstrumentationOptions`](../api/datafusion_tracing.rule_options.md) | supported | 7 | — |
| [`RuleInstrumentationOptionsBuilder`](../api/datafusion_tracing.rule_options.md) | reachable-undocumented | 11 | RuleInstrumentationOptions::builder() |

## Macros

| Macro | Accepted forms |
|---|---|
| `instrument_rules_with_spans!` | `target: $target:expr, $lvl:expr, options: $options:expr, state: $state:expr, $($fields:tt)*`<br>`target: $target:expr, $lvl:expr, options: $options:expr, state: $state:expr`<br>`$lvl:expr, options: $options:expr, state: $state:expr, $($fields:tt)*`<br>`$lvl:expr, options: $options:expr, state: $state:expr` |
| `instrument_rules_with_info_spans!` | `target: $target:expr, options: $options:expr, state: $state:expr, $($field:tt)*`<br>`options: $options:expr, state: $state:expr, $($field:tt)*`<br>`target: $target:expr, options: $options:expr, state: $state:expr`<br>`options: $options:expr, state: $state:expr` |

Full table: [`catalogs/macros.md`](../catalogs/macros.md)

## Spans it emits

| Span | Target | Level | Fields | Evidence |
|---|---|---|---:|---|
| `Phase` | `integration_utils` | INFO | 4 | confirmed |
| `Rule` | `integration_utils` | INFO | 1 | confirmed |
| `create_physical_plan` | `datafusion_tracing::planner` | INFO | 2 | recorded |

## What this seam cannot tell you

- A documented way to instrument one phase only. `full()` and `phase_only()` are the whole documented surface; the ten per-phase selectors are on the undocumented builder.
- What an individual optimizer rule does. The `Rule` span carries `otel.name` and nothing else.
- Plan diffs, by default. `with_plan_diff()` is opt-in, and upstream's own integration tests disable it because the output is not deterministic.

## Read next

- [`catalogs/options.md`](../catalogs/options.md)
- [`catalogs/macros.md`](../catalogs/macros.md)
- `content/corpus/source/rule_instrumentation.rs` — upstream, verbatim
- `content/corpus/source/rule_options.rs` — upstream, verbatim

## Before you call it done

- Do you need per-rule spans, or will `phase_only()` answer the question?
- Is `with_plan_diff()` worth its non-determinism for your use?
