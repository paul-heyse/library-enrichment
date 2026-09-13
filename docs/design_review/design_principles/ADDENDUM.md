# Charter addendum — library-enrichment

The charter, the directive and the review template are technology-neutral and are carried here
**verbatim**, so that a `DM-nn` or `Gn` citation means the same thing in this repository as it
does anywhere else the charter is used. This file is the only repo-specific layer. It says three
things, and nothing else:

1. which binding decisions the charter's gates bear on — the **§B-numbers**;
2. how the blueprint's own ten-question design-review checklist routes onto **G1–G7**;
3. how the three grading vocabularies this repo now carries stay distinct.

---

## 1. Binding decisions have stable IDs

`docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` §1.1 is a table of binding decisions with no
citable identity per row, so an ADR has had to quote a row's prose to refer to it. The rows get
stable IDs here. They are **citation targets, not new decisions** — §B*n* means exactly what the
blueprint row says, and `docs/design/DESIGN.md` carries each one as a section of the same number.
IDs are never reused or renumbered.

| ID | Binding decision | Source | Bears on |
|---|---|---|---|
| §B1 | **Repository boundary.** One standalone repository; no service code, environments, caches or generated indexes inside working repositories. | blueprint §1.1 | DM-28, DM-29, DM-45 · G4 |
| §B2 | **Core ownership.** Rust owns identities, resolution, fetching, normalization, evidence storage, querying, job state, policy and publication. | blueprint §1.1 | DM-02, DM-13, DM-23 · G1 |
| §B3 | **Python boundary.** A thin FastMCP adapter, a separate Griffe extraction worker, isolated runtime probes. Never a reimplementation of the core. | blueprint §1.1 | DM-41, DM-42, DM-05 · G1, G2 |
| §B4 | **Python semantic engine.** Astral **ty**, through its CLI/LSP boundary, not private internals. | blueprint §1.1 | DM-19, DM-43 · G7 |
| §B5 | **Rust semantic engine.** rust-analyzer over LSP, isolated behind a producer adapter. | blueprint §1.1 | DM-19, DM-41, DM-43 · G7 |
| §B6 | **Context7.** A separate MCP connection used directly by the calling agent. No proxy, no embedded LLM, no recursive research agent. | blueprint §1.1 | DM-04, DM-58 · G4 |
| §B7 | **Storage.** Immutable raw artifacts and Arrow/Parquet evidence snapshots; DataFusion for structured retrieval. An evidence cache, not a future fact store. | blueprint §1.1 | DM-14, DM-23, DM-29, DM-32 · G1, G5, G6 |
| §B8 | **MCP transport.** Per-client stdio adapters attached to one local Rust daemon. HTTP is an optional later profile. | blueprint §1.1 | DM-37, DM-42 · G2 |
| §B9 | **Design inference.** Belongs in the calling agent's evidence-backed brief, never silently promoted to extracted fact. | blueprint §1.1 | DM-08, DM-13, DM-59 · G2, G7 |
| §B10 | **Completion.** Real Rust and Python fixtures must work end to end, including tests of incomplete evidence and failure paths. | blueprint §1.1 | DM-30, DM-53, DM-54 · G3, G5 |
| §B11 | **Excluded mechanisms.** No graph database, embeddings, vector database, GPU workload, generic workflow engine, raw-SQL or arbitrary-shell MCP tool, whole-repository code-property graph, or automatic project edits. | blueprint §1.1 (closing paragraph) | DM-57, DM-58 · — |
| §B12 | **Static extraction.** Griffe with `allow_inspection=False` and explicit `search_paths`; extraction never imports the library under study. | blueprint §5.2; `AGENTS.md` | DM-20, DM-28, DM-45 · G4, G3 |
| §B13 | **Rust API source.** Hosted rustdoc JSON from docs.rs is tried before any local compilation. | blueprint §4.1; `AGENTS.md` | DM-19, DM-31 · G6 |

### A known divergence between the two binding-decision tables

`AGENTS.md` carries a nine-row summary of the binding decisions. It is **not** the same set as
blueprint §1.1: it adds §B12 and §B13 (which live in §4 and §5 of the blueprint, not §1.1), and
it omits §B1, §B9, §B10 and §B11. Both documents are read as normative today, which is a second
authority for the same fact (DM-02).

The §B-set above is the union, and `docs/design/DESIGN.md` is where it becomes single-sourced:
§B1–§B13 each appear there as a section, and that is the citation target an ADR's `design:`
field resolves against. Reconciling `AGENTS.md`'s table is an operator change — it is part of the
enforcement layer and is not editable from a session — and until it lands, a reviewer should
treat a disagreement between the two tables as a finding, not as an ambiguity to resolve
silently.

---

## 2. The blueprint's own checklist, routed onto the charter's gates

