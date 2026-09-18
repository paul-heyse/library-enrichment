# `buoyant_kernel::partition::hive`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.partition.hive.json).

<a id="op-03928925eb47795aa47c4767"></a>
## hive

`module` · `buoyant_kernel::partition::hive` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod hive
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/partition/hive.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/partition/hive.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Hive-style partition path encoding.

Percent-encodes partition column names and values for filesystem directory paths
(e.g., `region=US%2FEast/year=2024/`). Matches Hive's [`FileUtils.escapePathName`][hive]
and Spark's [`ExternalCatalogUtils.escapePathName`][spark].

```text
Step 2 (serialization):  Scalar::String("US/East")  -->  "US/East"     (partitionValues)
Step 3 (THIS MODULE):    "US/East"                  -->  "US%2FEast"   (directory name)
```

# Encoding rules

Encodes: ASCII control chars (0x00-0x1F), DEL (0x7F),
`"` `#` `%` `'` `*` `/` `:` `=` `?` `\` `{` `[` `]` `^`.

NOT encoded: space (0x20), non-ASCII (>= 0x80), `}`.

On Windows, space (0x20), `<`, `>`, and `|` are additionally encoded to match
Spark's platform-specific behavior in `ExternalCatalogUtils.escapePathName`.

See the encoding tables in the [`super`](../modules/buoyant_kernel.partition.md#op-c3f0fbf14b67d59e9094e867) module for comprehensive examples.

[hive]: https://github.com/apache/hive/blob/trunk/common/src/java/org/apache/hadoop/hive/common/FileUtils.java
[spark]: https://github.com/apache/spark/blob/master/sql/catalyst/src/main/scala/org/apache/spark/sql/catalyst/catalog/ExternalCatalogUtils.scala
