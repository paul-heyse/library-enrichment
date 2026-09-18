# `datafusion_common::error::Result`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.error.Result.json).

<a id="op-b73a5a953660113193cd6983"></a>
## Result

`type_alias` · `datafusion_common::error::Result` · datafusion-common 55.1.0

```rust
type Result<T, E = DataFusionError> = result::Result<T, E>
```

Source: `src/error.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Result type for operations that could result in an [DataFusionError](../operations/datafusion_common.error.DataFusionError.md#op-d335eb18d3c98363748854bb)

<a id="op-efaf20411c10fe21e5a0e19c"></a>
## data

`function` · `datafusion_common::error::Result::data` · datafusion-common 55.1.0

```rust
fn data(self) -> Result<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}}], "constraints": []}}, "id": "datafusion_common::error::Result", "path": "crate::Result"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1249, 1], "end": [1261, 2], "filename": "src/tree_node.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::TransformedResult", "path": "TransformedResult"}, "trait_path": "datafusion_common::tree_node::TransformedResult"}`

Source: `src/tree_node.rs:1250`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e24f18023327c67f2b0008f8"></a>
## tnr

`function` · `datafusion_common::error::Result::tnr` · datafusion-common 55.1.0

```rust
fn tnr(self) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}}], "constraints": []}}, "id": "datafusion_common::error::Result", "path": "crate::Result"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1249, 1], "end": [1261, 2], "filename": "src/tree_node.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::TransformedResult", "path": "TransformedResult"}, "trait_path": "datafusion_common::tree_node::TransformedResult"}`

Source: `src/tree_node.rs:1258`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3427ad870e89b6cd0245e76a"></a>
## transformed

`function` · `datafusion_common::error::Result::transformed` · datafusion-common 55.1.0

```rust
fn transformed(self) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}}], "constraints": []}}, "id": "datafusion_common::error::Result", "path": "crate::Result"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1249, 1], "end": [1261, 2], "filename": "src/tree_node.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::TransformedResult", "path": "TransformedResult"}, "trait_path": "datafusion_common::tree_node::TransformedResult"}`

Source: `src/tree_node.rs:1254`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
