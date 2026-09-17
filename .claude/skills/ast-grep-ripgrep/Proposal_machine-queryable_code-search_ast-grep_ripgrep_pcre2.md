Proposal: machine-queryable code-search capability selection for ast-grep and ripgrep including PCRE2 10.48 as exposed through ripgrep

**Build a single-snapshot capability-selection system for ast-grep and ripgrep, including PCRE2 10.48 as exposed through ripgrep.** Its principal artifact should be a catalog of **precise, composable search contracts**, supported by native schemas, documentation, behavioral tests, and a deep implementation-evidence graph.

The intended interaction is:

```text
Functional requirement
    ↓
Relevant capabilities and concrete mechanisms
    ↓
Discriminating choices, required options, and interactions
    ↓
Input/output contracts and semantic limitations
    ↓
Query construction by the programming agent
```

The system should supply the information needed to make a decision, rather than attempt to anticipate every use case or autonomously choose every query.

**Preserve depth in the underlying representation; minimize the amount that must be retrieved for an ordinary decision.**

---

## 1. Scope and architectural boundaries

### Analyze one selected tool configuration

Create one coherent snapshot containing the selected ast-grep and ripgrep implementations, the selected language grammars, and the PCRE2 10.48 integration.

Retain an artifact-level manifest identifying source revisions, executable versions or fingerprints, grammar revisions, and extraction formats. Bind PCRE2 sources and documentation to the `pcre2-10.48` release rather than an unversioned manual. The upstream release provides a concrete tag for this purpose. 

This manifest establishes what the catalog describes. It is **not** a compatibility matrix or an additional dimension the agent must routinely consider.

Exclude build-variant enumeration, MSRV characterization, supply-chain analysis, and system-level installation requirements. However, keep runtime query choices—engine selection, matching modes, traversal boundaries, output modes, file-selection behavior—because these determine search semantics within the selected configuration.

### Distinguish capability analysis from target-code analysis

Maintain two separate concepts:

```text
Implementation evidence:
    analysis of ast-grep, ripgrep, and their dependencies

Search semantics:
    what those tools can establish about a searched repository
```

A complete graph of ast-grep’s implementation does not give ast-grep resolved-symbol or data-flow knowledge about the repository being searched. Ast-grep explicitly excludes scope, type, control-flow, and data-flow analysis from its supported analysis model. 

Consequently, requests should distinguish:

```text
textual equality
syntactic structure
syntactic containment
resolved symbol identity
type relationships
control-flow relationships
data-flow relationships
```

A mechanism’s support for a request should be classified as:

```text
exact_under_stated_preconditions
candidate_generation_or_approximation
not_supported_through_this_surface
uncharacterized
```

For approximations, record whether recall is established, heuristic, or unknown. “Candidate generation” should not silently imply that every true result will be included.

### Prioritize the actual invocation surfaces

The first agent-facing catalog should emphasize ast-grep CLI/rule configuration and ripgrep CLI/pattern syntax. Rust APIs should primarily support implementation evidence; expose them as selectable mechanisms when the consuming agent explicitly requests programmatic integration.

This avoids mixing “available in a dependency’s Rust API” with “available through the command the agent is about to run.”

---

## 2. Canonical model: capabilities, mechanisms, and contracts

The central distinction is:

```text
Capability = an abstract querying ability

Mechanism = a concrete way to exercise that ability

Query = the agent’s application of mechanisms to a particular problem
```

For example:

```text
Capability:
    constrain a target by contained syntax

Mechanism:
    ast-grep rule.has

Query:
    select an enclosing construct containing a particular call pattern
```

### Represent problems through composable facets

Do not build an exhaustive hierarchy of prewritten use cases. Represent requirements through orthogonal fields:

```text
subject
    text, source code, paths, captured text

required_relationship
    equality, ordering, containment, exclusion, repetition, identity

requested_result
    existence, paths, matching text, spans, captures, enclosing nodes

semantic_requirement
    lexical, syntactic, resolved-symbol, type-aware, flow-sensitive

scope
    repository subset, file, line, multiline subject, syntax subtree

language
    selected source language and relevant grammar concepts

interface
    CLI, native rule configuration, Rust API

constraints
    recall requirement, accepted approximation, output requirements

workload
    optional execution characteristics relevant to choosing mechanisms
```

Natural-language descriptions and example tasks become retrieval aliases over these facets. They help an agent discover mechanisms without becoming a brittle ontology of every conceivable task.

### Use a small set of canonical entities

The core catalog should contain:

```text
Capability
MechanismContract
Parameter
Interaction
ResultContract
GrammarFragment
PlanFragment
Assertion
Evidence
SurfaceEntry
```

`SurfaceEntry` is the inventory item discovered upstream: a flag, rule field, enum choice, pattern construct, or other exposed feature. Its mapping to contracts provides coverage accounting.

`Assertion` is a scoped statement about behavior. `Evidence` records what supports or contradicts that statement.

### Mechanism contract

The following is a proposed internal shape, not an upstream API:

```text
MechanismContract
  id
  capability_ids
  surface
  native_schema_ref

  semantics
    subject
    semantic_layer
    matching_relation
    target_and_context_roles
    binding_and_capture_behavior
    guarantees
    non_guarantees

  configuration
    parameter_refs
    effective_defaults
    interaction_refs
    invocation_form

  results
    result_contract_ref

  selection
    applicable_requirements
    distinguishing_alternatives
    relevant_cost_conditions

  composition
    accepted_inputs
    produced_outputs
    plan_fragment_refs

  characterization
    assertion_refs
    unresolved_questions
    coverage_status
```

The native schema should remain authoritative for parameter structure. The contract adds what that schema usually lacks: meaning, interactions, selection conditions, and limitations.

**Do not normalize away distinctions merely because two tools expose similarly named options.**

---

## 3. Make interactions and effective behavior machine-evaluable

A list of options is not enough. The catalog must represent how options affect each other and the conditions under which they matter.

### Typed interaction records

Use explicit relationships such as:

```text
requires
enables
has_no_effect_unless
changes_meaning_of
overrides
accumulates_with
conflicts_with
consumes_binding_from
equivalent_under
```

An interaction should have a predicate, an effect, and supporting assertions:

