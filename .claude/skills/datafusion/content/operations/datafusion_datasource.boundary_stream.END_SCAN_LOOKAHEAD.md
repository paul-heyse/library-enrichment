# `datafusion_datasource::boundary_stream::END_SCAN_LOOKAHEAD`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.boundary_stream.END_SCAN_LOOKAHEAD.json).

<a id="op-b9f5c0a30085fb7a7ca2111e"></a>
## END_SCAN_LOOKAHEAD

`constant` · `datafusion_datasource::boundary_stream::END_SCAN_LOOKAHEAD` · datafusion-datasource 55.1.0

```rust
const END_SCAN_LOOKAHEAD: u64 = _
```

Source: `src/boundary_stream.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

How far past `raw_end` the initial bounded fetch covers. If the terminating
newline is not found within this window, `ScanningLastTerminator` issues
successive same-sized GETs until the newline is located or EOF is reached.
