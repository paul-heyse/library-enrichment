# MIR via rustc_driver + rustc_middle::mir

**Reach for it when** the question is about control flow, drops, or what actually runs.

## What it is for

how a body executes: basic blocks, terminators, locals, places, operands, rvalues, calls, branches, drops, moves.

## What it cannot answer

what the source looked like; original variable names, which survive only as debug annotations (XL002); anything about items with no body.

## Getting it

|  |  |
|---|---|
| Obtained by | `rustc -Zunpretty=mir, -Zdump-mir; or rustc_public for a typed API` |
| Entry point | `rustc_middle::mir::Body` |
| Needs a build | yes -- a full compilation |
| Needs a network | no |
| Stability | unstable and unversioned; the textual view says it is for human consumers only (MI006) |
| Crates pinned here | none -- this layer is the compiler itself |
| Indexed symbols | 0 |

## Questions routed here

- What does this function do? — `rustc -Zunpretty=mir`
- What does this macro expand to? — `rustc -Zunpretty=expanded`
- Which branch runs, and under what condition? — `rustc_middle::mir::TerminatorKind::SwitchInt`
- Where is this value dropped? — `rustc_middle::mir::TerminatorKind::Drop`
- What are the call edges out of this function? — `rustc_middle::mir::TerminatorKind::Call`
- Which MIR am I looking at -- before or after optimisation? — `rustc_middle::mir::MirPhase`

## Executed evidence

| Probe | Question | Verdict | Command |
|---|---|---|---|
| DF003 | Is more than one MIR produced per function during a compilation? | confirmed | `sh -c ls df001 \| grep -c 'build.*mir' \|\| true` |
| MI001 | Does a conditional produce a switchInt terminator? | confirmed | `rustc +nightly-2026-09-13 -Zunpretty=mir --crate-type=lib branch.rs` |
| MI002 | Is a function body presented as a graph of basic blocks? | confirmed | `rustc +nightly-2026-09-13 -Zunpretty=mir --crate-type=lib branch.rs` |
| MI003 | Does an owned value produce an explicit drop terminator? | confirmed | `rustc +nightly-2026-09-13 -Zunpretty=mir --crate-type=lib dropped.rs` |
| MI004 | Does the typed pre-MIR view exist and carry types? | confirmed | `rustc +nightly-2026-09-13 -Zunpretty=thir-flat --crate-type=lib branch.rs` |
| MI005 | Is there a semi-stable API surface for MIR? | confirmed | `rustc +nightly-2026-09-13 -Zunpretty=stable-mir --crate-type=lib branch.rs` |
| MI006 | Does the textual MIR view describe itself as an interface? | confirmed | `rustc +nightly-2026-09-13 -Zunpretty=mir --crate-type=lib branch.rs` |
| MI007 | Can the compiler show a macro's expansion? | confirmed | `rustc +nightly-2026-09-13 -Zunpretty=expanded --crate-type=lib macro.rs` |
| MI008 | Is a graphviz rendering of the control-flow graph available? | confirmed | `rustc +nightly-2026-09-13 -Zunpretty=mir-cfg --crate-type=lib branch.rs` |

