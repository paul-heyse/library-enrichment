# `grep_cli::process`

Crate `grep-cli` · 3 public items · structured records in [`model/grep_cli.process.json`](../model/grep_cli.process.json)

## CommandError

`struct` · `grep_cli::process::CommandError`

Also reachable as `grep_cli::CommandError`

```rust
struct CommandError
```

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(ioerr: io::Error) -> CommandError
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

An error that can occur while running a command and reading its output.

This error can be seamlessly converted to an `io::Error` via a `From`
implementation.

---

## CommandReader

`struct` · `grep_cli::process::CommandReader`

Also reachable as `grep_cli::CommandReader`

```rust
struct CommandReader
```

**Implements**: `alloc::io::read::Read`, `core::ops::drop::Drop`

**Derives**: Debug

**Methods** (2)

```rust
fn close(&mut self) -> io::Result<()>
fn new(cmd: &mut process::Command) -> Result<CommandReader, CommandError>
```

**via `alloc::io::read::Read`**

```rust
fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A streaming reader for a command's output.

The purpose of this reader is to provide an easy way to execute processes
whose stdout is read in a streaming way while also making the processes'
stderr available when the process fails with an exit code. This makes it
possible to execute processes while surfacing the underlying failure mode
in the case of an error.

Moreover, by default, this reader will asynchronously read the processes'
stderr. This prevents subtle deadlocking bugs for noisy processes that
write a lot to stderr. Currently, the entire contents of stderr is read
on to the heap.

# Example

This example shows how to invoke `gzip` to decompress the contents of a
file. If the `gzip` command reports a failing exit status, then its stderr
is returned as an error.

```no_run
use std::{io::Read, process::Command};

use grep_cli::CommandReader;

let mut cmd = Command::new("gzip");
cmd.arg("-d").arg("-c").arg("/usr/share/man/man1/ls.1.gz");

let mut rdr = CommandReader::new(&mut cmd)?;
let mut contents = vec![];
rdr.read_to_end(&mut contents)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## CommandReaderBuilder

`struct` · `grep_cli::process::CommandReaderBuilder`

Also reachable as `grep_cli::CommandReaderBuilder`

```rust
struct CommandReaderBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (3)

```rust
fn async_stderr(&mut self, yes: bool) -> &mut CommandReaderBuilder
fn build(&self, command: &mut process::Command) -> Result<CommandReader, CommandError>
fn new() -> CommandReaderBuilder
```

Configures and builds a streaming reader for process output.

---
