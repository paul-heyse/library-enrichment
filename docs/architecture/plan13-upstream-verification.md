# Plan 13 upstream verification

Retrieved and probed **2026-09-15**. Scope: the FastMCP boundary in W9 and the
DataFusion field and set-operation APIs in W2/W5 of
[Plan 13](../plans/13-datafusion-research-operations-hard-pivot.md).

## Decision

The installed releases support the required integration. Keep the existing pins:
**FastMCP / fastmcp-slim 4.0.3, MCP / mcp-types 2.2.0, DataFusion 55.1.0**.
This is verification of the selected interfaces, not a dependency upgrade or proof
that the replacement service already implements them correctly.

Two implementation constraints matter immediately:

- Validate original tool arguments against the generated request contract in
  `Middleware.on_call_tool`, before calling the next handler. Also enable
  `strict_input_validation=True`; it controls strict Pydantic binding, which is
  a separate check from the Rust-generated JSON schema.
- Validate every structured result in the adapter before emitting it. An explicit
  `ToolResult` passes through FastMCP conversion, and the installed MCP client skips
  output-schema validation when `is_error=True`. A schema advertisement therefore
  cannot establish the validity of error payloads.

## Source selection and exact-version boundary

The starting authority was [blueprint sources S01–S06](../blueprint/SOURCES.md).
Context7 resolution selected `/prefecthq/fastmcp` and `/apache/datafusion`.
FastMCP resolution advertised only `v3.2.0` and `v3.2.4`; its unversioned results
were useful discovery leads, but did not establish 4.0.3 behavior. Three focused
queries covered strict argument validation, structured output, and UDF fields.
The supplied 4.0.0 capability reference remains useful; no blanket claim of API
equivalence between all of 4.0.0 and 4.0.3 is made here.

Exact installed `METADATA` files were read for FastMCP, fastmcp-slim, MCP and
mcp-types. The Python probes also read installed distribution versions using
`importlib.metadata.version`. Rust APIs were read from the downloaded
`datafusion-expr-55.1.0` and `datafusion-55.1.0` sources in the Cargo registry.
The corresponding release-tag pages were fetched where listed below.

## Verified claims

`verified (source)` means the exact installed source was inspected. `verified
(probe)` adds an executed upstream integration probe. Neither means a product
acceptance gate passed. The short quotes below are exact source tokens or text.

