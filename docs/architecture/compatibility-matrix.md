# Compatibility matrix

The Phase 0 gate. `AGENT_HANDOFF.md`: *"Verify current upstream APIs and pin compatible tool
versions. The blueprint specifies architecture and behavior, not an assurance that every
dependency's latest release composes correctly."*

Every row needs a primary source, a retrieval date, and an exact quote. **A row with an empty
source is not a pin — it is an open question.** Fill rows with `/verify-upstream`, which runs the
`upstream-verifier` subagent against the 26 catalogued sources in `docs/blueprint/SOURCES.md`.

Verdicts: `verified` (read this session from the primary source) · `unverified` (not yet
established; says what was tried) · `contradicted` (upstream differs from a blueprint
assumption — **stop and open an ADR**).

---

## Measured on this workstation

These were produced by running the tools, not by reading about them.

### rustdoc JSON format versions

`just rustdoc-format-matrix`, measured 2026-09-13. Each installed dated nightly was used to
build a trivial crate with `-Z unstable-options --output-format json`, and the emitted
`format_version` recorded.

| Toolchain | rustc | Commit | `format_version` | Verdict |
|---|---|---|---:|---|
| `nightly-2026-02-28` | 1.95.0-nightly | `3a70d0349` | 57 | verified (measured) |
| `nightly-2026-05-23` | 1.98.0-nightly | `54333ff07` | 57 | verified (measured) |
| `nightly-2026-07-15` | 1.99.0-nightly | `da80ed070` | 60 | verified (measured) |
| `nightly-2026-08-18` | 1.100.0-nightly | `8fa1c96cf` | 61 | verified (measured) |
| **`nightly-2026-09-13`** | **1.100.0-nightly** | **`809936eac`** | **61** | **verified (measured) — selected producer** |

Consequences:

- The rustdoc producer identity is recorded in `config/toolchains.toml` as
  `nightly-2026-09-13` / format 61 — the current nightly, written in dated form because
  `nightly-2026-09-13` is byte-identical to this workstation's rolling `nightly` channel
  (both `809936eac`). That makes the pin simultaneously newest-available and reproducible.
- **The parser adapter must inspect `format_version` at read time.** Four installed toolchains
  span three format versions, and docs.rs may serve a version built by any rustdoc release
  (blueprint §4.1 [S07]).
- Acceptance gate R04 ("unsupported rustdoc JSON format → typed error, no silent misparse") has
  genuine supported *and* unsupported captures from this spread, with no synthesised artifact.
  Captures are under `$LIBENR_CACHE_HOME/rustdoc-format-matrix/`.

### Local toolchain

| Tool | Version | Note |
|---|---|---|
| rustc (pinned) | 1.98.1 (`48a229cea` 2026-09-01) | **Current stable.** `rustup check` on 2026-09-13: "stable — up to date: 1.98.1". Written as an exact version, not `stable`, so the pin does not float. Verified active by `just toolchain-check`. |
| rustup default | **nightly** 1.100.0 (`809936eac` 2026-09-12) | Why the stable pin is load-bearing — a bare `cargo` without it targets nightly |
| rustdoc producer | `nightly-2026-09-13` (`809936eac`) | Current nightly, dated for reproducibility |
| rust-analyzer | 1.98.1 (`48a229c`) | Rust semantic evidence over LSP |
| uv | 0.12.13 | The only supported Python package manager |
| just | 1.58.0 | Task surface |
| Python (system) | 3.12.3 | Not the target; see ADR 0004 |
| podman / docker / bwrap | all present | `build` and `runtime` profiles are implementable here |
| `ty` | **0.0.80** (project venv) | `/home/paul/library-enrichment/.venv/bin/ty`, not on `PATH`. Measured 2026-09-13: `ty version` → `ty 0.0.80`; `ty server` serves `implementationProvider: true`. Invoke by absolute venv path, never bare `ty`. |
| `cargo-public-api` (CLI) | absent | Intentional — blueprint §4.2 prefers the `public-api` library consuming rustdoc JSON directly |

---

## To verify

Nothing below is pinned yet. Do not add a dependency version to `Cargo.toml` or
`pyproject.toml` until its row is `verified`.

### Python boundary

Rows marked *verified (executed)* were established by resolving and importing the package on
Python 3.14.7 in this repository's own environment on 2026-09-13, not by reading about it.

