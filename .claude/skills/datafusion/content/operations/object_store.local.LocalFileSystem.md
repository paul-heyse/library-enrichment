# `object_store::local::LocalFileSystem`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.local.LocalFileSystem.json).

<a id="op-972e6ecc4b3db74073f32c0b"></a>
## LocalFileSystem

`struct` · `object_store::local::LocalFileSystem` · object_store 0.13.2

```rust
struct LocalFileSystem
```

Source: `src/local.rs:201`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Local filesystem storage providing an [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) interface to files on
local disk. Can optionally be created with a directory prefix

# Path Semantics

This implementation follows the [file URI] scheme outlined in [RFC 3986]. In
particular paths are delimited by `/`

[file URI]: https://en.wikipedia.org/wiki/File_URI_scheme
[RFC 3986]: https://www.rfc-editor.org/rfc/rfc3986

# Path Semantics

[`LocalFileSystem`](../operations/object_store.local.LocalFileSystem.md#op-972e6ecc4b3db74073f32c0b) will expose the path semantics of the underlying filesystem, which may
have additional restrictions beyond those enforced by [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b).

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
sequences, such as unpaired UTF-16 surrogates, and [`LocalFileSystem`](../operations/object_store.local.LocalFileSystem.md#op-972e6ecc4b3db74073f32c0b) will error on
encountering such sequences.

Finally, filenames matching the regex `/.*#\d+/`, e.g. `foo.parquet#123`, are not supported
by [`LocalFileSystem`](../operations/object_store.local.LocalFileSystem.md#op-972e6ecc4b3db74073f32c0b) as they are used to provide atomic writes. Such files will be ignored
for listing operations, and attempting to address such a file will error.

# Tokio Compatibility

Tokio discourages performing blocking IO on a tokio worker thread, however,
no major operating systems have stable async file APIs. Therefore if called from
a tokio context, this will use [`tokio::runtime::Handle::spawn_blocking`] to dispatch
IO to a blocking thread pool, much like `tokio::fs` does under-the-hood.

If not called from a tokio context, this will perform IO on the current thread with
no additional complexity or overheads

# Symlinks

[`LocalFileSystem`](../operations/object_store.local.LocalFileSystem.md#op-972e6ecc4b3db74073f32c0b) will follow symlinks as normal, however, it is worth noting:

* Broken symlinks will be silently ignored by listing operations
* No effort is made to prevent breaking symlinks when deleting files
* Symlinks that resolve to paths outside the root **will** be followed
* Mutating a file through one or more symlinks will mutate the underlying file
* Deleting a path that resolves to a symlink will only delete the symlink

# Cross-Filesystem Copy

[`LocalFileSystem::copy_opts`](../operations/object_store.local.LocalFileSystem.md#op-e32cd6a5fa4552ccda02bac9) is implemented using [`std::fs::hard_link`], and therefore
does not support copying across filesystem boundaries.


Unresolved upstream links (retained, not inferred): ``tokio::runtime::Handle::spawn_blocking``, ``std::fs::hard_link``.

<a id="op-7498891aa5df249627347abd"></a>
## clone

`function` · `object_store::local::LocalFileSystem::clone` · object_store 0.13.2

```rust
fn clone(&self) -> LocalFileSystem
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 10], "end": [200, 15], "filename": "src/local.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/local.rs:200`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e32cd6a5fa4552ccda02bac9"></a>
## copy_opts

`function` · `object_store::local::LocalFileSystem::copy_opts` · object_store 0.13.2

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [329, 1], "end": [667, 2], "filename": "src/local.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/local.rs:544`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3dac0807c8d78f8aede69df3"></a>
## default

`function` · `object_store::local::LocalFileSystem::default` · object_store 0.13.2

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 1], "end": [222, 2], "filename": "src/local.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/local.rs:219`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18ca02f26f910f6ce4da1f23"></a>
## delete_stream

`function` · `object_store::local::LocalFileSystem::delete_stream` · object_store 0.13.2

```rust
fn delete_stream(&self, locations: BoxStream<'static, Result<Path>>) -> BoxStream<'static, Result<Path>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [329, 1], "end": [667, 2], "filename": "src/local.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/local.rs:462`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ffb5767a530397fcc20b97a"></a>
## fmt

`function` · `object_store::local::LocalFileSystem::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [216, 2], "filename": "src/local.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/local.rs:213`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3033e7964436c25c07999d8"></a>
## fmt

`function` · `object_store::local::LocalFileSystem::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 17], "end": [200, 22], "filename": "src/local.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/local.rs:200`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7c4a724c2bc491fa26262e0"></a>
## get_opts

`function` · `object_store::local::LocalFileSystem::get_opts` · object_store 0.13.2

```rust
async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [329, 1], "end": [667, 2], "filename": "src/local.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/local.rs:421`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecc7647be71598417a1adb01"></a>
## get_ranges

`function` · `object_store::local::LocalFileSystem::get_ranges` · object_store 0.13.2

```rust
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> Result<Vec<Bytes>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [329, 1], "end": [667, 2], "filename": "src/local.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/local.rs:447`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a59144be87b7c88304a21d10"></a>
## list

`function` · `object_store::local::LocalFileSystem::list` · object_store 0.13.2

```rust
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [329, 1], "end": [667, 2], "filename": "src/local.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/local.rs:481`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1de165d3859f586172e9aeaa"></a>
## list_with_delimiter

`function` · `object_store::local::LocalFileSystem::list_with_delimiter` · object_store 0.13.2

```rust
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [329, 1], "end": [667, 2], "filename": "src/local.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/local.rs:493`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80b8a93c4891308bf2b47175"></a>
## list_with_offset

`function` · `object_store::local::LocalFileSystem::list_with_offset` · object_store 0.13.2

```rust
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [329, 1], "end": [667, 2], "filename": "src/local.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/local.rs:485`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-512521d62621680f9913f857"></a>
## new

`function` · `object_store::local::LocalFileSystem::new` · object_store 0.13.2

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [224, 1], "end": [263, 2], "filename": "src/local.rs"}, "trait": null, "trait_path": null}`

Source: `src/local.rs:226`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create new filesystem storage with no prefix

<a id="op-32357d932e2a02d02fe2eddf"></a>
## new_with_prefix

`function` · `object_store::local::LocalFileSystem::new_with_prefix` · object_store 0.13.2

```rust
fn new_with_prefix(prefix: impl AsRef<std::path::Path>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [224, 1], "end": [263, 2], "filename": "src/local.rs"}, "trait": null, "trait_path": null}`

Source: `src/local.rs:239`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create new filesystem storage with `prefix` applied to all paths

Returns an error if the path does not exist


<a id="op-7101aec82997fb878b0c3cf7"></a>
## path_to_filesystem

`function` · `object_store::local::LocalFileSystem::path_to_filesystem` · object_store 0.13.2

```rust
fn path_to_filesystem(&self, location: &Path) -> Result<PathBuf>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [224, 1], "end": [263, 2], "filename": "src/local.rs"}, "trait": null, "trait_path": null}`

Source: `src/local.rs:254`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Return an absolute filesystem path of the given file location

<a id="op-e8781561598aa3ecc5367911"></a>
## put_multipart_opts

`function` · `object_store::local::LocalFileSystem::put_multipart_opts` · object_store 0.13.2

```rust
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [329, 1], "end": [667, 2], "filename": "src/local.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/local.rs:404`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8907263ecbdabc6ca9c853fb"></a>
## put_opts

`function` · `object_store::local::LocalFileSystem::put_opts` · object_store 0.13.2

```rust
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [329, 1], "end": [667, 2], "filename": "src/local.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/local.rs:330`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff2c2f2175d5101030ba1cc1"></a>
## rename_opts

`function` · `object_store::local::LocalFileSystem::rename_opts` · object_store 0.13.2

```rust
async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [329, 1], "end": [667, 2], "filename": "src/local.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/local.rs:620`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d270322ee887bec61797877c"></a>
## with_automatic_cleanup

`function` · `object_store::local::LocalFileSystem::with_automatic_cleanup` · object_store 0.13.2

```rust
fn with_automatic_cleanup(self, automatic_cleanup: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::local::LocalFileSystem", "path": "LocalFileSystem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [224, 1], "end": [263, 2], "filename": "src/local.rs"}, "trait": null, "trait_path": null}`

Source: `src/local.rs:259`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Enable automatic cleanup of empty directories when deleting files
