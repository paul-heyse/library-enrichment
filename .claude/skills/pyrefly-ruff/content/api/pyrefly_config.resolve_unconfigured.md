# `pyrefly_config::resolve_unconfigured`

Crate `pyrefly_config` · 2 public items · structured records in [`model/pyrefly_config.resolve_unconfigured.json`](../model/pyrefly_config.resolve_unconfigured.json)

## UnconfiguredOverride

`enum` · `pyrefly_config::resolve_unconfigured::UnconfiguredOverride`

```rust
enum UnconfiguredOverride
```

**Variants**: `Auto`, `Off`, `Basic`, `Legacy`, `Default`, `Strict`, `All`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(preset: Option<Preset>) -> Self
```

User-facing override for the unconfigured-config resolver. Maps to the
values of the `python.pyrefly.typeCheckingMode` VS Code setting plus
the implicit `Auto` (which means "let the resolver auto-detect").

Anything other than `Auto` skips auto-detection entirely and produces
an empty config with the corresponding preset.

---

## resolve_unconfigured_config

`function` · `pyrefly_config::resolve_unconfigured::resolve_unconfigured_config`

```rust
fn resolve_unconfigured_config(start: &std::path::Path, over: UnconfiguredOverride) -> config::ConfigFile
```

Build a `ConfigFile` for a project root that has no `pyrefly.toml` /
`[tool.pyrefly]`.

- Any `over` value other than `Auto` produces an empty `ConfigFile`
  with that preset and `synthesized_preset_reason = UserOverride`. The
  user has explicitly chosen a behavior; we don't auto-detect.
- `Auto` searches for a nearby mypy/pyright config and runs the
  in-memory migration. The migrated result already carries the right
  preset (`Legacy` for mypy, `None`/Default for pyright); we just
  stamp the matching `synthesized_preset_reason` on it.
- If nothing is found, falls back to `Preset::Basic` /
  `NoNearbyConfig`.
- If migration fails (malformed config), logs at debug and falls back
  to `Basic` / `NoNearbyConfig`. A broken nearby mypy.ini must not
  prevent Pyrefly from running.

---