```text
Interaction
  id
  applies_when
  effect
  affected_mechanism_or_parameter
  ordering_requirement
  assertion_refs
```

Implement the predicates using a small typed expression model over request facets, selected options, and effective context. Do not leave them as arbitrary prose strings, and do not introduce a general-purpose theorem prover.

Evaluate conditions using **true, false, and unknown**. An unknown condition should become an explicit unresolved choice in retrieval, not be silently treated as false.

### Preserve ordering and binding semantics

Ast-grep documents that rule order can matter when metavariables and relational rules interact. Its composite rules operate on a target node; they are not unrestricted joins among independently matched nodes. Preserve ordered rule sequences and binding dependencies rather than assuming ordinary commutative Boolean semantics. 

Likewise, preserve CLI argument order where it affects effective configuration. Do not flatten every invocation into an unordered map.

### Encode operationally important distinctions

For ast-grep, `has` with its default traversal boundary and `has` with `stopBy: end` are meaningfully different choices. The relational-rule model must also identify the target returned versus the surrounding or contained node used to establish the relationship. 

For ripgrep, separately characterize:

```text
whether a match may span input lines
whether "." matches newlines
how line anchors behave
```

The implementation applies multiline and dot-all settings through distinct configuration paths. These should not collapse into one generic `multiline` field. 

Regex dialect must also be explicit. Ast-grep’s `regex` rule uses Rust regex syntax, rather than PCRE2 syntax; similarly named fields do not authorize transferring lookaround or backreference constructs between them. 

### Model application defaults, not just dependency defaults

Store three layers when relevant:

```text
declared library default
application override
effective behavior on the selected surface
```

For example, `grep-pcre2`’s builder documents UTF and Unicode-property modes as disabled by default, while ripgrep enables both unless Unicode handling is disabled. The default the agent needs is the effective ripgrep behavior. 

Likewise, characterize `--engine=auto` operationally: ripgrep first attempts its default matcher and falls back to PCRE2 if that construction fails. Do not describe it as a general performance optimizer. 

---

## 4. Treat input scope and output semantics as part of every contract

A search contract must state both **what was searched** and **what the result represents**.

### Search-domain contract

Record the effective corpus-selection assumptions relevant to the request:

```text
roots and explicit paths
file-type and glob restrictions
ignore handling
hidden/binary treatment
input decoding or transformation
excluded or unreadable inputs
early termination or result limits
```

The purpose is not to reproduce the full file-discovery manual on every request. It is to prevent the agent from interpreting a result more broadly than the actual search domain permits.

“No matches” must remain distinguishable from “the requested corpus was completely searched and the condition was absent.”

### Result contract

Each mechanism or output mode should specify:

```text
returned unit and cardinality
whole-match versus capture semantics
target versus contextual/witness nodes
offset units and coordinate origin
path representation
original versus transformed input
ordering and duplicate behavior
context records versus matches
completion, error, and truncation status
```

Ripgrep’s JSON format illustrates why this matters: its `submatches` describe matches within reported text, with byte-relative offsets, and arbitrary data may be represented as text or base64 bytes. That is not automatically a named-capture-group object. 

Keep transformation outcomes separate:

```text
printed replacement
generated edit
applied file modification
```

A consuming agent should never have to infer which of these an operation produces from the word “replace.”

Normalize coordinates only when the conversion is defined. Preserve native coordinates and input mappings alongside normalized fields; mark unavailable mappings explicitly.

---

## 5. Extraction strategy: native structured sources first

The most efficient construction path is **schema-led, evidence-backed extraction**, not reconstructing every exposed feature from compiler primitives.

### Ast-grep configuration and rule schemas

Use the native serializable configuration types and published schemas as the starting inventory. `SerializableRule` already provides structured rule fields and implements serialization, deserialization, and JSON Schema generation. 

The adapter should preserve native structure, attach stable catalog IDs, and associate semantic annotations and validation behavior.

Use the native rule compiler to validate configurations. JSON Schema validation alone should not be treated as proof that a rule is semantically meaningful.

### Tree-sitter grammar metadata

Ingest the `node-types.json` corresponding to the selected parser for each language. It exposes node kinds, named/anonymous status, fields, permitted child types, multiplicity, required children, and supertypes. 

Store this as a language-specific grammar catalog and retrieve only relevant fragments.

This is the bridge from an abstract requirement such as “the function’s body” to concrete grammar kinds and fields. It should not require bespoke language documentation for every query.

### Ripgrep flag definitions

Use ripgrep’s internal flag registry as a direct extraction point. Its `Flag` abstraction and global flag collection represent names, aliases, negated forms, documentation, and argument behavior. 

Because this is an internal interface, isolate it behind a snapshot-bound export adapter. Validate inventory completeness against the selected executable’s help surface and relevant tests.

Do not mistake the flag inventory for the complete CLI grammar: positional handling and interactions must also be characterized.

### PCRE2 documentation and ripgrep integration

Ingest release-bound PCRE2 syntax and behavioral documentation, then classify each relevant capability by its actual exposure:

```text
ripgrep CLI option
pattern syntax accepted through ripgrep
Rust API only
native PCRE2 API only
uncharacterized integration behavior
```

Some PCRE2 controls are expressible inside patterns and therefore need no corresponding Rust builder method. Absence of a wrapper method is not proof of unavailability through ripgrep. 

The capability test is the complete path:

```text
engine supports construct
    ∧ application permits/configures it
    ∧ selected output can expose the required result
```

### Documentation, examples, and tests

Use documentation for explicit semantics and selection rationale. Use examples as concrete compositions, and tests as behavioral evidence.

LLM-assisted characterization should primarily produce structured assertions, aliases, interaction candidates, and targeted probe specifications—not long prose summaries.

---

## 6. Role of the seven Rust extraction families

Retain the seven families as the implementation-evidence subsystem. They remain valuable, but should not all be mandatory steps for every catalog entry.

### 1. Rustdoc JSON + `rustdoc-types`

Use for public APIs, types, traits, implementations, generics, documentation, and links to exposed mechanisms. Use `rustdoc-json` for generation rather than implementing invocation handling independently. Where useful, place `trustfall_rustdoc` over the raw artifact for declarative extraction queries. Preserve the raw representation alongside derived facts. 

