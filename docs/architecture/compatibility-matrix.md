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

## Phase 2 Python version parsing and wheel archives — 2026-09-13

Context7 discovery resolved ZIP to `/zip-rs/zip2`; both underscore and hyphen searches for
`pep440_rs` and `pep508_rs` returned unrelated packages, so their exact release documentation
and published source archives were read directly. Context7 examples are not API authority:
its ZIP example used `name()?`, while the exact 8.6.0 method returns `&str`.

These are source and registry checks, not executed workspace integration checks. No Cargo
manifest, lockfile, dependency-policy exception, or production source was changed by this
verification. The selected candidates still require the affected-crate checks and
`just deps-policy` when admitted. The existing workspace requires Rust 1.98.1, one Arrow
type universe, allowed licenses, and no new duplicate-version exceptions.

| Claim | Source URL | Retrieved | Exact quote or measured evidence | Verdict | Selected pin | Blueprint reference |
|---|---|---|---|---|---|---|
| Current non-yanked stable PEP 440 parser release | [Sparse index](https://index.crates.io/pe/p4/pep440_rs), [published source archive](https://static.crates.io/crates/pep440_rs/pep440_rs-0.7.3.crate) | 2026-09-13 | Latest eligible row: `"vers":"0.7.3"`; archive manifest: `edition = "2021"`, `license = "Apache-2.0 OR BSD-2-Clause"`. Neither manifest nor index declares a minimum Rust version. | verified | `pep440_rs = "=0.7.3"` | §3.2, §5.1, §13 Phase 2 |
| `Version` parses strings and supports version ordering | [Version 0.7.3](https://docs.rs/pep440_rs/0.7.3/pep440_rs/struct.Version.html), published `src/version.rs` | 2026-09-13 | `impl FromStr for Version`; `type Err = VersionParseError;`; `fn from_str(version: &str) -> Result<Self, Self::Err>`. `Ord`, `Eq`, and `any_prerelease()` are public. | verified | same | §3.2, §5.1 |
| Specifiers use PEP 440 membership, not ordinary version comparisons | [VersionSpecifiers 0.7.3](https://docs.rs/pep440_rs/0.7.3/pep440_rs/struct.VersionSpecifiers.html), published `src/version_specifier.rs` | 2026-09-13 | `impl FromStr for VersionSpecifiers`; `type Err = VersionSpecifiersParseError;`; `pub fn contains(&self, version: &Version) -> bool`. The implementation requires every contained specifier to match. | verified | same | §3.2, §5.1 |
| Membership alone chooses eligible prereleases | [Crate documentation 0.7.3](https://docs.rs/pep440_rs/0.7.3/pep440_rs/) | 2026-09-13 | "we can't say whether a specifier matches without also looking at the environment". Release-selection policy must separately apply the declared prerelease/yanked policy and preserve the selected artifact identity. | contradicted | — | §3.2 |
| PEP 440 parser adds no evident duplicate-version requirement | [Published manifest](https://static.crates.io/crates/pep440_rs/pep440_rs-0.7.3.crate), [unscanny manifest](https://static.crates.io/crates/unscanny/unscanny-0.1.0.crate), current `Cargo.lock` | 2026-09-13 | Required `once_cell`, `serde`, and `unicode-width` ranges accept the current lock. The only new required subtree is `unscanny 0.1.0`, which has no normal dependencies and uses `MIT OR Apache-2.0`. Optional features remain disabled. | verified | same | §8.4 |
| Published PEP 440 MSRV and workspace execution are established | Same exact manifest/index and local inspection | 2026-09-13 | No `rust-version` field exists; no build or dependency-policy command was executed with this candidate. Its precise upstream MSRV and current-tree compilation therefore remain unverified. | unverified | same candidate, integration pending | §13 Phase 2 |
| Current PEP 508 parser supplies typed requirements and marker evaluation | [Requirement documentation](https://docs.rs/pep508_rs/0.9.2/pep508_rs/), [published archive](https://static.crates.io/crates/pep508_rs/pep508_rs-0.9.2.crate), [sparse index](https://index.crates.io/pe/p5/pep508_rs) | 2026-09-13 | Latest non-yanked stable: `0.9.2`. Public `Requirement<VerbatimUrl>::from_str`, `VersionOrUrl`, and `evaluate_markers(&self, env: &MarkerEnvironment, extras: &[ExtraName]) -> bool`. License: `Apache-2.0 OR BSD-2-Clause`; no declared MSRV. | verified | not selected | §5.1–§5.3 |
| `pep508_rs 0.9.2` fits the unchanged duplicate-version policy | [Published Cargo.toml](https://docs.rs/crate/pep508_rs/0.9.2/source/Cargo.toml), current `Cargo.lock` and `deny.toml` | 2026-09-13 | `[dependencies.thiserror] version = "1.0.59"` is mandatory; the workspace has `thiserror 2.0.20`, with no exception. Optional `schemars = "0.8.21"` would also conflict with workspace 1.2.2. | contradicted | not selected | §8.4 |
| Current ZIP release and MSRV | [Sparse index](https://index.crates.io/3/z/zip), [published manifest](https://docs.rs/crate/zip/8.6.0/source/Cargo.toml) | 2026-09-13 | `version = "8.6.0"`; `rust-version = "1.88"`; `license = "MIT"`. Latest eligible stable, non-yanked; declared MSRV is below workspace 1.98.1. | verified | `zip = { version = "=8.6.0", default-features = false, features = ["deflate-flate2"] }` | §5.1, §10 |
| Minimal ZIP feature selection reuses the existing deflate backend | [Published manifest](https://docs.rs/crate/zip/8.6.0/source/Cargo.toml), [typed-path archive](https://static.crates.io/crates/typed-path/typed-path-0.12.3.crate), current workspace | 2026-09-13 | `deflate-flate2 = ["_deflate-any", "dep:flate2"]`. ZIP disables flate2 defaults; the core's existing direct flate2 dependency enables its backend. Other required ranges accept existing crc32fast/indexmap/memchr; new typed-path 0.12.3 has no normal dependencies, MSRV 1.65.0, and an allowed license. | verified | same | §8.4, §10 |
| ZIP indexed reading requires `Read + Seek` | [Published ZIP source archive](https://static.crates.io/crates/zip/zip-8.6.0.crate), `src/read/zip_archive.rs` | 2026-09-13 | `impl<R: Read + Seek> ZipArchive<R>`; `pub fn new(reader: R) -> ZipResult<ZipArchive<R>>`; `pub fn by_index(&mut self, file_number: usize) -> ZipResult<ZipFile<'_, R>>`. | verified | same | §5.1, §10 |
| ZIP entries expose the required path, mode, and size evidence | [ZipFile 8.6.0](https://docs.rs/zip/8.6.0/zip/read/struct.ZipFile.html) | 2026-09-13 | `enclosed_name(&self) -> Option<PathBuf>`; `unix_mode(&self) -> Option<u32>`; `size(&self) -> u64`; `compressed_size(&self) -> u64`; `is_symlink(&self) -> bool`; `encrypted(&self) -> bool`. Entries implement `Read`. | verified | same | §10, C11 |
| Indexed iteration preserves all original duplicate archive names | [Published ZIP source archive](https://static.crates.io/crates/zip/zip-8.6.0.crate), `src/read/zip_archive.rs:49–51` | 2026-09-13 | `index_map.insert(file.file_name_raw.clone(), file);` replaces an earlier entry with the same raw name before indexed iteration. `len()` and `by_index()` cannot establish original central-directory uniqueness or entry count. | contradicted | same, requires inventory preflight | §10, C11 |

Implementation consequences:

- Use `Version::from_str` and `VersionSpecifiers::from_str` with `std::str::FromStr` in scope;
  propagate parse errors. Use `specifiers.contains(&version)` for `Requires-Python`/version
  matching, and `Ord` only for ranking candidates after eligibility policy is applied.
- Do not admit `pep508_rs` or widen dependency-policy exceptions. Preserve `Requires-Dist`
  metadata verbatim in Phase 2; semantic dependency resolution remains a separate Phase 4
  capsule producer. These candidate limitations do not revise a binding blueprint decision.
- Do not use `ZipArchive::extract` as the safety policy. Preflight the bounded raw central
  directory when enforcing duplicate rejection and original entry counts; then validate each
  enclosed path, collision, mode, compression method, encryption flag, and declared size.
  `enclosed_name` permits contained parent components, so apply the core's stricter path policy
  separately. `is_file` merely excludes directories and symlinks; reject other Unix file types
  using mode bits. Enforce actual decompressed-byte limits while reading, not only header sizes.
- Retain the existing direct flate2 backend dependency when using ZIP's `deflate-flate2`
  feature. Do not enable ZIP's broad default compression/encryption set or the unrelated
  parser optional features. Run the integrated build and dependency-policy checks before
  reporting either admitted candidate as operationally compatible.

## Phase 2 registry, inventory, and static worker contracts — 2026-09-13

Context7 discovery used `/pypi/warehouse`, `/websites/sphinx-doc_en_master`, and
`/mkdocstrings/griffe`. The primary sources below establish the actual contracts. Griffe
`models.py`, `encoders.py`, and `agents/visitor.py` from tag 2.3.0 matched the installed files
byte-for-byte. Small in-memory visitor probes exercised these APIs without importing analyzed
code, writing fixtures, or running a project gate; they are not P02/P03/P04 acceptance evidence.

| Claim | Source URL | Retrieved | Exact quote or measured evidence | Verdict | Selected pin | Blueprint reference |
|---|---|---|---|---|---|---|
| Release JSON carries artifact records separately from release metadata | [PyPI API](https://docs.pypi.org/api/json/), [Griffe release response](https://pypi.org/pypi/griffe/2.3.0/json), [sampleproject release response](https://pypi.org/pypi/sampleproject/4.0.0/json) | 2026-09-13 | Measured top-level keys: `info`, `last_serial`, `ownership`, `urls`, `vulnerabilities`. Artifact keys include `filename`, `url`, `packagetype`, `size`, `digests.sha256`, `requires_python`, `python_version`, `yanked`, `yanked_reason`, and `core-metadata`. | verified | protocol, no dependency | §5.1–§5.2 |
| Artifact Core Metadata availability is a union, not a Boolean alone | [PyPI API](https://docs.pypi.org/api/json/), [Griffe release response](https://pypi.org/pypi/griffe/2.3.0/json) | 2026-09-13 | Live wheel `core-metadata` is a digest object; live sdist value is `false`. Release `info.requires_dist` and `info.provides_extra` are arrays when present; `info.dynamic` was null in both measured releases. | verified | — | §5.1–§5.2 |
| Release JSON metadata is guaranteed to match every artifact | [PyPI API](https://docs.pypi.org/api/json/) | 2026-09-13 | "The first uploaded data for a release is stored, subsequent uploads do not update it." Preserve selected-wheel METADATA independently and surface disagreements. Project-level `releases` is deprecated; use the Index API for enumeration. | contradicted | — | §3.2, §5.1 |
| Python version eligibility and wheel tags express different constraints | [Core Metadata](https://packaging.python.org/en/latest/specifications/core-metadata/), [compatibility tags](https://packaging.python.org/en/latest/specifications/platform-compatibility-tags/) | 2026-09-13 | Requires-Python: "This field cannot be followed by an environment marker." Tags contain interpreter, ABI, and platform; a `py3-none-any` tag does not replace the artifact's Python version constraint. | verified | — | §5.1 |
| Wheel filename tags may encode multiple supported triples | [Wheel specification](https://packaging.python.org/en/latest/specifications/binary-distribution-format/), [compressed tags](https://packaging.python.org/en/latest/specifications/platform-compatibility-tags/#compressed-tag-sets) | 2026-09-13 | `{distribution}-{version}(-{build tag})?-{python tag}-{abi tag}-{platform tag}.whl`; each dot-separated tag component expands by Cartesian product. Rank matching triples against the declared target's supported tags, not the worker interpreter. | verified | — | §3.1, §5.1 |
| Importable wheel contents are not restricted to archive-root packages | [Wheel specification](https://packaging.python.org/en/latest/specifications/binary-distribution-format/) | 2026-09-13 | `Root-Is-Purelib`, repeated `Tag`, and `.data/(purelib\|platlib\|headers\|scripts\|data)` describe placement. Static import-root inventory must include the root and `.data/purelib`/`.data/platlib` mappings without executing installation or treating scripts/data as import roots. | verified | — | §5.1–§5.2 |
| Distribution names are not sufficient import-root evidence | [Core Metadata](https://packaging.python.org/en/latest/specifications/core-metadata/#import-name-multiple-use) | 2026-09-13 | `Import-Name` and `Import-Namespace` were added in metadata 2.5. The former may explicitly be empty; the latter may not. Missing fields mean unavailable metadata. Retain these declarations when present and compare them with selected artifact structure. | verified | — | §5.2, P01 |
| Native namespace packages can span distributions and lack initialization modules | [PEP 420](https://peps.python.org/pep-0420/) | 2026-09-13 | "Namespace packages have no `__init__.py` module." Portions may occupy different directories. One wheel establishes its own namespace contribution, not completeness of the namespace. | verified | — | §5.2–§5.3, P05 |
| Stub-only and partial-stub discovery depends on package layout | [Typing distribution specification](https://typing.python.org/en/latest/spec/distributing.html) | 2026-09-13 | `foopkg-stubs` is the package-directory convention, not a required PyPI distribution name. Stub-only packages need no marker; partial packages require `partial\n` in `py.typed`. Namespace stub roots omit `__init__.pyi`; regular descendants may contain it. | verified | — | §5.3, P03–P05 |
| A static extractor should implement type-checker stub precedence | [Typing distribution specification](https://typing.python.org/en/latest/spec/distributing.html), blueprint §5.3 | 2026-09-13 | The typing specification assigns source/stub merging and fallback to type checkers. Preserve separate source, stub, marker, provider, and supported-runtime observations; leave semantic precedence to ty in the selected consumer environment. | contradicted | ty remains semantic engine | §5.3 |
| Sphinx v2 inventory has a text header and compressed record body | [Sphinx source at e44a40eb2f810558ccd9da1425421270ccb81351](https://raw.githubusercontent.com/sphinx-doc/sphinx/e44a40eb2f810558ccd9da1425421270ccb81351/sphinx/util/inventory.py) | 2026-09-13 | `# Sphinx inventory version 2`; project and version header lines; zlib declaration; then UTF-8 records containing name, domain:type, signed priority, URI, and display name. Object names and display names may contain spaces. | verified | no Sphinx dependency | §5.2–§5.3 |
| Inventory entries have URI and display-name substitutions | Same exact Sphinx source | 2026-09-13 | `location = location[:-1] + name` applies only to a trailing `$`; display name `-` denotes the object name. Preserve non-Python domains and original inventory rows; malformed or ambiguous rows are coverage gaps. | verified | — | §5.3 |
| Parsing an inventory makes its target URLs safe to fetch | Same exact Sphinx source, [intersphinx behavior](https://www.sphinx-doc.org/en/master/usage/extensions/intersphinx.html) | 2026-09-13 | `location = posixpath.join(uri, location)` performs joining, not authorization. Core policy must bound decompression and records, resolve targets against the recorded docs base, and revalidate scheme/host/path and every fetch redirect. | contradicted | — | §5.3, §10, C11 |
| A live inventory's version header identifies the selected package release | [Live Griffe inventory](https://mkdocstrings.github.io/griffe/objects.inv) | 2026-09-13 | Measured headers: project Griffe, version `0.0.0`; compressed bytes 38,817; decompressed bytes 576,334; 5,812 records; SHA256 `aaa94d0d814e8cfc36360f94feab5b1dede8f54b6c064e144e09a7c3be18c10e`. This does not verify documentation for Griffe 2.3.0. | contradicted | — | §3.2, §5.3 |
| Public `visit` can analyze individual source and stub files separately | [Griffe 2.3.0 visitor](https://raw.githubusercontent.com/mkdocstrings/griffe/2.3.0/packages/griffelib/src/griffe/_internal/agents/visitor.py), installed signature and in-memory probes | 2026-09-13 | `visit(module_name, filepath, code, *, extensions=None, parent=None, docstring_parser=None, docstring_options=None, lines_collection=None, modules_collection=None) -> Module`. `filepath` is a Path. No `allow_inspection` or `search_paths` argument exists on this AST-only function. | verified | existing griffe 2.3.0 | §5.2, P02–P03 |
| Static visit exposes signatures, parameter structure, bases, and aliases | [Griffe 2.3.0 models](https://raw.githubusercontent.com/mkdocstrings/griffe/2.3.0/packages/griffelib/src/griffe/_internal/models.py), installed objects | 2026-09-13 | `Function.signature(*, return_type=True, name=None) -> str`; parameters retain kind, annotation, and default. `Class.bases` retains declared expressions. Unresolved aliases retain `target_path`, `resolved=False`, `alias_lineno`, and `alias_endlineno`. | verified | same | §5.2 |
| Generic Griffe JSON dumps preserve overload evidence | Same exact models and visitor, in-memory probes | 2026-09-13 | Both `Module.as_json(full=False)` and `full=True` omitted overloads. Stub-only overload declarations were absent from `members` and stored in `Module.overloads`; class equivalents use `Class.overloads`; implementation-backed overloads use `Function.overloads`. Emit all three explicitly. | contradicted | same, explicit worker projection | §5.2, P03–P04 |
| Generic serialization preserves alias identity and export certainty | Same exact models, in-memory probes | 2026-09-13 | A resolved `Alias.as_json()` serialized its target as kind `class`, while `Alias.as_dict()` retained kind `alias`. Literal and unresolved `ExprName` exports both become strings in normal dumps. Preserve their distinction before serialization. | contradicted | same, explicit worker projection | §5.2–§5.3 |

Worker contract consequences: visit each `.py` and `.pyi` independently using the Rust-supplied
logical module path, validated artifact path, decoded text, and explicit reviewed extensions.
Preload `LinesCollection[path] = code.splitlines()` when source slicing is required. Emit a
schema-versioned plain-data projection with separate observation provenance, declaration spans,
structured parameters and signature text, declared bases, every overload declaration, alias
identity/local spans, and literal versus unresolved export signals. Do not serialize aliased
targets in place of aliases, classify `runtime=True` on Griffe objects as executed evidence,
or treat a full Griffe dump as a complete worker contract. Unknown relative aliases, syntax or
encoding failures, and uncharacterized compiled modules remain explicit extraction gaps.

## Wheel metadata corroboration — 2026-09-13

Context7 discovery used `/pypa/packaging.python.org`. These are format and evidence checks,
not a claim that a statically inventoried wheel has passed an installer or runtime gate.

| Claim | Source URL | Retrieved | Exact quote or measured evidence | Verdict | Selected pin | Blueprint reference |
|---|---|---|---|---|---|---|
| Internal WHEEL tags describe the expanded filename tags | [Wheel format](https://packaging.python.org/en/latest/specifications/binary-distribution-format/), [compressed tags](https://packaging.python.org/en/latest/specifications/platform-compatibility-tags/#compressed-tag-sets) | 2026-09-13 | "Tag is the wheel’s expanded compatibility tags"; dot-separated filename components expand by Cartesian product. Corroborate the complete sets, separately from target compatibility. | verified | no dependency change | §5.1–§5.2 |
| Wheel format version has a compatibility policy independent of package version | [Wheel format](https://packaging.python.org/en/latest/specifications/binary-distribution-format/) | 2026-09-13 | "Warn if minor version is greater, abort if major version is greater." Preserve a newer-minor warning; do not interpret `Wheel-Version` as distribution `Version` or `Metadata-Version`. | verified | support Wheel 1.0 | §5.1–§5.2 |
| WHEEL and METADATA belong to the same distribution authority | [Wheel contents](https://packaging.python.org/en/latest/specifications/binary-distribution-format/#file-contents) | 2026-09-13 | `.dist-info` contains `METADATA`, `WHEEL`, and `RECORD`; the top-level directory is `{distribution}-{version}.dist-info`. The corresponding `.data` directory uses the same stem. Do not select unrelated nested metadata or entry points. | verified | — | §5.1–§5.2 |
| Package identity comparisons use normalized names and parsed versions | [Name normalization](https://packaging.python.org/en/latest/specifications/name-normalization/), [core metadata](https://packaging.python.org/en/latest/specifications/core-metadata/) | 2026-09-13 | "The name should be lowercased"; runs of `.`, `_`, and `-` collapse to one hyphen. Required metadata fields are `Metadata-Version`, `Name`, and `Version`; version parsing follows the version specification. Preserve original strings alongside comparisons. | verified | existing pep440_rs | §3.1, §5.1 |
| Selected-wheel Requires-Python remains an independent compatibility constraint | [Requires-Python](https://packaging.python.org/en/latest/specifications/core-metadata/#requires-python), [PyPI JSON](https://docs.pypi.org/api/json/) | 2026-09-13 | "This field cannot be followed by an environment marker." The optional internal specifier is not replaced by registry metadata or by wheel tags. Validate it even when the target interpreter is unspecified, retaining both internal and registry observations. | verified | existing pep440_rs | §3.2, §5.1 |
| A real compressed-tag wheel corroborates the standard's representation | [Release JSON](https://pypi.org/pypi/sampleproject/1.2.0/json), [selected artifact](https://files.pythonhosted.org/packages/30/52/547eb3719d0e872bdd6fe3ab60cef92596f95262e925e1943f68f840df88/sampleproject-1.2.0-py2.py3-none-any.whl) | 2026-09-13 | In-memory download length and registry SHA256 matched: `7a7a8b91086deccc54cac8d631e33f6a0e232ce5775c6be3dc44f86c2154019d`. WHEEL 1.0 has `Tag: py2-none-any` and `Tag: py3-none-any`; the matching METADATA names sampleproject 1.2.0 and omits Requires-Python. | verified | observation only | §5.1–§5.2 |

Conservative core validation: identify one top-level `.dist-info` authority and require its
WHEEL and METADATA identity to match the filename and resolved release. Read entry points from
that authority and anchor `.data` placement to its corresponding top-level stem. Reject
malformed required headers and duplicate scalar identity/version/constraint headers as explicit
service policy; allow repeated Tag fields. Bound filename-tag expansion and compare its set
with the expanded WHEEL Tag set. For supported Wheel 1.0, reject unsupported major versions
and preserve newer-minor warnings. Parse internal Requires-Python with `VersionSpecifiers`;
when an interpreter was declared, require both applicable internal and registry constraints
to accept it. Without one, preserve the constraint and report that environment compatibility
is unspecified. Do not require textual equality of semantically equivalent constraints or
versions, silently substitute the worker interpreter, or discard disagreement evidence.

## Generated enum defaults and formatter configuration — 2026-09-13

Context7 discovery used `/koxudaxi/datamodel-code-generator` and `/astral-sh/ruff`. Installed
CLI help reports datamodel-code-generator 0.80.0 and Ruff 0.14.4. The generator's tagged
arguments, parser, formatter, CLI, and deprecation source files matched installed bytes.

| Claim | Source URL | Retrieved | Exact quote or measured evidence | Verdict | Selected pin | Blueprint reference |
|---|---|---|---|---|---|---|
| Enum defaults can be emitted as typed members through the supported CLI | [0.80.0 arguments](https://raw.githubusercontent.com/koxudaxi/datamodel-code-generator/0.80.0/src/datamodel_code_generator/arguments.py), installed `--help` | 2026-09-13 | "Use --deserialize-default-values enum instead." `--set-default-enum-member` still exists but is deprecated; both flags produced `status: Status \| None = Status.two` for a declared default `two`. | verified | existing 0.80.0 | §6.3, C19 |
| Enum conversion preserves the declared default rather than inventing one | [0.80.0 parser](https://raw.githubusercontent.com/koxudaxi/datamodel-code-generator/0.80.0/src/datamodel_code_generator/parser/base.py), in-memory CLI probe | 2026-09-13 | "Convert matching defaults to enum members while preserving unmatched list values." A field without a schema default stayed `Missing \| None = None`; an explicit second member became `Status.two`. Context7's first-member-default summary is not authoritative. | verified | same | §6.3, C19 |
| Integrated Ruff formatting automatically uses the invoking repository's configuration for temporary outputs | [0.80.0 formatter](https://raw.githubusercontent.com/koxudaxi/datamodel-code-generator/0.80.0/src/datamodel_code_generator/format.py), [0.80.0 CLI](https://raw.githubusercontent.com/koxudaxi/datamodel-code-generator/0.80.0/src/datamodel_code_generator/__main__.py) | 2026-09-13 | `(ruff_path, "format", "-")` runs with `cwd=self.settings_path`; CLI settings paths derive from output paths. No explicit Ruff config or stdin filename is passed. Output outside the checkout can discover different settings. | contradicted | existing pins; explicit final formatting | §6.3, C19 |
| Ruff supports explicit configuration and logical stdin filenames | [Ruff 0.14.4 arguments](https://raw.githubusercontent.com/astral-sh/ruff/0.14.4/crates/ruff/src/args.rs), [0.14.4 configuration](https://raw.githubusercontent.com/astral-sh/ruff/0.14.4/docs/configuration.md), installed help | 2026-09-13 | `--config <CONFIG_OPTION>` and `--stdin-filename <STDIN_FILENAME>` are supported by both `check` and `format`. Explicit-config relative paths resolve against the current working directory. | verified | existing Ruff 0.14.4 | §6.3, C19 |

Generation consequence: use `--deserialize-default-values enum` consistently in generation and
reproducibility checks. The generator CLI supports `--formatters builtin` and
`--ignore-pyproject`; it does not expose a `--ruff-config` or `--settings-path` flag. An explicit
external Ruff formatting pass can use `ruff format --config <absolute-pyproject> <file...>`
on enumerated generated files, or `ruff format --config <absolute-pyproject>
--stdin-filename <logical-python-file> -` for text. Keep its working directory fixed to the
service checkout so relative config paths have the same meaning for normal and temporary
outputs. Apply the same generation/formatting sequence to both and compare the final bytes;
the small stdout-only probes above do not constitute the project's reproducibility gate.

## Phase 4 language-server negotiation and Podman prerequisites — 2026-09-13

Context7 discovery used `/astral-sh/ty`, `/rust-lang/rust-analyzer`, and
`/websites/podman_io_en`. The current toolchain configuration retains stable 1.98.1 with the
rust-analyzer component and the separate dated rustdoc producer nightly-2026-09-13
(`809936eac`, format 61). No reusable LSP probe script was found in the current scripts or
service-state inventory; earlier ty evidence remains documented in accepted ADR-0005.

Fresh probes used the installed executables with isolated service-owned working directories,
no workspace folders or opened target documents, and no build commands. Each completed
initialize, initialized, shutdown, and exit with status 0. Exact requests/responses and stderr
are retained under `.dev-state/cache/upstream-phase4-lsp-20260913/`. Podman metadata output is
under `.dev-state/p4p/`; an initial long-path failure is retained under
`.dev-state/cache/upstream-phase4-podman-20260913/`. No container or target code was run,
and no image was pulled or installed. The probe-owned Podman namespace keeper was terminated
after checking its UID, executable, and service-owned working directory; cleanup is recorded.

| Claim | Source URL | Retrieved | Exact quote or measured evidence | Verdict | Selected pin | Blueprint reference |
|---|---|---|---|---|---|---|
| The pinned ty server uses stdio without check-command flags | [ty 0.0.80 CLI](https://raw.githubusercontent.com/astral-sh/ty/0.0.80/docs/reference/cli.md), installed help and initialize | 2026-09-13 | `Usage: ty server`; its only listed option is help. ServerInfo is `{"name":"ty","version":"0.0.80"}`. Python environment/version flags belong to `ty check`; server settings use LSP configuration. | verified | existing ty 0.0.80 | §5.4, §13 Phase 4 |
| Installed ty advertises the navigation and pull-diagnostic APIs | [ty 0.0.80 release](https://github.com/astral-sh/ty/releases/tag/0.0.80), executable initialize captures | 2026-09-13 | `definitionProvider`, `referencesProvider`, and `implementationProvider` are true. `diagnosticProvider` identifies ty with `interFileDependencies=true`, `workspaceDiagnostics=true`, and `workDoneProgress=true`. This confirms negotiation, not correctness of a target analysis. | verified | same; ADR-0005 retained | §5.4, P07–P08b |
| ty exposes initialization controls that disable subprocess integrations | [ty 0.0.80 editor settings](https://raw.githubusercontent.com/astral-sh/ty/0.0.80/docs/reference/editor-settings.md) | 2026-09-13 | `untrustedWorkspace=true`: "ty does not run external commands". `experimental.useUv` accepts `off`, `scripts`, or `on`; initialize it to `off` for the service-controlled environment. `logFile` and `logLevel` are initialization options, distinct from changeable settings. | verified | same | §5.4, §10 |
| Pinned rust-analyzer starts its LSP server without a subcommand | [Rust 1.98.1 analyzer source](https://github.com/rust-lang/rust/tree/1.98.1/src/tools/rust-analyzer), installed `--help`, `--version`, and initialize | 2026-09-13 | `rust-analyzer 1.98.1 (48a229c 2026-09-01)`; top-level `--log-file <path>`, `--no-log-buffering`, `-v/--verbose`, and `-q/--quiet` exist. The `analysis-stats` and `diagnostics` subcommands' build-disabling flags are not LSP startup flags. | verified | existing Rust 1.98.1 component | §4.5, §13 Phase 4 |
| Rust-analyzer advertises navigation and document pull diagnostics | [Rust 1.98.1 capabilities](https://raw.githubusercontent.com/rust-lang/rust/1.98.1/src/tools/rust-analyzer/crates/rust-analyzer/src/lsp/capabilities.rs), executable initialize captures | 2026-09-13 | Definition, references, and implementation providers are true. `diagnosticProvider` identifies rust-analyzer with `interFileDependencies=true` and `workspaceDiagnostics=false`; do not request workspace diagnostics merely because document diagnostics exist. | verified | same | §4.5, R05–R06 |
| LSP position encoding must be negotiated and recorded | [Rust 1.98.1 capabilities](https://raw.githubusercontent.com/rust-lang/rust/1.98.1/src/tools/rust-analyzer/crates/rust-analyzer/src/lsp/capabilities.rs), [ty 0.0.80 release](https://github.com/astral-sh/ty/releases/tag/0.0.80), both executable initialize captures | 2026-09-13 | Both servers returned `positionEncoding=utf-16` for an empty client capability set and `utf-8` when offered `["utf-8","utf-16"]`. Both advertise incremental text synchronization (`change=2`) and open/close notifications. | verified | existing servers | §4.5, §5.4 |
| Disabling only rust-analyzer save checks disables all build activity | [Rust 1.98.1 configuration](https://raw.githubusercontent.com/rust-lang/rust/1.98.1/src/tools/rust-analyzer/crates/rust-analyzer/src/config.rs) | 2026-09-13 | `cargo_buildScripts_enable`, `procMacro_enable`, and `checkOnSave` default to true; proc macros imply build-script support. The negotiation probe explicitly set `cargo.buildScripts.enable=false`, `procMacro.enable=false`, and `checkOnSave=false`. These settings are not a sandbox substitute. | contradicted | explicit producer configuration | §4.5, §10 |
| Rootless Podman metadata operations work in isolated service state | [Podman 4.9.3 global options](https://docs.podman.io/en/v4.9.3/markdown/podman.1.html), local info/images captures | 2026-09-13 | Podman 4.9.3 info/images exit 0 with service-owned root/runroot/tmp/network/volume/XDG paths. Reported rootless=true, overlay, cgroup v2, cpu/memory/pids controllers, runc 1.5.1, and seccomp enabled; subordinate UID/GID ranges each have 65,536 IDs. | verified | installed Podman 4.9.3; no new pin | §10, C08, C20 |
| Arbitrarily long service-state paths work as Podman runroots | [Podman 4.9.3 options](https://docs.podman.io/en/v4.9.3/markdown/podman.1.html), installed executable failure | 2026-09-13 | Exit 125: "the specified runroot is longer than 50 characters". Retrying at `.dev-state/p4p/r` succeeds. Bound the runtime path separately from artifact-storage paths. | contradicted | short service-owned runtime root | §10 |
| A sanitized Podman environment automatically preserves systemd cgroup management | [Podman 4.9.3 options](https://docs.podman.io/en/v4.9.3/markdown/podman.1.html), local info captures | 2026-09-13 | Without the session-bus address, Podman warns and falls back to cgroupfs. Explicit `--cgroup-manager systemd` with host-side `DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus` reports systemd without warnings. The current session cgroup is not writable; the user service cgroup is delegated. | contradicted | explicit host-side broker configuration | §10, C08 |
| A suitable offline execution image is already available in the service store | [Podman 4.9.3 image listing](https://docs.podman.io/en/v4.9.3/markdown/podman-images.1.html), local image inventory | 2026-09-13 | `images --all --no-trunc --digests --format json` returns `[]`; info reports zero images and containers. Read-only inspection of the default user's overlay-images directory found only its lock file. A selected, digest-recorded producer image is still a prerequisite. | unverified | none selected | §10, §13 Phase 4 |
| Metadata preflight proves containment and resource enforcement | [Podman 4.9.3 run options](https://docs.podman.io/en/v4.9.3/markdown/podman-run.1.html), bounded probe scope | 2026-09-13 | Resource limits depend on rootless cgroup permissions. No container was started, so non-root execution, denied network/mount access, CPU/memory/PID limits, timeout cleanup, and C20 filesystem invariance remain unverified. Use `--pull=never` for later offline execution. | unverified | runtime acceptance still required | §10, C08, C20 |

Phase 4 consequence: retain full initialize transcripts and server identities in ProducerRun,
convert positions with the negotiated encoding, and distinguish missing capabilities from
startup/transport failures and analysis gaps. Configure the declared consumer environment;
do not let an empty workspace or ambient settings silently select the worker's interpreter.
Rootless Podman is available, but enabling execution still requires a verified producer image
and actual containment/resource gates. Explicitly select the cgroup manager; a host-side
systemd broker address is not a container environment variable or mount. These findings do
not turn unexecuted Phase 4 acceptance cases into passed or unsupported results.

## Phase 3 immutable GitHub revision acquisition — 2026-09-13

Context7 discovery used `/websites/github_en_rest`; GitHub's official REST/reference pages
and one bounded, unauthenticated public download established the following. The download was
inspected in memory without cloning, extraction to disk, builds, or target execution. GitHub
is an explicit initial provider restriction, not a claim of generic Git hosting support.

| Claim | Source URL | Retrieved | Exact quote or measured evidence | Verdict | Selected pin | Blueprint reference |
|---|---|---|---|---|---|---|
| GitHub accepts a commit SHA and returns its resolved identity | [Get a commit](https://docs.github.com/en/rest/commits/commits#get-a-commit) | 2026-09-13 | `GET /repos/{owner}/{repo}/commits/{ref}`; ref "Can be a commit SHA". The JSON response has top-level `sha` and `commit.tree.sha`. Require the returned commit SHA to equal the normalized requested 40-digit hexadecimal ID. | verified | protocol only | §3.1–§3.2, §13 Phase 3 |
| The commit response's file list is the complete revision tree | [Get a commit](https://docs.github.com/en/rest/commits/commits#get-a-commit) | 2026-09-13 | The `files` array describes the commit diff and may be paginated, with a documented 3,000-file maximum. It is not an inventory of the snapshot; preserve commit identity without deriving source coverage from that array. | contradicted | — | §3.2, §4.3 |
| Repository tar archives accept an explicit commit reference and redirect | [Tar archive endpoint](https://docs.github.com/en/rest/repos/contents#download-a-repository-archive-tar) | 2026-09-13 | `GET /repos/{owner}/{repo}/tarball/{ref}` returns `302`. "If you omit `:ref`, the repository’s default branch" is used. Always provide the verified full commit ID. | verified | protocol only | §3.2, §4.3 |
| Public commit and archive access requires a token | [Get a commit](https://docs.github.com/en/rest/commits/commits#get-a-commit), [tar archive](https://docs.github.com/en/rest/repos/contents#download-a-repository-archive-tar), [404 behavior](https://docs.github.com/en/rest/using-the-rest-api/troubleshooting-the-rest-api#404-not-found-for-an-existing-resource), live unauthenticated probe | 2026-09-13 | Both official endpoints permit unauthenticated public requests; no Authorization header was sent in the successful probe. A 404 can conceal a private resource, so preserve inaccessible-or-absent uncertainty separately from identified rate-limit failures. | contradicted | public repositories only | §3.2, §10 |
| The selected GitHub REST version can be explicit | [API versions](https://docs.github.com/en/rest/about-the-rest-api/api-versions), [required request headers](https://docs.github.com/en/rest/using-the-rest-api/troubleshooting-the-rest-api#user-agent-required) | 2026-09-13 | `X-GitHub-Api-Version: 2026-03-10` selects a currently supported version. Omitting the header currently defaults to `2022-11-28`; retired versions return 410 and nonexistent versions return 400. A valid User-Agent is required. The live probe supplied both headers. | verified | candidate API 2026-03-10 | §3.2 |
| The public tar endpoint currently redirects to codeload with the full SHA | [Live commit](https://api.github.com/repos/octocat/Hello-World/commits/7fd1a60b01f91b314f59955a4e4d4e80d8edf11d), [live tar endpoint](https://api.github.com/repos/octocat/Hello-World/tarball/7fd1a60b01f91b314f59955a4e4d4e80d8edf11d) | 2026-09-13 | Returned SHA exactly matched `7fd1a60b01f91b314f59955a4e4d4e80d8edf11d`; 302 Location was `https://codeload.github.com/octocat/Hello-World/legacy.tar.gz/7fd1a60b01f91b314f59955a4e4d4e80d8edf11d`, then 200 gzip. Measured 265 compressed bytes, SHA256 `9f40b519431e9754a1680244b820877ca975aa969ea4ae72798bfe3f67d0f139`. | verified | observation, not a permanent URL-shape guarantee | §3.2, C11 |
| One commit ID guarantees identical compressed archive bytes forever | [Archive stability](https://docs.github.com/en/repositories/working-with-files/using-files/downloading-source-code-archives#stability-of-source-code-archives) | 2026-09-13 | GitHub distinguishes stable commit file contents from compression/layout changes. Repository renaming also changes the wrapper directory. Retain the actual downloaded artifact digest independently of the immutable revision identity. | contradicted | content-addressed acquisition retained | §3.1–§3.2 |
| The archive wrapper directory is the full commit identity | [Archive stability](https://docs.github.com/en/repositories/working-with-files/using-files/downloading-source-code-archives#stability-of-source-code-archives), [Git archive](https://git-scm.com/docs/git-archive), live tar inspection | 2026-09-13 | Observed root `octocat-Hello-World-7fd1a60`; global PAX `comment` contained the full SHA. The archive had one wrapper directory and README. Validate paths before stripping one wrapper; handle bounded PAX metadata explicitly and corroborate a commit comment when present. | contradicted | — | §4.3, C11 |
| Archives materialize every external or generated source dependency | [Git LFS archive settings](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/managing-repository-settings/managing-git-lfs-objects-in-archives-of-your-repository), [Git archive attributes](https://git-scm.com/docs/git-archive#ATTRIBUTES), [submodule metadata](https://docs.github.com/en/rest/repos/contents#get-repository-content) | 2026-09-13 | LFS defaults to pointer files and can depend on repository settings; external LFS contents are excluded. `export-ignore` omits paths and `export-subst` changes archive text. Submodule metadata points to a separate repository/commit. Materialization of those dependencies is not established by the parent archive. | contradicted | explicit coverage gaps | §3.2, §4.3, §10 |
| The tar archive endpoint selects a library subdirectory | [Tar archive parameters](https://docs.github.com/en/rest/repos/contents#download-a-repository-archive-tar) | 2026-09-13 | Its path parameters are owner, repo, and ref; no package-root/path selector is provided. A monorepo archive can contain multiple package manifests. Selecting a package root is a separate identity decision, not part of archive acquisition. | contradicted | report unsupported/ambiguous roots explicitly | §3.1–§3.2, §13 Phase 3 |
| Unauthenticated public fetching has unlimited or exclusively primary rate limits | [REST rate limits](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api), [REST best practices](https://docs.github.com/en/rest/using-the-rest-api/best-practices-for-using-the-rest-api) | 2026-09-13 | Primary unauthenticated limit is 60/hour per originating IP; secondary limits also apply. Observe `retry-after`, `x-ratelimit-remaining`, and `x-ratelimit-reset`; 403/429 is not proof that a repository or commit is absent. Live commit response reported limit 60. | contradicted | bounded retries and recorded retry time | §3.2, §10 |

Conservative acquisition contract: accept only canonical HTTPS github.com owner/repository
URLs with two validated path components and a separate full hexadecimal commit ID; reject
credentials, unexpected ports, query/fragment selectors, extra path components, and encoded
separators. Construct API requests from those components, require successful commit JSON and
exact SHA agreement, then request the same SHA's archive. Preserve requested/final repository
URLs, redirect chain, HTTP validators, retrieval time, commit/tree IDs, and artifact digest.
Follow Location through the core's bounded redirect policy; revalidate HTTPS, allowed GitHub
hosts, public addresses, and endpoint scope at every hop. A changed redirect shape is a clear
provider limitation until supported, not permission for an arbitrary destination.

Apply compressed/decompressed/member/path limits and existing link/device/collision rules
before writing extracted members. Treat wrapper names as layout; keep repository-relative
source paths after safe removal of one common wrapper. Do not infer a package from the repo
name or take the first nested Cargo.toml/pyproject.toml: require a validated package selection
or return ambiguity, and retain its subdirectory in identity/provenance. Preserve missing
submodule/LFS/exported/generated content as gaps. The resulting object is a revision snapshot;
manifest version strings do not establish a published release relationship. Rust API evidence
requiring compilation remains unavailable in static mode, with the build-profile next action.

## Phase 4 immutable execution-image inputs and dependency acquisition — 2026-09-13

Context7 discovery used `/docker-library/docs`, `/docker-library/python`, `/astral-sh/uv`,
`/rust-lang/cargo`, and `/rust-lang/rustup`. Exact official Dockerfile commits, registry
manifests/configs, distribution manifests, PyPI release JSON, versioned source, and bounded
private-store probes established the rows below. These are Linux/amd64 candidates only.
Registry response digests were corroborated against SHA256 of the returned manifest bytes.
The earlier empty-store observation is superseded: both base images are now present in the
existing service-private `.dev-state/p4p/s` store. No derived execution image has been built.

| Claim | Source URL | Retrieved | Exact quote or measured evidence | Verdict | Selected pin | Blueprint reference |
|---|---|---|---|---|---|---|
| An official stable Rust base matches the repository compiler | [Official image metadata](https://raw.githubusercontent.com/docker-library/official-images/master/library/rust), [exact Dockerfile](https://raw.githubusercontent.com/rust-lang/docker-rust/0fc94fa4d5bac5532a9adf8e4f8b0fe6deca7d44/stable/trixie/slim/Dockerfile), [tag manifest](https://registry-1.docker.io/v2/library/rust/manifests/1.98.1-slim-trixie) | 2026-09-13 | `ENV RUST_VERSION=1.98.1`; official `1.98.1-slim-trixie` includes amd64. Index digest `sha256:ce84a5edd80c5f91e05c5533b1e53eb1da54028f33734dc06aa6b49fa190462d`; amd64 manifest `sha256:a2de23e559fd8afd260d22beb00f3987073ea0dcc2ba2646cccdaeda6a62a095`; config/image ID `sha256:b886ad7431930ba887022c12b2184006163493875059c3d7422259369f1202bc`. Measured compressed layers 321,993,690 bytes. | verified | candidate `docker.io/library/rust@sha256:a2de23e559fd8afd260d22beb00f3987073ea0dcc2ba2646cccdaeda6a62a095` | §5, §6, §13 Phase 4 |
| An official Python base matches the selected interpreter | [Official image metadata](https://raw.githubusercontent.com/docker-library/official-images/master/library/python), [exact Dockerfile](https://raw.githubusercontent.com/docker-library/python/688a0b86bb44289df16a363e9f41d90514c1a5f9/3.14/slim-trixie/Dockerfile), [tag manifest](https://registry-1.docker.io/v2/library/python/manifests/3.14.7-slim-trixie) | 2026-09-13 | `ENV PYTHON_VERSION=3.14.7`; official amd64 image. Index digest `sha256:cad9a2c871761c413caa6fdd6441c783451e740a48aaeba60ae62a8b53525ef6`; amd64 manifest `sha256:810da6270e43d30a1f3e0e1eabbeb6fbd9d78ad9dd2e754d5297a3d6cb42df46`; config/image ID `sha256:9b84b0c75adf7a34fb1b00b406a50dc9f7bebd3bfbe3e7fd9ff168a6f9d0dfb9`. Compressed layers 46,422,375 bytes. | verified | candidate `docker.io/library/python@sha256:810da6270e43d30a1f3e0e1eabbeb6fbd9d78ad9dd2e754d5297a3d6cb42df46` | §5, §6, §13 Phase 4 |
| The base images already contain every producer and native build prerequisite | The two exact Dockerfiles above | 2026-09-13 | Rust installs rustup with `--profile minimal` and gcc/libc6-dev, not rust-analyzer/rust-src or the fallback nightly. Python purges compiler/build dependencies after interpreter construction; it contains neither ty nor uv. Both have an unspecified default user, so execution must select a non-root UID. Arbitrary native dependencies require separately admitted image prerequisites. | unverified | derive minimal language-specific images | §5, §6 |
| Stable rust-analyzer and rust-src archives are available | [Stable manifest](https://static.rust-lang.org/dist/channel-rust-1.98.1.toml), [published checksum](https://static.rust-lang.org/dist/channel-rust-1.98.1.toml.sha256) | 2026-09-13 | Manifest date `2026-09-03`, SHA256 `a7c8774a5fd8441c997d94c029776cbc5eb111e9d72ab5d256fa69866644347e`, matched checksum. Both `available = true`. amd64 `rust-analyzer-1.98.1-x86_64-unknown-linux-gnu.tar.xz` SHA256 `19beefa939b986be0086a36a24c540801ce849ee74f39ee933b60f947b0d4431`; `rust-src-1.98.1.tar.xz` SHA256 `5c846ebcebcc7e2e0777a4cdaa12051691593f16a7e94edbae5e6241cc62d98c`. URLs use `https://static.rust-lang.org/dist/2026-09-03/`. | verified | stable 1.98.1 components; archives not installed by this probe | §5, §6 |
| The exact dated fallback nightly has required Linux components | [Dated nightly manifest](https://static.rust-lang.org/dist/2026-09-13/channel-rust-nightly.toml), [published checksum](https://static.rust-lang.org/dist/2026-09-13/channel-rust-nightly.toml.sha256) | 2026-09-13 | Manifest SHA256 `1940817baf8e016a115efb4442f587709ca77901820d97b018891806db175d32`, matched checksum. `rustc 1.100.0-nightly (809936eac 2026-09-12)` and cargo/rust-std are available for x86_64-unknown-linux-gnu. Respective `.tar.xz` SHA256: rustc `6ac25302f20012f3501eed63bbe33aee7dd726594be4a9f494517aba5d01f0ac`, cargo `596217e69cfc8bf4d8e6fe9976329369132fafd6a5fdf1dc4206c58fa9c6781b`, rust-std `504594603be011a90343d4185d972b977cbe1fbbe492cd612818aecd33bf9ea5`. | verified | `nightly-2026-09-13`; no nightly Docker tag assumed | §5.1, §13 Phase 4 |
| rustup supports explicit component/toolchain installation without self-update | [rustup 1.29.0 CLI source](https://raw.githubusercontent.com/rust-lang/rustup/1.29.0/src/cli/rustup_mode.rs), [components](https://rust-lang.github.io/rustup/concepts/components.html), [toolchain names](https://rust-lang.github.io/rustup/concepts/toolchains.html) | 2026-09-13 | `ComponentSubcmd::Add` accepts multiple components and `--toolchain`; `UpdateOpts` includes `profile`, `component`, and `no_self_update`. Official base installs rustup 1.29.0. Candidate commands: `rustup component add --toolchain 1.98.1 rust-analyzer rust-src`; `rustup toolchain install nightly-2026-09-13 --profile minimal --no-self-update`. | verified | recipe API only; derived installation remains unverified | §5, §6 |
| Exact ty and uv producer wheels support this Linux/Python image | [ty 0.0.80 release JSON](https://pypi.org/pypi/ty/0.0.80/json), [uv 0.12.13 release JSON](https://pypi.org/pypi/uv/0.12.13/json), private offline install/version probe | 2026-09-13 | Both Requires-Python `>=3.8`; wheels have `py3-none-manylinux_2_17_x86_64.manylinux2014_x86_64` tags. ty wheel 13,705,988 bytes, SHA256 `fe95feffa7156800c6f804195acb9fb5846a39671c7192c30fff23351aaf7c31`; uv wheel 20,029,009 bytes, SHA256 `5312891392eb5b72eaca9b880e85fe3e706a2b11342e59f7a5cf0304d14d5cc5`. Downloaded bytes matched both hashes. Offline pip installation in the pinned Python base succeeded; `ty --version` and `uv --version` returned 0.0.80 and 0.12.13. | verified | ty 0.0.80; candidate uv 0.12.13, matching workstation | §5.2, §6 |
| Installed producer executable identities can be recorded independently of wheel hashes | `.dev-state/p4p/python-producer-install-qualified.json` | 2026-09-13 | Measured installed native executable SHA256: ty `147dde31480eb80efb7ea4646486505673fbd254c0239bbc95785467c3defe39`; uv `b59310db262709ee92baf7954ef30820f1442ffa48b263f7001a236fe9004047`. No target library was imported or run. | verified | executable observations, not derived image digests | §3.3, §6 |
| The pinned bases start under the proposed rootless restrictions | `.dev-state/p4p/python-startup-qualified.json`, `.dev-state/p4p/rust-startup-qualified.json` | 2026-09-13 | Both exit 0 with UID/GID 65532, read-only root, dropped capabilities, no-new-privileges, no host-home/toolchain/repository mounts, `--network=none`, and configured resource limits. Python reports 3.14.7, only `lo`, `cpu.max=50000 100000`, `memory.max=268435456`, `pids.max=64`. Rust reports 1.98.1 and commit `48a229ceaefd4985c50990b14116b6d856af0985`. Podman 4.9.3 rejects `--dns=none` together with `--network=none`; removing the redundant DNS option succeeded. | verified | bounded startup observations only | §6, C08, C20 prerequisites |
| These startup probes establish full containment or a completed Phase 4 gate | Actual probe scope above | 2026-09-13 | Limits were observed, not stress-tested; canary invariance, blocked access, timeouts/cancellation cleanup, full LSP behavior, target compilation/runtime, and derived-image identity were not qualified. Initial producer installation exhausted the 64 MiB temporary filesystem; it succeeded with 384 MiB tmpfs mounts and 768 MiB memory. | unverified | operational qualification remains required | C08, C20, §13 Phase 4 |
| uv can resolve wheel metadata without a source build for admitted inputs | [uv 0.12.13 CLI source](https://raw.githubusercontent.com/astral-sh/uv/0.12.13/crates/uv-cli/src/lib.rs), `.dev-state/p4p/uv-acq/compile-wheel-only-log.json` | 2026-09-13 | `--only-binary`: "Only use pre-built wheels; don't build source distributions." A clean, private `uv pip compile --only-binary=:all: --python-version=3.14.7 --python-platform=x86_64-manylinux_2_40 --format=pylock.toml` resolved the two producer packages and emitted exact wheel URLs, sizes, and hashes. `--no-build` conflicts with `--only-binary`; the rejected combination returned exit 2. | verified | uv 0.12.13 exact flags | §5.2, §6 |
| Wheel-only resolution alone is a no-target-execution boundary | [uv 0.12.13 CLI source](https://raw.githubusercontent.com/astral-sh/uv/0.12.13/crates/uv-cli/src/lib.rs) | 2026-09-13 | The option documentation also says "uv may still build editable requirements" and permits cached wheels previously built from source. Reject editable, directory, Git, arbitrary URL, and unadmitted cached-build inputs before resolution; use a fresh owned cache and registry requirements. Source-only/dynamic-metadata cases need the separate authorized build producer. | unverified | input validation and acquisition policy required | §5.2, §6 |
| uv provides a wheelhouse download command or compile materializes wheels | Installed `uv 0.12.13 pip --help`, exact source above | 2026-09-13 | Available pip subcommands include `compile`, `sync`, `install`, and inspection/removal commands; no `download` subcommand exists. The exercised compile emitted a pylock file, not a wheelhouse. Core-owned bounded artifact fetching must consume validated resolved wheel URLs/hashes, or separately admit cache outputs. | unverified | do not invent `uv pip download` | §B1, §5.2, §6 |
| Offline hash-checked wheel installation has explicit supported flags | [uv 0.12.13 CLI source](https://raw.githubusercontent.com/astral-sh/uv/0.12.13/crates/uv-cli/src/lib.rs), installed `uv pip install --help` | 2026-09-13 | Supported: `--offline`, `--no-index`, `--find-links`, `--only-binary=:all:`, `--require-hashes`, `--no-deps`, `--target`, `--link-mode=copy`, `--no-config`, `--no-python-downloads`. Hash mode requires exact versions/direct URLs and rejects Git/editable dependencies. Use a core-generated fully pinned requirements file containing every admitted dependency and wheel hash. | verified | exact uv 0.12.13 call surface; generic dependency installation not exercised | §5.2, §6 |
| Cargo fetch can acquire dependencies separately from compilation | [Exact Cargo fetch implementation](https://raw.githubusercontent.com/rust-lang/cargo/797e8a9bc/src/cargo/ops/cargo_fetch.rs), [matching command source](https://raw.githubusercontent.com/rust-lang/cargo/797e8a9bc/src/doc/man/cargo-fetch.md), [Cargo fetch reference](https://doc.rust-lang.org/cargo/commands/cargo-fetch.html) | 2026-09-13 | "Subsequent Cargo commands will be able to run offline after a `cargo fetch` unless the lock file changes." Exact source resolves the workspace and downloads packages via `packages.get_many(to_download)?`; it does not enter the target compilation pipeline, though trusted rustc target discovery can run. `--locked` rejects absent/changed lockfiles; `--target x86_64-unknown-linux-gnu` bounds target fetching; `--frozen` means locked plus offline. | verified | Cargo bundled in stable 1.98.1, source commit 797e8a9bc | §5.1, §6 |
| Cargo's offline flag alone contains subprocesses or arbitrary acquisition configuration | Exact Cargo fetch source and command reference above | 2026-09-13 | The flags constrain Cargo dependency access/resolution; they do not establish a sandbox around build scripts, procedural macros, rustc wrappers, credential helpers, or configuration-selected executables. Strip unadmitted configuration/environment/source types, use service-owned cwd/cache/manifest inputs, and enforce network isolation on actual execution independently. | unverified | acquisition and execution remain separate policy operations | §6, C08, C20 |

Concrete candidate recipe, to be implemented and qualified separately:

1. Build a Rust producer image from the exact Rust amd64 manifest above. Add stable
   `rust-analyzer` and `rust-src`, then `nightly-2026-09-13 --profile minimal --no-self-update`.
   Retain the distribution manifests/checksums and component digests above; verify the
   resulting versions, compiler commits, and emitted rustdoc format before admission.
   Keep `RUSTUP_HOME=/usr/local/rustup` baked into the image. Give executions a fresh writable
   service-owned `CARGO_HOME` and target directory; never mount the host toolchain/home.
   Select stable and dated-nightly tools explicitly, independent of analyzed toolchain files.
2. Build a Python producer image from its exact amd64 manifest. Copy only the two verified
   wheels and a core-generated requirements file containing the hashes above into the build
   context. The successful bootstrap probe used the image's existing pip with
   `python3 -I -m pip --isolated --disable-pip-version-check install --no-cache-dir --no-index
   --find-links=/wheelhouse --only-binary=:all: --require-hashes --no-deps --no-compile
   --target=/tools -r /wheelhouse/requirements.txt`. Bake the installed producer executables
   into a fixed image-owned directory, with no host private-bin mount. Record their hashes
   and the final OCI manifest/config digests. Base/input digests do not substitute for the
   unbuilt final image digest or prove bit-identical rebuilds.
3. Network-enabled dependency acquisition is its own bounded producer operation. For Rust,
   use a validated service-owned manifest/lock and clean Cargo configuration with
   `cargo +1.98.1 fetch --locked --target x86_64-unknown-linux-gnu`; an absent lock needs an
   explicit resolution operation whose generated lock is retained. Import only admitted
   registry/cache artifacts into the later capsule. Do not allow unreviewed Git sources,
   credential providers, wrappers, or target-controlled configuration to run during fetch.
4. For Python, resolve sanitized registry requirements with uv's wheel-only compile to
   pylock, with explicit interpreter/platform identity, `--no-config`, an empty owned cache,
   and `--no-python-downloads`. Rust validates and fetches the chosen URLs into an immutable
   wheelhouse using the service's redirect/host/size/hash policy. Populate a fresh owned
   target using `uv pip install --offline --no-index --find-links=/wheelhouse
   --only-binary=:all: --require-hashes --no-deps --target=/capsule/python
   --link-mode=copy -r /capsule/requirements.txt`, retaining the global no-config/download
   settings. Resolve the complete dependency closure before using `--no-deps`; preserve
   unmet wheel/platform/native prerequisites as explicit gaps. A target-populated Python
   environment is executed only inside the execution sandbox, never by the host acquisition
   process. The demonstrated platform flag is a supported conservative candidate, not a
   measured universal target compatibility claim.
5. Language-server, build, rustdoc, and runtime invocations use the admitted derived image
   by digest with `--pull=never`, `--network=none`, non-root UID, isolated writable capsule
   paths, read-only roots/inputs, capability/resource bounds, and clean environment. Full
   acceptance still must prove actual denial, deadlines/cancellation, descendants cleanup,
   and C20 canary invariance for every enabled profile.

Probe logs and verified producer wheels are under `.dev-state/p4p/`. The offline pip probe
mounted only the service-owned wheelhouse read-only and used temporary container storage;
installed tools disappeared with the container. `images-final-containers.json` records zero
remaining containers; `keeper-cleanup-images.json` records termination of the exact owned
catatonit namespace keeper after PID/UID/executable/cwd validation. Base images and wheels
remain available for the implementation's image builder. No target libraries were executed,
no host toolchain directories were mounted, and no production/global state was used.

## FastMCP 4.0.3 bounded response shape — 2026-09-13

Context7 discovery used `/prefecthq/fastmcp`. The guessed GitHub `v4.0.3/src/fastmcp/`
source URLs returned 404, so exact-release proof used the published distribution instead.
`fastmcp==4.0.3` depends on `fastmcp-slim[client,server]==4.0.3`; the slim wheel SHA256 is
`3756ed4bd9f82f40adaf51974b7cdd1463ab6b9f479cf69b69b2692969312b87`. Its relevant files
matched installed bytes exactly. No dependency pin changed.

| Claim | Source URL | Retrieved | Exact quote or measured evidence | Verdict | Selected pin | Blueprint reference |
|---|---|---|---|---|---|---|
| Explicit compact content prevents automatic duplication of the structured envelope into text | [ToolResult documentation](https://gofastmcp.com/servers/tools#toolresult-and-metadata), [exact 4.0.3 slim wheel](https://files.pythonhosted.org/packages/43/4b/6b31820d87f56d5773538860878c8634b618cd1e9044b3bfbd8a78380a0f/fastmcp_slim-4.0.3-py3-none-any.whl), `fastmcp/tools/base.py` | 2026-09-13 | Constructor uses `elif content is None: content = structured_content`; supplying `ToolResult(content=compact_string, structured_content=envelope)` bypasses that fallback. `convert_result` returns an existing ToolResult unchanged. Thus the text contains the explicit summary and structured content contains the envelope. | verified | existing FastMCP 4.0.3 | §7.3 |
| An explicit output schema may be inlined when served | [Schema dereferencing documentation](https://gofastmcp.com/servers/tools#arguments), exact slim wheel `fastmcp/server/middleware/dereference.py` | 2026-09-13 | When `$defs` or `$ref` exists, middleware sets `updates["output_schema"] = dereference_refs(tool.output_schema)`. Compare schema meaning/validation rather than demanding byte equality with a canonical referenced schema. | verified | existing FastMCP 4.0.3 | §7.3, schema ownership |
| Client data and structured content have different representations | [Client tools documentation](https://gofastmcp.com/clients/tools), exact slim wheel `fastmcp/client/mixins/tools.py` | 2026-09-13 | With an output schema the client calls `json_schema_to_type(output_schema)` and `type_adapter.validate_python(structured_content)` for `data`; its returned `structured_content=result.structured_content` remains the raw dictionary. Schema-generated Pydantic data is a convenience representation, not the raw wire-size oracle. | verified | existing FastMCP 4.0.3 | §7.3 |

The service's 160-byte encoded summary limit and 512-byte framing allowance are local
blueprint/implementation constraints, not guarantees supplied by FastMCP. Actual stdio
serialization must remain the regression oracle for response size and envelope duplication.

## Phase 4 derived producer images and writable capsule mapping — 2026-09-13

This later verification supersedes the earlier unbuilt-derived-image prerequisite. Context7
discovery used `/websites/podman_io_en`; Podman 4.9.3 documentation and actual private-store
build/run logs are the evidence. Only producer executables and service-authored filesystem
probes ran. No analyzed package or target library was executed. Build contexts, exact argv,
input hashes, image inspections, and stdout/stderr are retained under
`.dev-state/p4p/producer-builds-20260913/`.

| Claim | Source URL | Retrieved | Exact quote or measured evidence | Verdict | Selected pin | Blueprint reference |
|---|---|---|---|---|---|---|
| A rootless caller can map to container UID/GID 65532 and write its own capsule bind without chown | [Podman 4.9.3 user namespaces](https://docs.podman.io/en/v4.9.3/markdown/podman-run.1.html#userns-mode), `.dev-state/p4p/keep-id-capsule-probe.json` | 2026-09-13 | `uid=UID` overrides the mapped container UID; equivalent `gid=GID` is supported. Actual `--userns=keep-id:uid=65532,gid=65532 --user=65532:65532` probe read and wrote a service-owned bind: process/file UID/GID 65532 inside, output UID/GID 1000 on the host. No `:U`, host chown, home, or toolchain mount was used. | verified | existing Podman 4.9.3 | §6, C20 prerequisite |
| The Python producer image can be built entirely offline from the pinned base and wheels | [Podman 4.9.3 build](https://docs.podman.io/en/v4.9.3/markdown/podman-build.1.html), `python-build.json`, `python-image-inspect.json` | 2026-09-13 | Build exit 0 with `--pull=never --network=none`; image ID `sha256:44afc73e29e9a456265b868623b52b76d48e22e7ea3b9a0ff24853f75d5e11de`; OCI manifest digest `sha256:35cf63101e435d726858cf51231d4cdd9de57df1815f15968e21fc49678a6ea1`. Image is linux/amd64, 254,822,568 bytes, default user `65532:65532`. | verified | private candidate image ID above | §5.2, §6 |
| Stable and dated-nightly Rust producers can be baked into the pinned Rust base | The verified Rust distribution manifests above; `rust-build.json`, `rust-image-inspect.json` | 2026-09-13 | Build exit 0 with explicit `--network=private`, installing stable rust-analyzer/rust-src and `nightly-2026-09-13`. Image ID `sha256:28fc46e4b2dab9405af3b813727fc05f9e1d935b8724e77364016114f2e5588d`; OCI manifest digest `sha256:95fa3d0b80f7ac1996c75193bf58c9283b338fb3af02e2a1e64a7c237fda52f2`. Image is linux/amd64, 1,672,707,277 bytes, default user `65532:65532`. | verified | private candidate image ID above | §5.1, §6 |
| The derived Python producers run offline with an owned writable capsule | `python-offline-probe.json` | 2026-09-13 | Exit 0 by immutable image ID, read-only root, all capabilities dropped, no-new-privileges, keep-id UID/GID 65532, only service-owned capsule bind, network none, 1 CPU/512 MiB/128 PID/25-second bounds. Python 3.14.7, ty 0.0.80, uv 0.12.13; producer executable hashes match the previous wheel-install probe. Only loopback was visible. | verified | derived Python candidate | §5.2, §6 prerequisites |
| Every requested Rust producer is accessible to the non-root offline capsule | `rust-offline-probe.json` | 2026-09-13 | Same execution restrictions; exit 0. Stable rustc/Cargo 1.98.1; rust-analyzer `1.98.1 (48a229c 2026-09-01)`; installed rust-src and readable standard-library source. Nightly rustc/rustdoc `1.100.0-nightly`, commit `809936eac66c547a5127ce1da805f0d3a6789b98`; nightly Cargo `1.100.0-nightly`, commit `7941be6fb416b4cd9666aef7b858dfea25587a8c`. No host toolchain mount was needed. | verified | derived Rust candidate | §5.1, §6 prerequisites |
| These builds prove byte-identical rebuilds or Phase 4 implementation acceptance | Actual build/probe scope | 2026-09-13 | Each candidate was built once from recorded inputs with fixed `--timestamp=1789257600`. No second-build identity comparison, target package build/import, LSP semantic operation, image admission integration, or C20 canary gate was run. Rustup downloaded through its verified dated/versioned distribution manifests; future rebuilding must corroborate those inputs again. | unverified | upstream producer candidates only | §6, §13 Phase 4 |

Exact Containerfiles are service-owned artifacts and their contents are reproduced here so
the recipe survives a development-state cleanup. Python Containerfile SHA256 is
`e6b235f16fa57328a9a0f734b41868fa682f3b97d5312c249396491bcee34fdf`:

```dockerfile
FROM docker.io/library/python@sha256:810da6270e43d30a1f3e0e1eabbeb6fbd9d78ad9dd2e754d5297a3d6cb42df46
COPY wheels /opt/producer-wheels
RUN python3 -I -m pip --isolated --disable-pip-version-check install --no-cache-dir --no-index --find-links=/opt/producer-wheels --only-binary=:all: --require-hashes --no-deps --no-compile --target=/opt/producers -r /opt/producer-wheels/requirements.txt
ENV PATH=/opt/producers/bin:/usr/local/bin:/usr/bin:/bin
ENV PYTHONDONTWRITEBYTECODE=1
ENV UV_NO_CONFIG=1
ENV UV_PYTHON_DOWNLOADS=never
WORKDIR /capsule
USER 65532:65532
```

`wheels/` contains only the two previously verified ty/uv wheels and this requirements file
(SHA256 `9be246e13af4a04d5b20e6fcf2ac2e71c524389c159e694b2941c8227eb8401a`):

```text
ty==0.0.80 --hash=sha256:fe95feffa7156800c6f804195acb9fb5846a39671c7192c30fff23351aaf7c31
uv==0.12.13 --hash=sha256:5312891392eb5b72eaca9b880e85fe3e706a2b11342e59f7a5cf0304d14d5cc5
```

Rust Containerfile SHA256 is
`5722450c065b538ca7c631e001e66a6b25ba7851f80ab83f3409a320f882e506`:

```dockerfile
FROM docker.io/library/rust@sha256:a2de23e559fd8afd260d22beb00f3987073ea0dcc2ba2646cccdaeda6a62a095
COPY provenance /opt/producer-provenance
RUN rustup component add --toolchain 1.98.1 rust-analyzer rust-src
RUN rustup toolchain install nightly-2026-09-13 --profile minimal --no-self-update
ENV RUSTUP_HOME=/usr/local/rustup
ENV RUSTUP_TOOLCHAIN=1.98.1
ENV CARGO_HOME=/capsule/cargo-home
ENV CARGO_TARGET_DIR=/capsule/target
ENV PATH=/usr/local/cargo/bin:/usr/local/bin:/usr/bin:/bin
WORKDIR /capsule
USER 65532:65532
```

The Rust context's `provenance/stable-manifest.toml` and `nightly-manifest.toml` are the exact
distribution manifests recorded above, with SHA256
`a7c8774a5fd8441c997d94c029776cbc5eb111e9d72ab5d256fa69866644347e` and
`1940817baf8e016a115efb4442f587709ca77901820d97b018891806db175d32` respectively. The build
used Rustup's normal checksum validation. Both builds used `--pull=never --no-cache
--layers=false --format=oci --timestamp=1789257600 --rm --force-rm --http-proxy=false
--cpu-period=100000 --cpu-quota=200000`. Python used `--network=none --memory=1g` and an
180-second external deadline; Rust used `--network=private --memory=2g` and 300 seconds.
They finished in approximately 1.7 and 9.5 seconds. Buildah 1.33.7 emitted ambient-capability
warnings retained in stderr; successful producer execution does not certify build resource
enforcement or make these commands suitable for untrusted target execution.

Measured executable SHA256 values inside the derived images:

| Executable | SHA256 |
|---|---|
| Python image `/opt/producers/bin/ty` | `147dde31480eb80efb7ea4646486505673fbd254c0239bbc95785467c3defe39` |
| Python image `/opt/producers/bin/uv` | `b59310db262709ee92baf7954ef30820f1442ffa48b263f7001a236fe9004047` |
| Stable toolchain `bin/rustc` | `859254978c0a0402c32f949f6de0d99aee73be8d15f45aac00ae1448aac51e74` |
| Stable toolchain `bin/cargo` | `da77c8b33849312255ccde3179198ada4c8deb370488d050286146b1d1b27e14` |
| Stable toolchain `bin/rust-analyzer` | `25da0296e9aa8e538e6ee36675f48b0cd3896c5d1764a3a71a0e2438757f5640` |
| Dated-nightly toolchain `bin/rustc` | `b2e1e90837af87675deb16c4219c6a9bad24ad5bd7594993f0502740a116b8ba` |
| Dated-nightly toolchain `bin/rustdoc` | `051c4536dd278082b957da9b303e43826e04be0f2046e0964e6bf9dc6830d165` |
| Dated-nightly toolchain `bin/cargo` | `19cab923611ddbc2e63bb2583cbae0f554a6b1167fba1d04fd70be136654113c` |

Reuse the full image IDs from `image-candidates.json` with `run --pull=never`; the exact
private Podman argv prefix, environment allowlist, and working directory are in
`podman-command-context.json`. All paths below are rooted at
`/home/paul/library-enrichment/.dev-state/p4p`: storage `s`, runroot `r`, temp directory `l`,
volume path `v`, network configuration `n`, hooks `h`; events backend none and cgroup manager
systemd. The process environment sets owned TMPDIR `t` and XDG cache/config/data/runtime
`c`/`f`/`d`/`x`, PATH `/usr/bin:/bin`, LANG `C.UTF-8`, USER/LOGNAME `paul`, and the host-only
systemd broker address `unix:path=/run/user/1000/bus`. That address is not passed or mounted
into the container. Do not invoke the candidates through default/global Podman storage or
rely on the mutable convenience tags. Exact offline run argument arrays are retained in
each language's `*-offline-probe.json`.

`final-containers.json` records an empty `podman ps --all --external` result, including
external build containers. `keeper-cleanup.json` records termination of the exact private
catatonit keeper after UID/executable/cwd/command validation. Derived/base images, contexts,
wheel inputs, and probe output files remain available for implementation reuse.

## Phase 4 exact ty check and offline uv installation — 2026-09-13

Context7 discovery used `/astral-sh/ty`; current official docs were checked against the exact
ty 0.0.80 and uv 0.12.13 executables in the admitted Python candidate image
`sha256:44afc73e29e9a456265b868623b52b76d48e22e7ea3b9a0ff24853f75d5e11de`.
All commands ran offline as UID/GID 65532 with keep-id, read-only root, and only service-owned
capsule/wheelhouse binds. Exact argv, fixture bytes, outputs, and return codes are under
`.dev-state/p4p/ty-uv-cli-20260913/`.

| Claim | Source URL | Retrieved | Exact quote or measured evidence | Verdict | Selected pin | Blueprint reference |
|---|---|---|---|---|---|---|
| ty accepts a `--no-config` switch analogous to uv | Exact image `ty check --help`; `ty-no-config-rejected.json`; [ty CLI](https://docs.astral.sh/ty/reference/cli/) | 2026-09-13 | Actual exit 2: `error: unexpected argument '--no-config' found`. The supported configuration-file option is `--config-file`; do not transfer uv's option to ty. | contradicted | existing ty 0.0.80; CLI recipe correction only | §5.2, §6 |
| An explicit ty config file disables all project and user configuration discovery | [ty environment reference](https://docs.astral.sh/ty/reference/environment/#ty_config_file), `discovery-control.json`, `explicit-config-still-loads-user.json` | 2026-09-13 | Docs say explicit configuration is used "instead of discovering configuration files automatically". Measured behavior is narrower: an explicit empty config bypassed malformed `/capsule/ty.toml`, but still failed on malformed `$XDG_CONFIG_HOME/ty/ty.toml`. The discovery control without an explicit file failed on project config. | contradicted | control both explicit config and user-config location | §5.2, §6 |
| Explicit interpreter/search paths plus controlled configuration support bounded file checking | [ty CLI](https://docs.astral.sh/ty/reference/cli/), `ty-check-help.json`, `valid-check.json`, `invalid-check.json` | 2026-09-13 | Supported `--python=/usr/local/bin/python3`, `--extra-search-path=/capsule/python`, `--python-version=3.14`, `--python-platform=linux`, `--project=/capsule`, and explicit positional `/capsule/consumer.py`. With a trusted empty config and `XDG_CONFIG_HOME=/opt/libenr-empty-config`, valid pathlib-only code exited 0; `result: int = Path("example.txt").name` exited 1 with `error[invalid-assignment]`. | verified | existing ty 0.0.80 | §5.2, §13 Phase 4 prerequisite |
| The image's installed ty executable lives at the producer path | Derived image inspection, exact executable/version probes above | 2026-09-13 | `/opt/producers/bin/ty` is the native executable installed by pip's `--target=/opt/producers`, and the image PATH begins `/opt/producers/bin`. Its SHA256 is `147dde31480eb80efb7ea4646486505673fbd254c0239bbc95785467c3defe39`. Installing capsule dependency wheels does not change the producer command path. | verified | existing derived image | §5.2, §6 |
| uv can install the admitted wheelhouse into an isolated capsule directory offline | Exact image `uv pip install --help`; [uv 0.12.13 source](https://raw.githubusercontent.com/astral-sh/uv/0.12.13/crates/uv-cli/src/lib.rs); `uv-offline-install.json` | 2026-09-13 | Actual exit 0 using the full invocation below; interpreter reported `/usr/local/bin/python3`, CPython 3.14.7. Hash-verified ty 0.0.80 and uv 0.12.13 wheels installed into `/capsule/python`, without dependency resolution beyond the admitted complete list, source builds, network, or bytecode compilation. | verified | uv 0.12.13; tested with producer wheels only | §5.2, §6 |

Working ty invocation inside the image:

```text
/opt/producers/bin/ty check --project=/capsule --python=/usr/local/bin/python3 --extra-search-path=/capsule/python --python-version=3.14 --python-platform=linux --output-format=concise --color=never --no-progress --config-file=/capsule/probe-config/ty.toml /capsule/consumer.py
```

The probe's trusted config file was empty. Container environment
`XDG_CONFIG_HOME=/opt/libenr-empty-config` selected a nonexistent location in the read-only
image, preventing a target-controlled user config from being loaded. Production may instead
provide an immutable empty service-config directory. An explicit config file alone is
insufficient in this version. Preserve a clean environment rather than inheriting
`TY_CONFIG_FILE`, `PYTHONPATH`, virtual-environment, or Conda settings. These findings narrow
an upstream documentation claim and correct invocation details; the ty-only semantic engine
and execution-policy boundaries are unchanged.

The valid fixture imported only `pathlib.Path`, returned `Path(value).name` from a function
annotated `str`, and assigned its result to `str`. The invalid fixture assigned that name to
`int`. Both were checked as `/capsule/consumer.py`, without executing either file. The
extra-search-path option was accepted with the installed target present; these fixtures do
not establish arbitrary third-party typing coverage or dependency-runtime compatibility.

Working uv invocation inside the image:

```text
/opt/producers/bin/uv --no-config --no-python-downloads --cache-dir=/capsule/cache pip install --python=/usr/local/bin/python3 --offline --no-index --find-links=/wheelhouse --only-binary=:all: --require-hashes --no-deps --target=/capsule/python --link-mode=copy -r /wheelhouse/requirements.txt
```

The wheelhouse was a read-only service-owned bind and the cache/target were writable owned
capsule paths. The requirement file was the exact two-package, hash-pinned file recorded
above. No `--no-build` was combined with `--only-binary`. No installed package was imported
or executed by these checks. `cleanup.json` records zero ordinary/external containers and
termination of the exact private namespace keeper after ownership validation. This verifies
producer CLI behavior, not a Phase 4 implementation acceptance gate.

## Phase 4 exact LSP settings and implementation observations — 2026-09-13

Context7 discovery used `/astral-sh/ty` and `/rust-lang/rust-analyzer`. The ty 0.0.80 tag's
`ruff` gitlink selects commit `e7230cac059fa28bd0e534e4571c3560c695efbe`; exact source below
therefore establishes the release's configuration deserialization. The actual ty transcript
used the derived Python image and service-authored fixture, with no target import/execution.
Exact sources, sample request files, framed stdin/stdout captures, decoded transcript,
diagnostics, implementation results, stderr, and cleanup are retained under
`.dev-state/p4p/lsp-settings-20260913/`.

| Claim | Source URL | Retrieved | Exact quote or measured evidence | Verdict | Selected pin | Blueprint reference |
|---|---|---|---|---|---|---|
| ty initialization settings are directly nested in initializationOptions | [Exact initialization/options source](https://raw.githubusercontent.com/astral-sh/ruff/e7230cac059fa28bd0e534e4571c3560c695efbe/crates/ty_server/src/session/options.rs), [release gitlink](https://api.github.com/repos/astral-sh/ty/git/trees/0.0.80?recursive=1), `ty-initialize-request.json` | 2026-09-13 | `InitializationOptions` flattens `ClientOptions`, which flattens global/workspace options. Thus `untrustedWorkspace`, `experimental`, `configurationFile`, and `configuration` are direct initializationOptions members; no outer `ty` or `settings` object belongs here. Actual initialization accepted the sample below and logged the parsed values. | verified | ty 0.0.80 | §5.2, §6 |
| Exact Python environment options can be supplied inline | [Release configuration reference](https://raw.githubusercontent.com/astral-sh/ty/0.0.80/docs/reference/configuration.md), exact options source above; `ty-server.stderr` | 2026-09-13 | `configuration.environment` supports `python`, `extra-paths`, `python-version`, `python-platform`. Actual logs resolved `/usr/local/lib/python3.14/site-packages`, real stdlib `/usr/local/lib/python3.14`, and explicitly added `/capsule/python`; version/platform were Python 3.14/Linux. A service-authored dependency stub resolved without an unresolved-import diagnostic. | verified | explicit interpreter `/usr/local/bin/python3` and extra path `/capsule/python` | §5.2 |
| Untrusted workspace and disabled uv integration have explicit initialization controls | [Release editor settings](https://raw.githubusercontent.com/astral-sh/ty/0.0.80/docs/reference/editor-settings.md), exact options source above | 2026-09-13 | `untrustedWorkspace: true`; `experimental: {useUv: "off"}`. Source `use_uv` returns `UseUv::Off` for an untrusted workspace. The exact source separately validates trust and preserves it if another option is malformed; invalid trust fails initialization. Actual logs showed `workspace_trust: Untrusted`. | verified | both controls explicit | §5.2, §6 |
| LSP configurationFile eliminates the need to control user configuration | `ty-server.stderr`, preceding CLI configuration probe | 2026-09-13 | LSP logs: `Using overridden configuration file at '/capsule/probe-config/ty.toml'`, followed by searching `/opt/libenr-empty-config/ty/ty.toml`. The malformed discoverable project ty.toml was bypassed; user-config lookup still occurred. Preserve explicit trusted configuration plus immutable-empty XDG_CONFIG_HOME. | contradicted | same controlled-user-config requirement as CLI | §6 |
| ty implementation requests can return nominal subclasses and method overrides | `ty-transcript.json`, `ty-results.json`, `ty-nominal_class-request.json`, `ty-nominal_method-request.json` | 2026-09-13 | Advertised `implementationProvider=true`, negotiated UTF-8. Query on Base returned LocationLinks to Base itself and Child; method query returned Base.value itself and Child.value. Target selection lines were 3/7 and 4/8 (zero-based). Successful navigation is not an exclusively-descendant relation. | verified | ty 0.0.80 nominal implementation observation | P08a prerequisite, §5.2 |
| A Protocol implementation query enumerates all structurally compatible classes | Same actual transcript; fixture `capsule/consumer.py` | 2026-09-13 | Shape(Protocol) query returned Shape itself and explicit Declared(Shape), on lines 11/18; method results were lines 12/19. It omitted Structural despite accepting `accepted: Shape = Structural()` without diagnostic. Preserve explicit/nominal observations and incomplete structural coverage; an omitted structural class is not proof of incompatibility. | contradicted | conservative Protocol coverage | P08b prerequisite, §5.2 |
| Pull diagnostics work with these settings and source-only fixtures | `ty-results.json`, raw `ty-sent.bin` / `ty-received.bin` | 2026-09-13 | `textDocument/diagnostic` returned `kind: full` and one intentional `invalid-assignment` at line 24, severity 1; no diagnostic for the structural Protocol assignment or dependency stub. Server completed shutdown/exit with code 0. Client did not advertise file watching and server warned about external-change staleness. | verified | bounded single-document transcript, not service acceptance | P08a/P08b prerequisites, §5.2 |
| rust-analyzer 1.98.1 exposes a cargo.offline configuration field | [Exact compiler-release config source](https://raw.githubusercontent.com/rust-lang/rust/48a229ceaefd4985c50990b14116b6d856af0985/src/tools/rust-analyzer/crates/rust-analyzer/src/config.rs), exact image `rust-analyzer --print-config-schema` | 2026-09-13 | No `rust-analyzer.cargo.offline` exists in the emitted schema. `cargo.extraArgs` accepts Cargo CLI arguments; `cargo.extraEnv` accepts an environment map. `cargo.noDeps=true` is a different choice that omits dependency metadata. Use admitted prefetched dependencies plus offline Cargo flags/environment and the execution network boundary. | contradicted | rust-analyzer 1.98.1 | §5.1, §6 |
| rust-analyzer accepts explicit feature/target and execution-disable settings | Exact config source and `rust-analyzer-config-schema.json` | 2026-09-13 | `cargo.features` is a string array or `"all"`; `cargo.noDefaultFeatures` is boolean; `cargo.target` is nullable string. `cargo.buildScripts.enable`, `procMacro.enable`, and `checkOnSave` are booleans defaulting true. Exact source computes build-script enablement as buildScripts OR procMacro, so explicitly disable both. | verified | sample request retained; no new Rust semantic transcript claimed | §5.1, §6 |

The exercised ty `initializationOptions` were:

```json
{
  "untrustedWorkspace": true,
  "experimental": {"useUv": "off"},
  "logLevel": "debug",
  "configurationFile": "/capsule/probe-config/ty.toml",
  "configuration": {
    "environment": {
      "python": "/usr/local/bin/python3",
      "extra-paths": ["/capsule/python"],
      "python-version": "3.14",
      "python-platform": "linux"
    }
  },
  "diagnosticMode": "openFilesOnly"
}
```

The client advertised `workspace.configuration=false`, so the server used initialization
options directly. If a production client enables `workspace/configuration`, it must answer
server requests with the corresponding options object; editor configuration wrappers are
client-specific and should not be copied into initializationOptions. Files were opened via
`textDocument/didOpen`; diagnostics and implementation requests used `file:///capsule/consumer.py`
and negotiated UTF-8 positions. The full trace includes initialize, initialized, didOpen,
diagnostic, four implementation requests, shutdown, and exit. Raw responses are evidence;
the service must preserve self-results/LocationLinks and report Protocol scope explicitly.

The Rust sample `initializationOptions`, read against exact source/emitted schema, are:

```json
{
  "cargo": {
    "extraArgs": ["--offline", "--locked"],
    "extraEnv": {"CARGO_NET_OFFLINE": "true"},
    "features": [],
    "noDefaultFeatures": false,
    "target": "x86_64-unknown-linux-gnu",
    "targetDir": "/capsule/target",
    "buildScripts": {"enable": false}
  },
  "procMacro": {"enable": false},
  "checkOnSave": false
}
```

Select features/default-feature behavior/target from the resolved capsule identity. The
locked invocation requires an existing admitted Cargo.lock and an acquired dependency
closure; neither flags nor an empty result manufacture those prerequisites. These options
still permit trusted Cargo/rustc metadata work, so they are not substitutes for the sandbox.
`rust-analyzer-initialize-request.json` is a source/schema-backed sample, not an executed
Rust LSP request in this subtask. Only its schema command ran. Cleanup recorded no ordinary
or external containers and removed the exact private namespace keeper. No Phase 4 gate
status was changed by this upstream verification.

## Phase 4 Python requirement parser candidate — 2026-09-13

Context7 discovery for `pep508_rs` did not produce an exact library match; `/astral-sh/uv`
provided discovery leads only. The exact released crate archive, its manifest/source, and
an isolated Rust 1.98.1 executable probe establish the findings below. No dependency was
admitted, no duplicate-policy exception was added, and no upstream fork was created.
Artifacts are retained under `.dev-state/p4p/pep508-20260913/`. Retrieval occurred on
2026-09-13 local time (2026-09-14 UTC for the final probe).

| Claim | Source URL | Retrieved | Exact quote or measured evidence | Verdict | Selected pin | Blueprint reference |
|---|---|---|---|---|---|---|
| pep508_rs 0.9.2 is the current released candidate | [Registry metadata](https://crates.io/api/v1/crates/pep508_rs), [exact release archive](https://static.crates.io/crates/pep508_rs/pep508_rs-0.9.2.crate) | 2026-09-13 | Registry newest/max-stable was 0.9.2, published 2025-01-02, not yanked. Archive SHA-256 matched `faee7227064121fcadcd2ff788ea26f0d8f2bd23a0574da11eca23bc935bcc05`. | verified | none; candidate only | §5.2, §6 |
| The candidate shares the existing pep440_rs public type | [Exact release manifest](https://docs.rs/crate/pep508_rs/0.9.2/source/Cargo.toml), `capsule/Cargo.lock`, `api-probe.json` | 2026-09-13 | Manifest requires pep440_rs 0.7.2 with version-ranges enabled. Resolver selected the explicitly pinned 0.7.3; a compiled assignment to `&pep440_rs::VersionSpecifiers` and version 1.5/2.0 contains checks succeeded. | verified | none | §5.2 |
| Default features can eliminate the candidate's thiserror 1 dependency | Exact release manifest above; `dependency-conflicts.json` | 2026-09-13 | `default = []`; `thiserror = "1.0.59"` is unconditional. Probe resolution selected thiserror/thiserror-impl 1.0.69 alongside the baseline's 2.0.20. Both are unapproved duplicates under current deny.toml. | contradicted | none; admission stopped | §6, dependency policy |
| thiserror is the only additional version in the candidate closure | Saved baseline lock and seeded candidate lock; `dependency-conflicts.json` | 2026-09-13 | Additional versions were itertools 0.13.0, thiserror 1.0.69, and thiserror-impl 1.0.69. Existing itertools exceptions already cover that name; no other additional version was observed. This compares the candidate closure with the saved workspace lock, not a completed integrated cargo-deny gate. | contradicted | none | dependency policy |
| Rust-owned code can inspect registry requirements before acquisition | [Requirement API](https://docs.rs/pep508_rs/0.9.2/pep508_rs/struct.Requirement.html), exact archive `src/lib.rs`; `api-probe.json` | 2026-09-13 | Public fields are name, extras, version_or_url, marker, origin. `Requirement<url::Url>` FromStr succeeded; normalized Demo_Pkg/Speed.Test became demo-pkg/speed-test. VersionSpecifier and Url variants were distinguishable before any artifact fetch. | verified | none | §5.2, §6 |
| The default requirement URL type is free of host environment expansion | [Exact URL source](https://docs.rs/crate/pep508_rs/0.9.2/source/src/verbatim_url.rs), [URL trait](https://docs.rs/pep508_rs/0.9.2/pep508_rs/trait.Pep508Url.html) | 2026-09-13 | VerbatimUrl's Pep508Url implementation calls `expand_env_vars(url)`. The exercised `Requirement<url::Url>` uses URL parsing without that expansion. Reject all Url variants and parse errors; clear_url would instead erase the source restriction and must not be used for admission. | contradicted | none; explicit URL rejection verified | §6 |
| Requirement marker evaluation accepts an explicit capsule environment and owning-package extras | [Exact environment source](https://docs.rs/crate/pep508_rs/0.9.2/source/src/marker/environment.rs), Requirement API above; `capsule/src/env.rs`, `api-probe.json` | 2026-09-13 | MarkerEnvironmentBuilder supplies eleven borrowed-string fields and converts fallibly to MarkerEnvironment. The CPython 3.14.7/Linux/x86_64 probe activated only with parent-extra; passing the dependency's speed-test extra did not activate it. | verified | none | §5.2, §6 |
| Successful parsing guarantees silent, current-standard marker evaluation | [Exact marker source](https://docs.rs/crate/pep508_rs/0.9.2/source/src/marker/tree.rs), `api-probe.json`, [current comparison rules](https://packaging.python.org/en/latest/specifications/dependency-specifiers/#marker-comparisons) | 2026-09-13 | Actual os_name >= 'foo' under posix evaluated true with LexicographicComparison warnings; current String ordering guidance instead gives false. Ordinary string equality also emitted this warning. Parsing extras/dependency_groups/unknown_field examples failed. Preserve raw metadata and warnings; do not equate a successful parse or warning count with admission. | contradicted | none; unsupported semantics remain unresolved | §5.2, §6 |
| The candidate's minimum supported Rust version is established | Exact manifest above; `resolved-package-policy.json`, `api-probe.json` | 2026-09-13 | The crate declares no rust-version. The actual offline executable compiled and completed on stable 1.98.1 with exit 0. This proves that producer combination, not an undeclared minimum compiler version. | unverified | none | §6 |

The exact standalone manifest used pep508_rs `=0.9.2` with default features disabled,
pep440_rs `=0.7.3`, and url `=2.5.8`. Acquisition was a separate bounded Cargo fetch in the
service-private capsule; compilation/execution used `cargo +1.98.1 run --frozen` under
`--network=none`, non-root keep-id, a read-only image, and explicit resource bounds. Only
the service-authored parser fixture ran. URL fixtures used example.invalid and were never
fetched. Probe input digests and command/stdout/stderr/exit evidence are recorded in
`probe-input-digests.json` and `api-probe.json`. An initial assertion that all ordinary-marker
warnings would be absent failed; `initial-warning-assumption-failed.json` retains that
failure, and the corrected final probe records the warnings without hiding them.

Besides the three additional versions listed above, new registry package names were
pep508_rs 0.9.2, boxcar 0.2.14, rustc-hash 2.1.3, urlencoding 2.1.3, and version-ranges 0.1.3.
Their exact downloaded manifests declare license expressions already allowed by deny.toml;
boxcar and rustc-hash declare Rust 1.72 and 1.77 respectively, while the other three do not
declare a minimum. thiserror/thiserror-impl 1.0.69 declare Rust 1.61 and itertools 0.13.0
declares 1.43.1. This is manifest inspection, not a substitute for a future integrated
advisory/license/duplicate gate. Optional schemars, tracing, and non-pep508-extensions were
disabled; this does not remove the unconditional dependency conflicts.

The exercised warning-preserving APIs were Requirement::parse_reporter with a
`(MarkerWarningKind, String)` callback, and evaluate_markers_and_report with an explicit
environment plus selected owning-package extras. The latter returns activation plus a
warning list. The builder fields were implementation_name, implementation_version, os_name,
platform_machine, platform_python_implementation, platform_release, platform_system,
platform_version, python_full_version, python_version, and sys_platform. The source-only
Python environment probe recorded kernel-dependent values as well as interpreter identity;
Python version and CPU architecture alone do not determine the full environment.

The [current grammar](https://packaging.python.org/en/latest/specifications/dependency-specifiers/#complete-grammar)
and [January 2026 amendment](https://packaging.python.org/en/latest/specifications/dependency-specifiers/#history)
are the reference for any bounded core parser. String equality/containment is case-sensitive;
ordered String comparisons should reduce <=/>= to equality and </> to false. String ~= and
=== may be equality for installers, but rejecting them is conservative. Version fields use
PEP 440; containment is invalid. platform_release/platform_version are Version-or-String,
with String fallback on invalid versions and String containment. extra tests membership in
the owning package's selected extras; extras/dependency_groups are lock-file fields.
Unknown fields make admission unresolved, not an arbitrary boolean. These rules are not
fully implemented by this released candidate, so the candidate is not selected.

`cleanup.json` records no ordinary or external containers and removal of the exact owned
namespace keeper. No source, Cargo manifest, Cargo lock, or acceptance result was changed.

## Arrow/DataFusion target architecture 2026-09-14

Verification for [Plan 10](../plans/10-arrow-datafusion-architecture.md), including the user's
design-stage hard pivot. **Interface/source checks only:** no query benchmark, dependency
upgrade or target implementation ran for these rows. Context7 `/apache/datafusion` supplied
discovery leads; exact local registry sources and tagged upstream documentation supplied the
version-specific checks. `cargo metadata --locked --offline --format-version 1` confirmed the
resolved versions and features on 2026-09-14. Local paths below are relative to
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`.

| Claim | Primary source / exact local evidence | Retrieved | Short exact quote or observed declaration | Verdict / target implication |
|---|---|---|---|---|
| Resolved engine universe | `Cargo.lock`; executed Cargo metadata | 2026-09-14 | `datafusion 55.1.0`, `arrow 59.3.0`, `parquet 59.3.0`, `object_store 0.13.2` | verified resolved graph; no Delta package; these versions support the proposed interface set |
| Native prefix matching exposes an optimizer rewrite | [tagged starts_with source](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/functions/src/string/starts_with.rs); `datafusion-functions-55.1.0/src/string/starts_with.rs:158–184` | 2026-09-14 | `Convert starts_with(col, 'prefix') to col LIKE 'prefix%' with proper escaping` | verified source; prefer native Expr eligibility over an opaque custom predicate UDF |
| Constraints assert keys, not admission validity | [tagged constraint source](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/common/src/functional_dependencies.rs); `datafusion-common-55.1.0/src/functional_dependencies.rs:30–54` | 2026-09-14 | `does not check whether the argument is valid` | verified source; only PK/Unique variants; validate domains/FKs/duplicates before declaring constraints |
| Specialized scorer can operate on Arrow batches | [ScalarUDFImpl 55.1.0 rustdoc](https://docs.rs/datafusion-expr/55.1.0/datafusion_expr/trait.ScalarUDFImpl.html); `datafusion-expr-55.1.0/src/udf.rs:426–438` | 2026-09-14 | `pub number_rows: usize`; `pub return_field: FieldRef` | verified interface; a pure batch scorer may return a Struct through ColumnarValue, with native grouping/windowing after it; behavior must be tested |
| A shared runtime has memory, spill and cache controls | [tagged RuntimeEnv source](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/execution/src/runtime_env.rs); `datafusion-execution-55.1.0/src/runtime_env.rs:397–480` | 2026-09-14 | `with_memory_pool`; `with_max_temp_directory_size`; `with_metadata_cache_limit` | verified interface; share runtime across scoped request sessions and explicitly bound non-engine allocations |
| Memory pools do not account for every allocation | [tagged memory pool source](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/execution/src/memory_pool/mod.rs); `datafusion-execution-55.1.0/src/memory_pool/mod.rs:53–84` | 2026-09-14 | `Intermediate memory used as data streams through the system is not accounted` | verified limitation; batch/row/result bounds and RSS tests remain necessary |
| Parquet schema metadata can be discarded by the query reader | [tagged file format source](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/datasource-parquet/src/file_format.rs); `datafusion-datasource-parquet-55.1.0/src/file_format.rs:393–408`; [55.1 config](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/common/src/config.rs) | 2026-09-14 | `Schema::try_merge(clear_metadata(schemas))`; `pub skip_metadata: bool, default = true` | verified source/default; explicitly retain and validate schema metadata for admitted evidence |
| Expression metadata propagation is operation-specific | [tagged Expr schema source](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/expr/src/expr_schema.rs); `datafusion-expr-55.1.0/src/expr_schema.rs:488–500` | 2026-09-14 | `Binary expressions: field metadata is empty` | verified limitation; casts strip extension tags unless explicit target fields are supplied, and derived results need declared semantic fields |
| Sorting properties are declarations | [Parquet 59.3 writer properties source](https://docs.rs/parquet/59.3.0/src/parquet/file/properties.rs.html); `parquet-59.3.0/src/file/properties.rs:817–820` | 2026-09-14 | `self.sorting_columns = value;` | verified implementation; physically sort first, then declare truthful file/row-group ordering |
| Arrow canonical extensions are already resolved; Parquet JSON mapping is a separate feature | executed Cargo metadata; `arrow-59.3.0/Cargo.toml`; `parquet-59.3.0/src/arrow/schema/extension.rs:150–172` | 2026-09-14 | `#[cfg(feature = "arrow_canonical_extension_types")]`; `LogicalType::Json` | verified: Arrow/arrow-schema canonical features are enabled transitively; Parquet's feature is absent. Declare intended direct features explicitly and run dependency policy if enabling Parquet JSON mapping |
| Nested fields do not imply nested statistics pruning | `datafusion-pruning-55.1.0/src/pruning_predicate.rs:1230`; [tagged pruning source](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/pruning/src/pruning_predicate.rs) | 2026-09-14 | `PruningPredicate does not support pruning on nested fields yet.` | verified source limitation; nested types improve meaning/queryability; measure leaf decoding separately from row-group pruning |
| Catalogs and views provide namespacing; a catalog is not a prerequisite for a join | `datafusion-55.1.0/src/dataframe/mod.rs:1299–1320,1722`; `datafusion-catalog-55.1.0/src/memory/{catalog,schema}.rs`; [tagged DataFrame source](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/core/src/dataframe/mod.rs) | 2026-09-14 | `DataFrame::join`; `DataFrame::into_view` | verified interface; join combines plans and uses left session state; a view uses its consumer session. Bind functions/settings explicitly |

The remaining physical-type checks were read in `arrow-array-59.3.0/src/array/mod.rs:693`
(`StringArrayType`), `parquet-59.3.0/src/arrow/schema/mod.rs:775–855` (nested conversions), and
`parquet-59.3.0/src/arrow/array_reader/byte_array_dictionary.rs:68–76` (dictionary limitations).
Struct/List/Map representations are available; an empty Struct errors and Union writing is
unsupported. Dictionary encoding does not enforce enum values or stable dictionary codes.
These are interface constraints, not a target roundtrip result.

Delta non-adoption remains the target choice. The cached published 0.32.4 record and
[rust-v0.32.4 manifest](https://raw.githubusercontent.com/delta-io/delta-rs/rust-v0.32.4/Cargo.toml)
use Arrow/Parquet 58 and DataFusion 53.1. The inspected local git checkout at `1181cf7` uses
Arrow 59/DataFusion 55 together with the reviewed `buoyant/main` git dependencies. The original
review's 283→421 package count and dependency-policy failures are **earlier recorded
measurements**, not rerun in this planning check. Revisit only for a released compatible graph
that passes policy and a concrete transaction/storage need beyond the target manifest protocol.

### T2 synchronous Parquet writer controls 2026-09-14

Context7 `/apache/arrow-rs` supplied discovery leads. The following checks used the resolved
`parquet 59.3.0` and `arrow-array 59.3.0` registry sources, with tagged upstream sources as
primary references. **Interface/source checks only; no build, writer probe or memory
benchmark ran.** Local paths use the registry root defined above.

| Claim | Primary source / exact local evidence | Retrieved | Short exact quote or observed declaration | Verdict / implementation consequence |
|---|---|---|---|---|
| Row count and encoded-byte thresholds are independent controls | [59.3 writer properties](https://docs.rs/parquet/59.3.0/src/parquet/file/properties.rs.html); `parquet-59.3.0/src/file/properties.rs:736-775` | 2026-09-14 | `set_max_row_group_row_count`; `set_max_row_group_bytes` | verified interface; use positive `Some` limits. The old `set_max_row_group_size` is deprecated. Byte size is estimated, not a hard memory or encoded-file-size ceiling. |
| Byte thresholds can overshoot within a write | [59.3 ArrowWriter source](https://raw.githubusercontent.com/apache/arrow-rs/59.3.0/parquet/src/arrow/arrow_writer/mod.rs); `parquet-59.3.0/src/arrow/arrow_writer/mod.rs:358-430` | 2026-09-14 | `if in_progress.buffered_rows > 0` | verified implementation; proactive byte splitting uses prior average row size only when a group already contains rows. A fresh group's input is encoded before the threshold check. Bound each input batch and individual nested values before encoding. |
| Buffered-memory and encoded-size estimates serve different purposes | [59.3 ArrowWriter source](https://raw.githubusercontent.com/apache/arrow-rs/59.3.0/parquet/src/arrow/arrow_writer/mod.rs); `parquet-59.3.0/src/arrow/arrow_writer/mod.rs:305-337,1173-1201`; `parquet-59.3.0/src/column/writer/mod.rs:720-741` | 2026-09-14 | `memory_size`; `in_progress_size`; `in_progress_rows` | verified interface/source; check memory and encoded-size thresholds independently and flush early. Memory is an estimate of in-progress column storage, not total process memory; input arrays, encoder/flush transients and file-wide metadata need separate budgets. Checks after write cannot prevent transient overshoot. |
| Row-group flush, buffered I/O flush and durable sync are distinct | [59.3 ArrowWriter source](https://raw.githubusercontent.com/apache/arrow-rs/59.3.0/parquet/src/arrow/arrow_writer/mod.rs); `parquet-59.3.0/src/arrow/arrow_writer/mod.rs:440-463`; [File::sync_all](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all) | 2026-09-14 | `flush`; `sync`; `sync_all` | verified interface/source; ArrowWriter `flush` closes the current row group but does not flush the underlying writer. `sync` invokes I/O flush, not fsync. Finalize the footer and then call File::sync_all before publication; directory durability remains a separate publication step. |
| Footer finalization can retain or return File access | [59.3 file writer source](https://raw.githubusercontent.com/apache/arrow-rs/59.3.0/parquet/src/file/writer.rs); `parquet-59.3.0/src/arrow/arrow_writer/mod.rs:472-507`; `parquet-59.3.0/src/file/writer.rs:301-310,377-386,444-450` | 2026-09-14 | `finish`; `close`; `into_inner`; `inner` | verified interface/source; with ArrowWriter<File>, use `finish()?` then `inner().sync_all()?` when metadata is needed. Alternatively call `into_inner()?` directly, then sync the returned File. Do not finalize twice: finish followed by into_inner/close errors. |
| Zero-row typed tables still need a finalized schema-bearing file | [59.3 ArrowWriter source](https://raw.githubusercontent.com/apache/arrow-rs/59.3.0/parquet/src/arrow/arrow_writer/mod.rs); `parquet-59.3.0/src/arrow/arrow_writer/mod.rs:244-265,358-361,499-507` | 2026-09-14 | `batch.num_rows() == 0` | verified implementation; zero-row writes are no-ops. Construct the writer with the target SchemaRef and finalize even if the iterator yields no batches; the footer retains the schema with no data row groups. Do not use a zero-row write as schema validation because it returns before encoding. |
| Early row-group flushing does not bound file-wide metadata | [59.3 file writer source](https://raw.githubusercontent.com/apache/arrow-rs/59.3.0/parquet/src/file/writer.rs); `parquet-59.3.0/src/file/writer.rs:240-275,330-355`; [59.3 writer properties](https://docs.rs/parquet/59.3.0/src/parquet/file/properties.rs.html); `parquet-59.3.0/src/file/properties.rs:175-180,1049-1077` | 2026-09-14 | `row_groups.push(metadata)`; `BloomFilterPosition::AfterRowGroup` | verified implementation; row-group/page-index metadata survives until footer creation. Cap rows/row groups per file and roll to another exact manifest member; prefer after-row-group bloom placement to avoid retaining all bloom payloads. Data-page/dictionary-page byte limits are best effort, not heap limits. |

The bounded encoder must admit scalar byte lengths, nested child counts/depth, total row
bytes and total rows before allocating batches; a row-count-only batch limit permits one
arbitrarily large row. `RecordBatch::get_array_memory_size` is an additional conservative
physical-buffer check (`arrow-array-59.3.0/src/record_batch.rs:804-815`), not an exact allocator
measurement; shared buffers may be counted repeatedly. Slicing a huge prebuilt batch does not
establish bounded ingestion because its backing buffers remain alive. Use a disk File sink,
bounded producer batches, independent writer thresholds with encoding headroom, bounded
concurrent writers and bounded metadata per file. These controls are independent of
DataFusion's query memory pool; this source check does not certify a hard RSS ceiling.

### T2/T3 nested projection naming 2026-09-14

Context7 `/apache/datafusion` supplied discovery leads; the following statements were checked
against the resolved 55.1.0 registry source and tagged primary source. **Source checks only:**
the reported search-query failure and proposed narrow-ranking repair were not executed by
this verifier. A successful target regression remains necessary before claiming the repair.

| Claim | Primary source / exact local evidence | Retrieved | Short exact quote or observed declaration | Verdict / implementation consequence |
|---|---|---|---|---|
| Qualified/unqualified top-level name collisions are rejected | [55.1 DFSchema](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/common/src/dfschema.rs); `datafusion-common-55.1.0/src/dfschema.rs:240-267` | 2026-09-14 | `unqualified_names.contains(name)`; `SchemaError::AmbiguousReference` | verified source; an intermediate schema containing qualified `s.path` and unqualified `path` produces this error. This is distinct from merely having a nested payload field with the same spelling. Renaming one derived flag is not a general fix. |
| Leaf projection merging can add resolved pass-through columns beside aliases | [55.1 extraction optimizer](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/optimizer/src/extract_leaf_expressions.rs); `datafusion-optimizer-55.1.0/src/extract_leaf_expressions.rs:598-674` | 2026-09-14 | `if let Expr::Column(c) = e`; `Projection::try_new` | verified implementation; the existing-column set includes bare columns but excludes aliases. A needed reference resolved through a same-name alias can therefore add a qualified column beside the unqualified alias. This explains a possible ambiguity mechanism; it is not a traced reproduction of the reported query. Keep intermediate projections explicit and avoid carrying broad nested payloads through ranking joins. |
| Native nested-field extraction has deliberate window/schema restrictions | [55.1 extraction optimizer](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/optimizer/src/extract_leaf_expressions.rs); `datafusion-optimizer-55.1.0/src/extract_leaf_expressions.rs:166-184,1020-1066`; `datafusion-functions-55.1.0/src/core/getfield.rs:665-689`; `datafusion-common-55.1.0/src/config.rs:1756-1760` | 2026-09-14 | `MoveTowardsLeafNodes`; `enable_leaf_expression_pushdown` | verified source; column-based literal-key get_field expressions participate in leaf extraction, enabled by default. The extraction pass skips Window expressions because rewriting can change derived output names. Project required scalar score/order values before windows, retain explicit stable fact IDs, and hydrate nested provenance after bounded page selection. Keep optimizer rules enabled and verify the complete query shape; aliases alone are not certified to avoid every rewrite collision. |

### Retention ownership through native plans 2026-09-14

Context7 discovery and exact DataFusion 55.1.0 registry sources were checked before implementing
the lifetime boundary. The wrapper owns no data-selection expressions; native Parquet scans,
optimizer traversal, ordering, partitioning, statistics and predicates remain in DataFusion.

| Claim | Primary source | Retrieved | Exact interface / observed behavior | Verdict |
|---|---|---|---|---|
| Modern scan arguments must survive provider wrapping | [TableProvider](https://github.com/apache/datafusion/blob/55.1.0/datafusion/session/src/table.rs) | 2026-09-14 | `scan_with_args`, `ScanResult::into_inner` | interface checked; wrapper forwards full arguments then retains the lease in its returned plan |
| A unary ownership node can preserve native plan properties | [ExecutionPlan](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/execution_plan.rs), [BufferExec](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/buffer.rs) | 2026-09-14 | `replace_children`, `child_stats_requests`, `FilterDescription::from_children` | interface checked; replacement retains lease, input order and statistics; physical filters pass through |
| A stream adapter releases its captured inner stream at EOF | [stream implementation](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/stream.rs) | 2026-09-14 | `self.stream.take()` | source checked; explicit lease field keeps ownership until the stream object itself drops |
| Cleanup is excluded through actual native plan/stream lifetimes | `crates/enrichment-store/tests/repository.rs::cleanup_is_excluded_until_catalog_plan_and_exhausted_stream_are_dropped` | 2026-09-14 | passed in `.dev-state/plan10-native-regressions.log` | tested: cached bindings permit cleanup when idle; catalog, logical plan, optimized physical plan and exhausted stream each retain exclusion |

### T5 nested comparison and derived schemas 2026-09-14

The upstream verifier checked the resolved DataFusion 55.1.0 and Arrow 59.3.0 sources.
These findings required no dependency upgrade and no disabled optimizer rules.

| Claim | Primary source | Retrieved | Short exact quote | Implementation consequence |
|---|---|---|---|---|
| Ordered distinct aggregation supports nested observation values | [array aggregation](https://github.com/apache/datafusion/blob/55.1.0/datafusion/functions-aggregate/src/array_agg.rs#L174) | 2026-09-14 | `ArrayAgg` | Alias a constructed Struct first, then use that same value as the DISTINCT argument and sole ordering expression. Bound members per semantic key before aggregation. Compare actual observations; an outer-join-produced all-null Struct is still a value. |
| Nested equality is supported through DataFusion's datum comparison | [datum comparison](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-expr-common/src/datum.rs#L151) | 2026-09-14 | `compare_op` | Full outer joins with `IS DISTINCT FROM` can compare canonical nested sets. Inner list order remains meaningful; normalize only fields defined as sets. |
| Logical and physical UNION disagree about inherited relation metadata | [logical union](https://github.com/apache/datafusion/blob/55.1.0/datafusion/expr/src/logical_plan/plan.rs#L3406), [physical union](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/union.rs#L965) | 2026-09-14 | `intersect_metadata_for_union`; `let all_metadata_merged` | Reproduced with heterogeneous comparison and search axes. Declare a common derived index schema rather than inherit a source relation identity. Preserve base schemas and field semantics. |
| A logical projection alone does not preserve an explicit metadata boundary | [projection rewrite](https://github.com/apache/datafusion/blob/55.1.0/datafusion/optimizer/src/optimize_projections/mod.rs#L838), [physical projection](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/projection.rs#L152) | 2026-09-14 | `try_new_with_schema_metadata` | A private derived TableProvider delegates normal optimized view scanning, then reconstructs the physical output schema. Logical inlining cannot erase the boundary. This adds no custom execution plan. |
| An explicit target field can reconstruct physical string representation and metadata | [cast expression](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-expr/src/expressions/cast.rs#L147) | 2026-09-14 | `new_with_target_field` | The public `CastExpr` constructor preserves the requested field. The similarly named helper function is crate-private. Convert only the derived index when physical Utf8View differs from logical Utf8; keep Parquet string views enabled. |

Executed evidence on 2026-09-14: `native_comparison_preserves_nested_alternatives_and_pages_complete_keys`
passed with all six comparison scopes and empty configuration relations; all 16 daemon retrieval
fixtures passed with fresh typed publication and native search. Logs are recorded in STATUS.md.
These establish the exercised query behavior, not a completed performance or Phase 4–6 gate.

### T6 Cargo/rustdoc context and LSP protocol contracts 2026-09-14

Context7 `/rust-lang/cargo` and the LSP 3.17 specification library supplied discovery leads.
Exact Cargo source below is pinned to the commit reported by the installed dated producer.
Read-only version/help commands ran; no build, dependency acquisition, container or new LSP
request probe ran in this verification. The workstation binaries reported Cargo
`1.100.0-nightly (7941be6fb 2026-09-11)`, rustc
`1.100.0-nightly (809936eac 2026-09-12)`, rust-analyzer
`1.98.1 (48a229c 2026-09-01)`, and ty `0.0.80`.
`rustup target list --toolchain nightly-2026-09-13 --installed` reported only
`x86_64-unknown-linux-gnu`; this is host inventory, not producer-image qualification.

| Claim | Primary source / local evidence | Retrieved | Short exact quote or observed declaration | Verdict / implementation consequence |
|---|---|---|---|---|
| The dated producer has distinct Cargo and rustc identities | `cargo +nightly-2026-09-13 --version --verbose`; `rustc +nightly-2026-09-13 --version --verbose`; [Cargo commit](https://github.com/rust-lang/cargo/tree/7941be6fb416b4cd9666aef7b858dfea25587a8c); [rustc commit](https://github.com/rust-lang/rust/tree/809936eac66c547a5127ce1da805f0d3a6789b98) | 2026-09-14 | `commit-hash: 7941be6fb416b4cd9666aef7b858dfea25587a8c`; `commit-hash: 809936eac66c547a5127ce1da805f0d3a6789b98` | verified local identities; retain both with the admitted image identity. A host version check does not qualify the image's executable. |
| Target and feature selection are Cargo arguments before the rustdoc separator | [pinned cargo-rustdoc](https://raw.githubusercontent.com/rust-lang/cargo/7941be6fb416b4cd9666aef7b858dfea25587a8c/doc/book/src/commands/cargo-rustdoc.md); installed `cargo +nightly-2026-09-13 rustdoc --help` | 2026-09-14 | `--target`; `--features`; `--no-default-features`; `--all-features` | verified; supply the admitted target and feature policy to this invocation. Repeated features accumulate. Without feature flags Cargo enables defaults. docs.rs' expanded configuration is separate evidence, not a substitute for a requested consumer configuration. Restrict target input to the admitted target policy because Cargo also accepts target-specification paths. |
| Offline and unchanged resolution are separate constraints | [pinned cargo-rustdoc manifest options](https://raw.githubusercontent.com/rust-lang/cargo/7941be6fb416b4cd9666aef7b858dfea25587a8c/doc/book/src/commands/cargo-rustdoc.md); installed rustdoc-command help | 2026-09-14 | `--frozen`; `--locked`; `--offline` | verified; `--frozen` combines locked and offline behavior. Locked mode errors on a missing or changed lockfile; offline alone can resolve differently. Build against the captured lockfile with frozen mode, and preserve actual Cargo failures. |
| Fetch can establish the lockfile, but is not feature-selected compilation | [pinned cargo-fetch](https://raw.githubusercontent.com/rust-lang/cargo/7941be6fb416b4cd9666aef7b858dfea25587a8c/doc/book/src/commands/cargo-fetch.md); [pinned fetch implementation](https://raw.githubusercontent.com/rust-lang/cargo/7941be6fb416b4cd9666aef7b858dfea25587a8c/src/ops/cargo_fetch.rs); installed `fetch --help` | 2026-09-14 | `ops::resolve_ws(ws, dry_run)?`; `pub targets: Vec<String>` | verified; fetch accepts target selection, not feature flags. It generates a missing lockfile before acquisition. Use locked fetch for an already authoritative lock; otherwise record the newly resolved lock explicitly. Capture bounded lock bytes and digest before scratch removal, then supply that same lock to the frozen build. A lock digest does not replace the separate target/features/toolchain context. |
| JSON rustdoc uses one selected target and final rustdoc arguments | [pinned cargo-rustdoc](https://raw.githubusercontent.com/rust-lang/cargo/7941be6fb416b4cd9666aef7b858dfea25587a8c/doc/book/src/commands/cargo-rustdoc.md); [pinned rustdoc configuration](https://raw.githubusercontent.com/rust-lang/rust/809936eac66c547a5127ce1da805f0d3a6789b98/src/librustdoc/config.rs); installed `rustdoc +nightly-2026-09-13 -Z unstable-options --help` | 2026-09-14 | `cargo rustdoc`; `IrJson`; `--output-format=json` | verified interface; retain `--lib` when forwarding `-- -Z unstable-options --output-format json`. Additional arguments go to the final target invocation. Inspect emitted `format_version`; a compiler identity alone is not parser compatibility proof. |
| LSP cancellation identifies a pending request and is best effort | [LSP 3.17 cancellation](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/specification.md); [Rust 1.98.1 handler](https://raw.githubusercontent.com/rust-lang/rust/1.98.1/src/tools/rust-analyzer/crates/rust-analyzer/src/handlers/notification.rs); [analyzer request queue integration](https://raw.githubusercontent.com/rust-lang/rust/1.98.1/src/tools/rust-analyzer/crates/rust-analyzer/src/global_state.rs) | 2026-09-14 | `$/cancelRequest`; `RequestCancelled: integer = -32800`; `state.cancel(id)` | verified protocol and analyzer source; send a notification with the original integer/string request ID. Cancellation can be ignored and still permits a response. Bound cancellation writes; either drain late responses by ID or discard a timed-out session. A partly read/written frame is not a reusable protocol boundary. |
| Diagnostic coordinates use the negotiated encoding | [LSP Position](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/types/position.md); [Rust 1.98.1 encoding negotiation](https://raw.githubusercontent.com/rust-lang/rust/1.98.1/src/tools/rust-analyzer/crates/rust-analyzer/src/lsp/capabilities.rs) | 2026-09-14 | `PositionEncodingKind`; `PositionEncoding::Utf8` | verified; line and character are zero-based, with UTF-16 the protocol default. The pinned analyzer selects offered UTF-8, so the server cannot be assumed to stay on UTF-16. Preserve returned encoding and convert non-ASCII positions for both queries and diagnostics. |
| Pull diagnostics have full and unchanged reports | [LSP pull diagnostics](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/language/pullDiagnostics.md); [Diagnostic fields](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/types/diagnostic.md) | 2026-09-14 | `items: Diagnostic[]`; `resultId: string`; `code?: integer \| string` | verified protocol; full reports contain items, while unchanged reports refer to an existing result ID. Do not decode unchanged as an empty result. Range and message are required; severity, code, source and related information are optional. A pull capability does not imply workspace diagnostic support. |
| Published diagnostics replace a resource's prior diagnostic set | [LSP published diagnostics](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/language/publishDiagnostics.md) | 2026-09-14 | `uri: DocumentUri`; `version?: integer`; `diagnostics: Diagnostic[]` | verified protocol; retain URI, optional document version and the bounded set. An empty published array clears prior diagnostics; absence of a notification is not a successful empty report. Interleaved diagnostic notifications must be routed if the service promises their preservation. |

ty's installed version was rechecked; its release-exact request-cancellation implementation
was not established in this bounded check. Existing initialize captures are earlier evidence,
not new probe results. Neither a successful open notification nor an empty navigation response
establishes indexing readiness or dependency completeness. These source checks do not close
producer, containment, cancellation or acceptance gates.

### T6 nullable execution locations in native UNNEST 2026-09-14

Context7 `/apache/arrow-rs` supplied discovery leads. The resolved Arrow/Parquet 59.3.0
and DataFusion 55.1.0 registry source and tagged primary source were inspected; this verifier
ran no build or query. The original failure was reported during execution-location admission;
its log has since been replaced by the successful rerun, so the mechanism below is a
source-grounded explanation, not a newly captured failing backtrace.

| Claim | Primary source / exact local evidence | Retrieved | Short exact quote or observed declaration | Verdict / implementation consequence |
|---|---|---|---|---|
| Required Struct children are legal when their nulls are covered by that Struct's mask | [Arrow 59.3 StructArray](https://github.com/apache/arrow-rs/blob/59.3.0/arrow-array/src/array/struct_array.rs); `arrow-array-59.3.0/src/array/struct_array.rs:127-186,590-596` | 2026-09-14 | `n.contains(&a)`; `Self::new(fields.into(), arrays, Some(nulls))` | verified invariant; nullable parents do not generally require nullable child fields. Validation is local to each Struct: an outer ancestor's mask cannot repair an inner Struct missing its own mask. The take tuple constructor supplies its mask correctly. |
| Parquet's selective list padding retains leaf masks without the ordinary required-leaf guard | [Parquet 59.3 record reader](https://github.com/apache/arrow-rs/blob/59.3.0/parquet/src/arrow/record_reader/mod.rs); [reader builder](https://github.com/apache/arrow-rs/blob/59.3.0/parquet/src/arrow/array_reader/builder.rs); `parquet-59.3.0/src/arrow/record_reader/mod.rs:246-275,289-332`; `array_reader/builder.rs:327-340` | 2026-09-14 | `consume_compact_bitmap`; `if self.column_desc.self_type().is_optional()` | verified source; list readers propagate a definition-level padding threshold to descendants. The compact-mask branch retains item-level nulls even for required leaves; only the ordinary branch discards required-leaf masks. A missing optional ancestor can therefore produce null line/byte leaves inside a required Position. |
| A required inner Struct can be reconstructed without a mask before UNNEST validates it | [Parquet Struct reader](https://github.com/apache/arrow-rs/blob/59.3.0/parquet/src/arrow/array_reader/struct_array.rs); [DataFusion 55.1 UNNEST](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/unnest.rs); [Arrow 59.3 take](https://github.com/apache/arrow-rs/blob/59.3.0/arrow-select/src/take.rs); `parquet-59.3.0/src/arrow/array_reader/struct_array.rs:117-155`; `datafusion-physical-plan-55.1.0/src/unnest.rs:1299-1330`; `arrow-select-59.3.0/src/take.rs:272-300` | 2026-09-14 | `if self.nullable`; `StructArray::new_unchecked_with_length`; `StructArray::from((fields, is_valid))` | verified source chain matching the reported panic: the Parquet reader supplies no bitmap for a required Struct; list UNNEST recursively takes its nested values, invoking checked Struct reconstruction. This is a pinned nested-list compatibility limitation, not evidence that custom semantic metadata was discarded. |
| Nullable physical location descendants can retain required domain values through validation | `crates/enrichment-store/src/projection/execution.rs:23-82`; `.dev-state/plan10-execution-roundtrip.log` | 2026-09-14 | `r.number("line")?`; `r.structure("start")?` | inspected application treatment: declare physical start/end/line/byte fields nullable while retaining required decoding for every present range/position. Record this compatibility distinction in the governing ADR; do not weaken domain validation, remove field semantic metadata, disable optimizer rules or claim all Arrow structs need this layout. The existing rerun records both execution tests passed; this verifier did not execute them. |

### T6 Python runtime-object observation interfaces 2026-09-14

Context7 `/python/cpython` supplied discovery leads for blueprint source S20. Exact
CPython `v3.14.7` source was checked, including `Include/patchlevel.h` declaring
`PY_VERSION "3.14.7"`; this is source-version evidence, not a new producer-image probe.
The adapter and ADR-0026 were read. No interpreter test, container or build ran.

| Claim | Primary source | Retrieved | Short exact quote or observed declaration | Verdict / implementation consequence |
|---|---|---|---|---|
| The three requested signature options are supported together | [3.14.7 signature entry point](https://github.com/python/cpython/blob/v3.14.7/Lib/inspect.py#L3329); [inspect documentation](https://github.com/python/cpython/blob/v3.14.7/Doc/library/inspect.rst) | 2026-09-14 | `follow_wrapped=True`; `eval_str=False`; `annotation_format=Format.VALUE` | verified; the adapter can pass `False`, `False`, and `annotationlib.Format.STRING`. Disabling wrapper following skips `__wrapped__` traversal; disabling string evaluation suppresses annotation un-stringizing, not all execution. |
| STRING annotation inspection remains effectful and approximate | [annotation execution](https://github.com/python/cpython/blob/v3.14.7/Lib/annotationlib.py#L718); [STRING selection](https://github.com/python/cpython/blob/v3.14.7/Lib/annotationlib.py#L1009); [limitations and security](https://github.com/python/cpython/blob/v3.14.7/Doc/library/annotationlib.rst) | 2026-09-14 | `return annotate(format)`; `return annotations_to_string(annotate(Format.VALUE))` | verified; STRING invokes annotation hooks, may fall back to VALUE, and can execute or raise while reconstructing expressions. It approximates annotation source; it is not exact source recovery or an effect-free mode. ADR-0026's explicit runtime intent and capsule containment remain necessary. |
| A displayed signature is introspection evidence, including object-provided overrides | [signature override](https://github.com/python/cpython/blob/v3.14.7/Lib/inspect.py#L2466); [default formatting](https://github.com/python/cpython/blob/v3.14.7/Lib/inspect.py#L2792) | 2026-09-14 | `sig = obj.__signature__`; `repr(self._default)` | verified; `follow_wrapped=False` still honors a supplied Signature and does not force its annotations through STRING conversion. Rendering defaults can invoke custom representation code. Some callables lack signatures; a returned signature does not prove a call will succeed. Treat it as an optional observed representation. |
| Actual object type is distinct from its selected public path | [3.14.7 type implementation](https://github.com/python/cpython/blob/v3.14.7/Objects/typeobject.c#L2331); [built-in type documentation](https://github.com/python/cpython/blob/v3.14.7/Doc/library/functions.rst) | 2026-09-14 | `Py_TYPE(PyTuple_GET_ITEM(args, 0))` | verified; `type(value)` returns the actual runtime type. For a class value this is its metaclass. The adapter's module/qualname string is an observed type label, not an immutable identity or an exhaustive function/class/callability classification. |
| Selected attributes and docstrings use dynamic lookup | [inspect attribute lookup documentation](https://github.com/python/cpython/blob/v3.14.7/Doc/library/inspect.rst) | 2026-09-14 | `getattr_static` | verified; ordinary `getattr` can execute descriptors and custom attribute hooks. The adapter deliberately observes resolved runtime values and `__doc__`; substituting static lookup would change that scope. It does not call `inspect.getdoc` or promise inherited/cleaned documentation. |
| dir results are customizable observations and are materialized before truncation | [3.14.7 dir implementation](https://github.com/python/cpython/blob/v3.14.7/Objects/object.c#L2156); [dir documentation](https://github.com/python/cpython/blob/v3.14.7/Doc/library/functions.rst) | 2026-09-14 | `PySequence_List(result)`; `PyList_Sort(sorted)` | verified; `__dir__` runs, then its result is fully materialized and sorted. Names may be incomplete or inaccurate; their presence does not prove successful access. The adapter's 1024-name slice bounds retained count, not enumeration memory/time or individual name length. Capsule and bounded output/admission controls still own those limits. |

ADR-0026 correctly classifies these operations as explicit effectful runtime observations;
none of these APIs upgrades observations into complete source/stub or behavioral contracts.

### T6 LSP readiness and progress 2026-09-14

Context7 `/rust-lang/rust-analyzer` and `/astral-sh/ty` supplied discovery leads. Rust
`1.98.1` analyzer source was checked against the documented producer pin. The ty `0.0.80`
tag's `ruff` gitlink was re-read as `e7230cac059fa28bd0e534e4571c3560c695efbe`, and the
server sources below use that commit. No server, container or test ran in this check.

| Claim | Primary source | Retrieved | Short exact quote or observed declaration | Verdict / implementation consequence |
|---|---|---|---|---|
| Rust-analyzer status requires a client capability | [1.98.1 client capabilities](https://github.com/rust-lang/rust/blob/1.98.1/src/tools/rust-analyzer/crates/rust-analyzer/src/lsp/capabilities.rs#L453); [extension parameters](https://github.com/rust-lang/rust/blob/1.98.1/src/tools/rust-analyzer/crates/rust-analyzer/src/lsp/ext.rs#L527) | 2026-09-14 | `self.experimental_bool("serverStatusNotification")`; `experimental/serverStatus` | verified; set `initialize.params.capabilities.experimental.serverStatusNotification=true`, not an initialization option. Retain notification health (`ok`, `warning`, `error`), quiescent boolean and optional/null message. No dynamic registration or progress token is required for this extension. |
| The emitted quiescent flag includes cache priming readiness | [1.98.1 readiness computation](https://github.com/rust-lang/rust/blob/1.98.1/src/tools/rust-analyzer/crates/rust-analyzer/src/reload.rs#L68); [status construction](https://github.com/rust-lang/rust/blob/1.98.1/src/tools/rust-analyzer/crates/rust-analyzer/src/reload.rs#L132) | 2026-09-14 | `quiescent: self.is_fully_ready()` | verified; readiness requires VFS/workspace/build-data/proc-macro loading and discovery to settle, and no cache-priming operation in progress. Keep health separate: a quiescent server can still report missing-workspace/dependency/configuration problems. This is readiness within configured scope, not complete semantic coverage. |
| Rust-analyzer sends status changes, not a per-document acknowledgement | [1.98.1 status emission](https://github.com/rust-lang/rust/blob/1.98.1/src/tools/rust-analyzer/crates/rust-analyzer/src/main_loop.rs#L814) | 2026-09-14 | `if self.last_reported_status != status` | verified; begin with unknown status, retain the latest state and continue consuming interleaved notifications. Bound the initial/busy wait. Do not require a fresh true notification after every didOpen when status remains unchanged. Unknown/busy at the bound supports incomplete indexing, not absence. |
| Standard progress tracks individual operations | [LSP 3.17 progress](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/types/workDoneProgress.md); [progress creation](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/window/workDoneProgressCreate.md) | 2026-09-14 | `window.workDoneProgress`; `workDoneToken`; `kind: 'end'` | verified; advertise `capabilities.window.workDoneProgress=true` only with a bounded handler for `window/workDoneProgress/create` returning `result:null`. Track integer/string tokens and begin/report/end separately from partial results. A progress end or an empty active-token set does not certify global readiness. |
| ty initializes workspaces before dispatching ordinary messages | [ty 0.0.80 gitlink](https://api.github.com/repos/astral-sh/ty/git/trees/0.0.80?recursive=1); [message deferral](https://github.com/astral-sh/ruff/blob/e7230cac059fa28bd0e534e4571c3560c695efbe/crates/ty_server/src/session.rs#L414); [configuration fallback](https://github.com/astral-sh/ruff/blob/e7230cac059fa28bd0e534e4571c3560c695efbe/crates/ty_server/src/session.rs#L876) | 2026-09-14 | `self.workspaces.all_initialized()` | verified; requests and document notifications are deferred until workspace initialization completes. With the adapter's `workspace.configuration=false`, ty initializes from initializationOptions directly. The initialize response itself precedes this setup. No dedicated global-ready notification was established in the inspected capabilities/session/dispatcher/progress sources; do not invent one. |
| ty progress is optional, and workspace diagnostics can intentionally long-poll | [lazy progress](https://github.com/astral-sh/ruff/blob/e7230cac059fa28bd0e534e4571c3560c695efbe/crates/ty_server/src/server/lazy_work_done_progress.rs#L105); [workspace diagnostics](https://github.com/astral-sh/ruff/blob/e7230cac059fa28bd0e534e4571c3560c695efbe/crates/ty_server/src/server/api/requests/workspace_diagnostic.rs#L78) | 2026-09-14 | `supports_work_done_progress()`; `Action::SuspendWorkspaceDiagnostics` | verified; ty uses request tokens or creates a server token, with reporting unavailable before creation acknowledgement. Workspace diagnostic requests can remain open after checking when reports are empty/unchanged. Their timeout is not proof of incomplete indexing; do not use workspace diagnostics as a readiness barrier. |
| A ty null navigation result or empty diagnostic report has multiple causes | [definition handler](https://github.com/astral-sh/ruff/blob/e7230cac059fa28bd0e534e4571c3560c695efbe/crates/ty_server/src/server/api/requests/goto_definition.rs); [document diagnostic handler](https://github.com/astral-sh/ruff/blob/e7230cac059fa28bd0e534e4571c3560c695efbe/crates/ty_server/src/server/api/requests/diagnostic.rs) | 2026-09-14 | `return Ok(None)`; `diagnostic_mode().is_off()` | verified; disabled services, missing file/position and unresolved navigation can return null. Document diagnostics can return empty when disabled or unavailable. A bounded document diagnostic response can support analysis of that known enabled document; it is not proof of complete indexing or dependency resolution. Preserve explicit unresolved/unknown scope and method limitations. |

Minimal strategy: retain readiness/progress alongside diagnostic state, start the ordinary LSP
handshake, send initialized and the versioned document, and process notifications during all
bounded exchanges. For rust-analyzer, wait for known quiescence and qualify health before
accepting an empty result. For ty, rely on its verified initialization deferral and completed
document-scoped analysis under known enabled settings; qualify unresolved inputs and any
unestablished search coverage. Where that evidence is insufficient, retain an incomplete empty
observation. Neither a didOpen write, a fixed quiet interval nor an initialize response supplies
the missing evidence. The inspected adapter's open-document check alone does not establish it.

### Revision-tree declarations versus distribution metadata 2026-09-14

Context7 `/pypa/packaging.python.org` supplied discovery leads. PyPA's current Core Metadata
specification identifies itself as version 2.6, approved May 2026; the required identity fields
below predate that version. This was a documentation/source inspection, with no backend,
interpreter test, container or dependency change.

| Claim | Primary source | Retrieved | Short exact quote | Verdict / implementation consequence |
|---|---|---|---|---|
| Completed Core Metadata requires identity fields | [Core Metadata](https://packaging.python.org/en/latest/specifications/core-metadata/); [name syntax](https://packaging.python.org/en/latest/specifications/name-normalization/#name-format) | 2026-09-14 | `Metadata-Version`; `Name`; `Version` | verified; keep required-header and name/version syntax validation for admitted distributions. Nullable observation fields do not make malformed present metadata valid. Preserve literal observed values with their source; compare names/versions using their specified normalization rules. |
| A VCS source tree is distinct from a packaged source distribution | [PEP 517 terminology](https://peps.python.org/pep-0517/#terminology-and-goals); [sdist contents](https://packaging.python.org/en/latest/specifications/source-distribution-format/#source-distribution-file-format); [wheel contents](https://packaging.python.org/en/latest/specifications/binary-distribution-format/#file-contents) | 2026-09-14 | "A source tree is something like a VCS checkout."; `PKG-INFO`; `METADATA` | verified distinction; a modern sdist requires PKG-INFO and a wheel contains METADATA. Service inference: a revision inventory without generated distribution metadata can retain absent name/version observations as null, while immutable revision/artifact identity remains available independently. This is not permission to accept a malformed wheel or sdist as complete. |
| pyproject name is static while version may be deferred | [pyproject project table](https://packaging.python.org/en/latest/specifications/pyproject-toml/#declaring-project-metadata-the-project-table); [dynamic rules](https://packaging.python.org/en/latest/specifications/pyproject-toml/#dynamic) | 2026-09-14 | `name`; `version`; `dynamic` | verified; when [project] exists, name must be static and version must be a literal or listed in dynamic. Version absent from both is malformed project metadata; a scalar version cannot be both static and dynamic. Without [project], the backend supplies metadata through its own conventions. Missing METADATA does not negate a separately sourced literal project name. |
| pyproject dynamic version does not permit Dynamic: Version in Core Metadata | [pyproject dynamic](https://packaging.python.org/en/latest/specifications/pyproject-toml/#dynamic); [Core Metadata Dynamic](https://packaging.python.org/en/latest/specifications/core-metadata/#dynamic-multiple-use) | 2026-09-14 | "may not be specified in this field" | verified; Name, Version and Metadata-Version cannot be listed in the Core Metadata Dynamic header. A source-tree dynamic version must become a concrete required version by distribution metadata production. Static extraction may report its unresolved value without executing the backend. |
| Reading declarations and producing metadata are different operations | [pyproject static metadata](https://packaging.python.org/en/latest/specifications/pyproject-toml/#declaring-project-metadata-the-project-table); [PEP 517 metadata hook](https://peps.python.org/pep-0517/#prepare-metadata-for-build-wheel) | 2026-09-14 | `prepare_metadata_for_build_wheel` | verified; parsing a literal pyproject field needs no backend invocation, while the metadata hook executes backend code. Keep such literals separately source-qualified, or leave unobserved distribution headers null. Never replace a revision's missing declared distribution version with its commit/archive name. The sdist specification permits interpretation of conforming distribution filenames; the service's stronger selected-artifact header corroboration remains its explicit evidence policy. |

Proposed model consequence: `Option<String>` represents an absent observed declaration;
`Some("")` or another malformed present declaration must not silently become absence. Preserve
the header/declaration authority and existing strict PyPI artifact admission. ADR-0015's
validated static package-name selection remains separate from optional generated metadata;
its commit-based revision release key is not a declared Python distribution version.

### LSP line endings and exact retained source coordinates 2026-09-14

Context7 `/websites/microsoft_github_io_language-server-protocol_specifications_lsp_3_17`
supplied discovery leads. Both versioned specification sources were read directly. Microsoft's
reference TextDocument source and tests were inspected at commit
`5010cdf9822e1038a30ee7eb6ee5d7aaa79acc4a`; that implementation uses UTF-16 string offsets,
so its absolute offsets must not be copied as UTF-8 byte offsets. No implementation, test,
container or dependency change was made by this verification.

| Claim | Primary source | Retrieved | Short exact quote or declaration | Verdict / implementation consequence |
|---|---|---|---|---|
| Both protocol versions recognize three line-ending forms | [LSP 3.17 text documents](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/types/textDocuments.md); [LSP 3.18 text documents](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.18/types/textDocuments.md) | 2026-09-14 | `export const EOL: string[] = ['\n', '\r\n', '\r'];` | verified; CRLF is one delimiter, standalone CR and LF each delimit a line. Positions exclude the delimiter's characters. Splitting only on LF and trimming all trailing CR contradicts this contract: `a\r\r\nb` has logical contents `["a", "", "b"]`. Preserve original bytes; Unicode NEL and line/paragraph separators are not additional LSP delimiters. |
| Empty documents and terminal delimiters retain an empty logical line | [Microsoft scanner](https://github.com/microsoft/vscode-languageserver-node/blob/5010cdf9822e1038a30ee7eb6ee5d7aaa79acc4a/textDocument/src/main.ts#L499); [reference tests](https://github.com/microsoft/vscode-languageserver-node/blob/5010cdf9822e1038a30ee7eb6ee5d7aaa79acc4a/textDocument/src/test/textdocument.test.ts#L18) | 2026-09-14 | `computeLineOffsets`; `result.push(textOffset + i + 1)` | verified reference behavior; initialize the first line even for empty text, consume only the LF immediately following CR, and record a new line after each delimiter including at EOF. Repeated CRs are separate delimiters unless a CR is immediately followed by LF. |
| Negotiated offsets count encoding units, not displayed glyphs | [3.17 Position](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/types/position.md); [3.18 Position](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.18/types/position.md); [encoding negotiation](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.18/types/textDocuments.md) | 2026-09-14 | `UTF8`; `UTF16`; `UTF32` | verified; UTF-8 counts bytes, UTF-16 counts code units, UTF-32 counts code points. UTF-16 remains mandatory and is the omitted-negotiation default. Validate the adapter's supported encoding before any zero-column early return. Reject a UTF-16 surrogate-interior or UTF-8 scalar-interior offset when exact conversion into the retained scalar-boundary contract is impossible; this is the service's exact-evidence boundary, not an upstream guarantee that every code-unit offset is a scalar boundary. |
| Oversized character offsets and nonexistent lines have distinct evidence | [3.17 Position prose](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/types/position.md); [3.18 Position prose](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.18/types/position.md); [3.17 metaModel](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/metaModel/metaModel.json); [3.18 metaModel](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.18/metaModel/metaModel.json); [reference Position and offsetAt](https://github.com/microsoft/vscode-languageserver-node/blob/5010cdf9822e1038a30ee7eb6ee5d7aaa79acc4a/textDocument/src/main.ts#L327) | 2026-09-14 | "If the character value is greater than the line length it defaults back to the line length." | verified with source inconsistency; both prose versions specify character clamping but omit line-number clamping. The 3.17 metaModel adds line clamping; the 3.18 metaModel removes both clamp notes. Microsoft's reference clamps oversized positions and labels these policies implementation specific. Do not present universal rejection or universal line clamping as an unqualified 3.17/3.18 requirement. Keep strict retained-coordinate validation separate from any explicitly chosen protocol normalization, and record a conservative rejection as service admission policy. |
| A range includes a delimiter by ending on the following line | [3.17 Range](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/types/range.md); [3.18 Range](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.18/types/range.md) | 2026-09-14 | "The end position is exclusive." | verified; line-content end is a valid position, delimiter-interior positions are not separately expressible, and a range that includes the delimiter ends at the next line's column zero. Exact retained ranges must continue to validate endpoint ordering independently. |

Concrete correction/oracle: share one scanner of original UTF-8 bytes across retained position
validation, automatic anchors and both LSP conversions. Expose content start/end and delimiter
end; recognize CRLF before individual CR/LF and retain the final line. For
`"a\r\r\n🌎é\r\n"`, logical contents are `["a", "", "🌎é", ""]`, with byte starts
`[0, 2, 4, 12]`. On line 2, scalar-boundary byte columns `[0, 4, 6]` correspond to UTF-16
columns `[0, 2, 3]`; UTF-16 column 1 is not exactly representable in the retained contract.
The empty document has `(0, 0)`; the fixture's final empty line has `(3, 0)`. Add explicit
oversized-column expectations for whichever documented protocol policy is selected, while
retained UTF-8 coordinates remain strict. Rust `str::lines()` alone is not this scanner.

### Plan 11 W7 executed-plan diagnostics and source-file opening 2026-09-14

Context7 `/apache/datafusion` and `/websites/doc_rust-lang_stable_std` supplied discovery
leads. Exact API evidence below is the installed Cargo registry source for DataFusion
55.1.0 and Arrow 59.3.0, and the installed Rust 1.98.1 standard-library source. Public source
links use the corresponding release tags; Linux flag values were also read from this host's
`/usr/include/x86_64-linux-gnu/bits/fcntl-linux.h`. No builds, tests or containers ran, and no
implementation or dependency files changed in this verification.

| Claim | Primary source | Retrieved | Short exact quote or declaration | Verdict / implementation consequence |
|---|---|---|---|---|
| Preserve the DataFrame session snapshot and optimize once | [55.1.0 DataFrame parts](https://github.com/apache/datafusion/blob/55.1.0/datafusion/core/src/dataframe/mod.rs#L1690); [SessionState planning](https://github.com/apache/datafusion/blob/55.1.0/datafusion/core/src/execution/session_state.rs#L684); [default physical planner](https://github.com/apache/datafusion/blob/55.1.0/datafusion/core/src/physical_planner.rs#L154) | 2026-09-14 | `pub fn into_parts(self) -> (SessionState, LogicalPlan)`; `self.optimize_physical_plan(plan, session_state, \|_, _\| {})` | verified; obtain `(state, logical)`, call `state.optimize(&logical)?`, retain that optimized plan for diagnostics, then `state.query_planner().create_physical_plan(&optimized, &state).await?`. This mirrors SessionState's normal path while preserving the exact optimized input and enabled physical rules. `state.task_ctx()` returns `Arc<TaskContext>`. Avoid discarding the attached state through `into_optimized_plan`, whose docs restrict that convenience to testing. |
| Stream the retained physical Arc rather than rebuilding it | [55.1.0 DataFrame execution](https://github.com/apache/datafusion/blob/55.1.0/datafusion/core/src/dataframe/mod.rs#L1601); [physical streaming](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/execution_plan.rs#L1772) | 2026-09-14 | `pub fn execute_stream(`; `plan: Arc<dyn ExecutionPlan>` | verified; `datafusion::physical_plan::execute_stream(Arc::clone(&plan), task_ctx)` returns `Result<SendableRecordBatchStream>` synchronously; poll batches asynchronously. Calling `frame.execute_stream()` after creating a separate diagnostic plan would build another physical plan. Retain the executed Arc through completion/error handling and metric capture, then release it and its leases. |
| Single-stream execution can add an otherwise unretained root | [55.1.0 execute_stream branches](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/execution_plan.rs#L1776); [CoalescePartitionsExec](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/coalesce_partitions.rs#L63) | 2026-09-14 | `CoalescePartitionsExec::new(Arc::clone(&plan))` | verified; for more than one output partition, the helper creates a local coalescing root. If diagnostics must include that operator, explicitly wrap and retain it before calling the helper. Import `ExecutionPlanProperties` for `output_partitioning().partition_count()`. Zero-partition plans return an empty typed stream without executing a node; absent metrics must remain absent. |
| Metrics are per-node shared observations, not frozen copies | [55.1.0 ExecutionPlan metrics](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/execution_plan.rs#L702); [MetricsSet](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-expr-common/src/metrics/mod.rs#L217) | 2026-09-14 | `fn metrics(&self) -> Option<MetricsSet>`; `pub fn iter(&self) -> impl Iterator<Item = &Arc<Metric>>` | verified; walk the retained root and its children after stream completion/drop. Copy values into bounded owned diagnostics: MetricsSet clones retain shared mutable metric values. Preserve node identity/path, `Metric::partition()`, `labels()`, `value()` and its name/type. A metric absent on an unexecuted/operator-specific path is not zero. Error/cancellation snapshots are partial observations; dropping a stream is the documented abort mechanism, not proof every counter is final immediately. |
| Pruning and ratio metrics require structured extraction | [55.1.0 MetricValue](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-expr-common/src/metrics/value.rs#L626); [Parquet file metrics](https://github.com/apache/datafusion/blob/55.1.0/datafusion/datasource-parquet/src/metrics.rs#L105) | 2026-09-14 | `Self::PruningMetrics { .. } => 0`; `Self::Ratio { .. } => 0` | verified; blanket `as_usize()` extraction would fabricate zero pruning/ratio observations. Match `PruningMetrics` and read `pruned()/matched()`; match `Ratio` and read `part()/total()` with its merge strategy. Retain per-file labels and units under independent limits. Available scan fields include statistics/bloom row-group pruning, page-index pruning, bytes_scanned and predicate errors. Sum only compatible partitions/operators; node output counts are not additive query output, and elapsed_compute is not classical CPU time or request wall time. |
| Public formatters need application-enforced diagnostic bounds | [55.1.0 logical display](https://github.com/apache/datafusion/blob/55.1.0/datafusion/expr/src/logical_plan/plan.rs#L1743); [physical display](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/display.rs#L155) | 2026-09-14 | `pub fn indent(&self, verbose: bool) -> impl fmt::Display + 'a`; `pub fn one_line(&self) -> impl fmt::Display + 'a` | verified; `optimized.display_indent()` and `DisplayableExecutionPlan::new(plan.as_ref()).indent(false)` are formatting interfaces, not byte/node/depth caps. `with_metrics` aggregates; `with_full_metrics` preserves low-level rendering. Write directly into a capped `fmt::Write` sink and record truncation; do not allocate a complete String and truncate afterward. For structural limits, perform a bounded walk and use logical `display()` or physical `one_line()` per node. A capped sink bounds retained output, not all internal formatter allocations or synchronous CPU time. |
| The pinned physical visitor does not honor its boolean stop contract | [55.1.0 visitor source](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/visitor.rs#L24) | 2026-09-14 | `visitor.pre_visit(plan)?;`; `visitor.post_visit(plan)?;` | contradicted implementation/documentation assumption; `accept` and `visit_execution_plan` discard the returned bool despite the trait docs describing `Ok(false)` as a stop. Do not depend on it for budgets. An error sentinel propagates, or a bounded iterative traversal over `children()` avoids this issue and recursive stack growth. This is a scoped adapter implementation choice, not grounds to disable query optimizer rules. |
| Arrow batch bytes are an allocation observation with sharing caveats | [Arrow 59.3.0 RecordBatch](https://github.com/apache/arrow-rs/blob/59.3.0/arrow-array/src/record_batch.rs#L804) | 2026-09-14 | `pub fn get_array_memory_size(&self) -> usize` | verified; the method sums column array memory and can count shared buffers more than once. Name this observation accordingly; it is neither unique resident bytes nor process RSS. Keep managed-pool, output batch, writer, spill and process measurements distinct. |
| Stable std supports safe Unix flag passing, not a portable no-follow setter | [Rust 1.98.1 OpenOptionsExt](https://github.com/rust-lang/rust/blob/1.98.1/library/std/src/os/unix/fs.rs#L499); [Linux UAPI flags](https://github.com/torvalds/linux/blob/v6.17/include/uapi/asm-generic/fcntl.h#L38) | 2026-09-14 | `fn custom_flags(&mut self, flags: i32) -> &mut Self`; `#define O_NOFOLLOW 00400000` | verified; `std::os::unix::fs::OpenOptionsExt` is safe stable Rust. It passes caller-supplied integer flags; std does not export O_NOFOLLOW. On the inspected Linux x86_64 target, dependency-free constants are `O_NOFOLLOW=0o400000` and `O_NONBLOCK=0o4000`. Scope numeric constants to a verified target ABI, rather than applying them to every Unix target. Combine flags in one call: later custom_flags calls replace earlier custom flags. |
| No-follow protects the final path component only | [Linux open semantics](https://man7.org/linux/man-pages/man2/open.2.html); [Rust 1.98.1 File metadata](https://github.com/rust-lang/rust/blob/1.98.1/library/std/src/fs.rs) | 2026-09-14 | "Symbolic links in earlier components of the pathname will still be followed." | verified; read-only OpenOptions with O_NOFOLLOW rejects a symlink basename at open time. Validate regular-file type/size from the returned File and read/hash through that same handle. O_NONBLOCK prevents FIFO-open waiting before type rejection, but does not make regular-file I/O asynchronous or deadline-bounded. This is not ancestor-path containment or immutability against in-place writes. A preceding symlink_metadata/canonicalize check alone does not close the open race; existing trusted parent-directory and content-digest requirements remain necessary. |

Suggested implementation oracles, not executed here: count one provider scan/planning and one
execution for an instrumented query; verify captured metrics belong to the retained executed
nodes and include an explicit multipart coalescing root. Exercise real Parquet statistics/bloom
pruning and forced spill, checking typed counters rather than formatted string presence. Cover
empty, failed, limited and cancelled streams without coercing missing metrics to zero. Bound
deep/wide plans, oversized literals and per-file metric labels during capture; assert an
explicit truncation marker and unchanged query results. For source reads, reject final symlinks,
directories and FIFOs, exercise replacement between preliminary inspection and open, and prove
reads/digests stay tied to the opened regular-file handle.

### Plan 11 signature-only payload projection 2026-09-14

Context7 `/apache/datafusion` supplied discovery leads; exact DataFusion 55.1.0 and
Arrow/Parquet 59.3.0 registry sources were inspected. At the implementation owner's explicit
request, a standalone bounded probe was compiled with Rust 1.98.1 against existing rlibs
(`datafusion-e0fbfc98c129a2e9`, `tokio-8df0b5bff2cbca52`), without Cargo, dependency changes,
application edits or containers. Its [source](../../.dev-state/upstream-signature-projection/probe.rs)
and [executed-plan log](../../.dev-state/upstream-signature-projection/probe.log) retain the
reproduction and observations; this is an upstream behavior probe, not an acceptance gate.

| Claim | Primary source | Retrieved | Short exact quote or declaration | Verdict / implementation consequence |
|---|---|---|---|---|
| Native named_struct and field access are public Rust expressions | [55.1.0 core expression exports](https://github.com/apache/datafusion/blob/55.1.0/datafusion/functions/src/core/mod.rs#L137); [named_struct implementation](https://github.com/apache/datafusion/blob/55.1.0/datafusion/functions/src/core/named_struct.rs#L102) | 2026-09-14 | `named_struct`; `StructArray::new`; `None` | verified; import `datafusion::functions::core::{expr_fn::named_struct, expr_ext::FieldAccessor}`. `named_struct(Vec<Expr>) -> Expr` takes alternating constant nonempty field names and values; `.field(name)` supplies native get_field. The constructor creates fresh nullable child fields and an all-valid parent, even if every child is null. It does not preserve the old struct's parent validity or direct child metadata automatically. |
| CASE can preserve absent joined observations | [55.1.0 CaseBuilder](https://github.com/apache/datafusion/blob/55.1.0/datafusion/expr/src/conditional_expressions.rs#L64); [physical CASE](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-expr/src/expressions/case.rs#L1198) | 2026-09-14 | `pub fn end(&self) -> Result<Expr>`; `ScalarValue::try_new_null` | verified; `datafusion::logical_expr::when(col("observation_id").is_not_null(), payload_expr).end()?` yields a typed nullable payload without an ELSE branch. Use the admitted scalar observation-presence key; a whole `payload IS NOT NULL` expression introduces a whole-struct dependency. A literal `ScalarValue::Utf8(None)` can preserve a presentation docs field's type without referencing source documentation, but does not itself prove scan pruning. |
| Drop unused aliases with a logical projection before execution | [55.1.0 DataFrame select/drop](https://github.com/apache/datafusion/blob/55.1.0/datafusion/core/src/dataframe/mod.rs#L410); [OptimizeProjections](https://github.com/apache/datafusion/blob/55.1.0/datafusion/optimizer/src/optimize_projections/mod.rs#L18) | 2026-09-14 | `pub fn drop_columns<T>(self, columns: &[T]) -> Result<DataFrame>` | verified; `select(...)` or `drop_columns(&["docs"])` eliminates an unused top-level alias and lets normal projection optimization act. It does not remove `payload.docs` while another expression still consumes the whole payload. Keep request projection in the shared view/query builder, before hydration; do not infer physical leaf exclusion from a smaller result DTO. |
| Parquet supports some nested leaf projection, with whole-root fallbacks | [55.1.0 read-plan resolution](https://github.com/apache/datafusion/blob/55.1.0/datafusion/datasource-parquet/src/projection_read_plan.rs#L325); [decoder mask installation](https://github.com/apache/datafusion/blob/55.1.0/datafusion/datasource-parquet/src/decoder_projection.rs#L100) | 2026-09-14 | `ProjectionMask::leaves`; `projection_mask: read_plan.projection_mask` | verified; resolved constant struct paths to primitive leaves can produce decoder leaf masks. Whole-root references override them. Returning `payload.deprecated` or `payload.python` as a Struct falls back to the entire payload; list projections have separate limited support. These internal read-plan helpers are private implementation evidence, not public application APIs. A displayed field projection alone does not prove which leaves were read. |
| A whole narrowing Struct cast can preserve nested siblings and validity | [55.1.0 cast expression](https://github.com/apache/datafusion/blob/55.1.0/datafusion/expr/src/expr_fn.rs#L330); [nested schema clipping](https://github.com/apache/datafusion/blob/55.1.0/datafusion/datasource-parquet/src/nested_schema_pruning.rs#L18); [read-plan cast rules](https://github.com/apache/datafusion/blob/55.1.0/datafusion/datasource-parquet/src/projection_read_plan.rs#L573) | 2026-09-14 | `pub fn cast(expr: Expr, data_type: DataType) -> Expr` | verified; construct a Struct target from admitted payload fields excluding docs, and project `cast(col("payload"), target).alias("payload")` on the base relation before joins. Carry that struct intact. Clipping matches Struct children by name and recurses through List/LargeList while preserving ancestor validity via retained leaves. Maps, dictionaries, fixed-size lists and view wrappers are kept whole below unsupported shapes; conflicting casts, mixed raw field access or whole roots can defeat clipping. Zero-overlap nested structs are not a valid shortcut. |
| Reconstructing fields after a narrowing cast can undo the benefit | [55.1.0 schema adapter](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-expr-adapter/src/schema_rewriter.rs#L370); [read-plan handling](https://github.com/apache/datafusion/blob/55.1.0/datafusion/datasource-parquet/src/projection_read_plan.rs#L325); [executed probe](../../.dev-state/upstream-signature-projection/probe.log) | 2026-09-14 | `try_narrow_struct_cast`; `READ_BYTES depth=2 node=DataSourceExec bytes=3932825` | verified limitation; the adapter moves field extraction through struct casts. In the exercised payload, nested reconstruction read full docs even with casts displayed inside the Parquet projection. A cast inserted only above the join also stayed too late to reduce reads. Preserve the whole narrowed struct through joins and decode the known missing docs field through a typed result projection. |
| The actual nested payload shape admits the whole-struct route | [probe source](../../.dev-state/upstream-signature-projection/probe.rs); [successful whole-struct variant](../../.dev-state/upstream-signature-projection/probe.log#L81); [Parquet bytes metric implementation](https://github.com/apache/datafusion/blob/55.1.0/datafusion/datasource-parquet/src/reader.rs#L237) | 2026-09-14 | `READ_BYTES depth=2 node=DataSourceExec bytes=566`; `nested_values=checked` | verified bounded execution; a 3,938,365-byte uncompressed fixture included declared_kind, signature, doc_summary, docs, deprecated, cfg_hints and the full nested python/publicness shape. Base narrowing without later field reconstruction reduced reported scan reads from 3,932,825 to 566 bytes. Three result rows retained one absent payload, one null python/deprecated pair, and asserted signature, list contents, publicness flags and nested nulls. A base narrowing cast followed by a whole cast back also read 566 bytes and null-filled docs; omitting docs from the private result schema is simpler. This proves the tested query/provider shape, not every production view or storage configuration. |

Production oracle: execute the actual shared-view request plan with an oversized docs sibling,
both present/absent nested values and an unmatched observation; assert typed results and inspect
the retained executed scan plus its read metrics. Cover documentation-enabled requests separately.
Where exact byte-range exclusion matters, the public `ParquetFileReaderFactory`/`AsyncFileReader`
extension can record requested ranges; distinguish footer/prefetch/cache reads from decoded
columns. Neither source inspection nor projected plan text substitutes for this exercised proof.

## Plan 11 selected-read and builder verification — 2026-09-14

Pinned DataFusion 55.1.0 / Arrow and Parquet 59.3.0, no dependency changes.

- The private `inspection_observations` provider uses the same admitted files, witnesses and
  native ExactParquet implementation with a narrower logical payload schema. DataFusion's
  `nested_schema_pruning.rs` and `schema_rewriter.rs` adapt it at the Parquet reader. Carrying a
  cast through logical views is insufficient: RepartitionExec rejects non-column projection
  swapping. The production regression retains the default two partitions and cached leased views.
  It measured 13,582 selected bytes versus 1,011,708 full bytes on four deterministic large docs.
  This is a scoped executed-scan result, not the real-workload benchmark series.
- `ProjectionExec::make_with_child` permits the lease wrapper to retain ownership while allowing
  native projection optimization. Do not expose an underlying logical plan if that would inline
  past the scan that installs the lease. Idle cached providers hold no request lease.
- `StringBuilder::with_capacity(items, utf8_bytes)` and
  `ListBuilder::with_capacity(child_builder, parent_rows)` are available. Child string capacity
  is independent of list-parent capacity. `finish` transfers buffers without shrinking;
  `get_array_memory_size` counts capacity recursively, including array structures and sometimes
  shared backing allocations. Accurate preallocation reduces incidental growth but does not
  replace the writer's preflight and final Arrow byte checks.
- Parquet `memory_size`, `in_progress_size` and `in_progress_rows` are current writer estimates.
  They exclude input batches and some footer/transient allocations, and can reset during flush.
  Encoded row-group/page size thresholds can overshoot; they are not process-memory ceilings.
  Input/record/batch/file/row-group bounds, explicit flushes and whole-process measurements remain
  independent controls.

Primary exact-release sources inspected locally: `datafusion-datasource-parquet-55.1.0/src/nested_schema_pruning.rs`,
`datafusion-physical-expr-adapter-55.1.0/src/schema_rewriter.rs`,
`datafusion-physical-plan-55.1.0/src/{projection.rs,repartition/mod.rs}`,
`arrow-array-59.3.0/src/{builder/generic_bytes_builder.rs,builder/generic_list_builder.rs,array/byte_array.rs,array/list_array.rs}`,
and `parquet-59.3.0/src/arrow/arrow_writer/mod.rs`. Source discovery used Context7; exact-version
proof used the lockfile-resolved registry sources. Diagnostic logs are recorded in the Plan 11 ledger.

### Native Parquet allocation boundary — verified 2026-09-14

Pinned Parquet/Arrow 59.3.0, Rust 1.98.1; upstream-verifier and installed source inspection.

| Claim | Primary source | Retrieved | Exact quote / evidence | Verdict and consequence |
|---|---|---|---|---|
| Native row-group isolation is supported | [Arrow reader](https://docs.rs/parquet/59.3.0/parquet/arrow/arrow_reader/struct.ArrowReaderBuilder.html) | 2026-09-14 | `new_with_metadata`; `with_row_groups`; `with_batch_size` | verified; load metadata once, select each group separately; batches need not cross group boundaries. |
| Footer/list parsing can allocate before contents are validated | [Parquet Thrift source](https://docs.rs/crate/parquet/59.3.0/source/src/parquet_thrift.rs) | 2026-09-14 | `Vec::with_capacity(list_ident.size as usize)` | verified; a serialized footer-byte cap does not bound declared list allocation. |
| Decompression can allocate from advertised length | [Page reader](https://docs.rs/crate/parquet/59.3.0/source/src/file/serialized_reader.rs) | 2026-09-14 | `Vec::with_capacity(uncompressed_page_size)` | verified; public reader properties offer no application allocation ceiling; a bounded ChunkReader does not intercept it. |
| Page peeking does not expose allocation sizes | [Page metadata](https://docs.rs/crate/parquet/59.3.0/source/src/column/page.rs) | 2026-09-14 | `num_rows`; `num_levels`; `is_dict` | verified; private page-header parsing is not an application preflight API. Eight-row admission is not a preallocation guarantee. |
| Linux address-space and CPU limits are separate controls | [getrlimit](https://man7.org/linux/man-pages/man2/getrlimit.2.html) | 2026-09-14 | “maximum size of the process's virtual memory (address space)” | verified; preserve tighter inherited limits and read back results; CPU is consumed seconds, not wall time. RLIMIT_AS is not RSS. |
| Allocation failure can abort instead of returning an Arrow error | [Rust allocation handler](https://doc.rust-lang.org/std/alloc/fn.handle_alloc_error.html) | 2026-09-14 | “abort the process” | verified; nonzero/signal/missing or malformed completion all reject, with parent kill/reap and bounded output. |

The private worker applies limits after exec and before parsing; no `pre_exec` allocation or mutex
work is required. Native successful decoding and existing domain checks remain the admission
authority. ADR-0031 records the new allocation boundary; real query/writer/RSS measurements remain
separate obligations.

### Native view reuse and retained stream ownership — verified 2026-09-14

Pinned DataFusion 55.1.0; upstream-verifier and exact installed-source inspection.

| Claim | Primary source | Retrieved | Exact quote / evidence | Verdict and consequence |
|---|---|---|---|---|
| Cached views hold logical plans; scans can plan again | [DataFrame source](https://docs.rs/crate/datafusion/55.1.0/source/src/dataframe/mod.rs) | 2026-09-14 | `into_view`; `create_physical_plan` | verified; opaque view wrappers prevent native inlining. Rebind leases at scan leaves before exposing the native logical view. |
| Inclusive logical transformation is public | [Logical-plan traversal](https://docs.rs/crate/datafusion-expr/55.1.0/source/src/logical_plan/tree_node.rs) | 2026-09-14 | `transform_up_with_subqueries` | verified; includes scalar, IN and EXISTS subqueries. Preserve all scan fields except the wrapped source and reject unexpected provider shapes. |
| Pristine session state can share defaults with isolated tables | [Session state](https://docs.rs/crate/datafusion/55.1.0/source/src/execution/session_state.rs) | 2026-09-14 | `register_catalog_list`; `mark_start_execution` | verified; clone pristine state, install fresh native catalog/schema objects, reset execution time. DataFusion session ID is shared; service query IDs remain distinct. |
| Physical reset is not fresh metric ownership | [Execution plan](https://docs.rs/crate/datafusion-physical-plan/55.1.0/source/src/execution_plan.rs); [scan source](https://docs.rs/crate/datafusion-datasource/55.1.0/source/src/source.rs) | 2026-09-14 | `reset_plan_states`; `reset_state` | verified limitation; reset excludes some plan kinds and retains shared data-source metrics. Keep fresh physical planning. |
| A final ownership adapter can delegate native planning | [QueryPlanner](https://docs.rs/crate/datafusion-session/55.1.0/source/src/planner.rs); [Session state](https://docs.rs/crate/datafusion/55.1.0/source/src/execution/session_state.rs) | 2026-09-14 | `with_query_planner`; `query_planner` | verified; DefaultQueryPlanner is private. Capture its public trait object before installing the adapter. Native physical optimization finishes inside that delegate; no optimizer runs after the configured planner returns. |
| Final coalescing must sit inside retention ownership | [Execution helper](https://docs.rs/crate/datafusion-physical-plan/55.1.0/source/src/execution_plan.rs); [stream implementation](https://docs.rs/crate/datafusion-physical-plan/55.1.0/source/src/stream.rs) | 2026-09-14 | `CoalescePartitionsExec`; `RecordBatchStreamAdapter` | verified; a coalescer can drop exhausted input streams and RecordBatchStreamAdapter drops its inner stream at None. The dedicated retained stream must enclose native coalescing and hold its lease independently of its inner stream. |

Focused source-bound regression: `.dev-state/plan11-w7-root-retention-tests.log` passes
repository/catalog sorted-plan and exhausted-stream ownership, isolated session catalogs and
typed Arrow cases. `.dev-state/plan11-w7-root-retention-pruning.log` preserves native nested
projection pruning. This is not the full interruption/concurrency or performance exit.

### Producer capacity and observed resource limits — verified 2026-09-14

Required upstream-verifier; installed Podman 4.9.3 and runc 1.5.1; actual selected image probes
subsequently passed in `.dev-state/plan11-w7-workstation-qualification-2.log`.

| Claim | Primary source | Retrieved | Exact quote / evidence | Verdict and consequence |
|---|---|---|---|---|
| Podman CPU bandwidth uses a 100,000-microsecond period | [4.9.3 conversion](https://github.com/containers/podman/blob/v4.9.3/pkg/util/utils.go#L1125) | 2026-09-14 | `100000` | verified; 32 CPUs produced `cpu.max` of `3200000 100000` in both actual images; this is bandwidth, not affinity. |
| Equal memory and memory-plus-swap values disable swap | [cgroups 0.0.6](https://github.com/opencontainers/cgroups/blob/v0.0.6/fs2/memory.go#L49) | 2026-09-14 | “memory and memorySwap set to the same value -- disable swap” | verified in installed runc 1.5.1's pinned dependency and both actual images; `memory.swap.max` was zero. Missing files must reject, since the upstream library can tolerate their absence. |
| Native filesystem capacity is block count times fundamental block size | [coreutils 9.7 stat](https://github.com/coreutils/coreutils/blob/v9.7/src/stat.c#L908) | 2026-09-14 | `f_blocks`; `f_frsize` | verified; image dpkg records contain coreutils 9.7-3 and dash 0.5.12-12. `%T` reported `tmpfs`; checked `%b` × `%S` was 17,179,869,184 bytes in both actual probes. Host stat 9.4 is a separate identity. |
| CPU/memory/PID limits are independent kernel controls | [cgroup v2](https://docs.kernel.org/admin-guide/cgroup-v2.html) | 2026-09-14 | `cpu.max`; `memory.max`; `pids.max` | verified; observed memory 34,359,738,368 bytes and PID/thread count 2,048 matched the selected workstation request. Ancestor quotas/contention still constrain achieved throughput. |

The receipt retains requested and observed typed values plus actual process output and confirmed
cleanup. These are actual capacity/enforcement observations, not performance or whole-service RSS
measurements. The small hostile tests remain separately configured. ADR-0032 governs the change.

### Native client event and isolated authentication contracts — verified 2026-09-14

Required upstream-verifier inspected Codex 0.154.0 and Claude Code 2.1.270. No authenticated client
acceptance follows from CLI help, schema inspection or parser fixtures.

| Claim | Primary source | Retrieved | Exact quote / evidence | Verdict and consequence |
|---|---|---|---|---|
| Codex JSON mode exposes native item/turn events | [Non-interactive mode](https://learn.chatgpt.com/docs/non-interactive-mode) | 2026-09-14 | `item.completed`; `turn.completed` | verified event tags in installed binary; correlate `mcp_tool_call` item IDs and native results. Nested code-mode MCP emission remains unverified until an actual trace; never fall back to tool-name mentions. |
| Claude stream messages contain tool invocation/result identities | [SDK types](https://code.claude.com/docs/en/agent-sdk/typescript) | 2026-09-14 | `tool_use`; `tool_result`; `tool_use_id` | verified installed 2.1.270 schemas; correlate assistant invocation IDs to user result blocks and require a successful final result. |
| Claude supports unattended permission decisions with an explicit allowlist | [CLI reference](https://code.claude.com/docs/en/cli-reference) | 2026-09-14 | `dontAsk`; `--allowedTools` | installed 2.1.270 accepts both `manual` and its internal `default` alias. Use `dontAsk` with explicitly allowed research tools for unattended runs; the earlier unsupported-default claim was incorrect. |
| Codex isolated direct execution can use an explicit API key | [Authentication](https://learn.chatgpt.com/docs/auth) | 2026-09-14 | `CODEX_API_KEY` | verified; map an explicitly supplied OPENAI_API_KEY into the child only when needed. Saved file credentials require explicit minimal adoption. |
| Context7 supports anonymous MCP access | [Context7 clients](https://context7.com/docs/resources/all-clients#api-key-or-anonymous-access) | 2026-09-14 | `https://mcp.context7.com/mcp` | verified; separate actual service connection, subject to anonymous rate limits. Connectivity does not prove a completed lookup. |
| Skill user roots differ from client configuration roots | [Codex skills](https://learn.chatgpt.com/docs/build-skills), [Claude skills](https://code.claude.com/docs/en/skills) | 2026-09-14 | `.agents/skills`; `.claude/skills` | verified; redirect HOME as well as CODEX_HOME/CLAUDE_CONFIG_DIR. Skill discovery alone does not prove the research workflow. |

## Plan 12 native assembly and extraction boundaries — verified 2026-09-14

Context7 supplied discovery leads; exact Cargo.lock and installed tagged sources supplied proof.
The DataFusion join example returned by Context7 was obsolete; implementation uses 55.1.0's
key-slice and optional-filter signatures. No dependency pin changed in this work.

| Claim | Primary source | Retrieved | Exact quote | Consequence |
|---|---|---|---|---|
| Native composition and streaming | [DataFusion 55.1.0](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/core/src/dataframe/mod.rs) | 2026-09-14 | `pub fn distinct`; `pub fn sort`; `pub fn join`; `pub fn join_on`; `pub async fn execute_stream` | Verified; globally sort semantic keys before streaming their digest. |
| Explicit scan schema and metadata | [Parquet options](https://raw.githubusercontent.com/apache/datafusion/55.1.0/datafusion/core/src/datasource/file_format/options.rs) | 2026-09-14 | `pub schema: Option<&'a Schema>`; `pub skip_metadata: Option<bool>` | Verified; exact staged file lists and canonical schema. |
| Whole-input signature parser | [public-api 0.52.2](https://raw.githubusercontent.com/cargo-public-api/cargo-public-api/public-api-v0.52.2/public-api/src/lib.rs) | 2026-09-14 | `std::fs::read_to_string(self.rustdoc_json)?`; `deserializer.disable_recursion_limit()` | Verified limitation; mandatory bounded worker. |
| Whole-crate language maps | [rustdoc-types 0.59.0](https://raw.githubusercontent.com/aDotInTheVoid/rustdoc-types/v0.59.0/src/lib.rs) | 2026-09-14 | `pub index: HashMap<Id, Item>`; `pub paths: HashMap<Id, ItemSummary>` | Verified limitation; raw language state stays inside the worker. |
| Python process limits | [Python 3.14 resource](https://docs.python.org/3.14/library/resource.html) | 2026-09-14 | “The soft limit can never exceed the hard limit.” | Verified; preserve inherited limits and read back the installed pair. AS is virtual memory, CPU is process CPU time. |

Python 3.14.7 exposes RLIMIT_AS/CPU/CORE on this host. Applying limits occurs only in the
separate extraction process, before studied-source parsing; parent wall deadlines still apply.

### Plan 12 unattended Codex setup — verified 2026-09-14

The required upstream verifier checked installed Codex 0.154.0 and official documentation.
Actual authenticated runs remain separately recorded by the client harness.

| Claim | Primary source | Retrieved | Exact evidence | Consequence |
|---|---|---|---|---|
| HTTP `mcp add` can initiate OAuth immediately | Installed 0.154.0 binary and [MCP configuration](https://learn.chatgpt.com/docs/extend/mcp) | 2026-09-14 | `Detected OAuth support. Starting OAuth flow…` | No skip-login flag exists. Register anonymous Context7 directly in the owned sandbox TOML. |
| Anonymous runtime access is supported | [MCP configuration](https://learn.chatgpt.com/docs/extend/mcp) | 2026-09-14 | “If no credential source resolves, Codex can connect to the server without authentication.” | Omit authentication fields; actual required lookups still establish connectivity. |
| Direct native tool routing can be selected | Installed 0.154.0 feature listing and [configuration reference](https://learn.chatgpt.com/docs/config-file/config-reference) | 2026-09-14 | `features.code_mode.enabled`; `features.code_mode_only` | Set both false in the isolated harness, then require real native MCP events. |

### Plan 12 authorized MCP inspection and real version-uncertain documentation — 2026-09-14

| Claim | Primary source | Retrieved | Exact evidence | Consequence |
|---|---|---|---|---|
| Codex supports per-tool approval without relaxing filesystem isolation | [Configuration reference](https://learn.chatgpt.com/docs/config-file/config-reference), installed Codex 0.154.0 parser | 2026-09-14 | “Per-tool approval behavior override for one MCP tool on this server.” | Set `mcp_servers.library-enrichment.tools.inspect_symbol.approval_mode="approve"` only in the authorized test home. Keep `sandbox_mode="read-only"` and truthful execution annotations. |
| The per-tool approve setting is documented | [MCP configuration](https://learn.chatgpt.com/docs/extend/mcp) | 2026-09-14 | `approval_mode = "approve"` | The earlier actual Codex runtime denial was an unattended client configuration gap, not failed contained execution. A fresh actual run must verify the correction. |
| Context7 can return relevant general Serde documentation without exact patch provenance | Context7 `/websites/serde_rs` pointing to [Serde derive](https://serde.rs/derive.html) | 2026-09-14 | `serde = { version = "1.0", features = ["derive"] }` | A04 now retrieves actual documents and compares their scope with independently resolved exact-release evidence. An unrelated-library resolver result alone is insufficient. |
| The exact A04 release exists and is not yanked | [crates.io serde 1.0.228](https://crates.io/api/v1/crates/serde/1.0.228) | 2026-09-14 | `num: 1.0.228`; `yanked: false` | Exact published archive checksum: `9a8e94ea7f378bd32cbbd37198a4a91436180c5bb472411e48b5ec2e2124ae9e`. The actual client must independently acquire and cite it. |

### Plan 12 native test build identity — 2026-09-14

Verified with installed cargo-nextest 0.9.143 (commit
`60fa45f638ffc3f35e74afa65737f45fcd32db2a`) and its actual workspace run.
`cargo nextest run --workspace --no-run` prepares the same Cargo selection before the receipt;
its help says “Compile, but don't run tests.” It may enumerate test binaries, but no test body
is credited until the subsequent recorded run. A narrower store-only build produced a different
worker feature union and is no longer the preparation path. The strict before/after binary
check remains enabled; the corrected 389-test run passed with matching hashes.
Primary references: [nextest running tests](https://nexte.st/docs/running/) and
[Cargo test target selection](https://doc.rust-lang.org/cargo/commands/cargo-test.html).

## Plan 13 W7: tar metadata preflight — 2026-09-15

Exact installed/locked pin: **tar 0.4.46**, upstream commit
`fc459c149f83bf4daceaa52e17d351989002e1a9`. Context7 resolution returned no relevant
Rust tar library in three attempts; pinned source and an isolated compiled probe
establish these claims. Full context and probe output are in
[Plan 13 upstream verification](plan13-upstream-verification.md#w7-follow-up-bounded-tar-metadata-before-native-path-decoding).

| Claim | Source | Retrieved | Exact quote | Verdict | Selected pin / reference |
|---|---|---|---|---|---|
| Raw iteration exposes extension headers before reading their metadata payload. | [archive.rs, pinned commit](https://github.com/composefs/tar-rs/blob/fc459c149f83bf4daceaa52e17d351989002e1a9/src/archive.rs#L261), installed lines 261–272, 386–389 | 2026-09-15 | `return self.next_entry_raw(None);` | verified (source and executed counting-reader probe: 512 bytes before yielding 32-KiB GNU metadata) | tar 0.4.46; Plan 13 W7 / blueprint §10 |
| Normal iteration consumes GNU/local PAX metadata and resolves extended names; raw extraction alone loses those semantics. | [archive.rs, pinned commit](https://github.com/composefs/tar-rs/blob/fc459c149f83bf4daceaa52e17d351989002e1a9/src/archive.rs#L411), installed lines 411–447 | 2026-09-15 | `fields.long_pathname = gnu_longname;` | verified (source and exact GNU/PAX path/link roundtrips; same-reader two-pass rewind at offset 37 preserved path and payload) | tar 0.4.46; Plan 13 W7 |
| PAX parsing itself must happen after a metadata size check. | [entry.rs, pinned commit](https://github.com/composefs/tar-rs/blob/fc459c149f83bf4daceaa52e17d351989002e1a9/src/entry.rs#L368), installed lines 368–380 | 2026-09-15 | `self.pax_extensions = Some(self.read_all()?);` | verified (source); preflight caps and complete framing checks are required service policy | tar 0.4.46; Plan 13 W7 / blueprint §10 |
| Effective link targets may come from GNU/PAX metadata rather than the fixed header. | [entry.rs, pinned commit](https://github.com/composefs/tar-rs/blob/fc459c149f83bf4daceaa52e17d351989002e1a9/src/entry.rs#L342), installed lines 342–365 | 2026-09-15 | `b"linkpath"` | verified (source and probe: 157-byte effective target with an empty fixed header field) | tar 0.4.46; Plan 13 W7 |
| Local path/linkpath are decoded; mtime can remain inert for this non-preserving extractor; unknown semantic overrides must be refused. | [entry.rs, pinned commit](https://github.com/composefs/tar-rs/blob/fc459c149f83bf4daceaa52e17d351989002e1a9/src/entry.rs#L307); [PAX parser](https://github.com/composefs/tar-rs/blob/fc459c149f83bf4daceaa52e17d351989002e1a9/src/pax.rs#L88) | 2026-09-15 | `Some([]) => return None` | verified (source and path/linkpath/mtime probe); reject duplicate keys, size/uid/gid/sparse/unknown overrides and malformed/partially parsed metadata | tar 0.4.46; Plan 13 W7 / blueprint §10 |

### Plan 13 typed research and MCP presentation — verified 2026-09-15

Exact installed evidence and executed upstream probes are recorded in
[Plan 13 upstream verification](plan13-upstream-verification.md). Context7 FastMCP results were
3.2.x/general; installed 4.0.3 source established the selected APIs. No dependency pin changed.

| Claim | Primary source | Retrieved | Exact evidence | Consequence |
|---|---|---|---|---|
| Explicit structured errors retain protocol error status | [FastMCP 4.0.3 ToolResult](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/fastmcp_slim/fastmcp/tools/base.py) | 2026-09-15 | `is_error=self.is_error` | Validate every outcome before explicit ToolResult emission; do not rely on client validation of errors. |
| Authored Pydantic views can compose generated domain DTOs | [Pydantic serialization](https://docs.pydantic.dev/latest/concepts/serialization/) and installed 2.13.5 probe in the verification record | 2026-09-15 | `model_json_schema(mode='serialization', by_alias=True)` | Match advertised schema to JSON-mode serialization; keep semantic lowering and native bounds explicit. |
| Distinct difference uses null-equal anti joins | [DataFusion 55.1.0 logical builder](https://github.com/apache/datafusion/blob/55.1.0/datafusion/expr/src/logical_plan/builder.rs) | 2026-09-15 | `JoinType::LeftAnti`; `NullEquality::NullEqualsNull` | Flat set reconciliation is valid; EXCEPT ALL is not assumed to subtract multiplicities. |
| Result field inference supports precise nullability | [DataFusion 55.1.0 UDF](https://github.com/apache/datafusion/blob/55.1.0/datafusion/expr/src/udf.rs) | 2026-09-15 | `fn return_field_from_args` | Identity UDF is nonnullable; scoring retains nullable nonmatches and captured query identity. |


## Plan 15 Python Arrow fact transport — verified 2026-09-16

| Claim | Primary source | Retrieved | Exact evidence | Verdict / consequence |
|---|---|---|---|---|
| PyArrow 25.0.1 supplies the required CPython 3.14 Linux wheel | [PyPI release metadata](https://pypi.org/pypi/pyarrow/25.0.1/json) | 2026-09-16 | `pyarrow-25.0.1-cp314-cp314-manylinux_2_28_x86_64.whl` | Verified by upstream-verifier; selected and installed exact `pyarrow==25.0.1`. SHA256 `9171748cdf796972d85a4b60157c279913e242992e350c90c7450182a9838b2a`. |
| The encoder accepts an explicit schema and IPC options | [pinned Arrow Python IPC API](https://github.com/apache/arrow/blob/apache-arrow-25.0.1/python/pyarrow/ipc.py#L151) | 2026-09-16 | `new_stream(sink, schema, *, options=None)` | Verified source and local encoding probe; Rust generates and packages the exact schema. |
| IPC metadata version can be selected independently of the package release | [pinned IPC options](https://github.com/apache/arrow/blob/apache-arrow-25.0.1/python/pyarrow/ipc.pxi#L267), [format versioning](https://arrow.apache.org/docs/format/Versioning.html) | 2026-09-16 | `MetadataVersion.V5` | V5, modern framing, no compression; package version equality with Arrow Rust is not assumed. |
| DataFusion reads IPC stream sources directly | Installed `datafusion-datasource-arrow-55.1.0/src/file_format.rs:208` and `src/source.rs:88` | 2026-09-16 | `ArrowSource::new_stream_file_source(table_schema)` | Verified exact source; no intermediary conversion to IPC file or Parquet required. Native worker-to-evidence qualification is recorded separately in Plan 15. |


## Plan 15 Rust fact extraction — verified 2026-09-16

Selected existing pins: **public-api 0.52.2**, **rustdoc-types 0.59.0** (format version 59).
These rows establish interfaces; native Rust fact/normalization qualification is not yet run.

| Claim | Primary source | Retrieved | Exact evidence | Verdict / consequence |
|---|---|---|---|---|
| One rustdoc ID can have several rendered public occurrences | [pinned item processor](https://github.com/cargo-public-api/cargo-public-api/blob/56b483933ceb741978a9f59228e48cae2013e74d/public-api/src/item_processor.rs#L335) | 2026-09-16 | “The reason this is a one-to-many mapping is because of re-exports.” | Verified; emit individual Arrow occurrences instead of the existing HashMap overwrite. |
| Builder omission flags discard implementation evidence | [pinned builder options](https://github.com/cargo-public-api/cargo-public-api/blob/56b483933ceb741978a9f59228e48cae2013e74d/public-api/src/lib.rs#L134) | 2026-09-16 | “at the cost of not fully describing the public API” | Verified; mechanical extraction retains blanket, auto-trait and derived occurrences for later native selection. |
| Occurrence iteration transfers an already built corpus | [pinned iterator](https://github.com/cargo-public-api/cargo-public-api/blob/56b483933ceb741978a9f59228e48cae2013e74d/public-api/src/lib.rs#L248) | 2026-09-16 | “ownership of all `PublicItem`s are transferred to the caller” | Verified; iteration is not streaming construction. The isolated producer must retain a measured external memory boundary. |
| Reexport target IDs are typed and may be absent | [pinned rustdoc-types Use](https://github.com/rust-lang/rustdoc-types/blob/5680fd81c93996938ce2e8e7034b09fee491f4a4/src/lib.rs#L1719) | 2026-09-16 | `pub id: Option<Id>` | Verified; retain missing/external targets as facts and classify them in native plans. |
| Rendered tokens and parent IDs do not identify an alias occurrence uniquely | [pinned tokens](https://github.com/cargo-public-api/cargo-public-api/blob/56b483933ceb741978a9f59228e48cae2013e74d/public-api/src/tokens.rs#L6), [pinned reexport processing](https://github.com/cargo-public-api/cargo-public-api/blob/56b483933ceb741978a9f59228e48cae2013e74d/public-api/src/item_processor.rs#L156) | 2026-09-16 | `Token::Type`; `Some(item.id)` | Verified; repeated aliases may share item/parent IDs. Treat joins as candidate associations; retain ambiguity or definition-scoped evidence. Do not attach every rendering to every alias or infer a structured path from token categories. |

## Plan 15 native Delta reads and planning stacks — verified 2026-09-16

Read-only upstream-verifier checks and exact installed source. Interface checks do not establish
end-to-end resource safety; scoped executable receipts are in Plan 15.

| Claim | Primary source | Retrieved | Exact evidence | Verdict and consequence |
|---|---|---|---|---|
| The native scan accepts an application schema | [DeltaScanConfig](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L230) | 2026-09-16 | `pub fn with_schema(mut self, schema: SchemaRef) -> Self` | Verified; semantic metadata and unsigned output use this public route. |
| A captured scan can be constructed directly; runtime log-store attachment is private | [DeltaScan](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L518) | 2026-09-16 | `pub fn new(snapshot: impl Into<SnapshotWrapper>, config: DeltaScanConfig)`; `pub(crate) fn with_log_store` | Verified; reads register the shared root ObjectStore explicitly. Published ViewTable exposes no mutation route. |
| Schema override does not supply nested leaf pruning at this pin | [kernel projection](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/plan.rs#L297) | 2026-09-16 | `.map(|field| field.name().as_str())`; `kernel_logical_schema.project(&kernel_projection_names)` | Verified source; executed narrow-output probes read unchanged bytes. Store bulky documentation as a separately projectable top-level column. No copied private reader or optimizer disable. |
| Changed storage/override field types can disable filter pushdown | [override type guard](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/plan.rs#L642) | 2026-09-16 | `TableProviderFilterPushDown::Unsupported` | Verified; retain native post-scan predicates. Schema availability is not proof of exact pushdown. |
| Session setup does not replace an existing file backend | [DeltaSessionExt](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L105) | 2026-09-16 | `if self.runtime_env().object_store(&url).is_err()` | Verified; QueryRuntime registers DurableLocalStore first and builders reuse the same Arc. Executed pointer-identity check passed. |
| Recursive protection is not a blanket optimizer guarantee | [EliminateJoin](https://github.com/apache/datafusion/blob/55.1.0/datafusion/optimizer/src/eliminate_join.rs#L193), [SQL stack](https://github.com/apache/datafusion/blob/55.1.0/datafusion/sql/src/stack.rs#L24) | 2026-09-16 | `rewrite_subtree`; `rewrite_node`; `maybe_grow` | Verified source: recursive optimizer functions are not all guarded; actual daemon traces overflow there and during SQL/plan work. Feature is already enabled. |
| Tokio's default worker stack is bounded but insufficient for the failing route | [Tokio 1.53.1 Builder](https://github.com/tokio-rs/tokio/blob/tokio-1.53.1/tokio/src/runtime/builder.rs#L657) | 2026-09-16 | “The default stack size for spawned threads is 2 MiB” | Verified; thread_stack_size also applies to blocking workers. SpawnedTask owns cancellation but does not enlarge stacks. Runtime/block_on caller and shared worker/resource limits need explicit qualification. |
| A late environment change cannot reliably resize Rust test threads | Installed Rust 1.98.1 `library/std/src/thread/mod.rs:133`; `library/test/src/lib.rs:695` | 2026-09-16 | “changes to `RUST_MIN_STACK` may be ignored after program start” | Verified; 8 MiB environment run was a diagnostic, not the service architecture or a qualified resource policy. |

## Plan 15 executor ownership — verified 2026-09-16

Read-only upstream verification at the selected Delta/kernel commits and Tokio 1.53.1. These
interface facts do not certify physical-effect cleanup or the final installed service.

| Claim | Primary source | Retrieved | Exact evidence | Verdict and consequence |
|---|---|---|---|---|
| Default kernel engines use the current Tokio runtime | [Delta engine construction](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L594), [kernel executor](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L202) | 2026-09-16 | `Handle::current()`; `TokioMultiThreadExecutor::new(handle)` | Verified; metadata and full native operations run on the service multi-thread executor. On current-thread runtimes the kernel constructs a background executor instead; its bounded submission channel is not an active-task bound. |
| Selecting an I/O handle does not wrap a custom backend | [table builder](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L265), [log store decoration](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L122) | 2026-09-16 | `with_storage_backend`; `logstore_with` | Verified; custom-backend decoration adds the prefix, not DeltaIOStorageBackend. The ineffective I/O-handle setting is removed. The public experimental wrapper remains excluded from this selected local route. |
| Background shutdown does not wait for completion | [Tokio Runtime](https://github.com/tokio-rs/tokio/blob/tokio-1.53.1/tokio/src/runtime/runtime.rs#L459) | 2026-09-16 | “Shuts down the runtime, without waiting for any spawned work to stop.” | Verified; runtime-owner drop uses this fallback inside async contexts. Running callbacks retain their owner and permits; shutdown_background is not a graceful-drain receipt. |
| Entering a runtime selects task spawning but does not relocate future construction | [Tokio Handle](https://github.com/tokio-rs/tokio/blob/tokio-1.53.1/tokio/src/runtime/handle.rs#L40), [DataFusion SpawnedTask](https://github.com/apache/datafusion/blob/55.1.0/datafusion/common-runtime/src/common.rs#L40) | 2026-09-16 | `tokio::task::spawn(trace_future(task))`; `tokio::task::spawn_blocking(trace_block(task))` | Verified; enter guards end before await. The actual default-stack trace failed in tracing's future boxing; boxing before wrapper composition removed that startup failure. |

## Plan 15 rustdoc declaration scope — verified 2026-09-16

| Claim | Primary source | Retrieved | Exact evidence | Verdict and consequence |
|---|---|---|---|---|
| public-api missing IDs are lookup diagnostics, not a missing-local-declaration count | [public-api 0.52.2 CrateWrapper](https://github.com/cargo-public-api/cargo-public-api/blob/56b483933ceb741978a9f59228e48cae2013e74d/public-api/src/crate_wrapper.rs#L8) | 2026-09-16 | “to aid with debugging” | The pinned implementation records any failed index lookup without severity or deduplication. Native rules classify known external trait enrichment separately; unknown inputs and missing required public edges remain gaps with bounded witnesses. |
| Trait lookup can be for inherited details of an otherwise present impl | [public-api item processor](https://github.com/cargo-public-api/cargo-public-api/blob/56b483933ceb741978a9f59228e48cae2013e74d/public-api/src/item_processor.rs#L283) | 2026-09-16 | “also include default trait methods” | Declaration coverage excludes expansion of external trait definitions and inherited method details, explicitly reported in MCP coverage. The fixture includes external Clone::clone_from enrichment, so complete inherited API coverage is not claimed. |

The fixture declares rustdoc format 61 while the pinned rustdoc-types package describes format 59.
Successful deserialization alone does not qualify that producer/format pairing; format admission
remains a separate open check.

## Plan 15 captured network destinations — verified 2026-09-16

The upstream verifier checked installed reqwest 0.13.5, hyper-rustls 0.27.9 and hyper-util
0.1.20 against their exact source. Context7 was a discovery lead. These are interface facts;
the native HTTP path has separate functional qualification.

| Claim | Primary source | Retrieved | Exact evidence | Verdict and consequence |
|---|---|---|---|---|
| Per-host overrides use captured addresses without a second DNS lookup | [reqwest resolver](https://github.com/seanmonstar/reqwest/blob/v0.13.5/src/dns/resolve.rs#L148) | 2026-09-16 | `Some(dest)` | Verified; the original URI still supplies HTTP authority and Rustls server-name verification. Bind a fresh client/pool to each admitted address set; unmatched names and literal addresses need independent admission. |
| URL ports take precedence over override ports | [reqwest builder](https://github.com/seanmonstar/reqwest/blob/v0.13.5/src/async_impl/client.rs#L2312) | 2026-09-16 | `resolve_to_addrs` | Verified; capture addresses using the URL's effective port. Overrides are not a global allowlist. |
| Explicit proxy disabling prevents ambient proxy routing | [reqwest builder](https://github.com/seanmonstar/reqwest/blob/v0.13.5/src/async_impl/client.rs#L1429) | 2026-09-16 | “system” | Verified; no_proxy clears proxies and disables system proxy discovery. The finite network driver uses direct, admitted destinations. |
| Manual redirect handling requires one outer deadline | [redirect policy](https://github.com/seanmonstar/reqwest/blob/v0.13.5/src/redirect.rs#L57), [timeout](https://github.com/seanmonstar/reqwest/blob/v0.13.5/src/async_impl/client.rs#L1444) | 2026-09-16 | “does not follow any redirect”; “until the response body has finished” | Verified; Policy::none returns redirect responses. A fresh request starts a new timer, so the driver retains one monotonic deadline across DNS, admissions, requests and final body chunks. |


### DataFusion 55.1 map entry layout — 2026-09-16

Verified against `datafusion-functions-nested` 55.1.0 `map_entries.rs` and the process identity
probe: `MapEntries::return_type` and `map_entries_inner` force the value field nullable while
reusing the input entries array. Supplying a non-nullable value field panicked in Arrow 59.3.0
`ListArray::new` with a nested type mismatch. The process Arrow map uses the native nullable read
layout; typed executor input and admission require actual `Entry`/`OutputKind` values. No dependency
or semantic-presence rule was relaxed. Native map ordering/identity and Delta-retained process
admission subsequently passed (`native-process-identity-tests.log`, `native-process-route-tests.log`).
Primary source: [DataFusion 55.1 map_entries implementation](https://docs.rs/datafusion-functions-nested/55.1.0/src/datafusion_functions_nested/map_entries.rs.html).

## Plan 17 FP07 rustdoc format 61 — verified 2026-09-16

Read-only upstream verification using the `verify-upstream` and `rust-code-model` skills, live
registry/GitHub metadata, and exact cached source. No dependency, product code or fixture was changed;
no build or new parser/renderer probe ran. These rows are **Interface-checked**. The existing
0.59 parser's advertised multi-format fidelity is **contradicted**; a renderer patch and full
field-preservation qualification remain required before claiming support.

| Claim | Primary source | Retrieved | Exact evidence | Verdict and consequence |
|---|---|---|---|---|
| The latest published renderer still requires the format-59 model | [public-api registry metadata](https://crates.io/api/v1/crates/public-api), [0.52.2 dependency metadata](https://crates.io/api/v1/crates/public-api/0.52.2/dependencies) | 2026-09-16 | `"max_version":"0.52.2"`; `"req":"^0.59.0"` | Verified: 0.52.2 is non-yanked, published 2026-09-12. Its dependency cannot select 0.61.0. There is no matching upgrade in the latest published pair. |
| Upstream main has no newer renderer fix to select | [exact main commit](https://api.github.com/repos/cargo-public-api/cargo-public-api/commits/56b483933ceb741978a9f59228e48cae2013e74d), [pinned manifest](https://github.com/cargo-public-api/cargo-public-api/blob/56b483933ceb741978a9f59228e48cae2013e74d/public-api/Cargo.toml) | 2026-09-16 | `version = "0.52.2"`; `version = "0.59.0"` | Verified: live main resolved to `56b483933ceb741978a9f59228e48cae2013e74d`, also the packaged 0.52.2 source commit. A branch switch alone does not fix the mismatch. |
| An exact published format-61 fact model exists | [rustdoc-types registry metadata](https://crates.io/api/v1/crates/rustdoc-types), [0.61.0 source commit](https://github.com/rust-lang/rustdoc-types/blob/2e63bdc44b94e86fe12d6e0034d1ed9c2ac6e9dc/src/lib.rs#L117) | 2026-09-16 | `"max_version":"0.61.0"`; `pub const FORMAT_VERSION: u32 = 61;` | Verified: 0.61.0 is non-yanked, published 2026-07-29. It is the exact target model; this verification does not change Cargo. |
| Format 61 changes stable-level encoding and format 60 adds default-body metadata | [0.59 source](https://github.com/rust-lang/rustdoc-types/blob/5680fd81c93996938ce2e8e7034b09fee491f4a4/src/lib.rs#L348), [0.61 source](https://github.com/rust-lang/rustdoc-types/blob/2e63bdc44b94e86fe12d6e0034d1ed9c2ac6e9dc/src/lib.rs#L347) | 2026-09-16 | `#[serde(flatten)]`; `#[serde(tag = "level", rename_all = "snake_case")]`; `pub level: StabilityLevel` | Verified source diff: stable records change from a flattened level/since pair to an externally tagged stable value inside level. The unstable unit variant retains its string form. Successful unstable-only fixtures do not qualify stable metadata. |
| Default-body instability is separate data on three item kinds | [format-61 function/default metadata](https://github.com/rust-lang/rustdoc-types/blob/2e63bdc44b94e86fe12d6e0034d1ed9c2ac6e9dc/src/lib.rs#L372), [associated item fields](https://github.com/rust-lang/rustdoc-types/blob/2e63bdc44b94e86fe12d6e0034d1ed9c2ac6e9dc/src/lib.rs#L895) | 2026-09-16 | `default_unstable: Option<Box<ProvidedDefaultUnstable>>` | Verified: Function, AssocConst and AssocType carry the new metadata; Item carries separate ordinary and const stability. Preserve these in typed facts, including default presence, without reconstructing them from attrs or signature text. |
| The retained renderer has one directly affected exhaustive pattern | [public-api render.rs](https://github.com/cargo-public-api/cargo-public-api/blob/56b483933ceb741978a9f59228e48cae2013e74d/public-api/src/render.rs#L158) | 2026-09-16 | `ItemEnum::AssocType {`; `generics,`; `bounds,`; `type_,` | Source-checked minimum patch: align the renderer dependency to exact 0.61.0 and account for default_unstable in this pattern. Function uses borrowed members and AssocConst already uses `..`. This is not a compile receipt; qualify the patched source and record its own identity. |
| The renderer reparses the payload and materializes its corpus | [public-api Builder](https://github.com/cargo-public-api/cargo-public-api/blob/56b483933ceb741978a9f59228e48cae2013e74d/public-api/src/lib.rs#L198), [deserializer](https://github.com/cargo-public-api/cargo-public-api/blob/56b483933ceb741978a9f59228e48cae2013e74d/public-api/src/lib.rs#L310) | 2026-09-16 | `std::fs::read_to_string(self.rustdoc_json)?`; `deserializer.disable_recursion_limit()` | Verified: updating only the service's first parser leaves the renderer's old parser active. The minimum patch preserves two decode passes and an externally bounded materialization cost; it does not add a compiler invocation. |
| Hosted format must be read from the actual artifact; available retained formats can differ | [docs.rs rustdoc JSON documentation](https://docs.rs/about/rustdoc-json) | 2026-09-16 | “we also keep old format versions around” | Verified current documentation: rebuilds can retain older format downloads. Do not infer format from the package version or assert a universal one-format URL policy. Qualify current external source-format routes separately from the removed historical internal epochs. |

The current `tests/fixtures/rustdoc/enr-fixture-*-*.json` files each contain one ordinary stability
record and eight const-stability records, all observed as unstable; none has non-null default_unstable.
The existing format-v57/v60/v61 parse test therefore does not cover the breaking stable shape or
the newly added default fields. The current `ItemFact` omits all three stability categories.

Required focused oracle: capture a small real `nightly-2026-09-13` staged-API fixture containing
ordinary stable/unstable and const stable/unstable items, plus non-null unstable defaults on a trait
function, associated constant and associated type. Check independently expected typed facts through
the worker and native normalization, and retain distinct declaration/import stability where used.
Include absent metadata/defaults, multiple reexport occurrences, malformed format-61 stability,
and an unsupported format that refuses before full parsing. Verify renderer output against a small
independent expected signature set. Remove the parse-success-only multi-format claim; do not relabel
or strip format-61 input to make the old renderer accept it. Final renderer compilation, runtime
resource bounds and end-to-end field preservation remain unqualified by this research.

## Native provider composition — verified 2026-09-16

DataFusion catalog 55.1.0 and delta-rs core 58f07cd6 require the narrowly vendored seams in
[ADR-0049](../adr/0049-bounded-native-provider-composition.md). Kernel stays at 8ba063f8.
The independent [source verification](../design_review/reviews/evidence/combined-schema-runtime-plan-2026-09-16/provider-composition.md)
records precise source locations, quotes and primary URLs: listing uses `try_collect().await?`;
Delta factory uses `table.table_provider()` without an owned opener. Those interfaces were
verified; complete composed-route behavior remains subject to the named tests, not inferred.

## Plan 17 owned kernel execution and telemetry — verified 2026-09-16

The DataFusion and Delta skills, exact cached sources, an independent upstream-verifier, and
focused executed probes establish this boundary. Pins remain DataFusion 55.1.0, Arrow 59.3.0,
delta-rs 58f07cd6 and kernel 8ba063f8. [ADR-0051](../adr/0051-owned-kernel-io-handlers.md)
records the narrow handler/context patches; `vendor/delta-rs/PROVENANCE.json` identifies their bytes.

| Capability | Exact source / evidence | Scope and consequence |
|---|---|---|
| Kernel handlers accept an application executor | [TaskExecutor and DefaultEngine, kernel 8ba063f8](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs) | `block_on`, `spawn`, `spawn_blocking`, `enter`; native JSON/Parquet/storage remain upstream implementations. Separate owned compute/I/O lanes avoid a synchronous callback awaiting filesystem work queued behind itself. |
| Provider scans construct their own DataFusionEngine | [Delta engine, 58f07cd6](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/mod.rs) | Overriding LogStore alone is insufficient. The vendored KernelIoEngine SessionConfig extension supplies native handlers to this constructor too. It binds the admitted local root backend; arbitrary multi-store routing is not qualified. |
| Local latest-version discovery uses the receiver's engine | [Delta LogStore, 58f07cd6](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L789) | The wrapper calls the native helper with itself as receiver. Delegating to its inner local store recreated the default engine and reproduced blocking-pool exhaustion. Other backends require separate qualification. |
| Kernel metrics have a typed callback boundary | [Kernel metrics, 8ba063f8](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics) | MetricsReporter/ReportGeneratorLayer now populate all twenty MetricEvent variants in generated native records. Binary metric IDs, exact durations and counters are retained. Text is bounded with explicit truncation. No Display parsing. |
| Task tracking waits for future destruction | [tokio-util 0.7.19 TaskTracker](https://docs.rs/tokio-util/0.7.19/src/tokio_util/task/task_tracker.rs.html#489-555) | Existing locked release promoted to a direct dependency with `rt`; no new crate version. Exact quote: “when it is dropped, not when it returns”. Tokens cover callbacks independently of dropped join handles. |
| Closing the tracker permits descendants | [TaskTracker close/wait](https://docs.rs/tokio-util/0.7.19/src/tokio_util/task/task_tracker.rs.html#299-338) | Exact quote: “It does not prevent you from spawning new tasks.” Stop/join transport roots before drain; tracked parents bridge child creation. Only explicit startup/export shutdown supervisors bypass counting to avoid waiting for themselves. |
| Closing channel admission preserves accepted messages | [Tokio 1.53.1 bounded Receiver](https://docs.rs/tokio/1.53.1/src/tokio/sync/mpsc/bounded.rs.html#437-480) | Verified 2026-09-16 against the exact registry source. `close()` refuses new admission; `recv()` reaches None only after queued messages and outstanding permits are consumed/released. Diagnostic shutdown drains to None, persists accepted events, then joins its actor. This does not recover observations previously rejected at capacity. |
| Native append returns the committed snapshot | [Delta write, 58f07cd6](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L624), [transaction conflict handling](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L869) | Exact quote: `commit.snapshot`; `DeltaTable::new_with_state`. With max_retries(0), a newer head or atomic version collision fails; the actor discards cached state and reopens through contract admission. Non-conflict errors stop persistence. This does not authorize external table replacement or log rewriting. |

Recorded focused receipts in `.dev-state/plan17/execution/`:

- `kernel-session-engine-oracle.log`: two passed, 0.70 s; real checkpoint-only reopen/scan and
  DataFusionEngine storage progress with one compute worker/blocking thread.
- `kernel-session-dv-oracle.log`: one passed, 1.18 s; current/historical external-DV readback and
  maintenance. Both receipts precede structured telemetry and physical-drain changes.
- `kernel-correlation-final.log`: two passed, 2.61 s; actual snapshot/scan correlation,
  non-recursive diagnostic reads, and an unowned span closed inside another operation.
- `native-history-two-writer-reload.log`: one passed, 3.76 s; two real runtime writers alternate
  commits against one Delta history, preserving exact per-runtime counts after stale-state reload.
- `owned-resource-history-final.log`: three passed, 3.31 s; current actor contract reuse,
  conflicting writers, lost close acknowledgement and deterministic accepted-message drain.

Full final-source shutdown, workload, resource, crash and installed-product qualification remain
Plan 17 requirements. Callback tracking does not make a stuck blocking operation cancellable.

## Plan 19 restricted binary descriptors — verified 2026-09-17

DataFusion 55.1.0, Arrow/Parquet 59.3.0 and both Delta/kernel pins remain unchanged.
Ciborium and ciborium-ll are pinned exactly to 0.2.2. The independent primary-source and
visitor-shape report is `.dev-state/plan19/execution/binary-codec-verification.md`.

| Claim | Primary source / exact support | Consequence |
|---|---|---|
| 0.2.2 is the non-yanked stable release | [Registry API](https://crates.io/api/v1/crates/ciborium), retrieved 2026-09-17: `max_stable_version` = `0.2.2` | Exact lock and vendor dependency; newer Context7 examples are not this API. |
| Public recursion-limited typed decode exists | [0.2.2 API](https://docs.rs/ciborium/0.2.2/ciborium/de/fn.from_reader_with_recursion_limit.html): `from_reader_with_recursion_limit` | Limit depth explicitly. The decoder type itself is private at this release. |
| Native serialization accepts an owned writer | [0.2.2 API](https://docs.rs/ciborium/0.2.2/ciborium/fn.into_writer.html): `pub fn into_writer` | Stop growth at the output limit. |
| Typed readers are not framing validation | [0.2.2 decoder source](https://docs.rs/ciborium/0.2.2/src/ciborium/de/mod.rs.html): `T::deserialize(&mut reader)` | Exact EOF alone misses a fixed visitor under-consuming a claimed sequence. Native header preflight validates cardinalities first. |
| Header decoder exposes native CBOR structure | [ciborium-ll 0.2.2](https://docs.rs/ciborium-ll/0.2.2/ciborium_ll/struct.Decoder.html): `pub struct Decoder` | Fixed-stack preflight bounds structure without an intermediate semantic Value tree. |
| Delta Snapshot contains materialized IPC and table root | [Snapshot serde at 58f07cd6](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/serde.rs) | Narrow vendor changes require definite sequences, raw IPC bytes and current materialization. Portable exports use native Delta bindings, not location-bound serialized snapshots. |

The isolated upstream codec probe covers native write/roundtrip/rebound scan and malformed
framing/footer refusal. It is not service replay, copied-bundle or final allocation qualification.
See [ADR-0053](../adr/0053-immutable-delta-provider-rebinding.md) and Plan 19 DC09.

### Plan 19: typed list expansion (2026-09-17)

DataFusion 55.1 `datafusion_expr::logical_plan::Unnest::try_new` calls
`get_unnested_columns`, which creates a new scalar Field from the element **datatype** and
drops the element Field metadata (`datafusion-expr-55.1.0/src/logical_plan/plan.rs:4836`).
Struct child fields remain in the datatype. The core `native_list_entries` full-field UDF
therefore wraps List/LargeList/FixedSizeList value buffers in a Struct without copying them;
native UNNEST and `get_field` then preserve identity metadata. Maps use the existing
metadata-preserving `native_map_entries`. The schema-generated reference walker uses these
projections for retained-result snapshot references as well as reference admission.

The isolated `plan19_schema_identity_closure_handles_optional_lists_and_decoys` check exercises
optional parents, duplicate list references and a text decoy. This is not a service/replay or
complete Arrow operator-matrix qualification. No semantic analyzer rule was relaxed.
