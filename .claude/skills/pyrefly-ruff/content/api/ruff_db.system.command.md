# `ruff_db::system::command`

Crate `ruff_db` · 2 public items · structured records in [`model/ruff_db.system.command.json`](../model/ruff_db.system.command.json)

## Command

`struct` · `ruff_db::system::command::Command`

Also reachable as `ruff_db::system::Command`

```rust
struct Command
```

**Derives**: Debug

**Methods** (10)

```rust
fn arg(&mut self, argument: impl Into<String>) -> &mut Self
fn args<I, S>(&mut self, arguments: I) -> &mut Self where I: IntoIterator<Item = S>, S: Into<String>
fn current_dir(&mut self, directory: impl AsRef<SystemPath>) -> &mut Self
fn env(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self
fn env_clear(&mut self) -> &mut Self
fn env_remove(&mut self, name: impl Into<String>) -> &mut Self
fn get_args(&self) -> &[String]
fn get_current_dir(&self) -> Option<&SystemPath>
fn get_executable(&self) -> &str
fn new(executable: impl Into<String>) -> Self
```

An owned description of a command to execute with a [`CommandExecutor`].

---

## CommandExecutor

`trait` · `ruff_db::system::command::CommandExecutor`

Also reachable as `ruff_db::system::CommandExecutor`

```rust
trait CommandExecutor: Send + Sync
```

**Implementors** (1)

- `ruff_db::system::test::TestSystem`

**Methods** (2)

```rust
fn dyn_clone(&self) -> Box<dyn CommandExecutor>
fn execute(&self, command: Command) -> Result<Output>
```

Executes [`Command`]s.

---
