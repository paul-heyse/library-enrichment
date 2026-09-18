# `datafusion_expr::extension_types::array_formatter_factory::DFArrayFormatterFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.extension_types.array_formatter_factory.DFArrayFormatterFactory.json).

<a id="op-c3223bfa39f3e53cc1645df7"></a>
## DFArrayFormatterFactory

`struct` · `datafusion_expr::extension_types::array_formatter_factory::DFArrayFormatterFactory` · datafusion-expr 55.1.0

```rust
struct DFArrayFormatterFactory
```

Source: `src/extension_types/array_formatter_factory.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A factory for creating [`ArrayFormatter`](../operations/arrow_cast.display.ArrayFormatter.md#op-c2a994be3b7cca3db9a55992)s that checks whether a registered extension type can
format a given array based on its metadata.

<a id="op-1901b5817fd628a9ce9531e0"></a>
## create_array_formatter

`function` · `datafusion_expr::extension_types::array_formatter_factory::DFArrayFormatterFactory::create_array_formatter` · datafusion-expr 55.1.0

```rust
fn create_array_formatter<'formatter>(&self, array: &'formatter dyn Array, options: &FormatOptions<'formatter>, field: Option<&'formatter Field>) -> Result<Option<ArrayFormatter<'formatter>>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::extension_types::array_formatter_factory::DFArrayFormatterFactory", "path": "DFArrayFormatterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [67, 2], "filename": "src/extension_types/array_formatter_factory.rs"}, "trait": {"args": null, "id": "arrow_cast::display::ArrayFormatterFactory", "path": "ArrayFormatterFactory"}, "trait_path": "arrow_cast::display::ArrayFormatterFactory"}`

Source: `src/extension_types/array_formatter_factory.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-604ae90ea5f38c65d46e059e"></a>
## fmt

`function` · `datafusion_expr::extension_types::array_formatter_factory::DFArrayFormatterFactory::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::extension_types::array_formatter_factory::DFArrayFormatterFactory", "path": "DFArrayFormatterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 10], "end": [25, 15], "filename": "src/extension_types/array_formatter_factory.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extension_types/array_formatter_factory.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7a0de60a24d8875ef0cdec1"></a>
## new

`function` · `datafusion_expr::extension_types::array_formatter_factory::DFArrayFormatterFactory::new` · datafusion-expr 55.1.0

```rust
fn new(registry: ExtensionTypeRegistryRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::extension_types::array_formatter_factory::DFArrayFormatterFactory", "path": "DFArrayFormatterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [36, 2], "filename": "src/extension_types/array_formatter_factory.rs"}, "trait": null, "trait_path": null}`

Source: `src/extension_types/array_formatter_factory.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new [`DFArrayFormatterFactory`](../operations/datafusion_expr.extension_types.array_formatter_factory.DFArrayFormatterFactory.md#op-c3223bfa39f3e53cc1645df7).
