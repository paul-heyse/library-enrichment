# Design Review — Reference

Companion to `.claude/skills/design-review/SKILL.md`. §1 is a lookup table. Everything after it is a set of lenses and calibration examples — angles that have turned up real defects before, offered because they are useful, not because the review owes them. The normative standard is [`DATA_MODEL_DESIGN_CHARTER.md`](DATA_MODEL_DESIGN_CHARTER.md); the repo-specific mapping is [`ADDENDUM.md`](ADDENDUM.md).

---

## §1 Principle index (DM-01–DM-60)

Titles and requirement levels, for accurate citation and for gate reasoning. A MUST-level gap on in-scope behavior cannot be waived by an exception record — it narrows the supported scope or the design is unresolved against that requirement. A SHOULD-level deviation can be accepted with a §10 record.

### Group 1 — Semantic authority and the modeling boundary

| ID | Level | Title |
|---|---|---|
| DM-01 | MUST | Make meaning—not storage shape—the primary model |
| DM-02 | MUST | Assign one authority to each semantic fact and revision |
| DM-03 | MUST | Unify logical contracts without mandating one physical structure |
| DM-04 | MUST | Declare the semantic boundary and expose opaque behavior |
| DM-05 | SHOULD | Separate intent from mechanisms and incidental technology |

### Group 2 — Semantic types, schemas, and invariants

| ID | Level | Title |
|---|---|---|
| DM-06 | MUST | Type semantic distinctions, not only machine representations |
| DM-07 | MUST | Make validity rules explicit and enforce them at identified boundaries |
| DM-08 | MUST | Represent absence, unknowns, uncertainty, invalidity, and failure distinctly |
| DM-09 | MUST | Model relationships and valid domains explicitly |
| DM-10 | SHOULD | Keep important structure typed and queryable |

### Group 3 — Identity, versions, and consistency

| ID | Level | Title |
|---|---|---|
| DM-11 | MUST | Use stable semantic identity independent of physical location |
| DM-12 | MUST | Distinguish entity, revision, artifact, and execution identity |
| DM-13 | MUST | Separate definitions, specifications, policies, observations, and results |
| DM-14 | MUST | Publish semantically consistent revisions through explicit commit boundaries |
| DM-15 | MUST | Define canonicalization and equivalence before using content identity |

### Group 4 — Declarative composition and reusable structure

| ID | Level | Title |
|---|---|---|
| DM-16 | SHOULD | Represent material structure and policy as declarations |
| DM-17 | SHOULD | Use templates and bindings instead of copied construction logic |
| DM-18 | SHOULD | Preserve high-level structure until expansion is required |
| DM-19 | MUST | Select behavior through declared capabilities and explicit bindings |
| DM-20 | MUST | Make inspection and validation semantically non-mutating |

### Group 5 — Compilation, derivation, and semantic preservation

| ID | Level | Title |
|---|---|---|
| DM-21 | SHOULD | Use explicit intermediate representations and progressive lowering |
| DM-22 | MUST | Give every meaningful transformation a contract |
| DM-23 | MUST | Make derived representations traceable and non-competing |
| DM-24 | MUST | Preserve semantics across rewrites and lowerings |
| DM-25 | SHOULD | Unify operation contracts while allowing specialized implementations |

### Group 6 — Planning, execution, mutable state, and effects

| ID | Level | Title |
|---|---|---|
| DM-26 | SHOULD | Separate preparation from repeated execution |
| DM-27 | SHOULD | Represent important workflows as inspectable plans or state machines |
| DM-28 | MUST | Declare effects, ambient inputs, and nondeterminism |
| DM-29 | MUST | Isolate mutable workspaces and commit their outcomes explicitly |
| DM-30 | MUST | Make partial failure and recovery explicit |

### Group 7 — Dependencies, incrementality, and concurrency

| ID | Level | Title |
|---|---|---|
| DM-31 | MUST | Expose every dependency that can affect meaning or output |
| DM-32 | MUST | Key reuse to semantic dependencies rather than convenience |
| DM-33 | SHOULD | Invalidate at the smallest trustworthy semantic granularity |
| DM-34 | MUST | Keep different relationship structures semantically distinct |
| DM-35 | MUST | Make concurrency respect dependencies, ownership, and declared ordering |

### Group 8 — Execution representations and performance

| ID | Level | Title |
|---|---|---|
| DM-36 | SHOULD | Choose physical layouts for demonstrated access patterns |
| DM-37 | SHOULD | Cross expensive boundaries in coarse, typed units |
| DM-38 | SHOULD | Match the execution mechanism to the operation’s semantics |
| DM-39 | MUST | Evaluate performance end to end and distinguish evidence from expectation |
| DM-40 | MUST | Declare precision, approximation, ordering, and determinism requirements |

