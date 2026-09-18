# `arrow_csv::reader::Reader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_csv.reader.Reader.json).

<a id="op-80f5b4a82073800a3487a573"></a>
## Reader

`type_alias` · `arrow_csv::reader::Reader` · arrow-csv 59.3.0

```rust
type Reader<R> = BufReader<std::io::BufReader<R>>
```

Source: `src/reader/mod.rs:498`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

CSV file reader using [`std::io::BufReader`]

See [`ReaderBuilder`](../operations/arrow_csv.reader.ReaderBuilder.md#op-743461f30ad5174b593cb0bd) to construct a CSV reader with options and  the
[module-level documentation](crate::reader) for more details and examples

Unresolved upstream links (retained, not inferred): ``std::io::BufReader``.
