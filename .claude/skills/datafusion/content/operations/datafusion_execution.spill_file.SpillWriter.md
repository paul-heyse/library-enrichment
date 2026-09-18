# `datafusion_execution::spill_file::SpillWriter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.spill_file.SpillWriter.json).

<a id="op-d6421be08333db67399a8e30"></a>
## SpillWriter

`trait` · `datafusion_execution::spill_file::SpillWriter` · datafusion-execution 55.1.0

```rust
trait SpillWriter: std::io::Write + Send
```

Source: `src/spill_file.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Writer for spill file backends.

<a id="op-753981049f3c4e93a27c8c78"></a>
## finish

`function` · `datafusion_execution::spill_file::SpillWriter::finish` · datafusion-execution 55.1.0

```rust
fn finish(&mut self) -> Result<()>
```

Source: `src/spill_file.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Intended for close/sync/commit operations.
