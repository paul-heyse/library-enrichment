# Plan v2 — the integrated design

**Round 2 of 2.** Round 1 ([`PLAN.md`](PLAN.md)) established what the libraries do, per subsystem,
backed by [twelve executed probes](evidence/). This document does the job round 1 deliberately
deferred: **making the subsystems into one design.** It supersedes PLAN.md §10's deferral list and
is the document to implement from. PLAN.md remains the per-subsystem capability record and is
still the place to look for *why* a library does what it does; where the two disagree, v2 wins.

Read [`HANDOFF.md`](HANDOFF.md) for the settled decisions, the pins, and the harness traps.

---

## 0. What round 2 had to decide, and what it found

The seven deferred items were not seven independent gaps. Five of them turned out to be the same
question asked in different vocabulary:

> When two facts disagree, or one is missing, what does the *combination* mean — and can that
> answer be computed by the query engine rather than by code?

`Precision` (exact / inexact / absent), interaction truth (true / false / unknown), prefilter
recall (preserving / heuristic / unknown) and extraction coverage are all **ordered lattices**.
Once they are encoded as ordered integers, composition is `min` and alternation is `max`, and
every one of them folds in SQL with no interpreter, no UDF and no procedural pass. That is §5 of
this document, and it is the load-bearing idea: it is what lets goals (1) and (4) — unified typed
data, and relational actions expressed relationally — be the *same* mechanism rather than two.

The remaining two deferred items were genuinely separate and are decided here: identity (§1) and
serving (§8).

**What changed from PLAN.md as a result of round 2, in one table:**

| PLAN.md said | v2 says | Because |
|---|---|---|
| "Validation lives in the table… no validation pass exists" | validation is **split by arity**: single-row predicates are CHECK constraints, everything else is a `projection.violations_*` view | PB07 — a constraint cannot reference another table (it panics) or a UDF (it locks out future writers) |
| `Precision` is "the epistemic spine" | `Precision` is one of **five** lattices sharing one encoding and one fold, stored as `Int8` ranks | §5; the others were latent in the proposal and unnamed. The fifth (`observed`) and the `Int8` encoding both came from executing the design — see §5.1 and §5.2 |
| `entity_id` (flat) | `entity_key` (stable, cross-snapshot) **and** `entity_id` (per-snapshot join key) | `compare` needs a key that survives a snapshot; joins need one that is unique within it |
| six merge passes | six passes with an explicit **DAG, per-pass commit, and partial-run visibility rule** | §6 |
| "is_distinct gives cycle tolerance" | withdrawn; every closure query carries a bounded `depth` | PB09 |
| one binary, serving undecided | **two binaries**, `codesearch-build` and `codesearch-query`, the latter one-shot | §8 |

---

### 0.1 Theses inherited from PLAN.md

PLAN.md states the architecture as **theses**; this document states it as **mechanisms**. That
exchange is lossy in one direction: a mechanism can be corrected by a probe without anyone noticing
that the thesis it served went with it. This table is the ledger, so a later reader can see what v2
is not allowed to have silently forgotten.

| PLAN.md | Thesis | Status in v2 |
|---|---|---|
| A.2 | `projection` is **`ViewTable`, not a function library** | **binding** — implicit in §7.4; implemented at `bridge/src/lazy.rs:191` |
| A.3 | the **one** custom `TableProvider` | **binding** — implicit in §8; `bridge/src/provider.rs`, earned by PB02 |
| A.4 | session assembly from delta-rs's `create_session()` | **binding** — §8 |
| B.1–B.5 | fixed-point column types, extension identity, `Precision` spine, nested-not-shredded | **binding** — §2 |
| C.1–C.3 | table features, validation in the table, physical layout | **binding**, C.2 refined by PB07 — §9.1, §2.3 |
| D.1 | a UDF is a **full optimizer participant** | **binding** — §10.1, with PB06's three silent preconditions |
| D.2 | the initial UDF inventory | **superseded** by §10.1: three of twelve are relations, not functions |
| D.3 | `CATALOG.md` is generated, not written | **binding** — §8 |
| **E.1** | adapters produce Arrow, not objects; each writes with **`with_input_plan`** | **binding** — §3, and the producer seam satisfies "not objects" by emitting TSV read *by DataFusion* |
| **E.2** | the six passes are **"a query over the catalog, not a procedure"** | **restored** — see §6.2. The *mechanism* was superseded by §6.4 (there is no merge pass); the *principle* was not, and `WriteBuilder::with_input_plan` satisfies it |
| E.3 | re-derivation is a **join against the CDF table**, not a diffing routine | **binding** — §6.5 |
| E.4 | graph questions are recursive CTEs | **binding** — §7.3, bounded per PB09/PB14 |
| F.1 | the five operations are five views | **binding** — §7.4 |
| F.2 | prerequisite completeness is **a join, not a policy** | **binding** — §7.4 states it verbatim, and `bridge/src/projection.rs:12-22` implements it; audited 2026-09-16, see below |
| F.3 | budget is `LIMIT` on candidates only, `omitted` from the same query | **binding** — §7.4 |
| G.1–G.4 | placeholder binding, the markup rule, renderable assignment, capture-tested output | **binding** — §11, PB08 |

**How E.2 was lost, recorded because the mechanism is general.** §6.4 discovered with a probe that
`MergeBuilder` has no `with_replace_where` at the pinned rev and concluded *"there is no merge
pass"* — correctly. But E.2's content was never only the builder; it was *"the source is a
DataFusion DataFrame … so each pass is a query over the catalog, not a procedure."* Replacing
`MergeBuilder` with `WriteBuilder` leaves that sentence true, because `with_input_plan(LogicalPlan)`
is what evidence 05:124 calls *"the direct path from a query to a table — no intermediate
materialisation."* The principle survived the mechanism's death and was buried with it, and
§3's staging layer — which says exactly this — went unbuilt for eight phases as a result.

