# Type inference and annotation recovery

The checker infers types everywhere except function parameters: `def foo(x): return True` is treated as `def foo(x: Any) -> bool`. That asymmetry is the whole game when recovering annotations for an under-typed codebase -- returns and variables come back, parameters do not. `pyrefly_types::types::Type` is the representation; `AnswersSolver` is the engine, with 352 methods, and is the largest single type in this index.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `pyrefly_types::types::Type` | enum | 112 | [prose](../api/pyrefly_types.types.md#type) | [records](../model/pyrefly_types.types.json) |
| `pyrefly_types::stdlib::Stdlib` | struct | 82 | [prose](../api/pyrefly_types.stdlib.md#stdlib) | [records](../model/pyrefly_types.stdlib.json) |
| `pyrefly_types::heap::TypeHeap` | struct | 67 | [prose](../api/pyrefly_types.heap.md#typeheap) | [records](../model/pyrefly_types.heap.json) |
| `pyrefly::alt::answers_solver::AnswersSolver` | struct | 352 | [prose](../api/pyrefly.alt.answers_solver.md#answerssolver) | [records](../model/pyrefly.alt.answers_solver.json) |

## Upstream guides

- [`corpus/pyrefly/python-features-and-peps.mdx`](../corpus/pyrefly/python-features-and-peps.mdx)
- [`corpus/pyrefly/autotype.mdx`](../corpus/pyrefly/autotype.mdx)

## Decision rules

- Want annotations written into source? `pyrefly infer`, not a hand-rolled pass.
- Want the inferred type at one position? The LSP `hover` or `inlayHint`.
- Want to know whether a typing feature is supported at all? The conformance corpus.

## Anti-patterns

- Expecting inferred parameter types; they are `Any` by design.
- Treating an inferred type as a promise upstream made.

## Agent checklist

- `--untyped-def-behavior` changes what an unannotated def means; record which you used.
- Inference is flow-sensitive, so a type can narrow between two lines.
