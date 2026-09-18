# `datafusion_ffi::ffi_option::FFI_Result`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.ffi_option.FFI_Result.json).

<a id="op-c532ce8cafc4f31813c8aeb9"></a>
## FFI_Result

`enum` · `datafusion_ffi::ffi_option::FFI_Result` · datafusion-ffi 55.1.0

```rust
enum FFI_Result<T>
```

Source: `src/ffi_option.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

An FFI-safe result type with SString as the error type.

<a id="op-224bb4452bc1b8a09f4fa20d"></a>
## Err

`variant` · `datafusion_ffi::ffi_option::FFI_Result::Err` · datafusion-ffi 55.1.0

```rust
Err
```

Source: `src/ffi_option.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-862b62797fdc080b64b40ba5"></a>
## Ok

`variant` · `datafusion_ffi::ffi_option::FFI_Result::Ok` · datafusion-ffi 55.1.0

```rust
Ok
```

Source: `src/ffi_option.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e323a005afc43e269abc7b5"></a>
## clone

`function` · `datafusion_ffi::ffi_option::FFI_Result::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> FFI_Result<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Result", "path": "FFI_Result"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 17], "end": [75, 22], "filename": "src/ffi_option.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ffi_option.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe9af4f249eb4dd038bfe0c3"></a>
## eq

`function` · `datafusion_ffi::ffi_option::FFI_Result::eq` · datafusion-ffi 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Result", "path": "FFI_Result"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [135, 2], "filename": "src/ffi_option.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ffi_option.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e4ae12d958213b9fe27324f"></a>
## fmt

`function` · `datafusion_ffi::ffi_option::FFI_Result::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Result", "path": "FFI_Result"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 10], "end": [75, 15], "filename": "src/ffi_option.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ffi_option.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55bd66591541c7e2022bc7d6"></a>
## from

`function` · `datafusion_ffi::ffi_option::FFI_Result::from` · datafusion-ffi 55.1.0

```rust
fn from(res: Result<T, E>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Result", "path": "FFI_Result"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::string::ToString", "path": "ToString"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [125, 2], "filename": "src/ffi_option.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"type": {"generic": "E"}}], "constraints": []}}, "id": "core::result::Result", "path": "Result"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ffi_option.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-112729c1a4a366aaf95cc335"></a>
## into_result

`function` · `datafusion_ffi::ffi_option::FFI_Result::into_result` · datafusion-ffi 55.1.0

```rust
fn into_result(self) -> Result<T, SString>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Result", "path": "FFI_Result"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [107, 2], "filename": "src/ffi_option.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi_option.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8dd560af5e3bb4f153545c4"></a>
## is_err

`function` · `datafusion_ffi::ffi_option::FFI_Result::is_err` · datafusion-ffi 55.1.0

```rust
fn is_err(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Result", "path": "FFI_Result"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [107, 2], "filename": "src/ffi_option.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi_option.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fc7b1bef4ce0d86eb17730f"></a>
## is_ok

`function` · `datafusion_ffi::ffi_option::FFI_Result::is_ok` · datafusion-ffi 55.1.0

```rust
fn is_ok(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Result", "path": "FFI_Result"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [107, 2], "filename": "src/ffi_option.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi_option.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a74d64dcd50b18f9b9c3cab8"></a>
## map

`function` · `datafusion_ffi::ffi_option::FFI_Result::map` · datafusion-ffi 55.1.0

```rust
fn map<U, F: FnOnce(T) -> U>(self, f: F) -> FFI_Result<U>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Result", "path": "FFI_Result"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [107, 2], "filename": "src/ffi_option.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi_option.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afff5e952366afbc5ca345b0"></a>
## unwrap_err

`function` · `datafusion_ffi::ffi_option::FFI_Result::unwrap_err` · datafusion-ffi 55.1.0

```rust
fn unwrap_err(self) -> SString
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_ffi::ffi_option::FFI_Result", "path": "FFI_Result"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [107, 2], "filename": "src/ffi_option.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi_option.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