The guard against a third instance is mechanical rather than editorial: `with_input_batches` is
permitted only at two recorded call sites, enforced by an ast-grep rule (§9.1's third tier).

**F.2 audited 2026-09-16, and it holds.** The join census over `bridge/src/projection.rs` is 12
`LEFT JOIN`, 2 `INNER JOIN`, 3 `FULL OUTER JOIN`, which reads alarming and is not. The discipline
is written once in the module doc (`projection.rs:12-22`) and every `LEFT JOIN` is optional context
by construction:

- `mechanism_detail` → `parameter` is `LEFT` and one-to-at-most-one; a mechanism with no parameter
  row is a flag that takes no argument, *"which is a fact rather than a gap"*
- `mechanism_detail` → `mechanism_ability` is `LEFT` under `coalesce(…, 0)`; zero abilities is a
  count, not an absence
- the three `catalog_record` / `bindable` / `surface_entry` joins are the coverage denominators,
  which §9.2 requires to be independent — *"neither is derived from the other, which is what makes
  the `LEFT JOIN` between them unable to drift"*

`DISCOVER` carries the mandatory case and is an `INNER JOIN`, with the reason in its own doc
comment: *"a mechanism the requested ability does not claim must not appear, and it does not,
because the row does not exist."* No projection reaches a mandatory prerequisite through a `LEFT
JOIN` or a post-filter, so no `violations_incomplete_packet` view is warranted.

**One latent case, named rather than built.** `projection.parameter` filters `retracted = false`,
so a *retracted* `required` parameter would reach `mechanism_detail` as NULL — indistinguishable
from "takes no argument", which is the F.2 failure exactly. It is unreachable today because
`retracted` is written as a literal `false` by every family, and it becomes reachable the moment
pass F writes its first retraction. Settling whether a withdrawn requirement means "no longer
required" or "requirement unknown" is a question about retraction semantics, not about joins, and
belongs with pass F (M7) rather than being guessed now.

---

## 1. Identity — one key space, three kinds of name

*Deferred item 1. Proposal §12: "One persistent identity system. Native IDs are scoped aliases;
canonical IDs are the only cross-layer join keys."*

### 1.1 Three kinds of name, and why three

Round 1 used `entity_id` for everything and PLAN.md §3.4 declared it a primary key. That is
under-specified in two directions at once, and the probes made both concrete.

| Name | Scope | Stable across | Used for |
|---|---|---|---|
| **`entity_key`** | global | snapshots, tool versions, re-extraction | `compare`; the human-legible identity; the thing a rule or doc cites |
| **`entity_id`** | one snapshot | nothing — derived | every join inside a snapshot; the declared `PrimaryKey` (PB02) |
| **`native_handle`** | one extraction run | nothing | crosswalk only, in `snapshot.native_binding` |

`entity_id = blake3(entity_key ‖ snapshot_id)`, truncated to 16 bytes, rendered lowercase hex. It
is a **function of the other two**, so it is never authored, never migrated, and never ambiguous.

Why not use `entity_key` directly as the join key? Because keys are long (a def path plus a
generic environment), they repeat in every child row, and PB02 measured that the optimiser's
payoff — an eliminated aggregation, a semi-join — comes from a declared key, not a short one. A
32-hex-character `Utf8` column is cheap to compare and cheap to Z-order.

Why not use `entity_id` alone? Because `compare` — one of the five retrieval operations — asks
"what changed between snapshot A and snapshot B", and by construction `entity_id` differs between
them for the *same* entity. `compare` joins on `entity_key`.

**Why native handles cannot be either**, measured rather than assumed: probe RD004 in the
`rust-code-model` skill showed that inserting an unrelated item *before* a type shifts that type's
rustdoc `Id` from `0` to `41`, while an identical rebuild keeps it. A rustdoc `Id` is a position
in one document. The same is true of HIR handles, `DefId`s and MIR local numbers, which do not
even share a namespace with each other. They live in `native_binding`, scoped by `run_id`, and
nowhere else.

### 1.2 The `entity_key` grammar

One grammar, one namespace prefix per kind family, so that the program side and the capability
side share a key space without colliding — which is what proposal §12's "one persistent identity
system" requires and what PLAN.md never wrote down.

```
entity_key := <namespace> ":" <scheme-specific-part>

pkg:   pkg:<registry>/<name>@<version>
       pkg:crates.io/serde@1.0.200
       pkg:path/home!workspace!crates!core          (path deps: '!' separates, never '/')

crate: crate:<pkg-key>#<target-name>/<target-kind>
       crate:pkg:crates.io/serde@1.0.200#serde/lib

file:  file:blake3:<64-hex>                          content-addressed, NOT path
       (path lives in snapshot.file_membership, many-to-one onto file:)

def:   def:<crate-key>#<relative-path>[<disambiguator>]
       def:crate:pkg:crates.io/serde@1.0.200#serde/lib#ser::Serializer
       def:...#ser::Serializer[trait]                when path alone is ambiguous across namespaces

impl:  impl:<crate-key>#<self-type-key>~<trait-key|inherent>@<anchor-key>
       (proposal §9 Pass B: "Two impls for similarly printed types should not collapse" --
        the anchor is what keeps them apart, and it is why `impl` is not a `def`)

anch:  anch:<file-key>@<start>-<end>                 byte offsets, Int64 (PB03)

ty:    ty:<structural-normal-form>                   see §4.5

mech:  mech:<tool>/<surface>/<natural-key>
       mech:rg/cli/--pcre2                            root flag
       mech:sg/cli/run/--json                         subcommand flag: the subcommand is part
                                                      of the key, because the same flag on two
                                                      subcommands can behave differently
       mech:sg/rule/ruleObject.has                    rule field, SCOPE-QUALIFIED: the shipped
                                                      schema has `has` under both `relation`
                                                      and `ruleObject`
       mech:rgx/pcre2/lookbehind
       mech:sg/rule/has@stopBy=end                   an operationally distinct configuration
                                                     (proposal §3) is its OWN mechanism

cap:   cap:<kebab-ability>
       cap:constrain-target-by-contained-syntax

plan:  plan:<kebab-fragment-name>
run:   run:<snapshot-id>/<family>/<ordinal>
```

Four rules make this work, and each answers a hazard an earlier dossier recorded:

1. **A surface-qualified mechanism key never carries a version.** Evidence 08 §4 found that flags
   move between ripgrep's help categories across releases; a key containing the category would
   churn. The version lives in `snapshot_id`, so `mech:rg/cli/--pcre2` is the same mechanism in
   every snapshot and `compare` works on it for free.
2. **An operationally distinct configuration is a distinct mechanism, not a parameter value.**
   Proposal §3 is explicit that `has` and `has` with `stopBy: end` "are meaningfully different
   choices". They get different keys. The `@k=v` suffix is reserved for exactly this and is used
   only where a `BehaviorAssertion` shows the semantics differ, never for ordinary parameters.
3. **A rule field's key carries its scope.** Measured while building step 4: **22 of the 70
   field names occur in more than one scope**. The shipped schema's scopes are `relation`,
   `ruleObject`, `ruleFile`, `rewriter`, `fix`, `labels`, `range` and the four `transform.*`
   ones; `has` appears under both `relation` and `ruleObject`, where it applies in a different
   place and takes different values. A bare field name merges two mechanisms that are not the same
   one — and licenses using one where only the other is valid. The uniqueness of `entity_id` is
   what surfaced it, which is a good argument for declaring that key even before the optimiser
   benefit.
4. **Files are content-addressed, paths are an attribute.** A file that moves is the same file; a
   file that changes is a different one. This is what makes `compare` meaningful across a refactor
   and it is why `SourceAnchor` hangs off a `file:` key rather than a path.

### 1.3 `NativeBinding` — the crosswalk, and the only place native handles appear

```
snapshot.native_binding
  binding_id          Utf8    PK   blake3(run_id ‖ native_namespace ‖ native_handle)
  run_id              Utf8    FK -> snapshot.extraction_run
  snapshot_id         Utf8
  native_namespace    Utf8         'rustdoc.id' | 'hir.def' | 'rustc.def_path_hash'
                                 | 'mir.local' | 'syntax.ptr' | 'cargo.pkgid' | 'sg.rule' | 'rg.flag'
  native_handle       Utf8         opaque; meaningful only with run_id
  canonical_entity_id Utf8         FK -> snapshot.entity, NULLABLE
  canonical_entity_key Utf8        denormalised for cross-snapshot crosswalk, NULLABLE
  mapping_status      Utf8         lattice, §5.2: 'bound'|'candidate'|'ambiguous'|'unbound'
  mapping_basis       Utf8         enumerated, §3 per family
  mapping_precision   Utf8         lattice: 'exact'|'inexact'|'absent'
  ord                 Int32        candidate rank when mapping_status='candidate'
```

`canonical_entity_id` is **nullable on purpose**. Proposal §9 Pass C: *"Uncertain identity matches
remain candidates. Do not merge first and attempt to recover distinctions later."* An unbound
native handle is a recorded fact — it is how extraction coverage is measured (§9.2) — not a
failure to be cleaned up.

`mapping_basis` is an enumerated vocabulary and never free text. Evidence 06 §3 is emphatic about
why: the seven `descend_into_macros` variants disagree *by construction*, so a `Resolution` that
does not say which one produced it is not interpretable. The full vocabulary is in §3.

### 1.4 Identity as an Arrow extension type

PB01 measured that Arrow extension metadata survives a Delta write/read cycle byte-for-byte, so
identity is a **storage property** and needs no re-attachment step (PLAN.md §3.2 already removed
that step). v2 fixes the concrete registrations:

| Extension name | Storage type | Carried on |
|---|---|---|
| `codesearch.entity_id` | `Utf8` | every `entity_id` and every FK to one |
| `codesearch.entity_key` | `Utf8` | every `entity_key` |
| `codesearch.lattice` | `Int8` | every `*_rank` column — all five lattices (§5) |
| `codesearch.byte_offset` | `Int64` | every offset — **never `UInt32`** (PB03) |

The extension name is what makes a mis-join detectable: joining `codesearch.entity_key` to
`codesearch.entity_id` is a type error the `bridge::rules::IdentityDiscipline` analyzer rule
rejects at plan time, before it silently returns zero rows.

---

## 2. Type discipline — the schema rules that come from the probes

*These apply to every column in §3 and §4 and are checked mechanically (§10.2).*

### 2.1 Every column type is a fixed point of the Delta conversion

PB03's decisive finding: Delta **does not reject types, it silently narrows them**. Ten of eleven
"unsupported" Arrow types are accepted and mapped. So the schema is authored in the *narrowed*
vocabulary, and a build-time assertion re-reads each table's Arrow schema and requires equality
with what was authored.

| Never author | Author instead | What the write path would otherwise do |
|---|---|---|
| `UInt32`, `UInt64` | `Int64` | narrow to `Int32`/`Int64` — **`UInt32` truncates above `i32::MAX`**, silently corrupting offsets in files over 2 GiB |
| `Dictionary(_, Utf8)` | `Utf8` | flatten to `Utf8`; authoring a dictionary buys nothing and hides the real stored type |
| `Utf8View`, `LargeUtf8` | `Utf8` | narrow to `Utf8` |
| `LargeList<T>` | `List<T>` | narrow to `List<T>` |
| `FixedSizeBinary(n)` | `Binary` | narrow to `Binary` |
| `Float16` | `Float32` | **rejected** — the only genuine rejection |

And the naming rule: nested child fields are **renamed** to Parquet conventions on write
(`item`→`element`, `entries/keys/values`→`key_value/key/value`). v2 authors the Parquet spellings
directly, so an authored schema and a read-back schema compare equal without a normalisation step.

**`normalize_for_delta` is not a preview of persistence** — PB03 measured it returning all twenty
inputs unchanged, including every type the write path then narrows. It must not be used as a
pre-flight check. The pre-flight check is a round trip.

### 2.2 The five column groups every canonical table carries

Uniformity here is what makes §5's folds possible and what lets one `bridge` rule check every
table. Every row in `snapshot.*`, `program.*`, `catalog.*` and `evidence.*` carries:

```
identity     entity_id  entity_key  snapshot_id            (+ owner_id where the record has a parent)
provenance   run_id  evidence_id                           FK into evidence.evidence
epistemics   precision_rank  coverage_status  observed_rank     Int8 lattices, §5
ordering     ord                     Int32, NULL unless the record is a child in an ordered sequence
lifecycle    first_seen_version  last_seen_version  retracted
```

`ord` is not decoration. Proposal §8.2: *"Use ordered child records for arguments, generic
substitutions, projections, and syntax children"*, and §3: *"Preserve ordered rule sequences and
binding dependencies rather than assuming ordinary commutative Boolean semantics."* A child
sequence without `ord` has silently asserted commutativity.

`retracted` + `last_seen_version` implement the proposal's retraction requirement using the merge
clauses PLAN.md §6.2 already chose (`when_not_matched_by_source_update`): a fact a re-run no
longer reports is **marked**, never deleted, so `compare` can say "this disappeared" rather than
returning nothing.

### 2.3 Partitioning and clustering, fixed by PB10

PB10 measured that the change feed narrows **only** on partition columns — a partition filter cut
three files to two, while a data-column filter read all three. Since incremental re-derivation
(§6.5) reads the change feed filtered by extraction family, `family` must be a partition column on
every table CDF is read from, not merely an attribute.

```
staging.*                    partition (family, snapshot_id)      cluster entity_key
snapshot.entity              partition (kind)                     cluster entity_id
snapshot.native_binding      partition (native_namespace)         cluster canonical_entity_id
snapshot.extraction_run      partition (family)                   cluster run_id
snapshot.source_anchor       —                                    cluster file_key
program.*                    partition (family, record_kind)      cluster owner_id, entity_id
evidence.evidence            partition (observation_kind)         cluster entity_id
evidence.derivation          partition (rule_id)                  cluster derivation_id
evidence.behavior_assertion  partition (subject_kind)             cluster subject_entity_id
catalog.*                    —                                    small; cluster entity_id
```

`OptimizeType::ZOrder` on `(entity_id, entity_key)` after each build. Z-ordering on the two join
keys is what keeps the identity joins cheap once the merge passes have fragmented the files.

---

## 3. The staging→canonical contract, per family

*Deferred item 2 — the largest. PLAN.md §6.1 gave the rules; these are the column lists.*

Every family has exactly one staging table, written whole by one adapter with
`WriteBuilder::with_input_plan` + `with_replace_where("run_id = '<this run>'")` (§6.4 -- the
`family AND snapshot_id` form this used to give is unwritable: `family` is a column of
`snapshot.extraction_run` and of no canonical table), and
exactly one contract to the canonical model. The contract has four parts, and the fourth is the
one that keeps the model honest:

1. **produces** — canonical tables this family may write
2. **key rule** — how `entity_key` is minted from native facts
3. **`mapping_basis` vocabulary** — the closed set this family may emit
4. **never fills** — columns this family must leave NULL, from evidence 06's `cannot_answer`

Part 4 is a build-time assertion, not a comment. If an adapter writes a column its family cannot
know, the build fails. This is proposal §12's "No silent semantic flattening" made mechanical, and
it is the direct answer to evidence 06's warning that four of seven families have blind spots
producing *plausible-looking nulls*.


#### How a staging table is read, fixed by PB19

The producer seam is the filesystem: an adapter writes headerless TSVs and `codesearch-build` reads
them **through DataFusion**, never through a hand-rolled parser. `IndexSpec { file, columns, .. }`
is already the schema — a filename and an ordered list of column names, every one `Utf8`.

```rust
CsvReadOptions::new()
    .has_header(false)
    .delimiter(b'\t')
    .file_extension(".tsv")   // default is ".csv"; it rejects every input otherwise
    .quote(b'\0')             // default is '"'; a cell BEGINNING with one loses it, silently
    .schema(&schema)          // borrows — the Schema must outlive the options
```

Every clause above is a PB19 arm (evidence 18), and two of them are load-bearing:

- **`truncated_rows` stays at its default `false`**, and that is the arity guard. A row whose field
  count disagrees with the schema fails the read with the line number and both counts — strictly
  louder than the per-row check it replaces. Nothing needs to be rebuilt in Rust to keep it.
- **`quote(b'\0')` is not optional.** The skill's contract (`index.rs:5-8`) says there is no
  quoting; the reader assumes RFC4180 unless told. Three cells of `behaviors.tsv` begin with `"`
  and lose it under the default, with no error — the exact failure mode `index.rs:12-14` names.

Where a family's staging spans many files, `ListingTableUrl` reads a directory (the path must end
in `/`) and the scalar functions `input_file_name()` / `file_row_index()` supply `file_key` and
`ord`, so an adapter never invents either.

### 3.1 `cargo-metadata` → package, target, dependency

```
staging.cargo_metadata
  row_kind        Utf8    'package'|'target'|'dep_edge'|'feature_decl'|'workspace_member'
  pkg_name        Utf8        pkg_version      Utf8       pkg_source       Utf8   (NULL for path deps)
  pkg_manifest    Utf8        target_name      Utf8       target_kind      Utf8
  target_src_path Utf8        dep_name         Utf8       dep_req          Utf8
  dep_kind        Utf8    'normal'|'dev'|'build'          dep_optional     Boolean
  dep_target_cfg  Utf8    NULL when unconditional         feature_name     Utf8
  feature_implies List<Utf8>  resolve_present  Boolean    metadata_version Int64
```

- **produces** `program.package`, `program.target`, `program.crate_unit`,
  `program.dependency_edge`, `program.feature_declaration`
- **key rule** `pkg:<source|path>/<name>@<version>`; never by name alone (proposal §9 Pass A)
- **`mapping_basis`** `cargo.metadata.v1`
- **never fills** anything in `program.definition`, `program.mir_*`, `program.resolution`,
  `source_anchor.*`. Evidence 06: this family answers *nothing whatsoever about code*.

**Two hazards become columns.** `resolve_present` exists because probe PL001 confirmed `--no-deps`
sets `resolve` to null, and *a null `resolve` is not "no dependencies"* — without this column the
transitive-closure query (§7.3) would silently return an empty graph. And features are recorded as
`program.feature_declaration`, never as a `feature_enabled` boolean: probe PL003 confirmed
`cargo metadata` reports what *could* be enabled, while rustdoc JSON silently omits what was not
(RD005). Merging them into one boolean would assert something neither family knows.

### 3.2 `project-load` → no rows, all the context

```
staging.project_load
  workspace_root       Utf8       loaded_crate_count     Int64
  cfg_overrides        List<Struct{key:Utf8, value:Utf8}>
  features_requested   List<Utf8> proc_macro_policy      Utf8   'enabled'|'disabled'|'server_failed'
  proc_macro_failures  List<Utf8> sysroot_kind           Utf8
  rustc_version        Utf8       load_status            Utf8
```

- **produces** no `program.*` rows at all — it writes `snapshot.extraction_run.context_id` and
  `.scope`, which every other family's rows are then interpreted against
- **never fills** every `program.*` table

This family is the reason `ExtractionRun` is a first-class record rather than a log line. Evidence
06 singles out `ProcMacroServerChoice` as decision-relevant: **with proc macros disabled, family
3.4's resolutions are systematically incomplete in a way nothing else records.** So
`proc_macro_policy` is not diagnostics — it is a precondition on the interpretation of every
`Resolution` row in the snapshot, and §9.2 counts coverage against it.

### 3.3 `syntax` → the anchor layer

```
staging.syntax
  file_key        Utf8        node_kind       Utf8    one of 331 SyntaxKind
  parent_ord      Int32       start_byte      Int64   end_byte        Int64      (Int64: PB03)
  is_error        Boolean     is_trivia       Boolean text_if_token   Utf8
  macro_call_unexpanded Boolean
```

- **produces** `program.syntax_node`, `program.token`, `program.attribute_occurrence`,
  `program.literal_occurrence`, `snapshot.source_anchor`
- **key rule** `anch:<file-key>@<start>-<end>`
- **`mapping_basis`** `syntax.parse`
- **never fills** any type or resolution column; `macro_expansion_of`

**This is the only family that works on code that does not compile** (probe SY001, with control:
broken source yields `ERROR` nodes, valid source contains none). That makes it the anchor layer,
and it is why `source_anchor` rows can exist for entities no other family can see. **Trivia
survives only here** (SY002) — `COMMENT` nodes exist at this layer and nowhere above — which is
what the proposal's authored-configuration extraction (ast-grep rule YAML) depends on.

### 3.4 `hir` → resolution and type observation

```
staging.hir
  anchor_key      Utf8        occurrence_kind Utf8   'path'|'method_call'|'callable'|'attr_macro'
                                                    |'derive_macro'|'bind_pat_const'
  resolved_def_handle  Utf8   resolution_kind Utf8
  descend_variant Utf8        THE mapping_basis; one of seven
  ty_display      Utf8        ty_kind         Utf8   ty_stage  Utf8  'inferred'|'adjusted'
  callable_kind   Utf8        is_unresolved   Boolean
```

- **produces** `program.resolution`, `program.reference_occurrence`, `program.call_site`,
  `program.call_target`, `program.type_observation`
- **key rule** resolutions bind to an existing `def:` key via `native_binding`; they never mint one
- **`mapping_basis`** exactly one of
  `hir.descend.plain` · `.exact` · `.no_opaque` · `.no_opaque_derives` · `.breakable` ·
  `.node_attributes` · `hir.direct` (no descent)
- **never fills** any MIR column; trivia; `source_text`

`descend_variant` is a **required** column, not an optional one. Evidence 06 §2: the seven
variants "disagree by construction", so a `Resolution` that does not record which produced it is
uninterpretable. Default is `hir.descend.exact`; `hir.descend.no_opaque` is used for
attribute-heavy input. The choice is per-extraction and recorded, never global and assumed.

