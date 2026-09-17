# Observed behaviour

12 probes executed against the pinned capsule: 11 confirmed, 1 recorded.

`confirmed` means the probe and its control both came out as expected and the control came out the other way. `recorded` is a shape probe with nothing to falsify -- weaker, and labelled as such. `refuted` means the control discriminated and the claim did not hold, which is a result rather than a failure. A `divergent` probe demonstrates nothing and fails `verify.py`; a `blocked` one means a prerequisite was missing and never that the behaviour is absent.

Every capture is a fact about a **construction**, not about this library in general. The query is the same three-row `VALUES` scan throughout, and the subscriber is upstream's own: an `fmt` layer formatting JSON with `flatten_event(true)` and `without_time()`, writing into a buffer. Timings and metric timestamps are replaced with `<VARIES>` before anything is compared.

---
## B001 — Does the hosted document carry NO item named add_custom_field while the private capture does?

`B001 · item:add_custom_field in docs.rs datafusion-tracing@55.0.0 · confirmed`

**Expect** `exit:1`

**Control** (`exit:0`, mode `opposite`): `the private capture of the same version`

```text
no index entry named 'add_custom_field'
```

A tripwire on this repository's central claim, not a span observation. The first draft of this probe searched the documents as text and came out `refuted` -- correctly, and that refutation is the finding in B002. Rewritten to ask the structured index whether any ITEM carries the name, which is the thing that decides whether a reader can look the method up.

---
## B002 — Does the hosted document SHOW add_custom_field being called while documenting no such method?

`B002 · prose:.add_custom_field( in docs.rs datafusion-tracing@55.0.0 · confirmed`

**Expect** `exit:0`

**Control** (`exit:0`, mode `record-both`): `the private capture of the same version`

```text
, 64, 3, 10).map(|fmt| fmt.to_string())         }))         .add_custom_field("env", "production")
```

The sharpest statement of why this repository exists. The crate's front-page documentation -- the first thing anyone reads -- chains six builder methods, and docs.rs documents none of them: the page for the type they are called on answers 404. A reader sees the call, searches for the method, finds nothing, and concludes they have misread the example. Found by accident: the first draft of B001 searched raw text, expected the name to be absent, and was refuted by this.

---
## S001 — Does registering the rule produce a node span at all?

`S001 · plain · confirmed`

**Expect** `InstrumentedExec`

**Control** (`absent:InstrumentedExec`, mode `opposite`): `bare`

```text
)","otel.name":"DataSourceExec","name":"InstrumentedExec"},"spans":[{"datafusion.boundedness":"B
```

The control is the point: without the rule the same query under the same subscriber emits no node spans, so the span is the rule's doing and not the subscriber's.

---
## S002 — Does preview_limit(0) suppress datafusion.preview?

`S002 · preview-zero · confirmed`

**Expect** `absent:datafusion.preview`

**Control** (`datafusion.preview`, mode `opposite`): `preview-on`

```text
absent 'datafusion.preview'
```

preview_limit defaults to 0, so the absent case is also the default case. Worth knowing that setting preview_fn alone changes nothing.

---
## S003 — Does record_metrics(false) suppress the datafusion.metrics.* fields?

`S003 · metrics-off · confirmed`

**Expect** `absent:datafusion.metrics.`

**Control** (`datafusion.metrics.output_rows`, mode `opposite`): `metrics-on`

```text
absent 'datafusion.metrics.'
```

record_metrics is off by default, which is the single most common reason a trace looks empty of numbers.

---
## S004 — Does add_custom_field reach a span only when the macro also declares the key?

`S004 · custom-undeclared · confirmed`

**Expect** `absent:"env"`

**Control** (`"env"`, mode `opposite`): `custom-declared`

```text
absent '"env"'
```

The sharpest probe here. Both halves call add_custom_field("env", "probe") with identical options; the only difference is whether the macro declares `env = field::Empty`. A tracing span's field set is fixed at creation, so the undeclared value goes nowhere -- silently, with no error at compile time or run time. Upstream's own comment in integration-utils says the same: custom fields keys must be defined at compile time.

---
## S005 — Does pretty_format_compact_batch draw the |===| header rule?

`S005 · preview-on · confirmed`

**Expect** `|===`

**Control** (`absent:|===`, mode `opposite`): `preview-zero`

```text
----+---------+\n| column1 | column2 |\n|=========+=========|\n| 1       | alpha   |
```

The compact formatter's header separator is what distinguishes its output from pretty_format_batches at a glance, and no signature carries it.

---
## S006 — Do the node spans carry the CALLING crate's target rather than datafusion_tracing?

`S006 · plain · recorded`

**Expect** `"target":"probe"`

**Control** (`absent:"target":"datafusion_tracing"`, mode `record-both`): `plain`

```text
usy":"<VARIES>","time.idle":"<VARIES>"},"target":"probe","span":{"datafusion.boundedness":"Bound
```

The macros default target: to module_path!(), which expands at the call site. An EnvFilter written as datafusion_tracing=info therefore receives nothing at all. Both halves are the same capture read two ways, so this is recorded rather than confirmed -- there is no second construction that would come out the other way without passing target: explicitly.

---
## S007 — Does full() emit Rule spans that phase_only() does not?

`S007 · rules-full · confirmed`

**Expect** `"Rule"`

**Control** (`absent:"Rule"`, mode `opposite`): `rules-phase-only`

```text
":"CombinePartialFinalAggregate","name":"Rule"},"spans":[{"logical_plan":"SubqueryAlia
```

On a large plan the per-rule spans are the bulk of the trace volume, so this is the switch to reach for before sampling.

---
## S008 — Can the undocumented builder instrument the physical optimizer alone?

`S008 · rules-physical-only · confirmed`

**Expect** `Phase`

**Control** (`Phase`, mode `differs`): `rules-full`

```text
.name":"optimize_physical_plan","name":"Phase"},"spans":[{"logical_plan":"SubqueryAli
```

There is no documented way to do this: full() and phase_only() are the whole published surface, and physical_optimizer() is on the builder that docs.rs answers 404 for. Both captures contain Phase spans, so the finding is that they differ -- which is why this probe uses the differs mode rather than an opposite.

---
## S009 — Does an INFO filter drop instrumentation emitted at DEBUG entirely?

`S009 · debug-filtered-to-info · confirmed`

**Expect** `absent:InstrumentedExec`

**Control** (`InstrumentedExec`, mode `opposite`): `info-filtered-to-info`

```text
absent 'InstrumentedExec'
```

Choosing instrument_with_debug_spans! and then filtering at INFO produces a silent nothing. The two halves differ only in which macro built the rule.

---
## S010 — Does the DEBUG-level macro emit at DEBUG when nothing filters it?

`S010 · debug-level · confirmed`

**Expect** `"level":"DEBUG"`

**Control** (`absent:"level":"DEBUG"`, mode `opposite`): `plain`

```text
{"level":"DEBUG","fields":{"message":"close","time.busy"
```

The level is baked into the macro's name, which is the only thing distinguishing the six of them and is invisible in a signature because there is no signature.

---
