# `pyrefly_config::file_kind`

Crate `pyrefly_config` · 1 public items · structured records in [`model/pyrefly_config.file_kind.json`](../model/pyrefly_config.file_kind.json)

## ConfigFileKind

`enum` · `pyrefly_config::file_kind::ConfigFileKind`

```rust
enum ConfigFileKind
```

**Variants**: `MyPy`, `Pyright`, `Pyrefly`, `Pyproject`

**Implements**: `clap_builder::derive::CommandFactory`, `clap_builder::derive::FromArgMatches`, `clap_builder::derive::Parser`, `clap_builder::derive::Subcommand`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug

**Methods** (3)

```rust
fn check_for_existing_config(&self, path: &Path) -> anyhow::Result<bool>
fn file_name(&self) -> &str
fn toml_identifier(self) -> String
```

**via `clap_builder::derive::CommandFactory`**

```rust
fn command<'b>() -> clap::Command
fn command_for_update<'b>() -> clap::Command
```

**via `clap_builder::derive::FromArgMatches`**

```rust
fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>
fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>
fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>
fn update_from_arg_matches_mut<'b>(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>
```

**via `clap_builder::derive::Subcommand`**

```rust
fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command
fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command
fn has_subcommand(__clap_name: &str) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Types of configuration files that can be detected or created.

---