### 2. `ra_ap_hir`, augmented by `ra_ap_ide`

Use HIR for resolved implementation relationships and syntax-to-definition mapping. Use the existing IDE layer for higher-level queries before rebuilding them from HIR primitives. Record unresolved results explicitly; a semantic extractor should not be represented as infallible. 

### 3. `ra_ap_syntax`

Use for exact Rust source structure, declarations, attributes, expressions, configuration definitions, and source locations. This supplies source fidelity where higher-level representations normalize or omit details. 

### 4. `ra_ap_load-cargo` + `ra_ap_project_model`

Use to load the selected workspaces into the rust-analyzer model, including collaborating internal crates. This is shared extraction infrastructure, not a separate agent-facing catalog. 

### 5. MIR through `rustc_driver` / `rustc_middle::mir`

Use for compiler-lowered bodies, control-flow graphs, operations, calls, locals, places, and branches. Preserve the MIR phase and extractor identity with the artifact. A CFG describes control structure; it does not by itself establish every possible runtime call target or feasible execution path. 

### 6. `rustc_mir_dataflow`

Use for selected flow-sensitive analyses needed to explain behavior. Treat it as a framework for analysis domains, transfer functions, and fixed-point computation—not a turnkey interprocedural option-to-behavior tracer. Custom summaries or interprocedural logic must be identified as additional implementation work. 

### 7. `cargo_metadata` + Guppy

Use Cargo metadata as the package/dependency input and Guppy for graph traversal and package-set operations. Restrict this to the selected workspace configuration; no feature-matrix project is required. 

**A full code-property graph can be generated and retained without making its completion a prerequisite for publishing useful capability contracts.**

---

## 7. Consolidation, identity, and evidence

### Keep native artifacts and canonical projections separate

Use three layers:

```text
Native artifacts
    source, schemas, rustdoc, semantic extraction, MIR, documentation

Canonical evidence graph
    normalized entities, relationships, assertions, provenance

Agent-facing contract catalog
    compact projections optimized for selection and composition
```

Do not claim the canonical projection is lossless merely because the raw artifacts are retained. Instead, give every adapter an explicit extraction-coverage description and maintain references back to the native data.

### Reconcile cross-layer identity conservatively

Give capabilities and mechanisms stable semantic IDs, separate from compiler or parser IDs.

For implementation entities, use snapshot-scoped identities and retain tool-specific aliases:

```text
canonical implementation entity
    rustdoc item alias
    rust-analyzer definition alias
    source anchor
    rustc definition alias
    MIR body alias
```

Source spans, names, and paths are matching evidence, not universal identity keys. Macro expansion, generated code, reexports, and associated items can require one-to-many mappings. Rust-analyzer itself explicitly describes syntax-to-HIR mapping as potentially one-to-many. 

Uncertain mappings should remain unresolved candidates rather than being merged prematurely.

### Evidence attaches to assertions

Use an assertion record such as:

```text
Assertion
  subject
  predicate
  object_or_value
  applicability_conditions

  basis
    documented
    derived_from_source
    observed_in_probe
    inferred

  supporting_evidence
  contradicting_evidence
  resolution_status
```

Avoid invented numerical confidence values. A successful probe establishes the recorded case, not universal behavior. A source reference establishes that a symbol exists, not necessarily the full behavioral interpretation attached to it.

Conflicting documentation, source, and runtime observations should remain visible until reconciled against the selected snapshot.

---

## 8. Behavioral validation and coverage accounting

### Use focused probes, not an option powerset

Prioritize interactions where a plausible query can be syntactically valid but semantically wrong.

Examples include traversal depth, binding order, multiline behavior, capture-versus-match output, corpus exclusions, encoding/offset interpretation, engine exposure, and transformed-input mappings.

Each probe should record:

```text
snapshot and fixture
invocation/configuration
controlled input context
expected matches
expected nonmatches
expected spans/captures/output
expected completion or error behavior
assertions exercised
```

Use small counterexamples that distinguish mechanisms, not only positive demonstrations.

Ast-grep’s native test framework already supports match/nonmatch cases and output snapshots. Normalize its `valid` and `invalid` terminology into unambiguous internal fields such as `should_not_match` and `should_match`. 

### Validate at distinct levels

Keep separate results for:

```text
schema validity
native compilation/parsing
behavior on fixtures
output-contract conformance
composition compatibility
```

Compilation success is not evidence that a query expresses the intended requirement.

### Track two independent kinds of completeness

**Surface coverage:** every discovered flag, rule field, enum choice, and in-scope syntax construct maps to a contract, an alias, an explicit exclusion, or an unresolved item.

**Semantic characterization:** each mapped mechanism has the required contract fields, interaction analysis, evidence, and appropriate validation status.

This prevents “every flag has a record” from being mistaken for “every relevant behavior is characterized.”

Missing information must remain **unknown**, not unsupported.

---

## 9. Retrieval: compact, prerequisite-complete decision packets

The retrieval system should combine flexible discovery with deterministic inclusion of necessary context.

```text
Requirement or mechanism query
    ↓
Candidate discovery
    ↓
Filter by interface, language, semantics, and requested result
    ↓
Evaluate relevant interactions
    ↓
Include prerequisites and unresolved choices
    ↓
Return a compact decision packet
```

Semantic or lexical search can find candidates. It should not decide which prerequisites are safe to omit.

### Decision packet contents

A normal response should contain:

```text
candidate mechanisms
why each matches the request
distinguishing alternatives
necessary parameters and effective defaults
relevant interactions
requested-result compatibility
guarantees and non-guarantees
unresolved conditions
evidence handles
```

Shared grammar, engine, or output facts should appear once.

Under a token budget, reduce optional explanation or the number of candidates. Do not silently truncate mandatory conditions. If the essential context cannot fit, return a smaller complete candidate set and indicate what was omitted.

### Small proposed interface

```text
discover(requirements, surface, language, requested_result)

describe(mechanism_ids, fields, context)

compare(mechanism_ids, dimensions, context)

validate(query_or_rule, intended_contract, fixtures?)

explain(assertion_ids, evidence_depth)
```

These are proposed system operations, not existing upstream APIs.

