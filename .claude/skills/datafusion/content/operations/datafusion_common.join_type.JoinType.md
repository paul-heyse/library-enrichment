# `datafusion_common::join_type::JoinType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.join_type.JoinType.json).

<a id="op-5dcdf4ccd1d8304160fcb295"></a>
## JoinType

`enum` · `datafusion_common::join_type::JoinType` · datafusion-common 55.1.0

```rust
enum JoinType
```

Source: `src/join_type.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Join type

<a id="op-40978b9231e19d6f33957f71"></a>
## Err

`assoc_type` · `datafusion_common::join_type::JoinType::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinType", "path": "JoinType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [216, 2], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/join_type.rs:198`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8814533155eaa08966361695"></a>
## Full

`variant` · `datafusion_common::join_type::JoinType::Full` · datafusion-common 55.1.0

```rust
Full
```

Source: `src/join_type.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Full Join (also called Full Outer Join) - Returns all rows from both tables, matching rows where possible.
When a row from either table has no match in the other table, the missing columns are filled with NULL values.
For example, if table A has row X with no match in table B, the result will contain row X with NULL values for all of table B's columns.
This join type preserves all records from both tables, making it useful when you need to see all data regardless of matches.

<a id="op-e3478b1e114578a0d1d4bd08"></a>
## Inner

`variant` · `datafusion_common::join_type::JoinType::Inner` · datafusion-common 55.1.0

```rust
Inner
```

Source: `src/join_type.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Inner Join - Returns only rows where there is a matching value in both tables based on the join condition.
For example, if joining table A and B on A.id = B.id, only rows where A.id equals B.id will be included.
All columns from both tables are returned for the matching rows. Non-matching rows are excluded entirely.

<a id="op-e736604238d9ce01db3943bb"></a>
## Left

`variant` · `datafusion_common::join_type::JoinType::Left` · datafusion-common 55.1.0

```rust
Left
```

Source: `src/join_type.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Left Join - Returns all rows from the left table and matching rows from the right table.
If no match, NULL values are returned for columns from the right table.

<a id="op-d0b65ad709051a32ed962727"></a>
## LeftAnti

`variant` · `datafusion_common::join_type::JoinType::LeftAnti` · datafusion-common 55.1.0

```rust
LeftAnti
```

Source: `src/join_type.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Left Anti Join - Returns rows from the left table that do not have a matching row in the right table.

<a id="op-cc52c7040c9e7e05cb61dce0"></a>
## LeftMark

`variant` · `datafusion_common::join_type::JoinType::LeftMark` · datafusion-common 55.1.0

```rust
LeftMark
```

Source: `src/join_type.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Left Mark join

Returns one record for each record from the left input. The output contains an additional
column "mark" which is true if there is at least one match in the right input where the
join condition evaluates to true. Otherwise, the mark column is false. For more details see
[1]. This join type is used to decorrelate EXISTS subqueries used inside disjunctive
predicates.

Note: This we currently do not implement the full null semantics for the mark join described
in [1] which will be needed if we and ANY subqueries. In our version the mark column will
only be true for had a match and false when no match was found, never null.

[1]: http://btw2017.informatik.uni-stuttgart.de/slidesandpapers/F1-10-37/paper_web.pdf

<a id="op-2f62a993394580199465213f"></a>
## LeftSemi

`variant` · `datafusion_common::join_type::JoinType::LeftSemi` · datafusion-common 55.1.0

```rust
LeftSemi
```

Source: `src/join_type.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Left Semi Join - Returns rows from the left table that have matching rows in the right table.
Only columns from the left table are returned.

<a id="op-3d6085dbc4b0d4004446b274"></a>
## Right

`variant` · `datafusion_common::join_type::JoinType::Right` · datafusion-common 55.1.0

```rust
Right
```

Source: `src/join_type.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Right Join - Returns all rows from the right table and matching rows from the left table.
If no match, NULL values are returned for columns from the left table.

<a id="op-825eb1912d1a7574ffedfe6e"></a>
## RightAnti

`variant` · `datafusion_common::join_type::JoinType::RightAnti` · datafusion-common 55.1.0

```rust
RightAnti
```

Source: `src/join_type.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Right Anti Join - Returns rows from the right table that do not have a matching row in the left table.

<a id="op-afee6ba73b01cae92730ddf5"></a>
## RightMark

`variant` · `datafusion_common::join_type::JoinType::RightMark` · datafusion-common 55.1.0

```rust
RightMark
```

Source: `src/join_type.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Right Mark Join

Same logic as the LeftMark Join above, however it returns a record for each record from the
right input.

<a id="op-c12bd9608dbc6dcafbbc9417"></a>
## RightSemi

`variant` · `datafusion_common::join_type::JoinType::RightSemi` · datafusion-common 55.1.0

```rust
RightSemi
```

