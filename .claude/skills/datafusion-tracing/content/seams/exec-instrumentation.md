# Instrumenting execution plans

You do not wrap nodes yourself. `instrument_with_*_spans!` builds a `PhysicalOptimizerRule`, you register it on a `SessionStateBuilder`, and every node in the final physical plan is wrapped as the rule runs. Register it **last**: the rule captures whatever plan it is handed, so an optimizer rule added after it produces nodes nobody instrumented, and instrumentation stays observational only as long as no other rule has to deal with a wrapped node.

## Entry points

| Item | Visibility | Methods | Reached via |
|---|---|---:|---|
| [`InstrumentationOptions`](../api/datafusion_tracing.options.md) | supported | 4 | — |
| [`InstrumentationOptionsBuilder`](../api/datafusion_tracing.options.md) | reachable-undocumented | 7 | InstrumentationOptions::builder() |

## Macros

| Macro | Accepted forms |
|---|---|
| `instrument_with_spans!` | `target: $target:expr, $lvl:expr, options: $options:expr, $($fields:tt)*`<br>`target: $target:expr, $lvl:expr, options: $options:expr`<br>`$lvl:expr, options: $options:expr, $($fields:tt)*`<br>`$lvl:expr, options: $options:expr` |
| `instrument_with_info_spans!` | `target: $target:expr, options: $options:expr, $($field:tt)*`<br>`options: $options:expr, $($field:tt)*`<br>`target: $target:expr, options: $options:expr`<br>`options: $options:expr` |
| `instrument_with_debug_spans!` | `target: $target:expr, options: $options:expr, $($field:tt)*`<br>`options: $options:expr, $($field:tt)*`<br>`target: $target:expr, options: $options:expr`<br>`options: $options:expr` |

Full table: [`catalogs/macros.md`](../catalogs/macros.md)

## Spans it emits

| Span | Target | Level | Fields | Evidence |
|---|---|---|---:|---|
| `InstrumentedExec` | `integration_utils` | INFO | 49 | confirmed |

## What this seam cannot tell you

- The wrapped node's type, from this index. `InstrumentedExec` is private by design; `datafusion.node` on the span carries the display of the inner plan, and `ExecutionPlan` introspection still works through the wrapper.
- What a macro arm expands to. rustdoc emits no macro bodies; read `corpus/source/exec_instrument_macros.rs`.
- Which DataFusion nodes exist, or what they cost. That is DataFusion's surface, not this one's.

## Read next

- [`catalogs/options.md`](../catalogs/options.md)
- [`catalogs/macros.md`](../catalogs/macros.md)
- [`catalogs/spans.md`](../catalogs/spans.md)
- `content/corpus/source/exec_instrument_rule.rs` — upstream, verbatim
- `content/corpus/source/instrumented_exec.rs` — upstream, verbatim

## Before you call it done

- Is the instrumentation rule the LAST physical optimizer rule registered?
- Are the custom-field keys declared in the macro as well as set on the options?
- Is `preview_limit` non-zero only where you can afford the formatting cost?