Note the API correction evidence 06 carries: the resolution methods live on `SemanticsImpl`, not
`Semantics` (which derefs to it), and `ra_ap_hir` is 12.84% documented upstream — the adapter is
written against the vendored source in the `rust-code-model` skill's
`content/corpus/rust-analyzer/hir-lib.rs`, not against docs.

### 3.5 `rustdoc-json` → declared API only

```
staging.rustdoc
  rustdoc_id      Utf8        item_kind       Utf8       path_segments  List<Utf8>
  visibility      Utf8        visibility_basis Utf8  'public'|'crate'|'private_module'|'cfg_excluded'
  signature_text  Utf8        generics_text   Utf8       trait_ref      Utf8
  self_ty         Utf8        docs            Utf8       format_version Int64
  span_file       Utf8        span_start      Int64      span_end       Int64     (nullable)
```

- **produces** `program.definition`, `program.signature`, `program.export_path`,
  `program.implementation`, `program.generic_parameter`, `program.predicate`
- **key rule** `def:<crate-key>#<path-segments joined>`; **`rustdoc_id` is never a key** — it goes
  to `native_binding` under `rustdoc.id`
- **`mapping_basis`** `rustdoc.json.v<format_version>`
- **never fills** anything inside a body: `call_site`, `mir_*`, `flow_fact`, and `source_anchor`
  for body-internal positions

Two hazards become columns, both probe-backed. **`Id` is not an identity** — probe RD004 showed an
unrelated insertion shifts a type's `Id` from 0 to 41 while an identical rebuild keeps it, which is
why any design that caches an `Id` as a key is wrong. And **absence has three indistinguishable
causes** (RD003, RD005): not public, `cfg`-gated off, or in a private module — hence
`visibility_basis` as an explicit column rather than an inferred `private` flag.

`format_version` is read **per payload**, never assumed: `rustdoc-types` 0.61.0, the crate that
defines `FORMAT_VERSION = 61`, is served by docs.rs at format **60**.

**Built, from the skill's shipped index rather than a live rustdoc run** — so `mapping_basis` is
`skill.index.symbols`, never `rustdoc.json.v<n>`, precisely because a TSV carries no
`format_version` to read. Three key rules degraded, each recorded rather than papered over:

- `pkg:` loses its source segment: `pins.crates` is a `name -> version` object.
- `impl:` loses §1.2's anchor: `impls.tsv` has no byte offsets. The builder detects collisions and
  fails; `precision` is `inexact` on every implementation row, because "no collision observed
  across 2,241 rows" is not "cannot collide".
- **A signature key needs the signature text.** `methods.tsv` records the trait PATH, not the trait
  REFERENCE, so `TryFrom<u16>` and `TryFrom<u64>` both arrive as `core::convert::TryFrom` — 34
  owner/method/trait triples across 126 rows. This is §4.2's rule one level up: two impls of one
  trait at different type arguments are not one impl, just as two parameters named `T` are not one
  type variable.

`visibility` and `visibility_basis` are in `program.definition` and **stay NULL**: an index of
public items records which of not-public / `cfg`-gated / private-module applies to exactly nothing.
`Columns::never_fills` makes writing them a build error, which is what §3 part 4 asked for and what
nothing previously enforced.

### 3.6 `mir` → operational structure

```
staging.mir
  body_owner_handle Utf8      mir_phase       Utf8  'built'|'analysis'|'runtime'  REQUIRED
  block_index     Int64       stmt_index      Int64      stmt_kind       Utf8
  terminator_kind Utf8        rvalue_kind     Utf8       operand_kind    Utf8
  local_index     Int64       local_ty        Utf8       debug_annotation Utf8   NULLABLE
  place_projection List<Utf8> edge_target     Int64      edge_kind       Utf8
```

- **produces** `program.mir_body`, `program.mir_block`, `program.mir_operation`,
  `program.cfg_edge`, `program.mir_local`, `program.place`, `program.operation_use`
- **key rule** `def:<owner>#mir@<phase>` for the body; children by `(body, block, stmt)` ordinals
- **`mapping_basis`** `mir.rustc_public@runtime` — **decided by PB15** (evidence 14)
- **never fills** `source_text`, `mir_local.name`, anything for a body-less item

**`mir_phase` is required because "the MIR" is ambiguous** (probe DF003): `built` →
`SimplifyCfg-initial` → `PromoteTemps` → `analysis` → `nll` → `runtime` all exist for one body. A
`MirBody` without a phase is three different bodies wearing one name.

**There is no `name` column on `mir_local`** — evidence 06: original variable names survive only as
`debug` annotations (probe XL002), so the column is `debug_annotation`, named for what it is.
Calling it `name` would invite exactly the join the proposal's §12 forbids.

The 105 operation-vocabulary entries across 16 enums (`StatementKind`, `TerminatorKind`, `Rvalue`,
`ProjectionElem`, …) are **generated from the pinned rustc source**, not hand-listed, so the
vocabulary CHECK constraints stay true when the toolchain moves.

The textual view is a probe surface, not an extraction format — it says so in its own first line
(probe MI006). **PB15 measured the choice rather than arguing it** (evidence 14): `rustc_public`
is reachable at the pinned nightly, a body walks into the column shape above including
`cfg_edge`, and the textual surface agrees with it on every fixture function. The typed API
yields structured terminators and successor kinds directly where the text needs a parser for the
grammar of every terminator to recover the same `edge_kind`, and both see exactly the same body —
so there is no accuracy argument for the parser and it is not worth writing.

**Only `'runtime'` is reachable, and that is the part worth remembering.** PB15 dumped one
function at every phase: `built` 14 blocks, `analysis` 8, `runtime-optimized` 6. Both surfaces
reported 6. `rustc_public::CrateItem::body()` goes through `tcx.instance_mir`, which is
`optimized_mir` for an ordinary item, and the public API takes no phase parameter;
`-Zunpretty=mir` reads the same query. So `mir_phase`'s vocabulary keeps all three values — the
14/8/6 spread is the strongest argument yet for the column existing at all — but a `built` or
`analysis` row requires the `-Zdump-mir` surface, which has a different shape (one file per pass,
phase in the filename) and therefore a different adapter. **No such row should be written until
that adapter exists**; a `mir_phase` of `'built'` written by the runtime surface would be the
silent mislabelling the column was added to prevent.

The vocabulary generator must read **`rustc_public`'s** enums, not `rustc_middle`'s. PB15 found
that `rustc_public::mir::StatementKind` has eleven variants where the internal one has twelve.
Generating from the internal enum would put a variant into a CHECK constraint that the extraction
surface can never produce — a constraint that cannot be violated, which is the same family of
mistake as a probe that cannot fail.

`mir_local` keeps `debug_annotation` and gains a reason at the type level rather than only in
prose: `rustc_public::mir::LocalDecl` is `{ ty, span, mutability }`, with no `name` field to be
tempted by.

### 3.7 `dataflow` → derived, never observed

```
staging.dataflow
  body_key        Utf8        analysis_name   Utf8       analysis_origin Utf8  'shipped'|'custom'
  direction       Utf8        domain_kind     Utf8       block_index     Int64
  stmt_index      Int64       fact_kind       Utf8       place_key       Utf8
  state_bits      Binary      rule_version    Utf8       REQUIRED when origin='custom'
```

- **produces** `evidence.flow_fact`, `evidence.function_summary`, `evidence.effect_summary`
- **`mapping_basis`** `dataflow.shipped.<name>` | `dataflow.custom.<name>@<rule_version>`
- **never fills** anything MIR does not already carry — this family derives over the graph, it
  does not add to it

The nine shipped analyses (`MaybeInitializedPlaces`, `MaybeLiveLocals`, `MaybeBorrowedLocals`, …)
are **observed**. The proposal's §7 asks for three analyses that are *not* in the shipped set —
reaching definitions, finite configuration-state propagation, function effect summaries — so those
are custom `Analysis` implementations and are recorded as **derived** facts carrying a
`Derivation.rule_version`. The `analysis_origin` column is what keeps the two apart, and it is the
difference between "the compiler says so" and "our rule concludes so".

Two API facts the adapter must respect: **`AnalysisDomain` no longer exists as a separate trait and
`GenKillAnalysis` was removed** — required methods are exactly `bottom_value`,
`initialize_start_block`, `apply_primary_statement_effect`. And flag attribution matters (probe
DF004): NLL region output comes from `-Zdump-mir=all` alone; only the dataflow `.dot` files need
the dataflow flag.

### 3.8 `catalog` → the ast-grep / ripgrep / PCRE2 surface

The eighth family, which PLAN.md treated separately and v2 folds into the same contract shape so
that one set of rules governs all extraction. Sources are the existing skill's own indexes
(evidence 08).

```
staging.catalog
  source_table    Utf8    'flags'|'rule-fields'|'regex'|'file-types'|'exit-codes'|'behaviors'
                        |'languages'|'unreachable'
  tool            Utf8    surface         Utf8    natural_key     Utf8
  ...column-per-source-table, nullable...
```

- **produces** `catalog.capability`, `catalog.mechanism`, `catalog.parameter`,
  `catalog.result_contract`, `catalog.search_domain`, `catalog.surface_entry`,
  `evidence.behavior_assertion`
- **key rule** `mech:<tool>/<surface>/<natural-key>` per §1.2
- **never fills** any `program.*` table — the *implementation* tables (`symbols`, `methods`,
  `impls`, `aliases`) feed `program.definition` through the **rustdoc-json** family instead, since
  that is what produced them

The join between the two halves of the system is `catalog.surface_binding`, with the proposal's
role vocabulary (`declared_by`, `parsed_by`, `validated_by`, `stored_in`, `normalized_by`,
`configured_through`, `implemented_by`, `reported_by`). That table is the entire answer to "how do
the capability catalog and the Rust program model meet", and because both sides mint keys in the
same `entity_key` grammar (§1.2), it is an ordinary join rather than a bridge.

**That was true of the design and false of the table until phase 5.** `surface_binding` held
`capability_key` / `mechanism_key` with `role` constant at `implemented_by` — it could not express
`mechanism -> def:` at all. It is now `subject_key` / `subject_kind` / `object_key` / `object_kind`
/ `role`, the same generalisation `surface_entry` went through when `mechanism_key` became
`catalog_key` + `catalog_kind`, and `projection.bindable` is the domain both ends are checked
against.

**Nothing shipped states the relationship, so these rows are authored.** `bindings.tsv` is
ast-grep's napi/pyo3 language bindings, not a crosswalk, and its `role` column is uniformly `item`.
A derived link — joining on crate-name similarity — was considered and rejected: it would cover
many rows while asserting a relationship nothing measured. Fourteen authored bindings exist, each
carrying its evidence and its own lattice placement: a crate the skill's own `topics/embedding.md`
names is `recorded` at `exact`, a builder type inferred from its name is `not-probed` at `inexact`.
`coverage --dimension api` reports seven of 323 mechanisms bound, which is the honest number and
the reason the dimension exists.

The 49 existing behaviour probes carry a control and an exit code already, so they become
`evidence.behavior_assertion` rows with `supporting_evidence` populated — not re-derived. Evidence
08 §6 asked whether they survive the re-typing without loss; they do, because the assertion schema
has a slot for every column the TSV carries. `kinds` (3,024 rows) is **referenced, not loaded**: it
is target-language grammar metadata that changes on every ast-grep grammar update, so it stays in
the skill and `catalog.grammar_fragment` holds a pointer plus the language roster.

---

## 4. The canonical tables

*Every table carries the five column groups of §2.2; only the distinguishing columns are listed.*

### 4.1 `snapshot` — provenance and identity

```
artifact          artifact_kind · content_hash · retained_location · byte_len(Int64)
extraction_run    family · extractor_identity · context_id · scope(Struct) ·
                  completion_status · started_at · finished_at · input_artifact_ids(List<Utf8>)
entity            kind · owner_id · display_name
native_binding    §1.3
source_anchor     file_key · native_range(Struct{start:Int64,end:Int64}) ·
                  normalized_byte_range(Struct, nullable) · origin · expansion_parent(nullable)
file_membership   -- moved to program.*, see §4.2. A file's membership of a crate unit is a fact
                  about the program, not about the snapshot, and listing it here as well let one
                  table appear to belong to two schemas.
```

`extraction_run.scope` is a `Struct`, not a JSON string: it holds the workspace root, the requested
features, the proc-macro policy and the toolchain identity, so that a query can *filter* on
interpretive context rather than parse it. `completion_status` is a lattice (§5.2) and is what §6.3
uses to keep partial runs out of retrieval by default.

### 4.2 `program` — the typed program records

Grouped exactly as proposal §8.2 groups them, one table each:

```
package · target · crate_unit · dependency_edge · file_membership · feature_declaration
definition · export_path · field · variant · implementation · generic_parameter ·
  predicate · signature
type_term · type_argument · type_observation
syntax_node · token · attribute_occurrence · literal_occurrence
reference_occurrence · resolution · call_site · call_target
mir_body · mir_block · mir_operation · cfg_edge · mir_local · place · operation_use
```

Two shapes need calling out because they encode a proposal requirement that is easy to lose:

**`generic_parameter` identity includes the binder and the position.** Proposal §8.2: *"Two
parameters named `T` are not one global type variable."* So the key is
`def:<owner>#generic[<binder-kind>:<index>]`, never the parameter's name. A schema keyed on name
would silently unify every `T` in the crate.

**`implementation` carries its own identity, trait ref, self type, generic environment and
predicates** as columns, per proposal §9 Pass B. `impl` keys embed an anchor (§1.2) precisely so
that two impls for similarly printed types cannot collapse.

### 4.3 `catalog` — the capability model

```
capability        ability · facet_profile(Struct per proposal §2 "composable facets")
mechanism         surface · tool · invocation_form · native_schema_ref · capability_ids(List)
parameter         mechanism_id · name · value_domain · declared_default · application_override ·
                  effective_default · ord
interaction       §7.1
result_contract   output_mode · unit · shape(Struct) · exit_code_semantics
search_domain     traversal · ignore_layers(List) · binary_policy · encoding_policy
plan_fragment     §7.2
grammar_fragment  language · pointer_to_skill_index · rule_schema_ref
surface_entry     the upstream inventory item — the coverage DENOMINATOR (§9.2)
surface_binding   surface_entry_id · implementation_entity_id · role · evidence_id
mechanism_effect  mechanism_id · condition_id · effect · affected_setting · assertion_ids(List)
```

