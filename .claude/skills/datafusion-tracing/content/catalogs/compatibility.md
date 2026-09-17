# Version compatibility

Every published release of `datafusion-tracing` — 16 of them, 16 with a readable example manifest. Registry read 2026-09-17; this is the one table here whose source is a release history rather than a pin, so it describes the world on that date.

## The rule that is written down nowhere else

`tracing-opentelemetry` runs exactly **+1 minor** ahead of the `opentelemetry` family. Pairing them by equal minor — the reading a version number invites — produces a compile error about trait bounds on the tracer, not about versions.

And `datafusion-tracing`'s own version number is DataFusion's major: the `55.0.0` release depends on `datafusion 55.0.0`. That has held at every release below, but the column is read from each release's manifest rather than assumed from its number.

## Releases

| datafusion-tracing | Published | datafusion | MSRV | Edition | opentelemetry* | tracing-opentelemetry |
|---|---|---|---|---|---|---|
| `55.0.0` | 2026-08-24 | `55.0.0` | 1.94.0 | 2024 | `0.31` | `0.32` |
| `54.0.0` | 2026-06-08 | `54.0.0` | 1.88.0 | 2024 | `0.31` | `0.32` |
| `53.0.2` | 2026-05-27 | `53.0.0` | 1.88.0 | 2024 | `0.31` | `0.32` |
| `53.0.1` | 2026-05-06 | `53.0.0` | 1.88.0 | 2024 | `0.31` | `0.32` |
| `53.0.0` | 2026-03-25 | `53.0.0` | 1.88.0 | 2024 | `0.31` | `0.32` |
| `52.0.0` | 2026-01-13 | `52.0.0` | 1.86.0 | 2024 | `0.31` | `0.32` |
| `51.0.0` | 2025-11-20 | `51.0.0` | 1.86.0 | 2024 | `0.30` | `0.31` |
| `50.0.2` | 2025-09-24 | `50.0.0` | 1.82.0 | 2021 | `0.30` | `0.31` |
| `50.0.1` | 2025-09-22 | `50.0.0` | 1.82.0 | 2021 | `0.30` | `0.31` |
| `50.0.0` *(yanked)* | 2025-09-16 | `50.0.0` | 1.82.0 | 2021 | `0.30` | `0.31` |
| `49.0.0` | 2025-08-02 | `49.0.0` | 1.82.0 | 2021 | `0.30` | `0.31` |
| `48.0.1` | 2025-06-20 | `48.0.0` | 1.82.0 | 2021 | `0.30` | `0.31` |
| `48.0.0` | 2025-06-12 | `48.0.0` | 1.82.0 | 2021 | `0.30` | `0.31` |
| `47.0.2` | 2025-06-20 | `47.0.0` | 1.82.0 | 2021 | `0.29` | `0.30` |
| `47.0.1` | 2025-04-24 | `47.0.0` | 1.82.0 | 2021 | `0.29` | `0.30` |
| `47.0.0` *(yanked)* | 2025-04-24 | `47.0.0` | 1.82.0 | 2021 | `0.29` | `0.30` |

`opentelemetry*` is the three crates that move together: `opentelemetry`, `opentelemetry_sdk` and `opentelemetry-otlp`. A single value means all three agree.

## Checking a manifest

ast-grep cannot parse TOML — measured: `ast-grep run --lang toml` is rejected by 0.45.3 with *`toml is not supported!`* — so there is no shipped rule for this and one would be the wrong tier. Read the pins out with ripgrep and compare them to the row above:

```
rg -N '^(tracing-opentelemetry|opentelemetry|opentelemetry_sdk|opentelemetry-otlp)\s*=' <your>/Cargo.toml
```

That block is not marked `bash`, deliberately: `verify.py` executes every fenced bash command in these pages and fails one that finds nothing, and this one reads a manifest that is not in this directory. A command shown as runnable here has been run.

## What a missing pair means

A combination absent from this table is **untested by upstream, not known incompatible**. In particular the newest releases on crates.io have already moved past the newest row here, and nothing in this repository establishes whether that pair composes — only that no release of `datafusion-tracing` has shipped against it.
