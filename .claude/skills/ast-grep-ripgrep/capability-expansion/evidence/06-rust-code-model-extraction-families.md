# Evidence 06 — the seven extraction families and what each can actually contribute

**Source:** the `rust-code-model` capability repository (11 crates at `ra_ap 0.0.352`,
`rustdoc-types 0.61.0`, `cargo_metadata 0.23.1`, toolchain `nightly-2026-09-13`), read
2026-09-16. Its `layers.tsv` and 33 executed probes are the authority for every claim below.

This dossier maps the proposal's §6 "Role of the seven Rust extraction families" onto what the
pinned libraries demonstrably do, and — more usefully — onto what each **cannot** contribute, so
that the canonical schema does not reserve columns nothing will ever fill.

---

## 1. The contract of each family, quoted

From `content/index/layers.tsv`:

| Family | Obtained by | **Cannot answer** |
|---|---|---|
| rustdoc-json | `rustdoc --output-format json`, or `docs.rs/crate/NAME/VERSION/json` | anything a function does — **there are no bodies in the document at all** (probe RD002); anything outside this crate's public API; anything cfg-gated off at build time (RD005) |
| syntax | `ra_ap_syntax::SourceFile::parse`, or `rust-analyzer parse` | what any name refers to; what type anything has; what a macro expands to (SY003) |
| hir | `ra_ap_hir::Semantics` over a loaded database | how a body executes as a graph; anything about a project it has not loaded; trivia, discarded above the syntax layer |
| project-load | `ra_ap_load_cargo::load_workspace_at` | **any fact about code** — it makes the other layers able to answer and answers nothing itself |
| mir | `rustc -Zunpretty=mir`, `-Zdump-mir`; or `rustc_public` | what the source looked like; original variable names, which survive only as `debug` annotations (XL002); anything about items with no body |
| dataflow | `rustc -Zdump-mir-dataflow=yes`, or the `Analysis` trait | anything MIR itself does not already carry — it derives facts over the graph, it does not add to it |
| cargo-metadata | `cargo metadata --format-version 1` | anything whatsoever about code; which features a particular build *resolved*, as opposed to which are *declared* (PL003) |

**These `cannot` rows are schema constraints, not caveats.** Three examples that change the
canonical model:

- `SourceAnchor.origin` cannot be filled from rustdoc JSON for anything inside a body, because
  there are no bodies. The column must be nullable with a `Precision::Absent` companion.
- `MirLocal` cannot carry a source name. The proposal's `MirLocal` record should hold the `debug`
  annotation as a *separate, optional* field named for what it is, not as `name`.
- `cargo_metadata` reports **declared** features. A `FeatureSelection` record sourced from it must
  be marked as declared-not-resolved, or the model will overstate what a build contained.

---

## 2. Family-by-family contribution to the canonical records

### 1. Rustdoc JSON + `rustdoc-types` → `Definition`, `Signature`, `ExportPath`, `Implementation`

Pinned at `rustdoc-types 0.61.0` (`FORMAT_VERSION = 61`). The proposal's §5 "Rustdoc JSON +
`rustdoc-types`: declared API contracts and documentation" maps cleanly.

Two hazards the plan must encode, both probe-backed:

- **`Id` is not an identity.** Probe RD004 (confirmed, with control): adding an unrelated item
  *before* a type shifts that type's `Id` from `0` to `41`, while an identical rebuild keeps it.
  An `Id` is a position in one document. It therefore belongs in `NativeBinding.native_handle`
  scoped by `run_id` — exactly as the proposal requires — and **never** as `Entity.entity_id`.
- **Absence has three indistinguishable causes** (probes RD003, RD005): not public, `cfg`-gated
  off, or in a private module. `Definition` rows sourced here need an explicit
  `visibility_basis` rather than an inferred `private` flag.

`format_version` must be read per payload, not assumed — `rustdoc-types` 0.61.0 is itself served
by docs.rs at format **60**.

### 2. `ra_ap_hir` (+ `ra_ap_ide`) → `Resolution`, `ReferenceOccurrence`, `CallSite`, `TypeObservation`

The proposal's §4 ("direct semantic extraction without IDE queries") is supported, with one
practical correction: **the resolution methods live on `SemanticsImpl`, not on `Semantics`**
(`Semantics` derefs to it). Verified entry points:

```rust
fn type_of_expr(…)                  // ra_ap_hir::semantics::SemanticsImpl
fn resolve_path(…)
fn resolve_method_call(…)
fn resolve_expr_as_callable(&self, call: &ast::Expr) -> Option<Callable<'db>>
fn resolve_attr_macro_call(&self, item: &ast::Item) -> Option<Macro>
fn resolve_derive_macro(&self, attr: &ast::Meta) -> Option<Vec<Option<Macro>>>
fn resolve_bind_pat_to_const(&self, pat: &ast::IdentPat) -> Option<ModuleDef>
fn descend_into_macros(&self, token: SyntaxToken) -> SmallVec<[SyntaxToken; 1]>
fn descend_into_macros_exact(&self, token: SyntaxToken) -> SmallVec<[SyntaxToken; 1]>
fn descend_into_macros_no_opaque(&self, token: SyntaxToken, always_descend_into_derives: bool) -> SmallVec<[InFile<SyntaxToken>; 1]>
fn descend_into_macros_breakable<T>(&self, token: InFile<SyntaxToken>, cb: …) -> Option<T>
fn descend_node_into_attributes<N: AstNode>(&self, node: N) -> SmallVec<[N; 1]>
```

The `descend_into_macros*` family — seven variants — is what the proposal's §4.2 ("resolve
occurrences, rather than asking for reference lists") needs. Note the variants differ in whether
they are *exact*, *opaque-preserving*, or *derive-descending*; the canonical `Resolution` record
must record **which variant produced it**, because they disagree by construction. That is a
`mapping_basis` value, not a footnote.

**Known limit inherited:** `ra_ap_hir` is 12.84% documented upstream. The skill vendors
`content/corpus/rust-analyzer/hir-lib.rs` for this reason; the plan's extraction adapter will be
written against source, not docs.

### 3. `ra_ap_syntax` → `SyntaxNode`, `Token`, `AttributeOccurrence`, `SourceAnchor`

331 `SyntaxKind` variants and 188 grammar nodes are catalogued in the skill
(`syntax-kinds.tsv`, `ast-nodes.tsv`), classified as node / keyword / punctuation / literal /
trivia / token-no-fixed-text.

Two probe-backed properties make this the anchor layer:

- **Parsing never fails** (SY001, confirmed): broken source yields a tree containing `ERROR`
  nodes, valid source contains none. So `SourceAnchor` can be produced for code that does not
  compile — which the other families cannot touch at all.
- **Trivia survives** (SY002, confirmed): `COMMENT` nodes are present here and nowhere above.
  The proposal's §3 "authored configuration" extraction (ast-grep rule YAML, `.gitignore`-style
  files) depends on this.

Macros stay unexpanded (SY003): the tree holds `MACRO_CALL`. Pair with family 2 or with
`rustc -Zunpretty=expanded` (probe MI007).

### 4. `ra_ap_load-cargo` + `ra_ap_project_model` → the universe, not facts

```rust
ra_ap_load_cargo::load_workspace_at
ra_ap_load_cargo::load_workspace
ra_ap_load_cargo::load_workspace_into_db
ra_ap_load_cargo::load_proc_macro
ra_ap_load_cargo::{LoadCargoConfig, ProjectFolders, SourceRootConfig, ProcMacroServerChoice}
```

`load_workspace_into_db` is the variant the plan wants — it populates a database the other
layers query, matching the proposal's "the concrete analysis universe".

This family contributes **no rows of its own** to the canonical model. It contributes
`ExtractionRun.context_id` and `scope`: which workspace, which features, which proc-macro policy
were in force. Those are the fields that make every other family's rows interpretable, and they
are the reason `ExtractionRun` is a first-class record rather than a log line.

`ProcMacroServerChoice` is decision-relevant: with proc macros disabled, family 2's resolutions
are systematically incomplete in a way nothing else records.

### 5. MIR → `MirBody`, `MirBlock`, `MirOperation`, `CfgEdge`, `MirLocal`, `Place`, `OperationUse`

The skill establishes that **no `rustc-dev` linkage is required**: `-Zunpretty=mir` and
`-Zdump-mir` produce the full operational vocabulary. 105 vocabulary entries are catalogued in
`mir-vocabulary.tsv` across 16 enums:

```
MirPhase AnalysisPhase RuntimePhase StatementKind TerminatorKind Rvalue Operand
ProjectionElem BorrowKind AggregateKind CastKind UnwindAction AssertKind
NonDivergingIntrinsic FakeReadCause CallSource
```