`parameter` carries **three default columns, not one**, because proposal §3 requires it: the
declared library default, the application override, and the effective behaviour on the selected
surface. The worked example is `grep-pcre2`, whose builder documents UTF and Unicode-property modes
as *disabled* by default while ripgrep *enables* both unless Unicode is disabled. An agent asking
"what is the default" needs the third column; collapsing them would make the catalog confidently
wrong on exactly the question it exists to answer.

`surface_entry` is separate from `mechanism` for one reason: it is the denominator. It is the
inventory of what the tool exposes, discovered upstream; `mechanism` is what has been
*characterised*. The `LEFT JOIN` between them is coverage (§9.2), and it cannot drift because
neither side is derived from the other.

### 4.4 `evidence`

```
evidence            extraction_run_id · native_locator · source_anchor_id(nullable) ·
                    observation_kind
derivation          rule_id · rule_version · input_fact_ids(List<Utf8>) ·
                    assumptions(List<Struct>) · output_fact_ids(List<Utf8>)
behavior_assertion  subject_entity_id · subject_kind · predicate · value_or_target ·
                    applicability_condition_id · interpretation · status ·
                    supporting_evidence(List) · contradicting_evidence(List)
flow_fact · function_summary · effect_summary       §3.7
coverage            surface_id · characterised · assertion_count · coverage_status
```

`behavior_assertion` holds **both** `supporting_evidence` and `contradicting_evidence`, per
proposal §8.3. A contradicted assertion is not deleted — it is an assertion whose status the
lattice in §5 will not let rise above `contested`. That is how "silence is not absence" is
represented for behaviour rather than merely stated.

### 4.5 Type normalisation — Pass E, and why it is not a winner-take-all

`type_term` is shared when structural identity **and** generic context agree; `ty:` keys are minted
from a structural normal form, so two spellings of the same type unify and two same-spelled types
in different generic environments do not.

`type_observation` is where the proposal's hard constraint lives. It records **five distinct
stages** for what may be the same expression:

```
written · documented_declaration · hir_inferred · hir_adjusted · mir_local_or_operand
```

Proposal §9 Pass E: *"Different observations may legitimately describe different stages of the same
expression. No global rule should declare one of them the winner."* So `type_observation` is a
table with a `stage` column and no uniqueness constraint across stages, and there is deliberately
**no** `program.resolved_type` view that picks one. A caller asking for "the type" must say which
stage, and the CLI's `describe` surfaces all five.

---

## 5. One lattice discipline

*The integration idea. Four epistemic dimensions that the proposal treats separately turn out to
have identical algebra, so they get one encoding, one set of folds, and one propagation rule.*

### 5.1 The lattices — five by design, six in the build

| Lattice | Values, lowest to highest | Asks |
|---|---|---|
| `precision` | `absent` → `inexact` → `exact` | how well is this fact known? |
| `truth` | `false` → `unknown` → `true` | does this condition hold? (Kleene) |
| `recall` | `unknown` → `heuristic` → `preserving` | does this step keep everything it should? |
| `binding` | `unbound` → `ambiguous` → `candidate` → `bound` | is this native handle mapped? |
| `observed` | `not-probed` → `unknown` → `recorded` → `confirmed` | was this checked, and how? |
| `completion` | `abandoned` → `running` → `partial` → `complete` | did the run that wrote this finish? |

The fifth was **not** in the round-2 draft. It was found on first contact with the real data: the
ast-grep-ripgrep skill's `regex.tsv.observed` and `behaviors.tsv.verdict` already carry exactly
this vocabulary, and the skill's own `reference.md` limit #3 insists that *"`unknown` is a real
value… Nothing should be inferred from them in either direction."* That a fifth dimension appeared
and needed no new machinery is the strongest evidence the generalisation was right.

`not-probed` and `unknown` are **different facts** — nothing was attempted, versus something was
attempted and did not settle. A consumer that maps either to `false` destroys the property this
catalog exists to provide.

**Stored as `Int8` rank, not as the label.** See §5.2 — this is the one place the design says
something other than "author what you read back".

### 5.2 The rank IS the stored value, so the fold is plain SQL

Every lattice column is stored as **`Int8`, holding the ordinal** — `precision_rank`, not
`precision`. Labels are rendered at the projection edge by `lattice_label(name, rank)` and the
mapping lives in the `catalog.lattice` reference table (`name · rank · label`), so it is queryable
data rather than code.

**Why not store the label, which would be more legible in raw Parquet?** Because rank order and
lexicographic order are different orders, and a design that stored labels would depend on their
spelling by accident:

```
precision by rank:  absent(0) < inexact(1) < exact(2)
precision by bytes: absent    < exact      < inexact
```

A filter like "precision at least `inexact`" is a contiguous range over ranks and *not* reliably
one over bytes. It happens to come out contiguous for all five current value sets — but renaming
`exact` to `certain`, or adding a value, would silently stop it pruning, with no error and no
change to the plan. That is exactly the failure mode PB06 exists to catch, so the design does not
rely on the coincidence.

Storing the ordinal makes every lattice filter a native integer comparison that prunes on Delta
min/max statistics with **no UDF and no `preimage` at all**. `Int8` maps to Delta `BYTE` and back
(`kernel/src/engine/arrow_conversion/mod.rs:348,558`), so it is a fixed point per §2.1; the
round-trip check confirms it rather than assuming it.

The CHECK constraint is correspondingly simpler and still single-row plain column algebra (§9.1):
`precision_rank BETWEEN 0 AND 2`.

Then, for every one of the five:

```
composition  (a chain is as good as its weakest link)     min(rank)
alternation  (a choice is as good as its best option)     max(rank)
```

That is the whole algebra. `AND` over Kleene truth is `min`; `OR` is `max`; a derivation's
precision is the `min` of its inputs' precisions; a plan fragment chain's recall is the `min` of
its steps' recalls; a set of candidate bindings resolves to the `max`. **Five problems, one line
of standard SQL:**

```sql
SELECT parent_id, min(precision_rank) AS precision_rank
FROM   child_facts GROUP BY parent_id
```

No interpreter, no procedural pass, no per-dimension code — and no UDF in the fold either. This is
goal (4), relational things done relationally, reaching the epistemics rather than only the data.

`lattice_label` is the only lattice function, it is `Volatility::Immutable`, and it appears in
projections rather than in predicates. Nothing in this family needs `preimage`, because nothing in
this family is ever applied to a column before a comparison.

### 5.3 Propagation is a rule, not a convention

`bridge::rules::LatticePropagation` is an `AnalyzerRule`. Any projection that joins a parent to
children and does not carry a lattice fold for each of the five dimensions is **rejected at plan
time**. The alternative — relying on every query author to remember — is exactly the silent
degradation the proposal's §12 forbids, and PB06 showed how comfortable a silently-wrong result
looks.

### 5.4 The one thing the lattices must not do

They must not be *combined with each other*. A fact that is `precision=exact` and
`recall=heuristic` is not "medium". The five dimensions stay five columns; there is no scalar
quality score, and §9.2's coverage reporting deliberately produces two independent numbers rather
than a percentage. PLAN.md's coverage section already refused to "convert gate results into a
quality percentage"; §5.4 is the same refusal applied to the data model.

---

## 6. Execution model

*Deferred item 3. What runs in what order, and what a partial failure leaves behind.*

### 6.1 The unit of work is a run, and it is visible before it succeeds

An extraction begins by committing a `snapshot.extraction_run` row with
`completion_status='running'`. Every row the run writes carries its `run_id`. On success the row
becomes `complete`; on failure, `partial` with the failing pass recorded; a process that dies
leaves `running`, which a later build reaps to `abandoned`.

This ordering matters: **the run row is written first**, so a crashed run leaves attributable rows
rather than orphans. The alternative — write rows, then record the run — makes an interrupted build
indistinguishable from a successful one that found nothing.

**Built, and measured end to end.** `snapshot.extraction_run` exists, `completion` is the sixth
lattice, and the whole lifecycle was exercised by killing a build mid-write: the crashed run was
left at `running`, the next build reaped it to `abandoned` while leaving a `complete` run
untouched, and the crashed run's rows were absent from `discover` until `--include-partial` was
passed, whereupon exactly they came back.

Three details the implementation settled:

- **Reaping is not time-based.** A heartbeat and a timeout would make the verdict depend on a clock
  skew nobody measured. "A different run started, and this one had not finished" is decidable
  without one.
- **The run key is `run:<snapshot>/<family>/0`, stable across rebuilds of the same snapshot**, so a
  rebuild *supersedes* rather than accumulating. That is what keeps `just catalog` idempotent — a
  documented property — and it is also the right semantics: re-running the same build is not a
  second run to be abandoned, it is the same one being redone. Reaping therefore fires between
  runs that genuinely differ, which after §6.4 means between families.
- **`scope` is a real `Struct`**, measured as a fixed point of the Delta conversion along with
  `Timestamp(Microsecond, "UTC")`. Both were new types for this schema; neither narrowed.

### 6.2 The pass DAG

PLAN.md §6.2 listed six passes in order. They are not a chain; they are a DAG, and writing it down
is what allows the independent branches to run concurrently and a failure to be scoped.

```
                    ┌─ A identity (cargo-metadata, project-load, artifacts)
                    │      │
                    │      ├──────────────┬───────────────┐
                    ▼      ▼              ▼               ▼
              B declarations        C crosswalk      (catalog family:
              (rustdoc, hir)        (all natives)     independent of A-E)
                    │                    │                    │
                    ├────────────────────┘                    │
                    ▼                                         │
              D semantics (hir + mir)                         │
                    │                                         │
                    ▼                                         │
              E type normalisation                            │
                    │                                         │
                    ▼                                         ▼
              F derivation (dataflow, assertions) ◄───────────┘
                                   │
                                   ▼
                          surface_binding + coverage
```

- **A must precede B**: proposal §9 Pass B reconciles *"declarations before their children"*, and a
  definition's key embeds its crate key.
- **C is a sibling of B, not a successor**: crosswalk rows may be written with
  `canonical_entity_id` NULL and bound later; that is what `mapping_status='candidate'` is for.
- **The catalog family is independent** of A–E entirely. It can build while the Rust side is still
  extracting, which matters because it is the half that serves `discover` and `describe`.
- **F is last and is the only pass that writes `derivation` rows**, so "what is derived" has a
  single writer.

**A pass is a query over the catalog, not a procedure** (PLAN.md E.2, restored per §0.1). Each node
of the DAG above is one SQL statement whose `LogicalPlan` goes straight to the writer:

```rust
let plan = ctx.sql(contract_sql).await?.into_unoptimized_plan();
WriteBuilder::new(log_store, snapshot)
    .with_input_plan(plan)                                   // evidence 05:124, proven by PB13
    .with_save_mode(SaveMode::Overwrite)
    .with_replace_where(col("run_id").eq(lit(run_id)))       // an Expr, never interpolated text
    .with_session_state(state)
    .with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState)   // PB05
```

`MergeBuilder` is not the mechanism — §6.4 settled that — but the substitution is `WriteBuilder`,
not a Rust loop. `with_input_plan(LogicalPlan)` is *"the direct path from a query to a table — no
intermediate materialisation"* (evidence 05:124), and PB13 exercised it against a constrained table.
**`with_input_execution_plan` is not an alternative**: its parameter is `Arc<LogicalPlan>` despite
the name, and no probe has touched it.

The consequence for the passes is that the DAG's edges are joins rather than call order. A pass
does not "run after" the one it depends on so much as *select from* it, which is why B's
"declarations before their children" is an `ORDER BY depth` inside a statement rather than a
traversal in a binary.

### 6.3 Failure semantics — what a partial run leaves

Each pass is **one Delta commit per target table**. There is no cross-table transaction, and v2
does not pretend otherwise. What that buys is that failure is *scoped and nameable*:

| Failure point | State left | Recovery |
|---|---|---|
| inside an adapter | staging table for that family unchanged (replace-where is atomic per table) | re-run that family |
| between passes | earlier passes committed, run row `partial` | re-run from the failed pass |
| inside a merge pass | that table committed or not; later tables untouched | re-run the pass — merges are keyed, so re-running is a no-op for rows already merged |

**The visibility rule is what makes this safe.** A partial run's rows are present, attributable and
queryable — but they do not silently contaminate a decision packet. That is the proposal's
"explicit extraction coverage" requirement applied to failures rather than to gaps.

**Where the rule lives changed during implementation**, and the reason is worth keeping. Writing
the join into each of the thirty `projection.*` views is thirty chances to write it differently,
and the one view that forgot would be the one nobody noticed. Instead the Delta providers are
registered as `<name>__all` and the *unsuffixed* name is a view over them filtered to the visible
runs, so every projection reads `mechanism` and `surface_entry` exactly as before and every one of
them is filtered — because there is no unfiltered name for a projection to read by accident.

Two tables are exempt, and the exemption is stated rather than inferred: `extraction_run` decides
visibility, so filtering it would be circular *and* would hide the only record that some rows are
unaccounted for; `lattice` is a reference table with no `run_id`, because rank-to-label is not an
observation about a snapshot.

`visible_run` is **evaluated once** rather than re-planned per reference. The obvious version — a
view, inlined as a subquery everywhere — cost 155 ms of planning across the thirty projections
against a 400 ms whole-process criterion, because a three-relation join was re-planned at every
reference. The visible set cannot change inside a one-shot query process, so it is computed into a
one-column in-memory table at startup: 96 ms, same rule, same enumerability.

**`--include-partial` reaches the rule through the `request` relation**, the same way `--bind` and
`--max-depth` do, so no SQL is assembled per invocation.

