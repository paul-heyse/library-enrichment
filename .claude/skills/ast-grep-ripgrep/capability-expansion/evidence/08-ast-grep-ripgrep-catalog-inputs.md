# Evidence 08 — What the existing skill already supplies to the catalog

**Source:** the `ast-grep-ripgrep` capability repository itself (ast-grep 0.45.3, ripgrep 15.2.0,
PCRE2 10.48 asserted, 23 library crates), read 2026-09-16.

The proposal's §5 says "extraction strategy: native structured sources first". A great deal of
that extraction **already exists** in this skill and does not need rebuilding — it needs
*promoting* from flat TSV projections into the typed relational model. This dossier inventories
what is there and maps each table onto the canonical records.

---

## 1. Inventory of existing index tables

| Table | Rows | Cols | Canonical destination |
|---|---:|---:|---|
| `flags` | 193 | 8 | `Mechanism` (surface = CLI), `Parameter` |
| `rule-fields` | 70 | 5 | `Mechanism` (surface = ast-grep rule), `Parameter` |
| `regex` | 44 | 9 | `Mechanism` (surface = pattern syntax), with `Contract` facets |
| `kinds` | 3,024 | 4 | grammar metadata — target-language node kinds |
| `fields` | 631 | 2 | grammar metadata — node fields |
| `languages` | 55 | 5 | grammar/language roster, with per-language rule schema |
| `file-types` | 224 | 2 | `SearchDomain` — the corpus-selection contract |
| `exit-codes` | 9 | 5 | `ResultContract` — how a run reports itself |
| `behaviors` | 49 | 10 | `BehaviorAssertion` + `Evidence`, already probe-and-control backed |
| `unreachable` | 10 | 5 | `not_supported_through_this_surface` rows |
| `symbols` | 716 | 8 | implementation evidence — `Definition` |
| `methods` | 4,727 | 5 | implementation evidence — `Signature` |
| `impls` | 2,241 | 3 | implementation evidence — `Implementation` |
| `aliases` | 273 | 3 | implementation evidence — `ExportPath` |
| `bindings` | 67 | 6 | the napi/pyo3 surfaces — a distinct `SurfaceBinding` role |
| `unresolved` | 5 | 1 | boundary of the indexed set |

**Roughly 12,000 rows of extracted, pinned, verified fact already exist.** The capability
expansion is therefore not a green-field extraction project for the tool surface; it is a
re-typing and enrichment project, with green-field work concentrated in the seven Rust families
(evidence 06) and in the contract/interaction layer the proposal calls for.

---

## 2. The three tables that carry contract semantics already

### `regex` (44 rows, 9 columns)

```
construct · syntax · rust_regex · pcre2 · reachable · since_pcre2 · probe · observed · purpose
```

`rust_regex` and `pcre2` are per-engine support columns and `reachable` is CLI reachability —
this is already the proposal's distinction between "available in a dependency" and "available
through the command the agent is about to run". `observed` takes `confirmed` / `recorded` /
`unknown` / `not-probed`, which is the proposal's coverage vocabulary in miniature.

**Mapping:** `construct` → `Mechanism`; `(rust_regex, pcre2)` → two `MechanismEffect` rows keyed
by engine; `reachable` → a `SurfaceBinding`; `observed` → `Precision` (evidence 03 §4);
`probe` → `Evidence.evidence_id`.

### `behaviors` (49 rows, 10 columns)

```
probe · tool · topic · verdict · exit · question · command · stdout · control · control_exit
```

This is already `BehaviorAssertion` + `Evidence` in one row, **with a control**. The proposal's
§8 "Behavioral validation and coverage accounting" asks for focused probes rather than an option
powerset; that discipline is established here, including the rule that a probe whose control did
not discriminate is not evidence.

