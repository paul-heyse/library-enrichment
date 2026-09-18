# `datafusion_datasource::display::FileGroupDisplay`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.display.FileGroupDisplay.json).

<a id="op-56ee614208490c164a4ab854"></a>
## FileGroupDisplay

`struct` · `datafusion_datasource::display::FileGroupDisplay` · datafusion-datasource 55.1.0

```rust
struct FileGroupDisplay<'a>
```

Source: `src/display.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A wrapper to customize partitioned group of files display

Prints in the format:
```text
[file1, file2,...]
```

<a id="op-2ca66fc321fd5e2735f4e080"></a>
## 0

`struct_field` · `datafusion_datasource::display::FileGroupDisplay::0` · datafusion-datasource 55.1.0

```rust
0: &'a file_groups::FileGroup
```

Source: `src/display.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8251dcf820e78108f5357992"></a>
## fmt

`function` · `datafusion_datasource::display::FileGroupDisplay::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource::display::FileGroupDisplay", "path": "FileGroupDisplay"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 10], "end": [61, 15], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/display.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f727d45e23f21c39fdf049a"></a>
## fmt_as

`function` · `datafusion_datasource::display::FileGroupDisplay::fmt_as` · datafusion-datasource 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> FmtResult
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_datasource::display::FileGroupDisplay", "path": "FileGroupDisplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [91, 2], "filename": "src/display.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/display.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
