# By what you are trying to do

Eighteen questions. The third column is the one to read: it names the source you were about to trust and says what it would have told you.

## Configuring it

| Question | Go to | Not — |
|---|---|---|
| How do I configure InstrumentationOptions? | `catalogs/options.md` | **docs.rs** — InstrumentationOptionsBuilder answers 404 and is absent from all.html, so the documentation shows a builder() returning a type with no methods; **struct literal** — the four fields are pub, so it compiles -- and there is no field for add_custom_field, whose whole job is to add keys |
| How do I record DataFusion metrics on the spans? | `seams/metrics.md` | **the metrics module** — metrics.rs is private; record_metrics(true) on the options builder is the only switch |
| How do I turn on result previews? | `seams/preview.md` | **preview_limit alone** — it sets a row count; with no preview_fn there is nothing to render and the field never appears |
| Which RuleInstrumentationOptions constructor do I want? | `catalogs/options.md` | **full() or phase_only()** — those are the only two documented, and neither instruments a single phase; the ten per-phase selectors are on the undocumented builder |

## Calling it

| Question | Go to | Not — |
|---|---|---|
| How do I instrument the object store? | `seams/object-store.md` | **InstrumentationOptions** — object-store instrumentation is a separate crate and a separate call; no option enables it |
| What arguments does the macro take? | `index/macros.tsv` | **the rendered signature** — rustdoc renders a macro as its header alone; the arms carry the grammar and the body is elided at every format version |
| Where do I register the instrumentation rule? | `seams/exec-instrumentation.md` | **anywhere in the chain** — the rule captures the plan it is handed, so a rule added after it produces nodes nobody instrumented |
| Which macro do I call to instrument execution? | `catalogs/macros.md` | **new_instrument_rule** — it is #[doc(hidden)] and upstream says it is public only because the macros need it; **InstrumentedExec::new** — the type is private by design and nothing public yields one |
| Which macro instruments the planning phases? | `catalogs/macros.md` | **instrument_session_state** — it is advertised by lib.rs and appears in NEITHER rustdoc document, hosted or private |
| Why does my custom field never appear on a span? | `catalogs/macros.md` | **add_custom_field alone** — it sets a value; a tracing span's field set is fixed at creation, so the key must also be declared in the macro as field::Empty |

## Reading the trace

| Question | Go to | Not — |
|---|---|---|
| What does a preview actually look like? | `index/previews.tsv` | **the signature of pretty_format_compact_batch** — four integers and a Result; nothing in it says what happens to a column that does not fit |
| What fields does a node span carry? | `index/span-fields.tsv` | **a fixed list** — datafusion.metrics.* is built with format!() from whatever the node reports, so the vocabulary is open |
| What span names will I see? | `catalogs/spans.md` | **the source** — span names are macro-generated and node-dependent; the observed set comes from upstream's snapshots |
| Why does my EnvFilter receive nothing? | `catalogs/spans.md` | **datafusion_tracing=info** — the macros default target: to module_path!(), which expands at the CALL SITE, so the spans carry your crate's target and not this library's |

## Wiring it up

| Question | Go to | Not — |
|---|---|---|
| How do I set up a subscriber and an OTLP exporter? | `content/corpus/examples/otlp.rs` | **this library** — it emits spans and collects nothing; until a Subscriber is installed there is no output at all |
| Which DataFusion version does this release need? | `index/compatibility.tsv` | **the version number** — it has matched DataFusion's major at every release, but the column is read from each release's own manifest rather than assumed |
| Which opentelemetry versions go together? | `catalogs/compatibility.md` | **matching minors** — tracing-opentelemetry runs one minor AHEAD of the opentelemetry family, and a mismatched pair fails with a trait error that never mentions versions; **crates.io latest** — the newest releases have already moved past anything datafusion-tracing has shipped against |

## Checking your own code

| Question | Go to | Not — |
|---|---|---|
| What is my own code leaving on the table? | `queries/rules/project/` | **reading the diff** — the mistakes here do not fail loudly: a rule registered too early still compiles and still traces, just not the plan you meant |

## Running a route

`index/questions.tsv` carries an executable recipe per row, relative to the skill directory:

```bash
rg -P '^Which macro' content/index/questions.tsv | cut -f6
```

`verify.py` executes all of them. A recipe that returns nothing fails the build, because a confident pointer to an empty set is the failure this repository exists to prevent.
