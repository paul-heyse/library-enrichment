# `datafusion_datasource::url::ListingTableUrl`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.url.ListingTableUrl.json).

<a id="op-1b5851ef006daad7359f3a88"></a>
## ListingTableUrl

`struct` · `datafusion_datasource::url::ListingTableUrl` · datafusion-datasource 55.1.0

```rust
struct ListingTableUrl
```

Source: `src/url.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A parsed URL identifying files for a listing table, see [`ListingTableUrl::parse`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-12f315f3fad747e727321fe7)
for more information on the supported expressions

<a id="op-2edcc1b285732fe4e45280ea"></a>
## as_ref

`function` · `datafusion_datasource::url::ListingTableUrl::as_ref` · datafusion-datasource 55.1.0

```rust
fn as_ref(&self) -> &Url
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [470, 1], "end": [474, 2], "filename": "src/url.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "url::Url", "path": "Url"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/url.rs:471`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbb06061a447ce3d963d9341"></a>
## as_ref

`function` · `datafusion_datasource::url::ListingTableUrl::as_ref` · datafusion-datasource 55.1.0

```rust
fn as_ref(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [464, 1], "end": [468, 2], "filename": "src/url.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/url.rs:465`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-627c55498bb12e9f79cfe18d"></a>
## as_str

`function` · `datafusion_datasource::url::ListingTableUrl::as_str` · datafusion-datasource 55.1.0

```rust
fn as_str(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:318`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns this [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88) as a string

<a id="op-7346749cb7c4340b8e331b99"></a>
## clone

`function` · `datafusion_datasource::url::ListingTableUrl::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> ListingTableUrl
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 17], "end": [38, 22], "filename": "src/url.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/url.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-facb85aad3334558cf354c90"></a>
## contains

`function` · `datafusion_datasource::url::ListingTableUrl::contains` · datafusion-datasource 55.1.0

```rust
fn contains(&self, path: &Path, ignore_subdirectory: bool) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns `true` if `path` matches this [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88)

<a id="op-c085d5f6e01459084fa36070"></a>
## eq

`function` · `datafusion_datasource::url::ListingTableUrl::eq` · datafusion-datasource 55.1.0

```rust
fn eq(&self, other: &ListingTableUrl) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 28], "end": [38, 37], "filename": "src/url.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/url.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c1d0e5c43c2b5d404bb844a"></a>
## file_extension

`function` · `datafusion_datasource::url::ListingTableUrl::file_extension` · datafusion-datasource 55.1.0

```rust
fn file_extension(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:220`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns the file extension of the last path segment if it exists

Examples:
```rust
use datafusion_datasource::ListingTableUrl;
let url = ListingTableUrl::parse("file:///foo/bar.csv").unwrap();
assert_eq!(url.file_extension(), Some("csv"));
let url = ListingTableUrl::parse("file:///foo/bar").unwrap();
assert_eq!(url.file_extension(), None);
let url = ListingTableUrl::parse("file:///foo/bar.").unwrap();
assert_eq!(url.file_extension(), None);
```

<a id="op-9b7923610beccbaae6f6d4e9"></a>
## fmt

`function` · `datafusion_datasource::url::ListingTableUrl::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/url.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/url.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9bd72013f854cd0c562daf5"></a>
## fmt

`function` · `datafusion_datasource::url::ListingTableUrl::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [476, 1], "end": [480, 2], "filename": "src/url.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/url.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfa12d6907febfbd96fe6c41"></a>
## get_glob

`function` · `datafusion_datasource::url::ListingTableUrl::get_glob` · datafusion-datasource 55.1.0

```rust
fn get_glob(&self) -> &Option<Pattern>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return the `glob` for [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88)

<a id="op-739b085c9d16f88d544b31f9"></a>
## get_table_ref

`function` · `datafusion_datasource::url::ListingTableUrl::get_table_ref` · datafusion-datasource 55.1.0

```rust
fn get_table_ref(&self) -> &Option<TableReference>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return the table reference for this [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88)

<a id="op-5c07ba32ec0533c5de4546e4"></a>
## get_url

`function` · `datafusion_datasource::url::ListingTableUrl::get_url` · datafusion-datasource 55.1.0

```rust
fn get_url(&self) -> &Url
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return the `url` for [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88)

<a id="op-8f704f00adc3898a703cc242"></a>
## hash

`function` · `datafusion_datasource::url::ListingTableUrl::hash` · datafusion-datasource 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 39], "end": [38, 43], "filename": "src/url.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/url.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-078947632864ed4a85fab147"></a>
## is_collection

`function` · `datafusion_datasource::url::ListingTableUrl::is_collection` · datafusion-datasource 55.1.0

