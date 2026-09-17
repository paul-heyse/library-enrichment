# `ignore::walk`

Crate `ignore` · 7 public items · structured records in [`model/ignore.walk.json`](../model/ignore.walk.json)

## WalkState

`enum` · `ignore::walk::WalkState`

Also reachable as `ignore::WalkState`

```rust
enum WalkState
```

**Variants**: `Continue`, `Skip`, `Quit`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

WalkState is used in the parallel recursive directory iterator to indicate
whether walking should continue as normal, skip descending into a
particular directory or quit the walk entirely.

---

## DirEntry

`struct` · `ignore::walk::DirEntry`

Also reachable as `ignore::DirEntry`

```rust
struct DirEntry
```

**Derives**: Clone, Debug

**Methods** (10)

```rust
fn depth(&self) -> usize
fn error(&self) -> Option<&Error>
fn file_name(&self) -> &OsStr
fn file_type(&self) -> Option<FileType>
fn ino(&self) -> Option<u64>
fn into_path(self) -> PathBuf
fn is_stdin(&self) -> bool
fn metadata(&self) -> Result<Metadata, Error>
fn path(&self) -> &Path
fn path_is_symlink(&self) -> bool
```

A directory entry with a possible error attached.

The error typically refers to a problem parsing ignore files in a
particular directory.

---

## Walk

`struct` · `ignore::walk::Walk`

Also reachable as `ignore::Walk`