Support batch requests, field projection, exact-ID retrieval, and explicit unresolved results. An intelligent agent should be able to enter through either a natural-language problem or a known option name.

### Example retrieval

Request:

```yaml
subject: source_code
relationship: syntactic_containment
target: enclosing_construct
contained_pattern: supplied_by_agent
descendant_scope: unrestricted
result: enclosing_node_spans
surface: ast_grep_rule
```

Relevant response:

```text
mechanism:
    ast-grep rule.has

required configuration:
    unrestricted traversal setting

result semantics:
    return the enclosing target, not merely the contained match

additional context:
    relevant target-language node kinds and fields

conditional considerations:
    binding order, if the subrules share dependent bindings
    explicit boundaries, if nested constructs should be excluded

non-guarantees:
    resolved callee identity
    execution or data-flow relationships
```

The catalog’s role is to supply these distinctions. The agent remains responsible for constructing the concrete rule.

---

## 10. Composition without a general-purpose planner

Represent reusable plan fragments with typed inputs and outputs:

```text
file selection → structural matching over complete files

structural matching → capture extraction → textual predicate

candidate generation → external semantic verification
```

The last step is a boundary to a separately available semantic system, not an implied capability of ast-grep or ripgrep.

Each fragment should state:

```text
accepted input units
produced output units
scope assumptions
coordinate preservation
recall conditions
ordering requirements
unresolved obligations
```

Do not feed only matching lines into a structural stage that requires complete enclosing syntax.

### Prefilter correctness

A recall-preserving prefilter requires:

```text
final_match(file) ⇒ prefilter_selects(file)
```

under compatible corpus-selection and input-interpretation assumptions.

When that implication is merely plausible, mark the prefilter heuristic. Keep it available as a choice, but do not present it as semantically equivalent to the unfiltered search.

Performance guidance should be conditional on workload and required output, rather than assigning unconditional “fastest tool” labels.

---

## 11. Storage, skill packaging, and implementation sequence

### One canonical model, multiple query projections

Use typed records for contracts and assertions, normalized relation tables for graph edges, and references to retained native artifacts.

An Arrow/DataFusion representation can be the chosen relational projection, but the architecture does not require maintaining a separate graph database. Graph semantics and physical storage are separate decisions.

Keep the contract schema stable while extractor-specific details remain behind adapters.

### Skill package

The skill should contain three logical artifacts:

```text
Entry point
    vocabulary, supported interfaces, retrieval operations,
    result interpretation, semantic boundaries

Queryable catalog
    contracts, parameters, interactions, grammar fragments,
    result contracts, plan fragments, coverage states

Evidence store
    documentation, source anchors, implementation graph,
    behavioral fixtures and observations
```

The evidence store should be referenced, not embedded, in routine responses. A JSON field that is returned to the agent is still context; selective access must happen before response serialization, not merely through nesting.

### Implementation sequence

**First: establish the snapshot, inventory, and schemas.** Export native surfaces, ingest grammar metadata, define stable mechanism IDs, and build coverage accounting.

**Second: characterize contracts and interactions.** Extract documented facts mechanically, use source analysis to resolve application behavior, and record remaining questions explicitly.

**Third: validate the important semantic boundaries.** Build focused probes and native-validation adapters; attach results to assertions rather than converting successful examples into universal claims.

**Fourth: implement retrieval and composition checks.** Provide field-selective decision packets that include the conditions necessary for correct use.

**Fifth: deepen implementation evidence independently.** Expand HIR, CFG, and dataflow coverage while preserving the compact agent-facing interface.

### Acceptance criteria

Judge the system by whether an agent can construct and correctly interpret queries using the retrieved information.

The principal measures should be query/task correctness, unsupported-guarantee detection, coverage of decision-relevant options, prerequisite inclusion, result-schema correctness, retrieval round trips, and tokens consumed per successful task.

Tests should include unfamiliar compositions and underspecified requests—not only examples used to author the catalog.

---

## Consolidated architecture

```text
SELECTED TOOL SNAPSHOT
    ast-grep + ripgrep + PCRE2 10.48 integration
                         │
          ┌──────────────┼────────────────┐
          ▼              ▼                ▼
   Native schemas   Documentation    Seven-family
   Flag registry    Examples/tests   Rust extraction
   Grammar metadata                  / code graph
          │              │                │
          └──────────────┼────────────────┘
                         ▼
              ASSERTIONS + EVIDENCE
                         │
               Behavioral validation
                         │
                         ▼
              SEARCH-CONTRACT CATALOG
              capabilities / mechanisms
              parameters / interactions
              outputs / non-guarantees
              grammar / plan fragments
                         │
                         ▼
               SELECTIVE RETRIEVAL
              relevant candidates plus
              required decision context
                         │
                         ▼
                 PROGRAMMING AGENT
              constructs and evaluates
                 the concrete query
```

**The implementation target is not “complete documentation in graph form.” It is a dependable selection interface over complete-enough evidence: native schemas minimize reconstruction, explicit contracts preserve semantics, behavioral probes validate consequential distinctions, and selective retrieval gives the agent only the context needed for the decision at hand.**

# Revised extraction and consolidation specification

I would make **your canonical dataset the only persistent analysis model**, with the listed libraries acting as extraction components—not as separate query services.

The revised flow is:

```text
Selected source snapshot
    ↓
Cargo metadata
Rustdoc JSON
rust-analyzer syntax + HIR
rustc MIR + dataflow
    ↓
Explicit, versioned extraction passes
    ↓
One canonical set of entities, relationships, and evidence
    ↓
All dependency, reference, implementation, flow,
and capability-selection queries
```

The central rule remains:

> **Unify entity identity where justified; preserve differences between source syntax, declared contracts, resolved semantics, executable operations, and derived behavior.**

This removes Guppy and `ra_ap_ide` from the application architecture without requiring you to discard information those layers would otherwise help retrieve. Instead, reverse dependencies, references, callers, and implementation relationships become explicit projections over your own records.

There is one implementation distinction worth preserving: rust-analyzer and rustc necessarily maintain temporary internal databases while extracting information. Those are **compiler working state**, not additional persistent models or agent-facing query systems. The Cargo loader returns a `RootDatabase`, virtual filesystem, and optional procedural-macro client, so it can initialize direct HIR analysis without an LSP session. Its underlying dependencies include `ra_ap_ide_db`, but that does not require using `ra_ap_ide` or its editor-oriented query operations. 

