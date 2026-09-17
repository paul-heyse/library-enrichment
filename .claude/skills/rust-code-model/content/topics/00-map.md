# Which layer answers this?

Seven ways to know something about Rust code. They overlap enough to look
interchangeable and they answer different questions, so the first decision is which one
to ask -- not which API to call.

**The failure this page exists to prevent is a confident answer from the wrong layer.**
Rustdoc JSON will tell you a function exists and has no opinion at all about what it
does, because it contains no bodies. The syntax tree will parse anything and resolve
nothing. MIR knows exactly which branch runs and has forgotten the names.

26 of the claims below were executed against the pinned toolchain, each
paired with a control that had to come out the other way. The commands are in
`content/index/behaviors.tsv`.

## The layers

| Layer | Answers | Cannot answer |
|---|---|---|
| rustdoc-json | what a crate exposes: modules, signatures, generics, bounds, trait impls, re-exports, attributes, docs | anything a function does -- there are no bodies in the document at all (RD002); anything about code that is not this crate's public API; anything cfg-gated off at build time (RD005) |
| syntax | exactly how source is written: items, tokens, comments, whitespace, offsets, nesting, and incomplete or invalid code | what any name refers to; what type anything has; what a macro expands to (SY003) |
| hir | what names mean: resolution, definitions and references, inferred types, method resolution, trait and impl relationships, macro-expanded semantics, and the mapping back to syntax | how a body executes as a graph; anything about a project it has not loaded; trivia, which is discarded above the syntax layer |
| project-load | how a real Cargo project becomes an analysis database: workspace discovery, the crate graph, roots, dependencies, sysroot, the selected configuration | any fact about code -- it makes the other layers able to answer, and answers nothing itself |
| mir | how a body executes: basic blocks, terminators, locals, places, operands, rvalues, calls, branches, drops, moves | what the source looked like; original variable names, which survive only as debug annotations (XL002); anything about items with no body |
| dataflow | how state evolves along execution paths: initialisation, liveness, borrows, moves, and any custom forward or backward analysis | anything MIR itself does not already carry -- it derives facts over the graph, it does not add to it |
| cargo-metadata | package identity, targets, workspace structure, dependency identities and renames, the resolved graph, manifest metadata, declared features | anything whatsoever about code; which features a particular build resolved, as opposed to which are declared (PL003) |

`content/index/layers.tsv` carries the same rows plus how each is obtained, whether it
needs a build or a network, its stability, and the pins.

## By what you are trying to do

| Task | Layer | Start at |
|---|---|---|
| What does this crate expose publicly? | rustdoc-json | `rustdoc_types::Crate::index` |
| What is the exact signature, generics and bounds of an item? | rustdoc-json | `rustdoc_types::ItemEnum` |
| Is this item deprecated, or gated behind a feature? | rustdoc-json | `rustdoc_types::Item::deprecation` |
| What does this function do? | mir | `rustc -Zunpretty=mir` |
| What type does this expression have? | hir | `ra_ap_hir::Semantics::type_of_expr` |
| Where is this symbol defined, and where is it used? | hir | `ra_ap_ide::Analysis` |
| Which impl does this method call resolve to? | hir | `ra_ap_hir::Semantics::resolve_method_call` |
| What traits does this type implement? | rustdoc-json | `rustdoc_types::ItemEnum::Impl` |
| What does this macro expand to? | mir | `rustc -Zunpretty=expanded` |
| Where exactly is this in the file, including comments and whitespace? | syntax | `ra_ap_syntax::SyntaxNode::text_range` |
| How do I analyse code that does not compile? | syntax | `ra_ap_syntax::SourceFile::parse` |
| How do I rewrite every occurrence of a pattern? | hir | `rust-analyzer ssr` |
| Which branch runs, and under what condition? | mir | `rustc_middle::mir::TerminatorKind::SwitchInt` |
| Where is this value dropped? | mir | `rustc_middle::mir::TerminatorKind::Drop` |
| Is this local initialised at this point? | dataflow | `rustc_mir_dataflow::impls::MaybeUninitializedPlaces` |
| Is this local live here? | dataflow | `rustc_mir_dataflow::impls::MaybeLiveLocals` |
| How do I write my own dataflow analysis? | dataflow | `rustc_mir_dataflow::Analysis` |
| What are the call edges out of this function? | mir | `rustc_middle::mir::TerminatorKind::Call` |
| What is the crate graph and which features are on? | cargo-metadata | `cargo_metadata::Metadata::resolve` |
| How do I load a real Cargo project into an analysis database? | project-load | `ra_ap_load_cargo::load_workspace_at` |
| Which MIR am I looking at -- before or after optimisation? | mir | `rustc_middle::mir::MirPhase` |
| Which rustdoc format version am I parsing? | rustdoc-json | `rustdoc_types::FORMAT_VERSION` |
| Can I cache a rustdoc Id and use it later? | rustdoc-json | `rustdoc_types::Id` |
| Why is this item missing from the documentation I generated? | rustdoc-json | `--document-private-items` |
| What node kinds can appear in a Rust syntax tree? | syntax | `ra_ap_syntax::SyntaxKind` |
| Which rust-analyzer version am I actually running? | hir | `rust-analyzer --version` |
| Can I script these tools and trust the exit code? | cross-layer | `-` |
| Which layer should I use at all? | cross-layer | `content/index/layers.tsv` |

`content/index/questions.tsv` carries the same rows with two more columns: the layers a
reader plausibly reaches for instead, and why each is wrong. That column is the point of
the table.

## Topic pages

| Topic | Covers |
|---|---|
| [What a crate exposes](public-api-surface.md) | The shape of a public contract: what exists, how it is spelled, what it takes and returns, what it implements, and what is deprecated. |
| [What a symbol means](names-and-types.md) | Resolution, inferred types, method resolution, trait and impl relationships, and the mapping between syntax and resolved entities. |
| [How code is written](source-structure.md) | Items, tokens, comments, whitespace, offsets, nesting, and code that does not compile. |
| [What actually runs](execution-and-control-flow.md) | Basic blocks, terminators, locals, places, operands, rvalues, calls, branches and drops. |
| [How state evolves along a path](value-state.md) | Initialisation, liveness, borrows, moves, and custom forward or backward analyses. |
| [Packages, features and the crate graph](project-context.md) | Package identity, targets, workspace structure, dependency identities and renames, resolved graphs, declared features -- and how a real project becomes an analysis database. |
| [Seeing through a macro](macros.md) | Where an invocation stops being an invocation, layer by layer. |
| [Knowing what you are actually running](versions-and-drift.md) | Which version each subject reports, what that reading covers, and where it misleads. |
| [Running these from a script](driving-the-tools.md) | Invocation shapes, exit codes and the failures that do not look like failures. |

## Layer pages

| Layer | Subject |
|---|---|
| [rustdoc-json](../layers/rustdoc-json.md) | Rustdoc JSON + rustdoc-types |
| [syntax](../layers/syntax.md) | ra_ap_syntax |
| [hir](../layers/hir.md) | ra_ap_hir |
| [project-load](../layers/project-load.md) | ra_ap_load-cargo + ra_ap_project_model |
| [mir](../layers/mir.md) | MIR via rustc_driver + rustc_middle::mir |
| [dataflow](../layers/dataflow.md) | rustc_mir_dataflow |
| [cargo-metadata](../layers/cargo-metadata.md) | cargo metadata / cargo_metadata |
