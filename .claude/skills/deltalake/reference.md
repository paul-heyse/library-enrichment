# Reference: layout, schemas, and query recipes

For task selection start with [reviewed decisions](content/routes/tasks.md) and the
[bounded reader](scripts/README.md). This page retains direct-file and structural-query recipes.
Full member contracts are under `content/operations/`; compact legacy module pages link to them.

Everything below is generated from pinned sources by `build/build.py` and verified by
`build/verify.py`. Counts are from the current build; `content/PROVENANCE.json` is
authoritative.

## What is pinned

| Source | Pin |
|---|---|
| delta-rs | `58f07cd62bfbce3649a7e1c87c696288068ae184` on `main`, 2026-09-15 |
| Delta kernel | `buoyant-data/delta-kernel-rs@8ba063f8f84fec222000f66d40d70911d7c79675` |
| Crates documented | 13 (11 delta-rs libraries + `buoyant_kernel` + `buoyant_kernel_engine`) |
| Toolchain | `nightly-2026-09-13`, rustc 1.100.0-nightly `809936eac`, rustdoc `format_version` 61 |
| Target | `x86_64-unknown-linux-gnu` |
| Transitively | arrow 59.3.0 · parquet 59.3.0 · datafusion 55.1.0 · object_store 0.13.2 · sqlparser 0.62.0 |

804 canonical items · 2,308 compact-index methods · 124 trait-implementation edges · 1,168 access-path
aliases · 211 modules.

**Why a commit and not a release.** At acquisition (2026-09-15), the published deltalake
line targeted Arrow 58 / DataFusion 53. This retained commit supplies the Arrow 59 /
DataFusion 55 profile. These are capture-time facts, not a current release recommendation. docs.rs therefore has nothing to serve for this pin, which is why
`build/acquire.py` produces the rustdoc JSON locally — see `build/README.md`.

**Why the kernel is pinned separately.** delta-rs declares `delta_kernel` as a git dependency
tracking `branch = "buoyant/main"`. A branch is not a pin: the same delta-rs commit resolves to
a different kernel next week. Replay restores the retained lock bytes. Explicit refresh resolves a new lock and checks each
package against the declared revision; it never silently follows a moved branch.

## Layout

```
content/
  PROVENANCE.json        versions, per-crate feature envelope, counts, per-file sha256
  index/*.tsv            line-oriented projection            -> ripgrep
  model/<module>.json    structure, signatures, edges        -> ast-grep
  api/<module>.md        compact legacy summaries            -> Read
  operations/*.json|md   full docs, types, outputs and access -> reader/Read
  capabilities/*.json|md reviewed conditional choices         -> reader/Read
  routes/*.md           task/crate/representation/effects     -> Read
  traits/<Trait>.md      the 42 extension points             -> Read
  topics/<slug>.md       15 capability axes + 00-map.md      -> Read
  catalogs/*.md          operations, properties, features, errors, foreign impls, crates
  corpus/examples/       8 upstream programs, verbatim       -> ast-grep
  corpus/tests/          27 integration tests, verbatim      -> ast-grep
  corpus/guides/         55 upstream guide pages, verbatim   -> ripgrep / Read
  corpus/protocol/       the normative Delta protocol spec   -> ripgrep / Read
queries/
  sgconfig.yml           always passed explicitly with -c
  rules/{model,corpus,project,generated}/
  utils/                 shared and parameterized sub-rules
  outline/deltalake.yml  custom outline extractor
  rule-tests/            valid/invalid fixtures + snapshots
build/                   the generator; stdlib + the ast-grep binary only
build/acquired/          the locally produced rustdoc JSON — tracked, see build/README.md
```

### Path construction

There is no lookup step. From a canonical path, take everything before the last `::` and
replace `::` with `.`:

| Canonical path | Prose | Records |
|---|---|---|
| `deltalake_core::table::DeltaTable` | `content/api/deltalake_core.table.md` | `content/model/deltalake_core.table.json` |
| `buoyant_kernel::schema::StructType` | `content/api/buoyant_kernel.schema.md` | `content/model/buoyant_kernel.schema.json` |

Anchors within a prose page are the lowercased item name: `…#deltatable`.

### High-traffic files inside the corpus

| File | Holds |
|---|---|
| `corpus/protocol/PROTOCOL.md` | The normative Delta specification: log actions, table features, reader/writer versions. delta-rs *implements* this; it does not define it. |
| `corpus/guides/feature-table.md` | Upstream's own view of which operations and backends are supported |
| `corpus/guides/usage/*.md` | Task-shaped narratives: loading, appending, merging, deleting, constraints, CDF, partitions |
| `corpus/guides/integrations/delta-lake-datafusion.md` | The DataFusion seam, in prose |
| `corpus/guides/upgrade-guides/guide-1.0.0.md` | What changed on the way to 1.0 |
| `corpus/tests/it_datafusion/` | 18 integration tests covering merge, optimize, vacuum, restore, checkpoint, filesystem check and the table provider |

