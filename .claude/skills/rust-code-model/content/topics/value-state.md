# How state evolves along a path

Initialisation, liveness, borrows, moves, and custom forward or backward analyses.

## Mental model

MIR gives the graph; this gives facts that hold *at a point* after a fixpoint over it. The distinction matters: 'which edges exist' is MIR, 'is this local initialised on every path that reaches here' is dataflow. If you remember this API, check it: `AnalysisDomain` no longer exists as a separate trait and `GenKillAnalysis` was removed outright.

## Questions this covers

| Question | Start at | Instead of |
|---|---|---|
| Is this local initialised at this point? | `rustc_mir_dataflow::impls::MaybeUninitializedPlaces` | mir |
| Is this local live here? | `rustc_mir_dataflow::impls::MaybeLiveLocals` | mir |
| How do I write my own dataflow analysis? | `rustc_mir_dataflow::Analysis` | dataflow |

## Why not the neighbouring layer

- **Is this local initialised at this point?** -- not `mir`: gives the graph; whether a fact holds along every path is a fixpoint over it
- **Is this local live here?** -- not `mir`: the same reason: liveness is derived over the graph, not recorded in it
- **How do I write my own dataflow analysis?** -- not `dataflow`: if you remember AnalysisDomain or GenKillAnalysis: both are gone, merged into Analysis and removed respectively

## What was executed

Each row ran against the pinned toolchain. A `confirmed` verdict means the probe
held *and* its control came out the other way; `recorded` means there was nothing
for a control to falsify.

| Probe | Question | Verdict |
|---|---|---|
| DF001 | Can the compiler dump the results of its own dataflow analyses? | confirmed |
| DF002 | Are initialisation and borrow analyses dumped alongside liveness? | confirmed |
| DF004 | Does NLL region inference output require -Zdump-mir-dataflow? | confirmed |

**DF001** — The analyses in rustc_mir_dataflow can be observed without linking it. The control is the same compilation without the dataflow flag, which produces MIR dumps but no dataflow ones, so the flag is what produces them.

**DF002** — ever_init, maybe_uninit, borrows and liveness are the analyses borrowck actually runs. The control is an invented analysis name, which rules out the listing matching anything asked of it.

**DF004** — No. Region inference and the nll MIR are dumped by -Zdump-mir=all on its own; only the dataflow .dot files -- borrows, ever_init, maybe_uninit, liveness -- need the extra flag. The control is the same directory checked for a dataflow dump, which is absent. Asking for the wrong flag is how a reader concludes a facility is unavailable.

