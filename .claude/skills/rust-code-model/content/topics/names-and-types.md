# What a symbol means

Resolution, inferred types, method resolution, trait and impl relationships, and the mapping between syntax and resolved entities.

## Mental model

This is the only layer that knows what a name refers to. Syntax has the spelling, MIR has the outcome, and neither can tell two items with the same name apart. The price is that it needs a loaded database: `Semantics` over a real crate graph, not a string. Its documentation coverage upstream is 12.84%, so the index here is largely bare signatures -- read `content/corpus/rust-analyzer/hir-lib.rs` when a signature is not enough.

## Questions this covers

| Question | Start at | Instead of |
|---|---|---|
| What type does this expression have? | `ra_ap_hir::Semantics::type_of_expr` | syntax, rustdoc-json, mir |
| Where is this symbol defined, and where is it used? | `ra_ap_ide::Analysis` | syntax, rustdoc-json |
| Which impl does this method call resolve to? | `ra_ap_hir::Semantics::resolve_method_call` | rustdoc-json, syntax |
| How do I rewrite every occurrence of a pattern? | `rust-analyzer ssr` | syntax |
| Which rust-analyzer version am I actually running? | `rust-analyzer --version` | hir |

## Why not the neighbouring layer

- **What type does this expression have?** -- not `syntax`: records the type as written and infers nothing
- **What type does this expression have?** -- not `rustdoc-json`: has no expressions
- **What type does this expression have?** -- not `mir`: has types, but the expression structure is gone
- **Where is this symbol defined, and where is it used?** -- not `syntax`: can find the spelling but cannot tell two different items with the same name apart
- **Where is this symbol defined, and where is it used?** -- not `rustdoc-json`: has no call sites
- **Which impl does this method call resolve to?** -- not `rustdoc-json`: lists impls but cannot resolve a call site
- **Which impl does this method call resolve to?** -- not `syntax`: sees a method name and nothing else
- **How do I rewrite every occurrence of a pattern?** -- not `syntax`: can match structurally but will not check that the replacement resolves
- **Which rust-analyzer version am I actually running?** -- not `hir`: the binary reports a rustc release number; there is no published mapping to an ra_ap crate version

## What was executed

Each row ran against the pinned toolchain. A `confirmed` verdict means the probe
held *and* its control came out the other way; `recorded` means there was nothing
for a control to falsify.

| Probe | Question | Verdict |
|---|---|---|
| HI001 | Does the semantic layer report inferred bodies for a loaded project? | recorded |
| HI002 | Does structural search-and-replace resolve its replacement path, or treat it as text? | confirmed |
| HI003 | Does a failing rust-analyzer subcommand exit non-zero? | confirmed |
| HI004 | Does the binary emit a machine-readable schema of its own configuration? | recorded |
| HI005 | Does the symbol outline carry inferred signatures? | confirmed |
| HI006 | Does ssr complete a rewrite whose replacement path does resolve? | recorded |

**HI001** — Shape probe: the counts vary with the fixture, so there is nothing here for a control to falsify. It establishes that the layer loads and infers at all.

**HI002** — This is the difference between SSR and a textual tool: a replacement naming something that does not resolve is refused, where ast-grep or sed would happily write it. Note the invocation -- `ssr` takes rules only and works on the current directory; passing a path makes it parse the path as a rule and report a missing delimiter, naming the wrong cause.

**HI003** — It does -- a refused SSR rule exits 1 while --version exits 0. Worth stating because the neighbouring layer behaves differently: `rust-analyzer parse` exits 0 on source it could not parse cleanly (SY004), so the exit code discriminates for some subcommands and not others. Assert on output when the subcommand is `parse`.

**HI004** — A catalogue the tool publishes about itself, and the authoritative list of what can be configured when driving it as a server.

**HI005** — `symbols` works on a single file with no project, so what it reports is structural plus a locally derived signature. The control file declares no return type and none is invented.

**HI006** — No: it panics on this build (101 is a Rust panic). Recorded rather than hidden, because an agent planning a mechanical rewrite needs to know the resolving path is the broken one here. Every rust-analyzer subcommand is documented as carrying no stability guarantee. If upstream fixes this the probe goes divergent, which is the intended signal to revisit the row rather than a failure of the build.

