# `ruff_python_formatter::cli`

Crate `ruff_python_formatter` · 3 public items · structured records in [`model/ruff_python_formatter.cli.json`](../model/ruff_python_formatter.cli.json)

## Emit

`enum` · `ruff_python_formatter::cli::Emit`

```rust
enum Emit
```

**Variants**: `Files`, `Stdout`

**Implements**: `clap_builder::derive::ValueEnum`

**Derives**: Clone, Debug

**via `clap_builder::derive::ValueEnum`**

```rust
fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>
fn value_variants<'a>() -> &'a [Self]
```

---

## format_and_debug_print

`function` · `ruff_python_formatter::cli::format_and_debug_print`

```rust
fn format_and_debug_print(source: &str, cli: &Cli, source_path: &std::path::Path) -> anyhow::Result<String>
```

---

## Cli

`struct` · `ruff_python_formatter::cli::Cli`

```rust
struct Cli
```

**Fields**: `files`, `emit`, `check`, `preview`, `print_ir`, `print_comments`, `skip_magic_trailing_comma`, `target_version`

**Implements**: `clap_builder::derive::Args`, `clap_builder::derive::CommandFactory`, `clap_builder::derive::FromArgMatches`, `clap_builder::derive::Parser`

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

---
