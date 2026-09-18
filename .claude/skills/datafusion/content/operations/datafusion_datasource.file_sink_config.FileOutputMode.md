# `datafusion_datasource::file_sink_config::FileOutputMode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_sink_config.FileOutputMode.json).

<a id="op-1f7d911317403d696cf6ec4e"></a>
## FileOutputMode

`enum` · `datafusion_datasource::file_sink_config::FileOutputMode` · datafusion-datasource 55.1.0

```rust
enum FileOutputMode
```

Source: `src/file_sink_config.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Determines how `FileSink` output paths are interpreted.

<a id="op-c621fb0b1ca3b68948771d36"></a>
## Automatic

`variant` · `datafusion_datasource::file_sink_config::FileOutputMode::Automatic` · datafusion-datasource 55.1.0

```rust
Automatic
```

Source: `src/file_sink_config.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Infer output mode from the output URL (for example, by extension / trailing `/`).

<a id="op-b3d0057c009eb5f495fa034a"></a>
## Directory

`variant` · `datafusion_datasource::file_sink_config::FileOutputMode::Directory` · datafusion-datasource 55.1.0

```rust
Directory
```

Source: `src/file_sink_config.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Write to a directory under the output path with generated filenames.

<a id="op-0b5ccb49e7fa85bc5510802b"></a>
## SingleFile

`variant` · `datafusion_datasource::file_sink_config::FileOutputMode::SingleFile` · datafusion-datasource 55.1.0

```rust
SingleFile
```

Source: `src/file_sink_config.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Write to a single output file at the exact output path.

<a id="op-effce519f842f7960cc0598c"></a>
## clone

`function` · `datafusion_datasource::file_sink_config::FileOutputMode::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> FileOutputMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_sink_config::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 22], "filename": "src/file_sink_config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file_sink_config.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6075173b3db7bea4a6f7e57e"></a>
## default

`function` · `datafusion_datasource::file_sink_config::FileOutputMode::default` · datafusion-datasource 55.1.0

```rust
fn default() -> FileOutputMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_sink_config::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 45], "end": [39, 52], "filename": "src/file_sink_config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_sink_config.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a489390fdb36a852536019d5"></a>
## eq

`function` · `datafusion_datasource::file_sink_config::FileOutputMode::eq` · datafusion-datasource 55.1.0

```rust
fn eq(&self, other: &FileOutputMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_sink_config::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 30], "end": [39, 39], "filename": "src/file_sink_config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file_sink_config.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c06b9b6574a7d7ab461f60e1"></a>
## fmt

`function` · `datafusion_datasource::file_sink_config::FileOutputMode::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_sink_config::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/file_sink_config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_sink_config.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3837507495f275f644b846c5"></a>
## from

`function` · `datafusion_datasource::file_sink_config::FileOutputMode::from` · datafusion-datasource 55.1.0

```rust
fn from(value: Option<bool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_sink_config::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [72, 2], "filename": "src/file_sink_config.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/file_sink_config.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b358a1b83650f2cc8b6f5a7"></a>
## single_file_output

`function` · `datafusion_datasource::file_sink_config::FileOutputMode::single_file_output` · datafusion-datasource 55.1.0

```rust
fn single_file_output(self, base_output_path: &ListingTableUrl) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_sink_config::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [62, 2], "filename": "src/file_sink_config.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_sink_config.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Resolve this mode into a `single_file_output` boolean for the demuxer.
