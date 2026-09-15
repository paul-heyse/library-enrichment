# `object_store::local`

Crate `object_store` · 1 public items · structured records in [`model/object_store.local.json`](../model/object_store.local.json)

## LocalFileSystem

`struct` · `object_store::local::LocalFileSystem`

```rust
struct LocalFileSystem
```

**Implements**: `core::fmt::Display`, `object_store::ObjectStore`

**Derives**: Clone, Debug, Default

**Methods** (4)

```rust
fn new() -> Self
fn new_with_prefix(prefix: impl AsRef<std::path::Path>) -> Result<Self>
fn path_to_filesystem(&self, location: &Path) -> Result<PathBuf>
fn with_automatic_cleanup(self, automatic_cleanup: bool) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `object_store::ObjectStore`**

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()>
fn delete_stream(&self, locations: BoxStream<'static, Result<Path>>) -> BoxStream<'static, Result<Path>>
async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult>
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> Result<Vec<Bytes>>
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, Result<ObjectMeta>>
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> Result<()>
```

Local filesystem storage providing an [`ObjectStore`] interface to files on
local disk. Can optionally be created with a directory prefix

# Path Semantics

This implementation follows the [file URI] scheme outlined in [RFC 3986]. In
particular paths are delimited by `/`

[file URI]: https://en.wikipedia.org/wiki/File_URI_scheme
[RFC 3986]: https://www.rfc-editor.org/rfc/rfc3986

# Path Semantics

[`LocalFileSystem`] will expose the path semantics of the underlying filesystem, which may
have additional restrictions beyond those enforced by [`Path`].

For example:

* Windows forbids certain filenames, e.g. `COM0`,
* Windows forbids folders with trailing `.`
* Windows forbids certain ASCII characters, e.g. `<` or `|`
* OS X forbids filenames containing `:`
* Leading `-` are discouraged on Unix systems where they may be interpreted as CLI flags
* Filesystems may have restrictions on the maximum path or path segment length
* Filesystem support for non-ASCII characters is inconsistent

Additionally some filesystems, such as NTFS, are case-insensitive, whilst others like
FAT don't preserve case at all. Further some filesystems support non-unicode character
sequences, such as unpaired UTF-16 surrogates, and [`LocalFileSystem`] will error on
encountering such sequences.

Finally, filenames matching the regex `/.*#\d+/`, e.g. `foo.parquet#123`, are not supported
by [`LocalFileSystem`] as they are used to provide atomic writes. Such files will be ignored
for listing operations, and attempting to address such a file will error.

# Tokio Compatibility

Tokio discourages performing blocking IO on a tokio worker thread, however,
no major operating systems have stable async file APIs. Therefore if called from
a tokio context, this will use [`tokio::runtime::Handle::spawn_blocking`] to dispatch
IO to a blocking thread pool, much like `tokio::fs` does under-the-hood.

If not called from a tokio context, this will perform IO on the current thread with
no additional complexity or overheads

# Symlinks

[`LocalFileSystem`] will follow symlinks as normal, however, it is worth noting:

* Broken symlinks will be silently ignored by listing operations
* No effort is made to prevent breaking symlinks when deleting files
* Symlinks that resolve to paths outside the root **will** be followed
* Mutating a file through one or more symlinks will mutate the underlying file
* Deleting a path that resolves to a symlink will only delete the symlink

# Cross-Filesystem Copy

[`LocalFileSystem::copy_opts`] is implemented using [`std::fs::hard_link`], and therefore
does not support copying across filesystem boundaries.

---
