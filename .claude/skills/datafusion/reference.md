# Decision and contract layers

Start at [task routes](content/routes/tasks.md), [crate roles](content/routes/crates.md), or
[representation routes](content/routes/representations.md). Use `scripts/reference.py find/show/compare`
for bounded JSON retrieval. A record can be read directly without the CLI.

| Layer | Location | Contract |
|---|---|---|
| Reviewed authoring | `authoring/capabilities/*.json` | Stable capability/claim IDs, conditional alternatives, input/output facets, evidence and unknowns |
| Generated briefs | `content/capabilities/` | Validated operation links, readable pages and structured records |
| Typed operations | `content/index/operations.tsv` | path, stable ID, kind, crate, record file, page plus anchor, display signature; no header |
| Complete contracts | `content/operations/*.json` | Full docs, raw type_tree, artifact-scoped IDs/references, implementation context, span, aliases and links |
| Modules | `content/modules/` | Module overviews previously absent from compact API pages |
| Diagnostics | `content/contract-diagnostics.json` | Unresolved links and display placeholders; no inferred replacement contract |
| Coverage | `content/capabilities/coverage.json` | Reviewed scope separate from full discovery and unreviewed packages |

Operation IDs derive from crate/version/path/kind and implementation context; raw rustdoc IDs are
local to the recorded artifact and never global identities. Original type trees retain lifetimes,
generics, predicates and variant/field payloads. Trait signatures rendered as async may have an
expanded Future in the preserved raw tree. Read the implementation context when paths are shared.
Unresolved type-reference paths remain explicit nulls. Availability in hosted documentation is
not a claim about the consumer's features.

Claim evidence classes distinguish upstream documentation, contract interpretation, runtime
observations and authored conditional judgments. The runtime receipt names tests and their
profile; it does not promote every sentence in a brief to an executed assertion. Follow
[maintenance.md](maintenance.md) for validation and update behavior.

---

# Reference: layout, schemas, and query recipes

Everything below is generated from pinned sources by `build/build.py` and verified by
`build/verify.py`. Counts are from the current build; `content/PROVENANCE.json` is authoritative.

## What is pinned

| Crate set | Version | Crates |
|---|---|---|
| DataFusion | 55.1.0 | 38 |
| Arrow family (incl. `parquet`) | 59.3.0 | 20 |
| `object_store` | 0.13.2 | 1 |
| `sqlparser` | 0.62.0 | 1 |

7,134 canonical items · 21,047 methods · 2,335 trait-implementation edges · 6,406 access-path
aliases · 1,315 modules. 37 access paths resolve to targets outside the pinned set (`core::ops`,
`csv`, `base64`) and are reported rather than guessed at.

## Layout

```
content/
  PROVENANCE.json        versions, per-crate docs.rs config, counts, per-file sha256
  index/*.tsv            line-oriented projection            -> ripgrep
  model/<module>.json    structure, signatures, edges        -> ast-grep
  api/<module>.md        item docs and compact member summaries     -> Read
  traits/<Trait>.md      the 30 extension points             -> Read
  topics/<slug>.md       15 capability axes + 00-map.md      -> Read
  catalogs/*.md          settings, SQL functions, crate map  -> Read / ripgrep
  corpus/examples/       79 upstream programs, verbatim      -> ast-grep
  corpus/guides/         59 upstream guide pages, verbatim   -> ripgrep / Read
queries/
  sgconfig.yml           always passed explicitly with -c
  rules/{model,corpus,project,generated}/
  utils/                 shared and parameterized sub-rules
  outline/datafusion.yml custom outline extractors
  rule-tests/            valid/invalid fixtures + snapshots
build/                   the generator; stdlib + the ast-grep binary only
```

### Path construction

There is no lookup step. From a canonical path, take everything before the last `::` and replace
`::` with `.`:

| Canonical path | Prose | Records |
|---|---|---|
| `datafusion_expr::expr::Expr` | `content/api/datafusion_expr.expr.md` | `content/model/datafusion_expr.expr.json` |
| `datafusion_session::table::TableProvider` | `content/api/datafusion_session.table.md` | `content/model/datafusion_session.table.json` |

Anchors within a prose page are the lowercased item name: `…#tableprovider`.

### High-traffic files inside the guide corpus

Upstream generates several of these from code, so they are exact for the pinned release and worth
knowing by name rather than searching for:

| File | Holds |
|---|---|
| `corpus/guides/user-guide/configs.md` | Every configuration setting with its default (204 KB) |
| `corpus/guides/user-guide/sql/scalar_functions.md` | Every built-in scalar function with examples (167 KB) |
| `corpus/guides/user-guide/sql/aggregate_functions.md` | Aggregate functions |
| `corpus/guides/user-guide/sql/window_functions.md` | Window functions |
| `corpus/guides/library-user-guide/upgrading/*.md` | Per-release breaking changes and new APIs, 46.0.0 → 55.0.0 |
| `corpus/guides/library-user-guide/custom-table-providers.md` | The narrative for `TableProvider` (41 KB) |
| `corpus/guides/library-user-guide/functions/adding-udfs.md` | The narrative for all three UDF traits (59 KB) |