The corpus separates `examples/` from `tests/` deliberately. Eight example files ship; the
integration suite is an order of magnitude larger and is where most real usage evidence lives.
A test is evidence that upstream keeps something working — it is not a curated demonstration,
and the directory name is there so you can say which kind you read.

## Index schemas

Tab-separated, sorted, one record per line. No header row.

| File | Rows | Columns |
|---|---|---|
| `symbols.tsv` | 804 | `canonical_path · kind · crate · api_page · alias_count · method_count · summary` |
| `methods.tsv` | 2,308 | `owner_path · method · via_trait ("-" if inherent) · signature · summary` |
| `impls.tsv` | 124 | `trait_path · implementor_path · implementor_crate` — traits defined *in* this index |
| `foreign-impls.tsv` | 2,233 | `trait_path · implementor_path · nameable (yes\|no) · crate` — traits defined elsewhere |
| `aliases.tsv` | 1,168 | `access_path · canonical_path · kind` |
| `features.tsv` | 123 | `crate · feature · enables · "default" or "-"` |
| `coverage.tsv` | 135 | `crate · feature · on\|off · enables · reason` |
| `unresolved.tsv` | 11 | `access_path · target_path` — re-export targets outside the indexed set |
| `unnameable.tsv` | 8 | `canonical_path · kind · crate · method_count` — real, callable, not nameable |

`kind` is one of `struct`, `enum`, `union`, `trait`, `trait_alias`, `function`, `type_alias`,
`constant`, `static`, `macro`, `proc_macro`, `primitive`.

The last three tables do not exist in the DataFusion skill. They were added here because each
records something that is otherwise invisible — and an invisible limitation is indistinguishable
from an absent capability.

## Model record schema

`content/model/<module>.json` is `{module, crate, api, items[]}`. Each item carries `path`,
`name`, `kind`, `crate`, `signature`, `summary`, `doc`, and, when non-empty, `deprecated`,
`aliases`, `variants`, `fields`, `implements`, `implementors`, `required_methods`,
`provided_methods`, `methods`. Prose lives only in `api/`; a model record points at it and
never copies it.

## Catalogs

| File | Holds |
|---|---|
| `catalogs/operations.md` | Structured return-based construction, awaited outputs and explicit build/execute/flush APIs |
| `catalogs/table-properties.md` | 24 `TableProperty` variants, and the extension trait that reads them back |
| `catalogs/table-features.md` | 48 protocol feature variants across the kernel and delta-rs enums |
| `catalogs/errors.md` | 183 error variants; `DeltaTableError` is the one a retry policy branches on |
| `catalogs/foreign-impls.md` | 47 traits from DataFusion, Arrow, object_store and the kernel that this library implements, counted separately for nameable and private implementors |
| `catalogs/crate-map.md` | Each crate's item and trait counts, its feature envelope, and its most-implemented extension point |

`crate-map.md` lists 12 crates for 13 documented: the `deltalake` facade defines nothing of its
own and exists purely to re-export, so no item is attributed to it.

## Query recipes

All runnable from the skill directory.

**Does a capability exist, and how is it spelled?**
```bash
rg -i 'vacuum|retention|z_?order' content/index/symbols.tsv
rg -P '^deltalake_core::table::DeltaTable\t' content/index/methods.tsv | cut -f2,4
```

**Where is a type really defined?** (the crate is renamed by Cargo)
```bash
rg -P '^delta_kernel::schema::StructType\t' content/index/aliases.tsv
```

**Who implements this trait?** Two tables, split by where the trait is defined.
```bash
rg -P '^deltalake_core::logstore::LogStore\t' content/index/impls.tsv | cut -f2
rg -P '::ExecutionPlan\t' content/index/foreign-impls.tsv | cut -f2,3
```
The second answers "where does this library plug into DataFusion?", and its `nameable` column
is the difference between an extension point you can substitute and machinery you can only
reach through a public return value. `catalogs/foreign-impls.md` summarises it but shows at
most four implementors per trait, so grep here whenever a catalog row ends in `+n`.

**Was this feature documented, and if not why not?**
```bash
rg -P '\toff\t' content/index/coverage.tsv | cut -f1,2,5
```

**Why does `deltalake::arrow::…` resolve to nothing?**
```bash
cat content/index/unresolved.tsv
```

**Structured questions over the model:**
```bash
ast-grep scan -c queries/sgconfig.yml --filter '^model-operation-builders$' content/model
ast-grep scan -c queries/sgconfig.yml --filter '^model-' content/model --json=stream --include-metadata
```

**How is it actually used?**
```bash
ast-grep run -l rust -p 'impl $TRAIT for $TYPE { $$$ }' content/corpus --json=compact
ast-grep outline --no-default-outline-rules --outline-rules queries/outline/deltalake.yml content/corpus/tests
rg -l 'register_handlers' content/corpus
```