Below, the seven families are ordered by their role in constructing the dataset, rather than by priority. Schema names are proposed internal structures.

---

# 1. Cargo metadata: package identity and dependency facts

## Evidence to gather

Read `cargo metadata` output directly and extract:

```text
Package
  Cargo package ID
  name and version
  package source
  manifest path
  workspace membership

Target
  owning package
  target name
  target kind
  crate types
  root source file

DeclaredDependency
  declaring package
  package name
  local rename
  dependency kind
  version/source requirement
  declared optionality and feature settings

ResolvedDependency
  source package
  destination package
  locally usable dependency name
  dependency-kind records
  target-condition records
```

Preserve declared and resolved dependencies separately. Cargo’s output distinguishes manifest declarations from resolution nodes, and its resolved edges carry dependency names and kinds. Treat package IDs as opaque identifiers rather than extracting identity by parsing their string representation. 

## What replaces Guppy

Insert the dependency edges directly into your canonical relation tables.

Then implement these as ordinary operations over those tables:

```text
direct dependencies       = outgoing resolved-dependency edges
reverse dependencies      = incoming resolved-dependency edges
transitive dependencies   = graph traversal over those edges
workspace-only subgraph   = package-membership restriction
dependency explanation    = retained edge path
```

You do not need a second `PackageGraph` object with its own persistent identity system.

An adjacency index or materialized reachability result is acceptable—it is an index over your canonical facts, not another independently maintained model.

## Merge contribution

Cargo establishes **package ownership and dependency context**. It does not establish that a particular symbol is called, a particular dependency body was analyzed, or a dependency capability is exposed through the CLI.

Use:

```text
Definition → owned by → CrateUnit → belongs to → Package
```

rather than attaching every symbol directly to a package name.

That intermediate `CrateUnit` is supplied by project loading and compiler extraction.

---

# 2. `ra_ap_load-cargo` + `ra_ap_project_model`: the concrete analysis universe

## Evidence to gather

Use these libraries to load the selected project and export the mappings required by subsequent passes:

```text
CrateUnit
  owning package and target
  crate root
  dependency crate units
  dependency names in this crate
  selected compilation-context fingerprint

FileMembership
  source artifact
  rust-analyzer file identity
  participating crate unit
  original/generated classification

LoadCoverage
  successfully loaded units
  missing generated files
  unavailable dependency sources
  macro-expansion failures
  loading diagnostics
```

Keep configuration information as extraction provenance for the **one selected configuration**. Do not create a feature-variant exploration subsystem.

Generated source and procedural macros matter because they can change which declarations and references exist. The loader exposes controls for loading Cargo-check output directories and configuring procedural-macro processing. 

## How to use it

Load the workspace once for a batch extraction run. Obtain the database and virtual filesystem, then run your syntax and HIR passes against that same loaded state.

Do not create an `AnalysisHost`/`Analysis` service or issue editor-navigation requests. The intended boundary is:

```text
load project
    → access syntax and HIR
    → export canonical records
    → finish extraction
```

For semantic traversal, obtain source nodes through the associated `Semantics` instance. Rust-analyzer specifically recommends its semantic source accessor because it registers the parsed tree in the semantics cache. Detached or independently reparsed nodes should not be casually substituted into that semantic context. 

## Merge contribution

This family reconciles:

```text
Cargo package/target
    ↔ analysis crate
    ↔ source file
    ↔ generated or expanded source
```

It also establishes the denominator for completeness claims.

For example:

```text
references extracted from all successfully analyzed bodies
```

is materially different from:

```text
all references in every dependency
```

The loader’s coverage records make that difference queryable.

---

# 3. `ra_ap_syntax`: exact source structure and authored configuration

## Evidence to gather

Export the full source structure using `ra_ap_syntax`’s CST and typed AST interfaces. The library provides full-fidelity syntax representation, including error-containing input. 

The canonical records should include:

```text
SyntaxNode
  source artifact
  kind
  parent
  ordered child position
  source range
  enclosing declaration/body
  original-source or expansion origin

Token
  kind
  source range
  parent and position

AttributeOccurrence
  owning syntax node
  attribute path
  structured argument syntax

LiteralOccurrence
  source spelling
  decoded value, where supported
  literal kind

SourceDiagnostic
  source range
  diagnostic category
  message
```

Store source bytes once. Nodes and tokens should reference ranges rather than duplicate their source text.

## Application-specific extraction

For ast-grep and ripgrep, this pass should identify the authored structures that define user-facing options:

```text
configuration structs and enum variants
serialization names and aliases
default annotations and default functions
flag-definition implementations
accepted-value literals
negated and alternate spellings
assignments to configuration fields
ordered builder calls
branch and match conditions
early returns and error construction
```

These should initially be **source observations**, not effective-behavior conclusions.

For example:

```rust
#[serde(default = "default_mode")]
```

establishes that the source names a default-producing function. It does not, by itself, establish the returned value.

Likewise:

```rust
args.mode = ...
```

identifies an assignment occurrence. HIR is needed to identify the field; MIR and analysis may be needed to establish which assignment reaches a later use.

## Merge contribution

Syntax supplies stable, source-backed anchors for other representations:

```text
Declaration → has source → SyntaxNode
ReferenceOccurrence → occurs at → SyntaxNode
MirOperation → corresponds to → SourceAnchor
Documentation → originates from → source range, when recoverable
```

Retain declaration and definition locations separately where needed. For example, an out-of-line module can have a declaration in one file and its definition in another; rust-analyzer exposes these as distinct source relationships. 

**Do not use syntax-node identity as semantic identity.** A declaration is a semantic entity represented by syntax; the two records have different purposes.

---

# 4. `ra_ap_hir`: direct semantic extraction without IDE queries

This is the most significant revision from the previous recommendation.

Use HIR to resolve individual observations, and construct the global reference/call/implementation indexes yourself from those observations.

## 4.1 Enumerate definitions explicitly

Start from each selected crate’s module tree. Enumerate module declarations, child modules, implementation blocks, and associated items.