| Claim | Source | Retrieved | Evidence | Verdict | Pin |
|---|---|---|---|---|---|
| FastMCP 4 current release resolving on 3.14 | executed | 2026-09-13 | `uv lock` → `fastmcp 4.0.3` (pulls `mcp 2.2.0`, `pydantic 2.13.5`) | **verified (executed)** | `fastmcp==4.0.3` |
| `from fastmcp import FastMCP` is the supported import | executed | 2026-09-13 | resolves to `<class 'fastmcp.server.server.FastMCP'>` | **verified (executed)** | — |
| `ToolResult` is available for output control | executed | 2026-09-13 | `fastmcp.tools.base.ToolResult` | **verified (executed)** | — |
| Native tasks need a separate package | executed | 2026-09-13 | `import fastmcp.tasks` → `ModuleNotFoundError` in the base install | **verified (executed)** | — |
| Griffe current release resolving on 3.14 | executed | 2026-09-13 | `uv lock` → `griffe 2.3.0` | **verified (executed)** | `griffe==2.3.0` |
| **`griffe.load(allow_inspection=...)` defaults to `True`** | executed | 2026-09-13 | `inspect.signature(griffe.load)` → `allow_inspection` default `True`; `search_paths` default `None`; `force_inspection` default `False` | **verified (executed)** | — |
| **Published `griffe.load()` signature matches installed 2.3.0 byte for byte** | [S12] + executed | 2026-09-13 | Docs signature: `load(objspec, /, *, submodules=True, try_relative_path=True, extensions=None, search_paths=None, docstring_parser=None, docstring_options=None, lines_collection=None, modules_collection=None, allow_inspection=True, force_inspection=False, store_source=True, find_stubs_package=False, prefer_stubs_docs=False, resolve_aliases=False, resolve_external=None, resolve_implicit=False) -> Object \| Alias`. Identical to `inspect.signature(griffe.load)` on griffe 2.3.0. All three parameters of interest are KEYWORD_ONLY | **verified (executed)** | — |
| **`allow_inspection=False` is what stops Griffe importing a package** | [S12] + executed | 2026-09-13 | "If you want to be careful about what gets executed in the current Python process, you can choose to disallow dynamic analysis by passing the `allow_inspection=False` argument. If Griffe cannot find sources for a package, it will not try to import it and will instead fail with a `ModuleNotFoundError` directly." Measured on griffe 2.3.0: `griffe.load("itertools")` succeeds via introspection; `griffe.load("itertools", allow_inspection=False)` raises `ModuleNotFoundError: itertools` | **verified (executed)** | — |
| Griffe falls back to introspection by default, and always for compiled modules | [S12] | 2026-09-13 | "Griffe will first try to find sources, and will fall back to introspection if it cannot find any. When Griffe finds compiled modules within a packages, it uses introspection again to extract API information." Docstring: `allow_inspection: Whether to allow inspecting modules when visiting them is not possible.` | **verified** | — |
| `force_inspection=True` means import-and-inspect even when sources exist | [S12] | 2026-09-13 | "If for some reason you want Griffe to use dynamic analysis instead (importing and inspecting runtime objects), you can pass the `force_inspection=True` argument"; "dynamic analysis will execute code, possibly arbitrary code if you import third-party dependencies, putting you at risk". Docstring: `force_inspection: Whether to force using dynamic analysis when loading data.` | **verified** | — |
| `search_paths` scopes where Griffe looks before falling back to import | [S12] | 2026-09-13 | "To specify in which directories Griffe should search for packages and modules, you can use the `search_paths` parameter on both the `load` function and the `GriffeLoader` class." and "If Griffe cannot find sources for the specified object in the given search paths, it will try to import the specified object and use dynamic analysis on it (introspection)." Docstring: `search_paths: The paths to search into.` | **verified** | — |
| ty current release resolving on 3.14 | executed | 2026-09-13 | `uv lock` → `ty 0.0.80`; `.venv/bin/ty --version` → `ty 0.0.80` | **verified (executed)** | `ty==0.0.80` |
| **ty still lists `textDocument/implementation` as unsupported** | [S18] | 2026-09-13 | Capability table: "`textDocument/implementation` \| ❌ Not supported \| #3514" | **contradicted** | — |
| **ty 0.0.80 actually serves `textDocument/implementation`** | executed + ty CHANGELOG | 2026-09-13 | LSP `initialize` against `.venv/bin/ty server` returns `implementationProvider: true`; a request on a base class returns 3 locations (base + both subclasses). CHANGELOG 0.0.64 (2026-07-27): "Implement LSP `textDocument/implementation` request ([#25410])" | **verified (executed)** | — |
| ty issue #3514 ("Support for goto implementations") is closed | github.com/astral-sh/ty/issues/3514 | 2026-09-13 | `state: closed`, `state_reason: completed`, `closed_at: 2026-07-26`, closed by MERGED ruff PR #25410 | **verified** | — |
| `ty server` and `ty check` exist as subcommands | [S19] + executed | 2026-09-13 | CLI reference Commands: "ty check — Check a project for type errors / ty server — Start the language server"; `.venv/bin/ty --help` prints the same five commands | **verified** | `ty==0.0.80` |
| **`ty check --python <PATH>` targets an external interpreter/venv** | [S19] + executed | 2026-09-13 | "Path to your project's Python environment or interpreter… This can be a path to: A Python interpreter, e.g. `.venv/bin/python3` - A virtual environment directory, e.g. `.venv` - A system Python `sys.prefix` directory, e.g. `/usr`" | **verified (executed)** | — |
| `ty server` takes no transport/config flags | [S19] + executed | 2026-09-13 | "Usage: ty server / Options: -h, --help  Print help" — stdio only, configured via LSP `initialize` | **verified (executed)** | — |
| `ty check --python-version` accepts `3.14` | [S19] | 2026-09-13 | Possible values: "3.7 3.8 3.9 3.10 3.11 3.12 3.13 3.14 3.15" | **verified** | — |
| `ToolResult` semantics: summary text vs structured data | [S02] | 2026-09-13 | "`content` - The traditional MCP content blocks that clients display to users… `structured_content` - A dictionary containing structured data that matches your tool's output schema… If only `structured_content` is provided, it will also be used as `content` (converted to JSON string)." | **verified** | — |
| `ToolResult` suppresses FastMCP's automatic wrapping | [S02] | 2026-09-13 | "When returning `ToolResult`, you have full control - FastMCP won't automatically wrap or transform your data. `ToolResult` can be returned with or without an output schema." | **verified** | — |
| `ToolResult` has a 4th field `is_error`, undocumented on [S02] | executed | 2026-09-13 | `ToolResult.model_fields` → `['content','structured_content','meta','is_error']`; [S02] says "`ToolResult` accepts three fields" | **verified (executed)** | — |
| Tool annotations use `readOnlyHint`/`destructiveHint`/`openWorldHint` | [S02] + executed | 2026-09-13 | "`@mcp.tool(annotations=ToolAnnotations(title=…, readOnlyHint=True, openWorldHint=False))`"; `mcp.types.ToolAnnotations` stores them snake_case and emits camelCase under `model_dump(by_alias=True)` | **verified (executed)** | — |
| Annotation defaults if unset: `destructiveHint=true`, `openWorldHint=true` | [S02] | 2026-09-13 | Table: "`readOnlyHint` \| boolean \| false … `destructiveHint` \| boolean \| true … `openWorldHint` \| boolean \| true" | **verified** | — |
| Read-only hints change client behavior, so they must be accurate | [S02] | 2026-09-13 | "MCP clients like Claude and ChatGPT use annotation hints to determine when to skip confirmation prompts… Always focus on making your annotations accurately represent what your tool actually does." | **verified** | — |
| Native tasks require a separate package | [S03] | 2026-09-13 | "Background tasks require the `fastmcp-tasks` package"; `pip install "fastmcp[tasks]"`; import is `from fastmcp_tasks import TasksExtension` | **verified** | — |
| Native tasks also require client protocol support | [S03] | 2026-09-13 | "the tasks capability is negotiated over `2026-07-28` connections, so a client pinned to `mode=\"legacy\"` never triggers one — the tool always runs synchronously for it" | **verified** | — |
| A `task=True` tool without the extension fails at startup | [S03] | 2026-09-13 | "a `task=True` tool on a server with no tasks extension registered raises at server startup" | **verified** | — |
| Neither `fastmcp.tasks` nor `fastmcp_tasks` is importable in the base install | executed | 2026-09-13 | both raise `ModuleNotFoundError` on the repo `.venv` (fastmcp 4.0.3, Python 3.14.7) | **verified (executed)** | — |
| **stdio servers are client-managed, not long-lived** | [S05] | 2026-09-13 | "With STDIO transport, the client spawns a new server process for each session and manages its lifecycle… This is why STDIO servers don't stay running - they're started on-demand by the client." | **verified** | — |
| stdio is the default transport for `mcp.run()` | [S05] | 2026-09-13 | "STDIO … is the default transport for FastMCP servers. When you call `run()` without arguments, your server uses STDIO transport." | **verified** | — |
| In-process contract tests use `Client(transport=mcp)` + pytest-asyncio | [S06] | 2026-09-13 | "`async with Client(transport=mcp) as mcp_client: yield mcp_client`"; requires `pytest-asyncio` with `asyncio_mode = "auto"`; `result.data` carries the deserialized value | **verified** | — |
| FastMCP recommends exact-version pinning | [S01] | 2026-09-13 | "For production use, always pin to exact versions: `fastmcp==4.0.0    # Good - an exact version` / `fastmcp>=4.0.0    # Bad - may install breaking changes`" | **verified** | `fastmcp==4.0.3` |
| Current FastMCP 4.x release is 4.0.3 | PyPI JSON [S14] | 2026-09-13 | `pypi.org/pypi/fastmcp/json` → `info.version = "4.0.3"` (uploaded 2026-09-05); 4.x line: 4.0.0 (2026-08-31), 4.0.1, 4.0.2, 4.0.3 | **verified** | `fastmcp==4.0.3` |
| FastMCP `requires_python` admits 3.14, but classifiers stop at 3.13 | PyPI JSON | 2026-09-13 | `requires_python = ">=3.10"`; classifiers list only 3.10–3.13. No 3.14 classifier — yet `fastmcp version` runs on Python 3.14.7 here | **verified (executed)** | — |
| Importable code lives in the `fastmcp-slim` distribution since 3.3 | [S01] | 2026-09-13 | "FastMCP 3.3 moved the importable code from the `fastmcp` distribution into `fastmcp-slim`." Affects vendoring/lockfile expectations | **verified** | — |

Two of these change how the code must be written:

- **`allow_inspection` defaults to `True`.** Griffe will import a package when static sources
  are unavailable unless `False` is passed explicitly, which would execute library code inside
  the extraction worker and fail gate P02. `rules/griffe-static-only.yml` enforces the explicit
  argument in the edit loop. Note `force_inspection` also exists and must stay `False`.
- **`fastmcp.tasks` is absent from the base install**, confirming [S03]. The default adapter
  must not import it; expensive work goes through ordinary core jobs and `job_control`
  (blueprint §7.4).

#### CONTRADICTED: gate P08 no longer holds

**ty added `textDocument/implementation` in 0.0.64 (2026-07-27). The pinned `ty 0.0.80` serves
it. Gate P08's `UNSUPPORTED_CAPABILITY` assertion is now false and must be rewritten — this is
an ADR, not a quiet test edit.**

The published capability table [S18] is *stale*, not merely conservative: it still prints
"`textDocument/implementation` | ❌ Not supported | #3514" and links issue #3514, which GitHub
reports as `closed` / `completed` on 2026-07-26 by merged PR astral-sh/ruff#25410, "[ty]
Implement LSP `textDocument/implementation` request". The ty CHANGELOG lists that PR under
release 0.0.64. The page footer reads "September 9, 2026", so the doc has been republished
since the feature landed without the table being corrected.

