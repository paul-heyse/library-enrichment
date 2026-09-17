# Seam map

Eight seams. Each says what it is for, what it hands you, and -- the section to read before concluding anything is absent -- what it refuses to answer.

| Seam | Covers |
|---|---|
| [Instrumenting execution plans](exec-instrumentation.md) | Wrapping every ExecutionPlan node so each one emits a span while it runs |
| [Instrumenting planning phases](rule-instrumentation.md) | Spans around analyzer, logical-optimizer and physical-optimizer passes |
| [Previewing partial results](preview.md) | Putting sample rows of each node's output into its span |
| [DataFusion metrics as span fields](metrics.md) | Recording each node's own MetricsSet onto its span |
| [Instrumenting the object store](object-store.md) | Spans around the GET and RANGE calls underneath a scan |
| [Wiring a subscriber and an exporter](subscriber-wiring.md) | tracing-subscriber layers, level filters, and the OTLP exporter |
| [The emitted span contract](span-contract.md) | Names, targets, levels, fields and nesting of everything emitted |
| [The DataFusion boundary](datafusion-boundary.md) | Where this repository stops and the DataFusion reference begins |

Every page ends with **What this seam cannot tell you**. Read it before reporting that a capability is absent — most of what looks missing in this library is either undocumented-but-present, or genuinely DataFusion's rather than this library's.