The public HIR interfaces expose module traversal and implementation enumeration directly, including `Module::declarations`, `Module::children`, `Module::impl_defs`, and `Impl::all_in_crate`. 

Export:

```text
Definition
  kind
  name and namespace
  semantic owner
  declaring crate unit
  visibility
  source representation
  generated/authored classification

Implementation
  self type
  implemented trait reference, if any
  generic parameters
  predicates
  associated items
  negative/synthetic/generated classification where available
```

Supplement module traversal with source/body traversal for block-local declarations, closures, and expansions. Deduplicate through native HIR identities within the extraction run.

## 4.2 Resolve occurrences, rather than asking for reference lists

Walk the source and expanded syntax, identify each relevant occurrence, and invoke the appropriate HIR operation.

The `Semantics` interface exposes path, method, field, callable, type, macro, and expression-adjustment operations directly. It also includes operations for implicit language mechanisms such as indexing, operators, `await`, and `?`. 

Export records such as:

```text
ReferenceOccurrence
  occurrence ID
  enclosing definition
  source anchor
  syntactic role

Resolution
  occurrence ID
  target definition
  namespace
  resolution kind
  substitutions, where available
  completion/uncertainty status

CallSite
  source occurrence
  enclosing callable
  invocation kind
  receiver
  ordered arguments

CallTarget
  call site
  resolved declaration
  identified implementation body, if established
  dispatch classification
  supporting evidence

TypeObservation
  expression/pattern/binding
  type term
  observation role
  generic environment
```

Preserve the difference between:

```text
method declaration resolved
implementation body identified
runtime target set established
```

Those are not interchangeable, particularly for generic and dynamic dispatch.

## 4.3 Build reverse queries from exported facts

After extraction:

```text
find references to X
    = resolutions whose target is X

find callers of X
    = call-target rows for X, joined to enclosing callables

find implementations of trait T
    = implementation rows referencing T

find uses of type T
    = type-reference occurrences and structural type links
```

Every returned result can therefore point to an occurrence and its resolution evidence.

This is more transparent than accepting a preassembled IDE answer whose traversal scope and filtering are external to your data model.

## 4.4 Keep implementation declarations separate from applicability

Do not use `Impl::all_for_type` as a complete semantic implementor index. Its documentation explicitly describes it as an approximation that excludes blanket implementations and performs a shallow type-constructor check. 

Instead, store all extracted impl declarations with their predicates. Represent conclusions separately:

```text
ImplDeclaration
    describes a generic implementation

ApplicabilityAssertion
    implementation applies to this instantiated type
    under this generic environment
    with this supporting resolution
```

You do not need to build another trait solver. Materialize applicability when HIR/compiler evidence establishes it for relevant concrete contexts; otherwise retain the predicates and unresolved status.

## 4.5 Preserve type context

Normalize types structurally, but preserve their binder and generic environment. HIR types carry origin context, and the API warns against combining types from incompatible origins without appropriate rebasing or instantiation. Its public interface provides structural inspection and traversal operations. 

When a public API does not expose enough structure for a complete encoding, emit an explicitly incomplete or opaque type observation. A display string is a useful label, not a substitute for a resolved type identity.

---

# 5. Rustdoc JSON + `rustdoc-types`: declared API contracts and documentation

## Evidence to gather

Generate or acquire rustdoc JSON for the selected source and parse it directly with `rustdoc-types`. No additional rustdoc query abstraction is required.

Gather:

```text
item index and module hierarchy
item kinds and visibility
public/export paths and reexports
callable signatures
ordered parameters and return types
generic parameters and predicates
struct fields and enum variants
traits and associated items
inherent and trait implementations
documentation and resolved intra-doc links
attributes and source spans
```

The root JSON record contains the item index, path summaries, external-crate information, format version, and private-item inclusion state. Item records contain documentation, links, visibility, attributes, and kind-specific data. 

Use private-item inclusion for the implementation characterization where appropriate; the public API is only part of the relevant configuration and CLI implementation.

## Normalize types and implementations carefully

Reuse the same canonical type-term vocabulary used by HIR and rustc, while tagging the observation as a **declared/documented type**.

Preserve distinctions among nominal types, references, tuples, generic parameters, associated-type projections, trait objects, and opaque types. Preserve synthetic and blanket implementation information rather than treating every documented impl as an authored body. 

## Documentation evidence

Store each documentation body once and make fragments addressable. Attach interpreted behavioral statements separately:

```text
DocumentedAssertion
  subject
  claimed behavior/default/constraint
  applicability conditions
  supporting document fragment
```

A documented default and an observed effective application setting may both be valid under different scopes.

## Merge contribution

Rustdoc enriches canonical definitions with contracts and documentation. It does not create a second authoritative symbol registry.

Reexports become:

```text
ExportPath → exposes → Definition
```

not duplicate definitions.

Also preserve documentation-specific context. Rustdoc sets `cfg(doc)`, so documentation presence alone does not prove that an item exists in the selected runtime compilation. This is an extraction distinction within your snapshot, not a request to enumerate build variants. 

---

# 6. MIR through rustc: operational bodies and control flow

## Evidence to gather

Use a narrow rustc adapter to export a selected MIR representation into your canonical schema.

I would choose one well-defined, pre-optimization runtime MIR stage for the initial operational graph, subject to what the pinned adapter can capture reliably. Record the exact stage. MIR dialects have meaningful semantic differences, including differences in drops and coroutine representation. 

Export:

```text
MirBody
  owning definition
  compiler-native identity
  phase
  argument count
  source scopes
  local declarations

MirBlock
  body
  block ordinal
  ordered operations

MirOperation
  statement/terminator kind
  operands
  destination
  source information

CfgEdge
  originating terminator
  destination
  normal/conditional/unwind/other edge kind
  branch value or condition, where represented

Place
  base local
  ordered projections
  type

OperationUse
  operation
  place or value
  read/write/move/copy/borrow/argument/return role
```

MIR bodies expose blocks, locals, phase, source scopes, and source-variable information. Use the compiler’s visitor infrastructure to traverse operations and places, including their use contexts, rather than independently reconstructing traversal rules. 

## Preserve operational distinctions

Do not reduce all successors to untyped arrows. Calls, switches, drops, returns, and unwinding have different effects and successor structures. 

