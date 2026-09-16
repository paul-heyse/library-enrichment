# `pyrefly::alt::class::dataclass`

Crate `pyrefly` · 1 public items · structured records in [`model/pyrefly.alt.class.dataclass.json`](../model/pyrefly.alt.class.dataclass.json)

## ReplaceKind

`enum` · `pyrefly::alt::class::dataclass::ReplaceKind`

```rust
enum ReplaceKind
```

**Variants**: `Replace`, `Evolve`, `Assoc`

**Derives**: Clone, Copy

Which constructor-copy builtin `call_dataclasses_replace` is serving. Chosen by the caller so the
method itself doesn't re-derive dispatch from boolean flags.

---
