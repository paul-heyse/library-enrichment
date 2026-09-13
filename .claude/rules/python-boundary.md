---
paths:
  - "python/**"
  - "pyproject.toml"
  - "uv.lock"
  - ".python-version"
  - "tests/**/*.py"
  - "conftest.py"
---

# Python boundary

Python is a **thin adapter** plus an extraction worker. It validates inputs, calls the daemon,
emits MCP results, and maps structured errors. It does not index libraries, own persistent
state, or reimplement any part of the core.

**stdout is the MCP protocol channel.** Every log, warning, progress message, and traceback
goes to stderr or the daemon log. A stray `print()` corrupts the session.

Import FastMCP as `from fastmcp import FastMCP` — the standalone `fastmcp` package, pinned to
an exact v4 release with the lockfile committed. Not `mcp.server.fastmcp`. Do not import the
tasks extension in the default adapter: native tasks require a separate package and client
protocol support, and expensive work goes through ordinary core jobs and `job_control` instead.

**ty is the Python semantic engine.** Not pyrefly, not pyright, not mypy — all three are
present on this workstation and two are globally plugin-enabled, which makes this an easy
mistake. Run `ty` against a generated consumer capsule with its selected interpreter, never
against this service's own virtual environment. Probe LSP capabilities at runtime: an
unsupported method returns `UNSUPPORTED_CAPABILITY`, not an empty result (P08).

Griffe is configured with explicit `search_paths` and `allow_inspection=False`. Without the
latter, Griffe imports packages when static sources are unavailable, which executes library
code and breaks P02. Preserve unresolved aliases rather than guessing them. Do not treat
Griffe's breaking-change checker as a feature-addition detector — compute set and field
differences over normalized snapshots (P06).

The worker interpreter is never the analysed interpreter (P10). Install packages and build
sdists only in service-owned isolated environments. Never import a target library merely to
read its metadata.

Python publicness is not a boolean. Store `__all__`, documented status, underscore convention,
re-export pattern, and author declaration as separate signals. A missing docstring or inventory
entry does not prove an API is private (P07).

`uv` only, always `uv run <tool>`, dev dependencies in `[dependency-groups]`. No unpinned
`uvx ...@latest` anywhere, least of all as a startup command. Ruff `line-length = 100`. No
`# noqa`, no `# type: ignore`.
