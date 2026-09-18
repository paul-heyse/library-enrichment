# Tool routing and contracts

These are contracts for the library-enrichment service specified in this bundle. Inspect the installed service's actual schemas; do not assume a skill installs these tools.

| Need | Tool | Avoid |
|---|---|---|
| Exact release/environment | `resolve_library` | Guessing latest or upgrading a pinned project |
| Unfamiliar features | `library_overview` | Dumping all public symbols |
| Find candidates/examples | `search_evidence` | Treating no match as no capability |
| Characterize a candidate | `inspect_symbol` | Starting LSP for every signature |
| Investigate an upgrade | `compare_releases` | Looking only for breaking changes |
| Test a proposed usage | `verify_usage` | Equating type correctness with runtime behavior |
| Read large supporting output | `read_artifact` | Requesting unrestricted filesystem paths |
| Complete an expensive request | `job_control` | Treating a pending handle as evidence of success |
| Check service/tool availability | `service_status` | Repeating status before every cached read |

Every research result identifies its actual context and snapshot. `ok` is success within the stated coverage. `partial` contains usable evidence and gaps. `pending` contains a job handle. `error` includes a stable error code and next action.

Use source locators and evidence IDs from results; do not invent citations. A useful citation names the library/version, artifact or source URI, and the relevant symbol/section/line range. A research brief may include both public upstream references and service evidence handles.

For pagination, pass the returned cursor with the same snapshot, query, filters, and sort. If you change the question, start a new search. Use `read_artifact` to retrieve only the sections supporting the current decision.

For long jobs, poll the supplied job ID with bounded waits. Cancel only your request interest; the service may share the job with another agent. Report persistent errors without silently switching versions or disabling isolation.

Tools may create service-owned caches or jobs, but none should modify the working repository. `verify_usage` and analysis involving build scripts/imports are execution, even if their purpose is inspection.

## Research contract 12.0

Read the installed catalog for exact schemas. Removed depth/aspects arguments and old root
pagination are rejected. Selection, scope assessment, cursor checks, policy, enforced byte
limits and durable results belong to the Rust service. Full-range UInt64, Int64 and native
size/count values use decimal JSON strings in wire 12.0, including `max_items`, `max_bytes`,
`wait_seconds` and pagination counts. Follow the generated field schema for smaller numeric types.

Job, caller-interest and producer-attempt identifiers use distinct `job_`, `interest_` and
`attempt_` prefixes followed by exactly 32 lowercase hexadecimal digits. Preserve the returned
identifier; a job ID is not an interest token or attempt ID. Job clocks use canonical UTC with
six fractional digits. The generated schema is the authority for every field. Older wire
epochs are refused; there is no compatibility reader.

### Focused selection

Default `inspect_symbol` selects signature, availability and a small documentation preview.
Each explicit aspect is independent: signature, availability, relationships, documentation,
examples, source, semantics, runtime, children or members. Children are lexical path children;
members come from retained membership relationships. An omitted aspect is not absent evidence.

```json
{"tool":"inspect_symbol","arguments":{"context_id":"ctx_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","symbol_path":"datafusion::execution::context::SessionContext","selection":{"mode":"explicit","aspects":[{"aspect":"signature","max_items":"8"},{"aspect":"members","max_items":"8"}]}}}
```

Preserve context/snapshot and the aspect's limits when supplying its cursor. Documentation and
examples may set `max_characters`; null requests the complete retained text. A preview's
`complete` action carries the correct full-text request, including its page position.
Semantic/runtime aspects read retained evidence unless `execution.intent` explicitly requests
`execute_on_miss` or `rerun`. Match that intent to the selected aspect. Runtime execution also
requires `execution.runtime` and a qualified runtime profile. Check `service_status` when an
actual refusal names a prerequisite; do not repeat status before every read.

`library_overview.discovery` independently pages features, README sections, release notes and
examples. Omit it for bounded previews, or pass `[]` for namespace navigation alone. Conflicting
same-name definitions retain separate source identities. `area` narrows namespaces; discovery
facets describe library-level source documents.

### Coverage, counts and failures

