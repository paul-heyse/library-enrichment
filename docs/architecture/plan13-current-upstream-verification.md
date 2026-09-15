# Plan 13 current upstream verification for ADR-0037 and ADR-0038

Retrieved **2026-09-15**. This bounded source review supplements
[the earlier verification and probes](plan13-upstream-verification.md). It verifies
upstream interfaces, not product acceptance or ADR acceptance. No broad tests or
production operations were run for this note.

## Version and source boundary

`Cargo.lock` names DataFusion **55.1.0** and Arrow **59.3.0**. The exact downloaded
sources inspected below are under
`/home/paul/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/` (abbreviated
`REGISTRY/`). `uv run --no-sync python` with `importlib.metadata.version` reported
FastMCP **4.0.3**, fastmcp-slim **4.0.3**, MCP **2.2.0**, and Pydantic **2.13.5**.
Installed Python sources are under
`/home/paul/library-enrichment/.venv/lib/python3.14/site-packages/` (abbreviated
`SITE/`). Release URLs below identify the corresponding primary-source locations;
the verdicts come from those exact local sources read in this review.

Context7 resolution selected `/apache/datafusion` and `/prefecthq/fastmcp`, following
[blueprint sources S01–S06](../blueprint/SOURCES.md) for FastMCP. Focused queries
covered analyzer/optimizer boundaries, struct aggregation, and progress/middleware
result handling. DataFusion results referred to `main`; FastMCP offered only
`v3.2.0` and `v3.2.4` as versioned IDs. These are discovery leads, not proof of the
selected releases. In particular, generic middleware documentation encouraging
exceptions does not replace inspection of structured `ToolResult` behavior.

## Exact-source findings

All rows were retrieved on **2026-09-15** and have verdict **verified (source)**.