### Group 9 — Boundaries, providers, and extensibility

| ID | Level | Title |
|---|---|---|
| DM-41 | MUST | Keep adapters mechanical and domain conversions explicit |
| DM-42 | MUST | Make interchange loss-aware and reject silent semantic degradation |
| DM-43 | MUST | Negotiate capabilities and expose unsupported behavior |
| DM-44 | MUST | Make extensions complete, versioned, and conformance-testable |
| DM-45 | MUST | Treat trust and authority as explicit execution constraints |

### Group 10 — Provenance, reproducibility, and explainability

| ID | Level | Title |
|---|---|---|
| DM-46 | MUST | Preserve source-to-result lineage through transformations |
| DM-47 | MUST | Represent diagnostics as structured evidence |
| DM-48 | MUST | Define and support the required reproducibility contract |
| DM-49 | SHOULD | Make changes understandable at the level of meaning |
| DM-50 | SHOULD | Observe the model lifecycle, not only low-level operations |

### Group 11 — Evolution, generation, and verification

| ID | Level | Title |
|---|---|---|
| DM-51 | MUST | Evolve schemas and semantics through explicit migrations |
| DM-52 | SHOULD | Generate repeated mechanical artifacts from shared contracts |
| DM-53 | MUST | Verify invariants and equivalence across representations |
| DM-54 | MUST | Test adversarial lifecycle and boundary conditions |
| DM-55 | SHOULD | Make contracts and extension paths discoverable to humans and agents |

### Group 12 — Architectural leverage and disciplined improvement

| ID | Level | Title |
|---|---|---|
| DM-56 | SHOULD | Optimize for fewer independent semantic decisions—not fewer lines |
| DM-57 | SHOULD | Prefer a small coherent core with explicit extension mechanisms |
| DM-58 | SHOULD | Scale architectural machinery to demonstrated needs |
| DM-59 | MUST | Make design claims falsifiable and label uncertainty |
| DM-60 | MUST | Turn the principles into change-level review and regression controls |

### Scope-to-group routing

A starting orientation, not an allocation. It exists to help justify the §7 applicability note; the scope decides which groups actually matter.

| If the scope is… | Groups that often carry the findings |
|---|---|
| The canonical evidence model, a record type, or the schema registry (§6) | 1, 2, 3, 11 |
| A producer, normalizer, or the snapshot pipeline (§4, §5) | 5, 6, 7, 10 |
| Storage, Arrow projections, or DataFusion retrieval (§6.3, §8) | 3, 8, 9 |
| The Python boundary, a registry client, or a producer adapter (§5, §B2) | 9, 8, 5 |
| Freshness, caching, or job orchestration (§3.3, §8) | 7, 3, 10 |
| Execution policy, capsules, or the worker sandbox (§10) | 6, 7, 10, 9 |
| MCP tool contracts or the companion skill (§7, §11) | 4, 9, 12 |
| A refactor claiming leverage or simplification | 12, 4, 11 |

---

## §2 Lenses that tend to pay off

Nothing here is a required step. These are the angles that have repeatedly turned up real defects, and the note on what it takes for each to hold up as evidence rather than as a suspicion.

### When the subject is a document

**Reconstructing beats reading.** Building the template's tables yourself from the document — rather than checking whether the document's own version looks complete — tends to surface the gaps quickly, because every cell you have to invent is a decision the design hasn't made:

- The **authority table** (§2): one row per semantic fact, with owner, revision boundary, and permitted update path. The list of cells you had to invent *is* the evidence for the finding.
- The **invariant table** (§3), with an enforcement boundary and a failure behavior per invariant. An invariant with neither is unresolved (DM-07), whatever the surrounding prose claims.
- The **stage graph** (§4), with inputs, output contract, effects, and invalidation per stage. Undeclared effects and ambient inputs land under DM-28.
- The **absence lattice** (DM-08): for each value that can be missing, which of unspecified / unknown / not-applicable / uncertain / invalid / partial / failed does the design distinguish, and which collapse into one representation? States that collapse when callers need to tell them apart are a G2 concern. In this repo the lattice is partly fixed by the six epistemic classes (§6.2) — check that the design does not add a seventh informally.

**Sorting the load-bearing sentences** is a cheap way to keep claim strength honest:

