# `pcre2::ffi`

Crate `pcre2` · 3 public items · structured records in [`model/pcre2.ffi.json`](../model/pcre2.ffi.json)

## escape

`function` · `pcre2::ffi::escape`

Also reachable as `pcre2::escape`

```rust
fn escape(pattern: &str) -> String
```

Escapes all regular expression meta characters in `pattern`.

The string returned may be safely used as a literal in a regular
expression.

---

## is_jit_available

`function` · `pcre2::ffi::is_jit_available`

Also reachable as `grep_pcre2::is_jit_available`, `pcre2::is_jit_available`

```rust
fn is_jit_available() -> bool
```

Returns true if and only if PCRE2 believes that JIT is available.

---

## version

`function` · `pcre2::ffi::version`

Also reachable as `grep_pcre2::version`, `pcre2::version`

```rust
fn version() -> (u32, u32)
```

Returns the version of PCRE2 being used.

The tuple returned corresponds to the major and minor version, e.g.,
`(10, 32)`.

---
