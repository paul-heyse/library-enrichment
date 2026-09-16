---
name: fastmcp
description: Find what FastMCP and the MCP protocol types can actually do, at a pinned release, from a prebuilt index of fastmcp, mcp and mcp-types plus 774 files of upstream documentation, examples and tests. Use when building an MCP server or client with FastMCP, when choosing between providers, transforms, middleware, transports or auth providers, when a capability might exist but you are not sure, and to check whether code already written targets an API that 4.0 removed. Do not use for general Python questions unrelated to MCP.
---

# FastMCP capability repository

FastMCP 4.0 removed a lot, and it removed it in the way that is hardest to notice: **the source
still looks right.** Eighteen keyword arguments that `FastMCP()` used to accept now raise
`TypeError`, and they raise it when the server is constructed, not when the file is read. A type
checker will usually pass them. `import_server` and `as_proxy` are simply gone.

Meanwhile the official SDK renamed *its* `FastMCP` to `MCPServer`, and left `mcp.server.fastmcp`
as a tombstone that raises on import. Three things now occupy that name.

So the failure this repository exists to prevent is not "I could not find the answer". It is
**confidently writing the previous API**. Check the index before deciding how something is
spelled, and before deciding a capability is absent.

`content/` is prebuilt and pinned. Nothing here queries the network or a service.

## What is pinned

`fastmcp==4.0.3` (the current release), with `mcp==2.2.0` and `mcp-types==2.2.0`. The full
resolved environment is 101 distributions, recorded in `content/PROVENANCE.json`, because the
declared pins are not a pin: `fastmcp` is a metapackage shipping **no code at all**, and the
`fastmcp-slim` that owns every module depends on `mcp` by *range*.

3,056 symbols · 6,612 members · 9,148 locations · 26,545 call edges · 39,204 references ·
25,705 parameters · 2,033 MRO rows · 2,867 ranked items · 74 conformance verdicts ·
364 module pages · 12 topics · 16 catalogs · 9 extension points · 774 corpus files ·
13 rules · 65 probes.

`mcp-types` is indexed because it carries 462 classes -- more than FastMCP itself -- and those
are the types in nearly every FastMCP signature.

## Escalation ladder

Stop at the first rung that answers the question. The rungs are the questions, not the file kinds.

0. **Which construct do I actually want?** `content/topics/00-map.md`. Twelve axes, each naming
   its entry points, the upstream guide, the decision rules and the anti-patterns. Start here,
   because the expensive mistake in this library is choosing the wrong construct, not misspelling
   the right one.
1. **Am I about to write `FastMCP(...)`, a proxy, or a mount?** `content/catalogs/removed-api.md`
   first. Eighteen rows, each with upstream's own replacement text. It is the single most likely
   thing to be written from memory and be wrong.
2. **Does it exist, and how do I import it?**
   ```
   rg -i 'middleware|proxy|elicit' content/index/symbols.tsv | cut -f1,2
   rg -P '^fastmcp\.server\.server\.FastMCP\t' content/index/members.tsv | cut -f2,6
   ```
   Column 2 of `symbols.tsv` is the spelling to **import**; column 1 is where it is defined.
   Those differ constantly, and the defining path is often one you must not write.
3. **What do I have to implement to plug in?** `content/extension-points/00-map.md`, then the
   page for the base. Each splits **Required** (`@abstractmethod`, or every member when the base
   is a `Protocol`) from **Provided**, and provided is where the capability hides: a `Provider`
   that overrides none of its hooks is a correct Provider that supplies nothing. `AuthProvider`
   has 11 obligations and 38 descendants, and upstream never subclasses it directly.
4. **How is this actually used?** The corpus is upstream's own documentation, examples and tests
   at the same tag.
   ```
   rg -l 'async with Client' content/corpus/tests | head
   ast-grep scan -c queries/sgconfig.yml --filter '^corpus-' content/corpus
   ```
   Five `corpus-*` rules: registration sites, extension implementations, composition sites,
   async hooks, client sessions.
5. **Who calls this? Where is it referenced?** `content/catalogs/cross-references.md`, then:
   ```
   rg -P '^fastmcp\.server\.server\.FastMCP\t' content/index/callers.tsv | cut -f2,3,4
   rg -P '^fastmcp\.server\.context\.Context\t' content/index/references.tsv
   ```
   26,545 call edges and 39,204 reference rows, from the checker's batch reports. Scoped to the
   three indexed distributions — see limit 2 in `reference.md` for what that excludes.
6. **Which of 3,056 symbols will I actually meet?** `content/catalogs/most-used.md`, ranked by
   how much the library uses each item. This is the fastest orientation the index offers.
