# `ruff::args`

Crate `ruff` · 19 public items · structured records in [`model/ruff.args.json`](../model/ruff.args.json)

## AnalyzeCommand

`enum` · `ruff::args::AnalyzeCommand`

```rust
enum AnalyzeCommand
```

**Variants**: `Graph`

**Implements**: `clap_builder::derive::FromArgMatches`, `clap_builder::derive::Subcommand`

**Derives**: Debug

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

---

## Command

`enum` · `ruff::args::Command`

```rust
enum Command
```

**Variants**: `Check`, `Rule`, `Config`, `Linter`, `Clean`, `GenerateShellCompletion`, `Format`, `Server`, `Analyze`, `Version`

**Implements**: `clap_builder::derive::FromArgMatches`, `clap_builder::derive::Subcommand`

**Derives**: Debug

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

---

## FormatRangeParseError

`enum` · `ruff::args::FormatRangeParseError`

```rust
enum FormatRangeParseError
```

**Variants**: `InvalidStart`, `InvalidEnd`, `StartGreaterThanEnd`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

---

## HelpFormat

`enum` · `ruff::args::HelpFormat`

```rust
enum HelpFormat
```

**Variants**: `Text`, `Json`

**Implements**: `clap_builder::derive::ValueEnum`

**Derives**: Clone, Copy, Debug

**via `clap_builder::derive::ValueEnum`**

```rust
fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>
fn value_variants<'a>() -> &'a [Self]
```

---

## LineColumnParseError

`enum` · `ruff::args::LineColumnParseError`

```rust
enum LineColumnParseError
```

**Variants**: `ZeroLineIndex`, `ZeroColumnIndex`, `ZeroLineAndColumnIndex`, `LineParseError`, `ColumnParseError`

**Derives**: Clone, Debug

---

## SingleConfigArgument

`enum` · `ruff::args::SingleConfigArgument`

```rust
enum SingleConfigArgument
```

**Variants**: `FilePath`, `SettingsOverride`

**Implements**: `clap_builder::builder::value_parser::ValueParserFactory`

**Derives**: Clone, Debug

**via `clap_builder::builder::value_parser::ValueParserFactory`**

```rust
fn value_parser() -> Self::Parser
```

Enumeration to represent a single `--config` argument
passed via the CLI.

Using the `--config` flag, users may pass 0 or 1 paths
to configuration files and an arbitrary number of
"inline TOML" overrides for specific settings.

For example:

```sh
ruff check --config "path/to/ruff.toml" --config "extend-select=['E501', 'F841']" --config "lint.per-file-ignores = {'some_file.py' = ['F841']}"
```

---

## AnalyzeGraphArgs

`struct` · `ruff::args::AnalyzeGraphArgs`

```rust
struct AnalyzeGraphArgs
```

**Derives**: Clone, Debug

CLI settings that are distinct from configuration (commands, lists of files, etc.).

---

## AnalyzeGraphCommand

`struct` · `ruff::args::AnalyzeGraphCommand`

```rust
struct AnalyzeGraphCommand
```

**Implements**: `clap_builder::derive::Args`, `clap_builder::derive::CommandFactory`, `clap_builder::derive::FromArgMatches`, `clap_builder::derive::Parser`

**Derives**: Clone, Debug

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

## Args

`struct` · `ruff::args::Args`

```rust
struct Args
```

**Implements**: `clap_builder::derive::Args`, `clap_builder::derive::CommandFactory`, `clap_builder::derive::FromArgMatches`, `clap_builder::derive::Parser`

**Derives**: Debug

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

## CheckCommand

`struct` · `ruff::args::CheckCommand`

```rust
struct CheckCommand
```

**Implements**: `clap_builder::derive::Args`, `clap_builder::derive::CommandFactory`, `clap_builder::derive::FromArgMatches`, `clap_builder::derive::Parser`

**Derives**: Clone, Debug

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

## ConfigArgumentParser

`struct` · `ruff::args::ConfigArgumentParser`

```rust
struct ConfigArgumentParser
```

**Implements**: `clap_builder::builder::value_parser::TypedValueParser`

**Derives**: Clone

**via `clap_builder::builder::value_parser::TypedValueParser`**

```rust
fn parse_ref(&self, cmd: &clap::Command, arg: Option<&clap::Arg>, value: &std::ffi::OsStr) -> Result<Self::Value, clap::Error>
```

---

## ConfigArguments

`struct` · `ruff::args::ConfigArguments`

```rust
struct ConfigArguments
```

**Implements**: `ruff_workspace::resolver::ConfigurationTransformer`

**Derives**: Default

**via `ruff_workspace::resolver::ConfigurationTransformer`**

```rust
fn transform(&self, config: Configuration) -> Configuration
```

Configuration-related arguments passed via the CLI.

---

## FormatArguments

`struct` · `ruff::args::FormatArguments`

```rust
struct FormatArguments
```

**Fields**: `check`, `no_cache`, `diff`, `files`, `stdin_filename`, `range`, `exit_non_zero_on_format`

CLI settings that are distinct from configuration (commands, lists of files,
etc.).

---

## FormatCommand

`struct` · `ruff::args::FormatCommand`

```rust
struct FormatCommand
```

**Fields**: `extend_exclude`

**Implements**: `clap_builder::derive::Args`, `clap_builder::derive::CommandFactory`, `clap_builder::derive::FromArgMatches`, `clap_builder::derive::Parser`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn partition(self, global_options: GlobalConfigArgs) -> anyhow::Result<(FormatArguments, ConfigArguments)>
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

---

## FormatRange

`struct` · `ruff::args::FormatRange`

```rust
struct FormatRange
```

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug

**via `core::str::traits::FromStr`**

```rust
fn from_str(value: &str) -> Result<Self, Self::Err>
```

A text range specified by line and column numbers.

---

## GlobalConfigArgs

`struct` · `ruff::args::GlobalConfigArgs`

```rust
struct GlobalConfigArgs
```

**Implements**: `clap_builder::derive::Args`, `clap_builder::derive::FromArgMatches`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn log_level(&self) -> LogLevel
```

**via `clap_builder::derive::Args`**

```rust
fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command
fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command
fn group_id() -> Option<clap::Id>
```

**via `clap_builder::derive::FromArgMatches`**

```rust
fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>
fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>
fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>
fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>
```

All configuration options that can be passed "globally",
i.e., can be passed to all subcommands

---

## LineColumn

`struct` · `ruff::args::LineColumn`

```rust
struct LineColumn
```

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(value: &str) -> Result<Self, Self::Err>
```

---

## LogLevelArgs

`struct` · `ruff::args::LogLevelArgs`

```rust
struct LogLevelArgs
```

**Implements**: `clap_builder::derive::Args`, `clap_builder::derive::FromArgMatches`

**Derives**: Clone, Debug, Default

**via `clap_builder::derive::Args`**

```rust
fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command
fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command
fn group_id() -> Option<clap::Id>
```

**via `clap_builder::derive::FromArgMatches`**

```rust
fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>
fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>
fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>
fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>
```

---

## ServerCommand

`struct` · `ruff::args::ServerCommand`

```rust
struct ServerCommand
```

**Implements**: `clap_builder::derive::Args`, `clap_builder::derive::CommandFactory`, `clap_builder::derive::FromArgMatches`, `clap_builder::derive::Parser`

**Derives**: Clone, Copy, Debug

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
