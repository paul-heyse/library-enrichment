# Seeing through a macro

Where an invocation stops being an invocation, layer by layer.

## Mental model

Three layers give three correct and different answers. The syntax tree holds `MACRO_CALL` and never expands it. The semantic layer expands and resolves through it. The compiler will print the expansion directly with `-Zunpretty=expanded`, which is the quickest answer when the question is simply 'what does this become' -- though even that stops short of full desugaring.

## Questions this covers

| Question | Start at | Instead of |
|---|---|---|
| What does this function do? | `rustc -Zunpretty=mir` | rustdoc-json, syntax |
| What type does this expression have? | `ra_ap_hir::Semantics::type_of_expr` | syntax, rustdoc-json, mir |
| Where is this symbol defined, and where is it used? | `ra_ap_ide::Analysis` | syntax, rustdoc-json |
| Which impl does this method call resolve to? | `ra_ap_hir::Semantics::resolve_method_call` | rustdoc-json, syntax |
| What does this macro expand to? | `rustc -Zunpretty=expanded` | syntax |
| Where exactly is this in the file, including comments and whitespace? | `ra_ap_syntax::SyntaxNode::text_range` | hir, mir |
| How do I analyse code that does not compile? | `ra_ap_syntax::SourceFile::parse` | hir, mir, rustdoc-json |
| How do I rewrite every occurrence of a pattern? | `rust-analyzer ssr` | syntax |
| Which branch runs, and under what condition? | `rustc_middle::mir::TerminatorKind::SwitchInt` | hir, syntax |
| Where is this value dropped? | `rustc_middle::mir::TerminatorKind::Drop` | syntax, hir |
| What are the call edges out of this function? | `rustc_middle::mir::TerminatorKind::Call` | rustdoc-json, hir |
| Which MIR am I looking at -- before or after optimisation? | `rustc_middle::mir::MirPhase` | mir |
| What node kinds can appear in a Rust syntax tree? | `ra_ap_syntax::SyntaxKind` | rustdoc-json |
| Which rust-analyzer version am I actually running? | `rust-analyzer --version` | hir |

## Why not the neighbouring layer

- **What does this function do?** -- not `rustdoc-json`: contains no function bodies at all -- the single most common wrong turn
- **What does this function do?** -- not `syntax`: has the body as text with nothing resolved
- **What type does this expression have?** -- not `syntax`: records the type as written and infers nothing
- **What type does this expression have?** -- not `rustdoc-json`: has no expressions
- **What type does this expression have?** -- not `mir`: has types, but the expression structure is gone
- **Where is this symbol defined, and where is it used?** -- not `syntax`: can find the spelling but cannot tell two different items with the same name apart
- **Where is this symbol defined, and where is it used?** -- not `rustdoc-json`: has no call sites
- **Which impl does this method call resolve to?** -- not `rustdoc-json`: lists impls but cannot resolve a call site
- **Which impl does this method call resolve to?** -- not `syntax`: sees a method name and nothing else
- **What does this macro expand to?** -- not `syntax`: holds the invocation unexpanded -- MACRO_CALL is as far as it goes
- **Where exactly is this in the file, including comments and whitespace?** -- not `hir`: discards trivia
- **Where exactly is this in the file, including comments and whitespace?** -- not `mir`: keeps spans but not the text between them
- **How do I analyse code that does not compile?** -- not `hir`: needs a resolvable program
- **How do I analyse code that does not compile?** -- not `mir`: needs a successful compilation
- **How do I analyse code that does not compile?** -- not `rustdoc-json`: needs rustdoc to succeed
- **How do I rewrite every occurrence of a pattern?** -- not `syntax`: can match structurally but will not check that the replacement resolves
- **Which branch runs, and under what condition?** -- not `hir`: has the expression tree, not the graph
- **Which branch runs, and under what condition?** -- not `syntax`: has the if as written
- **Where is this value dropped?** -- not `syntax`: drops appear nowhere in the source
- **Where is this value dropped?** -- not `hir`: knows ownership but not drop placement
- **What are the call edges out of this function?** -- not `rustdoc-json`: has no call graph of any kind
- **What are the call edges out of this function?** -- not `hir`: can resolve a call it is shown, but does not enumerate a body's calls as a graph
- **Which MIR am I looking at -- before or after optimisation?** -- not `mir`: asking for 'the MIR' is ambiguous: built, analysis and runtime differ
- **What node kinds can appear in a Rust syntax tree?** -- not `rustdoc-json`: documents the types, not the grammar they encode
- **Which rust-analyzer version am I actually running?** -- not `hir`: the binary reports a rustc release number; there is no published mapping to an ra_ap crate version

## What was executed

Each row ran against the pinned toolchain. A `confirmed` verdict means the probe
held *and* its control came out the other way; `recorded` means there was nothing
for a control to falsify.