```rust
struct Walk
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Methods** (2)

```rust
fn from_iter<P: AsRef<Path>, I: IntoIterator<Item = P>>(paths: I) -> Walk
fn new<P: AsRef<Path>>(path: P) -> Walk
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Result<DirEntry, Error>>
```

Walk is a recursive directory iterator over file paths in one or more
directories.

Only file and directory paths matching the rules are returned. By default,
ignore files like `.gitignore` are respected. The precise matching rules
and precedence is explained in the documentation for `WalkBuilder`.

---

## WalkBuilder

`struct` · `ignore::walk::WalkBuilder`

Also reachable as `ignore::WalkBuilder`

```rust
struct WalkBuilder
```

**Derives**: Clone, Debug

**Methods** (30)

```rust
fn add<P: AsRef<Path>>(&mut self, path: P) -> &mut WalkBuilder
fn add_custom_ignore_filename<S: AsRef<OsStr>>(&mut self, file_name: S) -> &mut WalkBuilder
fn add_ignore<P: AsRef<Path>>(&mut self, path: P) -> Option<Error>
fn build(&self) -> Walk
fn build_parallel(&self) -> WalkParallel
fn current_dir(&mut self, cwd: impl Into<PathBuf>) -> &mut WalkBuilder
fn empty() -> WalkBuilder
fn filter_entry<P>(&mut self, filter: P) -> &mut WalkBuilder where P: Fn(&DirEntry) -> bool + Send + Sync + 'static
fn follow_links(&mut self, yes: bool) -> &mut WalkBuilder
fn from_iter<P: AsRef<Path>, I: IntoIterator<Item = P>>(paths: I) -> WalkBuilder
fn git_exclude(&mut self, yes: bool) -> &mut WalkBuilder
fn git_global(&mut self, yes: bool) -> &mut WalkBuilder
fn git_ignore(&mut self, yes: bool) -> &mut WalkBuilder
fn hidden(&mut self, yes: bool) -> &mut WalkBuilder
fn ignore(&mut self, yes: bool) -> &mut WalkBuilder
fn ignore_case_insensitive(&mut self, yes: bool) -> &mut WalkBuilder
fn max_depth(&mut self, depth: Option<usize>) -> &mut WalkBuilder
fn max_filesize(&mut self, filesize: Option<u64>) -> &mut WalkBuilder
fn min_depth(&mut self, depth: Option<usize>) -> &mut WalkBuilder
fn new<P: AsRef<Path>>(path: P) -> WalkBuilder
fn overrides(&mut self, overrides: Override) -> &mut WalkBuilder
fn parents(&mut self, yes: bool) -> &mut WalkBuilder
fn require_git(&mut self, yes: bool) -> &mut WalkBuilder
fn same_file_system(&mut self, yes: bool) -> &mut WalkBuilder
fn skip_stdout(&mut self, yes: bool) -> &mut WalkBuilder
fn sort_by_file_name<F>(&mut self, cmp: F) -> &mut WalkBuilder where F: Fn(&OsStr, &OsStr) -> Ordering + Send + Sync + 'static
fn sort_by_file_path<F>(&mut self, cmp: F) -> &mut WalkBuilder where F: Fn(&Path, &Path) -> Ordering + Send + Sync + 'static
fn standard_filters(&mut self, yes: bool) -> &mut WalkBuilder
fn threads(&mut self, n: usize) -> &mut WalkBuilder
fn types(&mut self, types: Types) -> &mut WalkBuilder
```

WalkBuilder builds a recursive directory iterator.

The builder supports a large number of configurable options. This includes
specific glob overrides, file type matching, toggling whether hidden
files are ignored or not, and of course, support for respecting gitignore
files.

By default, all ignore files found are respected. This includes `.ignore`,
`.gitignore`, `.git/info/exclude` and even your global gitignore
globs, usually found in `$XDG_CONFIG_HOME/git/ignore`.

Some standard recursive directory options are also supported, such as
limiting the recursive depth or whether to follow symbolic links (disabled
by default).

# Ignore rules

There are many rules that influence whether a particular file or directory
is skipped by this iterator. Those rules are documented here. Note that
the rules assume a default configuration.

* First, glob overrides are checked. If a path matches a glob override,
then matching stops. The path is then only skipped if the glob that matched
the path is an ignore glob. (An override glob is a whitelist glob unless it
starts with a `!`, in which case it is an ignore glob.)
* Second, ignore files are checked. Ignore files currently only come from
git ignore files (`.gitignore`, `.git/info/exclude` and the configured
global gitignore file), plain `.ignore` files, which have the same format
as gitignore files, or explicitly added ignore files. The precedence order
is: `.ignore`, `.gitignore`, `.git/info/exclude`, global gitignore and
finally explicitly added ignore files. Note that precedence between
different types of ignore files is not impacted by the directory hierarchy;
any `.ignore` file overrides all `.gitignore` files. Within each precedence
level, more nested ignore files have a higher precedence than less nested
ignore files.
* Third, if the previous step yields an ignore match, then all matching
is stopped and the path is skipped. If it yields a whitelist match, then
matching continues. A whitelist match can be overridden by a later matcher.
* Fourth, unless the path is a directory, the file type matcher is run on
the path. As above, if it yields an ignore match, then all matching is
stopped and the path is skipped. If it yields a whitelist match, then
matching continues.
* Fifth, if the path hasn't been whitelisted and it is hidden, then the
path is skipped.
* Sixth, unless the path is a directory, the size of the file is compared
against the max filesize limit. If it exceeds the limit, it is skipped.
* Seventh, if the path has made it this far then it is yielded in the
iterator.

---

## WalkParallel

`struct` · `ignore::walk::WalkParallel`

Also reachable as `ignore::WalkParallel`

```rust
struct WalkParallel
```

**Methods** (2)

```rust
fn run<'s, F>(self, mkf: F) where F: FnMut() -> Box<dyn FnMut(Result<DirEntry, Error>) -> WalkState + Send + 's>
fn visit(self, builder: &mut dyn ParallelVisitorBuilder<'_>)
```

WalkParallel is a parallel recursive directory iterator over files paths
in one or more directories.

Only file and directory paths matching the rules are returned. By default,
ignore files like `.gitignore` are respected. The precise matching rules
and precedence is explained in the documentation for `WalkBuilder`.

Unlike `Walk`, this uses multiple threads for traversing a directory.

---

## ParallelVisitor

`trait` · `ignore::walk::ParallelVisitor`

Also reachable as `ignore::ParallelVisitor`

```rust
trait ParallelVisitor: Send
```

**Methods** (1)

```rust
fn visit(&mut self, entry: Result<DirEntry, Error>) -> WalkState
```

Receives files and directories for the current thread.

Setup for the traversal can be implemented as part of
[`ParallelVisitorBuilder::build`]. Teardown when traversal finishes can be
implemented by implementing the `Drop` trait on your traversal type.

---

## ParallelVisitorBuilder

`trait` · `ignore::walk::ParallelVisitorBuilder`

Also reachable as `ignore::ParallelVisitorBuilder`

```rust
trait ParallelVisitorBuilder<'s>
```

**Methods** (1)

```rust
fn build(&mut self) -> Box<dyn ParallelVisitor + 's>
```

A builder for constructing a visitor when using [`WalkParallel::visit`].
The builder will be called for each thread started by `WalkParallel`. The
visitor returned from each builder is then called for every directory
entry.

---
