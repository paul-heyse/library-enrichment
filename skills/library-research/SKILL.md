---
name: library-research
description: Research Rust or Python library capabilities, implementation patterns, feature flags or extras, exact APIs, and upgrade opportunities using Context7 plus the library-enrichment MCP service. Use for library selection, unfamiliar APIs, design reviews, version-sensitive implementation, and documentation/API disagreements. Do not use for routine local edits that require no external library research.
---

# Evidence-backed library research

Use Context7 for explanations and examples. Use `library-enrichment` for exact-release identity, structured discovery, targeted source/API evidence, changes, and verification. You own the architectural reasoning; the service does not decide what design is best.

## Required boundaries

Keep enrichment code and environments outside the working repository. Read the repository's manifests/lockfile with your normal tools; supply the relevant dependency context to the service. Do not install research dependencies into the project. Do not send private project text or credentials to Context7 when a generic technical question will suffice.

Discover actual connected tool names: clients may prefix them. Service tools are `resolve_library`, `library_overview`, `search_evidence`, `inspect_symbol`, `compare_releases`, `verify_usage`, `read_artifact`, `job_control`, and `service_status`. Context7 normally exposes `resolve-library-id` and `query-docs`; inspect its current schema rather than inventing IDs or parameter names.

The service must already be implemented and registered. Installing this skill alone does not install the service. If unavailable, use version-matched official documentation and ordinary repository tools, state the missing enrichment capability, and do not fabricate tool results.

## Workflow

### 1. Frame the research

Identify the design outcome, constraints, ecosystem/binding, current dependency version, and uncertainty to resolve. Distinguish:

- Project implementation: exact project version, features/extras, compiler/interpreter, and target.
- Upstream exploration: newest eligible release, verified through registry resolution.
- Upgrade comparison: separate before/after contexts.

Do not silently turn an implementation question into a dependency upgrade. A Rust crate and its Python bindings may expose different APIs.

### 2. Resolve identity first

Call `resolve_library` with the selected mode and available environment constraints. Use the returned `context_id`. Preserve exact version/artifact identity and the environment's resolution status. If dependencies/features are incompletely known, say so; do not describe the context as verified.

Use `freshness="revalidate"` for explicit latest/upstream questions. A cache hit alone does not establish that a release is still latest. For repeated reads, reuse a `snapshot_id` when reproducibility matters. Never combine snapshots/contexts without identifying the difference.

### 3. Choose breadth or depth

For a known symbol or focused usage question, go directly to Context7 and/or `inspect_symbol`; avoid whole-library enumeration.

For design review or unfamiliar capability discovery, call `library_overview` for the module/feature map, documentation headings, and relevant example/release-note index. Build a small candidate shortlist. Search both the desired outcome and plausible mechanisms. Do not conclude that a capability is absent simply because an initial semantic query found nothing.

For an upgrade, call `compare_releases` for API, configuration, documentation, and release-note changes. An unchanged API does not prove unchanged behavior or performance.

### 4. Retrieve explanatory context

Resolve the correct Context7 library entry and ask narrow, concept-specific questions. Match language binding and release where supported. Treat uncertain-version snippets as discovery leads, not exact-version proof.

Use official documentation through enrichment when Context7 lacks the relevant page, omits a prerequisite, or supplies a mismatched version. Do not ask Context7 to reproduce an entire reference manual.

### 5. Characterize candidate deployment

Use `search_evidence` and `inspect_symbol` to establish the concrete mechanism, signature, required features/extras, initialization/configuration, relevant relationships, examples, and constraints. Use explicit source, semantics or runtime aspects only when signature and documentation leave a material question unanswered. Retained inspection does not execute; execution requires an explicit intent and an enabled profile.

For Rust, separate docs.rs' observed build configuration from the project's feature/target configuration. For Python, separate distribution/import identity, inline or stub types, static API observations, and runtime behavior. An unresolved alias, unsupported LSP method, or missing native signature is an evidence gap, not proof of absence.

Read `coverage.assessments` and any `coverage.details` action. Each collection carries its own `page` with `count`, `has_more` and `next_cursor`; inspection aspects have independent pages. `text_complete=false` identifies a preview and supplies its complete-text action. Artifact delivery preserves the research outcome and offers direct result sections. Do not treat a first page as the entire result set.

### 6. Verify consequential assumptions

For uncertain or version-sensitive deployment, propose a minimal snippet and use `verify_usage` with the appropriate mode. Compile/typecheck and runtime verification establish different claims. Execute only through an authorized profile; do not weaken policy or import unfamiliar libraries into the working environment to bypass it.

A `pending` result is a submitted job, not finished evidence. Use `job_control` with bounded waits and the suggested polling interval. Terminal `data.result` is a compact outcome plus delivery descriptor; follow that descriptor to the retained answer. If blocked, failed, or unverified, carry that limitation into the brief.

### 7. Synthesize and stop

Return a compact, design-oriented brief. Separate source-backed facts from your judgments. Explain why each candidate is relevant, not just what its API is. State exact versions and required configuration. Include evidence IDs/source locators, verification scope, meaningful alternatives, and unresolved assumptions.

Stop when the decision is adequately supported. Do not recursively index every module or generate another comprehensive library-reference document. Preserve a small reusable research bundle only when useful and when the user has authorized its destination; do not create project files merely because this skill ran.

## Output shape

Use a concise narrative or comparison table covering:

1. Recommended capabilities and the design outcomes they address.
2. Deployment mechanism, exact-version/configuration requirements, and important composition constraints.
3. Evidence and verification: what is established, what is inferred, and what remains open.

A short implementation pattern is useful; an API dump is not. Never assign unsupported numeric confidence or call a library feature “fully verified” based only on a retrieved example.

## Additional references

Read `references/tool-contract.md` for tool selection, result states, pagination, and job handling.
Read `references/evidence-policy.md` when sources conflict, a feature appears absent, or environment/version matching is uncertain.