`coverage.assessments` distinguishes indexed, partial, missing and unknown for each requested
scope. Indexed empty results establish only that the selected domain was evaluated. Unknown
coverage and empty lexical matches do not establish capability absence. A count's `kind` is
exact, lower_bound or unknown. In a comparison, inspect both sides' coverage and each changed
key's independently paged before/after alternatives. Each alternative retains its own source.
Its `value.mode` is `inline` (the complete JSON is in `value.value`) or `artifact` (read
`value.artifact.receipt.artifact_id` with `read_artifact` and verify `size_bytes` and `sha256`).
A representation change is not automatically a proven breaking change.

`error.diagnostic` names the cause, stage, affected IDs, rule, limits, correlation ID and typed
recovery actions. Follow that action. An unknown job calls for a valid job ID; a malformed
response calls for a defect report. Repeating a request or repairing storage is appropriate
only when its diagnosed cause supports that step. An adapter timeout does not cancel a job.

### Artifact receipts

An issued artifact carries `receipt.artifact_id`, the complete SHA-256 digest and byte length,
media type, source URI and retrieval provenance. Pass `receipt.artifact_id` to `read_artifact`.
Identical bytes can have different source-qualified acquisition receipts. A receipt describes
exact bytes; it does not by itself establish that their claims apply to another release.

### Direct result retrieval

`delivery.mode="artifact"` preserves the research status. Use `delivery.read`, or select one
of its listed result sections. `coverage.details` points to retained limitation text when it
cannot fit beside the inline scope assessment. Text previews and resource links are optional;
the structured result always carries the portable tool action.

```json
{"tool":"read_artifact","arguments":{"artifact_id":"art_0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","section":{"kind":"result","name":"signature"},"max_bytes":"4096"}}
```

Result sections are envelope, coverage, signature, changes, aspects or data, when listed in the
descriptor. `delivery.read` selects `envelope`, the complete research answer. A whole retained
result artifact is a native Arrow IPC index plus encoded sections; do not parse its raw bytes as
JSON. Parse the selected `envelope` section directly as the response object. Section names share
the descriptor's one artifact identity.
Markdown selection instead uses `{"kind":"markdown","heading":"Heading"}`. Each artifact
page reports a digest and encoding. Follow `data.page.next_cursor` until `has_more=false`; retain
the same section and limits, and decode base64 when indicated. `delivery.limits` reports the
requested and effective complete response caps. For MCP stdio, the native selection includes
the actual JSON-RPC framing and resource text escaping. There is no additional fixed framing
allowance. A cap too small for required recovery/identity fields produces a declared refusal
with the required minimum; increase the cap rather than discarding those fields.

A pending result carries a durable job handle. Preserve its interest token when present.
Terminal `job_control.data.result.outcome` is a tagged record: `status` is `ok`, `partial`, or
`error`; only the error alternative carries `error`. The result carries its scope and delivery
beside that outcome. Cancellation affects only the supplied caller interest. A `call_tool`
recovery action carries a typed `request` with `method` and `params`.

The workflow resource is `library-evidence://workflow`. Artifact, overview, snapshot-manifest
and job resources expose the same validated reads as their tools. Resource support and progress
notifications are optional; ordinary tool calls complete the same workflow.

### Typed evidence and public bindings

Citations preserve `subject` as a discriminated native reference and `source` as the full
qualified provenance record. Read epistemic class, producer binding, extractor/version,
artifact, source URI, version match and typed `locator` from `source`. `display_subject` is
a label only. A missing source URI does not change the recorded version-match assessment.

Inspection's `symbol` is an identity-only `SymbolHeader`. Signatures, documentation, cfg
hints, deprecation and producer coordinates belong to the independently qualified
`observations`. An unknown observation is never a fabricated zero producer item.

Comparison alternatives have a declared `axis`: `api`, `fragment`, `rust_documentation`,
`python_header`, or `relationship`. API observations retain typed callable/parameter facts under
`observation.payload`; signatures remain producer renderings. Inline values and complete value
artifacts share this schema. A missing observation remains null and never proves absence.
