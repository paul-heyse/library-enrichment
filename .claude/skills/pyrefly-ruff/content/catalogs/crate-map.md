# Crate map

One release can carry more than one version line. ruff publishes `ruff`, `ruff_linter`
and `ruff_wasm` on the product line and every library crate on a `0.0.x` line; they are
the same release, published together, and `ruff_linter 0.16.7` depends on the library
crates at `^0.0.13`. Writing `ruff_python_ast = "0.16.7"` into a Cargo.toml selects a
version that does not exist.

| Crate | Version | Items | Traits |
|---|---|---:|---:|
| `pyrefly` | 1.3.1 | 589 | 7 |
| `pyrefly_build` | 1.3.1 | 22 | 4 |
| `pyrefly_bundled` | 1.3.1 | 6 | 0 |
| `pyrefly_config` | 1.3.1 | 45 | 0 |
| `pyrefly_glean_schema` | 1.3.1 | 191 | 2 |
| `pyrefly_graph` | 1.3.1 | 5 | 0 |
| `pyrefly_python` | 1.3.1 | 97 | 0 |
| `pyrefly_types` | 1.3.1 | 209 | 3 |
| `pyrefly_util` | 1.3.1 | 118 | 10 |
| `ruff` | 0.16.7 | 25 | 0 |
| `ruff_annotate_snippets` | 0.0.13 | 33 | 0 |
| `ruff_cache` | 0.0.13 | 4 | 1 |
| `ruff_db` | 0.0.13 | 104 | 14 |
| `ruff_diagnostics` | 0.0.13 | 6 | 0 |
| `ruff_formatter` | 0.0.13 | 133 | 11 |
| `ruff_graph` | 0.0.13 | 7 | 0 |
| `ruff_index` | 0.0.13 | 4 | 1 |
| `ruff_linter` | 0.16.7 | 294 | 4 |
| `ruff_macros` | 0.0.13 | 5 | 0 |
| `ruff_markdown` | 0.0.13 | 2 | 0 |
| `ruff_memory_usage` | 0.0.13 | 3 | 0 |
| `ruff_notebook` | 0.0.13 | 18 | 0 |
| `ruff_options_metadata` | 0.0.13 | 6 | 2 |
| `ruff_python_ast` | 0.0.13 | 468 | 7 |
| `ruff_python_codegen` | 0.0.13 | 5 | 0 |
| `ruff_python_edits` | 0.0.13 | 1 | 0 |
| `ruff_python_formatter` | 0.0.13 | 134 | 4 |
| `ruff_python_importer` | 0.0.13 | 1 | 0 |
| `ruff_python_index` | 0.0.13 | 3 | 0 |
| `ruff_python_literal` | 0.0.13 | 42 | 2 |
| `ruff_python_parser` | 0.0.13 | 41 | 2 |
| `ruff_python_semantic` | 0.0.13 | 149 | 4 |
| `ruff_python_stdlib` | 0.0.13 | 34 | 0 |
| `ruff_python_trivia` | 0.0.13 | 32 | 1 |
| `ruff_ranged_value` | 0.0.13 | 3 | 0 |
| `ruff_server` | 0.0.13 | 7 | 0 |
| `ruff_source_file` | 0.0.13 | 16 | 2 |
| `ruff_text_size` | 0.0.13 | 6 | 4 |
| `ruff_wasm` | 0.16.7 | 14 | 0 |
| `ruff_workspace` | 0.0.13 | 67 | 1 |
| `tsp_types` | 1.3.1 | 80 | 0 |
