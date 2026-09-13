# Operations

Install, configure, register, and run the service.

> **Phase 0 is implemented.** The daemon runs, the MCP adapter serves its tool catalog, and
> `service_status` answers truthfully. No evidence producer exists yet, so every research tool
> returns a typed `UNSUPPORTED_CAPABILITY` result naming the phase that implements it. Commands
> still marked *(not yet available)* need a later phase.

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

`just state-reset` discards the development sandbox. `just state-leak-check` digests the real
XDG paths before and after a run and fails on any change — so a code path that resolves the
production policy instead of the configured root surfaces immediately.

**No working repository is ever written to, used as a working directory, or used as an
extraction destination.** Acceptance gate C20 proves it with a filesystem digest over a canary
repository, run under every enabled execution profile.

## Execution profiles

| Profile | Permits | Default |
|---|---|---|
| `static` | registry and documentation fetching, safe archive extraction, static parsing, search | enabled |
| `build` | rustdoc fallback, Cargo checks, proc-macro and build-script activity, Python environment setup | enabled only with an operational sandbox |
| `runtime` | importing and executing library code | explicitly enabled only |

A caller selects from the profiles local configuration has enabled; it cannot grant itself
permission. An unavailable profile returns `POLICY_DENIED` with a next action — never a silent
fallback to running on the host.

This workstation has podman, docker and bwrap, so `build` and `runtime` are implementable here.

## Registering the service *(not yet available)*

Register at user scope, by absolute path, once the adapter exists. Derive the repository root
rather than hardcoding it:

```sh
REPO="$(git -C /path/to/library-enrichment rev-parse --show-toplevel)"

claude mcp add --scope user --transport stdio library-enrichment -- \
  "$REPO/.venv/bin/library-enrichment-mcp"

codex mcp add library-enrichment -- \
  "$REPO/.venv/bin/library-enrichment-mcp"
```

Keep Context7 registered separately; this service does not proxy it.

**Registration alone does not prove the service connects.** Make actual tool calls in both
clients and capture the traces — acceptance gates A01 and A02 require real traces, and an
unavailable client is `not_run`, never a mocked pass.

`scripts/hooks/pre_bash.sh` blocks these commands during agent sessions unless
`LIBENR_ALLOW_USER_INSTALL=1`, because they mutate user-scope configuration.

## Installing the companion skill

```sh
just install-skill              # dry run: prints destinations, writes nothing
just install-skill --apply      # install
just install-skill --uninstall  # remove
```

Defaults to a dry run by design: `AGENT_HANDOFF.md` forbids installing the skill into user
configuration without explicit setup invocation. It refuses to overwrite an unrelated skill of
the same name and records an installation manifest with the source commit and skill digest.

On this workstation `~/.claude/skills` is a symlink into `~/.codex/skills`; the installer
resolves and deduplicates destinations so the skill is not written twice.

Installing the skill does **not** register the service. They are separate steps.

## Running the daemon

```sh
cargo build -p enrichment-daemon   # or `just check`; the binary lands in target/debug
library-enrichmentd start          # foreground; Ctrl-C or SIGTERM stops it
library-enrichmentd status         # prints the service status as JSON, exits 1 if not running
library-enrichmentd stop
```

The daemon is the single writer and job owner. It survives adapter exits and retains jobs; an
adapter disconnect does not cancel a job another caller still needs. Logs go to stderr and the
daemon log — never to MCP stdout.

`start` runs in the foreground rather than self-daemonizing, so a supervisor or shell decides
how to background it. Two daemons cannot share a socket: a second `start` fails with
`AddrInUse`, while a socket file left behind by a crash is reclaimed. `stop` is an in-band RPC
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