| Kind | How it reads | What it deserves |
|---|---|---|
| Specification | States what holds, where enforced, what is rejected | Assess it directly |
| Intention | A desirable property with no mechanism — "the model is the single source of truth" | Unresolved until a mechanism is named |
| Assumption | Rests on an external system, library, or later decision | If not stated as an assumption, that's DM-59 |
| Benefit assertion | Performance, simplicity, extensibility | Hypothesis unless evidence is cited (DM-39, DM-59) — label it |

**The divergence sentence.** For a core mechanism, writing the one sentence two implementers would read differently is often the whole finding. Familiar shapes here: "identity is content-based" (over which canonical form, with which normalizations — DM-15, §6.3's rule that timestamps are excluded from content hashes); "stale evidence is refreshed" (against which of the four identities in §3.1, keyed on what — DM-32); "the result is partial" (partial coverage, a failed producer, or an unsupported capability — §6.2, DM-08); "the extraction is static" (no import, no build, or no network — §B5).

At `deep`, the counter-design is worth the time: the smallest alternative delivering the same observable outcome. If the proposal's extra machinery can't pay for itself against it, that's a DM-58 finding that usually outranks the local ones.

### When the subject is code

Six recurring defect shapes. The second column is what it takes for one to be reportable rather than suspected — below that bar, it belongs in the Method note as something you looked at and couldn't settle.

| Shape | What makes it evidence | Gate · principles |
|---|---|---|
| **Second authority** — the same semantic fact independently editable in two places (a Rust wire type and a hand-written Pydantic DTO; a schema and a validator; a normalizer and a query projection; a production default and a fixture encoding the same rule) | Both sites cited, plus the absence of a derivation, generation step, or assertion linking them. Two representations derived from one source are not a second authority — check for the generation step first (`just schemas-generate`) | G1 · DM-02, DM-23 |
| **Unguarded boundary** — an external input, partial construction, or deserialization reaching an operation whose correctness assumes an invariant | The entry point, the operation, and either no check between them or one that is advisory (logs, warns, opt-in) rather than rejecting. Type-level enforcement counts; say so and close it | G3 · DM-07, DM-22 |
| **Hidden effect** — `validate`, `check`, `inspect`, `plan`, `explain`, `search` paths that mutate, populate caches, lazily fetch, register, or read ambient state (clock, env, filesystem, network, global config) | The mutation or ambient read cited, plus a caller that reasonably assumes purity. A read path that silently triggers a fetch is the shape to look for here. Memoization that can't change observable semantics isn't a finding — but say why you concluded that | G4 · DM-20, DM-28 |
| **Silent degradation** — default-on-absence, catch-alls flattening distinct failure classes, unsupported-producer branches with different semantics, an `ok` envelope for a partial result | The branch, what the caller observes in the degraded versus supported case, and no declared loss or coverage statement. `ok` means successful *within the declared scope*; an `ok` that hides a producer gap is this shape | G2/G7 · DM-08, DM-42, DM-43 |
| **Incomplete reuse key** — a cache, snapshot, or content key missing something that can change the result: normalizer version, producer identity, execution policy, feature selection, resolved dependency digest, schema revision | The key construction, the omitted dependency, and a situation where changing it yields a stale hit. Volatile inputs in the key — wall-clock timestamps, temporary capsule paths — are the mirror-image defect (§6.3 forbids them explicitly) and also DM-32 | G6 · DM-31, DM-32 |
| **Unbacked capability** — the accepted-operation surface (an MCP tool's declared parameters, a producer's advertised kinds, a supported-ecosystem list, a config schema) wider than the implementation that handles it | The set difference, with the accepted variant and the missing or fallback handling both cited. An explicit typed `Unsupported`, or a `blocked` gate naming its prerequisite, is aligned; a silent fallback to different semantics is G7. Documentation of coverage is the claim, not the proof | G7 · DM-43, DM-44 |

Tracing the path that executes — the `impl` actually selected rather than the trait's doc comment, the branch taken under the real configuration — is what makes these hold. Where dispatch is dynamic or config-dependent, saying which path was traced and which wasn't keeps the coverage note honest.

### When the subject is both

Beyond what each half yields:

| Question | Finding shape |
|---|---|
| Do `DESIGN.md`'s semantics and the implementation's match? | Divergence against the pair; name which is authoritative and how they reconcile (DM-02) |
| Does `DESIGN.md` describe a state the code has passed through? | Stale specification — say whether the section is normative-forward (a target for a later phase) or descriptive (a record); one that is neither is a competing authority |
| Does `DESIGN.md` disagree with the frozen blueprint? | That is legitimate only where an ADR records the deviation. An undocumented divergence is a finding, and its correction is an ADR, never an edit to the blueprint |
| Does the code implement semantics `DESIGN.md` omits? | Undocumented surface; DM-04, DM-55 |
| Does the document claim capabilities the code lacks? | G7 — narrow the claim or relabel it Proposed |
| What evidence label does each design claim now deserve? | Re-label per charter §D. A Proposed claim becoming Implemented is often this mode's most useful output |

---

## §3 Finding calibration

Adequate and inadequate versions of the same observation. The difference is consistently the same three things: a concrete consequence, evidence at the right grain, and citations that do work. In this repo there is a fourth: the **Verification** column must name an executable oracle or say that none exists.

### A — Authority

**Inadequate.** "Violates DM-02: the schema definition appears in multiple places, which creates a single-source-of-truth problem. Recommend consolidating."

No sites cited, no demonstration that the two can drift, no consequence, and "consolidating" isn't a direction with a surface area.

**Adequate.** "The set of required envelope fields is independently editable in two places: the Rust wire type at `crates/enrichment-core/src/wire/schema.rs:88` and the hand-written check in the Python adapter at `python/enrichment_mcp/envelope.py:41`, which re-lists them as string literals. The Python side is not generated from the schema and no test compares them. **Consequence:** adding an optional field on the Rust side without editing the adapter makes the adapter reject valid envelopes; relaxing the adapter without editing the Rust type admits envelopes the daemon will refuse at `server.rs:131`. **Correction:** derive the adapter's field set from the generated schema (~2 call sites), or extend `just schemas-generate` to emit it. **Verification:** a contract test asserting set equality between the generated schema's required fields and the adapter's; it fails today. DM-02, DM-07, G1."

### B — Absence semantics

**Inadequate.** "DM-08 is not fully satisfied; the design should distinguish absence states more clearly."

**Adequate.** "A symbol absent from the index, a symbol whose producer failed, and a symbol outside the requested context all return an empty `symbols` array with `status: ok`. **Consequence:** the companion skill's caller cannot separate 'this capability does not exist' from 'extraction did not run', so an absent result reads as proof of absence — exactly the inference AGENTS.md forbids. **Correction:** carry the producer's coverage and gaps into the envelope beside the results, a one-declaration change while the envelope has one consumer. **Verification:** a contract test asserting the three cases are distinguishable at the MCP boundary; and an `ast-grep` rule in `rules/` forbidding an `ok` envelope constructed without a coverage field. DM-08, DM-30, G2."

### C — Over-construction

**Inadequate.** Silence — or praise for extensibility. This is the finding that most often goes unwritten.

**Adequate.** "§X introduces a producer registry with dynamic capability negotiation, but the design names exactly one producer for the ecosystem and no near-term second. The registry adds a capability-declaration surface, a negotiation protocol, and a conformance-test obligation (DM-44) the design doesn't fund. **Consequence:** the first genuine second producer will need capability distinctions the current vocabulary can't express, so the registry gets rewritten rather than extended — the cost paid twice. **Correction:** keep the trait boundary and the typed `Unsupported` result (the cheap, justified seam); defer the registry and protocol until a second producer exists. **Verification:** none needed — this removes machinery. DM-58, DM-57, charter §F, 'speculative flexibility with no demonstrated consumer'."

### D — Evidence labels

**Inadequate.** "The design is validated end to end — see the integration tests."

**Adequate.** "Round-trip guarantee: **Tested** for the structural case (`just schema-conformance`, N fixtures, behavioural equality against the frozen contract) and **Proposed** for the semantic case — nothing exercises metadata preservation or epistemic-class identity across the Arrow projection, which §6.3 claims is preserved. Either narrow the claim to structural round-trip or add the metadata case before making it. DM-53, DM-59."

Note the three vocabularies stay separate: this is a charter §D label on a *design claim*. The gate it corresponds to is `passed`/`failed`/`blocked`/`not_run`, and neither is an epistemic class. See [`ADDENDUM.md`](ADDENDUM.md).

---

## §4 How the sections tend to compress

Template sections are `DESIGN_REVIEW_TEMPLATE.md` §1–§11. A sketch of what usually survives, not a rule — the scope decides.

| Section | Document subject | Code subject | At `compact` |
|---|---|---|---|
| §1 Decision and scope | Plus Method and coverage | Plus Method and coverage | Compressed; Method note still earns its place |
| §2 Authority and lifecycle | Reconstructed; invented cells marked | Reconstructed; where each authority lives | Prose, with the authority gaps named |
| §3 Contracts and invariants | Usually the core of the review | Enforcement sites cited | Often merges into §7 |
| §4 Derivation and execution | Full | Tracing real paths | Prose |
| §5 Journeys | Extension and failure at minimum | Same, through real code | One journey, chosen for relevance |
| §6 Gates | Tabular | Tabular | Tabular |
| §7 Findings + applicability | Tabular | Tabular | Tabular |
| §8 Alternatives | Worth the work at standard/deep | Worth the work at standard/deep | Optional; say why omitted |
| §9 Verification plan | Proposed checks | Existing coverage and gaps, named | Top gaps only |
| §10 Exceptions | Only if deviations exist | Only if deviations exist | Only if deviations exist |
| §11 Decision and changes | Required | Required | Required |

---

## §5 Appendix — where these shapes tend to appear in this repository

Illustrative. The charter is technology-neutral and so is the review; this is a list of places the shapes in §2 have a habit of landing here, and their absence proves nothing. The right-hand column names the enforcement tier that would catch a regression, because §7's Verification column has to name one.

| Pattern | Shape | Principles · gate | Likely oracle |
|---|---|---|---|
| A Pydantic DTO or envelope field hand-written rather than generated from the Rust wire type | Second authority | DM-02, DM-52 · G1 | `just schema-conformance`; an `ast-grep` rule over `python/enrichment_mcp/` |
| A normalizer and a DataFusion projection each deciding what a field means | Second authority | DM-02, DM-23 · G1 | a round-trip test over the Arrow projection |
| `status: ok` returned for a result whose producer reported gaps | Silent degradation | DM-08, DM-43 · G2/G7 | a contract test; `rules/` entry on envelope construction |
| An empty result set presented, or consumed, as proof a capability is absent | Silent degradation | DM-08, DM-59 · G2 | a contract test asserting coverage accompanies emptiness |
| Six epistemic classes collapsing to a confidence score, or a stub annotation overwriting a `runtime_observed` fact | Silent degradation | DM-08, DM-13 · G2 | `rules/` entry; `.claude/rules/evidence-truthfulness.md` names the invariant |
| Wall-clock timestamps or capsule paths inside a content hash (§6.3 forbids it) | Incomplete reuse key, inverted | DM-15, DM-32 · G6 | a determinism test: hash twice across two runs |
| A snapshot key omitting normalizer version, producer identity, or execution policy | Incomplete reuse key | DM-31, DM-32 · G6 | a staleness test that changes one input and asserts invalidation |
| Griffe loaded without `allow_inspection=False`, or an extraction path that imports the library under study | Hidden effect, unguarded boundary | DM-28, DM-45 · G4/G3 | `rules/griffe-static-only.yml` (exists) |
| A read or search path that silently triggers a network fetch or a build | Hidden effect | DM-20, DM-28 · G4 | a test under a no-network execution profile |
| A subprocess whose working directory, extraction target, or install destination is a repository under study | Hidden effect, trust | DM-28, DM-45 · G4 | gate C20; `just state-leak-check` |
| Downloaded or extracted content reaching a path that treats it as instructions rather than evidence | Trust boundary | DM-45 · G3 | `.claude/rules/execution-policy.md`; a negative test |
| `unwrap` / `expect` at an NDJSON-RPC, registry-response, or archive-extraction boundary | Unguarded boundary | DM-07, DM-42 · G3 | a malformed-input test; clippy |
| An MCP tool declaring parameters or kinds the daemon does not implement | Unbacked capability | DM-43, DM-44 · G7 | `just schema-conformance`; a catalog contract test |
| A producer advertising an ecosystem whose path is only partly implemented | Unbacked capability | DM-43 · G7 | a gate that reports `blocked` with the prerequisite named |
| `print(...)` on stdout anywhere in the MCP adapter or worker | Silent degradation of the protocol channel | DM-42 · G2 | `rules/mcp-stdout-protocol-only.yml` (exists) |
| A gate marked `passed` without a recorded command and log | Unbacked capability | DM-59 · G7 | `just acceptance-check`; `stop_guard.sh` |
| A mock standing in for a real client in a contract, integration, or e2e tier | Unbacked capability | DM-59 · G7 | `rules/no-mocks-in-acceptance-tiers.yml` (exists) |
| A benchmark or probe measuring one producer while the claim is end-to-end | — | DM-39, DM-59 | name the conditions, or relabel the claim |
| A finding with no oracle at any of the four tiers | — | DM-60 | **say so.** That absence is the most valuable output; it becomes a new `rules/` entry |
