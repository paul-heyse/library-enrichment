# `datafusion_ffi::udwf::partition_evaluator::FFI_PartitionEvaluator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udwf.partition_evaluator.FFI_PartitionEvaluator.json).

<a id="op-231e3eb026f3df1ce003a95c"></a>
## FFI_PartitionEvaluator

`struct` · `datafusion_ffi::udwf::partition_evaluator::FFI_PartitionEvaluator` · datafusion-ffi 55.1.0

```rust
struct FFI_PartitionEvaluator
```

Source: `src/udwf/partition_evaluator.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`PartitionEvaluator`] across FFI boundaries.
For an explanation of each field, see the corresponding function
defined in [`PartitionEvaluator`].

<a id="op-8d3cddda22510e4f4187d8a4"></a>
## evaluate

`struct_field` · `datafusion_ffi::udwf::partition_evaluator::FFI_PartitionEvaluator::evaluate` · datafusion-ffi 55.1.0

```rust
evaluate: unsafe fn(&mut Self, stabby::vec::Vec<arrow_wrappers::WrappedArray>, super::range::FFI_Range) -> util::FFI_Result<stabby::vec::Vec<u8>>
```

Source: `src/udwf/partition_evaluator.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97a338e621e85a08330b5889"></a>
## evaluate_all

`struct_field` · `datafusion_ffi::udwf::partition_evaluator::FFI_PartitionEvaluator::evaluate_all` · datafusion-ffi 55.1.0

```rust
evaluate_all: unsafe fn(&mut Self, stabby::vec::Vec<arrow_wrappers::WrappedArray>, usize) -> util::FFI_Result<arrow_wrappers::WrappedArray>
```

Source: `src/udwf/partition_evaluator.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-662c626e9cb893c87616daa7"></a>
## evaluate_all_with_rank

`struct_field` · `datafusion_ffi::udwf::partition_evaluator::FFI_PartitionEvaluator::evaluate_all_with_rank` · datafusion-ffi 55.1.0

```rust
evaluate_all_with_rank: unsafe fn(&Self, usize, stabby::vec::Vec<super::range::FFI_Range>) -> util::FFI_Result<arrow_wrappers::WrappedArray>
```

Source: `src/udwf/partition_evaluator.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e76b535c592884188235fa56"></a>
## get_range

`struct_field` · `datafusion_ffi::udwf::partition_evaluator::FFI_PartitionEvaluator::get_range` · datafusion-ffi 55.1.0

```rust
get_range: unsafe fn(&Self, usize, usize) -> util::FFI_Result<super::range::FFI_Range>
```

Source: `src/udwf/partition_evaluator.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a40b6f65fa48e292446b1ca8"></a>
## include_rank

`struct_field` · `datafusion_ffi::udwf::partition_evaluator::FFI_PartitionEvaluator::include_rank` · datafusion-ffi 55.1.0

```rust
include_rank: bool
```

Source: `src/udwf/partition_evaluator.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1849db477d84c08c0ee1b4ec"></a>
## is_causal

`struct_field` · `datafusion_ffi::udwf::partition_evaluator::FFI_PartitionEvaluator::is_causal` · datafusion-ffi 55.1.0

```rust
is_causal: bool
```

Source: `src/udwf/partition_evaluator.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58164bc98b73fd7f62700119"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::udwf::partition_evaluator::FFI_PartitionEvaluator::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/udwf/partition_evaluator.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`] and
the crate's `README.md` for more information.

<a id="op-3951421bc5161e68834de5f8"></a>
## private_data

`struct_field` · `datafusion_ffi::udwf::partition_evaluator::FFI_PartitionEvaluator::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/udwf/partition_evaluator.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the evaluator.
A [`ForeignPartitionEvaluator`] should never attempt to access this data.

<a id="op-a4a39256ab3b9b46971efe79"></a>
## release

`struct_field` · `datafusion_ffi::udwf::partition_evaluator::FFI_PartitionEvaluator::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/udwf/partition_evaluator.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-9db2658a31f4dd4f6250883b"></a>
## supports_bounded_execution

`struct_field` · `datafusion_ffi::udwf::partition_evaluator::FFI_PartitionEvaluator::supports_bounded_execution` · datafusion-ffi 55.1.0

```rust
supports_bounded_execution: bool
```

Source: `src/udwf/partition_evaluator.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f05d0d8aecd974e5cb15ba1b"></a>
## uses_window_frame

`struct_field` · `datafusion_ffi::udwf::partition_evaluator::FFI_PartitionEvaluator::uses_window_frame` · datafusion-ffi 55.1.0

```rust
uses_window_frame: bool
```

Source: `src/udwf/partition_evaluator.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
