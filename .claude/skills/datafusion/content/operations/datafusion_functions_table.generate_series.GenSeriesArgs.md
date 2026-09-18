# `datafusion_functions_table::generate_series::GenSeriesArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_table.generate_series.GenSeriesArgs.json).

<a id="op-2e03beaacd6fac1157f5890f"></a>
## GenSeriesArgs

`enum` · `datafusion_functions_table::generate_series::GenSeriesArgs` · datafusion-functions-table 55.1.0

```rust
enum GenSeriesArgs
```

Source: `src/generate_series.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

Indicates the arguments used for generating a series.

<a id="op-fa9f748825f2ccedb074fc1e"></a>
## ContainsNull

`variant` · `datafusion_functions_table::generate_series::GenSeriesArgs::ContainsNull` · datafusion-functions-table 55.1.0

```rust
ContainsNull
```

Source: `src/generate_series.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

ContainsNull signifies that at least one argument(start, end, step) was null, thus no series will be generated.

<a id="op-5906668df798db63e2fb6a91"></a>
## DateArgs

`variant` · `datafusion_functions_table::generate_series::GenSeriesArgs::DateArgs` · datafusion-functions-table 55.1.0

```rust
DateArgs
```

Source: `src/generate_series.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

DateArgs holds the start, end, and step values for generating date series when all arguments are not null.
Internally, dates are converted to timestamps and use the timestamp logic.

<a id="op-3f179a42a5b8c8bc8a94adfa"></a>
## Int64Args

`variant` · `datafusion_functions_table::generate_series::GenSeriesArgs::Int64Args` · datafusion-functions-table 55.1.0

```rust
Int64Args
```

Source: `src/generate_series.rs:248`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

Int64Args holds the start, end, and step values for generating integer series when all arguments are not null.

<a id="op-7c45a73288e8cbc7ac991e24"></a>
## TimestampArgs

`variant` · `datafusion_functions_table::generate_series::GenSeriesArgs::TimestampArgs` · datafusion-functions-table 55.1.0

```rust
TimestampArgs
```

Source: `src/generate_series.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

TimestampArgs holds the start, end, and step values for generating timestamp series when all arguments are not null.

<a id="op-f46a736bb3ad25f397f8a63b"></a>
## clone

`function` · `datafusion_functions_table::generate_series::GenSeriesArgs::clone` · datafusion-functions-table 55.1.0

```rust
fn clone(&self) -> GenSeriesArgs
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::GenSeriesArgs", "path": "GenSeriesArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 17], "end": [243, 22], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generate_series.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fa3328831c91ef76f8250cd"></a>
## fmt

`function` · `datafusion_functions_table::generate_series::GenSeriesArgs::fmt` · datafusion-functions-table 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::GenSeriesArgs", "path": "GenSeriesArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 10], "end": [243, 15], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generate_series.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