Likewise, keep:

```text
source variable
MIR local
projected place
assignment-produced value
```

as distinct concepts.

A field index should be accompanied by owning-type and variant context, with a link to the canonical field definition where recoverable.

## Acquisition constraints

Compiler callbacks and queries must be arranged around the chosen MIR stage. Intermediate MIR results can be consumed through rustc’s “stealing” mechanism; a late extraction callback cannot assume every earlier representation remains available. 

Run the adapter for the selected Rust units whose bodies you need. Record bodies that were not extracted or are unavailable. Package dependency presence is not evidence that every dependency body was exported.

## Merge contribution

Attach the MIR body to the canonical callable definition.

Connect operations to source through mappings such as:

```text
MirOperation → lowered from → SyntaxOccurrence
MirCall → corresponds to → SourceCallSite
PlaceProjection → accesses → FieldDefinition
```

These are often many-to-many relationships. **Do not merge a source expression and a MIR operation into the same entity.**

---

# 7. `rustc_mir_dataflow`: explicit, inspectable behavioral derivations

Use this as an extraction-time computation framework. Export its results and assumptions into the same canonical dataset; do not expose a separate dataflow query service.

The framework supplies domains, transfer effects, fixed-point evaluation, and result inspection. Your application-specific analyses remain explicit code you own. 

## Initial analyses to implement

### Reaching definitions and value origins

Track which inputs and assignments may contribute to a value at a location.

For this application:

```text
CLI/configuration input
    → normalized field
    → builder argument
    → stored matcher setting
```

### Finite configuration-state propagation

Track a deliberately bounded domain:

```text
known boolean
known enum alternative
known small constant
known absent/present state
set of possible values
unknown
```

This is sufficient for many default, override, selection, and fallback questions without introducing general symbolic execution.

### Function effect summaries

Summarize relevant behavior:

```text
reads argument/receiver field
writes receiver field
returns transformed input
conditionally invokes operation
overwrites configuration state
may affect unknown reachable state
```

Use these summaries for interprocedural propagation. The summary records and call relationships remain in your canonical model.

## Record analysis semantics

Every result should identify:

```text
analysis and transfer-rule version
body and MIR phase
program point: before / after / edge
may-versus-must interpretation
path sensitivity
alias assumptions
external-call handling
derivation inputs
completion state
```

Use strong updates only for sufficiently identified destinations. Unknown calls and ambiguous aliases must weaken conclusions rather than being treated as harmless.

Separate **value dependence** from **control dependence**:

```text
flag supplies the value passed to a builder
```

is different from:

```text
flag determines whether the builder call occurs
```

If you add control-dependence analysis over the CFG, record its exit/unwind policy and algorithm explicitly.

## Merge contribution

Flow facts reference existing definitions, operations, places, and program points.

They do not overwrite raw MIR or turn every possible-flow edge into a guaranteed runtime behavior.

---

# 8. One canonical structure—not seven mirrored models

I would use typed tables or equivalent typed graph records, all sharing one ID system and provenance vocabulary.

The source libraries’ objects exist only inside extraction adapters. Their outputs are converted immediately into these records.

## 8.1 Provenance and identity

```text
Artifact
  artifact_id
  content_hash
  artifact_kind
  retained_location

ExtractionRun
  run_id
  family
  extractor_identity
  input_artifact_ids
  context_id
  scope
  completion_status

Entity
  entity_id
  kind
  owner_id
  snapshot_id

NativeBinding
  run_id
  native_namespace
  native_handle
  canonical_entity_id
  mapping_status
  mapping_basis

SourceAnchor
  artifact_id
  native_range
  normalized_byte_range?
  origin
  precision
  expansion_parent?
```

`NativeBinding` is a crosswalk, not an independently queryable program model.

Native handles must be run-scoped. Rustdoc IDs, HIR handles, compiler `DefId`s, and MIR local numbers do not share a namespace. A rustc `DefPathHash` is a useful compiler-native alias, but not a universal cross-extractor identifier. 

## 8.2 Typed program records

```text
Package, Target, CrateUnit, DependencyEdge, FileMembership

Definition, ExportPath, Field, Variant, Implementation,
GenericParameter, Predicate, Signature

TypeTerm, TypeArgument, TypeObservation

SyntaxNode, Token, AttributeOccurrence, LiteralOccurrence

ReferenceOccurrence, Resolution, CallSite, CallTarget

MirBody, MirBlock, MirOperation, CfgEdge,
MirLocal, Place, OperationUse

FlowFact, FunctionSummary, EffectSummary
```

These are categories within one schema, not separate backends.

Use ordered child records for arguments, generic substitutions, projections, and syntax children.

For generic parameters, identity must include the binder and parameter position. Two parameters named `T` are not one global type variable.

## 8.3 Evidence and derived assertions

Every extracted fact should reference evidence:

```text
Evidence
  evidence_id
  extraction_run
  native_locator
  source_anchor?
  observation_kind
```

Derived facts additionally reference a derivation:

```text
Derivation
  derivation_id
  rule_id
  rule_version
  input_fact_ids
  assumptions
  output_fact_ids
```

Behavioral assertions should be scoped:

```text
BehaviorAssertion
  subject
  predicate
  value_or_target
  applicability_condition
  interpretation
  supporting_evidence
  contradicting_evidence
  status
```

This avoids both untyped “everything is JSON” storage and repeated full copies of each native representation.

Retained source files and native artifacts provide auditability. They are an archive, not a second live query model.

## 8.4 Connection to the capability catalog

Keep user-facing concepts separate from implementation entities:

```text
SurfaceBinding
  surface_entry
  implementation_entity
  role
  evidence

MechanismEffect
  mechanism
  condition
  effect
  affected_setting_or_operation
  assertion_ids
```

Useful binding roles include:

```text
declared_by
parsed_by
validated_by
stored_in
normalized_by
configured_through
implemented_by
reported_by
```

A CLI option, Rust field, builder parameter, and native engine option remain different entities. The explicit relationships between them are the valuable result.

---

# 9. Reconciliation and merge procedure

## Pass A: establish package, crate, and source identity

Register source artifacts by content, then attach path and crate-membership information.