Blueprint §15 is a ten-question design-review checklist. It predates this charter, is frozen,
and was wired to nothing. It is not superseded — it is the repo's own statement of what matters,
and it routes onto G1–G7 like this. A review that settles the gates has answered the checklist;
one that answers the checklist without settling the gates has not.

| # | Blueprint §15 question | Gate | Principles |
|---|---|---|---|
| 1 | Does the agent request semantic research operations, or must it orchestrate raw shell commands? | — | DM-16, DM-55, DM-57 |
| 2 | Can every material returned fact be traced to exact source content and an extraction/verification environment? | G1 | DM-46, DM-48, DM-31 |
| 3 | Can the system find an unfamiliar capability without dumping the entire API into the context window? | — | DM-18, DM-50 |
| 4 | Are declared availability, configured availability, type-check success and runtime behavior separate states? | **G2** | DM-06, DM-08, DM-13 |
| 5 | Does each producer add typed evidence without becoming another publication authority? | **G1** | DM-02, DM-22, DM-23 |
| 6 | Can a caller get a useful partial answer without pretending the evidence is complete? | **G5, G7** | DM-08, DM-30, DM-59 |
| 7 | Are project code, service code, service environments and retained evidence physically separated? | **G4** | DM-28, DM-29, DM-45 |
| 8 | Does a new capability require a small producer/projection extension rather than a bespoke pipeline? | — | DM-44, DM-56, charter §E |
| 9 | Does the implementation work without embeddings, a graph, an embedded LLM, or a new general-purpose server framework? | — | DM-57, DM-58 |
| 10 | Can today's evidence bundles become tomorrow's graph evidence without changing their source identities? | **G1** | DM-11, DM-12, DM-51 |

Questions 1, 3, 8 and 9 have no gate: they are proportionality and ergonomics tests, and the
charter deliberately keeps those out of the gates so that a design cannot fail a correctness gate
and be rescued by being pleasant to use — or the reverse. They belong in §7 findings and §8
alternatives.

G3 and G6 have no §15 question. That is a gap in the checklist, not in the design: nothing in
§15 asks whether an invalid state can reach an operation that assumes validity, or whether a
cache hit can change an answer. Both are live risks for an evidence service with snapshots and
content keys, so a review here settles them on the charter's terms.

---

## 3. Three vocabularies, kept distinct

This repository now carries three grading scales. They answer different questions and are never
interchangeable. Conflating them is exactly the class of defect the charter exists to catch
(DM-08, DM-59), so it is worth stating flatly:

| Vocabulary | Grades… | Values | Defined in |
|---|---|---|---|
| **Charter §D evidence labels** | the strength of a **design claim** | `Proposed` · `Interface-checked` · `Implemented` · `Tested` · `Measured` · `Formally established` | `DATA_MODEL_DESIGN_CHARTER.md` §D |
| **Acceptance gate states** | the outcome of an **acceptance gate** in `tests/gates.toml` | `passed` · `failed` · `blocked` · `not_run` | `AGENTS.md`; blueprint §13 |
| **Epistemic classes** | the provenance of a **runtime evidence record** | `declared` · `statically_extracted` · `compiler_derived` · `typechecker_observed` · `runtime_observed` · `agent_inferred` | blueprint §6.2 |

A gate is never `Measured`. A design claim is never `not_run`. Neither is ever an epistemic
class. An evidence record is never `passed`.

The rules that already govern the second and third are unchanged, and the charter adds nothing to
them. In particular:

- A gate is `passed` only when a command ran and its log is recorded; a missing tool is
  `blocked` with the prerequisite named; never attempted is `not_run`. A mocked client is never
  a pass. Gate results are never converted into a quality percentage.
- `ok` on an evidence envelope means successful **within the declared scope**, not complete
  knowledge. An empty result is not proof that a capability is absent. A stub annotation never
  silently overwrites a `runtime_observed` fact.

What the charter adds is the first row: design claims in `DESIGN.md`, in an ADR's `evidence:`
field, in a plan's verification section, and in a register row now carry a §D label. `Proposed`
is not a failure state. **An unlabelled claim is.**

---

## 4. Findings must name an oracle

`AGENTS.md` sets out four enforcement tiers, chosen by asking what the cheapest reproducible
oracle is: a **hook** when the mistake is visible in the tool call alone; an **`ast-grep` rule**
in `rules/` with fixtures in `rule-tests/` when it is a code shape; a **`just` gate** when it
needs the whole repo; and **prose** only when there is no mechanical oracle.

Every §7 finding's *Verification* column names one of those, or says explicitly that no oracle
exists. That is not bookkeeping: an unenforced correction regresses, and the absence of an oracle
is the most useful thing a review can report, because `rules/` is where it lands and adding a
rule there is deliberately frictionless.

A hook that would have to parse a source file is the wrong tier — that is a rule.
