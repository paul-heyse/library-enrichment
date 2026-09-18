# `datafusion_execution::spill_file::TempFileFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.spill_file.TempFileFactory.json).

<a id="op-dd44208d8318480df91986d6"></a>
## TempFileFactory

`trait` · `datafusion_execution::spill_file::TempFileFactory` · datafusion-execution 55.1.0

```rust
trait TempFileFactory: Send + Sync
```

Source: `src/spill_file.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Factory for creating spill files.

<a id="op-061b69871a0b08e7b9630d47"></a>
## create_temp_file

`function` · `datafusion_execution::spill_file::TempFileFactory::create_temp_file` · datafusion-execution 55.1.0

```rust
fn create_temp_file(&self, description: &str) -> Result<Arc<dyn SpillFile>>
```

Source: `src/spill_file.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
