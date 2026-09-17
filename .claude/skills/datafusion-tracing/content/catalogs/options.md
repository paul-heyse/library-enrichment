# The options surface

Two option types, each with a builder. **The builders are documented nowhere else.** `mod options;` and `mod rule_options;` are private in `lib.rs`, which selectively re-exports only the two `*Options` types; the builders are `pub` inside those private modules, so:

- `docs.rs/.../struct.InstrumentationOptionsBuilder.html` answers **404**
- neither builder appears in `all.html`
- the hosted rustdoc JSON carries both structs with **zero impl blocks**

Every row below marked `reachable-undocumented` came from a local `--document-private-items` capture and appears in no published artifact. Three of the names — `record_metrics`, `preview_limit`, `preview_fn` — also exist as public *fields* on `InstrumentationOptions`, so searching for the name finds something and lands you on struct-literal construction, which silently has no `add_custom_field`.

## `InstrumentationOptionsBuilder`

`datafusion_tracing::options::InstrumentationOptionsBuilder` — reached via `InstrumentationOptions::builder()`

The builder for `InstrumentationOptions`.

| Method | Visibility | Signature |
|---|---|---|
| `add_custom_field` | **undocumented** | `fn add_custom_field<K: Into<String>, V: Into<String>>(self, key: K, value: V) -> Self` |
| `build` | **undocumented** | `fn build(self) -> InstrumentationOptions` |
| `custom_fields` | **undocumented** | `fn custom_fields(self, fields: HashMap<String, String>) -> Self` |
| `preview_fn` | **undocumented** | `fn preview_fn(self, func: Arc<dyn Fn(&datafusion::arrow::record_batch::RecordBatch) -> Result<String, datafusion::arrow::error::ArrowError> + Send + Sync>) -> Self` |
| `preview_limit` | **undocumented** | `fn preview_limit(self, limit: usize) -> Self` |
| `record_metrics` | **undocumented** | `fn record_metrics(self, record: bool) -> Self` |

## `RuleInstrumentationOptionsBuilder`

`datafusion_tracing::rule_options::RuleInstrumentationOptionsBuilder` — reached via `RuleInstrumentationOptions::builder()`

The builder for `RuleInstrumentationOptions`.

| Method | Visibility | Signature |
|---|---|---|
| `all` | **undocumented** | `fn all(self) -> Self` |
| `all_phase_only` | **undocumented** | `fn all_phase_only(self) -> Self` |
| `analyzer` | **undocumented** | `fn analyzer(self) -> Self` |
| `analyzer_phase_only` | **undocumented** | `fn analyzer_phase_only(self) -> Self` |
| `build` | **undocumented** | `fn build(self) -> RuleInstrumentationOptions` |
| `optimizer` | **undocumented** | `fn optimizer(self) -> Self` |
| `optimizer_phase_only` | **undocumented** | `fn optimizer_phase_only(self) -> Self` |
| `physical_optimizer` | **undocumented** | `fn physical_optimizer(self) -> Self` |
| `physical_optimizer_phase_only` | **undocumented** | `fn physical_optimizer_phase_only(self) -> Self` |
| `plan_diff` | **undocumented** | `fn plan_diff(self) -> Self` |

## `InstrumentationOptions`

`datafusion_tracing::options::InstrumentationOptions`

Configuration options for instrumented execution plans.

Public fields: `custom_fields`, `preview_fn`, `preview_limit`, `record_metrics` — settable directly, which is the trap: a struct literal compiles and has no `add_custom_field`.

| Method | Visibility | Signature |
|---|---|---|
| `builder` | supported | `fn builder() -> InstrumentationOptionsBuilder` |

## `RuleInstrumentationOptions`

`datafusion_tracing::rule_options::RuleInstrumentationOptions`

Configuration options for instrumented DataFusion rules (Analyzer, Optimizer, Physical Optimizer).

| Method | Visibility | Signature |
|---|---|---|
| `builder` | supported | `fn builder() -> RuleInstrumentationOptionsBuilder` |
| `full` | supported | `fn full() -> Self` |
| `phase_only` | supported | `fn phase_only() -> Self` |
| `with_plan_diff` | supported | `fn with_plan_diff(self) -> Self` |

## Which rule-instrumentation constructor

`RuleInstrumentationOptions` offers three documented entry points and a builder with ten undocumented ones. The documented three are shorthands:

| You want | Write |
|---|---|
| everything | `RuleInstrumentationOptions::full()` |
| phase spans only, no per-rule spans | `RuleInstrumentationOptions::phase_only()` |
| everything plus plan diffs | `RuleInstrumentationOptions::full().with_plan_diff()` |
| one phase only | `RuleInstrumentationOptions::builder().optimizer().build()` |
| two phases, one of them coarse | `…builder().analyzer().optimizer_phase_only().build()` |

The last two rows are the reason the builder matters: there is no documented way to instrument the physical optimizer and nothing else, and per-rule spans on a large plan are the bulk of the trace volume.

## Reading this back

```bash
rg -P '\treachable-undocumented\t' content/index/methods.tsv | cut -f1,2,5
```