Measured against `.venv/bin/ty server` (serverInfo `{"name":"ty","version":"0.0.80"}`) on
2026-09-13:

- `initialize` returns `implementationProvider: true` — whether or not the client advertises
  `textDocument.implementation`, so the service cannot opt out by withholding the capability.
- A request on a nominal base class returns the base plus every subclass (3 locations for
  `Base`/`Child`/`Other`); on an `@abstractmethod`, the abstract def plus both overrides.
- A `Protocol` class and its members return **only themselves** — structural implementors are
  not resolved. So the Python evidence path gets real nominal-subclass data and a silently
  incomplete answer for protocols, which is a correctness trap of its own.

Consequences: the Python semantic adapter must call `textDocument/implementation` and map real
locations, P08 needs a new assertion, and no `UNSUPPORTED_CAPABILITY` branch may be tested
against this method. Because the docs table is wrong in the *permissive* direction, treat [S18]
as unreliable for the whole table and probe `initialize` at runtime instead of trusting it.
`ty 0.0.80` is pre-1.0; expect further churn.

Also from this round: [S02] understates `ToolResult` (it lists three fields; the class has four,
including `is_error`), and [S01]'s pinning example prints `4.0.0` while PyPI's current 4.x
release is `4.0.3`. Read exact versions from PyPI, not from doc prose.

The three "supports Python 3.14" rows together decide ADR 0004. If any is `contradicted`, stop.

### Rust core

| Claim | Source | Retrieved | Quote | Verdict | Pin |
|---|---|---|---|---|---|
| DataFusion current release | executed | 2026-09-13 | `cargo add datafusion` in a clean crate on the pinned stable toolchain → `datafusion v55.1.0` | **verified (executed)** | `datafusion = "55.1.0"` |
| `arrow` major required by that DataFusion | executed | 2026-09-13 | `cargo tree -i arrow` in the real workspace → `arrow v59.3.0`; `arrow-schema v59.3.0` | **verified (executed)** | `arrow = "59.3.0"` |
| `parquet` compatible with that `arrow` | executed | 2026-09-13 | `cargo tree -i parquet` → `parquet v59.3.0`, same major as `arrow` | **verified (executed)** | `parquet = "59.3.0"` |
| `object_store` compatible with that `arrow` | executed | 2026-09-13 | `cargo tree -i object_store` → `object_store v0.13.2`, selected by datafusion 55.1.0 | **verified (executed)** | `object_store = "0.13.2"` |
| **One Arrow type universe: no duplicate majors** | executed | 2026-09-13 | `cargo tree -d` in the real workspace matches no `arrow`/`parquet`/`datafusion`/`object_store` entry. `cargo deny check bans` passes with `multiple-versions = "deny"` and 13 reasoned skips, none of them an Arrow-stack crate | **verified (executed)** | — |
| `public-api` release parsing format_version 61 | [S10] + executed | 2026-09-13 | `public-api v0.52.2` built against each locally captured rustdoc JSON: **format 57, 60 and 61 all parsed**, 25 items each, no rejections. Consumes rustdoc JSON directly, so no Cargo build is needed (§4.2) | **verified (executed)** | `public-api = "0.52.2"` |
| The pinned set compiles together | executed | 2026-09-13 | `cargo check --workspace --all-targets` succeeds with datafusion/arrow/parquet/object_store in `enrichment-store` and public-api in `enrichment-core` | **verified (executed)** | — |
| **docs.rs hosted rustdoc JSON endpoint is `/crate/{crate}/{version}/json`** | [S07] | 2026-09-13 | Documented URL table: "`https://docs.rs/crate/clap/latest/json`" → "latest version, default target, latest format-version". Measured: `GET https://docs.rs/crate/serde/1.0.219/json` → `200`, `num_redirects=0` | **verified (executed)** | — |
| Default compression is zstd; `.gz` opts into gzip | [S07] | 2026-09-13 | "By default we use `zstd` compression, which is more space efficient and faster to decompress. For more limited environments we also started supporting `gzip` compression. You can receive gzip by adding `.gz` to the given URL." Measured: `content-type: application/zstd` vs `application/gzip` for `/json.gz` | **verified (executed)** | — |
| `format_version` must be read from the payload, not assumed | [S07] | 2026-09-13 | "The JSON file you're downloading might have been built with an older version of rustdoc so you might have to check the `format_version` attribute to determine how to parse the structure. The `rustdoc-types` crate helps with this." Measured: serde 1.0.219 → 53, tokio 1.53.1 → 60, clap 4.6.6 → 61 | **verified (executed)** | — |
| Format-pinned URL form is `/json/{format_version}` and 404s for any other | [S07] | 2026-09-13 | "`https://docs.rs/crate/clap/latest/json/42`" → "format-version 42". Measured on 4 crates: exactly one format version returns `200` per build (clap→61, syn→61, tokio→60, serde→60); every neighbouring version 404s | **verified (executed)** | — |
| Missing build / pre-2025-05-23 release returns `404 NOT FOUND` | [S07] | 2026-09-13 | "In case of rebuilds we also keep old format versions around. Until we rebuilt all releases, we might return a `404 NOT FOUND` for some." and "We started building rustdoc JSON on 2025-05-23." Measured: `/crate/serde/1.0.0/json` → `404` | **verified (executed)** | — |
| Semver-range URLs redirect; `latest` and exact versions do not | [S07] | 2026-09-13 | Measured: `/crate/clap/~4/json` → `num_redirects=1`, final `/crate/clap/4.6.6/json`; `/crate/clap/latest/json` → `num_redirects=0` | **verified (executed)** | — |
| **Target-specific form is valid but JSON exists only for built targets** | [S07] | 2026-09-13 | Page's own example `https://docs.rs/crate/clap/latest/i686-pc-windows-msvc/json` measured → **`404`**. The default-target form `/crate/clap/4.6.6/x86_64-unknown-linux-gnu/json` → `200`. `content-disposition` names the target: `clap_4.6.6_x86_64-unknown-linux-gnu_latest.json.zst` | **contradicted (doc example stale; form itself verified)** | — |
| No documented retention policy limiting how many format versions are kept | [S07] | 2026-09-13 | The page states only "In case of rebuilds we also keep old format versions around." No "latest N formats" statement appears anywhere on the page (full page read, 10,162 bytes) | **verified (absence)** | — |
| docs.rs build metadata fields under `[package.metadata.docs.rs]` | [S08] | 2026-09-13 | "You can customize docs.rs builds by defining `[package.metadata.docs.rs]` table in your crates' `Cargo.toml`." Complete key list: `features` (default `[]`), `all-features` (default `false`), `no-default-features` (default `false`), `default-target` (default `"x86_64-unknown-linux-gnu"`), `targets`, `additional-targets`, `rustc-args` (default `[]`), `rustdoc-args` (default `[]`), `cargo-args` | **verified** | — |
| Default `targets` set when unconfigured | [S08] | 2026-09-13 | "Default targets: `x86_64-unknown-linux-gnu`, `aarch64-apple-darwin`, `x86_64-pc-windows-msvc`, `aarch64-unknown-linux-gnu`, `i686-pc-windows-msvc`" and "If both `default-target` and `targets` are unset, all tier-one targets will be built and `x86_64-unknown-linux-gnu` will be used as the default target." | **verified** | — |
| rustdoc JSON is nightly-gated behind `-Z unstable-options` | [S26] | 2026-09-13 | "These features are enabled by passing a command-line flag to Rustdoc, but the flags in question are themselves marked as unstable. To use any of these options, pass `-Z unstable-options` as well as the flag in question to Rustdoc on the command-line." and "`--output-format json` emits documentation in the experimental JSON format." Flag is spelled `-w`/`--output-format` | **verified** | — |
| The `-Z unstable-options --output-format json` shape still works on the pinned nightly | [S26] + executed | 2026-09-13 | `rustdoc +nightly-2026-09-13 -Z unstable-options --output-format json --out-dir OUT lib.rs` → `lib.json` with `format_version = 61`. Stable `rustdoc --help` lists only `-w, --output-format [html]` | **verified (executed)** | — |
| Toolchain crate JSON ships as a rustup component | [S26] | 2026-09-13 | "JSON Output for toolchain crates (std, alloc, core, test, and proc_macro) is available via the `rust-docs-json` rustup component." — `rustup component add --toolchain nightly rust-docs-json` | **verified** | — |