**Mapping:** split into `BehaviorAssertion` (subject, predicate, value, applicability, status) and
`Evidence` (run, locator, observation_kind), joined by `probe`. The `control` and `control_exit`
columns become a second `Evidence` row with `observation_kind = 'control'` — which is what lets a
query ask "which assertions rest on an uncontrolled probe".

### `flags` (193 rows, 8 columns)

```
tool · command · long · short · category · arg · values · summary
```

`category` is upstream ripgrep's **own** functional grouping, taken from `rg --help`. That is a
ready-made, non-invented capability axis — the proposal's §2 warns against building an
exhaustive hierarchy of prewritten use cases, and using the tool's own categories avoids
inventing one.

---

## 3. What the skill deliberately does not claim

From its `reference.md` known limits, each of which becomes a `not_supported_through_this_surface`
or `uncharacterized` row rather than a silent gap:

1. `rg --pcre2-version` understates the linked runtime by three releases; four readings are kept
   side by side in `PROVENANCE.json` and none is reconciled away.
2. `regex.tsv` covers the constructs an agent asks about, not every construct PCRE2 has.
   "A construct absent from the index is unindexed, not unsupported."
3. `unknown` is a real value — rows without a probe say so.
4. Node kinds describe the grammars **this** ast-grep bundles; three kinds published in
   `languages.json` are rejected by this binary.
5. The language roster is what this binary accepted when probed.
6. Flags come from the installed binaries' own help.
7. The binding index is what the declarations declare, not what resolves at runtime.
8. `unresolved.tsv` is a boundary, not a gap.

**Limit 2 is the one the expansion must address**, because a capability-selection system that
silently omits constructs is worse than one that says it omits them. The plan therefore carries a
`coverage` relation with an explicit denominator — see the plan's coverage-accounting section.

---

## 4. The identity problem this creates

The existing tables use tool-natural keys: a flag is `(tool, command, long)`, a construct is
`construct`, a probe is `probe`. The canonical model needs stable `mechanism_id`s that survive a
tool upgrade.

The proposal (§11 "First: establish the snapshot, inventory, and schemas") requires "stable
mechanism IDs". Evidence from this skill sharpens the requirement: flags move between categories
between ripgrep releases, and node kinds change when ast-grep updates a grammar. So
`mechanism_id` must be minted from a **surface-qualified natural key** and carried in
`NativeBinding` alongside the per-snapshot native handle, exactly as the Rust families do.

---

## 5. What the expansion must add

| Proposal concept | Present today | Gap |
|---|---|---|
| `Capability` (abstract ability) | — | entirely new; the axis above `Mechanism` |
| `Mechanism` | implicit in `flags`/`rule-fields`/`regex` | needs one identity space across three surfaces |
| `Contract` (input/output/semantic limits) | partial (`regex.reachable`, `exit-codes`) | the typed facet structure of proposal §2 |
| `Interaction` (option×option effects) | — | entirely new; proposal §3 |
| `PlanFragment` | — | entirely new; proposal §10 |
| `SearchDomain` | `file-types`, some flags | needs the traversal/ignore-layer contract |
| `ResultContract` | `exit-codes`, some flags | needs per-output-mode shape |
| `Evidence` / `BehaviorAssertion` | `behaviors` (49 rows, controlled) | re-typing, plus coverage denominator |
| implementation evidence graph | `symbols`/`methods`/`impls`/`aliases` | joining to the seven Rust families |

## 6. Open questions for round 2

1. Whether the existing 49 behaviour probes can be re-expressed as `BehaviorAssertion` rows
   without loss, or whether the probe schema needs a field the TSV does not carry.
2. Whether `kinds` (3,024 rows) should be loaded wholesale or referenced from the skill, given
   that it changes on every ast-grep grammar update and is target-language metadata rather than
   mechanism metadata.
3. Whether `bindings` (napi/pyo3) is in scope at all for round one — the proposal's §1 says to
   prioritise the actual invocation surfaces and treat Rust APIs as implementation evidence.
