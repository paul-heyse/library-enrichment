# `buoyant_kernel_engine::file_stream::OnError`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.file_stream.OnError.json).

<a id="op-4dbec1fcbb5830b15555aaba"></a>
## OnError

`enum` · `buoyant_kernel_engine::file_stream::OnError` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum OnError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L32).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:32`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Describes the behavior of the `FileStream` if file opening or scanning fails

<a id="op-6f43b0396ead8920c2dcfcc3"></a>
## Fail

`variant` · `buoyant_kernel_engine::file_stream::OnError::Fail` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Fail
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L35).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:35`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Fail the entire stream and return the underlying error

<a id="op-2b061a3862b2f3385b30072f"></a>
## Skip

`variant` · `buoyant_kernel_engine::file_stream::OnError::Skip` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Skip
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L37).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:37`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Continue scanning, ignoring the failed file

<a id="op-66cf23eadd10afc4738c379b"></a>
## default

`function` · `buoyant_kernel_engine::file_stream::OnError::default` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> OnError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::file_stream::OnError", "path": "OnError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 17], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
