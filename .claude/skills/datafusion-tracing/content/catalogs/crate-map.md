# Crate map

Three groups. Only the first is this repository's subject; the other two are indexed because you cannot use the subject without them, and their versions are the ones `datafusion-tracing 55.0.0` is known to compose with rather than the newest published.

| Crate | Group | Version | rustdoc format | Items | Methods |
|---|---|---|---:|---:|---:|
| `datafusion-tracing` | subject | 55.0.0 | 61 + private | 75 | 180 |
| `instrumented-object-store` | subject | 55.0.0 | 61 + private | 5 | 20 |
| `tracing` | wiring | 0.1.44 | 61 | 30 | 57 |
| `tracing-attributes` | wiring | 0.1.31 | 61 | 1 | 0 |
| `tracing-core` | wiring | 0.1.36 | 61 | 44 | 236 |
| `tracing-futures` | wiring | 0.2.5 | 60 | 4 | 62 |
| `tracing-subscriber` | wiring | 0.3.23 | 59 | 104 | 736 |
| `opentelemetry` | otel | 0.31.0 | 56 | 89 | 483 |
| `opentelemetry-otlp` | otel | 0.31.0 | 56 | 43 | 104 |
| `opentelemetry_sdk` | otel | 0.31.0 | 56 | 101 | 541 |
| `tracing-opentelemetry` | otel | 0.32.0 | 56 | 6 | 43 |

docs.rs serves this set at five different rustdoc format versions — it builds on its own schedule and its fleet is not uniform. All four OpenTelemetry crates come back at format 56, older than any other repository in this family parses. The build asserts the field vocabulary it depends on rather than trusting the version number.

## Not indexed

| Crate | Why | Use instead |
|---|---|---|
| `datafusion` | the thing being instrumented, not the instrumentation | the DataFusion reference |
| `arrow`, `parquet` | reached only through DataFusion | the DataFusion reference |
| `object_store` | `instrument_object_store` wraps it; the trait is not ours | the DataFusion reference |
| `tonic` | `opentelemetry-otlp` re-exports four types from it | tonic's own docs |

Those access paths land in `index/unresolved.tsv`. That is a boundary, not a gap.
