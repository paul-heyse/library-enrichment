# `datafusion_sql::planner::NullOrdering`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.planner.NullOrdering.json).

<a id="op-9468a3fdf83266ade3681014"></a>
## NullOrdering

`enum` · `datafusion_sql::planner::NullOrdering` · datafusion-sql 55.1.0

```rust
enum NullOrdering
```

Source: `src/planner.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Represents the null ordering for sorting expressions.

<a id="op-75f671577c4dafac6f2fc00f"></a>
## Err

`assoc_type` · `datafusion_sql::planner::NullOrdering::Err` · datafusion-sql 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::NullOrdering", "path": "NullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [210, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/planner.rs:197`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a434bb32d58f48bade6f5735"></a>
## NullsFirst

`variant` · `datafusion_sql::planner::NullOrdering::NullsFirst` · datafusion-sql 55.1.0

```rust
NullsFirst
```

Source: `src/planner.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Nulls appear first.

<a id="op-7d0119e58663d1b8e542cfa4"></a>
## NullsLast

`variant` · `datafusion_sql::planner::NullOrdering::NullsLast` · datafusion-sql 55.1.0

```rust
NullsLast
```

Source: `src/planner.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Nulls appear last.

<a id="op-0a1a0ca9a3a9c31aa46233aa"></a>
## NullsMax

`variant` · `datafusion_sql::planner::NullOrdering::NullsMax` · datafusion-sql 55.1.0

```rust
NullsMax
```

Source: `src/planner.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Nulls appear last in ascending order.

<a id="op-4dc705707511be4a441b94a3"></a>
## NullsMin

`variant` · `datafusion_sql::planner::NullOrdering::NullsMin` · datafusion-sql 55.1.0

```rust
NullsMin
```

Source: `src/planner.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Nulls appear first in descending order.

<a id="op-522a167b2a6067e1387e7130"></a>
## clone

`function` · `datafusion_sql::planner::NullOrdering::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> NullOrdering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::NullOrdering", "path": "NullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 17], "end": [168, 22], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/planner.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2eb0675fb0c2fa75bfa2d1e"></a>
## fmt

`function` · `datafusion_sql::planner::NullOrdering::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::NullOrdering", "path": "NullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 10], "end": [168, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5c608830bc5da5269e5765a"></a>
## from

`function` · `datafusion_sql::planner::NullOrdering::from` · datafusion-sql 55.1.0

```rust
fn from(s: &str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::NullOrdering", "path": "NullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [216, 2], "filename": "src/planner.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/planner.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c07b2517183d4453e6546efd"></a>
## from_str

`function` · `datafusion_sql::planner::NullOrdering::from_str` · datafusion-sql 55.1.0

```rust
fn from_str(s: &str) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::NullOrdering", "path": "NullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [210, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/planner.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-280c3a74aa2ab32a325ed86a"></a>
## nulls_first

`function` · `datafusion_sql::planner::NullOrdering::nulls_first` · datafusion-sql 55.1.0

```rust
fn nulls_first(&self, asc: bool) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::NullOrdering", "path": "NullOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 1], "end": [194, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Evaluates the null ordering based on the given ascending flag.

# Returns
* `true` if nulls should appear first.
* `false` if nulls should appear last.