### 6.4 Transactionality — measured, and DDL-first is the correct order

Constraints are added in a **DDL phase**, at table creation, before any data is written. No merge
pass ever adds a constraint.

**PB13 (evidence 13) measured why that is not merely convenient.** A write and a constraint
addition are always **two commits** — create at `v0`, write at `v1`, constraint at `v2` — and a
failed constraint-add rolls back only itself. Comparing the two orders on the same failure:

| Order | Violating data ends up | Constraint ends up | Table self-describes the invariant? |
|---|---|---|---|
| write → constraint | **committed, at v1** | **absent** (add fails: *"1 rows failed validation"*) | no |
| constraint → write | rejected, nothing written | present | yes |

Write-first leaves a table containing a row that violates the invariant *and* no record that the
invariant was ever intended — a later reader sees an unconstrained table and assumes none applies.
Constraint-first rejects the write and leaves the table consistent with its own metadata. The
control (a *conforming* write against the same constraint, which succeeds) confirms the rejection
is attributable to the data rather than to the constraint breaking writes.

So evidence 05 §4 question 4 is closed, and the ordering rule is upgraded from "sidesteps an open
question" to **the measured-correct order**.

This also handles PB07's write-time coupling hazard from the other direction: because constraints
are fixed at creation and are plain column algebra (§9.1), no constraint ever depends on a
`bridge` UDF, so no table can be locked out by a UDF rename.

#### The scoped write — measured by PB17, and not on the builder this section used to name

Three other sections pointed here for "§6.4's merge pass". It is specified now, and the first
thing to say is that **there is no merge pass**. `MergeBuilder` has no target-narrowing predicate
at the pinned rev; `with_replace_where` is `WriteBuilder`'s (`write/mod.rs:233`), exactly as
evidence 05 §2 recorded before this document mis-attributed it.

So each write is `SaveMode::Overwrite` plus `with_replace_where("run_id = '<this run>'")`. A run id
is `run:<snapshot>/<family>/0`, so one predicate over one existing column expresses both halves of
the scope — this family, this snapshot. The `family = 'F' AND snapshot_id = 'S'` form this document
used to give could not be written at all: `family` is a column of `snapshot.extraction_run` and of
no canonical table.

Four properties, all measured in PB17 (evidence 15) against a table carrying a CHECK constraint and
Arrow extension metadata rather than a bare fixture:

| | |
|---|---|
| a scoped write **preserves** other runs' rows | and the control, the same write with no predicate, destroys them — which is what makes the first statement attributable, since `with_replace_where` is *silently ignored* outside `Overwrite` mode |
| every row written must **satisfy** the predicate | enforced per row by delta-rs, or the write fails with `Invalid data found`. The writer proves its own scope; a misattributed batch cannot quietly widen what it replaces |
| **one commit**, always | removes and adds land in a single action list. Unlike merge there is no empty-actions guard, so a scoped write with nothing to do still advances a version |
| a failed write leaves **orphan parquet** | atomic at the log level, not at storage. Only `VacuumMode::Full` reclaims it. The catalog is derived and rebuildable, so this is disk space rather than correctness |

Two tables stay whole-table overwrites, each for a reason. `catalog.lattice` has no `run_id` to
scope by — rank-to-label is not an observation about a snapshot. `snapshot.extraction_run` is
rewritten *across* runs by reaping, so a whole-table read-modify-write is the operation there
rather than a shortcut; scoping it would be rejected by the conformance check above, which is a
pleasant way to learn the exemption is real.

**One thing not to do, recorded because evidence 04 recommends it on other grounds.** Enabling
`DeletionVectors` or `RowTracking` on these tables disables parquet pushdown
(`delta_datafusion/table_provider/next/scan/plan.rs:492-494`), which is what keeps the scoped
write's rescue scan to footer reads instead of decoding every matched file.

### 6.5 Incremental re-derivation

Re-deriving after a partial re-extraction reads the change feed rather than the tables. PB10 fixed
how:

```rust
let cdf = DeltaCdfTableProvider::try_new(
    table.scan_cdf()
        .with_starting_version(last_derived_version)   // NOT a predicate on _commit_version
)?;
```

Two rules, both measured:

1. **Narrow by version bounds, never by a predicate on `_commit_version`.** The predicate is
   correct and reads the entire change range anyway — PB10's arm returned the right answer while
   scanning all three files.
2. **Narrow by partition, because nothing else narrows.** `family` is a partition column on every
   table read this way (§2.3), which is why the partitioning decision is part of the execution
   design rather than a physical-layout footnote.

The change feed is then an ordinary relation: "re-derive what changed" is a join of
`changes` against `evidence.derivation.input_fact_ids`, selecting derivations whose inputs moved.
No diffing routine exists.

---

## 7. Interactions, composition, and retrieval

### 7.1 The `Interaction` model — predicates as rows, in DNF

*Deferred item 5. Proposal §3: "Implement the predicates using a small typed expression model…
Do not leave them as arbitrary prose strings, and do not introduce a general-purpose theorem
prover."*

Both halves of that instruction point the same way, and v2 takes it further than a typed
expression tree: **a predicate is stored in disjunctive normal form as ordinary rows.** Arrow has
no recursive types, an expression interpreter would be exactly the procedural code goal (4)
forbids, and DNF is expressive enough for every interaction the surfaces actually exhibit.

```
catalog.interaction
  interaction_id     Utf8   PK
  kind               Utf8   requires | enables | has_no_effect_unless | changes_meaning_of
                          | overrides | accumulates_with | conflicts_with
                          | consumes_binding_from | equivalent_under
  affected_id        Utf8   FK -> mechanism or parameter
  effect             Utf8
  ordering_requirement Utf8 'none'|'before'|'after'|'adjacent'
  assertion_ids      List<Utf8>

catalog.interaction_atom                       one row per literal
  interaction_id     Utf8   FK
  term_ord           Int32  which disjunct  (OR across distinct term_ord)
  atom_ord           Int32  position within the conjunct (AND within a term_ord)
  facet              Utf8   a request facet, selected option, or context key
  op                 Utf8   'eq'|'ne'|'in'|'present'|'absent'|'gte'|'lte'
  value              Utf8
  value_list         List<Utf8>
  negated            Boolean
```

**Evaluation is two `GROUP BY`s and nothing else.** With `truth` encoded per §5.2:

```sql
WITH atom AS (                                    -- three-valued: 0 false, 1 unknown, 2 true
  SELECT a.interaction_id, a.term_ord,
         atom_truth(a.facet, a.op, a.value, a.value_list, a.negated, r.bindings) AS t
  FROM catalog.interaction_atom a CROSS JOIN request r
),
term AS (                                         -- AND within a conjunct
  SELECT interaction_id, term_ord, min(t) AS t FROM atom GROUP BY interaction_id, term_ord
)
SELECT interaction_id, max(t) AS truth_rank          -- OR across conjuncts; label at the edge
FROM term GROUP BY interaction_id
```

`min` then `max`. The Kleene semantics the proposal demands — *"Evaluate conditions using true,
false, and unknown. An unknown condition should become an explicit unresolved choice in retrieval,
not be silently treated as false"* — is not implemented anywhere; it is a consequence of the
encoding. An atom whose facet the request does not bind returns `unknown`, `min` propagates it
through the conjunct, `max` lets a satisfied alternative outrank it, and an interaction that
finishes at `unknown` is surfaced by `discover` as an unresolved choice because it is neither
`true` nor `false`.

`atom_truth` is the only UDF involved, and it is a pure scalar over six arguments.

**Built.** Thirty-two interactions, and the split between derived and authored is the part worth
recording. Evidence 08 listed `Interaction` as "entirely new", which is true of the table and not
of all its rows:

- **Twenty-six are derived from a column.** `regex.tsv.reachable` already says `requires -P` on 25
  rows and `default-only` on one. Both are an engine precondition on a construct, so both become a
  `requires` interaction with a single atom, and neither can drift from the index that states it.
  Inventing two kinds for one relation would have been a distinction the data does not make.
- **Six are authored from a sentence.** `unreachable.tsv` says of the size-limit flags: "Both
  flags bound the default engine only. Under `-P` they do not constrain PCRE2 at all." That is an
  interaction in prose, and rendering it as atoms is a reading, so it lives in
  `seeds/interactions.json` where it can be reviewed as a judgement rather than buried in a parser.

**The request is a relation, not a placeholder.** §7.1's SQL says `CROSS JOIN request r`, and that
turns out to be the only form that types: `atom_truth` takes six arguments of mixed kinds
including a `List<Utf8>`, so its signature gives the planner nothing from which to infer what a
placeholder in the sixth position would be. A one-row table states the type by construction, and
the caller's values reach the plan inside an Arrow array rather than near the SQL text at all.

**Ordering is a column, not an assumption.** `ordering_requirement` exists because proposal §3
warns that ast-grep rule order matters when metavariables and relational rules interact, and that
CLI argument order can affect effective configuration. A catalog that flattened invocations into an
unordered map would have asserted commutativity it cannot support — which is the same failure as
omitting `ord` from child records (§2.2).

### 7.2 `PlanFragment` composition

*Deferred item 6. Proposal §10: typed inputs and outputs, no general-purpose planner.*

```
catalog.plan_fragment
  fragment_id        Utf8 PK      name Utf8
  accepted_input     Utf8         'file_set'|'file_content'|'line_set'|'node_set'|'capture_set'
  produced_output    Utf8         same domain
  scope_assumption   Utf8         coordinate_preservation Utf8  'preserved'|'remapped'|'lost'
  recall_rank        Int8         lattice §5.1: 0 unknown, 1 heuristic, 2 preserving
  ordering_requirement Utf8
  mechanism_ids      List<Utf8>
  unresolved_obligations List<Utf8>
```

Composition is a **join, not a planner**: fragment A may precede B exactly when
`A.produced_output = B.accepted_input` and B's scope assumption is satisfied by A's coordinate
preservation. That single predicate enforces the proposal's concrete warning — *"Do not feed only
matching lines into a structural stage that requires complete enclosing syntax"* — because a
fragment producing `line_set` simply does not join to one accepting `node_set`. The rule is
structural; it cannot be forgotten.

**Built.** Eight fragments, and all three `recall` values carry real data:

- `preserving` requires a **confirmed** assertion, not merely an existing one. A `recorded`
  verdict is the tool's own word for it, which is enough to catalogue a mechanism and not enough
  to guarantee a chain keeps what it should. A fragment claiming `preserving` with nothing
  confirmed behind it fails the build.
- `heuristic` fragments **cite assertions too**, as evidence of the limit rather than of a claim.
  `rg --json` is heuristic because probes P022 and P024 *confirm* that ignored and hidden files are
  skipped by default — so the step does not see every file it was handed, and `--no-ignore
  --hidden --text` is what it owes. That condition is in `unresolved_obligations`, where a reader
  can check it.
- `unknown` is not a placeholder. `ast-grep run -U --rewrite` holds it because no probe in the
  repository establishes whether a rewrite applies to every match it found. Nobody has looked, and
  the bottom of the lattice is what that means.

The fold then does its job without anyone invoking it: `rg -l LITERAL -> ast-grep run -p PATTERN`
reports **heuristic**, because `min` takes the narrowing step's rank. That is the composition the
topic page recommends, correctly labelled as lossy.

**The cycle is in the shipped data, not only in the probe.** `rg -r` accepts a `line_set` and
produces one, so the fragment graph has a self-loop and the bound is doing real work: chains grow
15 → 46 → 144 → 290 at depths 2, 4, 8, 12 and terminate at each.

**Chain recall is the §5 fold.** A chain's `recall_rank` is `min` over its steps, so a chain
containing one heuristic prefilter is heuristic overall, automatically. The proposal's requirement

```
final_match(file) ⇒ prefilter_selects(file)
```

is what `preserving` asserts, and it is carried by a `BehaviorAssertion` with evidence — never
inferred from the fragment's shape. When the implication is merely plausible the value is
`heuristic`, and §7.4's packet prints that word rather than presenting the chain as equivalent to
the unfiltered search.

Chains are enumerated with a recursive CTE, and per PB09 the bound lives **inside** the recursive
term:

```sql
WITH RECURSIVE chain(head, tail, depth, recall_rank) AS (
    SELECT fragment_id, fragment_id, 1, recall_rank
      FROM catalog.plan_fragment WHERE accepted_input = :start_unit
  UNION ALL
    SELECT c.head, f.fragment_id, c.depth + 1,
           least(c.recall_rank, f.recall_rank)
      FROM chain c
      JOIN catalog.plan_fragment f ON f.accepted_input = (SELECT produced_output
                                                            FROM catalog.plan_fragment
                                                           WHERE fragment_id = c.tail)
     WHERE c.depth < :max_depth            -- INSIDE the recursive term. PB09.
)
```

PB09 measured the alternative: with the bound in an outer `WHERE`, a cyclic graph **timed out at
20 s** where this form returns in 24 ms. Fragment composition is cyclic in general — a capture set
can feed a textual predicate that produces another capture set — so this is not hypothetical.

**`UNION` instead of `UNION ALL` is not an alternative to the bound.** PB14 (evidence 13) measured
that `RecursiveQueryExec`'s deduplicator runs over the **full output tuple**, so a projection
carrying `depth` is unique by construction and `is_distinct=true` is inert: the same cyclic graph
still timed out at 15 s, with the plan confirming `is_distinct=true` had reached it. The two
mitigations are **mutually exclusive** — `depth` is exactly what a bound needs and exactly what
defeats distinct-based termination. Every recursive query here carries `depth`, so `UNION ALL` is
used throughout and its lack of cycle tolerance is not a latent risk.

### 7.3 Closure queries

The six graph questions PLAN.md §6.4 listed all take the same shape as above: an explicit `depth`
column, bounded inside the recursive term, with a path witness accumulated as a `List<Utf8>`.

