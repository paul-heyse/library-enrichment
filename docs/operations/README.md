# Operations

Install, configure, register, and run the service.

> **Nothing here is implemented yet.** These are the procedures the implementation must satisfy
> (blueprint §12). They are recorded now so the setup contract is fixed before code depends on
> it. Commands marked *(not yet available)* will work once the corresponding phase lands.

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

## Running the daemon *(not yet available)*

```sh
library-enrichmentd start
library-enrichmentd status
library-enrichmentd stop
```

The daemon is the single writer and job owner. It survives adapter exits and retains jobs; an
adapter disconnect does not cancel a job another caller still needs. Logs go to stderr and the
daemon log — never to MCP stdout.
