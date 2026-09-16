# FastMCP capability repository — reference

Layout, schemas, recipes and limits. Read `SKILL.md` first for the escalation ladder.

## What is pinned

| | Version |
|---|---|
| `fastmcp` | 4.0.3 (current release; a metapackage shipping no code) |
| `fastmcp-slim` | 4.0.3 (owns all 257 modules and every extra) |
| `mcp` | 2.2.0 (the official SDK) |
| `mcp-types` | 2.2.0 (the wire model, 462 classes) |
| Python | 3.14 |
| Griffe | 2.3.0, `allow_inspection=False`, Google docstring parser |
| ty | 0.0.80, hover on definition sites only |
| pyrefly | 1.3.1, the batch-report producer — installed beside the capsule, never in it |
| ast-grep | 0.45.2 |

`build/acquired/` holds what acquisition produced -- the three Griffe documents, the ty rows,
the five vendored corpora, and `ANALYSIS.json` -- because nobody can re-serve bytes that only
exist because someone ran Griffe, a language server and a type checker against a specific
capsule. `build/.cache/` holds the GitHub tarballs and the 800 MB of raw checker reports, and
stays ignored: GitHub can re-serve the first and nine seconds of CPU can reproduce the second.

The full resolved graph — 101 distributions — is in `content/PROVENANCE.json` and asserted on
every acquisition. This matters because the declared pins are not a pin: `fastmcp` requires
`fastmcp-slim[client,server]==4.0.3`, which requires `mcp>=2.0.0,<3.0.0`, a range. Two builds a
month apart would otherwise index different libraries and say nothing about it.

Every extra is installed on purpose (`anthropic`, `openai`, `gemini`, `azure`, `apps`,
`code-mode`, `tasks`). Twelve modules bind names under `try`/`except ImportError`, and Griffe
records whichever branch is live, so installing the full set is what makes the index describe
the library as it runs rather than as it degrades.

Counts: 3,056 symbols · 6,612 members · 9,148 locations · 3,173 access paths ·
26,545 call edges · 39,204 references · 25,705 parameters · 2,033 MRO rows ·
74 conformance verdicts · 364 module pages · 1,078 subtype edges · 9,345 override rows ·
12 topics · 16 catalogs · 9 extension points · 774 corpus files · 43 corpus edges ·
13 rules · 65 probes.

## Layout

```
content/
  PROVENANCE.json        pins, counts, per-file digests
  index/*.tsv            line-oriented projection              -> ripgrep
  model/*.json           one record per module                 -> ast-grep (language: json)
  api/*.md               one prose page per module             -> Read
  catalogs/*.md          direct answers to fixed questions     -> Read
  topics/*.md            12 capability axes; 00-map.md is rung 0
  extension-points/*.md  9 bases: required vs provided         -> Read
  corpus/                774 files of upstream text, verbatim  -> ripgrep + ast-grep
    docs/     149        the documentation site at v4.0.3
    examples/ 152        runnable servers and clients
    tests/    420        where the edge cases are demonstrated
    spec/      53        the MCP specification, both eras
  modules.md             module -> distribution -> page
queries/
  sgconfig.yml           ruleDirs, utilDirs, testConfigs
  rules/model/           4 rules over content/model  (language: json)
  rules/corpus/          5 rules over content/corpus (language: python)
  rules/project/         3 rules for the repo you are editing
  rules/generated/       1 rule derived from the pinned data; never hand-edited
  utils/json-record.yml  the shared anchor every model-* rule matches on
  rule-tests/            fixtures and snapshots
build/
  acquire.py             capsule, Griffe, ty, corpora   (network + uv + griffe)
  fetch.py               pinned GitHub tarballs
  extract.py             Griffe -> normalized JSON
  semantic.py            the ty LSP client
  model.py               stitch, alias closure, descendants, overrides
  link.py                structural symbol -> corpus edges
  build.py emit.py catalogs.py topics.py extension_points.py queries.py
                         offline: stdlib + ast-grep
  verify.py probes.json  eleven checks, 65 probes
  analysis.py            the checker's batch reports    (pysa JSON, glean, coverage)
  analysis_catalogs.py   the five pages those reports feed
  acquired/              the captured documents and corpora
    ANALYSIS.json        the reduction: 104,217 rows, digest-checked on every build
```

### Path construction

