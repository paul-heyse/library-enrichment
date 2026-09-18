# `sqlparser::ast::CopyLegacyOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CopyLegacyOption.json).

<a id="op-cfebe257862b1a17b6b0dc0d"></a>
## CopyLegacyOption

`enum` · `sqlparser::ast::CopyLegacyOption` · sqlparser 0.62.0

```rust
enum CopyLegacyOption
```

Source: `src/ast/mod.rs:9378`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An option in `COPY` statement before PostgreSQL version 9.0.

[PostgreSQL](https://www.postgresql.org/docs/8.4/sql-copy.html)
[Redshift](https://docs.aws.amazon.com/redshift/latest/dg/r_COPY-alphabetical-parm-list.html)

<a id="op-33ab4802e3fcad5ad608bb53"></a>
## AcceptAnyDate

`variant` · `sqlparser::ast::CopyLegacyOption::AcceptAnyDate` · sqlparser 0.62.0

```rust
AcceptAnyDate
```

Source: `src/ast/mod.rs:9380`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ACCEPTANYDATE

<a id="op-267c8fddfe7b1ac1280f3c89"></a>
## AcceptInvChars

`variant` · `sqlparser::ast::CopyLegacyOption::AcceptInvChars` · sqlparser 0.62.0

```rust
AcceptInvChars
```

Source: `src/ast/mod.rs:9382`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ACCEPTINVCHARS

<a id="op-7859c804ef5ea61ec5d9f62a"></a>
## AddQuotes

`variant` · `sqlparser::ast::CopyLegacyOption::AddQuotes` · sqlparser 0.62.0

```rust
AddQuotes
```

Source: `src/ast/mod.rs:9384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ADDQUOTES

<a id="op-0085b2433e9eb2ebff8f49bb"></a>
## AllowOverwrite

`variant` · `sqlparser::ast::CopyLegacyOption::AllowOverwrite` · sqlparser 0.62.0

```rust
AllowOverwrite
```

Source: `src/ast/mod.rs:9386`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ALLOWOVERWRITE

<a id="op-a68a773617754e7fc241e9ff"></a>
## Binary

`variant` · `sqlparser::ast::CopyLegacyOption::Binary` · sqlparser 0.62.0

```rust
Binary
```

Source: `src/ast/mod.rs:9388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

BINARY

<a id="op-5e2185050d7154543879fa76"></a>
## BlankAsNull

`variant` · `sqlparser::ast::CopyLegacyOption::BlankAsNull` · sqlparser 0.62.0

```rust
BlankAsNull
```

Source: `src/ast/mod.rs:9390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

BLANKSASNULL

<a id="op-4b6824b997510420b8754d07"></a>
## Bzip2

`variant` · `sqlparser::ast::CopyLegacyOption::Bzip2` · sqlparser 0.62.0

```rust
Bzip2
```

Source: `src/ast/mod.rs:9392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

BZIP2

<a id="op-185a4faa78403b90d488b3d2"></a>
## CleanPath

`variant` · `sqlparser::ast::CopyLegacyOption::CleanPath` · sqlparser 0.62.0

```rust
CleanPath
```

Source: `src/ast/mod.rs:9394`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CLEANPATH

<a id="op-81ebe66495a73bb251a3e572"></a>
## CompUpdate

`variant` · `sqlparser::ast::CopyLegacyOption::CompUpdate` · sqlparser 0.62.0

```rust
CompUpdate
```

Source: `src/ast/mod.rs:9396`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

COMPUPDATE [ PRESET | { ON | TRUE } | { OFF | FALSE } ]

<a id="op-f74f66169309919e76e01c72"></a>
## Credentials

`variant` · `sqlparser::ast::CopyLegacyOption::Credentials` · sqlparser 0.62.0

```rust
Credentials
```

Source: `src/ast/mod.rs:9462`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Redshift `CREDENTIALS 'auth-args'`
<https://docs.aws.amazon.com/redshift/latest/dg/copy-parameters-authorization.html>

<a id="op-2d613e60ada5e96e4a1d964d"></a>
## Csv

`variant` · `sqlparser::ast::CopyLegacyOption::Csv` · sqlparser 0.62.0

```rust
Csv
```

Source: `src/ast/mod.rs:9403`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CSV ...

<a id="op-81552e1b905cbe325754b0d1"></a>
## DateFormat

`variant` · `sqlparser::ast::CopyLegacyOption::DateFormat` · sqlparser 0.62.0

```rust
DateFormat
```

Source: `src/ast/mod.rs:9405`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DATEFORMAT \[ AS \] {'dateformat_string' | 'auto' }

<a id="op-f8e1ee55768cd1b123615a19"></a>
## Delimiter

`variant` · `sqlparser::ast::CopyLegacyOption::Delimiter` · sqlparser 0.62.0

```rust
Delimiter
```

Source: `src/ast/mod.rs:9407`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DELIMITER \[ AS \] 'delimiter_character'

<a id="op-cc83934f28574122aa31e7d4"></a>
## EmptyAsNull

`variant` · `sqlparser::ast::CopyLegacyOption::EmptyAsNull` · sqlparser 0.62.0

```rust
EmptyAsNull
```

Source: `src/ast/mod.rs:9409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

EMPTYASNULL

<a id="op-429125ade4948b893610ce94"></a>
## Encrypted

`variant` · `sqlparser::ast::CopyLegacyOption::Encrypted` · sqlparser 0.62.0

```rust
Encrypted
```

Source: `src/ast/mod.rs:9411`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ENCRYPTED \[ AUTO \]`

<a id="op-b6e9cf228bcfc057af5cddcf"></a>
## Escape

`variant` · `sqlparser::ast::CopyLegacyOption::Escape` · sqlparser 0.62.0

```rust
Escape
```

Source: `src/ast/mod.rs:9416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ESCAPE

<a id="op-97cc477dc9784a9b957c4690"></a>
## Extension

`variant` · `sqlparser::ast::CopyLegacyOption::Extension` · sqlparser 0.62.0

```rust
Extension
```

Source: `src/ast/mod.rs:9418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

EXTENSION 'extension-name'

<a id="op-107a529d5c2846188159d9a9"></a>
## FixedWidth

`variant` · `sqlparser::ast::CopyLegacyOption::FixedWidth` · sqlparser 0.62.0

```rust
FixedWidth
```

Source: `src/ast/mod.rs:9420`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

FIXEDWIDTH \[ AS \] 'fixedwidth-spec'

<a id="op-43a2b1bf3a0aba045dead8ef"></a>
## Gzip

`variant` · `sqlparser::ast::CopyLegacyOption::Gzip` · sqlparser 0.62.0

```rust
Gzip
```

Source: `src/ast/mod.rs:9422`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

GZIP

<a id="op-043b8bf4baf0f90ca3097778"></a>
## Header

`variant` · `sqlparser::ast::CopyLegacyOption::Header` · sqlparser 0.62.0

```rust
Header
```

Source: `src/ast/mod.rs:9424`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

HEADER

<a id="op-2b7e9af3d24338ff628a0571"></a>
## IamRole

`variant` · `sqlparser::ast::CopyLegacyOption::IamRole` · sqlparser 0.62.0

```rust
IamRole
```

Source: `src/ast/mod.rs:9426`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

IAM_ROLE { DEFAULT | 'arn:aws:iam::123456789:role/role1' }

<a id="op-ecd98984086a3fe77313695d"></a>
## IgnoreHeader

`variant` · `sqlparser::ast::CopyLegacyOption::IgnoreHeader` · sqlparser 0.62.0

```rust
IgnoreHeader
```

Source: `src/ast/mod.rs:9428`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

IGNOREHEADER \[ AS \] number_rows

<a id="op-ddb3aa19dc431cd5afffe8f3"></a>
## Json

`variant` · `sqlparser::ast::CopyLegacyOption::Json` · sqlparser 0.62.0

```rust
Json
```

Source: `src/ast/mod.rs:9430`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

JSON \[ AS \] 'json_option'

<a id="op-e43578a630eae5074339ff12"></a>
## Manifest

`variant` · `sqlparser::ast::CopyLegacyOption::Manifest` · sqlparser 0.62.0

```rust
Manifest
```

Source: `src/ast/mod.rs:9432`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MANIFEST \[ VERBOSE \]

<a id="op-d90fb586f6718cc3420e5f9f"></a>
## MaxFileSize

`variant` · `sqlparser::ast::CopyLegacyOption::MaxFileSize` · sqlparser 0.62.0

```rust
MaxFileSize
```

Source: `src/ast/mod.rs:9437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MAXFILESIZE \[ AS \] max-size \[ MB | GB \]

<a id="op-75caf92214475d2eb4b4bc52"></a>
## Null

`variant` · `sqlparser::ast::CopyLegacyOption::Null` · sqlparser 0.62.0

```rust
Null
```

Source: `src/ast/mod.rs:9439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NULL \[ AS \] 'null_string'`

<a id="op-a8cfbd4e484511fc03d0bbcb"></a>
## Parallel

`variant` · `sqlparser::ast::CopyLegacyOption::Parallel` · sqlparser 0.62.0

```rust
Parallel
```

Source: `src/ast/mod.rs:9441`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`PARALLEL [ { ON | TRUE } | { OFF | FALSE } ]`

<a id="op-cdfb108fbaab9eb523151f50"></a>
## Parquet

`variant` · `sqlparser::ast::CopyLegacyOption::Parquet` · sqlparser 0.62.0

```rust
Parquet
```

Source: `src/ast/mod.rs:9443`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PARQUET

<a id="op-75fa072358cd9c939d3a8ed0"></a>
## PartitionBy

`variant` · `sqlparser::ast::CopyLegacyOption::PartitionBy` · sqlparser 0.62.0

```rust
PartitionBy
```

Source: `src/ast/mod.rs:9445`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PARTITION BY ( column_name [, ... ] ) \[ INCLUDE \]

<a id="op-720865c12cccfab41eb52ca4"></a>
## Region

`variant` · `sqlparser::ast::CopyLegacyOption::Region` · sqlparser 0.62.0

```rust
Region
```

Source: `src/ast/mod.rs:9447`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

REGION \[ AS \] 'aws-region' }

<a id="op-552ddaf1a73cbc0ab26ae8f4"></a>
## RemoveQuotes

`variant` · `sqlparser::ast::CopyLegacyOption::RemoveQuotes` · sqlparser 0.62.0

```rust
RemoveQuotes
```

Source: `src/ast/mod.rs:9449`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

REMOVEQUOTES

<a id="op-915f68233dab271251716a00"></a>
## RowGroupSize

`variant` · `sqlparser::ast::CopyLegacyOption::RowGroupSize` · sqlparser 0.62.0

```rust
RowGroupSize
```

Source: `src/ast/mod.rs:9451`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ROWGROUPSIZE \[ AS \] size \[ MB | GB \]

<a id="op-73a27e1e19d8154f9b5e181f"></a>
## StatUpdate

`variant` · `sqlparser::ast::CopyLegacyOption::StatUpdate` · sqlparser 0.62.0

```rust
StatUpdate
```

Source: `src/ast/mod.rs:9453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

STATUPDATE [ { ON | TRUE } | { OFF | FALSE } ]

<a id="op-b8eb49392a5108e51ae199b0"></a>
## TimeFormat

`variant` · `sqlparser::ast::CopyLegacyOption::TimeFormat` · sqlparser 0.62.0

```rust
TimeFormat
```

Source: `src/ast/mod.rs:9455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

TIMEFORMAT \[ AS \] {'timeformat_string' | 'auto' | 'epochsecs' | 'epochmillisecs' }

<a id="op-24f593c7848bd9351e0a2e83"></a>
## TruncateColumns

`variant` · `sqlparser::ast::CopyLegacyOption::TruncateColumns` · sqlparser 0.62.0

```rust
TruncateColumns
```

Source: `src/ast/mod.rs:9457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

TRUNCATECOLUMNS

<a id="op-19e76a99ae04e5f8fb7b03c3"></a>
## Zstd

`variant` · `sqlparser::ast::CopyLegacyOption::Zstd` · sqlparser 0.62.0

```rust
Zstd
```

Source: `src/ast/mod.rs:9459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ZSTD

<a id="op-dc8a050d66c0df1e21a97968"></a>
## clone

`function` · `sqlparser::ast::CopyLegacyOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CopyLegacyOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyOption", "path": "CopyLegacyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9375, 17], "end": [9375, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9375`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5eb4f8ba72ffdd7f54253bdf"></a>
## cmp

`function` · `sqlparser::ast::CopyLegacyOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CopyLegacyOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyOption", "path": "CopyLegacyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9375, 51], "end": [9375, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9375`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3251c09662d2e58f5395fd4"></a>
## deserialize

`function` · `sqlparser::ast::CopyLegacyOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyOption", "path": "CopyLegacyOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9376, 49], "end": [9376, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9376`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-412580e95a1d8a3240777f4c"></a>
## eq

`function` · `sqlparser::ast::CopyLegacyOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CopyLegacyOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyOption", "path": "CopyLegacyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9375, 24], "end": [9375, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9375`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2654362e1f9ab679e6dbe311"></a>
## fmt

`function` · `sqlparser::ast::CopyLegacyOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyOption", "path": "CopyLegacyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9375, 10], "end": [9375, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9375`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67ece27aabff81231ec93df4"></a>
## fmt

`function` · `sqlparser::ast::CopyLegacyOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyOption", "path": "CopyLegacyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9465, 1], "end": [9576, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9466`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec034aa4d83a292c8b0f2665"></a>
## hash

`function` · `sqlparser::ast::CopyLegacyOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyOption", "path": "CopyLegacyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9375, 56], "end": [9375, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9375`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-078145203bceb38e88ccb456"></a>
## partial_cmp

`function` · `sqlparser::ast::CopyLegacyOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CopyLegacyOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyOption", "path": "CopyLegacyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9375, 35], "end": [9375, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9375`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-789439cdb5cc6abc463a92cb"></a>
## serialize

`function` · `sqlparser::ast::CopyLegacyOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyOption", "path": "CopyLegacyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9376, 38], "end": [9376, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9376`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b07dda4430101e88ea50235d"></a>
## visit

`function` · `sqlparser::ast::CopyLegacyOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyOption", "path": "CopyLegacyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9377, 40], "end": [9377, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9377`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee1c54301f6a117c6b158892"></a>
## visit

`function` · `sqlparser::ast::CopyLegacyOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyOption", "path": "CopyLegacyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9377, 47], "end": [9377, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9377`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
