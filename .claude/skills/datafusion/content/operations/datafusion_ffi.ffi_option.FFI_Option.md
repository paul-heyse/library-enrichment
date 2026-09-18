# `datafusion_ffi::ffi_option::FFI_Option`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.ffi_option.FFI_Option.json).

<a id="op-d85b26d44cadf91dee69d7e6"></a>
## FFI_Option

`enum` · `datafusion_ffi::ffi_option::FFI_Option` · datafusion-ffi 55.1.0

```rust
enum FFI_Option<T>
```

Source: `src/ffi_option.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

An FFI-safe option type.

<a id="op-8d55b0c9806cf87bea739d2f"></a>
## None

`variant` · `datafusion_ffi::ffi_option::FFI_Option::None` · datafusion-ffi 55.1.0

```rust
None
```

Source: `src/ffi_option.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fdda97a699ce37cf8df5270"></a>
## Some

`variant` · `datafusion_ffi::ffi_option::FFI_Option::Some` · datafusion-ffi 55.1.0

```rust
Some
```

Source: `src/ffi_option.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04c3927982d06a5c9e429945"></a>
## as_ref

`function` · `datafusion_ffi::ffi_option::FFI_Option::as_ref` · datafusion-ffi 55.1.0

```rust
fn as_ref(&self) -> Option<&T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Option", "path": "FFI_Option"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [71, 2], "filename": "src/ffi_option.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi_option.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9454606954d5496408bd4fe8"></a>
## clone

`function` · `datafusion_ffi::ffi_option::FFI_Option::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> FFI_Option<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Option", "path": "FFI_Option"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 17], "end": [29, 22], "filename": "src/ffi_option.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ffi_option.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e415c642093ff3c5cb2374de"></a>
## fmt

`function` · `datafusion_ffi::ffi_option::FFI_Option::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Option", "path": "FFI_Option"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "src/ffi_option.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ffi_option.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9251e516ce16b19a19743e42"></a>
## from

`function` · `datafusion_ffi::ffi_option::FFI_Option::from` · datafusion-ffi 55.1.0

```rust
fn from(opt: Option<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Option", "path": "FFI_Option"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [42, 2], "filename": "src/ffi_option.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ffi_option.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41ab79d7f9df86b3e1305e88"></a>
## into_option

`function` · `datafusion_ffi::ffi_option::FFI_Option::into_option` · datafusion-ffi 55.1.0

```rust
fn into_option(self) -> Option<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Option", "path": "FFI_Option"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [71, 2], "filename": "src/ffi_option.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi_option.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8acedfd8477679563159ff9"></a>
## map

`function` · `datafusion_ffi::ffi_option::FFI_Option::map` · datafusion-ffi 55.1.0

```rust
fn map<U, F: FnOnce(T) -> U>(self, f: F) -> FFI_Option<U>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Option", "path": "FFI_Option"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [71, 2], "filename": "src/ffi_option.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi_option.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