**Pin DataFusion first and derive `arrow`, `parquet` and `object_store` from it.** The reverse
order fails: `deny.toml` sets `multiple-versions = "deny"` because two `arrow` majors in one
graph means two incompatible `RecordBatch` types (blueprint §8.4). Done in that order on
2026-09-13, `datafusion 55.1.0` selects `arrow`/`arrow-schema`/`parquet 59.3.0` and
`object_store 0.13.2`, and the resulting graph has no Arrow-stack duplicate.

Note the version skew this creates for the rustdoc producer: our pinned nightly emits
**format 61**, `public-api 0.52.2` parses 57, 60 and 61, but hosted builds on docs.rs go back
to at least **53** (serde 1.0.219). The parser adapter must therefore handle a range it did not
produce, and `public-api`'s own supported range is the binding constraint — not the nightly's.

Consequences of the [S07]/[S08]/[S26] round (all read or measured 2026-09-13):

- **Blueprint §4.1's "illustrative" endpoint is correct as written.** `https://docs.rs/crate/{crate}/{exact-version}/json` resolves `200` with zero redirects. Its surrounding caveats also hold: `latest` does not redirect, but a semver range such as `~4` redirects once to the exact version, so the fetcher must still follow redirects to learn *which* version it actually got.
- **There is no "request the format version you can parse" escape hatch.** Across clap, syn, tokio and serde, exactly one `/json/{n}` returns `200` per build — the version rustdoc emitted at build time. §4.1's "A format-specific download is useful only when docs.rs actually has that format" is therefore the operative rule: probe or read `format_version`, never negotiate it. Pairing this with the measured nightly spread above, the pinned `nightly-2026-09-13` emits 61, which matches recently built crates (clap, syn) but **not** older hosted builds (serde 1.0.219 → 53), so the parser adapter is load-bearing, not defensive.
- **docs.rs's own target-specific example URL 404s.** The documented `…/latest/i686-pc-windows-msvc/json` is dead while `…/4.6.6/x86_64-unknown-linux-gnu/json` works. Target-qualified JSON exists only for targets actually built with JSON output. Treat a target-qualified 404 as "no JSON for this target", never as "crate missing", and read the real target back out of `content-disposition` (`{crate}_{version}_{target}_{format}.json.zst`) rather than trusting the requested one.
- **[S08] confirms blueprint §4.3's premise and adds two keys it does not name.** Alongside `features` / `no-default-features` / `all-features` / `targets` / `rustdoc-args`, maintainers can also set `default-target`, `additional-targets`, `rustc-args` and `cargo-args`. `observed_configuration` must capture all nine, because `cargo-args` and `rustc-args` can change what is compiled at all — not merely which docs are rendered.

### Registries and clients

| Claim | Source | Retrieved | Quote | Verdict |
|---|---|---|---|---|
| PyPI JSON API routes | [S14] | 2026-09-13 | "Route: `GET /pypi/<project>/json`" and "Route: `GET /pypi/<project>/<version>/json` — Returns metadata about an individual release at a specific version, otherwise identical to `/pypi/<project_name>/json` minus the `releases` key." | **verified** |
| **PyPI JSON: per-file hashes live in `digests`, not a top-level field** | [S14] + executed | 2026-09-13 | Measured `pypi.org/pypi/griffe/2.3.0/json`: top-level keys `info`, `last_serial`, `ownership`, `urls`, `vulnerabilities`. Each entry of `urls` has `digests` = `{blake2b_256, md5, sha256}`, plus `filename`, `url`, `size`, `packagetype`, `requires_python`, `yanked`, `yanked_reason`, `core-metadata`, `upload_time_iso_8601` | **verified (executed)** |
| PyPI JSON: `requires_python` and `yanked` appear at both levels | [S14] + executed | 2026-09-13 | `info.requires_python = ">=3.10"` and `info.yanked = False` (release-level); each file object carries its own `requires_python` and `yanked`/`yanked_reason`. The per-file value is the one installers honour | **verified (executed)** |
| **PyPI JSON `releases` key is deprecated in favour of the Index API** | [S14] | 2026-09-13 | "Deprecated keys … `releases`: projects should shift to using the Index API to get this information, where possible. `downloads`: this key is always -1 … `has_sig`: this key is always false … `bugtrack_url`: this key is always null … In the future, each of these keys may be removed entirely from this API response." Also: `releases` was already removed from the release-specific endpoint | **verified** |
| PyPI JSON metadata is upload-time, immutable, and may not match the files | [S14] | 2026-09-13 | "Metadata returned comes from the values provided at upload time and does not necessarily match the content of the uploaded files. The first uploaded data for a release is stored, subsequent uploads do not update it." | **verified** |
| **Simple API: file listing is `GET /simple/<normalized-name>/` with an explicit Accept header** | [S15] + executed | 2026-09-13 | "application/vnd.pypi.simple.v1+json". Measured: `curl -H 'Accept: application/vnd.pypi.simple.v1+json' https://pypi.org/simple/griffe/` → `200`, `content-type: application/vnd.pypi.simple.v1+json`, `meta.api-version = "1.4"`. **With no Accept header the same URL returns `content-type: text/html`** | **verified (executed)** |
| Simple API JSON: hashes, requires-python, yanked are per-file, hyphenated | [S15] + executed | 2026-09-13 | "`hashes`: A dictionary mapping a hash name to a hex encoded digest of the file… The `hashes` dictionary MUST be present, even if no hashes are available"; "`requires-python`: An optional key that exposes the Requires-Python metadata field"; "`yanked`: An optional key which may be either a boolean… or a non empty, but otherwise arbitrary, string to indicate that a file has been yanked with a specific reason"; "`size`: A mandatory key." Measured file keys: `filename`, `url`, `hashes`, `requires-python`, `yanked`, `size`, `upload-time`, `core-metadata`, `provenance`, `data-dist-info-metadata` | **verified (executed)** |
| Simple API recommends a q-weighted Accept list | [S15] | 2026-09-13 | `CONTENT_TYPES = ["application/vnd.pypi.simple.v1+json", "application/vnd.pypi.simple.v1+html;q=0.2", "text/html;q=0.01"]` and "the server… may even return a content type that they did not ask for… servers MUST send a `Content-Type` header indicating the content type of the response" | **verified** |
| **`objects.inv` is Sphinx inventory version 2, zlib-compressed after a 4-line plain header** | [S17] + executed | 2026-09-13 | Retrieved three live inventories. Byte-exact header: `# Sphinx inventory version 2` / `# Project: Python` / `# Version: 3.14` / `# The remainder of this file is compressed using zlib.` Body lines are `name domain:role priority uri dispname`, where `$` in `uri` expands to `name` and `-` in `dispname` means "same as name" | **verified (executed)** |
| **`objects.inv` contains non-Python entries (blueprint §5.3)** | [S17] + executed | 2026-09-13 | docs.python.org/3 inventory, 19,449 entries: `c:functionParam` 2250, `std:label` 2078, `c:function` 1310, `std:doc` 538, `c:member` 468, `c:macro` 427, `std:cmdoption` 410. Sphinx's own inventory adds `std:confval` 341, `rst:directive:option` 176, `rst:directive` 120, `rst:role` 81. [S17] independently names "`std:doc`, `py:func`, or `cpp:class`" as reference types | **verified (executed)** |
| Inventory version is retainable from the header, per blueprint §5.3 | [S17] + executed | 2026-09-13 | `# Project:` and `# Version:` header lines carry the documented project and version (Python `3.14`, Sphinx `9.1.1`, NumPy `2.5`). [S17] also documents the inspection command: "`python -m sphinx.ext.intersphinx https://docs.python.org/3/objects.inv`" | **verified (executed)** |
| Context7 current tool names and arguments | [S21] | | | partially verified |
| **Codex stdio MCP registration** | [S22] + executed | 2026-09-13 | `codex mcp add --help` (codex-cli 0.154.0): `Usage: codex mcp add [OPTIONS] <NAME> (--url <URL> \| -- <COMMAND>...)`. `--env <KEY=VALUE>` is "Only valid with stdio servers". Config is written to `~/.codex/config.toml`. `codex mcp list` renders a table with Name/Command/Args/Env/Cwd/Status/Auth | **verified (executed)** |
| **Claude Code stdio MCP registration, user scope** | [S24] + executed | 2026-09-13 | `claude mcp add --help` (Claude Code 2.1.270): `Usage: claude mcp add [options] <name> <commandOrUrl> [args...]`; `-s, --scope <scope>  Configuration scope (local, user, or project) (default: "local")`; `-t, --transport <transport>  Transport type (stdio, sse, http). Defaults to stdio if not specified.` Documented example uses a `--` separator: `claude mcp add my-server -- my-command --some-flag arg1` | **verified (executed)** |
| Blueprint §12.2's registration templates are still accurate | executed | 2026-09-13 | Both templates typecheck against the live CLIs: `claude mcp add --scope user --transport stdio <name> -- <abs-path>` and `codex mcp add <name> -- <abs-path>`. `--scope user` and `--transport stdio` are both real flags with the documented meanings | **verified (executed)** |

