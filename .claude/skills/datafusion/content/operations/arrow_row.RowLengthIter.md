# `arrow_row::RowLengthIter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_row.RowLengthIter.json).

<a id="op-0408bf3cc4adf46fe16993b3"></a>
## RowLengthIter

`type_alias` · `arrow_row::RowLengthIter` · arrow-row 59.3.0

```rust
type RowLengthIter<'a> = std::iter::Map<std::slice::Windows<'a, usize>, fn(&'a [usize]) -> usize>
```

Source: `src/lib.rs:1341`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

The iterator type for [`Rows::lengths`](../operations/arrow_row.Rows.md#op-95641c05b7cecabe89a314ec)