| Canonical path | Page |
|---|---|
| `fastmcp.server.server.FastMCP` | `content/api/fastmcp.server.server.md` |
| `fastmcp.server.context.Context` | `content/api/fastmcp.server.context.md` |
| `mcp_types._types.Tool` | `content/api/mcp_types._types.md` |

The module part of the canonical path, verbatim. No lookup table exists because none is needed.

## Index schemas

Tab-separated, sorted, no header row. Tabs and newlines are scrubbed from free text.

| File | Rows | Columns |
|---|---:|---|
| `symbols.tsv` | 3,056 | `canonical · preferred · kind · dist · api_page · nameable · in_all · alias_count · protocol_era · summary` |
| `members.tsv` | 6,612 | `owner · name · kind · declared_on · labels · signature · summary` |
| `aliases.tsv` | 3,173 | `access_path · canonical · kind · export\|import-site` |
| `bases.tsv` | 964 | `class · base · dist` |
| `descendants.tsv` | 1,078 | `base · subtype · depth · nameable` |
| `overrides.tsv` | 9,345 | `subclass · declared_on · member · relation` |
| `overloads.tsv` | 104 | `owner · member ("-" for the item) · ordinal · signature` |
| `decorators.tsv` | 421 | `decorated · decorator` — items and members, so `@abstractmethod` is visible |
| `types-used.tsv` | 3,741 | `owner · member ("-" for the item) · type it mentions`, joined to canonical |
| `inferred.tsv` | 445 | `item · kind · inferred type · source · agreement` — the last is agree / narrower / differ / unanswered |
| `unannotated.tsv` | 354 | `item · kind · reason` |
| `conditional.tsv` | 17 | `module · name · guard · fallback-defined\|import-only` |
| `unresolved.tsv` | 3,134 | `access_path · target` — re-exports leaving the indexed set |
| `not-indexed.tsv` | 0 | `module · reason` — an empty file, not a blank line |
| `extras.tsv` | 108 | 7 extras mapped to distributions and modules, then the lockfile |
| `locations.tsv` | 9,148 | `canonical · kind · file · line · endline` — items and members |
| `callers.tsv` | 26,545 | `callee · caller · file · line` — a constructor call names the class |
| `references.tsv` | 39,204 | `target · file · count` — non-call references, from the Glean report |
| `external-refs.tsv` | 2,143 | `target · defining file · count` — refs leaving the set |
| `usage.tsv` | 2,867 | `canonical · refs · files · callers · callees` — the ranking |
| `parameters.tsv` | 25,705 | `function · signature · position · name · kind · required · annotation` — the signature ordinal keeps overload variants apart |
| `mro.tsv` | 2,033 | `class · position · ancestor` — resolved, not the declared bases |
| `type-coverage.tsv` | 4,802 | `symbol · kind · line · typable · typed · any · untyped` |
| `suppressions.tsv` | 233 | `module · line · kind · codes` — where upstream silenced the checker |
| `imports.tsv` | 2,563 | `importer · imported`, file-level |
| `satisfies.tsv` | 74 | `class · protocol · verdict · disqualifier` — adjudicated, not matched |
| `checker-types.tsv` | 871 | `item · type · line` — the checker's module-level types |

`preferred` is ranked, not guessed: a spelling declared in its module's `__all__` beats one that
is merely importable, a path rooted in the defining distribution beats a re-export through
another, and shallower beats deeper. Before that ranking existed, `Provider` resolved to
`fastmcp.apps.app.Provider` — a plain import inside a module body.

`aliases.tsv` separates `export` (599) from `import-site` (2,574) for the same reason. Every
`from X import Y` anywhere in the indexed set creates a reachable path; `FastMCP` collects 28.
They work, but they are not the API.

## Catalogs

| File | What it answers |
|---|---|
| `removed-api.md` | the 18 removed constructor keywords, with upstream's own replacement text |
| `registration.md` | the decorator surface, and what each decorator actually returns |
| `extension-points.md` | the five base classes and their transitive descendants (the nine per-base pages are `content/extension-points/`) |
| `transports.md` | 4 server transports, 11 client transport classes |
| `auth-providers.md` | every provider that descends from `AuthProvider` |
| `middleware.md` | the 12 hooks and the built-in middleware |
| `exceptions.md` | what a caller branches on |
| `context.md` | what a tool body can do |
| `protocol-eras.md` | the dated copies of the wire model |
| `distribution-map.md` | which distribution owns what |
| `cross-references.md` | who calls this, where it is referenced, and what that excludes |
| `most-used.md` | the 40 items this library uses most — the triage page |
| `import-graph.md` | the file-level import structure, and that it is not a call graph |
| `typing-health.md` | coverage per symbol, and the 233 places upstream suppressed |
| `protocols.md` | which classes satisfy which Protocol, and what disqualifies a near miss |
| `inference-agreement.md` | where ty and the checker differ, and where ty is merely narrower |

