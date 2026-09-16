# Embedding the crates in Rust

One release, two version lines: `ruff`, `ruff_linter` and `ruff_wasm` are 0.16.7 while every library crate is 0.0.13, and `ruff_linter 0.16.7` depends on the library crates at `^0.0.13`. Writing `ruff_python_ast = "0.16.7"` selects a version that does not exist. The checker publishes nothing usable at all -- its crates.io entry is a name squat at 0.0.1 -- so embedding it means vendoring from the tagged source. `../catalogs/crate-map.md` has the per-crate breakdown.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `ruff_text_size::range::TextRange` | struct | 39 | [prose](../api/ruff_text_size.range.md#textrange) | [records](../model/ruff_text_size.range.json) |
| `ruff_source_file::SourceFile` | struct | 10 | [prose](../api/ruff_source_file.md#sourcefile) | [records](../model/ruff_source_file.json) |
| `ruff_index::vec::IndexVec` | struct | 26 | [prose](../api/ruff_index.vec.md#indexvec) | [records](../model/ruff_index.vec.json) |
| `ruff_db::files::File` | struct | 38 | [prose](../api/ruff_db.files.md#file) | [records](../model/ruff_db.files.json) |

## Upstream guides

- [`corpus/ruff/versioning.md`](../corpus/ruff/versioning.md)

## Decision rules

- Want to parse and inspect Python from Rust? `ruff_python_parser` + `ruff_python_ast`.
- Want name resolution too? Add `ruff_python_semantic`.
- Want types? You are vendoring the checker; there is no published crate.

## Anti-patterns

- Pinning a library crate to the product version.
- Depending on `ruff_linter` to get at the AST -- it pulls in everything.

## Agent checklist

- `content/index/features.tsv` is the only source of feature gating; signatures never say.
