# `pyrefly_util::panic`

Crate `pyrefly_util` · 4 public items · structured records in [`model/pyrefly_util.panic.json`](../model/pyrefly_util.panic.json)

## PANIC_EXIT_CODE

`constant` · `pyrefly_util::panic::PANIC_EXIT_CODE`

```rust
const PANIC_EXIT_CODE: u8 = 101
```

The code that Rust uses for panics.

---

## exit_on_panic

`function` · `pyrefly_util::panic::exit_on_panic`

```rust
fn exit_on_panic()
```

When Pyrefly panics, we want to print a backtrace and exit.

We want to exit immediately because otherwise we'd have to ensure all our thread/lock code
properly bubbled up the panic, without producing a deadlock, or another panic, which is
a) a lot of work, and b) almost impossible to test.

---

## has_panicked

`function` · `pyrefly_util::panic::has_panicked`

```rust
fn has_panicked() -> bool
```

Returns true if Pyrefly has panicked up to this point.

---

## print_panic

`function` · `pyrefly_util::panic::print_panic`

```rust
fn print_panic(info: &std::panic::PanicHookInfo<'_>)
```

---