Source: `src/join_type.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Right Semi Join - Returns rows from the right table that have matching rows in the left table.
Only columns from the right table are returned.

<a id="op-b14b91cee70ef20ef2632451"></a>
## clone

`function` · `datafusion_common::join_type::JoinType::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> JoinType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinType", "path": "JoinType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 17], "end": [29, 22], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/join_type.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd98aaf83596a090c0e118a9"></a>
## empty_build_side_produces_empty_result

`function` · `datafusion_common::join_type::JoinType::empty_build_side_produces_empty_result` · datafusion-common 55.1.0

```rust
fn empty_build_side_produces_empty_result(self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinType", "path": "JoinType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [177, 2], "filename": "src/join_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_type.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns true when an empty build side necessarily produces an empty
result for this join type.

<a id="op-21bf23f0d383f8b2b926e6de"></a>
## empty_map_produces_empty_result

`function` · `datafusion_common::join_type::JoinType::empty_map_produces_empty_result` · datafusion-common 55.1.0

```rust
fn empty_map_produces_empty_result(self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinType", "path": "JoinType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [177, 2], "filename": "src/join_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_type.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns true when an empty build-side map necessarily produces an empty
result for this join type, even if the build side still contains rows.

Every output row of these join types requires a matching build row, so
when the map has no matchable keys the result is empty regardless of the
probe side. Note this is a subset of
[`Self::empty_build_side_produces_empty_result`](../operations/datafusion_common.join_type.JoinType.md#op-cd98aaf83596a090c0e118a9): an empty build side
yields an empty map, but the map can also be empty when every build row
has a NULL join key under [`NullEquality::NullEqualsNothing`].

[`NullEquality::NullEqualsNothing`]: crate::NullEquality::NullEqualsNothing

<a id="op-9d13aede95689317e617afc0"></a>
## eq

`function` · `datafusion_common::join_type::JoinType::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &JoinType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinType", "path": "JoinType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 30], "end": [29, 39], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/join_type.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a049a875cc270be633f5500"></a>
## fmt

`function` · `datafusion_common::join_type::JoinType::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinType", "path": "JoinType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/join_type.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf2614c72e753c5fd801aa0f"></a>
## fmt

`function` · `datafusion_common::join_type::JoinType::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinType", "path": "JoinType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [195, 2], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/join_type.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba17f07be28c0682eaefa1db"></a>
## from_str

`function` · `datafusion_common::join_type::JoinType::from_str` · datafusion-common 55.1.0

```rust
fn from_str(s: &str) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinType", "path": "JoinType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [216, 2], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/join_type.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0892454735546b013c8e6cf"></a>
## hash

`function` · `datafusion_common::join_type::JoinType::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinType", "path": "JoinType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 57], "end": [29, 61], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/join_type.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cec70bce8705f8332a980f9"></a>
## is_outer

`function` · `datafusion_common::join_type::JoinType::is_outer` · datafusion-common 55.1.0

```rust
fn is_outer(self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinType", "path": "JoinType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [177, 2], "filename": "src/join_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_type.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47f4bf3fddc31ea2655bad16"></a>
## on_lr_is_preserved

`function` · `datafusion_common::join_type::JoinType::on_lr_is_preserved` · datafusion-common 55.1.0

```rust
fn on_lr_is_preserved(&self) -> (bool, bool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinType", "path": "JoinType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [177, 2], "filename": "src/join_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_type.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Whether each side of the join is preserved for ON-clause filter pushdown.

It is only correct to push ON-clause filters below a join for preserved
inputs.

# "Preserved" input definition

A join side is preserved if the join returns all or a subset of the rows
from that side, such that each output row directly maps to an input row.
If a side is not preserved, the join can produce extra null rows that
don't map to any input row.

# Return Value

A tuple of booleans - (left_preserved, right_preserved).

<a id="op-9c828adeaea191357018f851"></a>
## partial_cmp

`function` · `datafusion_common::join_type::JoinType::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &JoinType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinType", "path": "JoinType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 45], "end": [29, 55], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/join_type.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9b74dc52a6b6218a15c8c61"></a>
## supports_swap

`function` · `datafusion_common::join_type::JoinType::supports_swap` · datafusion-common 55.1.0

```rust
fn supports_swap(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinType", "path": "JoinType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [177, 2], "filename": "src/join_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_type.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Does the join type support swapping inputs?

<a id="op-5b0febfe13f713de63f718bd"></a>
## swap

`function` · `datafusion_common::join_type::JoinType::swap` · datafusion-common 55.1.0

```rust
fn swap(&self) -> JoinType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinType", "path": "JoinType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [177, 2], "filename": "src/join_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_type.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the `JoinType` if the (2) inputs were swapped

Panics if [`Self::supports_swap`](../operations/datafusion_common.join_type.JoinType.md#op-a9b74dc52a6b6218a15c8c61) returns false