Consequences of the [S14]/[S15]/[S17] round (all read or measured 2026-09-13):

- **The Simple API needs an explicit `Accept` header or it silently returns HTML.** `https://pypi.org/simple/griffe/` with no `Accept` responds `content-type: text/html`; with `Accept: application/vnd.pypi.simple.v1+json` it responds `application/vnd.pypi.simple.v1+json`. A client that forgets the header gets a `200` and a parse failure, not an error. Always send the q-weighted list from [S15] and dispatch on the returned `Content-Type`, which the spec requires servers to set.
- **Prefer the Simple API over the JSON API for file listings.** [S14] deprecates `releases` in favour of the Index API and has already removed it from the release-specific endpoint. The Simple API also carries `size` as a mandatory key and `upload-time`, and PyPI currently serves `api-version 1.4`.
- **Hash field names differ between the two APIs.** JSON API: `digests` with `blake2b_256` / `md5` / `sha256`. Simple API: `hashes`, lowercase hashlib names, `sha256` recommended. Both expose per-file `yanked` and per-file Python compatibility, but the JSON API spells it `requires_python` and the Simple API `requires-python`. Do not share a deserializer.
- **PyPI JSON metadata is frozen at first upload.** "does not necessarily match the content of the uploaded files… The first uploaded data for a release is stored, subsequent uploads do not update it." It is a claim about a release, not evidence about an artifact — which is the same `observed` vs `requested` split blueprint §4.3 draws for Rust.
- **Blueprint §5.3's `objects.inv` assertion is confirmed, and stronger than stated.** In docs.python.org/3's 19,449-entry inventory the C domain and `std:` entries together outnumber several Python roles (`c:functionParam` 2250 vs `py:class` 1052). Sphinx's own inventory carries `std:confval`, `rst:directive` and `rst:role`. A resolver that assumes `py:*` will drop most of a typical inventory. The version to retain per §5.3 is on header line 3 (`# Version: 3.14`); line 1 is `# Sphinx inventory version 2`, the *format* version, and the two must not be conflated.

Registration was verified by reading the installed CLIs' own help output, not by running
`mcp add` — that mutates user-scope configuration and `scripts/hooks/pre_bash.sh` blocks it
without an explicit operator opt-in. Gates A01 and A02 still require *real tool call traces*
from each client and stay `not_run` until the adapter exists and those traces are captured.

Context7 is *partially* verified by direct observation on 2026-09-13: it is connected to Claude
Code as a plugin MCP exposing `resolve-library-id` and `query-docs`, surfaced under the
namespaced names `mcp__plugin_context7_context7__resolve-library-id` and
`…__query-docs`. This confirms blueprint §1.2's warning that clients namespace these tools and
the service must discover actual tool definitions rather than hardcoding a prefix. The upstream
argument shapes are still unverified.

### Phase 1 Rust dependencies: fetch, decompress, extract, compare, hash

All rows read on 2026-09-13 from the published crate manifests and sources in the local cargo
registry (`~/.cargo/registry/src/index.crates.io-*/`, byte-identical to what crates.io serves),
the crates.io API (`/api/v1/crates/{name}`), and two scratch crates built with
`cargo +1.98.1` outside the repository. *verified (executed)* means a command ran, not a page
was read. Newest published release is chosen unless a row says why not.

