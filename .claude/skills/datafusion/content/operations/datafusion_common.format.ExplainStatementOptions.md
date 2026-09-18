# `datafusion_common::format::ExplainStatementOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.format.ExplainStatementOptions.json).

<a id="op-aa41302c6d233871885d8cce"></a>
## ExplainStatementOptions

`struct` · `datafusion_common::format::ExplainStatementOptions` · datafusion-common 55.1.0

```rust
struct ExplainStatementOptions
```

Source: `src/format.rs:449`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Normalized options for a single `EXPLAIN` statement.

This collects the knobs that can be set per-statement from either the
legacy keyword form (`EXPLAIN ANALYZE VERBOSE FORMAT tree ...`) or the
Postgres-style `EXPLAIN (option [arg], ...) ...` form supported on
dialects whose
[`Dialect::supports_explain_with_utility_options`](https://docs.rs/sqlparser/latest/sqlparser/dialect/trait.Dialect.html#method.supports_explain_with_utility_options)
returns `true`.

Fields that are `None` / `false` mean "not set at the statement level" —
the physical planner falls back to the corresponding session config
value.

<a id="op-88ecf83e53791f46e12cedf7"></a>
## analyze

`struct_field` · `datafusion_common::format::ExplainStatementOptions::analyze` · datafusion-common 55.1.0

```rust
analyze: bool
```

Source: `src/format.rs:453`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Whether to actually execute the plan and gather metrics.

Corresponds to the `ANALYZE` keyword or the `ANALYZE` option.

<a id="op-078857c6f9b353ec1d077dec"></a>
## analyze_categories

`struct_field` · `datafusion_common::format::ExplainStatementOptions::analyze_categories` · datafusion-common 55.1.0

```rust
analyze_categories: Option<ExplainAnalyzeCategories>
```

Source: `src/format.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Override for [`ExplainAnalyzeCategories`](../operations/datafusion_common.format.ExplainAnalyzeCategories.md#op-4332623cb5931495db7b9adf) (rows / bytes / timing
/ uncategorized) when running `EXPLAIN ANALYZE`.

<a id="op-21608ce19073c5f6c89e0515"></a>
## analyze_level

`struct_field` · `datafusion_common::format::ExplainStatementOptions::analyze_level` · datafusion-common 55.1.0

```rust
analyze_level: Option<MetricType>
```

Source: `src/format.rs:463`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Override for [`MetricType`](../operations/datafusion_common.format.MetricType.md#op-fb6314931b98a471bb076826) (summary / dev) when running
`EXPLAIN ANALYZE`.

<a id="op-1d54e4b8ade4aab5276da2c2"></a>
## clone

`function` · `datafusion_common::format::ExplainStatementOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ExplainStatementOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainStatementOptions", "path": "ExplainStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 17], "end": [448, 22], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/format.rs:448`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2794feaa1b41a33a19e22d8"></a>
## default

`function` · `datafusion_common::format::ExplainStatementOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> ExplainStatementOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainStatementOptions", "path": "ExplainStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 24], "end": [448, 31], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/format.rs:448`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c63ab72a31cca2c70110bf2"></a>
## eq

`function` · `datafusion_common::format::ExplainStatementOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ExplainStatementOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainStatementOptions", "path": "ExplainStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 33], "end": [448, 42], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/format.rs:448`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21537867c35c8219a82a783e"></a>
## fmt

`function` · `datafusion_common::format::ExplainStatementOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainStatementOptions", "path": "ExplainStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 10], "end": [448, 15], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/format.rs:448`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88ca7cfd9aa43150d74383d1"></a>
## format

`struct_field` · `datafusion_common::format::ExplainStatementOptions::format` · datafusion-common 55.1.0

```rust
format: Option<ExplainFormat>
```

Source: `src/format.rs:460`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Output format for the plan. When `None`, the session-config
default (`datafusion.explain.format`) is used.

<a id="op-c76cf5c458ec4e8b7abe7ef2"></a>
## from_utility_options

`function` · `datafusion_common::format::ExplainStatementOptions::from_utility_options` · datafusion-common 55.1.0

```rust
fn from_utility_options(opts: &[UtilityOption]) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainStatementOptions", "path": "ExplainStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [472, 1], "end": [557, 2], "filename": "src/format.rs"}, "trait": null, "trait_path": null}`

Source: `src/format.rs:489`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Parse a list of [`UtilityOption`](../operations/sqlparser.ast.UtilityOption.md#op-7bfbbf6b89706e1b0e9e7b9b) values (produced by sqlparser's
`parse_utility_options`) into a normalized [`ExplainStatementOptions`](../operations/datafusion_common.format.ExplainStatementOptions.md#op-aa41302c6d233871885d8cce).

Argument grammar accepted:
- `OPTION` — bare, implies `TRUE` for boolean options.
- `OPTION TRUE` / `OPTION FALSE`
- `OPTION ON` / `OPTION OFF`
- `OPTION 1` / `OPTION 0`
- `OPTION <ident>` or `OPTION '<string>'` for format / level / metrics.

Options recognized by DataFusion are: `ANALYZE`, `VERBOSE`, `FORMAT`,
`METRICS`, `LEVEL`, `TIMING`, `SUMMARY`, `COSTS`.

Postgres-only options (`BUFFERS`, `WAL`, `SETTINGS`, `GENERIC_PLAN`,
`MEMORY`) return a helpful "not supported" error. Any other option
name produces an `unknown EXPLAIN option` error.

<a id="op-71cfc0884d746f8fb4436669"></a>
## hash

`function` · `datafusion_common::format::ExplainStatementOptions::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainStatementOptions", "path": "ExplainStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 48], "end": [448, 52], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/format.rs:448`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0df2099198c66fd965b0bbe1"></a>
## show_statistics

`struct_field` · `datafusion_common::format::ExplainStatementOptions::show_statistics` · datafusion-common 55.1.0

```rust
show_statistics: Option<bool>
```

Source: `src/format.rs:468`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Override for `datafusion.explain.show_statistics`.

<a id="op-ae3e22ec40096626c27ce63a"></a>
## verbose

`struct_field` · `datafusion_common::format::ExplainStatementOptions::verbose` · datafusion-common 55.1.0

```rust
verbose: bool
```

Source: `src/format.rs:457`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Whether to include extra detail in the output.

Corresponds to the `VERBOSE` keyword or the `VERBOSE` option.
