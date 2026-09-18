# `datafusion_ffi::udaf::accumulator::FFI_Accumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udaf.accumulator.FFI_Accumulator.json).

<a id="op-1cc2a981f786cf89e70bdd28"></a>
## FFI_Accumulator

`struct` · `datafusion_ffi::udaf::accumulator::FFI_Accumulator` · datafusion-ffi 55.1.0

```rust
struct FFI_Accumulator
```

Source: `src/udaf/accumulator.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`Accumulator`] across FFI boundaries.
For an explanation of each field, see the corresponding function
defined in [`Accumulator`].

<a id="op-e6886001e202f3bf97cedf71"></a>
## evaluate

`struct_field` · `datafusion_ffi::udaf::accumulator::FFI_Accumulator::evaluate` · datafusion-ffi 55.1.0

```rust
evaluate: unsafe fn(&mut Self) -> util::FFI_Result<stabby::vec::Vec<u8>>
```

Source: `src/udaf/accumulator.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b2b2d04cd854fc5376af25e"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::udaf::accumulator::FFI_Accumulator::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/udaf/accumulator.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`] and
the crate's `README.md` for more information.

<a id="op-2ae9a7d11a6af0bd6e297b95"></a>
## merge_batch

`struct_field` · `datafusion_ffi::udaf::accumulator::FFI_Accumulator::merge_batch` · datafusion-ffi 55.1.0

```rust
merge_batch: unsafe fn(&mut Self, stabby::vec::Vec<arrow_wrappers::WrappedArray>) -> util::FFI_Result<()>
```

Source: `src/udaf/accumulator.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c33ac2efeb46f557aff7eebf"></a>
## private_data

`struct_field` · `datafusion_ffi::udaf::accumulator::FFI_Accumulator::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/udaf/accumulator.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the accumulator.
A [`ForeignAccumulator`] should never attempt to access this data.

<a id="op-4754104c197efe00cf03bd4b"></a>
## release

`struct_field` · `datafusion_ffi::udaf::accumulator::FFI_Accumulator::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/udaf/accumulator.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-40ce37611112de8facf1862e"></a>
## retract_batch

`struct_field` · `datafusion_ffi::udaf::accumulator::FFI_Accumulator::retract_batch` · datafusion-ffi 55.1.0

```rust
retract_batch: unsafe fn(&mut Self, stabby::vec::Vec<arrow_wrappers::WrappedArray>) -> util::FFI_Result<()>
```

Source: `src/udaf/accumulator.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33a8e56ff81c1c3d97a9b3a3"></a>
## size

`struct_field` · `datafusion_ffi::udaf::accumulator::FFI_Accumulator::size` · datafusion-ffi 55.1.0

```rust
size: unsafe fn(&Self) -> usize
```

Source: `src/udaf/accumulator.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3febc7a8759753610cff7f24"></a>
## state

`struct_field` · `datafusion_ffi::udaf::accumulator::FFI_Accumulator::state` · datafusion-ffi 55.1.0

```rust
state: unsafe fn(&mut Self) -> util::FFI_Result<stabby::vec::Vec<stabby::vec::Vec<u8>>>
```

Source: `src/udaf/accumulator.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3431524f8dc0cda47744f24e"></a>
## supports_retract_batch

`struct_field` · `datafusion_ffi::udaf::accumulator::FFI_Accumulator::supports_retract_batch` · datafusion-ffi 55.1.0

```rust
supports_retract_batch: bool
```

Source: `src/udaf/accumulator.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c11f96ab8732d008918b8dc"></a>
## update_batch

`struct_field` · `datafusion_ffi::udaf::accumulator::FFI_Accumulator::update_batch` · datafusion-ffi 55.1.0

```rust
update_batch: unsafe fn(&mut Self, stabby::vec::Vec<arrow_wrappers::WrappedArray>) -> util::FFI_Result<()>
```

Source: `src/udaf/accumulator.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