| Probe | Question | Verdict |
|---|---|---|
| DF003 | Is more than one MIR produced per function during a compilation? | confirmed |
| HI001 | Does the semantic layer report inferred bodies for a loaded project? | recorded |
| HI002 | Does structural search-and-replace resolve its replacement path, or treat it as text? | confirmed |
| HI003 | Does a failing rust-analyzer subcommand exit non-zero? | confirmed |
| HI004 | Does the binary emit a machine-readable schema of its own configuration? | recorded |
| HI005 | Does the symbol outline carry inferred signatures? | confirmed |
| HI006 | Does ssr complete a rewrite whose replacement path does resolve? | recorded |
| MI001 | Does a conditional produce a switchInt terminator? | confirmed |
| MI002 | Is a function body presented as a graph of basic blocks? | confirmed |
| MI003 | Does an owned value produce an explicit drop terminator? | confirmed |
| MI004 | Does the typed pre-MIR view exist and carry types? | confirmed |
| MI005 | Is there a semi-stable API surface for MIR? | confirmed |
| MI006 | Does the textual MIR view describe itself as an interface? | confirmed |
| MI007 | Can the compiler show a macro's expansion? | confirmed |
| MI008 | Is a graphviz rendering of the control-flow graph available? | confirmed |
| SY001 | Does the parser produce a tree for source that does not compile? | confirmed |
| SY002 | Are comments retained in the tree? | confirmed |
| SY003 | Is a macro invocation expanded in the concrete syntax tree? | confirmed |
| SY004 | Does the parser signal failure through its exit code? | recorded |

**DF003** — The dump filenames are the pass pipeline: built, then analysis passes, then runtime. 'The MIR' of a function is ambiguous until you say which phase, and the phases differ in what they contain.

**HI001** — Shape probe: the counts vary with the fixture, so there is nothing here for a control to falsify. It establishes that the layer loads and infers at all.

**HI002** — This is the difference between SSR and a textual tool: a replacement naming something that does not resolve is refused, where ast-grep or sed would happily write it. Note the invocation -- `ssr` takes rules only and works on the current directory; passing a path makes it parse the path as a rule and report a missing delimiter, naming the wrong cause.

**HI003** — It does -- a refused SSR rule exits 1 while --version exits 0. Worth stating because the neighbouring layer behaves differently: `rust-analyzer parse` exits 0 on source it could not parse cleanly (SY004), so the exit code discriminates for some subcommands and not others. Assert on output when the subcommand is `parse`.

**HI004** — A catalogue the tool publishes about itself, and the authoritative list of what can be configured when driving it as a server.

**HI005** — `symbols` works on a single file with no project, so what it reports is structural plus a locally derived signature. The control file declares no return type and none is invented.

**HI006** — No: it panics on this build (101 is a Rust panic). Recorded rather than hidden, because an agent planning a mechanical rewrite needs to know the resolving path is the broken one here. Every rust-analyzer subcommand is documented as carrying no stability guarantee. If upstream fixes this the probe goes divergent, which is the intended signal to revisit the row rather than a failure of the build.

**MI001** — The layer's reason to exist: control flow is explicit here and implicit everywhere above. A straight-line function produces no switchInt, so the terminator tracks the branch rather than appearing in every body.

**MI002** — The control is the same source through the `normal` view, which is just the source back again. Nested expressions become blocks and terminators only at this layer.

**MI003** — Drops are inserted by the compiler and appear nowhere in the source. A function over Copy types has none, so the terminator reflects ownership rather than being boilerplate. Note what the first version of this probe got wrong: a String that is RETURNED is moved out and never dropped in the callee, so the fixture has to let the value die locally.

**MI004** — THIR sits between HIR and MIR: fully typed, still expression-shaped. The control shows the neighbouring view is a different thing, not a synonym.

**MI005** — StableMIR has been renamed to rustc_public, and the view names the project in its own banner. Anything written against a crate called stable_mir is looking for a name that no longer exists.

**MI006** — It says so itself, in the first line of its own output. The control is the one layer here that IS a documented wire format, which is the contrast worth drawing: parse cargo metadata, read textual MIR.

**MI007** — The complement of SY003. The syntax layer holds the invocation; the compiler will show what it becomes. Both are correct about different questions. Note that `expanded` stops short of full desugaring: format_args! survives in the output as a built-in macro.

**MI008** — Useful when the shape of the graph is the question rather than the contents of the blocks.

**SY001** — The defining property of this layer, and the reason it is the only one that can be used on code mid-edit. Valid source yields no ERROR node, so the marker is a real signal rather than something the parser always emits.

**SY002** — Trivia survives here and nowhere above. Any question about comments, formatting or exact source offsets has exactly one layer that can answer it.

**SY003** — The tree holds the invocation, not its expansion. The control looks for machinery that only appears once println! is expanded, and does not find it. To see through a macro you need the semantic layer or the compiler's own `-Zunpretty=expanded`.

**SY004** — It exits 0 on input it could not parse cleanly. Scripting this means reading the tree for ERROR nodes; the exit code discriminates nothing.