| Claim and implementation consequence | Exact local locator | Primary source and short exact quote |
|---|---|---|
| The ordinary logical pipeline runs analysis and then optimization. Public accessors allow separate observation while retaining the configured engines and default rules. | `REGISTRY/datafusion-55.1.0/src/execution/session_state.rs:651–657,758–764` | [SessionState 55.1.0](https://github.com/apache/datafusion/blob/55.1.0/datafusion/core/src/execution/session_state.rs): `self.optimizer.optimize(analyzed_plan, self, \|_, _\| {})` |
| Native analyzer checks invariants and offers an observer. Default rules include grouping-function resolution and type coercion; do not replace them with an empty/custom-only analyzer. | `REGISTRY/datafusion-optimizer-55.1.0/src/analyzer/mod.rs:88–94,118–132` | [Analyzer 55.1.0](https://github.com/apache/datafusion/blob/55.1.0/datafusion/optimizer/src/analyzer/mod.rs): `pub fn execute_and_check<F>`; `Arc::new(TypeCoercion::new())` |
| `SessionState::create_physical_plan` first optimizes. After explicit analyzer and optimizer passes, call the configured query planner to avoid an accidental second pipeline. | `REGISTRY/datafusion-55.1.0/src/execution/session_state.rs:684,768–779` | [SessionState 55.1.0](https://github.com/apache/datafusion/blob/55.1.0/datafusion/core/src/execution/session_state.rs): `this first calls` |
| Native logical type compatibility deliberately omits properties that are consequential service contracts. Independently check required field names, nullability, selected metadata and nested shape. | `REGISTRY/datafusion-common-55.1.0/src/dfschema.rs:744–748` | [DFSchema 55.1.0](https://github.com/apache/datafusion/blob/55.1.0/datafusion/common/src/dfschema.rs): `ignoring both metadata and nullability` |
| Scalar UDF return-field inference can preserve explicit nullability/metadata; the default nullable field is insufficient for a guaranteed non-null identity result. | `REGISTRY/datafusion-expr-55.1.0/src/udf.rs:690–698` | [ScalarUDFImpl 55.1.0](https://github.com/apache/datafusion/blob/55.1.0/datafusion/expr/src/udf.rs): `Field::new(self.name(), return_type, true)` |
| Native aggregate grouping supports compound Arrow keys through row encoding. Struct encoding includes parent validity and each child, supporting a full typed source key instead of a detached hash or display label. | `REGISTRY/datafusion-physical-plan-55.1.0/src/aggregates/group_values/row.rs:86–99`; `REGISTRY/arrow-row-59.3.0/src/lib.rs:313–319,712–727` | [GroupValuesRows 55.1.0](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/aggregates/group_values/row.rs): `RowConverter::new`; [Arrow row 59.3.0](https://github.com/apache/arrow-rs/blob/59.3.0/arrow-row/src/lib.rs): `row encoding of each child` |
| Foreground progress awaits notification when a token exists, with no local catch around that await. A transport exception can propagate. Isolate optional notification failure from the completed research result; preserve cancellation semantics. | `SITE/fastmcp/server/context.py:453–491` | [Context 4.0.3](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/server/context.py): `await self.session.send_progress_notification(` |
| Tool middleware sees parsed argument mappings before function binding. Resource reads have their own hook; tool-hook correlation does not automatically cover reads. | `SITE/fastmcp/server/server.py:1413–1438`; `SITE/fastmcp/server/middleware/middleware.py:239–251` | [Server 4.0.3](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/server/server.py): `arguments=arguments or {}`; [Middleware 4.0.3](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/server/middleware/middleware.py): `async def on_read_resource(` |
| Strict binding reaches Pydantic; an explicit output schema is selectable. Explicit ToolResult conversion passes through without proving that its structured content matches that schema. Validate authored output models and the generated domain contract before constructing every result. | `SITE/fastmcp/tools/function_tool.py:337,474–481`; `SITE/fastmcp/tools/base.py:367–375` | [FunctionTool 4.0.3](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/tools/function_tool.py): `type_adapter.validate_python(arguments, strict=strict)`; [Tool 4.0.3](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/tools/base.py): `return raw_value` |
| MCP client output validation excludes error results. Client validation cannot certify all service outcomes. | `SITE/mcp/client/session.py:1101–1102` | [ClientSession 2.2.0](https://github.com/modelcontextprotocol/python-sdk/blob/v2.2.0/src/mcp/client/session.py): `and not result.is_error` |

## Doc alias qualification and limits

The current `search_plan.rs` groups and windows fragments by
`coalesce(definition_id, fragment_id)`, kind, text and the complete typed `source`,
then rejoins source with `IS NOT DISTINCT FROM`. This is a conservative application
of native struct grouping: identical prose from different producer/artifact/locator
or epistemic sources remains distinct. A fragment without a definition falls back
to its own fragment identity. Labels are collected independently only after this
qualified source group has been established.

This grouping does **not** prove two different source records equivalent, and it
does not authorize global text deduplication. The product tests must exercise
compatible aliases, independent equal-text sources, null fields inside sources,
stable ranking and complete cursor traversal. This review did not execute those
tests. It also does not establish memory/performance bounds for high-fanout groups;
that remains a measured W11 disposition.

## Review implications

ADR-0038 can rely on configured native analysis, optimization and Arrow schemas
without introducing a custom analyzer or type system. It still needs independent
query-family contracts and witnesses, including checks before physical planning.
ADR-0037 can rely on authored presentation schemas and explicit structured results,
but validation must run on successful, partial, pending and error outcomes. Optional
progress and resource-link presentation do not own the durable result outcome.

The selected upstream interfaces support these decisions. Product fault injection,
raw-stdio validation, export closure, actual client qualification and closing design
review remain separate obligations; this note promotes none of them to passed.

## Implementation continuation — 2026-09-15

Context7 `/apache/datafusion` subquery guidance was used again for discovery; it returned `main`
and no 55.1-specific version ID. Exact 55.1.0 native execution rejected a projected `EXISTS`
expression in the revision-disposition union. The implementation now uses explicit joins,
`UNION` and `count(*) > 0`, verified by the native revision disposition fixture. This is a
specific lowering limitation, not a claim that DataFusion lacks predicate-subquery support.
The pinned optimizer's `decorrelate_predicate_subquery.rs:18,45` documents predicate lowering
into semi/anti joins. [55.1.0 optimizer source](https://github.com/apache/datafusion/blob/55.1.0/datafusion/optimizer/src/decorrelate_predicate_subquery.rs).

Arrow **59.3.0** `arrow-json/src/writer/mod.rs:398–411` encodes an entire row into a `Vec<u8>`
before checking its flush threshold; `writer/encoder.rs:297–301` likewise takes `&mut Vec<u8>`.
A bounded underlying `Write` does not bound that row allocation. Comparison delivery therefore
uses borrowed serialization of its closed Arrow scalar/list/struct shapes into the existing
bounded blob sink. It does not add a general Arrow JSON framework or reimplement selection,
comparison or ranking. [Arrow 59.3.0 JSON writer](https://docs.rs/arrow-json/59.3.0/src/arrow_json/writer/mod.rs.html).
The fixture preserves a value whose JSON escaping exceeds the former 16 MiB render ceiling,
with exact canonical digest, Unicode, nested nulls and repeatable content addressing.

`LogicalPlan::apply_with_subqueries` and `apply_expressions` are present in pinned
`datafusion-expr-55.1.0/src/logical_plan/tree_node.rs:799,418`. The bounded diagnostic inventory
uses those native visitors over the optimized plan, with scalar/aggregate/window function names
and UDF identity metadata actually present in the plan. It does not query an unrelated session
or infer that an engine inventory is complete library evidence.
[55.1.0 logical-plan tree traversal](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/logical_plan/tree_node.rs.html).


### Declared revision input paths (2026-09-15)

Context7 `/rust-lang/cargo` was used for discovery; its master snippets are not exact-version
DataFusion evidence. The [Cargo workspace reference](https://doc.rust-lang.org/cargo/reference/workspaces.html)
and [manifest reference](https://doc.rust-lang.org/cargo/reference/manifest.html), retrieved
2026-09-15, establish ancestor/explicit workspace discovery and workspace-relative inherited
readme/license paths. The collector records declared local dependency candidates and file paths,
not a Cargo resolver result; no registry fetch, metadata subprocess or build runs during collection.
Actual DataFusion **55.1.0** joins still decide affected omission membership from the resulting
flat Arrow relation. `.dev-state/plan13-source-roots-native.log` records the expanded native cases.


### Operation-local native reuse (2026-09-15)

Context7 `/apache/datafusion` confirmed lazy DataFrame execution and default cache collection;
its main snippets are discovery. Exact Cargo-registry DataFusion **55.1.0** source confirms:

- `datafusion/src/dataframe/mod.rs:2412`: default `cache()` calls `collect_partitioned` and
  constructs a MemTable. It is not used to bypass the service's allocation policy.
- `datafusion-physical-plan/src/windows/window_agg_exec.rs:496`: the default global window
  stream retains input batches. A global count window is not assumed to be a free count/page
  reuse strategy.
- `datafusion-physical-plan/src/spill/mod.rs:20–30`: SpillManager's public name is a hidden
  doctest re-export. The implementation does not depend on that hidden surface.
- Public `datafusion-execution/src/disk_manager.rs` and `spill_file.rs` supply quota-accounted
  spill file lifetime, writers and asynchronous byte reads. The OS writer at lines 478–505
  returns a string-backed Other I/O error when its quota fails; it does not preserve a typed
  ResourcesExhausted source. Counter-based index preflight adds a typed witness where provable,
  without replacing native atomic enforcement or interpreting error strings.
- Arrow **59.3.0** StreamWriter/StreamDecoder implement IPC encoding and incremental validation;
  the index does not add an IPC codec or skip Arrow validation.
- [DataFusion 55.1.0 StreamingTable](https://docs.rs/datafusion/55.1.0/datafusion/catalog/streaming/struct.StreamingTable.html),
  retrieved 2026-09-15, supplies a finite provider from PartitionStream values. Native physical
  filter/projection/limit execution consumes the private index; no custom relational execution
  operator is introduced.

Focused native probes establish actual null/duplicate preservation, replay after dropping the
original handle, empty-index behavior, quota failure and release of disk/memory reservations.
The [measurement report](../reports/plan13-native-operation-measurements-2026-09-15.md) records
observed equal-result reuse and its limited fixture scope.

### Native semantic scope and spill-reader buffer (2026-09-15)

Context7's main documentation confirms `COUNT(*) FILTER` and `UNNEST` as discovery leads.
The actual DataFusion 55.1.0 native scope fixture evaluates empty/object-only classes, absent
Python payloads, conflicting declarations and unresolved bases through the same query used by
the semantic producer. This replaces that producer's dependence on a hydrated `Symbol.python`.
The qualification's implementation digest is included in producer identity. It is not a claim
to infer exhaustive structural protocol implementors.

Pinned `datafusion-execution-55.1.0/src/disk_manager.rs:534–558` opens an independent file reader
and allocates a 128 KiB `ReaderStream` buffer. Operation-index reader reservations now include
that buffer even for small batches; the earlier 64 KiB allowance understated this native
allocation. `arrow-ipc-59.3.0/src/reader/stream.rs:159–285` consumes available input until it can
return a record batch or needs more bytes. Validation remains enabled. Multi-batch index and
large namespace selection outcomes are separate from the source-level API verification.

## Installed client boundary observations — 2026-09-15

Actual Claude traces from `.dev-state/plan13-client-complete/recovery-claude/` retained error text
while hiding structured content on MCP errors. The raw FastMCP 4.0.3 frame still held canonical
structured output. The adapter therefore projects native error code/cause/stage/retryability and
recovery into one bounded text block; terminal artifacts keep their full read action. This is a
measured host integration behavior, not a claim that the protocol discards structured errors.
The successor installed recovery trace and independent native journal witness are under
`.dev-state/logs/clients/run-snt4jm4k/recovery-claude/`; that journey passed.

Codex's isolated client configuration gives its research tools explicit per-tool approval modes
while retaining truthful effect annotations. Current official configuration documents
`mcp_servers.<id>.tools.<tool>.approval_mode` with `auto`, `prompt`, `writes` and `approve` values:
[Codex configuration reference](https://learn.chatgpt.com/docs/config-file/config-reference),
retrieved 2026-09-15. The throwaway client profile changes no operator configuration or service
policy. A rejected wait duration remains a pre-admission argument failure; it does not establish
that the referenced durable job failed.

The installed Python candidate was assembled with locked uv dependencies, `--no-editable`, an
explicit `UV_PROJECT_ENVIRONMENT`, and isolated Python startup. Context7 `/astral-sh/uv` supplied
discovery; installed uv 0.12.13 help and the actual non-editable setup/import plus raw-MCP campaign
proved the chosen setup. FastMCP is 4.0.3 in that installation; Context7's 3.2.x examples were not
used as version-specific setup proof.
