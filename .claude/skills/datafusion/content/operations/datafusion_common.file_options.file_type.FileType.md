# `datafusion_common::file_options::file_type::FileType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.file_options.file_type.FileType.json).

<a id="op-a3f73eb11aa44031af77da32"></a>
## FileType

`trait` · `datafusion_common::file_options::file_type::FileType` · datafusion-common 55.1.0

```rust
trait FileType: GetExt + Display + Send + Sync
```

Source: `src/file_options/file_type.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Defines the functionality needed for logical planning for
a type of file which will be read or written to storage.

<a id="op-2f9f784be599d5823ff134b4"></a>
## as_any

`function` · `datafusion_common::file_options::file_type::FileType::as_any` · datafusion-common 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Source: `src/file_options/file_type.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the table source as [`Any`] so that it can be
downcast to a specific implementation.

Unresolved upstream links (retained, not inferred): ``Any``.