The upgrade guides are the fastest way to find capability that postdates a training window: read
them newest-first and cross-check anything they introduce against `index/symbols.tsv`.

## Topic pages

`topics/00-map.md` indexes fifteen capability axes: sessions and runtime, reading data, writing
data, the DataFrame API, expressions, SQL, logical planning, the optimizer, physical execution,
user-defined functions, custom table providers, catalogs, Arrow interop, Parquet, and cross-process
interop.

Each page has the same shape, and everything below the first paragraph is generated from the model:

| Section | Source |
|---|---|
| Mental model | Curated, one paragraph |
| Entry points | The types that provide the capability, with method counts and links |
| Extension points | Traits, with required/provided counts and implementor counts |
| Configuration methods | The `with_*` builders on the relevant types |
| Settings | The `datafusion.*` settings for this area |
| Runnable examples | The upstream examples in the matching directories |
| Upstream guides | The guide pages for this area |
| Decision rules, Anti-patterns, Agent checklist | Curated, factual |

Topic seeds are resolved against the model at build time. An unresolved or ambiguous seed **fails
the build** rather than disappearing from a page, because a page that silently drops a type would
assert by omission that the capability does not exist. Seeds may be leaf names when unique, or
qualified paths when not — `Statistics` alone is four different types across the pinned set.

## Catalogs

| File | Holds |
|---|---|
| `catalogs/config-options.md` | 155 settings by subsystem with heuristic builder-name candidates. A dash means no name match was found; typed ConfigOptions fields and options_mut may still provide access. Check runtime versus session-setting lifetime. |
| `catalogs/sql-functions.md` | 332 scalar, aggregate and window functions plus operators, categorized, each linking to its upstream description |
| `catalogs/crate-map.md` | All 60 crates with item and trait counts, and the most-implemented extension point in each |

## Index schemas

Tab-separated, sorted, one record per line. No header row.

| File | Rows | Columns |
|---|---|---|
| `symbols.tsv` | 7,134 | `canonical_path · kind · crate · api_page · alias_count · method_count · summary` |
| `methods.tsv` | 21,047 | `owner_path · method · via_trait ("-" if inherent) · signature · summary` |
| `impls.tsv` | 2,335 | `trait_path · implementor_path · implementor_crate` |
| `aliases.tsv` | 6,406 | `access_path · canonical_path · kind` |
| `features.tsv` | 172 | `crate · feature · enables (comma-separated) · "default" or "-"` |

`kind` is one of `struct`, `enum`, `union`, `trait`, `trait_alias`, `function`, `type_alias`,
`constant`, `static`, `macro`, `proc_macro`, `primitive`.

## Model record schema

`content/model/<module>.json` is `{module, crate, api, items[]}`. Each item:

```json
{
  "path": "datafusion_session::table::TableProvider",
  "name": "TableProvider",
  "kind": "trait",
  "crate": "datafusion-session",
  "signature": "trait TableProvider: Any + Debug + Sync + Send",
  "summary": "A table which can be queried and modified.",
  "doc": "api/datafusion_session.table.md#tableprovider",
  "aliases": ["datafusion::datasource::TableProvider"],
  "implementors": ["datafusion_catalog::memory::table::MemTable"],
  "required_methods": ["scan", "schema", "table_type"],
  "provided_methods": ["supports_filters_pushdown", "merge_into"],
  "methods": [{"name": "scan", "signature": "async fn scan(...) -> ...", "via_trait": null}]
}
```

Optional keys appear only when non-empty: `deprecated`, `aliases`, `variants`, `fields`,
`implements`, `implementors`, `methods`. `doc` is `null` when the item has no doc comment.
`implements` omits the ubiquitous derives (`Clone`, `Debug`, `PartialEq`, …); the prose page lists
those separately under **Derives**.

Prose lives only in `api/`. A model record points at it and never copies it.

## Query recipes

All runnable from the skill directory.

**Does a capability exist?**
```bash
rg -i 'pushdown|prune|skip' content/index/symbols.tsv
rg -iP '\t(spill|memory_pool|batch_size)' content/index/methods.tsv
```

**What can this type do?** (`SessionConfig` has 64 methods; reading the constructor finds none of them)
```bash
rg -P '^datafusion_execution::config::SessionConfig\t' content/index/methods.tsv | cut -f2,4
```

