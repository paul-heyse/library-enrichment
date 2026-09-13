# ADR 0006: NDJSON-RPC transport details

- **Status:** accepted
- **Date:** 2026-09-13
- **Binding boundary touched:** Transport (§1.1 — "Per-client stdio adapters over one local Rust daemon")

## Context

Blueprint §2.1 fixes the protocol in one sentence:

> Use a Unix-domain socket on Linux and macOS for daemon RPC. For v1, use bounded
> newline-delimited JSON-RPC 2.0, with embedded newlines JSON-escaped and an explicit message
> size limit. Large payloads travel by content-addressed artifact handles, not inline RPC
> messages. Do not confuse this internal protocol with MCP or LSP framing.

That settles the framing and nothing else. Four things a working implementation needs are
unspecified, and §2.3 says only "Keep the socket path short and private to the user":

1. where the socket lives;
2. the method vocabulary;
3. the numeric value of the "explicit message size limit";
4. how a JSON-RPC `error` object relates to the frozen thirteen service error codes.

This is a gap to fill, not a deviation to justify. Recording it here means the next phase
extends a decision rather than re-deriving one.

## Decision

**Socket path**, resolved in this order, first match wins:

| Source | Path |
|---|---|
| `LIBENR_SOCKET` | used verbatim |
| `LIBENR_HOME` | `$LIBENR_HOME/run/d.sock` |
| `XDG_RUNTIME_DIR` | `$XDG_RUNTIME_DIR/library-enrichment/d.sock` |
| `XDG_CACHE_HOME` | `$XDG_CACHE_HOME/library-enrichment/run/d.sock` |
| `HOME` | `~/.cache/library-enrichment/run/d.sock` |
| nothing set | **hard error** — the daemon refuses to start |

**There is no working-directory fallback, and no relative path is accepted from any source.**
An earlier revision of this ADR ended the table at `~/.cache`, while the code fell back to a
relative `.library-enrichment/run` when `HOME` was unset. That silently put a live socket inside
whatever directory the daemon happened to start in — for an operator or an agent, very often a
repository under study. That is exactly the breach gate C20 exists to catch, so resolution now
fails loudly instead: `PathError::NoRuntimeDirectory` when every source is unset, and
`PathError::NotAbsolute` when one is set to a relative path. A daemon that will not start is a
far better outcome than one that quietly writes into someone's checkout.

The containing directory is created mode `0700`. The filename is `d.sock`, not something
descriptive, because `sockaddr_un.sun_path` caps at 108 bytes on Linux and 104 on macOS and the
prefix is not under our control. `XDG_RUNTIME_DIR` ranks above `XDG_CACHE_HOME` because a socket
is runtime state, not a cache. `LIBENR_HOME` outranks both so that a development session lands
in the gitignored `.dev-state/` that `scripts/env.sh` sets.

**Message size limit: 1048576 bytes.** Not invented — the frozen
`config/service.example.toml` already specifies `limits.rpc_message_bytes = 1048576`. Both sides
enforce it. The reader refuses to *buffer* beyond the limit rather than reading a line and then
measuring it: a peer that opens the socket and streams bytes without a newline would otherwise
be an out-of-memory vector against the process that owns every other agent's jobs. After a
rejection the offending frame is consumed, so the connection carries on.

**Method vocabulary:** dotted, `noun.verb`, namespaced by subsystem — `service.status`,
`daemon.ping`, `daemon.shutdown`, `wire.validate`, `producer.probe_format`. Deliberately
*not* the MCP tool names: the
tools are the public contract and the RPC methods are an internal one, and letting them drift
apart is what allows one MCP tool to become several core calls in Phase 1.

`service.status` returns a **complete wire envelope**, not a bare payload. Identity, coverage
and freshness are evidence-model assertions, and §1.1 gives the evidence model to Rust; an
adapter that composed its own `coverage` would be asserting what the service looked at without
having looked. The adapter forwards what the core produced.

`producer.probe_format` is gate R04, and the fourth clause of the blueprint's Phase-0 gate: an
unsupported producer format returns a typed error. It reads `format_version` and decides
*before* touching the body, because an unsupported rustdoc document frequently deserializes
successfully into the wrong shape — that silent misparse is the failure, and it is worse than a
refusal because the resulting evidence looks fine.

`wire.validate` exists to make gate C19 measurable. The gate says inputs violating the wire
schema are "rejected **consistently** through CLI/RPC/MCP boundaries", and consistency cannot be
established by three validators that each reject something — they could disagree about which
documents are valid and nothing would notice. So `wire.validate` and
`library-enrichmentd validate` both call one `enrichment_daemon::validate::validate`, and the
shared corpus in `tests/wire_corpus.py` is pushed through both plus the schemas and the adapter,
comparing verdicts document by document.

**Error mapping.** Every RPC error carries both codes:

```json
{"code": -32601, "message": "unknown method `x`",
 "data": {"code": "UNSUPPORTED_CAPABILITY", "next_action": "..."}}
```

