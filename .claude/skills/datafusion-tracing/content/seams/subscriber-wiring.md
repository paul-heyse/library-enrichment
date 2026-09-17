# Wiring a subscriber and an exporter

This library emits spans; it does not collect them. Nothing is recorded until a `Subscriber` is installed, and what reaches a collector is decided by the layer stack: a `tracing-opentelemetry` layer for OTLP, an `fmt` layer for stdout, and a filter on each. The version lock between `tracing-opentelemetry` and the `opentelemetry` family is the single most common way this fails.

## Entry points

| Item | Visibility | Methods | Reached via |
|---|---|---:|---|
| [`Registry`](../api/tracing_subscriber.registry.sharded.md) | supported | 16 | — |
| [`SubscriberInitExt`](../api/tracing_subscriber.util.md) | supported | 3 | — |
| [`OtelData`](../api/tracing_opentelemetry.md) | supported | 3 | — |

## What this seam cannot tell you

- Whether your subscriber filter matches anything. The macros default `target:` to `module_path!()`, which expands at the CALL SITE -- so filtering on `datafusion_tracing` receives nothing.
- Whether an untested opentelemetry pair composes. `catalogs/compatibility.md` records what upstream shipped, and a pair absent from it is untested rather than broken.
- A replacement for the OpenTelemetry SDK's own documentation on sampling, batching or resource attributes.

## Read next

- [`catalogs/compatibility.md`](../catalogs/compatibility.md)
- `content/corpus/examples/otlp.rs` — upstream, verbatim
- `content/corpus/tests/integration_tests.rs` — upstream, verbatim
- `content/corpus/tests/test_utils/in_memory_writer.rs` — upstream, verbatim

## Before you call it done

- Is the level filter above the level of the macro you chose?
- Do the tracing-opentelemetry and opentelemetry minors differ by exactly one?
- Is the tracer provider shut down so the batch exporter flushes?
