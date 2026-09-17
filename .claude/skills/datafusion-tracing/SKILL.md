---
name: datafusion-tracing
description: Find what datafusion-tracing and instrumented-object-store can actually do, at the pinned release, from an index that includes the API their published documentation omits, the macro grammar, the span contract observed from upstream's own trace captures, executed probes, and the opentelemetry version matrix. Use when instrumenting DataFusion queries, wiring a tracing-subscriber or an OTLP exporter, deciding which spans and fields you will get, or debugging a trace that came out empty. Do not use for general DataFusion questions, which are a different subject.
---

# datafusion-tracing capability repository

The whole published API of this crate is **fifteen items**. That is not the problem.

The problem is that `lib.rs` declares `mod options;` and `mod rule_options;` **private**, and
re-exports only the two `*Options` types out of them. Both option *builders* are `pub` inside
those private modules, so no published artifact carries them:

- `docs.rs/.../struct.InstrumentationOptionsBuilder.html` answers **404**
- neither builder appears in `all.html`
- the hosted rustdoc JSON carries both structs with **zero impl blocks**

**16 public methods exist that no published artifact documents** — including the six the crate's
own front-page example chains. That example is *in* the hosted documentation. A reader sees
`.add_custom_field("env", "production")` demonstrated, searches for the method, finds nothing,
and concludes they misread it. Three of the names — `record_metrics`, `preview_limit`,
`preview_fn` — are also public *fields* on `InstrumentationOptions`, so a name search does find
something, and lands you on struct-literal construction, which has no `add_custom_field` at all.

So the failure this repository exists to prevent is not "I could not find the answer". It is
**concluding from the documentation that the answer does not exist**. `content/` is built from
the hosted document *and* a local `--document-private-items` capture, and every row says which.

Nothing here queries the network or a service.

## What is pinned

`datafusion-tracing` and `instrumented-object-store` **55.0.0**, whose corpus is tag `55.0.0` at
commit `d8f205bf52ef`. Alongside them, the wiring you cannot use them without, at the versions
this release is known to compose with rather than the newest published: `tracing`,
`tracing-core`, `tracing-subscriber`, `tracing-futures`, `tracing-attributes`, and
`tracing-opentelemetry 0.32` with the `opentelemetry` family at `0.31`. **11 crates.**

DataFusion, Arrow and `object_store` are deliberately **not** indexed. Their access paths land
in `index/unresolved.tsv`. That is a boundary, not a gap.

## Escalation ladder

Stop at the first rung that answers the question.

**0. Which question is this?** → `content/topics/00-map.md`, then `content/index/questions.tsv`.

**This rung is not optional here.** 18 routed questions, and the column to read is `rejected` —
the source you were about to trust and what it would have told you. Five of the eight limits in
`reference.md` are consequences of trusting the wrong one.

```
rg -P '^How do I configure' content/index/questions.tsv | cut -f3,4
```

**1. Does it exist, and can I actually call it?** → `rg` over `content/index/*.tsv`.

The `visibility` column is the one that matters: `supported`, `doc-hidden`,
`reachable-undocumented`, `internal`.

```
rg -P '\treachable-undocumented\t' content/index/methods.tsv | cut -f1,2
rg -P '^datafusion_tracing::options::InstrumentationOptions\t' content/index/methods.tsv
```

**2. A direct lookup.** → `content/catalogs/` — `options.md` (the builders nothing else
documents), `macros.md` (49 invocation arms), `spans.md` (what the trace looks like),
`compatibility.md` (the version lock), `crate-map.md`, and `not-reachable.md`.

**3. What does this seam refuse to answer?** → `content/seams/<slug>.md`, 8 of them. Read the
**What this seam cannot tell you** section *before* concluding a capability is missing. Most of
what looks absent in this library is either undocumented-but-present or genuinely DataFusion's.

**4. What will the trace actually contain?** → `content/index/spans.tsv` and `span-fields.tsv`,
derived from upstream's own pinned trace captures; 9 spans, and the option each field depends on.

**5. How does it really behave?** → `content/index/behaviors.tsv` and
`content/probes/00-index.md`. **12 probes, 11 confirmed**, each an executed query under a
capturing subscriber with a control that had to come out the other way. `content/corpus/` holds
upstream's source, tests and examples verbatim.

**6. What is my own code missing?** → the rule corpus, against the repository you are editing.

```
ast-grep scan -c queries/sgconfig.yml --filter '^project-' PATH
```

Rung 6 is the one to reach for unprompted after writing instrumentation code. None of what it
catches fails loudly: a rule registered one link too early still compiles and still traces —
just not the plan that ran.

## Rules that keep answers correct

**Absence from the documentation is not absence from the library.** That is this subject's
defining property, not an edge case. Check `visibility` before reporting that something does not
exist, and quote `reached_via` when you recommend a method with no path a caller can `use`.

**`reachable-undocumented` and `internal` are not the same, and rustdoc cannot tell them
apart.** rustdoc records `InstrumentedExec` as `public` exactly as it records
`InstrumentationOptionsBuilder`. One is returned by a documented call; the other is reached by
nothing and upstream says it is private by design. Recommending the second would be worse than
saying nothing.

**The span target is your crate, not this one.** Every macro arm that omits `target:` expands to
`module_path!()`, which expands at the **call site**. An `EnvFilter` written as
`datafusion_tracing=info` receives **nothing**, silently. `instrumented-object-store` is the
opposite — it calls `tracing` directly and carries its own target. The two halves of this
library do not agree.

**A custom field needs declaring as well as setting.** `add_custom_field` sets a value; a
`tracing` span's field set is fixed at creation, so the macro must also carry
`env = field::Empty`. Neither half alone errors, at compile time or at run time.

**The metrics field vocabulary is open.** Names are built with `format!()` from whatever node
ran, so a field absent from `span-fields.tsv` is **unobserved, not unavailable**.

**Register the instrumentation rule last.** It wraps the plan it is handed, so anything
registered after it produces nodes nothing instrumented.

**Snapshot-derived rows are `recorded`; only a probe with a control makes one `confirmed`.**
Say which you are quoting.

**ast-grep establishes syntax, not semantics — and cannot parse TOML at all** (measured:
`--lang toml` is rejected). Manifest checks are ripgrep recipes by necessity.

## Reporting

Cite the file you read the answer in, and the probe id when the claim is behavioural. When a
capability exists but you are not recommending it, say so — the point of this repository is that
the caller learns the option existed. When an index is silent, report silence rather than
absence, and say which applies: not documented but present (`visibility`), not reachable
(`unreachable.tsv`), not an indexed crate (`unresolved.tsv`), or not observed under the pinned
scenarios.

## Additional references

`reference.md` for the layout, every column schema, the rule inventory and the eight known
limits. `queries/README.md` before writing or changing a rule. `build/README.md` before
rebuilding or re-pinning.