These are exactly the proposal's §6 "operational distinctions", already enumerated with their
doc comments — so the canonical `MirOperation.kind` vocabulary can be **generated from the
pinned rustc source** rather than hand-listed.

Two hazards, both probe-backed:

- **"The MIR" is ambiguous** (DF003): `built` → `SimplifyCfg-initial` → `PromoteTemps` →
  `analysis` → `nll` → `runtime` all exist for one body. `MirBody` must carry `MirPhase`, and the
  proposal's §6 "preserve operational distinctions" depends on it.
- **The textual view is not an interface** (MI006, confirmed): it says so in its own first line.
  The plan therefore prefers `rustc_public` (the renamed StableMIR, probe MI005) where a typed
  API is needed, and treats `-Zunpretty=mir` as a probe surface rather than an extraction format.

### 6. `rustc_mir_dataflow` → `FlowFact`, `FunctionSummary`, `EffectSummary`

The `Analysis` trait's shape is catalogued (`dataflow.tsv`, 27 rows). **Required** methods are
exactly three — `bottom_value`, `initialize_start_block`, `apply_primary_statement_effect` — with
`Domain`, `Direction` and `NAME` as associated items on `Analysis` itself.

**`AnalysisDomain` no longer exists as a separate trait and `GenKillAnalysis` was removed.** Any
design written from memory of this API will not compile.

Nine analyses ship and can be observed without writing one:

```
EverInitializedPlaces  MaybeInitializedPlaces  MaybeUninitializedPlaces
MaybeLiveLocals  MaybeTransitiveLiveLocals  MaybeBorrowedLocals
MaybeRequiresStorage  MaybeStorageDead  MaybeStorageLive
```

The proposal's §7 asks for reaching definitions, finite configuration-state propagation and
function effect summaries. None of those three is in the shipped set, so they are custom
`Analysis` implementations — which is fine, but the plan must record them as **derived** facts
with a `Derivation.rule_version`, distinct from the shipped analyses which are *observed*.

Flag attribution matters (probe DF004, confirmed): NLL region output comes from `-Zdump-mir=all`
alone; only the dataflow `.dot` files need `-Zdump-mir-dataflow=yes`.

### 7. `cargo_metadata` → `Package`, `Target`, `CrateUnit`, `DependencyEdge`, `FileMembership`

Schema version 1 (probe PL002). Two probe-backed hazards:

- `--no-deps` sets `resolve` to null (PL001, confirmed). A null `resolve` is *not* "no
  dependencies", and the proposal's §1 "What replaces Guppy" — transitive dependency facts —
  requires the full form.
- Features are **declared, not resolved** (PL003, confirmed). `cargo metadata` lists what could
  be enabled, while rustdoc JSON silently omits what was not enabled (RD005). The two are
  complementary and must not be merged into one `feature_enabled` boolean.

The proposal's replacement of Guppy with recursive queries over `DependencyEdge` is supported by
`LogicalPlan::RecursiveQuery` (evidence 02 §1).

---

## 3. What this means for the canonical schema

1. **Every typed record needs a `precision` companion**, because four of seven families have a
   documented blind spot that produces a plausible-looking null.
2. **`NativeBinding.mapping_basis` must be an enumerated vocabulary**, not free text — at minimum
   it must distinguish which `descend_into_macros` variant, which MIR phase, and whether a
   feature fact is declared or resolved.
3. **`ExtractionRun` carries the interpretive context** (workspace, features, proc-macro policy,
   MIR phase, rustdoc format version). Without it the rows are not comparable across runs.
4. Two vocabularies can be **generated from pinned upstream source** rather than authored: the
   MIR operation vocabulary (16 enums, 105 variants) and the syntax node set (331 kinds, 188
   grammar nodes). Generating them keeps the catalog honest when the toolchain moves.

## 4. Open questions for round 2

1. Whether `rustc_public` is stable enough at the pinned nightly to be the MIR extraction surface,
   or whether `-Zdump-mir` text plus a parser is the pragmatic path for round one.
2. How `ra_ap_hir` handles a workspace whose proc macros fail to build — silently incomplete
   resolutions, or an error the extraction run can record.
3. Whether the seven `descend_into_macros` variants can be reduced to two for the plan's purposes,
   or whether the choice is genuinely per-query.
