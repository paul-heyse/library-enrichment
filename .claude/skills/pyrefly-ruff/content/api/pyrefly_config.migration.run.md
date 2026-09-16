# `pyrefly_config::migration::run`

Crate `pyrefly_config` · 5 public items · structured records in [`model/pyrefly_config.migration.run.json`](../model/pyrefly_config.migration.run.json)

## MigratedConfigSource

`enum` · `pyrefly_config::migration::run::MigratedConfigSource`

```rust
enum MigratedConfigSource
```

**Variants**: `DedicatedFile`, `PyprojectToml`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

Where the migrated mypy / pyright settings physically lived: a
dedicated config file (`mypy.ini` / `pyrightconfig.json`), or a
`[tool.mypy]` / `[tool.pyright]` section of a `pyproject.toml`. The
surfaced wording differs ("`mypy.ini`" vs "`[tool.mypy]` in
`pyproject.toml`"), but the migrated settings themselves don't.

---

## MigratedFromKind

`enum` · `pyrefly_config::migration::run::MigratedFromKind`

```rust
enum MigratedFromKind
```

**Variants**: `Mypy`, `Pyright`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

Which type checker config we successfully migrated from, plus the
kind of file we read it out of. Used by the LSP/CLI to label the
synthesized config and explain to the user where the imported
settings came from.

---

## MigrationSource

`enum` · `pyrefly_config::migration::run::MigrationSource`

```rust
enum MigrationSource
```

**Variants**: `Auto`, `MyPy`, `Pyright`

**Implements**: `clap_builder::derive::ValueEnum`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `clap_builder::derive::ValueEnum`**

```rust
fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>
fn value_variants<'a>() -> &'a [Self]
```

Which type checker config to migrate from.

When set to a specific source (`MyPy` or `Pyright`), only that source is
tried — there is no fallback. `Auto` (the default) tries mypy first, then
pyright, preserving the historical behavior.

---

## config_migration

`function` · `pyrefly_config::migration::run::config_migration`

```rust
fn config_migration(path: &std::path::Path, migrate_from: MigrationSource, dry_run: bool, print_config: bool) -> anyhow::Result<std::path::PathBuf>
```

Migrate the config file at a given location (pyproject, mypy, pyright etc), producing a new file.
In some cases, e.g. pyproject, we will modify the original file in-place.

When `dry_run` is true, no files are created or modified: the migrated config
is computed and printed to the log, and the returned `PathBuf` is the path
where the config *would* have been written.

When `print_config` is true, the migrated config TOML is also written to
stdout (independent of `dry_run`), so downstream tooling can capture it.

---

## find_and_migrate_in_memory

`function` · `pyrefly_config::migration::run::find_and_migrate_in_memory`

```rust
fn find_and_migrate_in_memory(start: &std::path::Path) -> anyhow::Result<Option<(config::ConfigFile, MigratedFromKind)>>
```

Search upward from `start` for a mypy or pyright config (`mypy.ini`,
`pyrightconfig.json`, or a `pyproject.toml` with `[tool.mypy]` /
`[tool.pyright]`) and migrate it to a Pyrefly `ConfigFile` entirely in
memory — no files are written. The result is equivalent to what
`pyrefly init` would produce, only without touching disk.

Returns `Ok(None)` if no migrate-able config exists. Returns `Err` if a
candidate config was found but couldn't be parsed; callers that want to
fall back to a plain `Basic` preset on parse failure should catch the
error themselves (see `resolve_unconfigured_config`).

---
