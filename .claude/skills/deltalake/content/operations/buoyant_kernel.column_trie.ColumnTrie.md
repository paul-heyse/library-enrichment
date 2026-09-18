# `buoyant_kernel::column_trie::ColumnTrie`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.column_trie.ColumnTrie.json).

<a id="op-98dbbade94b8dcc444c33b0e"></a>
## ColumnTrie

`struct` · `buoyant_kernel::column_trie::ColumnTrie` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ColumnTrie<'col>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/column_trie.rs#L23).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A trie (prefix tree) for efficient column path matching.

The lifetime `'col` ties this trie to the column names it was built from,
allowing it to borrow string slices instead of cloning.

The `Default` implementation creates an empty trie node with no children and
`is_terminal = false`. This is used both for creating a new root trie and for
creating intermediate nodes during insertion (via `or_default()`).

<a id="op-4a2433159bf000c4ef35bef0"></a>
## contains_prefix_of

`function` · `buoyant_kernel::column_trie::ColumnTrie::contains_prefix_of` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn contains_prefix_of(&self, path: &[String]) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/column_trie.rs#L117).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'col"}], "constraints": []}}, "id": "buoyant_kernel::column_trie::ColumnTrie", "path": "ColumnTrie"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'col"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [133, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs:117`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns true if `path` equals or is a descendant of any inserted column.

For example, if the trie contains `["a", "b"]`:
- `["a", "b"]` -> true (exact match)
- `["a", "b", "c"]` -> true (descendant)
- `["a"]` -> false (ancestor, not descendant)
- `["a", "x"]` -> false (divergent path)

<a id="op-f68d758acc0ed34a429baef0"></a>
## default

`function` · `buoyant_kernel::column_trie::ColumnTrie::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> ColumnTrie<'col>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/column_trie.rs#L21).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'col"}], "constraints": []}}, "id": "buoyant_kernel::column_trie::ColumnTrie", "path": "ColumnTrie"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'col"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 17], "end": [21, 24], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs:21`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc6b4e92340091a76ae52a55"></a>
## fmt

`function` · `buoyant_kernel::column_trie::ColumnTrie::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/column_trie.rs#L21).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'col"}], "constraints": []}}, "id": "buoyant_kernel::column_trie::ColumnTrie", "path": "ColumnTrie"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'col"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 10], "end": [21, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs:21`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3788e6953d9ef3bee2723df6"></a>
## from_columns

`function` · `buoyant_kernel::column_trie::ColumnTrie::from_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_columns(columns: &'col [ColumnName]) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/column_trie.rs#L48).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'col"}], "constraints": []}}, "id": "buoyant_kernel::column_trie::ColumnTrie", "path": "ColumnTrie"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'col"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [133, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs:48`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builds a trie from a list of column names.

For example, `from_columns(&[column_name!("a.b"), column_name!("a.c")])` creates:
```text
root (is_terminal=false)
└── "a" (is_terminal=false)
    ├── "b" (is_terminal=true)
    └── "c" (is_terminal=true)
```

<a id="op-6c02f95db0a599f4156dc2de"></a>
## insert

`function` · `buoyant_kernel::column_trie::ColumnTrie::insert` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn insert(&mut self, column: &'col ColumnName)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/column_trie.rs#L70).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'col"}], "constraints": []}}, "id": "buoyant_kernel::column_trie::ColumnTrie", "path": "ColumnTrie"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'col"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [133, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs:70`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Inserts a column path into the trie.

Walks down the trie for each path component, creating nodes as needed via `or_default()`
(which initializes `is_terminal = false`). After the loop, only the final node is marked
as terminal.

For example, inserting `a.b.c` creates:
```text
root (is_terminal=false)
└── "a" (is_terminal=false)
    └── "b" (is_terminal=false)
        └── "c" (is_terminal=true)
```

<a id="op-1a89c2edd81200e3a200e746"></a>
## is_terminal

`function` · `buoyant_kernel::column_trie::ColumnTrie::is_terminal` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_terminal(&self, path: &[String]) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/column_trie.rs#L98).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'col"}], "constraints": []}}, "id": "buoyant_kernel::column_trie::ColumnTrie", "path": "ColumnTrie"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'col"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [133, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs:98`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns true if `path` exactly matches a terminal (leaf) column in the trie.

This is used by stats collection to detect columns that are structs at the Arrow level
but should be treated as leaf columns for statistics (e.g. Variant, which is
`Struct { metadata, value }` in Arrow but a single leaf in the stats schema). For such
columns, stats collection computes nullCount at the struct level instead of recursing
into sub-fields.

Unlike [`contains_prefix_of`], this does NOT match descendants. It returns true only
when the path ends at a terminal node. `contains_prefix_of` would return true for both
a terminal column and its descendants, which can't distinguish a stats-leaf struct from
a regular struct whose children are stats columns.

For example, if the trie contains `["a", "b"]`:
- `["a", "b"]` -> true (exact terminal match)
- `["a", "b", "c"]` -> false (descendant, not exact)
- `["a"]` -> false (not terminal)

[`contains_prefix_of`]: Self::contains_prefix_of

<a id="op-cc5f27083afe84b8e8fd9b70"></a>
## new

`function` · `buoyant_kernel::column_trie::ColumnTrie::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/column_trie.rs#L34).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'col"}], "constraints": []}}, "id": "buoyant_kernel::column_trie::ColumnTrie", "path": "ColumnTrie"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'col"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [133, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs:34`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates an empty trie.

<a id="op-ccfc1e3bddfd3c127bce52cf"></a>
## children

`struct_field` · `buoyant_kernel::column_trie::ColumnTrie::children` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
children: std::collections::HashMap<&'col str, ColumnTrie<'col>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/column_trie.rs#L24).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs:24`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1f4c5dba71f2953729ac1a0"></a>
## is_terminal

`struct_field` · `buoyant_kernel::column_trie::ColumnTrie::is_terminal` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
is_terminal: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/column_trie.rs#L28).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs:28`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

True if this node represents the end of a specified column path.
Intermediate nodes have `is_terminal = false`; only the final node of
an inserted column path has `is_terminal = true`.