| Claim | Source | Retrieved | Quote or evidence | Verdict | Pin |
|---|---|---|---|---|---|
| reqwest current release | crates.io API | 2026-09-13 | `newest_version = "0.13.5"`, created 2026-09-08, `license = "MIT OR Apache-2.0"`, `rust_version = "1.85.0"` | **verified** | `reqwest = "0.13.5"` |
| **reqwest 0.13 has no `rustls-tls`, `rustls-tls-webpki-roots` or `rustls-tls-native-roots` features** | reqwest CHANGELOG `# v0.13.0` + `reqwest-0.13.5/Cargo.toml` `[features]` | 2026-09-13 | "`rustls-tls` has been renamed to `rustls`." · "rustls roots features removed, `rustls-platform-verifier` is used by default. To use different roots, call `tls_certs_only(your_roots)`." The 0.13.5 feature table lists `rustls` and `rustls-no-provider` only; the `rustls-tls*` names exist in `reqwest-0.12.28/Cargo.toml` and nowhere in 0.13.5 | **contradicted (Phase 1 brief assumed the 0.12 names)** | — |
| rustls is now the default backend and defaults to aws-lc | CHANGELOG v0.13.0 + manifest | 2026-09-13 | "`rustls` is now the default TLS backend, instead of `native-tls`." · "`rustls` crypto provider defaults to aws-lc instead of _ring_. (`rustls-no-provider` exists if you want a different crypto provider)". Manifest: `default = ["default-tls","charset","http2","system-proxy"]`, `default-tls = ["rustls"]`, `rustls = ["__rustls-aws-lc-rs","dep:rustls-platform-verifier","__rustls"]`, `__rustls-aws-lc-rs = ["hyper-rustls?/aws-lc-rs","tokio-rustls?/aws-lc-rs","rustls?/aws-lc-rs","quinn?/rustls-aws-lc-rs"]` | **verified** | — |
| **`rustls-no-provider` is the aws-lc-free feature, and it panics unless a provider is installed** | manifest + `src/async_impl/client.rs` | 2026-09-13 | `rustls-no-provider = ["dep:rustls-platform-verifier","__rustls"]`. Client build: "let provider = rustls::crypto::CryptoProvider::get_default().map(\|arc\| arc.clone()).unwrap_or_else(default_rustls_crypto_provider);" and, under `#[cfg(not(feature = "__rustls-aws-lc-rs"))]`: `panic!("No rustls crypto provider is configured. When using the `rustls-no-provider` feature you must install a crypto provider before building a Client. …")` | **verified** | `features = ["rustls-no-provider", "http2"]`, `default-features = false` |
| reqwest's own rustls-stack dependencies are `default-features = false`, so a direct `rustls` dependency selects the provider | manifest | 2026-09-13 | `[…dependencies.rustls] version = "0.23.4"` · `features = ["std","tls12"]` · `optional = true` · `default-features = false`; `hyper-rustls` `version = "0.27.0"`, `features = ["http1","tls12"]`, `default-features = false`; `tokio-rustls` `version = "0.26"`, `default-features = false`. rustls 0.23.44 itself: `default = ["aws_lc_rs","logging","prefer-post-quantum","std","tls12"]` — so the direct dependency must also be `default-features = false` | **verified** | `rustls = { version = "0.23.44", default-features = false, features = ["ring","std","tls12","logging"] }` |
| **That pair yields a ring-only graph: no aws-lc anywhere** | executed (scratch probe) | 2026-09-13 | `cargo tree -i ring` → `ring v0.17.14` ← `rustls v0.23.44` ← reqwest, hyper-rustls, tokio-rustls, rustls-platform-verifier; `cargo tree -i aws-lc-sys` and `-i aws-lc-rs` → "did not match any packages" (all targets and `--target x86_64-unknown-linux-gnu`) | **verified (executed)** | — |
| `http2` is a separate, default-on feature | manifest | 2026-09-13 | `http2 = ["dep:h2","hyper/http2","hyper-util/http2","hyper-rustls?/http2"]`; with `default-features = false` it must be named explicitly | **verified** | — |
| Root store: `rustls-platform-verifier` is mandatory; on Linux it reads native certs | manifest + `rustls-platform-verifier-0.7.0/Cargo.toml` | 2026-09-13 | Both `rustls` and `rustls-no-provider` carry `dep:rustls-platform-verifier` (`version = ">=0.6.0, <0.8.0"`). Verifier manifest: `[target.'cfg(all(unix, not(target_os = "android"), not(target_vendor = "apple"), not(target_arch = "wasm32")))'.dependencies.rustls-native-certs]`. Override exists: `pub fn tls_certs_only(mut self, certs: impl IntoIterator<Item = Certificate>) -> ClientBuilder` and `add_root_certificate` | **verified** | — |
| `ClientBuilder::timeout` · `read_timeout` · `connect_timeout` | `src/async_impl/client.rs` | 2026-09-13 | `pub fn timeout(mut self, timeout: Duration) -> ClientBuilder` — "Enables a total request timeout. The timeout is applied from when the request starts connecting until the response body has finished." `pub fn read_timeout(…)` — "The timeout applies to each read operation, and resets after a successful read." `pub fn connect_timeout(…)` — "Set a timeout for only the connect phase of a `Client`. Default is `None`." | **verified** | — |
| `redirect::Policy::custom` and the `Attempt` API | `src/redirect.rs` | 2026-09-13 | `pub fn custom<T>(policy: T) -> Self where T: Fn(Attempt) -> Action + Send + Sync + 'static`; `Attempt::{status, url, previous, follow, stop, error}`. "The default `Policy` handles a maximum loop chain, but the custom variant does not do that for you automatically. The custom policy should have some way of handling those." | **verified** | — |
| `Response::chunk` for bounded streaming | `src/async_impl/response.rs` | 2026-09-13 | `pub async fn chunk(&mut self) -> crate::Result<Option<Bytes>>` — "When the response body has been exhausted, this will return `None`." Also `content_length(&self) -> Option<u64>`, `bytes_stream`, `error_for_status` | **verified** | — |
| `use_rustls_tls()` is soft-deprecated | `src/async_impl/client.rs` | 2026-09-13 | "Deprecated: use [`ClientBuilder::tls_backend_rustls()`] instead." CHANGELOG: "previous name left in place with a "soft" deprecation. (just documented, no warnings)" | **verified** | use `tls_backend_rustls()` |
| **`webpki-root-certs` is `CDLA-Permissive-2.0` and fails `cargo deny check licenses` even though it is wasm32-only** | `webpki-root-certs-1.0.9/Cargo.toml:26` + executed | 2026-09-13 | `license = "CDLA-Permissive-2.0"`. Verifier manifest: `[target.'cfg(target_arch = "wasm32")'.dependencies.webpki-root-certs]`. `cargo tree -i webpki-root-certs` → "nothing to print" on the host, but `cargo deny check licenses` with the repo's `deny.toml` (no `[graph] targets`) → `error[rejected]: failed to satisfy license requirements … webpki-root-certs v1.0.9 └── rustls-platform-verifier v0.7.0 └── reqwest v0.13.5`. With `[graph] targets = ["x86_64-unknown-linux-gnu","aarch64-unknown-linux-gnu","aarch64-apple-darwin"]` the licence check passes | **verified (executed) — decision needed** | — |
| `webpki-roots` (the 0.12-era root crate) is not in the graph at all | crates.io API + executed | 2026-09-13 | crates.io: `webpki-roots 1.0.9`, `license = "CDLA-Permissive-2.0"`. `cargo tree -i webpki-roots` → no match in either probe | **verified (absent)** | — |
| `ring` licence is allowlisted | `ring-0.17.14/Cargo.toml:160` + executed | 2026-09-13 | `license = "Apache-2.0 AND ISC"`; both identifiers are in `deny.toml`; cargo-deny raises no error for ring | **verified (executed)** | — |
| `aws-lc-sys` licence expression, for the record | crates.io API (0.45.0) | 2026-09-13 | `"ISC AND (Apache-2.0 OR ISC) AND Apache-2.0 AND MIT AND BSD-3-Clause AND (Apache-2.0 OR ISC OR MIT) AND (Apache-2.0 OR ISC OR MIT-0)"`. Not in the pinned graph; recorded so a future `rustls` (not `-no-provider`) feature flip is recognisable | **verified** | — |
| Remaining rustls-stack licences | manifests | 2026-09-13 | rustls 0.23.44 `Apache-2.0 OR ISC OR MIT`; rustls-webpki 0.103.15 `ISC`; rustls-native-certs 0.8.4 `Apache-2.0 OR ISC OR MIT`; rustls-platform-verifier 0.7.0 `MIT OR Apache-2.0`; rustls-pki-types 1.15.1 `MIT OR Apache-2.0`; hyper-rustls 0.27.9 `Apache-2.0 OR ISC OR MIT` | **verified** | — |
| **reqwest 0.13.5 introduces a `base64` duplicate that `deny.toml` does not skip** | manifests + executed | 2026-09-13 | `reqwest-0.13.5/Cargo.toml`: `[dependencies.base64] version = "0.23"`; `hyper-util-0.1.20/Cargo.toml`: `[dependencies.base64] version = "0.22"` · `optional = true` (0.1.20 is hyper-util's newest, 2026-02-02). Repo already has `base64 0.23.1` via `arrow-cast 59.3.0`. `cargo deny check bans` → `error[duplicate]: found 2 duplicate entries for crate 'base64'` (0.22.1 ← hyper-util ← hyper-rustls, reqwest; 0.23.1 ← reqwest, arrow-cast) | **verified (executed) — needs a reasoned `skip`** | — |
| tar current release and API | crates.io API + `tar-0.4.46/src` | 2026-09-13 | `0.4.46` (2026-05-18), `MIT OR Apache-2.0`, `rust-version = "1.63"`. `pub fn entries(&mut self) -> io::Result<Entries<'_, R>>`; `pub fn path(&self) -> io::Result<Cow<'_, Path>>`; `pub fn link_name(&self) -> io::Result<Option<Cow<'_, Path>>>`; `pub fn unpack_in<P: AsRef<Path>>(&mut self, dst: P) -> io::Result<bool>` — "Extracts this file under the specified path, avoiding security issues."; `Header::entry_type(&self) -> EntryType` | **verified** | `tar = "0.4.46"` |
| tar `EntryType` distinguishes symlinks, hardlinks and devices before any write | `tar-0.4.46/src/entry_type.rs` | 2026-09-13 | `pub enum EntryType { Regular, Link, Symlink, Char, Block, Directory, Fifo, Continuous, GNULongName, GNULongLink, GNUSparse, XGlobalHeader, XHeader, #[doc(hidden)] __Nonexhaustive(u8) }` with doc comments "Hard link", "Symbolic link", "Character device", "Block device", "Named pipe (fifo)". Predicates: `is_symlink`, `is_hard_link`, `is_character_special`, `is_block_special`, `is_fifo`, `is_dir`, `is_file`, `is_pax_global_extensions`, `is_pax_local_extensions`, `is_gnu_longname`, `is_gnu_longlink`, `is_gnu_sparse`. The enum is non-exhaustive: match with a wildcard arm that rejects | **verified** | — |
| tar's own security scope is best-effort and excludes TOCTOU | `tar-0.4.46/src/lib.rs` `# Security` | 2026-09-13 | "a best-effort is made to prevent writing files outside the destination directory: paths containing `..` are rejected, and symlink targets within the archive are validated before use." · "**Concurrent mutation of the destination tree is outside the threat model.**" | **verified** | — |
| flate2 current release; default backend is pure-Rust miniz_oxide | crates.io API + `flate2-1.1.10/Cargo.toml` | 2026-09-13 | `1.1.10` (2026-08-28), `rust-version = "1.67.0"`. `default = ["rust_backend","runtime_detection"]`, `rust_backend = ["miniz_oxide","any_impl"]`, `miniz_oxide = ["any_impl","dep:miniz_oxide","dep:crc32fast"]`; `[dependencies.miniz_oxide] version = "0.9.0"` · `features = ["simd"]` · `optional = true`. Already in the repo lock at 1.1.10 with `miniz_oxide 0.9.1` (`MIT OR Zlib OR Apache-2.0`) | **verified** | `flate2 = "1.1.10"` |
| flate2 `GzDecoder` over any `Read` | `flate2-1.1.10/src/gz/read.rs` | 2026-09-13 | `impl<R: Read> GzDecoder<R> { pub fn new(r: R) -> GzDecoder<R>` — "Creates a new decoder from the given reader, immediately parsing the gzip header."; `impl<R: Read> Read for GzDecoder<R>`. A `MultiGzDecoder<R>` also exists for multi-member streams | **verified** | — |
| zstd current release; `stream::read::Decoder` is a `Read` | crates.io API + `zstd-0.14.0/src/stream/read/mod.rs` | 2026-09-13 | `0.14.0` (2026-09-04), `BSD-3-Clause`, `rust-version = "1.64"`, depends on `zstd-safe 8.0.0` and `zstd-sys 2.1.0+zstd.1.5.7`. `impl<R: Read> Decoder<'static, BufReader<R>> { pub fn new(reader: R) -> io::Result<Self>`; `impl<'a, R: BufRead> Decoder<'a, R>` with `with_buffer`, `single_frame`; `impl<R: BufRead> Read for Decoder<'_, R>`. Bounded reading is therefore `std::io::Read::take` on the decoder (bounds decompressed bytes) or on the inner reader (bounds compressed bytes) — a std guarantee, nothing crate-specific | **verified** | `zstd = "0.14.0"` |
| **Both zstd majors are already in the repo graph; 0.14.0 adds no crate** | executed (repo) | 2026-09-13 | `cargo tree -i zstd` → "ambiguous: zstd@0.13.3, zstd@0.14.0". `zstd 0.13.3` (zstd-safe 7.3.0) ← `arrow-ipc 59.3.0` and `parquet 59.3.0`; `zstd 0.14.0` (zstd-safe 8.0.0) ← `compression-codecs 0.4.41` ← `async-compression 0.4.46` ← `datafusion-datasource 55.1.0`. `deny.toml` already skips `zstd` and `zstd-safe` | **verified (executed)** | 0.14.0, not the 0.13.3 parquet selects |
| semver current release and API | crates.io API + `semver-1.0.28/src/lib.rs` | 2026-09-13 | `1.0.28` (2026-04-04), `rust-version = "1.68"`. `pub fn parse(text: &str) -> Result<Self, Error>` — "Create `Version` by parsing from string representation."; `pub fn cmp_precedence(&self, other: &Self) -> Ordering` — doc example: "The three 1.20.0 versions differ only in build metadata so they are not reordered relative to one another."; `Prerelease::EMPTY` and `pub fn is_empty(&self) -> bool`. Already in the repo lock at 1.0.28 | **verified** | `semver = "1.0.28"` |
| sha2 current release; one-shot and streaming digests | crates.io API + `sha2-0.11.0/src/lib.rs` + `digest-0.11.3/src/digest.rs` | 2026-09-13 | `0.11.0` (2026-03-25), `rust-version = "1.85"`, `[dependencies.digest] version = "0.11"`. `pub struct Sha256(CtOutWrapper<block_api::Sha256VarCore, U32>)`; `pub use digest::{self, Digest};`. `pub trait Digest`: `fn digest(data: impl AsRef<[u8]>) -> Output<Self>`, `fn update(&mut self, data: impl AsRef<[u8]>)`, `fn finalize(self) -> Output<Self>`; low-level `pub trait Update { fn update(&mut self, data: &[u8]); }`. Already in the repo lock at 0.11.0 via `datafusion-functions`; the `digest 0.10.7` duplicate (via `blake2`) is already skipped | **verified** | `sha2 = "0.11.0"` |
| url current release; `Url::parse`, `host()`, `Host` variants | crates.io API + `url-2.5.8/src` | 2026-09-13 | `2.5.8` (2026-01-05), `rust-version = "1.63"`. `pub fn parse(input: &str) -> Result<Self, crate::ParseError>`; `pub fn host(&self) -> Option<Host<&str>>` — "Return the parsed representation of the host for this URL."; `pub enum Host<S = String> { Domain(S), Ipv4(Ipv4Addr), Ipv6(Ipv6Addr) }`; also `host_str`, `domain`, `port_or_known_default`. Already in the repo lock at 2.5.8 | **verified** | `url = "2.5.8"` |
| **`public-api 0.52.2` pins `rustdoc-types 0.59.0`; newest is 0.61.0** | `public-api-0.52.2/Cargo.toml` + crates.io API + `rustdoc-types-*/src/lib.rs` | 2026-09-13 | `[dependencies.rustdoc-types] version = "0.59.0"`. crates.io: `rustdoc-types newest_version = "0.61.0"` (2026-07-29). `pub const FORMAT_VERSION: u32 = 59` in 0.59.0, `60` in 0.60.0, `61` in 0.61.0. Repo lock already holds `rustdoc-types 0.59.0` (dev-dep). Declaring 0.61.0 anywhere in the workspace would produce a second `rustdoc_types::Id` type and break interop with `public-api` | **verified** | `rustdoc-types = "0.59.0"` |
| Scratch probe, new pins only | executed | 2026-09-13 | `cargo +1.98.1 tree -e no-dev --prefix none \| sort -u \| wc -l` → **154** (same on `--target x86_64-unknown-linux-gnu`). `cargo tree -d` → `base64` 0.22.1 and 0.23.1, `syn` 2.0.119 and 3.0.5 (already skipped). `cargo deny check licenses bans` → 2 errors: `webpki-root-certs` licence, `base64` duplicate | **verified (executed)** | — |
| Scratch probe, new pins + the repo's `Cargo.lock` and workspace deps | executed | 2026-09-13 | Lock grows 317 → 361 packages (47 added, none of the repo's existing versions moved). No-dev unique crates: **421**. `cargo deny check licenses bans` → exactly the same 2 errors; every other duplicate (`zstd`, `zstd-safe`, `syn`, `digest`, `hashbrown`, `getrandom`, `itertools`, `num-bigint`, `foldhash`, `block-buffer`, `crypto-common`, `windows-sys`) is already covered by the existing skip list | **verified (executed)** | — |

