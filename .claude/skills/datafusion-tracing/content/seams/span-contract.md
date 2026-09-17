# The emitted span contract

Seven span names were observed across nine upstream scenarios. Four come from the instrumentation itself and the rest from upstream's own test harness, which is a useful reminder that a trace is a join of your spans and this library's.

## Spans it emits

| Span | Target | Level | Fields | Evidence |
|---|---|---|---:|---|
| `InstrumentedExec` | `integration_utils` | INFO | 49 | confirmed |
| `Phase` | `integration_utils` | INFO | 4 | confirmed |
| `Rule` | `integration_utils` | INFO | 1 | confirmed |
| `get_opts` | `instrumented_object_store::instrumented_object_store` | INFO | 5 | recorded |
| `get_ranges` | `instrumented_object_store::instrumented_object_store` | INFO | 4 | recorded |

## What this seam cannot tell you

- Completeness. Every row is `recorded` -- upstream's observation under upstream's harness -- unless a probe with a control promoted it to `confirmed`.
- Tell you the target your spans will carry. It is your crate's module path unless you pass `target:`.
- Spans from scenarios upstream does not test. `07_scrabble_all_options` sets `ignore_full_trace()` because its ordering is not deterministic, so it contributes no rows at all.

## Read next

- [`catalogs/spans.md`](../catalogs/spans.md)

## Before you call it done

- Are you filtering on the target this library will actually emit?
- Is the field you expect conditional on an option you did not set?
