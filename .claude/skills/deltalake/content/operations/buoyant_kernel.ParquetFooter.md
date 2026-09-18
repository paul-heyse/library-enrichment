# `buoyant_kernel::ParquetFooter`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.ParquetFooter.json).

<a id="op-b5dbf337ed7e73bfbf58e27d"></a>
## ParquetFooter

`struct` · `buoyant_kernel::ParquetFooter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ParquetFooter
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L735).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:735`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Metadata from a Parquet file footer.

This struct contains metadata extracted from a Parquet file's footer, including the schema.
It is designed to be extensible for future additions such as row group statistics.

<a id="op-11f1fe5884cd5eb17e3e02e0"></a>
## clone

`function` · `buoyant_kernel::ParquetFooter::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ParquetFooter
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L734).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::ParquetFooter", "path": "ParquetFooter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [734, 17], "end": [734, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:734`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2700ee6ccfce226109cf5cc"></a>
## fmt

`function` · `buoyant_kernel::ParquetFooter::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L734).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::ParquetFooter", "path": "ParquetFooter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [734, 10], "end": [734, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:734`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f23a86e53c5e20480ed23c4"></a>
## schema

`struct_field` · `buoyant_kernel::ParquetFooter::schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
schema: self::schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L737).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:737`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The schema of the Parquet file, converted to Delta Kernel's schema format.
