# `arrow_ipc::gen::Schema::UnionArgs`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.UnionArgs.json).

<a id="op-de2f69777e2ead5903d2ca8c"></a>
## UnionArgs

`struct` · `arrow_ipc::gen::Schema::UnionArgs` · arrow-ipc 59.3.0

```rust
struct UnionArgs<'a>
```

Source: `src/gen/Schema.rs:2066`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-726005dc5edf89589f67e51e"></a>
## default

`function` · `arrow_ipc::gen::Schema::UnionArgs::default` · arrow-ipc 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::UnionArgs", "path": "UnionArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2070, 1], "end": [2078, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gen/Schema.rs:2072`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56dfc997fbf7e3b3f322bedc"></a>
## mode

`struct_field` · `arrow_ipc::gen::Schema::UnionArgs::mode` · arrow-ipc 59.3.0

```rust
mode: UnionMode
```

Source: `src/gen/Schema.rs:2067`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84b3b34bfc9950bddb59f829"></a>
## typeIds

`struct_field` · `arrow_ipc::gen::Schema::UnionArgs::typeIds` · arrow-ipc 59.3.0

```rust
typeIds: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, i32>>>
```

Source: `src/gen/Schema.rs:2068`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