```
containment closure       program.definition.owner_id
call-graph reachability   call_site -> call_target -> definition
dependency closure        program.dependency_edge          (replaces Guppy)
CFG reachability          program.cfg_edge
re-export chains          program.export_path
derivation provenance     evidence.derivation.input_fact_ids
```

Dependency closure is the one with an extra guard: §3.1's `resolve_present` must be `true`, or the
query returns *no rows for a reason* rather than an empty graph that reads as "no dependencies".
That check is in the view definition, not in the caller.

**Two of the six are built, and building them generalised that guard.** `resolve_present` was
written as a special case for one closure; it is really an instance of a rule every closure needs,
because **a closure that traversed nothing looks exactly like a closure that found nothing to
traverse**. So `projection.closure` reports each closure's node count, edge count, deepest chain
and the reason it stops — the same discipline `validate` applies with `checked` beside
`violations`, and for the same reason.

It earns its place immediately. Against the shipped index:

| closure | nodes | edges | max depth | why |
|---|---:|---:|---:|---|
| `containment` | 716 | **0** | 0 | `symbols.tsv` indexes items, not modules, so every `owner_path` names something that is not a definition |
| `reexport` | 273 | 273 | **1** | every alias resolves in one hop; no `canonical_path` is itself an `access_path` |

Containment is **empty on real data** — and an empty result a caller reads as "this definition
contains nothing" would be a confidently wrong answer about 716 rows. The view is still correct and
still the shape §7.3 specifies; a family that indexes modules fills it without a line changing.

**The `List<Utf8>` witness is achievable, which §7.2's `chain` had not established.** `chain`
concatenates strings; these closures use `make_array` in the base term and `array_append` in the
recursive one, and the two terms agree on type *and* field metadata. So the list shape §7.3 asks
for was never blocked — the string form was a choice, not a constraint.

Bounded termination is tested where it can fail. The real graphs are acyclic and, in one case,
edgeless, so the fixture builds a two-node cycle and asserts 4 / 8 / 16 rows at `--max-depth`
2 / 4 / 8 — the PB09 shape, with `depth` bounded inside the recursive term.

Path witnesses are required, not optional — proposal §10: *"Dependency queries use
`DependencyEdge`, with path witnesses for transitive results."* A reachability answer without the
path is exactly the opaque convenience result the proposal's §12 forbids.

**One narrow exception, recorded as available but not adopted.** PB14 measured that a `UNION`
recursion over a projection with a *finite tuple domain* — the bare node, no `depth` — terminates
on a cyclic graph unbounded, in 26 ms. That shape fits a pure reachability question asked without
path witnesses. Since this design requires path witnesses for transitive results, none of the six
queries above takes it today; it is written down so that a future "is X reachable at all" query
does not re-derive it.

### 7.4 The five projections and the decision packet

`projection.discover` · `.describe` · `.compare` · `.validate` · `.explain`, each a `ViewTable`
(PLAN.md §2.2). v2 fixes their join discipline:

- every projection filters `completion_status='complete'` (§6.3)
- every projection carries all five lattices per row (§5.3, enforced by the analyzer rule)
- **prerequisite completeness is an `INNER JOIN`**: a mechanism with an unsatisfied required
  parameter cannot appear without it, because the row does not exist. Optional explanation is a
  `LEFT JOIN` and may be projected away under budget; mandatory context cannot be.
- **`compare` joins on `entity_key`**, the only projection that crosses snapshots (§1.1)
- token budget is `LIMIT` on the *candidate* CTE only — fewer candidates, never fewer columns of a
  candidate — with the `omitted` count from the same query as a window total, so the two cannot
  disagree

A packet therefore states, for every candidate: what matched, at what `precision`, with which
interactions `true` / `false` / **`unknown`**, over which `search_domain`, returning which
`result_contract`, with `recall` for any composed chain, and evidence handles — never the evidence
itself.

**Built, and it is nineteen views rather than five.** The extra fourteen are the seven
`violations_*` views plus the small relations they and the five compose from — `mechanism`,
`parameter`, `mechanism_ability`, `mechanism_detail`, `identity`, `invariant`, `violations`. That
is the shape a view registry takes once the invariants are relational: `validate` is a union of
named views rather than one large query, so each invariant is separately queryable, separately
explainable, and separately listed in `information_schema`. `codesearch-query projections` reads
that list out of the session, and `bridge/CATALOG.md` is generated from it.

Three of the bullets above are qualified by what the implementation found:

- `completion_status` does not exist yet, because runs are not modelled until step 6. Every
  projection filters `retracted = false`, which is the column that carries the same meaning today.
- "all five lattices per row" is **two** today — `precision` and `observed` are on every canonical
  row; `truth`, `recall` and `binding` belong to interactions and chains, which are step 7. The
  projections carry what exists rather than null columns that would look like absent facts.
- the `INNER JOIN` rule holds for capability binding, which is where a mechanism is genuinely
  unreachable without the join. A mechanism's *parameter* is a `LEFT JOIN`, because in this slice
  a missing parameter row means "takes no argument" and not "unknown" — and a Rust test derives
  the expected parameter count from the source rows so that a genuinely missing one fails the
  build instead of presenting as an argument-less flag.

The binary holds no SQL of its own beyond a choice of view and a list of predicates, and caller
values bind as `Expr::Placeholder` rather than being interpolated — asserted by a test that feeds
`' OR 1=1 --` through `describe` and checks the SQL text.

---

## 8. Serving — two binaries, and the query side is one-shot

*Deferred item 7.*

```
codesearch-build    long-running. Runs adapters, staging writes, the run-scoped writes (§6.4),
                    OPTIMIZE/Z-ORDER, and the generated bridge/CATALOG.md. Never serves queries.

codesearch-query    one-shot per invocation. Opens the catalog read-only, plans, executes,
                    writes Arrow IPC or JSON to stdout, exits.

codesearch (Python) Typer/Rich CLI. Shells out to codesearch-query. Never opens a table.
```

**Decision: `codesearch-query` is one-shot, not a resident session.** Three reasons, in order of
weight:

1. **A resident session would have to invalidate a cache against tables a build rewrites.** The
   merge passes and `OPTIMIZE` both rewrite files; a long-lived `EagerSnapshot` would serve stale
   data or need a watch. That is a whole subsystem to get wrong, and PB02b's lesson — that an
   unobserved channel and a working one look identical — applies directly to cache staleness.
2. **The catalog is small.** Evidence 08 counts ~12,000 existing rows; the Rust program model for a
   single workspace is larger but still bounded by one snapshot. Delta log replay for a table with
   a checkpoint is a bounded read, not a scan.
3. **One-shot makes the process boundary the isolation boundary**, which is what the repository's
   own boundary rule wants: no resident process holding handles into state directories.

**The trend, and where it stopped being one:**

| measured at | tables | views | p95 | registration |
|---|---|---|---|---|
| step 3 | 6 | 19 | 142 ms | eager |
| step 5 | 11 | 27 | 198 ms | eager |
| step 7 | 13 | 30 | 218 ms | eager |
| phase 4 | 14 | 45 | ~322 ms, breaching under load | eager |
| phase 5a step A | 14 | 45 | ~255 ms | eager, metadata-only opens |
| phase 5 | 20 | 52 | ~330 ms | eager, metadata-only opens |
| **phase 5a step B** | **20** | **52** | **~185 ms** | **lazy** |
| phase 7, one snapshot | 20 | 40 | ~200 ms | lazy |
| phase 7, two snapshots | 20 | 40 | ~246 ms | lazy, snapshot-filtered |
| **M1, with the analyzer rule** | **20** | **41** | **~273 ms** | **lazy, snapshot-filtered** |

The cost was never the queries. It was preparation nobody asked for: at 20 tables and 52 views,
63 ms opening Delta tables, 67 ms building run-filtered base views and 118 ms planning projections
— 250 ms before an ~80 ms answer, for a query that reads four tables. Session construction, the
usual suspect, is **1.2 ms**.

Two changes, measured separately:

- **`DeltaTableBuilder::load()` is the expensive half of opening a table, and registration does not
  need it.** `load()` runs `EagerSnapshot::try_new`, whose second phase replays every Add action
  into Arrow and parses statistics. The first phase alone yields protocol, metadata and schema, and
  `TableProviderBuilder::build()` given only a log store does exactly that — the path delta-rs's own
  `DeltaTable::table_provider()` takes. Replay moves to scan time for the tables actually read.
- **Registration is lazy.** `SchemaProvider::table()` is `async` and called only for references the
  parser found; `table_names()` is sync, so the whole surface is still enumerable. `bridge/lazy.rs`
  resolves a Delta table or plans a view on first reference.

M1's rise is the analyzer rule: `IdentityDiscipline` walks every plan, resolving each join's
operands against a merged schema, and that is paid on every invocation rather than once per gate.
It buys a class of silently-wrong answer being impossible to express. ~27 ms against a 400 ms
criterion is a price worth naming rather than absorbing — if it ever mattered, the rule could move
to the gate the way §7's eager-registration guarantee did, at the cost of letting a bad join reach
a caller once.

Phase 7's rise is the honest cost of holding two snapshots: the tables carry twice the rows, and
every base view resolves `selected_snapshot` before it serves anything. It is a cost per
*snapshot held*, not per table added, and it is the one the catalog now exists to pay.

**The important part is not the number, it is the shape.** Eager registration made every
invocation pay for the whole catalog, so every phase made it worse and the criterion was a cliff
ahead. Lazy registration makes an invocation pay for what it touches, so phases 6, 7 and 8 add
tables and views without adding latency to a query that does not read them.

**§7's argument for eager registration survives, relocated.** It said a projection that will not
plan is a broken retrieval surface, and discovering that per caller is how a catalog rots. True —
so `projection::check_all` and the `information_schema.views` test plan every projection, and
`just codesearch-check` runs them. The guarantee is now paid once per gate rather than once per
caller, which is where a developer-time guarantee belongs.

**Recorded as available, not adopted**, in the manner of PB14's narrow exception: delta-rs
`Snapshot` and `EagerSnapshot` implement `Serialize`/`Deserialize`, and upstream
`test_materialized_snapshot_serde_update_provider_preserves_reuse` proves deserialise plus
incremental `update()` avoids log replay entirely. A cross-process warm cache is therefore
buildable. It is not built, because it adds a cache-invalidation surface to a tool whose whole
isolation argument is the process boundary — but it is the cheaper cousin of the resident-server
fallback below, and should be tried first if this is ever revisited.

**With a falsifiable exit criterion**, because "it will be fast enough" is not an argument: the
build emits a `bench` recipe measuring p50 and p95 wall time for `describe` on a warm page cache.
**If p95 exceeds 400 ms, the decision is revisited** — the fallback is a resident
`codesearch-serve` over a unix socket with the CDF as its invalidation signal (§6.5 already gives
the mechanism). That fallback is designed but not built.

Session construction is not free and is the thing to watch: per PB05/PB07 the session must be
built from delta-rs's `create_session()` (a bare DataFusion `SessionContext` cannot execute a Delta
write at all), and the UDF registry, extension types and rules are registered on it each time.

---

## 9. Validation and coverage

### 9.1 Where each invariant lives — the correction PB07 forced

PLAN.md §4.2 claimed *"No validation pass exists, because there is nothing left for it to check."*
**That was too strong.** PB07 measured two hard limits: a Delta CHECK constraint that names another
table **panics the process** (`get_table_source` is `unimplemented!()`), and one that calls a UDF
makes the table unwritable by any future session lacking that UDF.

So invariants are placed by **arity**, and the placement is mechanical:

| Invariant shape | Home | Example | Enforcement |
|---|---|---|---|
| single row, single table, plain column algebra | **Delta CHECK constraint** | `precision IN ('exact','inexact','absent')`; `anchor.start <= anchor.end` | write-blocking |
| cross-row or cross-table | **`projection.violations_*` view** | every `evidence_id` resolves; every `owner_id` exists | queried by `validate`, reported, not write-blocking |
| code shape | **ast-grep rule** in `queries/rules/project/` | no `Console.print` without `escape`; no `UInt32` offset column | pre-commit |
| library behaviour | **probe with a control** | the twelve in `evidence/probes/` | build |

**No CHECK constraint may reference a `bridge` UDF.** That is a rule with a mechanical check
(§10.2), not a guideline, because the failure mode is a table nobody can write and the cause is
invisible at the call site.

The `validate` projection is therefore a real operation with real output, not a no-op — it unions
the `violations_*` views and reports counts by invariant. Round 1's aspiration survives in the part
that was actually achievable: **no imperative validation pass exists.** Validation is still
entirely relational; it is just that half of it is a query rather than a constraint.

**It reports `checked` beside `violations`**, added while building step 5. Eight invariants
reporting `0 violations` reads as a clean bill of health whether they inspected a thousand rows or
none, and one of them genuinely inspects none today — `behavior_assertion.subject resolves` is
scoped to `subject_kind = 'mechanism'` and every assertion's subject is a topic. So
`projection.invariant` declares each invariant with the number of rows it examines and `validate`
joins the two. The join is a `FULL OUTER` one, which also catches the reverse mistake: a violation
emitted under a name no invariant declares appears with a null `checked` rather than vanishing
into an unmatched row. This is the same discipline the surrounding skills apply to search results
— silence is not absence, and neither is a check that never ran.

Writing these views is what found two defects in step 4 that no CHECK constraint could have
caught, because both are cross-table facts: 16 regex mechanisms that appeared in no coverage
denominator, and 49 assertions naming mechanisms that do not exist (§12.1b).

### 9.2 Coverage — two numbers, never one

**There are three numbers now, not two**, and the third exists because the program model does. It
asks how much of the tool's own code a capability mechanism can point at, which has a different
denominator and a different remedy from the other two. It is expected to be small and small is the
answer: seven of 323. A derived number would have said "all of them" on the strength of a
crate-name match, which is the confidently-wrong result the whole design refuses.