```rust
fn is_collection(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns `true` if `path` refers to a collection of objects

<a id="op-b34682b5bfd224173bd03e5d"></a>
## is_folder

`function` · `datafusion_datasource::url::ListingTableUrl::is_folder` · datafusion-datasource 55.1.0

```rust
fn is_folder(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:329`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns true if the [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88) points to the folder

<a id="op-f47cb7955fea047c4f1d2bd6"></a>
## list_all_files

`function` · `datafusion_datasource::url::ListingTableUrl::list_all_files` · datafusion-datasource 55.1.0

```rust
async fn list_all_files<'a>(&'a self, ctx: &'a dyn Session, store: &'a dyn ObjectStore, file_extension: &'a str) -> Result<BoxStream<'a, Result<ObjectMeta>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

List all files identified by this [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88) for the provided `file_extension`

<a id="op-7372498b865446fd714aad29"></a>
## list_prefixed_files

`function` · `datafusion_datasource::url::ListingTableUrl::list_prefixed_files` · datafusion-datasource 55.1.0

```rust
async fn list_prefixed_files<'a>(&'a self, ctx: &'a dyn Session, store: &'a dyn ObjectStore, prefix: Option<Path>, file_extension: &'a str) -> Result<BoxStream<'a, Result<ObjectMeta>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

List all files identified by this [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88) for the provided `file_extension`,
optionally filtering by a path prefix

<a id="op-bcf2ee45342f735b97f3c8fd"></a>
## object_store

`function` · `datafusion_datasource::url::ListingTableUrl::object_store` · datafusion-datasource 55.1.0

```rust
fn object_store(&self) -> ObjectStoreUrl
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:323`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return the [`ObjectStoreUrl`](../operations/datafusion_execution.object_store.ObjectStoreUrl.md#op-80582a87395549e6a527145e) for this [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88)

<a id="op-12f315f3fad747e727321fe7"></a>
## parse

`function` · `datafusion_datasource::url::ListingTableUrl::parse` · datafusion-datasource 55.1.0

```rust
fn parse(s: impl AsRef<str>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Parse a provided string as a `ListingTableUrl`

A URL can either refer to a single object, or a collection of objects with a
common prefix, with the presence of a trailing `/` indicating a collection.

For example, `file:///foo.txt` refers to the file at `/foo.txt`, whereas
`file:///foo/` refers to all the files under the directory `/foo` and its
subdirectories.

Similarly `s3://BUCKET/blob.csv` refers to `blob.csv` in the S3 bucket `BUCKET`,
whereas `s3://BUCKET/foo/` refers to all objects with the prefix `foo/` in the
S3 bucket `BUCKET`

# URL Encoding

URL paths are expected to be URL-encoded. That is, the URL for a file named `bar%2Efoo`
would be `file:///bar%252Efoo`, as per the [URL] specification.

It should be noted that some tools, such as the AWS CLI, take a different approach and
instead interpret the URL path verbatim. For example the object `bar%2Efoo` would be
addressed as `s3://BUCKET/bar%252Efoo` using [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88) but `s3://BUCKET/bar%2Efoo`
when using the aws-cli.

# Paths without a Scheme

If no scheme is provided, or the string is an absolute filesystem path
as determined by [`std::path::Path::is_absolute`], the string will be
interpreted as a path on the local filesystem using the operating
system's standard path delimiter, i.e. `\` on Windows, `/` on Unix.

If the path contains any of `'?', '*', '['`, it will be considered
a glob expression and resolved as described in the section below.

Otherwise, the path will be resolved to an absolute path based on the current
working directory, and converted to a [file URI].

If the path already exists in the local filesystem this will be used to determine if this
[`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88) refers to a collection or a single object, otherwise the presence
of a trailing path delimiter will be used to indicate a directory. For the avoidance
of ambiguity it is recommended users always include trailing `/` when intending to
refer to a directory.

## Glob File Paths

If no scheme is provided, and the path contains a glob expression, it will
be resolved as follows.

The string up to the first path segment containing a glob expression will be extracted,
and resolved in the same manner as a normal scheme-less path above.

The remaining string will be interpreted as a [`glob::Pattern`] and used as a
filter when listing files from object storage

[file URI]: https://en.wikipedia.org/wiki/File_URI_scheme
[URL]: https://url.spec.whatwg.org/

Unresolved upstream links (retained, not inferred): ``glob::Pattern``, ``std::path::Path::is_absolute``.

<a id="op-9f5e7ec776e465182faecc92"></a>
## prefix

`function` · `datafusion_datasource::url::ListingTableUrl::prefix` · datafusion-datasource 55.1.0

```rust
fn prefix(&self) -> &Path
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return the URL path not excluding any glob expression

If [`Self::is_collection`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-078947632864ed4a85fab147), this is the listing prefix
Otherwise, this is the path to the object

<a id="op-a6b4341121b91786afc4b504"></a>
## scheme

`function` · `datafusion_datasource::url::ListingTableUrl::scheme` · datafusion-datasource 55.1.0

```rust
fn scheme(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns the URL scheme

<a id="op-1e9ae9437c25f06c5db74987"></a>
## strip_prefix

`function` · `datafusion_datasource::url::ListingTableUrl::strip_prefix` · datafusion-datasource 55.1.0

```rust
fn strip_prefix<'a, 'b: 'a>(&'a self, path: &'b Path) -> Option<impl Iterator<Item = &'b str> + 'a>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:234`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Strips the prefix of this [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88) from the provided path, returning
an iterator of the remaining path segments

<a id="op-9da57ac4791e3690fa79dd43"></a>
## try_new

`function` · `datafusion_datasource::url::ListingTableUrl::try_new` · datafusion-datasource 55.1.0

```rust
fn try_new(url: Url, glob: Option<Pattern>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Creates a new [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88) from a url and optional glob expression

[`Self::parse`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-12f315f3fad747e727321fe7) supports glob expression only for file system paths.
However, some applications may want to support glob expression for URLs with a scheme.
The application can split the URL into a base URL and a glob expression and use this method
to create a [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88).

<a id="op-af1b039907a01e7ccce6bb5b"></a>
## with_glob

`function` · `datafusion_datasource::url::ListingTableUrl::with_glob` · datafusion-datasource 55.1.0

```rust
fn with_glob(self, glob: &str) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns a copy of current [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88) with a specified `glob`

<a id="op-9169234e4e5eee95b51c7697"></a>
## with_table_ref

`function` · `datafusion_datasource::url::ListingTableUrl::with_table_ref` · datafusion-datasource 55.1.0

```rust
fn with_table_ref(self, table_ref: TableReference) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::url::ListingTableUrl", "path": "ListingTableUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [360, 2], "filename": "src/url.rs"}, "trait": null, "trait_path": null}`

Source: `src/url.rs:351`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the table reference for this [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88)
