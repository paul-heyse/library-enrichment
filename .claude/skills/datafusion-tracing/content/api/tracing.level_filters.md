# `tracing::level_filters`

Crate `tracing` · 1 public items · structured records in [`model/tracing.level_filters.json`](../model/tracing.level_filters.json)

## STATIC_MAX_LEVEL

`constant` · `tracing::level_filters::STATIC_MAX_LEVEL`

```rust
const STATIC_MAX_LEVEL: LevelFilter = _
```

The statically configured maximum trace level.

See the [module-level documentation] for information on how to configure
this.

This value is checked by the `event!` and `span!` macros. Code that
manually constructs events or spans via the `Event::record` function or
`Span` constructors should compare the level against this value to
determine if those spans or events are enabled.

[module-level documentation]: self#compile-time-filters

---