```sql
-- surface coverage: how much of the tool is catalogued
SELECT s.tool, s.source_table,
       count(*)                               AS surface_entries,
       count(m.entity_key)                    AS characterised,
       count(*) - count(m.entity_key)         AS uncharacterised
FROM   catalog.surface_entry s
LEFT   JOIN catalog.mechanism m ON m.entity_key = s.mechanism_key
GROUP  BY s.tool, s.source_table;
-- The count is of the JOINED column, not of `s.mechanism_key`. Counting the entry side's own
-- claim would count a DANGLING key as characterised, which overstates coverage exactly where the
-- catalog is broken. `projection.violations_dangling_surface_entry` reports those separately.

-- evidence coverage: how much of what is catalogued is supported
SELECT c.mechanism_id,
       count(*)                                              AS contract_facets,
       count(b.assertion_id)                                 AS supported,
       min(coalesce(b.precision_rank, 0)) AS weakest_rank   -- 0 = absent
FROM   catalog.contract_facet c
LEFT   JOIN evidence.behavior_assertion b ON b.subject_entity_id = c.mechanism_id
GROUP  BY c.mechanism_id;
```

Both are `LEFT JOIN`s against a denominator that exists independently, which is what makes them
undriftable. A facet with no assertion appears with `precision='absent'` rather than being absent
from the output — the `coalesce` is the whole point, and it is the data-model form of the
"silence is not absence" discipline the surrounding skills enforce in prose.

**The evidence half is implemented differently, because `catalog.contract_facet` belongs to step
7.** What exists today is the `observed` lattice on `catalog.mechanism`, whose values come
straight from the skill's own `regex.tsv`, so `projection.coverage_evidence` folds that instead:
per tool and surface, how many mechanisms there are, how many reach `confirmed`, and the `min`
of the group's ranks as `weakest`. It answers the same question — how much of what is catalogued
rests on something that was actually run — against the denominator that exists, and it exercises
§5's fold in a projection rather than only in a test. The `confirmed` threshold is read from
`catalog.lattice` rather than written as a literal, so adding a value to the lattice cannot leave
a stale bound behind. It is replaced by the query above once contract facets exist.

Per §5.4 these stay **separate numbers** -- three of them since the program model arrived, and
the count is the only thing about that sentence which has changed. There is no composite score. The CLI prints them as two
tables and the JSON form keys them separately rather than concatenating two differently-shaped row
sets into one array, because an array invites a consumer to sum it.

---

## 10. `bridge/` — the consolidated extension catalog

### 10.1 Final UDF inventory

Every scalar UDF below is `Volatility::Immutable`, implements `documentation()` and
`return_field_from_args()`, and — where it can appear in a filter — implements `preimage()`
returning a **half-open** upper bound over a **bare stored column**. Those three clauses are PB06's
measured requirements (§10.2 checks them).

| Function | Kind | Job | `preimage`? |
|---|---|---|---|
| `lattice_label(name, rank)` | scalar | render a stored rank for display (§5.2) | n/a — never in a predicate |
| `atom_truth(facet, op, value, list, negated, bindings)` | scalar | one interaction literal (§7.1) | no — not a range |
| `anchor_contains(outer, inner)` | scalar | source-range containment | **yes**, on `start`/`end` |
| `anchor_overlaps(a, b)` | scalar | range intersection | **yes** |
| `mechanism_satisfies(contract, request)` | scalar | the §2 selection predicate | no |
| `supports_relationship(mech, rel)` | scalar | the relationship vocabulary | no |
| `kind_is_a(lang, kind, super)` | scalar | grammar node subsumption | no |
| `regex_engine_supports(construct, engine)` | scalar | the two-engine matrix | no |
| `entity_key_namespace(key)` | scalar | the §1.2 prefix | **yes** — prefix range |
| `coverage_denominator(surface)` | aggregate | §9.2's honest denominator | — |
| `evidence_chain(fact_id, max_depth)` | table | provenance as rows; **`max_depth` required** | — |
| `probe_replay(probe_id)` | table | re-execute a behaviour probe | — |

`evidence_chain` takes `max_depth` as a **required** argument, not an optional one, because its
implementation is the §7.3 recursive CTE and PB09 showed an unbounded one does not terminate on a
cyclic graph. Derivation provenance is cyclic whenever a rule's output feeds another rule that
feeds it back.

Removed from PLAN.md §5.2's inventory: `precision_and` / `precision_or`, subsumed by plain
`min`/`max` over the stored rank (§5.2). Round 2 also dropped `lattice_rank`, which the first
draft of this section still carried: once the rank is what is stored, converting to it is not a
step anyone performs. Three fewer UDFs and one fewer concept.

### 10.2 The mechanical checks

`just codesearch-check` runs these; each exists because a probe showed the failure is silent.

| Check | Rejects | From |
|---|---|---|
| **schema fixed-point** | any table whose read-back Arrow schema differs from the authored one | PB03 — Delta narrows silently |
| **no unsigned, no `UInt32` offsets** | an offset column not `Int64` | PB03 — truncates above `i32::MAX` |
| **UDF volatility** | a `bridge::udf::scalar` that is not `Immutable` | PB06 — loses its filter entirely, no diagnostic |
| **preimage boundary test** | a filter-eligible UDF whose tests lack a bucket-boundary case | PB06 — the closed/half-open error is invisible without one |
| **preimage targets a stored column** | a `preimage` returning a derived expression | PB06 — rewrites but never prunes |
| **constraint purity** | a CHECK constraint naming a UDF or another table | PB07 — locks out writers / panics |
| **constraint placement** | adding a constraint to a table that already holds data | PB13 — write-first commits the violating row *and* fails to record the invariant |
| **session policy** | a delta-rs builder call site without `RequireSessionState` | PB05 — default discards the session with a log line |
| **recursion bound** | a recursive CTE whose depth bound is not inside the recursive term | PB09 — 20 s timeout vs 24 ms |
| **lattice propagation** | a projection joining children without a fold per dimension | §5.3 |
| **family contract** | an adapter writing a column its family "never fills" | §3, evidence 06 |
| **markup safety** | a `Console.print` of non-literal text without `escape()`/`markup=False` | P01 — `[a-z]` is deleted |

`bridge/CATALOG.md` is **generated** from `information_schema` plus each UDF's `documentation()`,
so the inventory of custom functionality cannot drift from the code.

---

## 11. The CLI

```
codesearch discover  --subject --relationship --result --semantic-layer --scope --language
                     [--surface] [--max-depth N] [--budget N] [--include-partial] [--json]
codesearch describe  ENTITY_KEY... [--field ...] [--stage ...] [--json]
codesearch compare   ENTITY_KEY --from SNAPSHOT --to SNAPSHOT [--json]
codesearch validate  [--invariant ...] [--json]
codesearch explain   ASSERTION_ID [--max-depth N] [--json]
codesearch coverage  [--tool ...] [--json]
```

Arguments are `Expr::Placeholder` bindings, never interpolated strings. `--json` never touches a
`Console`. Human output obeys P01: `escape()` or `markup=False` on every non-literal, enforced by
an ast-grep rule rather than by review.

Per PB08, patterns and rule fragments render through `Syntax`, which **does not markup-parse** —
measured at 0 of 8 P01-lossy forms altered, with both controls clean. There is no regex lexer among
pygments 2.21.0's 602, so they render safe and unhighlighted; `Syntax(code, "regex")` resolves to
`None` *identically to a nonsense name*, so the renderable assignment names `text` explicitly
rather than asking for a lexer that silently is not there.

`--max-depth` is exposed on every command that reaches a recursive CTE, defaulting to 8. That is a
user-facing consequence of PB09: the bound is real, so it is visible rather than hidden.

---

## 12. Build sequence, risks, and what is still open

### 12.1 Sequence

Each step ends with a runnable, tested system; a scaffold is not an implementation.

**Steps 1–5 and 7 are built.** The code is [`codesearch/`](codesearch/), its state is in
[`codesearch/README.md`](codesearch/README.md), and the implementation handoff — including
everything still outstanding — is [`IMPLEMENTATION.md`](IMPLEMENTATION.md).

1. ✅ **`bridge/`** — session (`create_session`, `RequireSessionState`), the lattices as stored
   `Int8` ranks, `lattice_label` for display, identity minting, the `CanonicalTable` wrapper.
2. ✅ **Schema + DDL phase** — constraints at creation (§6.4), partitioning per §2.3, and the
   fixed-point round trip against real Delta storage.
3. ✅ **`CanonicalTable` wrapper** — `constraints()` declared, no `statistics()` (PB02b).
4. ✅ **Catalog family** (§3.8) — 654 index rows to 13 canonical tables and 1,859 rows: 8
   capabilities, 323 mechanisms, 165 parameters, 305 bindings, 9 result contracts, 279 search
   domains, 10 negative-space records, 621 surface entries, 49 behaviour assertions. Idempotent.
5. ✅ **Projections + CLI** — thirty registered views (§7.4) behind `discover`, `describe`,
   `capabilities`, `coverage`, `validate`, `negative-space`, `result-contracts`, `domains`,
   `interactions`, `fragments`, `chains` and `projections`, through a Typer/Rich front end.
   **First end-to-end slice.**
6. **Rust families A → F** in DAG order (§6.2), each with its contract assertion.
   ✅ **The execution model is built** (§6.1, §6.3): `snapshot.extraction_run`, the sixth lattice,
   the write-the-run-row-first lifecycle and the visibility rule.
   ✅ **The first family is built** — `library-api` (§3.5's declaration half, from the skill's own
   rustdoc-derived index rather than a live rustdoc run): 23 packages, 716 definitions, 4,727
   signatures, 2,241 implementations, 273 export paths, 5 unbound native handles. Two families now
   coexist, each bracketing its own tables with its own run row, in one process.
   The remaining families need a toolchain, a network fetch, or pass F.
7. ✅ **Interactions and plan fragments** (§7.1, §7.2) — 32 interactions over 33 atoms, evaluated
   by the two `GROUP BY`s; 8 plan fragments composed by a join and enumerated by a bounded
   recursive CTE.
   ✅ **Two of §7.3's six closure queries are built** — containment and re-export chains, with
   `projection.closure` reporting what each traversed. The other four need the `hir` and `mir`
   families, `cargo-metadata`'s `resolve_present` guard, or pass F.
8. ✅ **`compare` is built** (§7.4, §6.4); `explain` and CDF re-derivation remain.
   Two snapshots coexist in one catalog: writes are scoped to a run, reads are scoped to a
   snapshot, and `compare` is the one projection that deliberately is neither. `explain` still
   needs `evidence.derivation` rows, which only pass F writes.

Measured after phase 7: **175 tests green** (128 Rust, 47 Python), nine invariants at zero
violations over 9,843 identity rows per snapshot, **20** canonical tables and **40** projections,
and **two snapshots coexisting** in one catalog. `describe` p95 is **~246 ms** against §8's 400 ms
— see §8 for the trend and what the second snapshot cost.

#### The order changed twice, and both times for a stated reason

**`validate` moved out of step 8 into step 5.** It needed only the `violations_*` views, and
writing them is what found two defects in step 4 (§12.1b).

**Step 7 moved ahead of step 6.** §6.2 states the catalog family is "independent of A–E entirely",
and §7.1/§7.2's tables live wholly in `catalog.*` with no `program.*` reference — so the ordering
above was a listing order, not a dependency. Building interactions and fragments early turned out
to matter for a second reason: they are where the `truth` and `recall` lattices first carry data,
and until something used them the five-lattice generalisation was asserted rather than exercised.

**Step 8 is half done.** `compare` needed two snapshots in one catalog, and getting them cost more
than the projection did: the write had to be scoped to a run, and — the part no document
anticipated — *every existing view* had to be scoped to a snapshot, because none of the
thirty-seven filtered on `snapshot_id` and a second snapshot would have doubled every count in
silence. `explain` still needs `evidence.derivation` rows, which only pass F writes.

#### The one design item still deferred

The `LatticePropagation` / `IdentityDiscipline` analyzer rules (§5.3, §1.4), and the plan for them
has since split in two:

- **`IdentityDiscipline` is buildable and its prerequisite is now measured.** A recursive CTE
  refused to plan because one term's `recall_rank` carried the `codesearch.lattice` extension name
  and the other's did not — which establishes that Arrow field metadata reaches the *logical plan
  schema*, not merely storage. That is exactly what a rule rejecting `entity_key ⋈ entity_id` at
  plan time needs, and it was established by something failing rather than by reading source.
- **`LatticePropagation` should not be an analyzer rule.** Its trigger is "joins a parent to
  children without folding each lattice", and parent-child is not decidable from a logical plan. A
  version conservative enough not to block legitimate queries catches almost nothing; one
  aggressive enough to catch something blocks them. A test over the registered projection set is
  enumerable, known, and exactly the population the rule was meant to police.

Note also that `projection.violations_underived_entity_id` already checks the identity half as
*data* on every `validate`, which is the cheaper half of what the analyzer rule was for.

### 12.1b What implementation changed in this design

Building falsified several things in this document and added others. All of them are already
applied above; they are collected here because a reader comparing the design to the code should
know which parts were revised by contact with it.