## Query recipes

**Does it exist, and how do I import it?**
```bash
rg -i 'elicit|sampling' content/index/symbols.tsv | cut -f1,2
```

**What methods does this class have, including inherited ones?**
```bash
rg -P '^fastmcp\.server\.server\.FastMCP\t' content/index/members.tsv | cut -f2,4,6
```

**What must I implement to subclass this?**
```bash
rg -P '\tfastmcp\.server\.providers\.base\.Provider\t' content/index/overrides.tsv | cut -f1,3,4
```

**Everything that is a Provider, transitively:**
```bash
rg -P '^fastmcp\.server\.providers\.base\.Provider\t' content/index/descendants.tsv | cut -f2,3
```

**Is this signature a dispatcher rather than a contract?**
```bash
rg -P '^fastmcp\.server\.server\.FastMCP\ttool\t' content/index/overloads.tsv | cut -f3,4
```

**Which types does this touch, as canonical paths?**
```bash
rg -P '^fastmcp\.server\.context\.Context\t' content/index/types-used.tsv | cut -f2,3
```

**Was this type declared or inferred, and does the second engine agree?**
```bash
rg -P '\tty\t' content/index/inferred.tsv | head
cut -f5 content/index/inferred.tsv | sort | uniq -c
```

**Why does `fastmcp.dependencies.Depends` have no documentation?**
```bash
rg -P '^fastmcp\.dependencies\.Depends\t' content/index/symbols.tsv    # kind is `reexport`
```

**Who calls this, and from where?**
```bash
rg -P '^fastmcp\.server\.server\.FastMCP\t' content/index/callers.tsv | cut -f2,3,4
```

**Where is this referenced, and in how many files?**
```bash
rg -P '^fastmcp\.server\.context\.Context\t' content/index/references.tsv
```

**Which items does this library actually use most?**
```bash
sort -t$'\t' -k4,4nr content/index/usage.tsv | head -20
```

**Is this parameter keyword-only? Required? What is its resolved type?**
```bash
rg -P '^fastmcp\.server\.server\.FastMCP\.__init__\t' content/index/parameters.tsv | cut -f3-7
```

**How many overloads does this have, and which parameter differs?**
```bash
rg -P '^fastmcp\.server\.server\.FastMCP\.tool\t' content/index/parameters.tsv | cut -f2,4,7
```

**What is the resolved MRO, as against the declared bases?**
```bash
rg -P '^fastmcp\.server\.server\.FastMCP\t' content/index/mro.tsv | cut -f2,3
rg -P '^fastmcp\.server\.server\.FastMCP\t' content/index/bases.tsv | cut -f2
```

**Does this class satisfy that Protocol, and if not, why not?**
```bash
rg -P '\tmcp\.server\.auth\.provider\.TokenVerifier\t' content/index/satisfies.tsv | cut -f1,3,4
```

**Where is this library's typing weakest, and where did upstream suppress?**
```bash
sort -t$'\t' -k6,6nr content/index/type-coverage.tsv | head
cut -f4 content/index/suppressions.tsv | tr ',' '\n' | sort | uniq -c | sort -rn
```

**Structured questions over the model:**
```bash
ast-grep scan -c queries/sgconfig.yml --filter '^model-' content/model
ast-grep test -c queries/sgconfig.yml
```

## Rule inventory

All `severity: hint`, none carries a `fix`, every one has fixtures.

