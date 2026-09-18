# `datafusion_sql::unparser::dialect::IntervalStyle`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.dialect.IntervalStyle.json).

<a id="op-0ea38587f6a09aa42f018a69"></a>
## IntervalStyle

`enum` · `datafusion_sql::unparser::dialect::IntervalStyle` · datafusion-sql 55.1.0

```rust
enum IntervalStyle
```

Source: `src/unparser/dialect.rs:310`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

`IntervalStyle` to use for unparsing

<https://www.postgresql.org/docs/current/datatype-datetime.html#DATATYPE-INTERVAL-INPUT>
different DBMS follows different standards, popular ones are:
postgres_verbose: '2 years 15 months 100 weeks 99 hours 123456789 milliseconds' which is
compatible with arrow display format, as well as duckdb
sql standard format is '1-2' for year-month, or '1 10:10:10.123456' for day-time
<https://www.contrib.andrew.cmu.edu/~shadow/sql/sql1992.txt>

<a id="op-5c1502581677a140056ed30f"></a>
## MySQL

`variant` · `datafusion_sql::unparser::dialect::IntervalStyle::MySQL` · datafusion-sql 55.1.0

```rust
MySQL
```

Source: `src/unparser/dialect.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e613ac3792c9544080bd3bd2"></a>
## PostgresVerbose

`variant` · `datafusion_sql::unparser::dialect::IntervalStyle::PostgresVerbose` · datafusion-sql 55.1.0

```rust
PostgresVerbose
```

Source: `src/unparser/dialect.rs:311`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d9a093dc95ea61c7c4c1d2e"></a>
## SQLStandard

`variant` · `datafusion_sql::unparser::dialect::IntervalStyle::SQLStandard` · datafusion-sql 55.1.0

```rust
SQLStandard
```

Source: `src/unparser/dialect.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e2fe7f92083fcdfa930d593"></a>
## clone

`function` · `datafusion_sql::unparser::dialect::IntervalStyle::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> IntervalStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::IntervalStyle", "path": "IntervalStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 10], "end": [309, 15], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unparser/dialect.rs:309`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
