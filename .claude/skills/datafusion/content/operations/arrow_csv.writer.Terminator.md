# `arrow_csv::writer::Terminator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_csv.writer.Terminator.json).

<a id="op-e79fd847891f521ff7913a44"></a>
## Terminator

`enum` · `arrow_csv::writer::Terminator` · arrow-csv 59.3.0

```rust
enum Terminator
```

Source: `src/writer.rs:387`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

The line terminator to use when writing CSV files.

<a id="op-48268892266c371607db7e82"></a>
## Any

`variant` · `arrow_csv::writer::Terminator::Any` · arrow-csv 59.3.0

```rust
Any
```

Source: `src/writer.rs:391`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Use the specified byte character as the line terminator

<a id="op-29fcdee60cb09b029205aa67"></a>
## CRLF

`variant` · `arrow_csv::writer::Terminator::CRLF` · arrow-csv 59.3.0

```rust
CRLF
```

Source: `src/writer.rs:389`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Use CRLF (`\r\n`) as the line terminator

<a id="op-7555ea4e031d21bae2ebe903"></a>
## clone

`function` · `arrow_csv::writer::Terminator::clone` · arrow-csv 59.3.0

```rust
fn clone(&self) -> Terminator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::Terminator", "path": "Terminator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 10], "end": [386, 15], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/writer.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a092a7b47503a119cf3d0d34"></a>
## fmt

`function` · `arrow_csv::writer::Terminator::fmt` · arrow-csv 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::Terminator", "path": "Terminator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 17], "end": [386, 22], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
