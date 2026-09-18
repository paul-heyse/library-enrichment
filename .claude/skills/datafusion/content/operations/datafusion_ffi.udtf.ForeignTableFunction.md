# `datafusion_ffi::udtf::ForeignTableFunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udtf.ForeignTableFunction.json).

<a id="op-c7757232ac2a939114bee408"></a>
## ForeignTableFunction

`struct` · `datafusion_ffi::udtf::ForeignTableFunction` · datafusion-ffi 55.1.0

```rust
struct ForeignTableFunction
```

Source: `src/udtf.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This struct is used to access an UDTF provided by a foreign
library across a FFI boundary.

The ForeignTableFunction is to be used by the caller of the UDTF, so it has
no knowledge or access to the private data. All interaction with the UDTF
must occur through the functions defined in FFI_TableFunction.

<a id="op-965e735cd8a7ab4b727fa895"></a>
## call

`function` · `datafusion_ffi::udtf::ForeignTableFunction::call` · datafusion-ffi 55.1.0

```rust
fn call(&self, args: &[datafusion_expr::Expr]) -> Result<Arc<dyn TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udtf::ForeignTableFunction", "path": "ForeignTableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [312, 2], "filename": "src/udtf.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableFunctionImpl", "path": "TableFunctionImpl"}, "trait_path": "datafusion_session::table::TableFunctionImpl"}`

Source: `src/udtf.rs:297`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3eada76f7d4f068631cc5028"></a>
## call_with_args

`function` · `datafusion_ffi::udtf::ForeignTableFunction::call_with_args` · datafusion-ffi 55.1.0

```rust
fn call_with_args(&self, args: TableFunctionArgs<'_, '_>) -> Result<Arc<dyn TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udtf::ForeignTableFunction", "path": "ForeignTableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [312, 2], "filename": "src/udtf.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableFunctionImpl", "path": "TableFunctionImpl"}, "trait_path": "datafusion_session::table::TableFunctionImpl"}`

Source: `src/udtf.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3fd4e2967bc6be476adc7da"></a>
## fmt

`function` · `datafusion_ffi::udtf::ForeignTableFunction::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udtf::ForeignTableFunction", "path": "ForeignTableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 10], "end": [259, 15], "filename": "src/udtf.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udtf.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
