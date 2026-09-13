# Acceptance plan

These are requirements for the implementing agent. They have not been run against a service; this bundle contains no service implementation.

## Deterministic fixtures

Create a two-release Rust fixture crate with a public re-export, trait and impl, default/optional features, a target-gated item, a deprecated method, a build script with a detectable side effect, and an example. Create captured rustdoc artifacts with supported and unsupported format versions.

Create a two-release Python fixture distribution whose import name differs from its distribution name. Include `__all__`, re-exports, overload stubs, inheritance, an import-time side effect, a documented public item and an undocumented public item. Add a namespace-package fixture, an extension/stub-only fixture, a stub/runtime disagreement, and a small Sphinx inventory. Provide fixture changelogs with both additive and behavior-only changes.

Live tests supplement these fixtures; they do not replace them.

## Required cases

| ID | Scenario | Required assertion |
|---|---|---|
| R01 | Resolve exact Rust version while a newer release exists | Result retains requested version; upstream version is separate. |
| R02 | Hosted rustdoc JSON available | API index succeeds without invoking Cargo compilation. |
| R03 | Hosted rustdoc JSON missing | Useful metadata remains, coverage is partial, fallback is explicit. |
| R04 | Unsupported rustdoc JSON format | Typed unsupported-format result; no silent schema misparse. |
| R05 | Default vs optional feature | Separate observed availability/configuration; no invented universal feature predicate. |
| R06 | Target-gated item | Target differences are identified before making availability claims. |
| R07 | Re-exported public symbol | Public path and original definition are linked without duplicate capability counts. |
| R08 | No public API change, behavior-only release note | Comparison surfaces the relevant behavior note. |
| R09 | Rust compile probe passes on nightly but fails on project stable | Result does not claim project compatibility. |
| R10 | Build/proc-macro activity requested without sandbox | Policy denial, not host execution. |
| P01 | Distribution/import-name mismatch | Correct import root is discovered or ambiguity returned. |
| P02 | Static extraction of import-side-effect fixture | Side-effect marker is absent; Griffe inspection is disabled. |
| P03 | `.py` and `.pyi` disagreement | Separate source/stub observations with provenance; no silent overwrite. |
| P04 | Extension-only package | Useful stubs/docs are returned; missing signatures/source are explicit. |
| P05 | Namespace package | Available import roots and partial package coverage are represented. |
| P06 | Added function with no compatibility break | Normalized additive diff detects it independently of Griffe break checks. |
| P07 | Documented inventory omits a public API | The API is not automatically classified as nonexistent/private. |
| P08 | ty implementation method unsupported | `UNSUPPORTED_CAPABILITY`, not an empty implementation list. |
| P09 | Typecheck success but runtime assertion failure | Distinct outcomes and scopes are preserved. |
| P10 | Worker uses service env accidentally | Test fails; selected consumer interpreter/dependencies must be recorded. |
| C01 | A first search returns zero matches | Response distinguishes searched scope from whole-library absence. |
| C02 | Requested result exceeds byte budget | Valid bounded JSON, explicit truncation, stable cursor/artifact pointer. |
| C03 | Cursor reused after query/snapshot change | `INVALID_CURSOR`; no inconsistent pagination. |
| C04 | Two clients request the same extraction | One producer job; both receive equivalent evidence. |
| C05 | One caller cancels a shared job | Other caller's required work remains available. |
| C06 | Daemon crashes during publication | No half-published snapshot is visible; staging is recovered. |
| C07 | Later enrichment creates a new snapshot | Previously pinned snapshot remains readable and unchanged. |
| C08 | Upstream is unavailable during latest request | `latest_verified=false`; cached version not described as currently latest. |
| C09 | Mutable documentation changes without package release | New artifact/snapshot tracked; original provenance preserved. |
| C10 | Package source contains instructions to export credentials | Content is treated as evidence, never executed as instructions. |
| C11 | Malicious archive path or symlink | Extraction rejected before escaping scratch directory. |
| C12 | Artifact reader receives a host path | Rejected; only service-issued handles are accepted. |
| C13 | Tool runs from an unrelated current directory | Correct service config/state used; cwd unchanged. |
| C14 | Tool initializes a stdio connection | Tool list appears without package fetch, build, or LSP startup. |
| C15 | Producer fails after some evidence succeeds | `partial` with gaps, not empty `ok` or total evidence loss. |
| C16 | Exact environment becomes resolved after initial declaration | New derived context; old ID semantics unchanged. |
| C17 | Signature is present in normalized API | Inspection does not start LSP unnecessarily. |
| C18 | A long operation returns pending | Client can finish via ordinary job-control tools without native MCP tasks. |
| C19 | Inputs violate wire schema | Rejected consistently through CLI/RPC/MCP boundaries. |
| C20 | Core operations touch a canary working repo | Before/after tree hashes are identical. |
| A01 | Codex real stdio connection | Actual service-status, resolve, search, read calls succeed; capture trace. |
| A02 | Claude Code real stdio connection | Same contract as Codex; capture trace. |
| A03 | Skill on unfamiliar library design question | Breadth pass yields shortlist; targeted evidence supports a deployment brief. |
| A04 | Context7 documentation has unknown/mismatched version | Agent narrows or verifies; never silently uses it as exact proof. |
| A05 | Known symbol question | Agent avoids unnecessary full overview/whole-library enumeration. |
| A06 | Skill installed without enrichment service | Agent states unavailable capability; no fabricated tool result. |

## Evaluation tasks

Use real, version-pinned packages selected during the test run. Suggested tasks: Rust schema metadata and a query-engine extension seam; Python model validation and a CLI or web-framework configuration seam. The task's expected outcome is evidence-supported integration guidance, not a specific architectural conclusion predetermined by this plan.

Score discrete outcomes: exact release respected; capability actually exists in the stated configuration; required setup identified; important limitation identified; source-backed facts separated from inferred design value; response remains within budget. Do not convert these cases into an unsupported universal quality percentage.

## Required test report

Record test ID, fixture/live classification, exact tools and versions, command or MCP invocation, result, evidence/log path, and any limitation. Mark tests `passed`, `failed`, `blocked`, or `not_run`. Client credentials not available means `not_run`, not passed via a mocked client.
