# `pyrefly_config::args`

Crate `pyrefly_config` · 2 public items · structured records in [`model/pyrefly_config.args.json`](../model/pyrefly_config.args.json)

## ConfigOverrideArgs

`struct` · `pyrefly_config::args::ConfigOverrideArgs`

```rust
struct ConfigOverrideArgs
```

**Implements**: `clap_builder::derive::Args`, `clap_builder::derive::CommandFactory`, `clap_builder::derive::FromArgMatches`, `clap_builder::derive::Parser`, `core::convert::From`

**Derives**: Clone, Debug, Default

**Methods** (9)

```rust
fn disable_project_excludes_heuristics(&self) -> Option<bool>
fn has_overrides(&self) -> bool
fn override_config(&self, config: ConfigFile) -> (ArcId<ConfigFile>, Vec<ConfigError>)
fn override_config_at(&self, config: ConfigFile, project_root: Option<&Path>) -> (ArcId<ConfigFile>, Vec<ConfigError>)
fn preset(&self) -> Option<Preset>
fn set_check_unannotated_defs_if_unset(&mut self, value: bool)
fn set_infer_return_types_if_unset(&mut self, value: InferReturnTypes)
fn set_untyped_def_behavior_if_unset(&mut self, behavior: UntypedDefBehavior)
fn validate(&self) -> anyhow::Result<()>
```

**via `clap_builder::derive::Args`**

```rust
fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command
fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command
fn group_id() -> Option<clap::Id>
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
fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>
```

**via `core::convert::From`**

```rust
fn from(env: EnvironmentArgs) -> Self
```

Full config overrides for the `check` command: environment resolution
plus type-checking behavior flags.

---

## EnvironmentArgs

`struct` · `pyrefly_config::args::EnvironmentArgs`

```rust
struct EnvironmentArgs
```

**Implements**: `clap_builder::derive::Args`, `clap_builder::derive::CommandFactory`, `clap_builder::derive::FromArgMatches`, `clap_builder::derive::Parser`

**Derives**: Clone, Debug, Default

**Methods** (4)

```rust
fn disable_project_excludes_heuristics(&self) -> Option<bool>
fn override_environment_config(&self, config: &mut ConfigFile)
fn preset(&self) -> Option<Preset>
fn validate(&self) -> anyhow::Result<()>
```

**via `clap_builder::derive::Args`**

```rust
fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command
fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command
fn group_id() -> Option<clap::Id>
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
fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>
```

Environment and module-resolution overrides shared by all commands that
load a Pyrefly config and resolve files (check, coverage, dump-config, etc.).

---
