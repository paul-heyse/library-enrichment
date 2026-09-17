# rustc_mir_dataflow

**Reach for it when** the question is whether something holds at a program point, not merely which edges exist.

## What it is for

how state evolves along execution paths: initialisation, liveness, borrows, moves, and any custom forward or backward analysis.

## What it cannot answer

anything MIR itself does not already carry -- it derives facts over the graph, it does not add to it.

## Getting it

|  |  |
|---|---|
| Obtained by | `rustc -Zdump-mir-dataflow=yes, or the Analysis trait inside the compiler` |
| Entry point | `rustc_mir_dataflow::Analysis` |
| Needs a build | yes |
| Needs a network | no |
| Stability | unstable; roughly ten signature-affecting commits a year, and one reversal |
| Crates pinned here | none -- this layer is the compiler itself |
| Indexed symbols | 0 |

## Questions routed here

- Is this local initialised at this point? — `rustc_mir_dataflow::impls::MaybeUninitializedPlaces`
- Is this local live here? — `rustc_mir_dataflow::impls::MaybeLiveLocals`
- How do I write my own dataflow analysis? — `rustc_mir_dataflow::Analysis`

## Executed evidence

| Probe | Question | Verdict | Command |
|---|---|---|---|
| DF001 | Can the compiler dump the results of its own dataflow analyses? | confirmed | `sh -c rm -rf df001 && rustc +nightly-2026-09-13 -Zdump-mir=all -Zdump-mir-dataflow=yes -Zd` |
| DF002 | Are initialisation and borrow analyses dumped alongside liveness? | confirmed | `sh -c ls df001 \| grep -cE 'ever_init\|borrows' \|\| true` |
| DF004 | Does NLL region inference output require -Zdump-mir-dataflow? | confirmed | `sh -c ls df001c \| grep -c regioncx \|\| true` |

