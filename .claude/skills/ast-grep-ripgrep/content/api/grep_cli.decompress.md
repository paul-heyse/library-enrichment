# `grep_cli::decompress`

Crate `grep-cli` · 5 public items · structured records in [`model/grep_cli.decompress.json`](../model/grep_cli.decompress.json)

## resolve_binary

`function` · `grep_cli::decompress::resolve_binary`

Also reachable as `grep_cli::resolve_binary`

```rust
fn resolve_binary<P: AsRef<std::path::Path>>(prog: P) -> Result<std::path::PathBuf, process::CommandError>
```

Resolves a path to a program to a path by searching for the program in
`PATH`.

If the program could not be resolved, then an error is returned.

The purpose of doing this instead of passing the path to the program
directly to Command::new is that Command::new will hand relative paths
to CreateProcess on Windows, which will implicitly search the current
working directory for the executable. This could be undesirable for
security reasons. e.g., running ripgrep with the -z/--search-zip flag on an
untrusted directory tree could result in arbitrary programs executing on
Windows.

Note that this could still return a relative path if PATH contains a
relative path. We permit this since it is assumed that the user has set
this explicitly, and thus, desires this behavior.

# Platform behavior

On non-Windows, this is a no-op.

---

## DecompressionMatcher

`struct` · `grep_cli::decompress::DecompressionMatcher`

Also reachable as `grep_cli::DecompressionMatcher`

```rust
struct DecompressionMatcher
```

**Derives**: Clone, Debug, Default

**Methods** (3)

```rust
fn command<P: AsRef<Path>>(&self, path: P) -> Option<Command>
fn has_command<P: AsRef<Path>>(&self, path: P) -> bool
fn new() -> DecompressionMatcher
```

A matcher for determining how to decompress files.

---

## DecompressionMatcherBuilder

`struct` · `grep_cli::decompress::DecompressionMatcherBuilder`

Also reachable as `grep_cli::DecompressionMatcherBuilder`

```rust
struct DecompressionMatcherBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (5)

```rust
fn associate<P, I, A>(&mut self, glob: &str, program: P, args: I) -> &mut DecompressionMatcherBuilder where P: AsRef<OsStr>, I: IntoIterator<Item = A>, A: AsRef<OsStr>
fn build(&self) -> Result<DecompressionMatcher, CommandError>
fn defaults(&mut self, yes: bool) -> &mut DecompressionMatcherBuilder
fn new() -> DecompressionMatcherBuilder
fn try_associate<P, I, A>(&mut self, glob: &str, program: P, args: I) -> Result<&mut DecompressionMatcherBuilder, CommandError> where P: AsRef<OsStr>, I: IntoIterator<Item = A>, A: AsRef<OsStr>
```

A builder for a matcher that determines which files get decompressed.

---

## DecompressionReader

`struct` · `grep_cli::decompress::DecompressionReader`

Also reachable as `grep_cli::DecompressionReader`

```rust
struct DecompressionReader
```

**Implements**: `alloc::io::read::Read`

**Derives**: Debug

**Methods** (2)

```rust
fn close(&mut self) -> io::Result<()>
fn new<P: AsRef<Path>>(path: P) -> Result<DecompressionReader, CommandError>
```

**via `alloc::io::read::Read`**

```rust
fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
```

A streaming reader for decompressing the contents of a file.

The purpose of this reader is to provide a seamless way to decompress the
contents of file using existing tools in the current environment. This is
meant to be an alternative to using decompression libraries in favor of the
simplicity and portability of using external commands such as `gzip` and
`xz`. This does impose the overhead of spawning a process, so other means
for performing decompression should be sought if this overhead isn't
acceptable.

A decompression reader comes with a default set of matching rules that are
meant to associate file paths with the corresponding command to use to
decompress them. For example, a glob like `*.gz` matches gzip compressed
files with the command `gzip -d -c`. If a file path does not match any
existing rules, or if it matches a rule whose command does not exist in the
current environment, then the decompression reader passes through the
contents of the underlying file without doing any decompression.

The default matching rules are probably good enough for most cases, and if
they require revision, pull requests are welcome. In cases where they must
be changed or extended, they can be customized through the use of
[`DecompressionMatcherBuilder`] and [`DecompressionReaderBuilder`].

By default, this reader will asynchronously read the processes' stderr.
This prevents subtle deadlocking bugs for noisy processes that write a lot
to stderr. Currently, the entire contents of stderr is read on to the heap.

# Example

This example shows how to read the decompressed contents of a file without
needing to explicitly choose the decompression command to run.

Note that if you need to decompress multiple files, it is better to use
`DecompressionReaderBuilder`, which will amortize the cost of compiling the
matcher.

```no_run
use std::{io::Read, process::Command};

use grep_cli::DecompressionReader;

let mut rdr = DecompressionReader::new("/usr/share/man/man1/ls.1.gz")?;
let mut contents = vec![];
rdr.read_to_end(&mut contents)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## DecompressionReaderBuilder

`struct` · `grep_cli::decompress::DecompressionReaderBuilder`

Also reachable as `grep_cli::DecompressionReaderBuilder`

```rust
struct DecompressionReaderBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (5)

```rust
fn async_stderr(&mut self, yes: bool) -> &mut DecompressionReaderBuilder
fn build<P: AsRef<Path>>(&self, path: P) -> Result<DecompressionReader, CommandError>
fn get_matcher(&self) -> &DecompressionMatcher
fn matcher(&mut self, matcher: DecompressionMatcher) -> &mut DecompressionReaderBuilder
fn new() -> DecompressionReaderBuilder
```

Configures and builds a streaming reader for decompressing data.

---
