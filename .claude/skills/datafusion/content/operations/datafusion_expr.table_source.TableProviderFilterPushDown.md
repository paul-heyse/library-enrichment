# `datafusion_expr::table_source::TableProviderFilterPushDown`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.table_source.TableProviderFilterPushDown.json).

<a id="op-4fd1b17084602ce2b715fb21"></a>
## TableProviderFilterPushDown

`enum` · `datafusion_expr::table_source::TableProviderFilterPushDown` · datafusion-expr 55.1.0

```rust
enum TableProviderFilterPushDown
```

Source: `src/table_source.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Indicates how a filter expression is handled by
[`TableProvider::scan`].

Filter expressions are boolean expressions used to reduce the number of
rows that are read from a table. Only rows that evaluate to `true` ("pass
the filter") are returned. Rows that evaluate to `false` or `NULL` are
omitted.

[`TableProvider::scan`]: https://docs.rs/datafusion/latest/datafusion/datasource/trait.TableProvider.html#tymethod.scan

<a id="op-76dd051fb92b19d07ce613a6"></a>
## Exact

`variant` · `datafusion_expr::table_source::TableProviderFilterPushDown::Exact` · datafusion-expr 55.1.0

```rust
Exact
```

Source: `src/table_source.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

**Reference annotation (source_runtime_reconciliation, separate from upstream):** The upstream variant sentence about omitting tuples which pass the predicate conflicts with TableProvider::scan and observed removal of the residual filter. Exact must fully enforce the predicate and retain qualifying rows. Treat that sentence as an upstream documentation defect; the original text is preserved below. [Evidence](../capabilities/df.pushdown.md).

The provider **guarantees** that it will omit **only** tuples which
pass the filter.

In this case, DataFusion will not apply additional filtering.

<a id="op-5317f423dd41036b1a8d1522"></a>
## Inexact

`variant` · `datafusion_expr::table_source::TableProviderFilterPushDown::Inexact` · datafusion-expr 55.1.0

```rust
Inexact
```

Source: `src/table_source.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The filter can be used, but the provider might still return some tuples
that do not pass the filter.

In this case, DataFusion applies an additional `Filter` operation
after the scan to ensure all rows are filtered correctly.

<a id="op-9639f255964a0e41a46e32d1"></a>
## Unsupported

`variant` · `datafusion_expr::table_source::TableProviderFilterPushDown::Unsupported` · datafusion-expr 55.1.0

```rust
Unsupported
```

Source: `src/table_source.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The filter cannot be used by the provider and will not be pushed down.

<a id="op-09344aae33195c789fa0cf8f"></a>
## clone

`function` · `datafusion_expr::table_source::TableProviderFilterPushDown::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> TableProviderFilterPushDown
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::table_source::TableProviderFilterPushDown", "path": "TableProviderFilterPushDown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 17], "end": [36, 22], "filename": "src/table_source.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/table_source.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ef4a41dbb7032f8afbd0408"></a>
## eq

`function` · `datafusion_expr::table_source::TableProviderFilterPushDown::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &TableProviderFilterPushDown) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::table_source::TableProviderFilterPushDown", "path": "TableProviderFilterPushDown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 24], "end": [36, 33], "filename": "src/table_source.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/table_source.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5aaff58a415e62d28b497bf9"></a>
## fmt

`function` · `datafusion_expr::table_source::TableProviderFilterPushDown::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::table_source::TableProviderFilterPushDown", "path": "TableProviderFilterPushDown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/table_source.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table_source.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
