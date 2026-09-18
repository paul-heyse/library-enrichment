# `arrow::error::Result`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.error.Result.json).

<a id="op-42f3348a5b24abb9052e34da"></a>
## Result

`type_alias` · `arrow::error::Result` · arrow 59.3.0

```rust
type Result<T> = std::result::Result<T, ArrowError>
```

Source: `src/error.rs:23`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

A specialized `Result` type for Arrow operations.
