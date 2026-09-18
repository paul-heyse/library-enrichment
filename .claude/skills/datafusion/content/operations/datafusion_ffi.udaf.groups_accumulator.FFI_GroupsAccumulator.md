# `datafusion_ffi::udaf::groups_accumulator::FFI_GroupsAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udaf.groups_accumulator.FFI_GroupsAccumulator.json).

<a id="op-ac074053e9b2c2779e525cec"></a>
## FFI_GroupsAccumulator

`struct` · `datafusion_ffi::udaf::groups_accumulator::FFI_GroupsAccumulator` · datafusion-ffi 55.1.0

```rust
struct FFI_GroupsAccumulator
```

Source: `src/udaf/groups_accumulator.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`GroupsAccumulator`] across FFI boundaries.
For an explanation of each field, see the corresponding function
defined in [`GroupsAccumulator`].

<a id="op-ae09084a92c73916d112c16e"></a>
## convert_to_state

`struct_field` · `datafusion_ffi::udaf::groups_accumulator::FFI_GroupsAccumulator::convert_to_state` · datafusion-ffi 55.1.0

```rust
convert_to_state: unsafe fn(&Self, stabby::vec::Vec<arrow_wrappers::WrappedArray>, util::FFI_Option<arrow_wrappers::WrappedArray>) -> util::FFI_Result<stabby::vec::Vec<arrow_wrappers::WrappedArray>>
```

Source: `src/udaf/groups_accumulator.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a1bb079faa905d643a6088f"></a>
## evaluate

`struct_field` · `datafusion_ffi::udaf::groups_accumulator::FFI_GroupsAccumulator::evaluate` · datafusion-ffi 55.1.0

```rust
evaluate: unsafe fn(&mut Self, FFI_EmitTo) -> util::FFI_Result<arrow_wrappers::WrappedArray>
```

Source: `src/udaf/groups_accumulator.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13993a5e923843625893002e"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::udaf::groups_accumulator::FFI_GroupsAccumulator::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/udaf/groups_accumulator.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`] and
the crate's `README.md` for more information.

<a id="op-00e7e0ac3cd72e79f7357ba5"></a>
## merge_batch

`struct_field` · `datafusion_ffi::udaf::groups_accumulator::FFI_GroupsAccumulator::merge_batch` · datafusion-ffi 55.1.0

```rust
merge_batch: unsafe fn(&mut Self, stabby::vec::Vec<arrow_wrappers::WrappedArray>, stabby::vec::Vec<usize>, usize) -> util::FFI_Result<()>
```

Source: `src/udaf/groups_accumulator.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f62207da1721a819554ba76"></a>
## private_data

`struct_field` · `datafusion_ffi::udaf::groups_accumulator::FFI_GroupsAccumulator::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/udaf/groups_accumulator.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the accumulator.
A [`ForeignGroupsAccumulator`] should never attempt to access this data.

<a id="op-03379177076b8cef5c8e8e62"></a>
## release

`struct_field` · `datafusion_ffi::udaf::groups_accumulator::FFI_GroupsAccumulator::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/udaf/groups_accumulator.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-43283b504c8ccfec8573d648"></a>
## size

`struct_field` · `datafusion_ffi::udaf::groups_accumulator::FFI_GroupsAccumulator::size` · datafusion-ffi 55.1.0

```rust
size: unsafe fn(&Self) -> usize
```

Source: `src/udaf/groups_accumulator.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c73e027911374ef3bf9f5679"></a>
## state

`struct_field` · `datafusion_ffi::udaf::groups_accumulator::FFI_GroupsAccumulator::state` · datafusion-ffi 55.1.0

```rust
state: unsafe fn(&mut Self, FFI_EmitTo) -> util::FFI_Result<stabby::vec::Vec<arrow_wrappers::WrappedArray>>
```

Source: `src/udaf/groups_accumulator.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f78bec290062d324989b802"></a>
## update_batch

`struct_field` · `datafusion_ffi::udaf::groups_accumulator::FFI_GroupsAccumulator::update_batch` · datafusion-ffi 55.1.0

```rust
update_batch: unsafe fn(&mut Self, stabby::vec::Vec<arrow_wrappers::WrappedArray>, stabby::vec::Vec<usize>, util::FFI_Option<arrow_wrappers::WrappedArray>, usize) -> util::FFI_Result<()>
```

Source: `src/udaf/groups_accumulator.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