Consequences of this round:

- **The Phase 1 brief's reqwest feature names are 0.12-era.** In 0.13.5 the aws-lc-free
  configuration is `default-features = false, features = ["rustls-no-provider", "http2"]` plus
  a direct `rustls = { version = "0.23.44", default-features = false, features = ["ring", "std",
  "tls12", "logging"] }`, and the process must install the ring provider
  (`rustls::crypto::ring::default_provider().install_default()`) before the first
  `Client::builder().build()` or reqwest panics by design. Prefer `tls_backend_rustls()` over the
  soft-deprecated `use_rustls_tls()`.
- **Two `deny.toml` decisions are required before the pin lands**, both measured, neither
  silent: (1) `webpki-root-certs` (CDLA-Permissive-2.0) is reachable only under
  `cfg(target_arch = "wasm32")` but cargo-deny evaluates every target unless `[graph] targets`
  is set — either restrict `targets` to the platforms we ship or allow the identifier;
  (2) `base64 0.22` (hyper-util 0.1.20, its newest) alongside `base64 0.23` (reqwest, arrow-cast)
  needs a reasoned `skip` entry, since base64 never appears in a public type.
- `flate2`, `semver`, `sha2`, `url`, `zstd 0.14.0`, `rustdoc-types 0.59.0` and `miniz_oxide` are
  already in the lock at exactly the recommended versions, so promoting them to direct
  dependencies adds no crate. `tar` and `reqwest` are new.