**What is my own code missing?**
```bash
ast-grep scan -c queries/sgconfig.yml --filter '^project-' /path/to/the/repo/being/edited
```

Doc prose, storage-option strings and feature names are ripgrep territory — ast-grep matches
syntax, and a name inside a `//` comment is not a node it can match.

## Rule inventory

Fifteen rules, each with `valid` and `invalid` fixtures under `queries/rule-tests/`. All carry
`severity: hint`: a match is a finding of interest, never a violation.

### `model-*` — structured questions over `content/model`

| Rule | Answers |
|---|---|
| `model-operation-builders` | Structs implementing `IntoFuture` — the complete operation surface |
| `model-ext-traits` | `*Ext` traits, whose methods are invisible until imported |
| `model-extension-points` | Traits with concrete implementors |
| `model-async-methods` | Methods needing a runtime and an await |
| `model-deprecated` | Items deprecated at this pin |
| `model-builder-methods` | `with_*` chainable configuration |
| `model-undocumented` | Public items with no doc comment |

### `corpus-*` — how upstream code actually uses it

`corpus-extension-impl` (every extension point implemented in the corpus) and
`corpus-operation-chain` (which `with_*` options upstream actually reaches for).

### `project-*` — capability gaps in the repository being edited

| Rule | Flags |
|---|---|
| `project-deltaops-entry-point` | `DeltaOps`, which no longer exists — the most likely thing to be written from memory |
| `project-write-without-schema-mode` | A write that will not carry a new column |
| `project-write-without-commit-properties` | No app transaction id, so a retry duplicates data |
| `project-vacuum-default-retention` | The one irreversible operation, left on its default |
| `project-merge-without-aliases` | A merge whose predicate is ambiguous across two schemas |
| `project-deprecated-api` | Qualified use of a deprecated identifier (generated from the model) |

## Provenance and known limits

`content/PROVENANCE.json` records crate versions, the resolved feature envelope per crate,
counts, and a sha256 for every generated file. `build/acquired/*/ACQUISITION.json` records the
toolchain, the rustc build hash, the resolved git revisions, the lockfile digest and the
`format_version` of every captured document.

Six things this repository deliberately does not claim:

- **Feature gating is not in the API surface.** rustdoc emits no per-item `cfg(feature = …)`.
  `features.tsv` comes from each crate's `[features]` table, and `coverage.tsv` from the feature
  set cargo actually resolved.
- **The index is one feature envelope, not the library.** `--all-features` is impossible:
  `rustls` and `native-tls` are mutually exclusive and both forward to the kernel's default
  engine. `native-tls`, `python`, `delta-cache`, sixteen `opendal-*` services and several
  kernel experiments were off. Every one is in `coverage.tsv` with a reason.
- **Derive macros cannot be indexed at all.** rustdoc format 61 emits no `paths` entry for a
  `#[proc_macro_derive]` export, so a derive macro has no canonical path to record.
  `deltalake-derive`'s sole export is such a macro, which is why that crate is excluded rather
  than documented empty.
- **Types in private modules lose their impls.** rustdoc emits a re-exported private-module type
  without any of its impls. This repository recovers them from a second
  `--document-private-items` capture, used only to fill in impls for items the public document
  already admitted — never to add items. Without it `LoadBuilder` would show zero methods.
- **Arrow, DataFusion, object_store and parquet are not indexed.** delta-rs re-exports all four;
  those access paths appear in `unresolved.tsv` rather than resolving. That is a deliberate
  boundary — use their own references.
- **ast-grep resolves no imports.** `project-deprecated-api` therefore matches only qualified
  paths rooted in a pinned crate. One rule was written and then removed for this reason: a
  credential-hygiene check could not distinguish a secret's *value* from the *name* of the
  environment variable holding it, and fired on correct code. Precision was chosen over recall.

## Rebuilding

```bash
python3 build/acquire.py                 # once; needs network, git, cargo and the pinned nightly
python3 build/build.py --manifest build/manifests/deltalake.json
python3 build/verify.py                  # determinism, integrity, rule tests, probes, transferability
ast-grep test -c queries/sgconfig.yml    # rule fixtures alone
```

Only `acquire.py` needs a toolchain or a network. Once `build/acquired/` exists, the build and
every check run offline on Python 3.11+ standard library plus the `ast-grep` binary, with no
import from any host repository — `verify.py` enforces that last property. Point
`build/build.py` at a different manifest to cover a different crate family; a docs.rs-sourced
manifest needs no acquisition step at all.

To re-pin, edit the revisions in `build/manifests/deltalake.json` and re-run `acquire.py`. Run
`python3 build/acquire.py --check` first: it reports whether the tracked branches have moved off
their pins, and never re-pins on its own.
