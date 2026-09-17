# `regex_automata::util::escape`

Crate `regex-automata` · 2 public items · structured records in [`model/regex_automata.util.escape.json`](../model/regex_automata.util.escape.json)

## DebugByte

`struct` · `regex_automata::util::escape::DebugByte`

```rust
struct DebugByte
```

**Derives**: Clone, Copy, Debug

Provides a convenient `Debug` implementation for a `u8`.

The `Debug` impl treats the byte as an ASCII, and emits a human readable
representation of it. If the byte isn't ASCII, then it's emitted as a hex
escape sequence.

---

## DebugHaystack

`struct` · `regex_automata::util::escape::DebugHaystack`

```rust
struct DebugHaystack<'a>
```

**Derives**: Debug

Provides a convenient `Debug` implementation for `&[u8]`.

This generally works best when the bytes are presumed to be mostly UTF-8,
but will work for anything. For any bytes that aren't UTF-8, they are
emitted as hex escape sequences.

---