### crates.io registry and docs.rs build status (measured)

| Claim | Source | Retrieved | Quote or evidence | Verdict |
|---|---|---|---|---|
| Sparse index URL and `config.json` | Cargo reference `registry-index.html` + executed | 2026-09-13 | "the sparse index URL for crates.io is `sparse+https://index.crates.io/`." · "The root of the index contains a file named `config.json`". Measured `GET https://index.crates.io/config.json` → `{"dl": "https://static.crates.io/crates", "api": "https://crates.io"}` | **verified (executed)** |
| **Index path prefix rule** | Cargo reference + executed | 2026-09-13 | "Packages with 1 character names are placed in a directory named `1`. Packages with 2 character names are placed in a directory named `2`. Packages with 3 character names are placed in the directory `3/{first-character}` … All other packages are stored in directories named `{first-two}/{second-two}` … For example, `cargo` would be stored in a file named `ca/rg/cargo`." Also: "the filename is the name of the package in lowercase". Measured `200` for `1/a`, `2/io`, `3/s/syn`, `se/rd/serde`, `re/qw/reqwest` | **verified (executed)** |
| **Index line fields** | Cargo reference + executed | 2026-09-13 | "`name`: The name of the package… `vers`: The version of the package this row is describing… `deps`: Array of direct dependencies… `cksum`: A SHA256 checksum of the `.crate` file. `features`: Set of features defined for the package… Since Cargo 1.84, defaults to `{}` if not specified. `features2`: This optional field contains features with new, extended syntax… `yanked`: Boolean of whether or not this version has been yanked. `links`: … optional and defaults to null. `v`: An unsigned 32-bit integer value indicating the schema version of this entry. If not specified, it should be interpreted as the default of 1. `rust_version`: The minimal supported Rust version (optional)… `pubtime`: The publish time of this package version (optional). Format: `yyyy-mm-ddThh:mm:ssZ`". Measured last line of `se/rd/serde` (vers 1.0.229): keys `cksum, deps, features, name, pubtime, rust_version, vers, yanked` — `features2`, `links`, `v` absent, so the parser must apply the documented defaults. Dep keys: `default_features, features, kind, name, optional, req, target`. Response: `content-type: text/plain`, `etag`, `last-modified`, `cache-control: public,max-age=600` | **verified (executed)** |
| **crates.io data-access policy: order of preference and rate guidance** | `crates.io/data-access` (source `svelte/src/routes/data-access/+page.svelte` @ rust-lang/crates.io `e6ee42ed5e83`, 2026-09-13; the live page is client-rendered) | 2026-09-13 | "crates.io provides several ways of accessing crate data and metadata… Please try them in the order below." Index: "No rate limits are required to use data from the sparse crate index." Content: "Crates can be downloaded directly from the crates.io CDN… `https://static.crates.io/crates/rand/rand-0.10.1.crate`. No rate limits apply to static.crates.io at present." API: "Should you be unable to use one of the previous options, you are welcome to use the crates.io API provided you abide by the following limits: A maximum of 1 request per second, and A user-agent header that identifies your application. We strongly suggest providing a way for us to contact you (whether through a repository, or an e-mail address, or whatever is appropriate)" | **verified** |
| **Missing `User-Agent` is rejected with 403 and the policy text** | executed | 2026-09-13 | `curl -H 'User-Agent:' https://crates.io/api/v1/crates/serde` → `403`, body: "We require that all requests include a `User-Agent` header. To allow us to determine the impact your bot has on our service, we ask that your user agent actually identify your bot, and not just report the HTTP client library you're using. Including contact information will also reduce the chance that we will need to take action against your bot. Bad: User-Agent: reqwest/0.9.1 Better: User-Agent: my_crawler Best: User-Agent: my_crawler (my_crawler.com/info) User-Agent: my_crawler (help@my_crawler.com)" | **verified (executed)** |
| API endpoints per the OpenAPI description | `https://crates.io/api/openapi.json` (OpenAPI 3.1.0; data-access calls it "experimental") + executed | 2026-09-13 | `/api/v1/crates/{name}` (GET) — "Gets crate metadata."; `/api/v1/crates/{name}/{version}` (GET) — "Get crate version metadata."; `/api/v1/crates/{name}/{version}/download` (GET) — "Download a crate version. This returns a URL to the location where the crate is stored." Responses: `302` "Successful Response (default)", `200` "Successful Response (for `content-type: application/json`)". Measured crate response top-level keys `categories, crate, keywords, versions`; `crate` carries `max_version`, `max_stable_version`, `newest_version`, `default_version`, `num_versions`, `yanked`; each `versions[]` entry carries `num, checksum, dl_path, yanked, yank_message, license, rust_version, crate_size, created_at, features, edition` | **verified (executed)** |
| **Download redirects to `static.crates.io/crates/{name}/{name}-{version}.crate`** | executed | 2026-09-13 | `HEAD https://crates.io/api/v1/crates/serde/1.0.219/download` → `HTTP/2 302`, `location: https://static.crates.io/crates/serde/serde-1.0.219.crate`. Direct `HEAD` on that URL → `200`, `content-type: application/gzip`, `content-length: 78983` — equal to the version JSON's `crate_size: 78983`; version JSON `checksum` = `5f0e2c6ed6606019b4e29e69dbaba95b11854410e5347d525002456dbbb786b6` (sha256, same field the index calls `cksum`) | **verified (executed)** |
| **docs.rs documents no build or status JSON API** | `https://docs.rs/about` + `https://docs.rs/about/badges` | 2026-09-13 | `/about` links only Badges, Builds, Metadata, Shorthand URLs, Download, Rustdoc JSON, Build queue; the About page says "Summaries of the documentation build processes are available at `/releases/`." The badges page body in full: "Docs.rs no longer has its own badges. Consider using shields.io instead." No `status.json`, `builds.json` or API is mentioned on any `/about/*` page | **verified (absence)** |
| **`builds.json` does not exist; `status.json` exists but reports HTML rustdoc status, not JSON availability** | executed (operator 2026-09-13T21:07Z; re-measured 2026-09-13T21:25Z) | 2026-09-13 | `/crate/serde/1.0.219/builds.json` → `404 text/html`; `/crate/serde/1.0.219/status.json` → `200 application/json` `{"doc_status":true,"version":"1.0.219"}`; `/crate/serde/1.0.100/status.json` → `200` `{"doc_status":true,"version":"1.0.100"}` while `/crate/serde/1.0.100/json` → `404`; `/crate/serde/1.0.219/builds` → `200 text/html`; `/crate/serde/1.0.219/status` → `404` | **verified (executed)** |
| Source of `status.json` confirms its meaning | rust-lang/docs.rs `main` @ `830d2508e921`, `crates/bin/docs_rs_web/src/routes.rs` + `handlers/build_status.rs` + `match_release.rs` | 2026-09-13 | Route: `"/crate/{name}/{version}/status.json", get_internal(build_status::status_handler)`. Handler: `let rustdoc_status = matched_release.rustdoc_status();` … `serde_json::json!({ "version": version.to_string(), "doc_status": rustdoc_status, })`, served with `ACCESS_CONTROL_ALLOW_ORIGIN, "*"` and `CachePolicy::NoStoreMustRevalidate`. `pub(crate) fn rustdoc_status(&self) -> bool { self.release.rustdoc_status.unwrap_or(false) }`. Undocumented, `pub(crate)`, and unrelated to rustdoc JSON | **verified** |

Consequences: the Rust registry client should walk the documented order — sparse index for
metadata (no rate limit, cacheable by `etag`), `static.crates.io` for `.crate` files (no rate
limit, sha256 checked against the index `cksum`), and `/api/v1` only for what neither carries
(e.g. `license`, `crate_size`, `published_by`), at no more than 1 request per second and always
with a `User-Agent` that names the project and a contact. For docs.rs, "does this version have
JSON?" is answerable only by requesting `/crate/{name}/{version}/json` and reading the status;
`status.json` must not be used as a proxy — serde 1.0.100 reports `doc_status: true` and has no
JSON.
