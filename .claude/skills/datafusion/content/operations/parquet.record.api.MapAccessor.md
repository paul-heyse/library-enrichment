# `parquet::record::api::MapAccessor`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.api.MapAccessor.json).

<a id="op-155560b1f0839cc22db2d4b7"></a>
## MapAccessor

`trait` · `parquet::record::api::MapAccessor` · parquet 59.3.0

```rust
trait MapAccessor
```

Source: `src/record/api.rs:497`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Trait for type-safe access of an index for a `Map`

<a id="op-df6d66213ce3a171a444707f"></a>
## get_keys

`function` · `parquet::record::api::MapAccessor::get_keys` · parquet 59.3.0

```rust
fn get_keys<'a>(&'a self) -> Box<dyn ListAccessor + 'a>
```

Source: `src/record/api.rs:499`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get the keys of the map.

<a id="op-14bc1307eeb3f3ab3fadff33"></a>
## get_values

`function` · `parquet::record::api::MapAccessor::get_values` · parquet 59.3.0

```rust
fn get_values<'a>(&'a self) -> Box<dyn ListAccessor + 'a>
```

Source: `src/record/api.rs:501`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get the values of the map.