Join Cargo targets to loaded/compiler crate units using the selected package, target, root file, and invocation context. Do not identify packages by name alone or files by basename.

Keep original files, generated files, and expansion artifacts distinguishable.

## Pass B: establish declarations before their children

Reconcile hierarchically:

```text
crate
    → module
    → type / trait / impl
    → associated item / field
    → body-local occurrence
```

Use a combination of owner, kind, namespace, source anchor, and structural signature.

Prefer direct source-to-definition mappings to guessed path equivalence.

For impl blocks, include the implementation’s own identity, trait reference, self type, generic environment, and predicates. Two impls for similarly printed types should not collapse.

## Pass C: preserve non-identity mappings

Use separate relationships for:

```text
represents
generated_from
expanded_from
lowered_from
exposes
corresponds_to
```

Rust-analyzer’s macro source mapping may fall back to the enclosing invocation when precise mapping is unavailable. That fallback is not an exact declaration identity. 

Likewise, a lowered closure or coroutine body can have a relationship to a source construct without being identical to that construct.

Uncertain identity matches remain candidates. Do not merge first and attempt to recover distinctions later.

## Pass D: attach semantic and operational facts

Join resolved reference targets to canonical definitions.

Join MIR bodies to owning definitions and map source calls to MIR calls using owner, source information, call target, and argument structure. Preserve multiple candidate mappings when needed.

Represent source-call and MIR-call evidence separately so an aggregate caller query does not double-count the same conceptual call without knowing why.

## Pass E: normalize types conservatively

Share type terms when their structural identity and generic context agree.

Preserve separate observations for:

```text
written type
documented declaration type
HIR inferred type
HIR adjusted type
MIR local/operand type
```

Different observations may legitimately describe different stages of the same expression. No global rule should declare one of them the winner.

## Pass F: derive behavior from explicit inputs

An effective-setting assertion needs more than a reference chain.

For example:

```text
parameter determines setting at use
```

requires evidence for parameter binding, transformations, relevant assignments, overwrite order, call effects, and the applicable control-flow conditions.

When only part of that chain is established, emit:

```text
may_influence
conditionally_reaches
unresolved_effect
```

rather than a stronger assertion.

Documented defaults and application overrides should remain separately scoped. They are not contradictions merely because their values differ.

---

# 10. Replacing convenience queries with transparent projections

The removed tools primarily saved traversal and query implementation. Their useful results can be reconstructed from explicit canonical facts.

**Dependency queries** use `DependencyEdge`, with path witnesses for transitive results.

**Reference queries** use `ReferenceOccurrence` joined to `Resolution`. Query results include the resolution kind and extraction coverage.

**Caller queries** use `CallSite` and `CallTarget`. Keep direct, generic, dynamic, and unresolved targets distinguishable.

**Implementation queries** use `Implementation` and its predicates. “Declared impls mentioning this type” and “impls proven applicable to this instantiated type” should be separate operations.

**Configuration-impact queries** traverse `SurfaceBinding`, field resolutions, operation uses, summaries, and flow facts.

**Capability explanations** retrieve the mechanism’s behavioral assertions and the derivation subgraph supporting them.

These operations can all return records from the same schema. No query needs to start a rust-analyzer session, consult a Guppy graph, or combine a second engine’s independently interpreted answer.

The tradeoff is explicit: you own enumeration, indexing, and derived-query semantics. You do **not** need to reimplement Rust parsing, type inference, name resolution, MIR construction, or the dataflow fixed-point engine.

---

# 11. Example: consolidated evidence for ripgrep engine selection

Ripgrep’s inspected implementation handles automatic engine selection by attempting its default matcher first, returning it on success, and attempting PCRE2 after an error. 

The intended extraction—not a claim that these passes have already been executed—would produce:

```text
SurfaceEntry("--engine")
    → parsed/stored configuration field
    → normalized engine selection
    → branch discriminant

automatic branch
    → default matcher construction
        success → return matcher
        error   → PCRE2 matcher construction
```

Each family supplies a distinct part:

```text
Cargo metadata:
    ownership and dependency context

Project loading:
    concrete crate/file identities and extraction coverage

Syntax:
    option declarations, assignments, ordered match branches

HIR:
    resolved fields, enum variants, and construction calls

Rustdoc:
    contracts and documentation for relevant library interfaces

MIR:
    operational calls, successors, return handling

Dataflow:
    option origin, branch dependence, and propagated outcomes
```

The compact capability record can then express:

```yaml
mechanism: rg.engine.auto

effect:
  kind: ordered_fallback
  first_attempt: rg.engine.default
  fallback_attempt: rg.engine.pcre2
  fallback_condition: first_attempt_returns_error

evidence:
  surface_binding: ...
  resolved_calls: ...
  control_flow: ...
  dataflow_derivation: ...
```

The record is derived from your graph. It is not a separate interpretation supplied by an IDE query engine.

For native PCRE2 behavior, the Rust graph ends at the foreign-call boundary. Represent that boundary explicitly; do not invent Rust MIR or flow facts for the C implementation. Release-bound documentation and behavioral probes from the broader proposal can support native semantic assertions without becoming additional Rust extraction libraries.

---

# 12. Construction rules I would enforce

**One persistent identity system.** Native IDs are scoped aliases; canonical IDs are the only cross-layer join keys.

**One query model.** Dependency, source, semantic, operational, and capability records share the same schema and query interface.

**Explicit extraction coverage.** Distinguish absent, unavailable, unresolved, excluded, and not yet extracted.

**No opaque replacement for evidence.** A convenience result must either be derived from retained facts or carry enough provenance to explain its construction.

**No silent semantic flattening.** Documentation, source form, resolved meaning, MIR operations, and dataflow conclusions remain distinguishable.

**No unsupported completeness claims.** A complete traversal of the selected representation is not automatically a complete set of runtime targets, feasible paths, or exposed capabilities.

**No live compiler dependency in ordinary retrieval.** Once the snapshot is built, the agent queries your dataset. New compiler analysis is a new extraction operation whose results are merged through the same rules.

The resulting design is:

> **Use the listed libraries to compute facts; use your canonical schema to own those facts; use explicit derivations to construct every higher-level relationship.**

That retains all seven families’ distinct contributions while eliminating the parallel package graph and IDE-query layers you do not want.