# `walkdir`

Crate `walkdir` · 4 public items · structured records in [`model/walkdir.json`](../model/walkdir.json)

## FilterEntry

`struct` · `walkdir::FilterEntry`

```rust
struct FilterEntry<I, P>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**Methods** (2)

```rust
fn filter_entry(self, predicate: P) -> FilterEntry<Self, P>
fn skip_current_dir(&mut self)
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Result<DirEntry>>
```

A recursive directory iterator that skips entries.

Values of this type are created by calling [`.filter_entry()`] on an
`IntoIter`, which is formed by calling [`.into_iter()`] on a `WalkDir`.

Directories that fail the predicate `P` are skipped. Namely, they are
never yielded and never descended into.

Entries that are skipped with the [`min_depth`] and [`max_depth`] options
are not passed through this filter.

If opening a handle to a directory resulted in an error, then it is yielded
and no corresponding call to the predicate is made.

Type parameter `I` refers to the underlying iterator and `P` refers to the
predicate, which is usually `FnMut(&DirEntry) -> bool`.

[`.filter_entry()`]: struct.IntoIter.html#method.filter_entry
[`.into_iter()`]: struct.WalkDir.html#into_iter.v
[`min_depth`]: struct.WalkDir.html#method.min_depth
[`max_depth`]: struct.WalkDir.html#method.max_depth

---

## IntoIter

`struct` · `walkdir::IntoIter`

```rust
struct IntoIter
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**Methods** (2)

```rust
fn filter_entry<P>(self, predicate: P) -> FilterEntry<Self, P> where P: FnMut(&DirEntry) -> bool
fn skip_current_dir(&mut self)
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Result<DirEntry>>
```

An iterator for recursively descending into a directory.

A value with this type must be constructed with the [`WalkDir`] type, which
uses a builder pattern to set options such as min/max depth, max open file
descriptors and whether the iterator should follow symbolic links. After
constructing a `WalkDir`, call [`.into_iter()`] at the end of the chain.

The order of elements yielded by this iterator is unspecified.

[`WalkDir`]: struct.WalkDir.html
[`.into_iter()`]: struct.WalkDir.html#into_iter.v

---

## WalkDir

`struct` · `walkdir::WalkDir`

```rust
struct WalkDir
```

**Implements**: `core::iter::traits::collect::IntoIterator`

**Derives**: Debug

**Methods** (11)

```rust
fn contents_first(self, yes: bool) -> Self
fn follow_links(self, yes: bool) -> Self
fn follow_root_links(self, yes: bool) -> Self
fn max_depth(self, depth: usize) -> Self
fn max_open(self, n: usize) -> Self
fn min_depth(self, depth: usize) -> Self
fn new<P: AsRef<Path>>(root: P) -> Self
fn same_file_system(self, yes: bool) -> Self
fn sort_by<F>(self, cmp: F) -> Self where F: FnMut(&DirEntry, &DirEntry) -> Ordering + Send + Sync + 'static
fn sort_by_file_name(self) -> Self
fn sort_by_key<K, F>(self, cmp: F) -> Self where F: FnMut(&DirEntry) -> K + Send + Sync + 'static, K: Ord
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> IntoIter
```

A builder to create an iterator for recursively walking a directory.

Results are returned in depth first fashion, with directories yielded
before their contents. If [`contents_first`] is true, contents are yielded
before their directories. The order is unspecified but if [`sort_by`] is
given, directory entries are sorted according to this function. Directory
entries `.` and `..` are always omitted.

If an error occurs at any point during iteration, then it is returned in
place of its corresponding directory entry and iteration continues as
normal. If an error occurs while opening a directory for reading, then it
is not descended into (but the error is still yielded by the iterator).
Iteration may be stopped at any time. When the iterator is destroyed, all
resources associated with it are freed.

[`contents_first`]: struct.WalkDir.html#method.contents_first
[`sort_by`]: struct.WalkDir.html#method.sort_by

# Usage

This type implements [`IntoIterator`] so that it may be used as the subject
of a `for` loop. You may need to call [`into_iter`] explicitly if you want
to use iterator adapters such as [`filter_entry`].

Idiomatic use of this type should use method chaining to set desired
options. For example, this only shows entries with a depth of `1`, `2` or
`3` (relative to `foo`):

```no_run
use walkdir::WalkDir;
# use walkdir::Error;

# fn try_main() -> Result<(), Error> {
for entry in WalkDir::new("foo").min_depth(1).max_depth(3) {
    println!("{}", entry?.path().display());
}
# Ok(())
# }
```

[`IntoIterator`]: https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html
[`into_iter`]: https://doc.rust-lang.org/nightly/core/iter/trait.IntoIterator.html#tymethod.into_iter
[`filter_entry`]: struct.IntoIter.html#method.filter_entry

Note that the iterator by default includes the top-most directory. Since
this is the only directory yielded with depth `0`, it is easy to ignore it
with the [`min_depth`] setting:

```no_run
use walkdir::WalkDir;
# use walkdir::Error;

# fn try_main() -> Result<(), Error> {
for entry in WalkDir::new("foo").min_depth(1) {
    println!("{}", entry?.path().display());
}
# Ok(())
# }
```

[`min_depth`]: struct.WalkDir.html#method.min_depth

This will only return descendents of the `foo` directory and not `foo`
itself.

# Loops

This iterator (like most/all recursive directory iterators) assumes that
no loops can be made with *hard* links on your file system. In particular,
this would require creating a hard link to a directory such that it creates
a loop. On most platforms, this operation is illegal.

Note that when following symbolic/soft links, loops are detected and an
error is reported.

---

## Result

`type_alias` · `walkdir::Result`

```rust
type Result<T> = ::std::result::Result<T, Error>
```

A result type for walkdir operations.

Note that this result type embeds the error type in this crate. This
is only useful if you care about the additional information provided by
the error (such as the path associated with the error or whether a loop
was dectected). If you want things to Just Work, then you can use
[`io::Result`] instead since the error type in this package will
automatically convert to an [`io::Result`] when using the [`try!`] macro.

[`io::Result`]: https://doc.rust-lang.org/stable/std/io/type.Result.html
[`try!`]: https://doc.rust-lang.org/stable/std/macro.try.html

---
