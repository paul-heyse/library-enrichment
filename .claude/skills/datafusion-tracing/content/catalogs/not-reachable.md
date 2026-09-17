# What exists and cannot be used

Three kinds of thing, and they need different answers. An invisible limitation reads exactly like an absent capability, so each row says what to do instead.

## Real, public to rustdoc, and unreachable

`rustdoc` records these as `public` — they are `pub` items inside private modules — but nothing you can name ever yields one, so there is no spelling that reaches them. `InstrumentedExec` is the one that matters: upstream's README says it is *intentionally private so downstream code cannot depend on its internals*, and the supported surface is the optimizer rule plus the ordinary `ExecutionPlan` trait.

| Item | Kind | Instead |
|---|---|---|
| `AnalyzerPhaseSentinel` | struct | no public route; this is an implementation detail |
| `BoxedAny` | type_alias | no public route; this is an implementation detail |
| `BoxedClosure` | type_alias | no public route; this is an implementation detail |
| `BoxedFuture` | type_alias | no public route; this is an implementation detail |
| `DefaultDisplay` | struct | no public route; this is an implementation detail |
| `ErrorCleanupAnalyzerRule` | struct | no public route; this is an implementation detail |
| `ErrorCleanupOptimizerRule` | struct | no public route; this is an implementation detail |
| `ErrorCleanupPhysicalOptimizerRule` | struct | no public route; this is an implementation detail |
| `ExecutionRecorders` | struct | no public route; this is an implementation detail |
| `ExecutionRecordingStream` | struct | no public route; this is an implementation detail |
| `FormatPlan` | trait | no public route; this is an implementation detail |
| `InstrumentRule` | struct | no public route; this is an implementation detail |
| `Instrumentable` | trait | no public route; this is an implementation detail |
| `InstrumentationLevel` | enum | choose the macro whose name carries the level |
| `InstrumentedAnalyzerRule` | struct | no public route; this is an implementation detail |
| `InstrumentedExec` | struct | register the rule; introspect through `ExecutionPlan` as usual |
| `InstrumentedMultiPartUpload` | struct | no public route; this is an implementation detail |
| `InstrumentedObjectStore` | struct | no public route; this is an implementation detail |
| `InstrumentedOptimizerRule` | struct | no public route; this is an implementation detail |
| `InstrumentedPhysicalOptimizerRule` | struct | no public route; this is an implementation detail |
| `MetricsRecorder` | struct | no public route; this is an implementation detail |
| `MetricsRecordingStream` | struct | no public route; this is an implementation detail |
| `NodeRecorder` | struct | no public route; this is an implementation detail |
| `NodeRecordingStream` | struct | no public route; this is an implementation detail |
| `OptimizerPassTracker` | struct | no public route; this is an implementation detail |
| `OptimizerPhaseSentinel` | struct | no public route; this is an implementation detail |
| `PhaseSpanCreateFn` | type_alias | no public route; this is an implementation detail |
| `PhysicalOptimizerPhaseSentinel` | struct | no public route; this is an implementation detail |
| `PlanningContext` | struct | no public route; this is an implementation detail |
| `PlanningPhase` | enum | no public route; this is an implementation detail |
| `PreviewFn` | type_alias | pass a closure to `preview_fn`; inference supplies the type |
| `PreviewRecorder` | struct | no public route; this is an implementation detail |
| `PreviewRecorderBuilder` | struct | set `preview_limit` and `preview_fn` on the options builder |
| `PreviewRecordingStream` | struct | no public route; this is an implementation detail |
| `RuleSpanCreateFn` | type_alias | no public route; this is an implementation detail |
| `SingleSpanTreeTraverser` | struct | no public route; this is an implementation detail |
| `SpanCreateFn` | type_alias | no public route; this is an implementation detail |
| `SpanTracer` | struct | no public route; this is an implementation detail |
| `TracingQueryPlanner` | struct | no public route; this is an implementation detail |

Note that `PreviewFn` is not merely unreachable — rustdoc expands the alias away entirely, so the public signature of `InstrumentationOptions.preview_fn` reads `Option<Arc<dyn Fn(&RecordBatch) -> Result<String> + Send + Sync>>`. Neither the name in the source nor the name in the documentation is one you can write.

## Advertised by `lib.rs`, emitted by neither rustdoc capture

| Name | Crate | What it is | Instead |
|---|---|---|---|
| `instrument_session_state` | `datafusion-tracing` | a `#[doc(hidden)]` `pub use` that appears in neither the hosted document nor the `--document-private-items` capture | call the `instrument_rules_with_*_spans!` macro, which expands to it |

This one is worth dwelling on: the crate root re-exports the name, so it is genuinely public and genuinely callable, and **no rustdoc document in existence mentions it**. The index would have contradicted the crate's own `lib.rs` by silence. It is recorded in `index/unreachable.tsv` instead.

## Public, documented, and not for you

| Item | Why it is public | Instead |
|---|---|---|
| `new_instrument_rule` | `#[doc(hidden)]`; upstream's comment says *only public because they need to be accessed by the macros* | the macro that expands to it |

`#[doc(hidden)]` is **not recorded in rustdoc JSON** at format 61 — measured: `new_instrument_rule` comes back with `attrs: []` while its source declaration carries the attribute. The classification is read from `content/corpus/source/lib.rs` with an ast-grep rule, because the attribute is a sibling node of the item it decorates and a line-oriented pattern gets that wrong in both directions.