`code` is the standard JSON-RPC number, so a generic client behaves sensibly. `data.code` is
the stable service code from the frozen thirteen, which the adapter maps into the wire
envelope's `error.code`. The transport therefore stays ordinary JSON-RPC while the frozen
evidence contract remains the authority on what went wrong. Current mappings:

| JSON-RPC | Service code | When |
|---|---|---|
| `-32700` parse error | `UNSUPPORTED_FORMAT` | the line is not JSON |
| `-32600` invalid request | `UNSUPPORTED_FORMAT` | not JSON-RPC 2.0 |
| `-32601` method not found | `UNSUPPORTED_CAPABILITY` | unknown method |
| `-32000` (application) | `BUDGET_EXCEEDED` | frame over `rpc_message_bytes` |

**Lifecycle.** `start` runs in the foreground until SIGINT/SIGTERM; a supervisor or `just`
backgrounds it. `stop` is an in-band `daemon.shutdown` call, acknowledged before the daemon
exits, so stopping needs no pidfile and no signal-sending privileges. A socket file with nothing
listening is reclaimed as a crash leftover; one with a live listener makes `start` fail with
`AddrInUse`, because two daemons on one socket would break the single-writer invariant.
Connecting is the test, since a pidfile can be stale in the other direction.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Framing is bounded NDJSON JSON-RPC 2.0, not MCP/LSP | `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` §2.1 | 2026-09-13 | "For v1, use bounded newline-delimited JSON-RPC 2.0, with embedded newlines JSON-escaped and an explicit message size limit… Do not confuse this internal protocol with MCP or LSP framing." |
| The socket path is otherwise unspecified | `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` §2.3 | 2026-09-13 | "Keep the socket path short and private to the user." |
| The size limit is already fixed by the frozen config | `config/service.example.toml` line 27 | 2026-09-13 | `rpc_message_bytes = 1048576` |
| Lifecycle verbs are start/status/stop | `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` §2.2 | 2026-09-13 | "Provide explicit `start`, `status`, and `stop` CLI commands… The daemon survives adapter exits and retains jobs." |
| `sun_path` is 108 bytes on Linux | `unix(7)` man page, `sockaddr_un` | 2026-09-13 | `char sun_path[108];` — measured against on this workstation by `paths::tests::the_socket_path_stays_well_under_the_sun_path_limit` |

## Tests that prove it

`crates/enrichment-daemon/tests/rpc_boundary.rs` drives a real daemon over a real socket:

- `an_oversized_frame_is_rejected_without_being_buffered` — the bound, and that the error names it
- `the_connection_survives_a_rejected_frame` — a rejection does not poison the stream
- `malformed_json_is_a_typed_error_not_a_dropped_connection`, `an_unknown_method_is_a_typed_error`,
  `a_wrong_protocol_version_is_rejected` — the error mapping
- `a_second_daemon_refuses_to_steal_a_live_socket`, `a_stale_socket_from_a_dead_daemon_is_reclaimed`
- `shutdown_is_acknowledged_and_removes_the_socket`

Path resolution, in `paths::tests`:

- `an_empty_environment_fails_rather_than_using_the_working_directory` — the C20 regression above
- `a_relative_socket_override_is_refused`, `a_relative_runtime_base_is_refused`
- `every_resolved_path_is_absolute` — across all five sources
- `the_socket_path_stays_well_under_the_sun_path_limit`

Plus `rpc::tests::embedded_newlines_are_escaped_so_the_delimiter_stays_unambiguous` and the
end-to-end path in `tests/e2e/test_mcp_daemon_handshake.py`. The rejection tests are registered
under gate **C19**.

`scripts/state-leak-check.sh` and `tests/conftest.py` both digest
`$XDG_RUNTIME_DIR/library-enrichment` as well as the cache and data roots, because this ADR's
resolution order reaches the runtime directory first — without that, the one directory Phase 0
actually writes to would be the one neither oracle watched.

## Consequences

Easier: Phase 1 adds a method by adding a `dispatch` arm; the framing, the bound and the error
mapping are settled. The Python adapter resolves the same socket from the same variables, so a
development session and the daemon it started cannot disagree about where it is.

Harder: the socket resolution order now lives in two places —
`crates/enrichment-daemon/src/paths.rs` and `python/enrichment_mcp/daemon_client.py`. They must
stay in step, and `tests/e2e/test_mcp_daemon_handshake.py` is what catches a divergence, since
the adapter would silently fall back to its daemon-absent branch.

Foreclosed: nothing. A future Windows named-pipe transport implements the same RPC contract, as
§2.1 anticipates, and reversing any of this is a change to one module plus its tests.

## Boundaries preserved

Rust still owns identities, resolution, evidence, job state, policy and publication. Python
remains a thin adapter that validates input, calls the daemon, and maps structured errors — it
gained no state here. No FastAPI, gRPC, Redis, or second daemon was added to carry this
interface, exactly as §2.1 warns against. The frozen thirteen error codes remain the authority
on what went wrong; the JSON-RPC numbers sit alongside them and never replace them.
