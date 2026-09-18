# `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.metadata.xdbc_info.XdbcTypeInfo.json).

<a id="op-bfed009298c4a18a52f417be"></a>
## XdbcTypeInfo

`struct` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo` · arrow-flight 59.3.0

```rust
struct XdbcTypeInfo
```

Source: `src/sql/metadata/xdbc_info.rs:43`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Data structure representing type information for xdbc types.

<a id="op-76e6cc68e4002c27fe9868bf"></a>
## auto_increment

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::auto_increment` · arrow-flight 59.3.0

```rust
auto_increment: Option<bool>
```

Source: `src/sql/metadata/xdbc_info.rs:67`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Whether the type is auto-incrementing

<a id="op-1fe60b5639d79e9120f95b53"></a>
## case_sensitive

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::case_sensitive` · arrow-flight 59.3.0

```rust
case_sensitive: bool
```

Source: `src/sql/metadata/xdbc_info.rs:59`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Whether the type is case sensitive

<a id="op-fe3de8c6216648e5e8c93a61"></a>
## clone

`function` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> XdbcTypeInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo", "path": "XdbcTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 17], "end": [42, 22], "filename": "src/sql/metadata/xdbc_info.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/metadata/xdbc_info.rs:42`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-011e4312830813cd1cbb4b13"></a>
## column_size

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::column_size` · arrow-flight 59.3.0

```rust
column_size: Option<i32>
```

Source: `src/sql/metadata/xdbc_info.rs:49`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The column size of the type

<a id="op-d940dc9899237e3bba86bfca"></a>
## create_params

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::create_params` · arrow-flight 59.3.0

```rust
create_params: Option<Vec<String>>
```

Source: `src/sql/metadata/xdbc_info.rs:55`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The create parameters of the type

<a id="op-dc3a64462db8048a47cdd81a"></a>
## data_type

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::data_type` · arrow-flight 59.3.0

```rust
data_type: sql::XdbcDataType
```

Source: `src/sql/metadata/xdbc_info.rs:47`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The data type of the type

<a id="op-cf836d116a4cc749d9d42c28"></a>
## datetime_subcode

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::datetime_subcode` · arrow-flight 59.3.0

```rust
datetime_subcode: Option<sql::XdbcDatetimeSubcode>
```

Source: `src/sql/metadata/xdbc_info.rs:77`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The optional datetime subcode of the type

<a id="op-619955d2bdf6b44de6d76ef8"></a>
## default

`function` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::default` · arrow-flight 59.3.0

```rust
fn default() -> XdbcTypeInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo", "path": "XdbcTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 24], "end": [42, 31], "filename": "src/sql/metadata/xdbc_info.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/metadata/xdbc_info.rs:42`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-988558e24f3ebcb0c69dbc8e"></a>
## fixed_prec_scale

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::fixed_prec_scale` · arrow-flight 59.3.0

```rust
fixed_prec_scale: bool
```

Source: `src/sql/metadata/xdbc_info.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Whether the type has fixed precision and scale

<a id="op-28335acab2bffd34e5936cb3"></a>
## fmt

`function` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo", "path": "XdbcTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 10], "end": [42, 15], "filename": "src/sql/metadata/xdbc_info.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/metadata/xdbc_info.rs:42`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-732fa94f751482ee5edef97c"></a>
## interval_precision

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::interval_precision` · arrow-flight 59.3.0

```rust
interval_precision: Option<i32>
```

Source: `src/sql/metadata/xdbc_info.rs:81`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The interval precision of the type

<a id="op-78b4c36b1809fb4a5bd186f5"></a>
## literal_prefix

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::literal_prefix` · arrow-flight 59.3.0

```rust
literal_prefix: Option<String>
```

Source: `src/sql/metadata/xdbc_info.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The prefix of the type

<a id="op-12955149a3b9dbe405972e0f"></a>
## literal_suffix

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::literal_suffix` · arrow-flight 59.3.0

```rust
literal_suffix: Option<String>
```

Source: `src/sql/metadata/xdbc_info.rs:53`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The suffix of the type

<a id="op-73eb70464351f3f8e83bfc3d"></a>
## local_type_name

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::local_type_name` · arrow-flight 59.3.0

```rust
local_type_name: Option<String>
```

Source: `src/sql/metadata/xdbc_info.rs:69`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The local type name of the type

<a id="op-48a9345bfa3de3fd89e84568"></a>
## maximum_scale

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::maximum_scale` · arrow-flight 59.3.0

```rust
maximum_scale: Option<i32>
```

Source: `src/sql/metadata/xdbc_info.rs:73`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The maximum scale of the type

<a id="op-6d2858f11d6380e6ef0c9e41"></a>
## minimum_scale

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::minimum_scale` · arrow-flight 59.3.0

```rust
minimum_scale: Option<i32>
```

Source: `src/sql/metadata/xdbc_info.rs:71`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The minimum scale of the type

<a id="op-36c0ceb5041a9f35ecf8d693"></a>
## nullable

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::nullable` · arrow-flight 59.3.0

```rust
nullable: sql::Nullable
```

Source: `src/sql/metadata/xdbc_info.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The nullability of the type

<a id="op-f811aa9b254e267baa84d29e"></a>
## num_prec_radix

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::num_prec_radix` · arrow-flight 59.3.0

```rust
num_prec_radix: Option<i32>
```

Source: `src/sql/metadata/xdbc_info.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The number precision radix of the type

<a id="op-00ea52a14e01bb705116fed6"></a>
## searchable

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::searchable` · arrow-flight 59.3.0

```rust
searchable: sql::Searchable
```

Source: `src/sql/metadata/xdbc_info.rs:61`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Whether the type is searchable

<a id="op-d45366faa3e99d68d8e4ee03"></a>
## sql_data_type

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::sql_data_type` · arrow-flight 59.3.0

```rust
sql_data_type: sql::XdbcDataType
```

Source: `src/sql/metadata/xdbc_info.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The SQL data type of the type

<a id="op-5df3b25e7c2b0c4e571b48d3"></a>
## type_name

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::type_name` · arrow-flight 59.3.0

```rust
type_name: String
```

Source: `src/sql/metadata/xdbc_info.rs:45`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The name of the type

<a id="op-33e21183ee41d28f22156663"></a>
## unsigned_attribute

`struct_field` · `arrow_flight::sql::metadata::xdbc_info::XdbcTypeInfo::unsigned_attribute` · arrow-flight 59.3.0

```rust
unsigned_attribute: Option<bool>
```

Source: `src/sql/metadata/xdbc_info.rs:63`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Whether the type is unsigned
