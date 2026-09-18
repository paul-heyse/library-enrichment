# `arrow_flight::sql::gen::CommandGetXdbcTypeInfo`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandGetXdbcTypeInfo.json).

<a id="op-9d0281cac1bf91cc4944a7dc"></a>
## CommandGetXdbcTypeInfo

`struct` · `arrow_flight::sql::gen::CommandGetXdbcTypeInfo` · arrow-flight 59.3.0

```rust
struct CommandGetXdbcTypeInfo
```

Source: `src/sql/arrow.flight.protocol.sql.rs:103`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a request to retrieve information about data type supported on a Flight SQL enabled backend.
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned schema will be:
<
   type_name: utf8 not null (The name of the data type, for example: VARCHAR, INTEGER, etc),
   data_type: int32 not null (The SQL data type),
   column_size: int32 (The maximum size supported by that column.
                       In case of exact numeric types, this represents the maximum precision.
                       In case of string types, this represents the character length.
                       In case of datetime data types, this represents the length in characters of the string representation.
                       NULL is returned for data types where column size is not applicable.),
   literal_prefix: utf8 (Character or characters used to prefix a literal, NULL is returned for
                         data types where a literal prefix is not applicable.),
   literal_suffix: utf8 (Character or characters used to terminate a literal,
                         NULL is returned for data types where a literal suffix is not applicable.),
   create_params: list<utf8 not null>
                        (A list of keywords corresponding to which parameters can be used when creating
                         a column for that specific type.
                         NULL is returned if there are no parameters for the data type definition.),
   nullable: int32 not null (Shows if the data type accepts a NULL value. The possible values can be seen in the
                             Nullable enum.),
   case_sensitive: bool not null (Shows if a character data type is case-sensitive in collations and comparisons),
   searchable: int32 not null (Shows how the data type is used in a WHERE clause. The possible values can be seen in the
                               Searchable enum.),
   unsigned_attribute: bool (Shows if the data type is unsigned. NULL is returned if the attribute is
                             not applicable to the data type or the data type is not numeric.),
   fixed_prec_scale: bool not null (Shows if the data type has predefined fixed precision and scale.),
   auto_increment: bool (Shows if the data type is auto incremental. NULL is returned if the attribute
                         is not applicable to the data type or the data type is not numeric.),
   local_type_name: utf8 (Localized version of the data source-dependent name of the data type. NULL
                          is returned if a localized name is not supported by the data source),
   minimum_scale: int32 (The minimum scale of the data type on the data source.
                         If a data type has a fixed scale, the MINIMUM_SCALE and MAXIMUM_SCALE
                         columns both contain this value. NULL is returned if scale is not applicable.),
   maximum_scale: int32 (The maximum scale of the data type on the data source.
                         NULL is returned if scale is not applicable.),
   sql_data_type: int32 not null (The value of the SQL DATA TYPE which has the same values
                                  as data_type value. Except for interval and datetime, which
                                  uses generic values. More info about those types can be
                                  obtained through datetime_subcode. The possible values can be seen
                                  in the XdbcDataType enum.),
   datetime_subcode: int32 (Only used when the SQL DATA TYPE is interval or datetime. It contains
                            its sub types. For type different from interval and datetime, this value
                            is NULL. The possible values can be seen in the XdbcDatetimeSubcode enum.),
   num_prec_radix: int32 (If the data type is an approximate numeric type, this column contains
                          the value 2 to indicate that COLUMN_SIZE specifies a number of bits. For
                          exact numeric types, this column contains the value 10 to indicate that
                          column size specifies a number of decimal digits. Otherwise, this column is NULL.),
   interval_precision: int32 (If the data type is an interval data type, then this column contains the value
                              of the interval leading precision. Otherwise, this column is NULL. This fields
                              is only relevant to be used by ODBC).
>
The returned data should be ordered by data_type and then by type_name.

<a id="op-e07714cc4e86bdc6654cb8dd"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandGetXdbcTypeInfo::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetXdbcTypeInfo", "path": "CommandGetXdbcTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0626baeecbef52730f085b7c"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandGetXdbcTypeInfo::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetXdbcTypeInfo", "path": "CommandGetXdbcTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 44], "end": [102, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-743b131e7b216eb62a9692f7"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandGetXdbcTypeInfo::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandGetXdbcTypeInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetXdbcTypeInfo", "path": "CommandGetXdbcTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 10], "end": [102, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52ed1485f1996732c8d13911"></a>
## data_type

`function` · `arrow_flight::sql::gen::CommandGetXdbcTypeInfo::data_type` · arrow-flight 59.3.0

```rust
fn data_type(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetXdbcTypeInfo", "path": "CommandGetXdbcTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 44], "end": [102, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `data_type`, or the default value if `data_type` is unset.

<a id="op-7c9f31c2994e6197fe8acaad"></a>
## data_type

`struct_field` · `arrow_flight::sql::gen::CommandGetXdbcTypeInfo::data_type` · arrow-flight 59.3.0

```rust
data_type: ::core::option::Option<i32>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:107`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Specifies the data type to search for the info.

<a id="op-8db5d30ee923458f3d166a51"></a>
## default

`function` · `arrow_flight::sql::gen::CommandGetXdbcTypeInfo::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetXdbcTypeInfo", "path": "CommandGetXdbcTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 44], "end": [102, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a87bbcbe0f69efa6680740a6"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandGetXdbcTypeInfo::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetXdbcTypeInfo", "path": "CommandGetXdbcTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 44], "end": [102, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-173fb47c23890ea1bcd14a04"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandGetXdbcTypeInfo::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandGetXdbcTypeInfo) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetXdbcTypeInfo", "path": "CommandGetXdbcTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 23], "end": [102, 32], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d9fa7c1f6f4dcd3edcdc29f"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandGetXdbcTypeInfo::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetXdbcTypeInfo", "path": "CommandGetXdbcTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 44], "end": [102, 60], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c67a2f0c10f43fecd1782c5"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandGetXdbcTypeInfo::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetXdbcTypeInfo", "path": "CommandGetXdbcTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 38], "end": [102, 42], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e3c5fc281a7a6533b19ed49"></a>
## into_builder

`function` · `arrow_flight::sql::gen::CommandGetXdbcTypeInfo::into_builder` · arrow-flight 59.3.0

```rust
fn into_builder(self, infos: &XdbcTypeInfoData) -> GetXdbcTypeInfoBuilder<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetXdbcTypeInfo", "path": "crate::sql::CommandGetXdbcTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [308, 2], "filename": "src/sql/metadata/xdbc_info.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/xdbc_info.rs:302`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a builder suitable for constructing a response

<a id="op-75103e9b368c33bc43163d21"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandGetXdbcTypeInfo::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetXdbcTypeInfo", "path": "CommandGetXdbcTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