| Claim | Source and exact locator | Retrieved | Exact quote | Verdict | Selected pin / plan reference |
|---|---|---|---|---|---|
| Server strictness is explicitly configurable and reaches Pydantic argument validation. | [FastMCP function execution, v4.0.3](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/tools/function_tool.py), installed lines 124–136, 374–390, 456–483 | 2026-09-15 | `type_adapter.validate_python(arguments, strict=strict)` | verified (source and probe) | 4.0.3; W9 / blueprint S02 |
| Tool middleware sees the argument mapping before execution binding. | [FastMCP server, v4.0.3](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/server/server.py), installed lines 1411–1438; [middleware](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/server/middleware/middleware.py), lines 239–251 | 2026-09-15 | `arguments=arguments or {}`; `async def on_call_tool(` | verified (source and probe) | 4.0.3; W9 / S02 |
| A composed object schema can be passed through `output_schema`; explicit structured content is supported. | [FastMCP function registration, v4.0.3](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/tools/function_tool.py), installed lines 333–343; [tools documentation](https://gofastmcp.com/servers/tools) | 2026-09-15 | `final_output_schema = metadata.output_schema` | verified (source and probe) | 4.0.3; W1/W9 / S02 |
| An explicit error result retains structured data and sets the MCP error flag. | [ToolResult, v4.0.3](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/tools/base.py), installed lines 95–190 | 2026-09-15 | `is_error=self.is_error` | verified (source and probe) | 4.0.3; W9 / S02 |
| Explicit `ToolResult` conversion does not itself validate the advertised output schema. | [ToolResult conversion, v4.0.3](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/tools/base.py), installed lines 367–380 | 2026-09-15 | `return raw_value` | verified (source and probe) | 4.0.3; W9 / S02 |
| The installed MCP client validates structured output only for non-error tool results. | [MCP client session, v2.2.0](https://github.com/modelcontextprotocol/python-sdk/blob/v2.2.0/src/mcp/client/session.py), installed lines 1101–1102, 1118–1154 | 2026-09-15 | `and not result.is_error` | verified (source and probe) | 2.2.0; W9 / S02/S06 |
| Unexpected tool exceptions can be masked explicitly; intentional FastMCP exceptions have a separate path. | [FastMCP server, v4.0.3](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/server/server.py), installed lines 300, 348–352, 1493–1556 | 2026-09-15 | `if self._mask_error_details:` | verified (source and probe) | 4.0.3; W9 / S02 |
| Context offers progress notifications and supplies a foreground notification only when a progress token is present. | [FastMCP context, v4.0.3](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/server/context.py), installed lines 453–491 | 2026-09-15 | `if progress_token is not None:` | verified (source) | 4.0.3; W9 / S02/S03 |
| Context injection and lightweight lifespan ownership are available without adding a job scheduler. | [FastMCP dependencies, v4.0.3](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/server/dependencies.py), installed lines 1–3, 284–288, 447–454, 957 onward; server constructor lines 294–298 | 2026-09-15 | `DI features (Depends, CurrentContext, CurrentFastMCP) work without pydocket` | verified (source) | 4.0.3; W9 / S03/S05 |
| Resource links can coexist with a concise text block and the structured result. | Installed `mcp_types/_types.py` lines 1310–1320 and FastMCP `tools/base.py` lines 95–190; executed `mcp.types.ResourceLink` probe | 2026-09-15 | `type: Literal["resource_link"] = "resource_link"` | verified (source and probe) | mcp-types 2.2.0; W9 / S02 |
| Unix stream readers default to 64 KiB and require an explicit limit for a larger frame contract. | [Python 3.14 streams documentation](https://docs.python.org/3.14/library/asyncio-stream.html#asyncio.open_unix_connection); installed CPython 3.14.7 `asyncio/streams.py` lines 23, 90–98 | 2026-09-15 | `limit=65536` | verified (source) | existing Python 3.14.7; W9 |
| `readuntil` distinguishes incomplete frames and configured-limit overflow. | [Python 3.14 streams documentation](https://docs.python.org/3.14/library/asyncio-stream.html#asyncio.StreamReader.readuntil); installed `asyncio/streams.py` lines 550–575 | 2026-09-15 | `IncompleteReadError`; `LimitOverrunError` | verified (source) | existing Python 3.14.7; W9 |
| A scalar UDF can return an Arrow field carrying precise nullability. | [DataFusion UDF, 55.1.0](https://github.com/apache/datafusion/blob/55.1.0/datafusion/expr/src/udf.rs), installed lines 457–467, 652–700 | 2026-09-15 | `fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef>` | verified (source) | 55.1.0; W2 |
| Default UDF field inference marks the output nullable, independently of specialized function behavior. | [DataFusion UDF, 55.1.0](https://github.com/apache/datafusion/blob/55.1.0/datafusion/expr/src/udf.rs), installed lines 690–699 | 2026-09-15 | `Field::new(self.name(), return_type, true)` | verified (source) | 55.1.0; W2 |
| Native distinct set difference uses a null-equal anti join. | [DataFusion logical builder, 55.1.0](https://github.com/apache/datafusion/blob/55.1.0/datafusion/expr/src/logical_plan/builder.rs), installed lines 1375–1447 | 2026-09-15 | `JoinType::LeftAnti`; `NullEquality::NullEqualsNull` | verified (source) | 55.1.0; W5 |

The source-only entries are interface checks. Progress delivery, dependency
cleanup and typed struct reconciliation still need the Plan 13 product tests.
No blueprint assumption was contradicted by these checks. In particular, the
plan already avoids treating `EXCEPT ALL` as a proven bag-difference operation.

## Executed FastMCP probes

Executed from this checkout on 2026-09-15 with
`uv run --frozen --no-sync python -` against the installed packages. These were
small in-process upstream servers with actual FastMCP `Client` connections;
they used no daemon, repository evidence, production state or mock client.
They are not raw-stdio or real-agent qualification.

### Probe A: original arguments and structured error delivery

The server used explicit strict validation and masking, a middleware validator,
and a tool with an object output schema containing two `oneOf` branches.
The middleware applied `Draft202012Validator` before `call_next`. The tool
returned explicit text and `ResourceLink` content, a structured error object,
and `is_error=True`.

Observed:

```json
{
  "versions": {"fastmcp": "4.0.3", "fastmcp-slim": "4.0.3", "mcp": "2.2.0"},
  "output_schema_registered": true,
  "is_error": true,
  "structured_content": {"status": "error", "count": 2},
  "content_types": ["text", "resource_link"],
  "raw_arguments_observed": [
    {"count": 2},
    {"count": "2"},
    {"count": true},
    {"count": null},
    {"count": 2, "extra": 1}
  ],
  "body_calls": [2]
}
```

The original mappings reached the middleware; only the valid integer mapping
reached the function body. Invalid inputs returned FastMCP validation failures.

### Probe B: strict binding

A server with strict validation and no custom argument middleware rejected
`"2"`, `true`, `null`, an extra argument, and the floating-point value `2.0`
for an `int` parameter. Each result had `is_error=True` and no structured body
payload. Test this complete boundary against Rust admission: JSON Schema's
integer semantics alone do not establish lexical integer equivalence to Rust
deserialization or strict Pydantic binding.

### Probe C: explicit output validation and masking

A tool advertised an object requiring `count`, but deliberately returned
`{"wrong":2}` through `ToolResult`. An initial probe stopped at the expected
client output-schema error; a follow-up probe caught and classified that result.

```json
[
  {"explicit_is_error": false, "client_rejected_shape": true, "exception": "RuntimeError"},
  {"explicit_is_error": true, "client_rejected_shape": false, "shape": {"wrong": 2}},
  {"unexpected_is_error": true, "masked": true}
]
```

The final row came from a tool raising an unexpected `ValueError` containing
a distinctive internal marker. The client received an error result without
that marker. This supports explicit masking; domain errors should still use
the service's validated structured diagnostics and recovery instructions.

## Integration guidance

### FastMCP

1. Keep thin explicit wrappers. Obtain generated schemas, defaults, effects and
   recovery metadata from the Rust operation catalog. Apply the generated input
   schema to `context.message.arguments` in the tool middleware before binding.
   The raw JSON-RPC envelope is still the MCP SDK's concern; the supported hook
   observes parsed argument values, not the original byte string.
2. Set `strict_input_validation=True` and `mask_error_details=True` explicitly.
   Exercise missing fields, nulls, unknown fields/enums, strings-as-numbers,
   booleans-as-integers and integral floating-point numbers across MCP and RPC.
   Return precise pre-admission validation errors when no operation exists yet.
3. Validate composed output for every admitted outcome before constructing
   `ToolResult`. Use a short explicit text preview to prevent automatic text
   serialization of the whole structured object. Preserve the sole recovery
   handle in structured output and any required readable guidance.
4. Derive `is_error` from the actual outcome. Successful artifact continuation,
   pending work and qualified partial evidence remain ordinary results. A
   terminal failed job must reveal its failure outcome, not only successful
   retrieval of its job record.
5. Use `on_call_tool` and `on_read_resource` for bounded request correlation,
   timings and unexpected-error observation. Context/DI/lifespan can own cheap
   adapter resources; catalog listing must not acquire evidence or connect to
   the daemon. Do not create parallel Python job state.
6. Forward real core stage information with `Context.report_progress` when
   available. Its optional `total` allows an honest stage/count model without
   invented percentages. A client lacking a progress token still receives the
   same result. Resource links supplement ordinary tool reads and must identify
   an actual readable resource; do not depend on client link presentation.

### Daemon framing

The Unix stream is owned by this adapter, not by FastMCP. Align its explicit
reader limit with the service's encoded-frame contract, including the chosen
delimiter accounting. `readline` accepts partial data on EOF and turns a stream
limit overrun into `ValueError`; `readuntil(b'\n')` provides separate
`IncompleteReadError` and `LimitOverrunError` paths. Keep a final encoded-length
check. Validate UTF-8, JSON object shape, JSON-RPC version, response ID and exactly
one of result/error before interpreting a domain payload. Distinguish request
oversize, connect failure, timeout, truncated frame, overlong frame, malformed
JSON and invalid response. A local timeout does not establish that a Rust job
was cancelled. Verify these against real Unix sockets in W9/J13.

### DataFusion

Implement `return_field_from_args` using `arg_fields` and, when relevant,
`scalar_arguments`; retain a coherent `return_type` implementation. Declare
parent and nested field nullability from actual UDF behavior, then test empty,
scalar, array, nullable and partitioned execution. Field precision does not
justify removing conservative filter residuals or inventing key constraints.

For flat comparison alternatives, `DataFrame.except_distinct` delegates to the
builder's distinct, null-equal anti join. Both sides need compatible typed
columns and explicit set semantics. The installed builder's `is_all=true`
branch uses a plain anti join without duplicate-count subtraction. Do not use
its name as proof of multiset semantics; use distinct alternatives or explicit
native per-value counts when multiplicity is meaningful. Struct-valued keys,
nulls, duplicates and ordering require W5's independent expected-result tests.

## Remaining product verification

- All generated per-tool schema variants and real output paths, including
  offline status and errors, need contract tests.
- Raw stdio, actual client negotiation, resource reads, progress delivery,
  frame limits and disconnect/cancellation ownership need the planned journeys.
- Native UDF nullability and flat comparison need compile and execution tests
  against the actual operation plans. No Rust probe was compiled in this
  verification subtask.
- Append the claim rows needed for the active architectural decision to the
  [compatibility matrix](compatibility-matrix.md) as required by the
  verify-upstream workflow; this document retains the complete supporting detail.

## Decision update: authored Pydantic MCP presentation

**2026-09-15, following user direction.** Python should own agent-facing
presentation models and defaults; Rust should own the requested data operation
and its truthful result. This updates the earlier recommendations that all MCP
defaults and complete output schemas be generated from the Rust catalog.
MCP and RPC schemas may differ intentionally. The invariant is that every
admitted MCP input lowers to one explicit, valid native request, and presentation
preserves the native result's meaning and recovery handles.

### Exact installed support

Installed Pydantic is **2.13.5**, read from distribution metadata and an executed
version probe. Context7 `/pydantic/pydantic` supplied discovery examples; the
release-tag source and installed package establish these APIs.

| Claim | Primary source / installed locator | Short exact quote | Verdict, retrieved 2026-09-15 |
|---|---|---|---|
| Authored request models support strict validation, forbidden extras and validated defaults. | [Pydantic config 2.13.5](https://github.com/pydantic/pydantic/blob/v2.13.5/pydantic/config.py), `strict`, `extra`, `validate_default`; [BaseModel](https://github.com/pydantic/pydantic/blob/v2.13.5/pydantic/main.py), lines 694–735 | `strict: bool`; `validate_default: bool` | verified (source and probe) |
| Generic authored models can contain generated domain DTOs; dumping can use JSON mode and output aliases. | [BaseModel 2.13.5](https://github.com/pydantic/pydantic/blob/v2.13.5/pydantic/main.py), `__class_getitem__`, lines 427–461 | `by_alias: bool`; `warnings: bool` | verified (source and probe) |
| The published schema can describe the serialization shape rather than the input shape. | [BaseModel 2.13.5](https://github.com/pydantic/pydantic/blob/v2.13.5/pydantic/main.py), lines 562–592 | `mode: JsonSchemaMode = 'validation'` | verified (source and probe) |
| Field serializers support a declared return type; model validators can enforce relationships between fields. | [Serializers 2.13.5](https://github.com/pydantic/pydantic/blob/v2.13.5/pydantic/functional_serializers.py), lines 232–246; installed `functional_validators.py` lines 668–703 | `return_type: Any = PydanticUndefined`; `mode: Literal['wrap', 'before', 'after']` | verified (source and probe) |

Use ordinary nested models and a small generic presentation wrapper; a generic
model factory or custom schema framework is unnecessary. A serializers-based
projection can change presentation names and preview formatting without changing
the domain DTO type. For an output alias, use `Field(serialization_alias=...)`.
Pair `model_dump(mode="json", by_alias=True, warnings="error")` with
`model_json_schema(mode="serialization", by_alias=True)`. JSON-schema validation
of that final dictionary remains useful for every emitted outcome, including
errors. Model validators cover presentation relationships that JSON Schema does
not automatically express, such as a resource link matching its native handle.

### Executed composition probe

An in-process probe on 2026-09-15 used actual generated `ArtifactHandle` and
`OverviewRequest` models from this checkout. An authored strict input's default
`limit=16` lowered to explicit native `max_items=16`. Four invalid input variants
(string, boolean, null and an extra field) failed validation. An authored generic
`McpView[ArtifactHandle]` nested the generated domain DTO, serialized presentation
aliases and normalized preview whitespace. Its model validator rejected a link
that differed from the native handle. Final output passed its serialization
schema and reached a real FastMCP `Client` with text and resource-link content.

```json
{
  "pydantic": "2.13.5",
  "default_lowered": 16,
  "strict_rejections": 4,
  "link_invariant_rejected": true,
  "serialization_schema_props": ["resourceUri", "result", "summary"],
  "generated_payload_unchanged": true,
  "client_result_matches": true,
  "content_types": ["text", "resource_link"]
}
```

This establishes composition support, not the final Plan 13 contract: the probe
used the generated DTO generation present at execution time. New native variants
must be regenerated and qualified. Ensure generated DTO parsing preserves opaque
identity, URI and timestamp values where their exact representation matters;
Pydantic types can normalize values. Keep original native values when needed,
or fix generator configuration rather than editing generated models. Strict
Python-object parsing of enum/date objects can also differ from JSON-aware
parsing; test actual serialized native payloads rather than assuming equivalence.

### Smallest ownership boundary

| Rust domain and execution ownership | Python MCP presentation ownership |
|---|---|
| Typed operations, supported selections, argument validity and policy admission | Authored MCP request models, helpful defaults and explicit lowering into native selectors |
| DataFusion/Arrow queries, physical data limits, deadlines, cancellation and resource ownership | Tool descriptions, MCP annotations derived from actual effects, request validation messages |
| Coverage, outcome, diagnostics, provenance, stable IDs and canonical identity preimages | Names and aliases, previews, recovery wording, readable summaries and resource links |
| Page membership/order, opaque cursor validation and durable result sections | Typed output wrappers composed around generated native domain DTOs; MCP content-block layout |
| Job/result publication, durable handles and typed recovery prerequisites | Mapping native outcomes to `ToolResult.is_error`, reporting observed progress, optional presentation choices |
| Bounded RPC serialization and durable overflow before transport | Final MCP encoded-size accounting and formatting inside the adapter's own bounded response |

The native result must stay independently useful to CLI/RPC consumers. Python
must not infer missing coverage, downgrade native failures, mint cursors, invent
recovery eligibility, or acquire domain data merely to customize presentation.
Presentation defaults select valid native operations; they do not replace
native validation or operator-enforced limits. Keep one active implementation
of each responsibility; the hard pivot does not require old-envelope translation.

### Formatting that can move, and bounds that must remain

Native `delivery.rs` currently repeatedly halves evidence excerpts to fit an
inline response and truncates the summary when producing an overflow descriptor.
These are presentation choices: move preview selection, preview clipping,
readable artifact labels and summary/recovery wording to the adapter. The
existing Python `_tool_result` preview formatter and fixed MCP allowance should
be replaced by the authored model plus actual MCP-size accounting. Keep native
source excerpt selection and its provenance separate: shortening the readable
preview does not change the source locator or claim a smaller evidence artifact.

Rust must still bound requested pages and native objects before serialization,
and bound encoded RPC frames before writing the socket. A Python formatter
cannot protect Rust from building or transmitting an unbounded result. Native
overflow must select a bounded typed durable-result descriptor with genuine
section/cursor handles; Rust retains atomic publication, recovery, canonical
artifact encoding and disk limits. If even mandatory identity/recovery metadata
cannot fit a native frame, return a typed minimum-capacity failure. The adapter
can reserve MCP framing/content overhead when choosing a native request budget,
then measure final output; it must never silently discard native rows to fit.

Acceptance should prove strict authored-input lowering, generated native DTO
conformance, presentation invariant checks, final serialization-schema agreement,
unchanged domain facts across tool/resource views, and native/MCP byte limits
independently. This supports the user's presentation ownership change while
keeping native data correctness and resource bounds enforceable.

## W7 follow-up: bounded tar metadata before native path decoding

Verified **2026-09-15** against locked **tar 0.4.46**, package checksum
`3f6221d9a6003c78398e3b239969f352578258df48c8eb051caadae0015bc840`, whose
`.cargo_vcs_info.json` records commit
`fc459c149f83bf4daceaa52e17d351989002e1a9`. Three Context7 resolution attempts
(`Rust tar crate`, `alexcrichton tar-rs`, `tar-rs`) returned unrelated Node/Go
libraries. No unrelated library was queried as a substitute. Exact local crate
source and the corresponding upstream commit were used instead; docs.rs page
fetches failed, while upstream GitHub fetches succeeded.

### Verified behavior and replacement boundary

- [`Entries::raw(true)`](https://github.com/composefs/tar-rs/blob/fc459c149f83bf4daceaa52e17d351989002e1a9/src/archive.rs#L261)
  exposes GNU long-name/long-link and PAX local/global headers before their
  variable-size payload is read. Default iteration instead invokes `read_all()`
  for GNU long-name/long-link and local PAX metadata before yielding its file.
  Raw iteration still reads and validates the fixed tar header; it is not
  allocation-free.
- Raw mode also bypasses extended path, link and PAX-size interpretation. Use a
  bounded raw preflight, rewind the same immutable `Read + Seek` input to its
  captured initial position, then use normal tar iteration. This keeps tar's
  path/link decoding instead of reimplementing it. Apply filesystem containment
  and omission policy to the normal pass's **effective** entry path and target.
- [`Entry::pax_extensions`](https://github.com/composefs/tar-rs/blob/fc459c149f83bf4daceaa52e17d351989002e1a9/src/entry.rs#L117)
  reads an extension entry completely. Check its declared size and charge all
  raw headers/declared bytes before calling it. The proposed 4,096-byte GNU
  metadata and 16,384-byte PAX caps are service policy, not crate defaults.
- Admit local PAX `path` and `linkpath` only with bounded, valid target text;
  these affect normal `Entry::path` and `Entry::link_name`. Accept `mtime` only
  as bounded inert metadata for this extraction policy, which does not preserve
  archive timestamps. The probe confirmed PAX `mtime` did not rewrite the
  ordinary header time. Reject duplicate keys, `size`, `uid`, `gid`, sparse
  directives and unknown semantic overrides. In particular, a PAX size override
  can change normal iteration's payload boundaries while raw iteration uses
  the header's size; rejecting it keeps both passes aligned.
- [`PaxExtensions`](https://github.com/composefs/tar-rs/blob/fc459c149f83bf4daceaa52e17d351989002e1a9/src/pax.rs#L40)
  exposes parsed keys/values, but its iterator stops on an empty line. Validate
  the bounded payload's complete framing, including a final newline and no
  interior empty record; reject every parse error. Do not treat iterator
  exhaustion alone as proof that all metadata bytes were validated. The
  existing commit-only global-comment policy remains separate from local keys.
- Normal `Entry::link_name` resolves GNU long-link/PAX `linkpath` metadata;
  `entry.header().link_name()` reads only the fixed header field and can be empty
  for a valid long target. It reports a target name, not a resolved filesystem
  object. Containment validation remains service policy; symlink targets are
  relative to the link's parent, while archive hard-link targets use the archive
  root. Revision omissions must not create or follow either type of link.

### Executed exact-version probe

Ran `cargo +1.98.1 run --offline` with an isolated manifest pinning
`tar = "=0.4.46"` under `/tmp/plan13-tar-verification-20260915/`; no product code
or extraction destination was modified. A counting reader established that raw
iteration yielded a 32-KiB GNU metadata entry after reading only its 512-byte
header. GNU and PAX paths/targets round-tripped exactly through tar's normal
decoder. A two-pass probe reused one reader starting at offset 37, then asserted
the exact 159-byte path and `hello` payload after rewind.

```text
metadata_header raw=true type=L declared=32768 reader_bytes=512
metadata_header raw=false type=0 declared=1 reader_bytes=33792
gnu_raw_entry_types=['L', '0', 'K', '2']
gnu_effective_path_length=159 effective_link_length=157 header_link_length=0
bounded_pax_metadata_keys=["path", "linkpath", "mtime"]
pax_effective_path_and_link=true pax_mtime_did_not_rewrite_header=true
same_reader_two_pass_initial_offset=37 exact_long_path_and_contents=true
global_pax_visible_before_payload=true declared=13
```

Verdict: **verified** for this conservative two-pass integration. These are
upstream behavior probes; gzip budgets, all rejection cases and revision closure
remain product test obligations. The source stream must remain immutable between
passes. Ordinary package link rejection remains a separate policy; this
verification does not authorize broadening it.

## W2 follow-up: native field contracts across planning and execution

Verified **2026-09-15** against installed **DataFusion 55.1.0 / Arrow 59.3.0**.
Context7 `/apache/datafusion` provided discovery leads. Exact registry source,
release-tag source and a standalone compiled execution probe establish the
following boundary; no product code was changed.

### Supported APIs and important limits

| Claim | Exact primary source / installed locator | Short exact quote | Verdict |
|---|---|---|---|
| Preserve the DataFrame's captured session while preparing its native plan. | [DataFrame 55.1.0](https://github.com/apache/datafusion/blob/55.1.0/datafusion/core/src/dataframe/mod.rs#L1677), installed lines 1677–1715 | `pub fn into_parts(self) -> (SessionState, LogicalPlan)` | verified (source and probe) |
| Native logical and physical schemas expose Arrow fields for direct checks. | [DFSchema](https://github.com/apache/datafusion/blob/55.1.0/datafusion/common/src/dfschema.rs#L142), `as_arrow`, `fields`, `field`; [ExecutionPlan](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/execution_plan.rs#L151), `schema`; stream `schema()` | `fn schema(&self) -> SchemaRef` | verified (source and probe) |
| Weak schema-equivalence helpers do not validate the requested nullability/metadata contract. | [DFSchema 55.1.0](https://github.com/apache/datafusion/blob/55.1.0/datafusion/common/src/dfschema.rs#L619), installed lines 619–650, 744–753 | `ignores differences in`; `nullability and metadata` | verified (source and counterexample probe) |
| Column references preserve field metadata; aliases merge metadata with alias values taking precedence. UDF fields come from the function's declared return field. | [ExprSchemable 55.1.0](https://github.com/apache/datafusion/blob/55.1.0/datafusion/expr/src/expr_schema.rs#L511), installed lines 511–537, 612–632 | `combined_metadata.extend(metadata.clone());` | verified (source and alias/projection/scalar probes) |
| Physical scalar evaluation receives the declared return field and row count, and checks array output length. | [ScalarFunctionExpr 55.1.0](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-expr/src/scalar_function.rs#L243), installed lines 243–280 | `return_field: Arc::clone(&self.return_field)` | verified (source and probe) |
| Physical projection has supported metadata-aware construction; it still derives names/types/nullability from actual expressions. | [ProjectionExec 55.1.0](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/projection.rs#L152), installed lines 152–178 | `try_new_with_schema_metadata` | verified (source); custom physical projection is unnecessary for the recommended integration |
| Checked Arrow batch construction verifies column count, lengths, types and non-nullable fields against actual nulls. It does not establish domain identity or required service metadata. | [RecordBatch 59.3.0](https://github.com/apache/arrow-rs/blob/59.3.0/arrow-array/src/record_batch.rs#L324), installed lines 324–394 | `!f.is_nullable() && c.null_count() > 0` | verified (source and invalid-batch probe) |

Do not replace the current preparation path with `into_optimized_plan()` or
`into_unoptimized_plan()`: their installed documentation discourages production
use because they discard the captured `SessionState`. The current
`frame.into_parts()` → `state.optimize(&logical)` → the same state's query
planner is the appropriate integration point.

### Executed field behavior

Compiled `/tmp/plan13-datafusion-fields-20260915.rs` with `rustc +1.98.1`, linking
the checkout's existing `libdatafusion-e0fbfc98c129a2e9.rlib` and its dependency
directory. The executable printed `datafusion_version=55.1.0`. It used a pure
identity UDF returning a `FieldRef` with `enrichment.function=probe/1`, inspected
initial and optimized logical schemas, then executed the physical plan and
checked its stream and actual batch schemas.

| Probe | Observed result |
|---|---|
| UDF on nullable column with alias | Function metadata and nullable=true survived initial, optimized, physical, stream and actual batch schemas; two rows |
| Nested projection and second alias | Same metadata survived; two rows |
| Immutable UDF on scalar literal | Function metadata and nullable=false survived constant folding and execution; one row |
| UDF projection with `WHERE false` | Metadata remained in optimized/physical/stream schemas; zero rows and **zero batches** |
| Direct scalar and empty-array UDF calls | Both succeeded; the invocation received its declared return-field metadata; empty output length was zero |
| Non-nullable Arrow field with an actual null | `RecordBatch::try_new` rejected the batch |
| Weak DFSchema comparison against changed nullability and missing metadata | `has_equivalent_names_and_types` returned success, demonstrating why it is insufficient |
| Left join against an always-false condition | Right input became nullable; optimizer replaced its values with nulls and removed source metadata from that output; left source metadata remained |
| Same join with explicit native alias role metadata | Declared `enrichment.role=optional-reference` survived the null rewrite; the source metadata still disappeared |

The outer-join result disproves unconditional preservation of arbitrary source
metadata through every optimizer rewrite. It does not justify disabling the
optimizer or restoring provenance onto synthesized nulls. A declared role tag
describes the output field's contract; it is not evidence that a row came from
the right-side source. Metadata identity is not row membership or referential
integrity proof.

### Smallest fail-closed integration

1. Reuse Arrow `Field`/`Schema` descriptors for the few required inputs and
   outputs of each fixed operation family. Check required names, native types,
   stage-appropriate nullability and required metadata keys/values. Reject
   missing or ambiguous required fields. Existing admitted schemas remain the
   source relation authority; result contracts describe the projection that
   consumers actually decode.
2. At the existing preparation boundary, validate required admitted inputs,
   then the optimized logical result, physical result and stream schema. Keep
   DataFusion's default analyzer/coercion/optimizer sequence and captured state.
   Validate stream schema even when execution produces no batches.
3. Check every returned batch against the prepared physical output contract
   before decoding or publishing it. Keep checked Arrow constructors and
   existing native row/key witnesses; a field metadata check does not replace
   either. Return bounded typed diagnostics naming the stage, field, expected
   contract and observed mismatch.
4. Scope required metadata to the operation's actual semantic obligations.
   Do not demand byte-for-byte equality between every pre-optimization field
   and every output: joins can widen nullability, projections rename fields,
   and optimizer-generated nulls can legitimately lose source tags. Use
   `alias_with_metadata` only for an explicitly declared derived field role
   after validating its inputs, not as an automatic repair for missing evidence.
5. Let specialized UDFs declare their actual output field with
   `return_field_from_args`. Keep `return_type` coherent and function identity
   explicit. Required output metadata does not automatically inherit from
   argument metadata. Test nullable inputs, scalar folding and zero rows for
   each changed UDF; source/metadata preservation in this identity-UDF probe
   does not certify the service's scoring or canonical-key implementations.

This requires a small preparation/output validator over native Arrow schemas,
not a custom analyzer, new type algebra or parallel Python interpretation.
Physical-plan invariants remain useful engine checks but do not know the
service's required identity tags. No new dependency or pin is required.

### W2 representation follow-up: Utf8, LargeUtf8 and Utf8View

Verified **2026-09-15** after a search fixture exposed a result-schema check
rejecting `path: Utf8` becoming `path: Utf8View`. This is a supported string
representation change; the required name, nullability and metadata checks must
remain independent of it.

The exact installed APIs establish a narrow rule:

- [Arrow 59.3.0 `DataType::is_string`](https://github.com/apache/arrow-rs/blob/59.3.0/arrow-schema/src/datatype.rs#L636)
  is exactly `matches!(self, Utf8 | LargeUtf8 | Utf8View)`. These are the three
  native string representations intended for this result check.
- [DataFusion 55.1.0 `string_coercion`](https://github.com/apache/datafusion/blob/55.1.0/datafusion/expr-common/src/type_coercion/binary.rs#L1744)
  chooses `Utf8View` when either input uses views; otherwise it chooses
  `LargeUtf8` when either input is large. `type_union_coercion` calls that rule
  and recursively coerces list and struct children. SQL
  [`map_string_types_to_utf8view`](https://github.com/apache/datafusion/blob/55.1.0/datafusion/sql/src/planner.rs#L56)
  defaults to true, so `CAST(NULL AS VARCHAR)` supplies a view-typed null.
- [`OptimizeProjections`](https://github.com/apache/datafusion/blob/55.1.0/datafusion/optimizer/src/optimize_projections/mod.rs#L839)
  rebuilds projections from rewritten inputs and recomputes parent schemas.
  It contains no explicit Utf8-to-view conversion branch. The changing rule in
  the minimal reproduction below was the **type-coercion analyzer**.
  `SessionState::optimize` runs both analyzer and optimizer phases; a schema
  read before that call must not automatically be labeled fully analyzed.
- DataFusion's `assert_expected_schema` uses logical equivalence, including
  Utf8/view compatibility, but deliberately ignores nullability and metadata.
  Its logical type equality also permits dictionary and run-end wrappers and
  weakens other type details. It remains too broad for the service's complete
  required-field check.

Executed the existing standalone 55.1.0 probe with this search-shaped query:

```sql
SELECT value AS path FROM input
UNION ALL
SELECT CAST(NULL AS VARCHAR) AS path
```

The input column was Utf8. Native analyzer/optimizer observers identified
`stage=analyzer rule=type_coercion` as the change from Utf8 to Utf8View;
nullable=true and source metadata remained unchanged. Physical, stream and
actual batch schemas used Utf8View; three rows were returned. Two more probes
wrapped each branch in `named_struct('path', ...)` and `make_array(...)`:

| Shape before analysis | Shape after analysis/optimization and execution |
|---|---|
| `Utf8` | `Utf8View` |
| `Struct<path: Utf8>` | `Struct<path: Utf8View>` |
| `List<Utf8>` | `List<Utf8View>` |

All three completed with unchanged names/nullability and batches conforming to
their physical schema. This confirms nested representation changes, not just
top-level text changes. It does not independently identify the exact changing
rule in the original larger service fixture.

**Recommended check:** accept exact data-type equality, or a pair for which
both `DataType::is_string()` values are true. For the nested forms the operation
actually uses, apply that same leaf rule recursively while retaining the same
container kind, child count/order/names, list sizes, nullability and required
metadata. Keep all other leaf types and container parameters exact. Generic
native coercion can change list widths, struct layouts and numeric types too;
those are not automatically approved by a string representation exception.

Dictionary-encoded strings are supported by the existing `TextColumn` decoder
through an explicit cast, but are not a fourth value of `is_string()`. Admit
that wrapper only through a separately declared decoder/contract path. Binary,
numeric, temporal, arbitrary castable types and changed identity metadata must
still fail this check. Once a physical plan's schema is selected, require the
stream and its actual batches to agree with that concrete schema. Preserve
string-view optimizations instead of globally disabling them or inserting
unnecessary casts solely to satisfy byte-for-byte type equality.
