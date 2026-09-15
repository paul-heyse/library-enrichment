# Operations

Install, configure, register, and run the service.

> The service uses one Rust Arrow/DataFusion evidence path. Exact-version evidence persists
> until explicit cleanup or a changed qualified identity requires new evidence. See
> [STATUS.md](../../STATUS.md) for dated acceptance results and outstanding work.

Two claims this document is careful about, because both are easy to overstate:

- **Configured is not qualified.** An execution image named in configuration is a setting. Only a
  recorded containment run makes a profile operational, and `service_status` reports the two
  separately. [Execution images](#execution-images-setup-and-qualification) is the whole story.
- **Installed is not registered.** Installing the companion skill and registering the MCP server
  are separate operator actions, and neither one proves a client can call the service.

## Development setup

```sh
git clone https://github.com/paul-heyse/library-enrichment
cd library-enrichment
direnv allow          # optional but recommended; see below
just setup            # toolchain check, uv sync, dev state, doctor
just --list           # the command surface
```

`just setup` is idempotent. `just doctor` reports which tools are genuinely present and **fails**
on a missing hard requirement rather than reporting a tool as available when it is not.

### direnv

`.envrc` sources `scripts/env.sh`, which is also what the agent session hook uses — one
definition, so a human and an agent cannot end up running against different state. With it
active:

- `ty`, `ruff`, `pytest`, `python` resolve to the project environment with no `uv run` prefix
- built binaries resolve from `target/debug` with no path prefix
- service state resolves to the gitignored `.dev-state/`, not your real XDG directories

Without direnv, everything still works through `just`, and `uv run <tool>` for the rest.

## Where state lives

| | Development | Production |
|---|---|---|
| Cache | `.dev-state/cache/` | `~/.cache/library-enrichment/` |
| Retained evidence | `.dev-state/data/` | `~/.local/share/library-enrichment/` |
| Capsules | `.dev-state/cache/capsules/` | `~/.cache/library-enrichment/capsules/` |

Overridden by `LIBENR_CACHE_HOME` and `LIBENR_DATA_HOME`. The resolver is OS-aware; these Linux
paths are not assumed on every platform (blueprint §2.3).

`just state-reset` previews removal of known development service payloads; `just state-reset --apply` performs the scoped cutover under ownership locks. Logs, credentials, execution images and unrelated fixture directories are preserved. `just state-leak-check` digests the real
XDG paths before and after a run and fails on any change — so a code path that resolves the
production policy instead of the configured root surfaces immediately.

**No working repository may be written to, used as a working directory, or used as an
extraction destination.** Acceptance gate C20 proves this with a filesystem digest — path, mode
and content hash for every file — taken over a canary repository before and after one full
operation of every kind, under every enabled profile. `tests/e2e/test_canary_repository.py` is
the gate; it also starts the daemon *inside* the canary, because a relative path anywhere in the
service would then land there rather than somewhere harmless.

## Execution profiles

| Profile | Permits | Default |
|---|---|---|
| `static` | registry and documentation fetching, safe archive extraction, static parsing, search | enabled |
| `build` | rustdoc fallback, Cargo checks, proc-macro and build-script activity, Python environment setup | enabled only with an operational sandbox |
| `runtime` | importing and executing library code | explicitly enabled only |

A caller selects from the profiles local configuration has enabled; it cannot grant itself
permission. An unavailable profile returns `POLICY_DENIED` with a next action — never a silent
fallback to running on the host.

Execution is rootless Podman against a **service-owned storage root**, never the operator's
Podman. Containers run read-only, with `--cap-drop=all`, `--network=none`, `--userns=keep-id`,
and cgroup memory/PID bounds. `execution.storage_root` (or `LIBENR_EXECUTION_ROOT`) names the
root; everything below assumes it is set, or falls back to `<cache>/podman`.

The host-side broker also needs access to the user's existing systemd session bus.
Agent or remote shells may omit `DBUS_SESSION_BUS_ADDRESS` even when the bus socket exists.
For this workstation, the verified persistent shell and user-manager configuration is
recorded in [the 2026-09-14 environment repair](../reports/user-session-environment-2026-09-14.md).
The broker retains its private XDG storage paths and passes no session-bus setting into
the target container. A working bus is a prerequisite; containment still requires the
real qualification tests below.

## Execution images: setup and qualification

Two operator steps, in this order. Both preview by default and do nothing until `--apply`.

```sh
just execution-images           # preview: what would be fetched and built
just execution-images --apply   # build both producer images from pinned inputs

just execution-qualify          # preview: which images, which probes, which tests
just execution-qualify --apply  # probe, run the real containment tier, write the receipt
```

`execution-images` builds from `execution-images/{python,rust}/Containerfile` with every input
pinned by digest in `execution-images/inputs.toml` — both bases by manifest digest, both wheels
and both rustup channel manifests by sha256, and a fixed `--timestamp`. A tag would be a mutable
pointer, and an image whose contents can change is not a recorded producer identity.

`execution-qualify` is the step that earns the operational claim, and it earns it the only way it
can be earned: it probes each image's tool identities *inside the same containment the service
uses*, runs the real-container test tier, and writes `admitted-images.json` beside the images —
**only if every step succeeded**. A failure writes nothing, so a stale receipt can never outlive
the images it describes.

Then `service_status` reports, per image:

```sh
library-enrichmentd status | jq '.result.data.sandbox'
```

| Field | Means |
|---|---|
| `enabled_profiles` | what configuration permits |
| `available_runtimes` | what is installed on this host — detection only |
| `execution_qualified` | whether a receipt covers the **currently configured** image IDs |
| `admitted_images` | the image IDs that receipt covers |
| `execution_readiness` | when qualification happened, or the missing prerequisite |

Three different facts sit next to each other on purpose. A profile can be enabled, a runtime can
be present, and images can be configured, and none of the three means the service can contain
anything. Without a receipt, the sandbox-tier tests skip and their gates report **`blocked`** with
this recipe named — which is the truthful state, and never a false pass.

## Recovering from unresolved cleanup

The service owns every container it creates and reports cleanup as confirmed only after
container absence is established. An unresolved cleanup can produce a partial terminal result
with the ownership record retained and further admission quarantined. A dropped execution future, a panic, or a failed `podman rm`
hands the container to the cleanup supervisor, which **retains the worker permit** and retries
removal until absence is confirmed or `execution.cleanup_deadline_seconds` elapses.

While cleanup is unresolved, admission is quarantined: `verify_usage` returns `POLICY_DENIED`
naming `<execution root>/owned` rather than starting more work beside a container nobody has
accounted for. Two symptoms and their answers:

```sh
# What the service thinks is outstanding
library-enrichmentd status | jq '.result.data.health'

# What is actually there, asked through the broker rather than through the service
podman --root "$LIBENR_EXECUTION_ROOT/s" --runroot "$LIBENR_EXECUTION_ROOT/r" ps -a
cat "$LIBENR_EXECUTION_ROOT/owned/"*        # the ownership records the service wrote
```

A restart reconciles: `Runner::recover_owned` runs **before** the journal turns interrupted work
terminal, precisely so a job is never marked finished while its container may still be live. If a
container survives a restart and reconciliation — a stuck runtime, a full disk — remove it through
the broker above and then delete its ownership record. Removing the record without removing the
container is the one order that loses the invariant.

A probe whose container could not be confirmed gone returns `partial`, not `ok`, with
`cleanup_confirmed: false` and the missing assurance named. The process evidence is kept: an
unconfirmed removal is a boundary fact, not a reason to discard what the probe observed.

## Pruning regenerable state

```sh
just capsules-prune                    # preview regenerable cache removal
just capsules-prune --apply            # remove the listed cache payloads
just evidence-cleanup                 # preview explicit retained-evidence removal
just evidence-cleanup --apply         # remove the listed evidence and job journals
just state-reset                      # preview development cutover of cache and evidence
just state-reset --apply              # execute the scoped development cutover
```

These commands use the Rust maintenance implementation. It verifies physical ownership markers
and holds data-writer, cache-writer, retention and capsule-storage locks from inventory through
deletion. A running daemon, query, physical plan, stream or export prevents cleanup. Stop the
daemon first. Startup and apply reconcile every execution root recorded for this cache, including
roots no longer selected by configuration, before releasing reservations or deleting payloads.
Unknown or linked ownership records fail closed; images and engine storage are excluded.

Cache removal includes disposable capsules, source/download caches, worker scratch and spill.
It leaves retained evidence intact. Exact validated evidence has no age-based expiry; removing
it requires the separate explicit evidence action. Apply reports actual removed file bytes and
any failure, including partial removal; it does not estimate reclaimed filesystem blocks.

The development reset recognizes only this checkout's physical `.dev-state/{cache,data}` roots
and known service payload names. It preserves logs, source provenance, credentials, execution
images and unrelated directories. It initializes the target format after successful removal;
it does not migrate old snapshots. Review its printed candidate and preserved-path inventory.

## Exporting provenance

```sh
just export <context-id> /path/to/bundle     # write a bundle
just verify-bundle /path/to/bundle           # check one
```

A bundle is a directory carrying `MANIFEST.sha256`, `bundle.json` (the service's own release,
environment and context records), the context's snapshot copied file for file, and the release's
artifacts under their content-addressed names. ADR-0018 records the decision.

Neither command contacts the daemon — a bundle is a copy of evidence that is already immutable on
disk, and an operator wants one most when the service is wedged or stopped. A recipient needs no
copy of this service to check it:

```sh
cd /path/to/bundle && sha256sum --check MANIFEST.sha256
```

`verify-bundle` additionally reports files that are *present but unlisted*, which `sha256sum`
cannot: appending to a bundle and leaving the manifest alone would otherwise still verify.

What verifying proves is that the copy is intact — **not** that the original was correct. The
bundle says so in its own `note` field, where a reader months later will actually meet it.

## Operational metrics

```sh
library-enrichmentd status | jq '.result.data.health'
```

Blueprint §14.3 diagnostics, scoped to one daemon process and reset by a restart —
`uptime_seconds` is published beside them so a count is readable:

| Group | Counters |
|---|---|
| `fetch` | `hits`, `revalidated`, `misses`, `failures`, `fetched_bytes` |
| `evidence` | `requests`, `ok`, `partial`, `pending`, `errors`, `gaps`, `response_bytes` |
| `single_flight` | `started`, `shared`, `inflight`, `total_millis` (producer duration) |
| `lsp` | `started`, `reused`, `evicted`, `warm` |
| `verification` | `succeeded`, `failed`, `unresolved` |
| `native_queries` | recorded executions/completed/incomplete, summed planning/elapsed microseconds, admitted queries and configured concurrency/memory/spill/cache limits |
| queue | `queued_jobs`, `running_jobs`, `cache_ready` |

Three distinctions are load-bearing. A cache `hit` opened no socket; a `revalidated` request
opened one and got `304`, so freshness was re-established without a transfer; only a `miss` moved
bytes, and only a miss adds to `fetched_bytes`. A `gap` is an answer that named what it was
missing — a correct answer to a bounded question, counted apart from `errors`. And a verification
`unresolved` is a probe the *service* could not run, never a failure attributed to the caller's
code.

Native query counts describe physical plans whose execution traces have settled; failures before
plan construction are excluded. `admitted` is the current permit count. Timings can overlap;
configured byte limits are not measured memory consumption or peak RSS. The field is null when
no native runtime is open. No query plan text or library source is exposed by these counters.

These are engineering diagnostics. They are deliberately not rolled up into a score: §13 forbids
converting gate results into a quality percentage, and the same reasoning applies here.

The daemon also writes one structured line per request to **stderr** and the daemon log — never
MCP stdout:

```json
{"at":"2026-09-14T03:41:53Z","event":"rpc","method":"service.status","status":"ok","duration_ms":0,"response_bytes":3753}
```

Method, outcome, duration and size only. What a caller asked about is theirs: a log that records
which libraries someone researched is a different artifact than one that records that the service
answered.

## Registering the service

Generate a concrete launch description before registration:

```sh
just launch-config --state /absolute/service/state --config /absolute/service/service.toml
```

This prints JSON and writes nothing. It includes the exact daemon/adapter commands, state and
socket environment, binary/lockfile digests, and TOML with the installation's absolute Python
worker interpreter. Write and review that TOML at the named configuration path before starting
the daemon. Build all three native executables in the selected profile: daemon, executor and
`library-enrichment-native-worker`. This mandatory sibling isolates raw Rust producer parsing
and Parquet admission; bundle verification needs it too. Missing workers fail explicitly.

The generator defaults to the release build and personal workstation resources. Use
`--profile debug` for development or `--resources portable` for the original small-pool
reference configuration. The workstation configuration provides 32 GiB shared managed query
memory, a 64 GiB spill ceiling, 2 GiB metadata cache, 16 query partitions and 16 concurrent
queries. It does not eagerly allocate those maxima. Query output and malformed-input bounds
remain separate. `config/service.workstation.toml` is the editable resource template; execution
permissions and qualified images are configured separately. The priority is prompt response
latency for coding agents, with larger-corpus and concurrency measurements guiding tuning.

The generated adapter uses `uv run --frozen --no-sync` with an absolute project and interpreter,
and Python isolated mode. Run `uv sync --locked` during explicit setup; startup does not install
dependencies. An executed static-Python resolve/inspect regression confirms that adapter,
daemon and worker work from an unrelated cwd without relying on `PYTHONPATH`.

The following operator registration examples use absolute paths; derive the repository root:

```sh
REPO="$(git -C /path/to/library-enrichment rev-parse --show-toplevel)"

claude mcp add --scope user --transport stdio library-enrichment -- \
  "$REPO/.venv/bin/library-enrichment-mcp"

codex mcp add library-enrichment -- \
  "$REPO/.venv/bin/library-enrichment-mcp"
```

Keep Context7 registered separately; this service does not proxy it.

When inspection reports multiple definitions at one public path, choose the returned candidate
by resubmitting its `path` as `symbol_path` and its `definition_id`. Selection preserves the
independent source/stub observations of that definition and stays within the supplied snapshot.
Overview chooses a documented representative when one observation lacks a summary; inspect the
definition for all qualified observations. Producer status describes the current daemon process: a
static worker becomes qualified after a validated extraction, so refresh status after running it.

Removing them again:

```sh
claude mcp remove --scope user library-enrichment
codex mcp remove library-enrichment
```

**Registration alone does not prove the service connects.** Make actual tool calls in both
clients and capture the traces — acceptance gates A01 and A02 require real traces, and an
unavailable client is `blocked`, never a mocked pass.

`scripts/hooks/pre_bash.sh` blocks these commands during agent sessions unless
`LIBENR_ALLOW_USER_INSTALL=1`, because they mutate user-scope configuration.

## Installing the companion skill

```sh
just install-skill              # dry run: prints destinations, writes nothing
just install-skill --apply      # install
just install-skill --uninstall  # preview removal
just install-skill --uninstall --apply  # remove the checked installation
```

Defaults to a dry run by design: `AGENT_HANDOFF.md` forbids installing the skill into user
configuration without explicit setup invocation. One installer owns install, update, recovery
and uninstall; the shell entry point only delegates through the locked project environment.
It checks the exact source identity and every file digest, refusing modified, unrelated or old
in-place installations. An old directory must be deliberately removed by its owner before the
new installer can claim that name; there is no implicit import or destructive conversion.

Each physical destination has a permanent coordination lock and a private generation directory.
The public `library-research` path switches atomically to a fully written, synced generation.
A bounded transaction permits recovery after interrupted writes or pointer replacement, and
retirement manifests preserve ownership through interrupted old-generation deletion. Repeating
the same operator command finishes recovery. Preview reports pending recovery without changing
files. Destinations are independently atomic; a run interrupted between distinct destinations
is completed by rerunning. Unknown staging or changed bytes cause explicit refusal.

On this workstation `~/.claude/skills` is a symlink into `~/.codex/skills`; the installer
resolves and deduplicates destinations so the skill is not written twice.

Installing the skill does **not** register the service. They are separate steps.

Initialization and unpublished intent bytes now stay in the locked installer-owned directory.
A killed initializer can resume only an exact owner prefix with no public pointer; an unpublished
intent is bounded scratch and cannot authorize changes until atomic promotion to the committed
journal. Unknown children, links and foreign ownership fail before removal. Install, update,
preview recovery and explicit uninstall share this implementation.

## Client acceptance

Gates A01–A06 are about a *real* client. `.claude/rules/evidence-truthfulness.md` is explicit
that a mocked client is never a pass for A01 or A02, so the harness installs the product skill
and registers the service for real — and does it inside a **throwaway `HOME`** that is not the
operator's:

```sh
just client-acceptance           # preview: what it would install, register and run
just client-acceptance --apply   # run it
just test-client                 # the same scenarios as pytest, for the gate report
```

`--apply` creates a separate fresh user root for each selected scenario under
`$LIBENR_HOME/clients/<timestamp>/`. It redirects HOME, CODEX_HOME, CODEX_SQLITE_HOME,
CLAUDE_CONFIG_DIR and XDG paths, installs the skill, starts a real daemon over fresh fixture
upstreams, and registers its absolute locked adapter command. Context7 is a separate client
connection. Codex's anonymous HTTP entry uses supported direct TOML configuration because
`mcp add --url` may initiate an interactive OAuth flow; Claude uses its HTTP registration CLI.
The A06 scenario installs the skill with the enrichment server actually unregistered.

The harness drives `codex exec --json` and `claude -p --output-format stream-json` with supported
read/research permission settings. It correlates native call IDs and results, follows completed
jobs and artifact pages, validates generated contracts, and independently checks snapshot and
artifact identities against the daemon. It also runs two-release upgrade and qualified runtime
research in each client. `--gate A01` selects one scenario; `--gate` is repeatable.

Transcripts and independent witnesses are recorded under the output root. `just test-client`
retains them in `.dev-state/logs/clients/run-*`; its JSON test results bind every retained file
by hash, and acceptance validation rejects missing or changed traces. Client credential
copies are removed with their owned sandbox on exit; `--keep` retains it for inspection.
Before/after digests cover the operator's actual configuration, credential and skill files,
including modes. Concurrent operator session histories are outside that configuration check.
A missing summary or harness crash is a failure. Prerequisites are assessed per client/scenario.

### Credentials

The sandboxed `HOME` has no credentials of its own, so by default the clients cannot authenticate
and each affected client reports **`blocked`** with its prerequisite named. Two ways to supply them:

```sh
ANTHROPIC_API_KEY=... just client-acceptance --apply
# or, explicitly opting in to reusing your own logged-in session:
just client-acceptance --apply --use-operator-credentials
```

`--use-operator-credentials` copies only the selected client's credential file into its throwaway
directory, with mode 0600. Other client credentials are not passed to that child. It is **opt-in and never the default**: reusing a personal login consumes the
operator's account and is their decision, not the harness's. Nothing about this flag writes to
the operator's own configuration — the before/after digests still hold.

A missing client binary, absent credentials, or an unreachable Context7 is reported as a blocked
prerequisite naming what is missing. It is never worked around and never converted into a pass.

## Running the daemon

```sh
cargo build --locked --workspace --bins  # includes daemon, executor and native worker
library-enrichmentd start          # foreground; Ctrl-C or SIGTERM stops it
library-enrichmentd status         # prints the service status as JSON, exits 1 if not running
library-enrichmentd stop
```

The daemon is the single writer and job owner. It survives adapter exits and retains jobs; an
adapter disconnect does not cancel a job another caller still needs. Logs go to stderr and the
daemon log — never to MCP stdout.

`start` runs in the foreground rather than self-daemonizing, so a supervisor or shell decides
how to background it. Exclusive data-root and cache-root locks prevent competing writers from
reconciling live execution state. A second daemon is rejected before recovery; a socket file
left behind by a crash is reclaimed by the owning service. `stop` is an in-band RPC
call, acknowledged before the daemon exits, so it needs no pidfile.

### Where is the socket?

```sh
library-enrichmentd socket-path    # prints the resolved path and exits
```

The Python adapter re-implements the same resolution order, so
`tests/contract/test_socket_resolution.py` uses this subcommand to compare the two across every
branch. A divergence would be quiet and nasty — the adapter would report the daemon unavailable
while it sat listening elsewhere.

### Validating a wire document

```sh
library-enrichmentd validate contracts/examples/ok.fixture.json   # or read stdin
```

Prints a verdict as JSON and exits 0 for a conforming response envelope, 1 otherwise. Needs no
running daemon: it calls the same validator the `wire.validate` RPC method uses, which is what
lets acceptance gate C19 compare the two boundaries rather than merely observe that each rejects
something. Useful for checking a captured response by hand.

### Where the socket lives

Resolved in this order — ADR 0006 records why, and
`python/enrichment_mcp/daemon_client.py` resolves it identically so the adapter and the daemon
cannot disagree:

| Source | Path |
|---|---|
| `LIBENR_SOCKET` | used verbatim |
| `LIBENR_HOME` | `$LIBENR_HOME/run/d.sock` — the development sandbox |
| `XDG_RUNTIME_DIR` | `$XDG_RUNTIME_DIR/library-enrichment/d.sock` |
| `XDG_CACHE_HOME` | `$XDG_CACHE_HOME/library-enrichment/run/d.sock` |
| `HOME` | `~/.cache/library-enrichment/run/d.sock` |
| nothing set | the daemon refuses to start |

No relative path is accepted from any source, and there is no working-directory fallback: a
relative socket path would resolve against wherever the daemon was started, which may be a
repository under study (§2.3, gate C20).

The transport is bounded newline-delimited JSON-RPC 2.0 — not MCP framing, not LSP framing
(blueprint §2.1). The bound is `limits.rpc_message_bytes` from the configuration, 1 MiB by
default; an over-long frame is refused with `BUDGET_EXCEEDED` rather than buffered.

## Running the adapter

```sh
library-enrichment-mcp             # serves MCP over stdio; normally launched by a client
```

It starts without the daemon: `service_status` then returns `partial`, reporting the daemon as
unavailable and naming the gap. That is deliberate — a stopped daemon is exactly when a caller
most needs a usable answer, so it is reported rather than raised.


## Bounded research and revisions

`max_bytes` bounds the complete service envelope (minimum1024 bytes, configured maximum by
default). MCP transports it once as structured content with a compact text summary and a fixed
512-byte framing allowance. An oversized answer returns `BUDGET_EXCEEDED` with
`data.result_artifact_id`; use `read_artifact` and follow its cursor to recover the immutable
answer. Its per-call request_id is omitted from stored content. A page budget too small for
artifact metadata returns a typed budget error; increase it before continuing. Cursors bind
snapshot/query/scope/budget and cannot be reused across changed requests.

For development revisions, pass `mode="revision"`, a canonical public GitHub repository URL,
a full commit SHA in `revision`, and an explicit `package_subdir` for monorepos. Omit `version`.
The default package root is the repository root; nested packages are never guessed. Revision
identity remains separate from any manifest-declared published version. Static Rust revisions
provide source/configuration/docs with compiled API missing; Python revisions use the static
worker and disclose inferred layout and unmaterialized/generated content. `freshness="offline"`
reuses only the same source registry/repository/root, commit and declared environment.

## Final acceptance commands

Use the same selected execution root throughout qualification and producer acceptance. A rebuilt
executor or changed containment configuration requires fresh qualification before execution tests.
Run qualification and the real container tier serially with other execution clients: ownership
oracles deliberately observe the selected root, and qualification replaces its receipt.

```sh
export LIBENR_EXECUTION_ROOT=/absolute/owned/execution-root
just execution-qualify --apply
just ci
just test-execution
just test-live
LIBENR_CLIENT_USE_OPERATOR_CREDENTIALS=1 just test-client
just acceptance-report
just acceptance-check
```

`test-execution` records the ignored native containment/cleanup tier in
`nextest-execution.json`; reporting joins it with ordinary Rust evidence instead of replacing it.
`test-python` and `test-client` stop on build or execution-environment setup errors. Receipts bind
source (including the shipped skill), dependencies, native executable hashes before/after,
commands, relevant non-secret configuration and logs. The credential reuse flag is recorded as a
boolean; credential values are never recorded. An edited source tree or rebuilt native binary
during a run prevents promotion. A partial run or a prerequisite skip remains visible.

The unattended Codex acceptance home approves the explicitly authorized `inspect_symbol` MCP
tool through its per-tool policy while keeping the client filesystem sandbox read-only. It does
not change the tool's execution annotations or bypass the daemon's contained execution policy.
A04 uses real Serde 1.0.228 registry evidence and actual Context7 documents, whose general `1.0`
examples do not establish that exact patch or the caller's project configuration.