**Where is `datafusion::prelude::Expr` really defined?**
```bash
rg -P '^datafusion::prelude::Expr\t' content/index/aliases.tsv
```

**Who implements this trait?**
```bash
rg -P '^datafusion_session::table::TableProvider\t' content/index/impls.tsv | cut -f2
```

**What does this feature turn on?**
```bash
rg -P '^datafusion\t' content/index/features.tsv
```

**Structured questions over the model:**
```bash
ast-grep scan -c queries/sgconfig.yml --filter '^model-deprecated$' content/model
ast-grep scan -c queries/sgconfig.yml --filter '^model-' content/model --json=stream --include-metadata
```

**How is it used in practice?**
```bash
ast-grep run -l rust -p 'impl $TRAIT for $TYPE { $$$ }' content/corpus/examples --json=compact
ast-grep outline --no-default-outline-rules --outline-rules queries/outline/datafusion.yml content/corpus/examples
rg -l 'register_object_store' content/corpus/examples
```

**What is my own code missing?**
```bash
ast-grep scan -c queries/sgconfig.yml --filter '^project-' /path/to/the/repo/being/edited
```

Doc prose, SQL string literals and feature names are ripgrep territory — ast-grep matches syntax,
and a name inside a `//` comment is not a node it can match.

## Rule inventory

Thirteen rules, each with `valid` and `invalid` fixtures under `queries/rule-tests/`. All carry
`severity: hint`: a match is a finding of interest, never a violation.

### `model-*` — structured questions over `content/model`

| Rule | Answers | Current |
|---|---|---|
| `model-extension-points` | Traits with concrete implementors | 207 |
| `model-async-methods` | Methods needing a runtime and an await | 520 |
| `model-deprecated` | Items deprecated in the pinned release | 116 |
| `model-builder-methods` | `with_*` chainable configuration | 887 |
| `model-undocumented` | Public items with no doc comment | 1,362 |

`model-builder-methods` and `model-deprecated` are the two highest-yield: builder chains are the
largest blind spot in the API, and deprecated code still compiles, so nothing forces a correction.

### `corpus-*` — how upstream examples actually use it

| Rule | Answers | Current |
|---|---|---|
| `corpus-extension-impl` | Every extension point an example implements | 107 |
| `corpus-session-registration` | `register_*` / `add_*` / `enable_*` calls | 108 |
| `corpus-builder-chain` | Which `with_*` methods anyone reaches for | 137 |

### `project-*` — capability gaps in the repository being edited

| Rule | Flags |
|---|---|
| `project-tableprovider-no-pushdown` | `TableProvider` not overriding `supports_filters_pushdown`; every filter runs after the scan |
| `project-execution-plan-no-statistics` | `ExecutionPlan` reporting no statistics; the optimizer plans that node blind |
| `project-unbounded-session` | `SessionContext::new` / `new_with_config`: default `RuntimeEnv` has an unbounded pool; disk defaults to OS temporary storage |
| `project-collect-over-stream` | Awaited `collect()` where `execute_stream()` would bound memory |
| `project-deprecated-api` | Qualified use of one of 113 unambiguously deprecated identifiers (generated) |

`project-deprecated-api` is regenerated from the model on every build, so it cannot go stale.

## Provenance and known limits

`content/PROVENANCE.json` records crate versions, rustdoc `format_version`, the ast-grep version,
per-crate docs.rs configuration, counts, and a sha256 for every generated file — the full
cache-invalidation key.

Four things this repository deliberately does not claim:

- **Feature gating is not in the API surface.** rustdoc formats 57–61 emit no per-item
  `cfg(feature = …)`. `features.tsv` comes from each crate's `[features]` table instead.
- **Four crates were documented without `all-features`** (`datafusion-physical-expr-adapter`,
  `datafusion-pruning`, `parquet-variant`, `parquet-variant-compute`). Their surface here is
  default-features only. The other 56 set `all-features = true`.
- **Two extension points show zero implementors** — `PartitionEvaluator` and
  `UserDefinedLogicalNode`. That is accurate, not missing data: their implementations are private
  types, or a generic blanket impl with no resolvable concrete path.
- **ast-grep resolves no imports.** `project-deprecated-api` therefore matches only qualified paths
  rooted in a pinned crate, and misses `use arrow::x::name; name()`. Precision was chosen over
  recall because an unanchored version produced 47 hits on a real workspace, all false.

## Rebuilding

```bash
python3 build/build.py                 # ~14s warm, from build/.cache
python3 build/verify.py                # determinism, integrity, rule tests, probes
ast-grep test -c queries/sgconfig.yml  # rule fixtures alone
```

The skill directory is self-contained: Python 3.11+ standard library plus the `ast-grep` binary,
no imports from any host repository. Copy it anywhere and rebuild. Point `build/build.py` at a
different `--manifest` to cover a different crate family.
