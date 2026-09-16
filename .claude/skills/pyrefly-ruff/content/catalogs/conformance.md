# Typing conformance

Measured by upstream against the Python typing council's conformance suite, and
committed to the repository rather than asserted:

| | |
|---|---:|
| Tests | 144 |
| Pass | 139 |
| Fail | 5 |
| Pass rate | 0.97 |
| Recorded deviations | 42 |

A test passes when the checker's diagnostics match the suite's expectations exactly, so
a failure is a **deviation from the specification**, not a crash. The deviations are
per-line and readable in `corpus/conformance/third_party/conformance.result`.

## The 5 failing areas

- `annotations_forward_refs.py` — 2 deviations
- `generics_mixed_variance_inference.py` — 3 deviations
- `generics_paramspec_variance.py` — 20 deviations
- `generics_typevartuple_variance.py` — 16 deviations
- `protocols_variance.py` — 1 deviations

Every deviation, including those in otherwise-passing files, is in
`content/index/conformance.tsv`:

```bash
rg -P '\tfail\t' content/index/conformance.tsv | cut -f2,4
```

The suite's own sources are in `corpus/conformance/third_party/`, so a question such as
whether PEP 695 type parameter defaults are handled is answered by reading the test
rather than by trying it.