7. **Is this parameter keyword-only? Required? What is its resolved type?**
   ```
   rg -P '^fastmcp\.server\.server\.FastMCP\.__init__\t' content/index/parameters.tsv
   ```
   25,705 rows carrying calling convention, requiredness and *fully-qualified* annotations —
   which a source-local signature string cannot give you. `mro.tsv` carries the checker's
   resolved linearisation, as against the declared bases in `bases.tsv`.
8. **Does this class satisfy that Protocol? Do the two engines agree on a type?**
   `content/catalogs/protocols.md` and `content/catalogs/inference-agreement.md`. Both are
   adjudicated by the checker rather than matched by name, and both state what they do not cover.
9. **Where is this defined, and what can I register?** `content/index/locations.tsv` gives file
   and line range for 9,148 items and members; `content/catalogs/registration.md` gives the
   decorator surface and what each decorator returns at runtime.
10. **Full prose for a resolved item.** Read `content/api/<module>.md`. The path is a rule, not a
   lookup: the module part of the canonical path, verbatim.
   `fastmcp.server.context.Context` → `content/api/fastmcp.server.context.md`.
11. **What is my own code getting wrong?**
   ```
   ast-grep scan -c queries/sgconfig.yml --filter '^project-' <path to the repo you are editing>
   ```
   Reach for this unprompted after writing FastMCP code.
   `project-removed-constructor-kwarg` alone catches the most likely error in the library, and it
   is generated from the pinned table rather than typed, so it cannot go stale at a re-pin.

Catalogs that sit outside the ladder because they answer a lookup directly:
`transports.md`, `auth-providers.md`, `middleware.md`, `exceptions.md`, `context.md`,
`protocol-eras.md`, `distribution-map.md`.

## Rules that keep answers correct

**Import path and definition path are different facts.** `mcp_types.Tool` is defined at
`mcp_types._types.Tool`; `fastmcp.FastMCP` is defined at `fastmcp.server.server.FastMCP`. Quote
the importable spelling when you tell someone what to write, and the canonical path when you
report where something lives. `symbols.tsv` carries both.

**Three axes of composition, easy to conflate.** A **Provider** decides where components come
from; a **Transform** rewrites the catalog observably; **Middleware** intercepts requests.
`mount()` and `create_proxy()` are thin wrappers over Providers, and a `FastMCP` server is
*itself* a Provider. If you are reaching for `import_server`, you want one of these.

**Every hook is `async`.** Provider, Transform and Middleware hooks, and most of `Context`.
Writing one as `def` compiles and returns a coroutine where a value was expected, so the failure
appears far from its cause. `project-sync-hook-override` finds it; `corpus-async-hook` shows the
60 upstream hooks that get it right.

**A single signature can be a dispatcher, not a contract.** Where an item has `@overload`
variants, the implementation signature is wider than any real call — and for `FastMCP.tool` it
is simply wrong: it is annotated as returning a `FunctionTool` while both overloads and the
runtime return the decorated function unchanged. Check `content/index/overloads.tsv`.

**Some types are real but cannot be named.** Defined in a private module, never re-exported:
you can receive one and use it, but not import it or write it in an annotation. Column 6 of
`symbols.tsv` says `no` for these.

**Some rows are inferred, not declared.** `content/index/inferred.tsv` holds 445 types the
source does not state, recovered from the `ty` language server. They are correct today and are
not a promise upstream made. `unannotated.tsv` holds the 354 it could not answer.

**An optional dependency can change what an item *is*.** `fastmcp.exceptions.MCPError` is bound
by `try`/`except ImportError`; with `mcp` installed it is one class, without it another. See
`content/index/conditional.tsv`, and `content/index/extras.tsv` for which of the seven extras
unlocks which modules.

**Absence from `symbols.tsv` has four possible causes**, and they are not the same: the name is
a member rather than a top-level item (check `members.tsv`), it is re-exported from a
distribution not indexed here (check `unresolved.tsv` and the boundary rows), it failed to index
(check `not-indexed.tsv`, currently empty), or it genuinely does not exist at this version.

**The corpus is upstream's text, not this repository's claims.** It is verbatim at tag `v4.0.3`
and verified by digest on every build. Quote it as upstream's word; quote `content/` as this
index's reading of the installed code.

**ast-grep establishes syntax, not semantics.** It resolves no imports or types. A rule hit is a
question about your code, not a proven defect — every one of them is `severity: hint`.

## Reporting

Cite the importable spelling and the file you read it in. When a capability exists but you are
not recommending it, say so — the point of this repository is that the caller learns the option
existed. When the index is silent, report silence rather than absence, and name which of the
four causes above applies.

## Additional references

Read `reference.md` for the full layout, the table schemas, the rule inventory, the known limits
and runnable query recipes. Read `build/README.md` before rebuilding or re-pinning.
