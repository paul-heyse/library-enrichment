# `arrow_array::timezone::private::Tz`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.timezone.private.Tz.json).

<a id="op-558b488a36daef9703572670"></a>
## Tz

`struct` · `arrow_array::timezone::private::Tz` · arrow-array 59.3.0

```rust
struct Tz
```

Source: `src/timezone.rs:80`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An Arrow [`TimeZone`]

Unresolved upstream links (retained, not inferred): ``TimeZone``.

<a id="op-ae0b4ecf6a168cc1f95cae43"></a>
## Err

`assoc_type` · `arrow_array::timezone::private::Tz::Err` · arrow-array 59.3.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::timezone::private::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 5], "end": [99, 6], "filename": "src/timezone.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/timezone.rs:89`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cffa7f0ddc2bfa28019ca30"></a>
## Offset

`assoc_type` · `arrow_array::timezone::private::Tz::Offset` · arrow-array 59.3.0

```rust
Offset
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::timezone::private::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 5], "end": [161, 6], "filename": "src/timezone.rs"}, "trait": {"args": null, "id": "chrono::offset::TimeZone", "path": "TimeZone"}, "trait_path": "chrono::offset::TimeZone"}`

Source: `src/timezone.rs:120`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3c51d95352ff415b64e7e93"></a>
## clone

`function` · `arrow_array::timezone::private::Tz::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> Tz
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::timezone::private::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 27], "end": [79, 32], "filename": "src/timezone.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/timezone.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3343c9b3d511f6f1bbd78288"></a>
## fmt

`function` · `arrow_array::timezone::private::Tz::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::timezone::private::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 14], "end": [79, 19], "filename": "src/timezone.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/timezone.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66e3fabfbdd3f27eb5b24013"></a>
## fmt

`function` · `arrow_array::timezone::private::Tz::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::timezone::private::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 5], "end": [108, 6], "filename": "src/timezone.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/timezone.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e52d374f8e8d58adba2ba236"></a>
## from_offset

`function` · `arrow_array::timezone::private::Tz::from_offset` · arrow-array 59.3.0

```rust
fn from_offset(offset: &Self::Offset) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::timezone::private::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 5], "end": [161, 6], "filename": "src/timezone.rs"}, "trait": {"args": null, "id": "chrono::offset::TimeZone", "path": "TimeZone"}, "trait_path": "chrono::offset::TimeZone"}`

Source: `src/timezone.rs:122`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0916e8ab8179f090f83afc51"></a>
## from_str

`function` · `arrow_array::timezone::private::Tz::from_str` · arrow-array 59.3.0

```rust
fn from_str(tz: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::timezone::private::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 5], "end": [99, 6], "filename": "src/timezone.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/timezone.rs:91`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fa43cde1b77655aab21403f"></a>
## offset_from_local_date

`function` · `arrow_array::timezone::private::Tz::offset_from_local_date` · arrow-array 59.3.0

```rust
fn offset_from_local_date(&self, local: &NaiveDate) -> LocalResult<Self::Offset>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::timezone::private::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 5], "end": [161, 6], "filename": "src/timezone.rs"}, "trait": {"args": null, "id": "chrono::offset::TimeZone", "path": "TimeZone"}, "trait_path": "chrono::offset::TimeZone"}`

Source: `src/timezone.rs:126`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76c7749db061295304fef772"></a>
## offset_from_local_datetime

`function` · `arrow_array::timezone::private::Tz::offset_from_local_datetime` · arrow-array 59.3.0

```rust
fn offset_from_local_datetime(&self, local: &NaiveDateTime) -> LocalResult<Self::Offset>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::timezone::private::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 5], "end": [161, 6], "filename": "src/timezone.rs"}, "trait": {"args": null, "id": "chrono::offset::TimeZone", "path": "TimeZone"}, "trait_path": "chrono::offset::TimeZone"}`

Source: `src/timezone.rs:135`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-971adf256ffff5ce7e67e1ee"></a>
## offset_from_utc_date

`function` · `arrow_array::timezone::private::Tz::offset_from_utc_date` · arrow-array 59.3.0

```rust
fn offset_from_utc_date(&self, utc: &NaiveDate) -> Self::Offset
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::timezone::private::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 5], "end": [161, 6], "filename": "src/timezone.rs"}, "trait": {"args": null, "id": "chrono::offset::TimeZone", "path": "TimeZone"}, "trait_path": "chrono::offset::TimeZone"}`

Source: `src/timezone.rs:144`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7de2fcbf437b40e25080308"></a>
## offset_from_utc_datetime

`function` · `arrow_array::timezone::private::Tz::offset_from_utc_datetime` · arrow-array 59.3.0

```rust
fn offset_from_utc_datetime(&self, utc: &NaiveDateTime) -> Self::Offset
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::timezone::private::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 5], "end": [161, 6], "filename": "src/timezone.rs"}, "trait": {"args": null, "id": "chrono::offset::TimeZone", "path": "TimeZone"}, "trait_path": "chrono::offset::TimeZone"}`

Source: `src/timezone.rs:153`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
