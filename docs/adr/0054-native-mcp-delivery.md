---
id: ADR-0054
title: Measure and project complete MCP deliveries in the native runtime
status: proposed
date: 2026-09-17
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-06, DM-16, DM-42, DM-52]
design: [§7.2, §7.3, §7.4]
review: docs/design_review/reviews/design_review_schema-engineering-typed-values_2026-09-16.md
evidence: Implemented
supersedes: []
superseded-by: null
revisit: The FastMCP or MCP serializer pin changes, or an additional transport is proposed.
verification: mcp_delivery units; result_measure units; mcp_delivery_probe plus installed SDK parity; tests/unit/test_error_preview.py; Plan 18 Q06/Q13 and SC07/SC09.
---

# ADR-0054: Measure and project complete MCP deliveries in the native runtime

## Context

ADR-0047 requires native result selection and actual MCP framing. The old adapter still chose
summary/error-preview reductions after the envelope budget had been enforced. A fixed 1024-byte
allowance cannot bound arbitrary JSON-RPC IDs. Resource text also escapes the envelope a second
time. This implements ADR-0047's target and replaces obsolete framing language in §7.3; it does
not supersede its wider schema authority. The record stays proposed pending final conformance.

## Scope

Nine tool routes and the four evidence resource templates over stdio. The packaged workflow
resource is local guidance. Adapter-local availability/protocol failures and SDK admission errors
are explicit transport boundaries, not native-budget-certified evidence deliveries. No HTTP/SSE,
WebSocket or in-process transport is admitted as stdio by this contract.

## Drivers

Make one native format projection authoritative for emitted bytes, measurement, failure recovery
and retained representation. Keep exact request-ID framing distinct from application correlation.

## Options

- Fixed framing allowance: disproved by long and differently typed RPC IDs.
- Python trim-and-retry: duplicates core policy and can silently lose mandatory recovery.
- Native projection plus measured SDK transport facts: selected; final adapter checks refuse drift.

## Decision

Wire **5.0** gives `max_bytes` the meaning of the complete response at its admitted endpoint.
Internal envelope reads measure the envelope; MCP stdio tool/resource reads include the actual
JSON-RPC wrapper and newline. The only accepted external profiles bind protocol era, measured
framing bytes (35 through 16384), and the exact resource URI when applicable. Unknown protocols,
non-stdio contexts and out-of-bound facts refuse. There is no previous wire reader.

A full-field immutable DataFusion UDF measures a typed ResultRecord through the same Rust format
projection used at the final boundary. It counts oversized candidates instead of aborting before
native selection can choose another view. DataFusion filters inline eligibility and ranks retained
views. An impossible cap yields the declared minimum-budget refusal; no semantic trimming occurs.
Remaining handler page/section composition still belongs to Plan 18 CP06.

Native tool content is explicit: complete typed recovery previews for direct/terminal job errors,
and an artifact link for retained results. The canonical envelope appears once in structuredContent.
Retained job-control views keep the terminal outcome and recovery. Python's preview/summary fitting
loop, implicit content duplication and fixed allowance are deleted. The native resource projection
emits the exact JSON text once; its outer escaping and URI contribute to the measurement.

FastMCP's modern mandatory fields are explicit: complete result type and the generated service
identity stamp; resources additionally use private cache scope and zero TTL. The adapter forwards
through ToolResult.from_mcp_result or ResourceResult/ResourceContent. Strict model validation,
public version shaping and the actual SDK JSONRPCResponse serializer must preserve the native
object and exact declared byte count. It checks the serverInfo stamp before the runner can add it.
The request ID is read through the pinned SDK request-context escape hatch because the convenient
Context.request_id property stringifies numbers. No guessed reconstruction is permitted.

## Evidence

| Claim | Primary source | Retrieved | Exact support |
|---|---|---|---|
| FastMCP preserves an explicit native tool model | [ToolResult](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/src/fastmcp/tools/base.py) | 2026-09-17 | `from_mcp_result` retains the original model; `to_mcp_result` returns it. |
| Public context distinguishes transport | [Context](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/src/fastmcp/server/context.py) | 2026-09-17 | `transport` exposes the actual transport; request absence alone is insufficient. |
| ResourceContent controls text and MIME mechanically | [ResourceResult](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/src/fastmcp/resources/base.py) | 2026-09-17 | `to_mcp_result` builds resource contents from the supplied URI. |
| SDK adds modern identity/caching fields and uses Pydantic stdio encoding | Installed mcp 2.2.0 runner, mcp-types 2.2.0 declarations and methods | 2026-09-17 | Pinned paths, hashes and assertions are in the source receipts below. |

Read-only upstream-verifier receipts and scripts are in
`.dev-state/plan18/execution/fastmcp-framing/receipt.json`, `review-receipt.json`, `REVIEW.md`.
The 144-case native/installed-SDK parity probe includes both protocol eras, four RPC-ID shapes,
all four outcomes, retained links and nested terminal failures. Resource probing additionally
found required modern `cacheScope=private` and `ttlMs=0`; both are now explicit native fields.
These are library/codec observations, not daemon, real-client or installed acceptance results.

## Provenance

The active product tool guide adopts wire 5.0 decimal strings and full MCP budgets. Its older
research-v2 seal already failed at the execution baseline: committed HEAD held SHA-256
6a6e6d34592bb9ae734236fdd09177dcd847bf90b631fb054255c5c236967c4b instead of sealed
39af9cff4896866a5df5626680e8c18bbe4c586d0c6d9d32a9b9ab878ef9288b. The exact sealed bytes
were recovered from a78c741 and verified before retention. The new dated
`docs/provenance/bundle-2026-09-17-native-delivery/` seals the updated guide and an overlay map;
original manifests/maps remain byte-identical. The provenance gate verifies all three bundles.
This preserves requested frozen provenance without retaining an old runtime/wire implementation.

## Verification

`cargo run -p enrichment-core --example mcp_delivery_probe` emits independent native vectors.
`.dev-state/plan18/execution/fastmcp-framing/native_parity.py` checks the installed SDK against
those vectors. Focused Rust units exercise oversized candidates, recovery preservation, exact
native size and inline boundaries. Python units reject changed byte facts and missing stamps.
Schema generation rejects wire 4.0. Full Q/SC application qualification remains not_run until the
architecture/source/installed-state deletion barrier. Resource allocation/lifecycle completion
remains CP09; exact byte measurement is not proof of complete external-allocation accounting.

## Boundaries preserved

§B1–§B13: Rust owns evidence/policy/selection; DataFusion executes fit selection; Delta retains
complete results. Python owns transport facts and validation only. No automatic repository edits,
arbitrary SQL/shell surface, wire migration or protocol-version upgrade is added.

## Status history

- 2026-09-17 — proposed; source implementation and isolated serializer evidence recorded.
