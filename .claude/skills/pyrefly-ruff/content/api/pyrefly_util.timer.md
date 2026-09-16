# `pyrefly_util::timer`

Crate `pyrefly_util` · 2 public items · structured records in [`model/pyrefly_util.timer.json`](../model/pyrefly_util.timer.json)

## set_timing_enabled

`function` · `pyrefly_util::timer::set_timing_enabled`

```rust
fn set_timing_enabled(enabled: bool)
```

Enable or disable all [`Timer`]s process-wide. Enabled by default; a disabled
timer never calls `Instant::now()` and reports zero elapsed time.

---

## Timer

`struct` · `pyrefly_util::timer::Timer`

```rust
struct Timer
```

**Derives**: Clone, Copy, Debug

**Methods** (3)

```rust
fn elapsed(&self) -> Duration
fn elapsed_nanos(&self) -> u64
fn start() -> Self
```

A profiling timer that captures a start instant only when timing is enabled.
When disabled, `elapsed*` return zero and no `clock_gettime` syscall is made.

---
