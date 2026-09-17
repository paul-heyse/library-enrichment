# What actually runs

Basic blocks, terminators, locals, places, operands, rvalues, calls, branches and drops.

## Mental model

MIR is the source after every convenience has been removed: control flow is an explicit graph, types are explicit everywhere, nested expressions are gone, and drops the source never mentioned are written in. Reaching it costs a full compilation. Two cautions: 'the MIR' is ambiguous until you name a phase -- built, analysis and runtime differ -- and the textual view states in its own first line that it is for human consumers only.

## Questions this covers

| Question | Start at | Instead of |
|---|---|---|
| What does this function do? | `rustc -Zunpretty=mir` | rustdoc-json, syntax |
| What does this macro expand to? | `rustc -Zunpretty=expanded` | syntax |
| Which branch runs, and under what condition? | `rustc_middle::mir::TerminatorKind::SwitchInt` | hir, syntax |
| Where is this value dropped? | `rustc_middle::mir::TerminatorKind::Drop` | syntax, hir |
| What are the call edges out of this function? | `rustc_middle::mir::TerminatorKind::Call` | rustdoc-json, hir |
| Which MIR am I looking at -- before or after optimisation? | `rustc_middle::mir::MirPhase` | mir |

## Why not the neighbouring layer

- **What does this function do?** -- not `rustdoc-json`: contains no function bodies at all -- the single most common wrong turn
- **What does this function do?** -- not `syntax`: has the body as text with nothing resolved
- **What does this macro expand to?** -- not `syntax`: holds the invocation unexpanded -- MACRO_CALL is as far as it goes
- **Which branch runs, and under what condition?** -- not `hir`: has the expression tree, not the graph
- **Which branch runs, and under what condition?** -- not `syntax`: has the if as written
- **Where is this value dropped?** -- not `syntax`: drops appear nowhere in the source
- **Where is this value dropped?** -- not `hir`: knows ownership but not drop placement
- **What are the call edges out of this function?** -- not `rustdoc-json`: has no call graph of any kind
- **What are the call edges out of this function?** -- not `hir`: can resolve a call it is shown, but does not enumerate a body's calls as a graph
- **Which MIR am I looking at -- before or after optimisation?** -- not `mir`: asking for 'the MIR' is ambiguous: built, analysis and runtime differ

## What was executed

Each row ran against the pinned toolchain. A `confirmed` verdict means the probe
held *and* its control came out the other way; `recorded` means there was nothing
for a control to falsify.

| Probe | Question | Verdict |
|---|---|---|
| DF003 | Is more than one MIR produced per function during a compilation? | confirmed |
| MI001 | Does a conditional produce a switchInt terminator? | confirmed |
| MI002 | Is a function body presented as a graph of basic blocks? | confirmed |
| MI003 | Does an owned value produce an explicit drop terminator? | confirmed |
| MI004 | Does the typed pre-MIR view exist and carry types? | confirmed |
| MI005 | Is there a semi-stable API surface for MIR? | confirmed |
| MI006 | Does the textual MIR view describe itself as an interface? | confirmed |
| MI007 | Can the compiler show a macro's expansion? | confirmed |
| MI008 | Is a graphviz rendering of the control-flow graph available? | confirmed |

**DF003** — The dump filenames are the pass pipeline: built, then analysis passes, then runtime. 'The MIR' of a function is ambiguous until you say which phase, and the phases differ in what they contain.

**MI001** — The layer's reason to exist: control flow is explicit here and implicit everywhere above. A straight-line function produces no switchInt, so the terminator tracks the branch rather than appearing in every body.

**MI002** — The control is the same source through the `normal` view, which is just the source back again. Nested expressions become blocks and terminators only at this layer.

**MI003** — Drops are inserted by the compiler and appear nowhere in the source. A function over Copy types has none, so the terminator reflects ownership rather than being boilerplate. Note what the first version of this probe got wrong: a String that is RETURNED is moved out and never dropped in the callee, so the fixture has to let the value die locally.

**MI004** — THIR sits between HIR and MIR: fully typed, still expression-shaped. The control shows the neighbouring view is a different thing, not a synonym.

**MI005** — StableMIR has been renamed to rustc_public, and the view names the project in its own banner. Anything written against a crate called stable_mir is looking for a name that no longer exists.

**MI006** — It says so itself, in the first line of its own output. The control is the one layer here that IS a documented wire format, which is the contrast worth drawing: parse cargo metadata, read textual MIR.

**MI007** — The complement of SY003. The syntax layer holds the invocation; the compiler will show what it becomes. Both are correct about different questions. Note that `expanded` stops short of full desugaring: format_args! survives in the output as a built-in macro.

**MI008** — Useful when the shape of the graph is the question rather than the contents of the blocks.