| Rule | Family | Finds |
|---|---|---|
| `project-removed-constructor-kwarg` | **generated** | a `FastMCP(...)` keyword removed in 4.0 |
| `project-removed-composition-api` | project | `import_server` / `as_proxy` |
| `project-sdk-server-confusion` | project | importing the SDK's server instead of FastMCP |
| `project-sync-hook-override` | project | an `on_*` hook written `def` where the base is `async def` |
| `model-unnameable` | model | items that exist but cannot be imported |
| `model-dispatch-only` | model | signatures that are dispatchers, not contracts |
| `model-inferred-type` | model | types from `ty` rather than the source |
| `model-boundary` | model | names re-exported from an unindexed distribution |
| `corpus-tool-registration` | corpus | 2,028 upstream registration sites |
| `corpus-extension-subclass` | corpus | 79 upstream implementations of an extension point |
| `corpus-composition-site` | corpus | 675 mounts, proxies and `add_*` calls |
| `corpus-async-hook` | corpus | 60 hooks upstream writes `async def` |
| `corpus-client-session` | corpus | 1,029 `async with Client(...)` sessions |

`project-removed-constructor-kwarg` is written by `build/queries.py` from the same 18-row table
`catalogs/removed-api.md` reports, together with its own fixture. A hand-typed alternation is
exact on the day it is written and silently wrong at the next re-pin; this one cannot be.

## Provenance and known limits

Six boundaries. The first two were once flat refusals and are now scoped claims; the
rest are unchanged. Read them before concluding a capability is absent.

1. **Structural `Protocol` conformance is adjudicated, not matched, and only where asked.**
   A table built by comparing member names would be the most dangerous page here -- confidently
   wrong wherever a signature differed -- so `satisfies.tsv` contains no matched verdicts. Every
   row is a probe the checker decided:

   ```python
   def _p(c: SomeClass) -> SomeProtocol:
       return c
   ```

   Names choose the questions and nothing else: only classes whose public members are a superset
   of the protocol's are probed, because a class missing a member cannot satisfy it. So absence
   from the table is not a verdict.

   Two limits stand. `ty` 0.0.80 returns zero subtypes for a Protocol, so nominal inheritance
   still comes from `descendants.tsv`. And the conformance suite records that this checker
   deviates on **variance**, so a verdict turning on variance is worth confirming.
2. **Call graphs and cross-file references are scoped to the three indexed distributions.**
   This was previously absent entirely: `textDocument/references` returned a single hit for a
   heavily-used class, so nothing was built on it. It is now answered from the checker's *batch*
   reports -- `--report-pysa --report-pysa-format json` for call edges, definitions and
   parameters, `--report-glean` for non-call references -- in one run over 388 files rather than
   a loop over 3,056 symbols.

   What that does **not** cover: callers outside `fastmcp`, `mcp` and `mcp_types`. An item with
   no row in `usage.tsv` was never called or referenced *within these three distributions*,
   which is not the same as unused. References leaving the set are counted in
   `external-refs.tsv`, never joined.

   Two shapes to know before reading a row. A constructor call is recorded against the **class**,
   not `__init__`. And a caller reading `… (module level)`, `… (decorator)` or `… (class body)`
   is a real call site that no function name describes.
3. **`ty` was narrowed to hover on definition sites.** Its `typeHierarchy/subtypes` is
   direct-only — 10 children of `Provider` against a closure of 26, omitting `FastMCP` itself —
   so descendants come from Griffe instead. Whole-file inlay hints returned mostly
   parameter-name hints and method-body locals.
4. **Dynamically constructed API is invisible.** Griffe runs with `allow_inspection=False`,
   which is what makes the build deterministic and safe. Anything built at import time by code
   rather than declared in source is not here.
5. **Members are MRO-flattened in the tables, not on the pages.** A class page lists what it
   declares and names its inherited members by origin; the full flattened set is
   `members.tsv`. Repeating every base's signatures on every descendant tripled the tree for no
   new fact.
6. **`unresolved.tsv` is large on purpose** (3,134 rows). Most are stdlib and third-party
   imports that were never going to resolve here. The rows that matter are re-exports an
   indexed `__all__` declares, and those were promoted to boundary items in `symbols.tsv`
   instead.

One rule was considered and not shipped: a check for `@mcp.tool` functions with unannotated
parameters. FastMCP annotates 99.2% of its own parameters and the schema-generation path
tolerates the rest, so the rule would have fired mostly on correct code. Precision over recall.

## Rebuilding

```bash
python3 build/acquire.py                 # once; network + uv + griffe + ty + the checker
python3 build/build.py                   # offline: stdlib + ast-grep only
python3 build/verify.py                  # all eleven checks
python3 build/acquire.py --check         # report whether PyPI has moved; never re-pins
```

`acquire.py` is deliberately unreachable from `build.py`: `verify.py` re-runs the build to prove
determinism, and if acquisition were reachable that check would start installing packages.