| Section | Change | Found by |
|---|---|---|
| §5.1, §5.2, §10.1 | lattices stored as **`Int8` ranks**, not `Utf8` labels; `lattice_rank` deleted | rank order and byte order differ, so a label column cannot reliably prune |
| §5.1 | a **fifth lattice**, `observed` (`not-probed → unknown → recorded → confirmed`) | the skill's `regex.tsv` already carries exactly this vocabulary |
| §1.2 | an ast-grep rule-field key is **scope-qualified** (`ruleObject.has`, not `has`) | 22 field names occur in more than one scope; a bare name merges mechanisms that are not the same |
| §3.8, §9.2 | a regex construct supported by both engines is **two surface entries**, not one | writing `projection.violations_orphan_mechanism` — 16 `rg` pattern mechanisms were in no coverage denominator at all |
| §3.8 | a behaviour assertion's subject is a **topic**, not a mechanism | writing `projection.violations_dangling_assertion_subject` — 49 references named mechanisms that do not exist |
| §9.2 | `validate` reports **`checked` alongside `violations`** | an invariant that inspected nothing reported `0 violations`, which is indistinguishable from passing |
| §4.3 | a new table, **`catalog.negative_space`** | nothing existing could say "known absent, for this reason, do this instead" without asserting something false |
| §9.2 | the coverage numerator is **`catalog_key` + `catalog_kind`**, not `mechanism_key` | three quarters of the inventory can never become a mechanism, so coverage was asking the wrong question of it |
| §7.2 | `preserving` requires a **confirmed** assertion, not merely an existing one | a `recorded` verdict is the tool's own word for it, and the failure it would hide is a chain quietly dropping candidates |
| §1.4 | Arrow extension metadata **reaches the logical plan schema**, not only storage | a recursive CTE refused to plan because the base term's `recall_rank` carried `codesearch.lattice` and the recursive term's `CASE` did not |
| §6.3 | the visibility rule lives at the **base-table boundary**, not in each of the thirty views | thirty statements of one rule is thirty chances to write it differently, and the view that forgot would be the one nobody noticed |
| §6.1 | reaping is **not time-based**, and a rebuild of the same snapshot **supersedes** rather than being reaped | a timeout makes the verdict depend on an unmeasured clock skew; and re-running the same build is not a second run to abandon |
| §8 | eager registration must go **lazy at phase 5**, not after it | phase 4 took p95 from 218 ms to 326 ms against a 400 ms criterion, and phase 5's six tables would land near 380 ms |
| §3.6 | `mapping_basis` is **`mir.rustc_public@runtime`**, and only `runtime` is reachable | PB15 — one function is 14 / 8 / 6 blocks at `built` / `analysis` / `runtime`, and both candidate surfaces read the last |
| §8 | registration is **lazy**, and eager registration's guarantee moves to the gate | preparation nobody asked for was 250 ms of a 330 ms answer; session construction was 1.2 ms of it |
| §3.5, §1.2 | a signature's identity includes its **signature text** | `methods.tsv` records the trait PATH, not the trait REFERENCE — 34 triples across 126 rows, every one a generic `From`/`TryFrom`/`PartialEq` |
| §3.8, §4.3 | `catalog.surface_binding` is a **subject/object** relation, not capability↔mechanism | it is what §3.8 says it is only if it can hold `mechanism -> def:`; it could not |
| §3.1 | `pkg:` keys **lose the source segment** | `pins.crates` is a `name -> version` object; `crates-io` would be invented, and several of these crates are workspace members |
| §3.8, §9.2 | an index file **declares its coverage denominator** | an unhandled stem fell through `_ => continue` and counted for nothing, silently — indistinguishable from a deliberate exclusion |
| §3 part 4 | "never fills" is a **real build-time check** | it was prose in two doc comments and nothing enforced it |
| §7.3 | every closure reports its **edge count and its limit**, not just its rows | `resolve_present` was written as one closure's special case; containment is empty on real data and an empty reachability answer reads as "there is nothing there" |
| §1.1, §7.4 | a canonical row carries a **content digest**, `payload_digest` | `compare` joins on `entity_key`, which settles added and removed and leaves `changed` undecidable -- `entity_id` differs between snapshots for an *unchanged* entity by construction |
| §1.1 | the digest excludes every column **typed** `codesearch.entity_id`, not just the row's own | the phase-7 fixture reported all 165 `catalog.parameter` rows as changed when three things had changed: `mechanism_id` is an entity reference and moves with the snapshot |
| §2.2 | the digest excludes `ord` | a position describes the *sequence*, not the element. Removing one flag shifted every later sibling's `ord` and restated one removal as a hundred changes; `interaction_atom` carries its order twice over, so nothing is lost |
| §1.1 | `snapshot_id` is derived from the tool pins **and the index bytes**; `context_id` keeps the pins alone | two skills with different index content and the same tool versions shared a snapshot id, so their rows collided on `entity_id` and `compare` had nothing to compare |
| §6.3 | the **snapshot filter** joins the run filter at the base-table boundary | no view filtered on `snapshot_id`, so all 37 silently unioned every snapshot; a second one doubled every coverage count with nothing raising an error |
| §6.4 | `with_replace_where` is a **`WriteBuilder`** method, and the predicate is `run_id` | `MergeBuilder` has no such method at the pinned rev, and `family` is not a column of any canonical table |
| §11 | `compare`'s `ENTITY_KEY` is **optional** | "these differences and nothing else" is not a claim a single-key query can make, and it is the acceptance criterion |
| §5.3 | `LatticePropagation` is a **test over the registered projections**, not an analyzer rule | parent-child is not decidable from a logical plan; the population it polices is enumerable, so a red test gives the same guarantee against the set that matters |
| §10.1 | three of the twelve UDFs are **relations, not functions** | `regex_engine_supports`, `supports_relationship` and `coverage_denominator` would each have to carry a table inside the binary, where a skill update could silently disagree with it. Two are already answerable by existing projections and the third *is* `projection.coverage_surface` |
| §10.1 | `entity_key_namespace`'s `preimage` is **measured**, not asserted | PB18: a `Utf8` `Interval` is constructible, the rewrite fires, and it reaches the predicate a person would have written -- with a control that does not rewrite |
| §9.1 | an ast-grep rule **cannot see inside a macro** | `vec![...]` is a token tree, so a pattern over `Field::new(...)` matches the three call sites written as plain expressions and none of the two hundred authored inside `table(vec![...])`. A rule was written, passed its own fixtures, and matched nothing real |
| §1.2 | the grammar has **four** rules, and §5.1 has **six** lattices | both were miscounted against their own lists |
| §3.1, §1.2 | `program.target` and `program.crate_unit` are **one table**, keyed `crate:` | §1.2's grammar has no `target:` namespace; a first pass invented `#target:` and nothing checked it, because nothing checks key shapes |
| §1.2, §3.1 | `pkg:` **regains its registry segment** | reading vendored checkouts with `--no-deps` reports a registry crate as a PATH package with `source: null`; a resolve over a pinned subject manifest recovers it |
| §3.1, §7.3 | `resolve_present` is **true**, and the dependency closure traverses 369 edges | the adapter resolves a pinned subject rather than reading checkouts, so both endpoints are package keys rather than a name and a requirement |
| §3.2 | `proc_macro_policy` is **per crate**, and must be read from the per-crate `ProcMacroLoadResult` | PB16: `load_workspace_at`'s `Option<ProcMacroClient>` collapses `disabled` and `server_failed`, which evidence 06 says must not be collapsed |
| §3 | a family reads **one root**, and `IndexSpec` says which | `read_all` hard-errored on the first file a producer root does not ship, and two roots can ship the same filename |

The third is the one worth dwelling on: `has` under `relation` and `has` under `ruleObject` apply
in different places and take different values. Merging them would have licensed using one where
only the other is valid — precisely the normalisation the proposal forbids — and nothing in the
design would have caught it. A uniqueness test on `entity_id` did.

The two table-shape changes came from the same slice. `unreachable.tsv`'s ten rows had been sitting
in the denominator as permanently uncharacterised surface entries; making them first-class is what
gives `discover` a **negative** answer worth returning — "ripgrep never modifies a file, by design;
use `ast-grep -U`" instead of an empty result a caller reads as "search harder". Forcing them into
`capability` would have asserted the ability is one this model holds; into `mechanism`, that a way
to exercise it exists. Both are false.

Generalising the numerator followed immediately: once an exit code becomes a `result_contract` and
a language a `search_domain`, a column called `mechanism_key` is a question three quarters of the
inventory cannot answer, and `characterised = 0` was reporting a modelling gap as a coverage gap.
The `truth` lattice now carries its first real data as a side effect — `languages.tsv`'s
`accepted`/`rejected` is an adjudication reached by running each language against the binary, and
`unreachable.tsv`'s `partial` is a genuine `unknown`.

The last three of the first table all came from the same act: **writing the validation views found
the defects they were written to guard against.** That is the argument for building `validate` early rather than in
step 8. Two of them were wrong *data* that no schema constraint could have caught, because both
are cross-table facts — exactly the class §9.1 moved out of CHECK constraints and into queries.
The fourth is about the report rather than the data, and is the same discipline the surrounding
skills apply to search results: silence is not absence, and neither is a check that never ran.

### 12.2 Risks that survive round 2

| Risk | Mitigation |
|---|---|
| `entity_key` grammar proves too rigid for a family | the grammar is data (§1.2), and `native_binding` absorbs anything unkeyable as `mapping_status='unbound'` rather than forcing a bad key |
| DNF is not expressive enough for some interaction | the atom table is append-only; a genuine need for nesting becomes a *new* `interaction` row referencing another, not an interpreter |
| one-shot query latency | **retired as a live risk.** Registration is lazy (§8), so cost tracks what a query touches rather than how big the catalog is: p95 fell from ~330 ms to ~200 ms while the catalog grew from 14 tables to 20 and 30 views to 37. The criterion and the resident fallback both stand; they are simply no longer being approached |
| a second snapshot silently destroys the first | **discharged in phase 7.** Every write is `SaveMode::Overwrite` scoped by `with_replace_where("run_id = '<this run>'")`, and delta-rs rejects a batch whose rows do not all satisfy that predicate -- so the writer proves its own scope rather than being trusted. PB17's control shows the unscoped path destroying what the scoped path keeps |
| a second snapshot silently changes every existing answer | **found in phase 7, and it was the larger half of the work.** No view filtered on `snapshot_id`, so all 37 silently unioned every snapshot: a second one would have doubled every coverage count and returned two rows per `describe`. The filter now lives beside the run filter, in one statement, and `runs` prints which snapshot a request resolved to |
| the catalog accumulates snapshots with nothing pruning them | true, and stated rather than solved. `runs` lists them and `--snapshot` selects among them; a `prune` command is deferred. `WriteBuilder` with a zero-row input plan and a `replace_where` predicate is the mechanism it would use |
| the seven `descend_into_macros` variants prove per-query, not per-run | `mapping_basis` already records which; a per-query choice becomes a column rather than a redesign |

### 12.3 Still open

~~1. Does `WriteBuilder::with_input_plan` share a transaction with `ConstraintBuilder`?~~
**CLOSED — PB13** (evidence 13): no, always two commits, and a failed constraint-add is
non-destructive. §6.4 upgraded from "sidestepped" to "the measured-correct order".

~~2. Does a `UNION`-flavoured recursive CTE terminate on a cyclic graph?~~
**CLOSED — PB14** (evidence 13): only when the projection excludes `depth`, so it cannot serve as
a safety net for queries that carry one. §7.2 and §7.3 updated; the narrow reachability exception
is recorded.

~~3. Is `rustc_public` stable enough at the pinned nightly to be the MIR extraction surface?~~
**CLOSED — PB15** (evidence 14): yes, and `mapping_basis` is decided on capability rather than
preference. §3.6 updated — including a narrowing the question did not ask for.

~~4. Does `MergeBuilder::with_replace_where` commit in one transaction at the pinned rev?~~
**CLOSED — PB17** (evidence 15), and the question was about the wrong type: **`MergeBuilder` has
no `with_replace_where` at this rev.** `with_replace_where` is `WriteBuilder`'s alone
(`write/mod.rs:233`), which is what evidence 05 §2 recorded in round 1 before this document
compressed it onto the wrong builder. So the scoped write is one builder call rather than a merge
rewrite. §6.4 and §12.2 are corrected.

~~5. How does `ra_ap_hir` behave when proc macros fail to build?~~
**CLOSED — PB16** (evidence 17): `enabled` is distinguishable, `disabled` and `server_failed` are
not — both present as `client = None` from `load_workspace_at`. The distinction is made at
`load-cargo-lib.rs:111-134` and discarded at line 204 by `and_then(Result::ok)`, surviving only in
tracing and in the **per-crate** `proc_macros` map. So §3.2's value is reachable, and §3.2 needs
two changes: the family must read the per-crate `ProcMacroLoadResult`, and `proc_macro_policy` is
per crate rather than per load.

**Nothing remains open.**

**The probe backlog is empty.** Nineteen probes have executed.
**PB18** (evidence 16) was added while building the UDF inventory, because §10.1 claimed a string
prefix `preimage` and nothing had established that a `Utf8` `Interval` is constructible at all.

**PB19** (evidence 18) was added before building §3's staging reader, because no dossier had ever
touched a DataFusion file reader and §3 replaces the one guarantee `index.rs` exists to provide.
It found that the arity guard is kept by default — `truncated_rows` is `false`, and a short or long
row fails the read naming the line and both counts — and that **two options must be set explicitly,
one of which would have silently corrupted shipped data.** `file_extension` defaults to `".csv"`
and rejects every `.tsv`; `quote` defaults to `'"'`, and a cell *beginning* with a quote loses it
with no error. Three cells of `behaviors.tsv` begin with one. The arm that mattered was not the
control but **E against F** — a cell *containing* a quote parses correctly and licenses exactly the
wrong conclusion about a cell that *starts* with one.

**A lesson from closing question 2 that is worth more than the answer.** The question named a
method that does not exist, and it had been recorded correctly one round earlier: evidence 05 §2
lists `with_replace_where` under `WriteBuilder`. The error entered when this document summarised
that dossier. **A summary of a primary source is not a primary source** -- and the cheap guard is
that a plan citing an API should cite the dossier line it came from, so the two can be compared
without re-reading the library.

**A dependency-graph fact already paid for:** `ra_ap_*` 0.0.352 does not resolve to a buildable
graph unaided. `ra-ap-rustc_lexer` asserts at compile time that `unicode-ident` and
`unicode-properties` share a Unicode version, and a fresh resolution pairs Unicode 18 with 17.
`unicode-ident = "=1.0.24"` is pinned in that probe's manifest to force the pair.
