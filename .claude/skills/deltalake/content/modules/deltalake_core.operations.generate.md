# `deltalake_core::operations::generate`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.generate.json).

<a id="op-50bb2f2c378858dc3dac9367"></a>
## generate

`module` · `deltalake_core::operations::generate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod generate
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/generate.rs#L1).

Source: `crates/core/src/operations/generate.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).


The generate supports the fairly simple "GENERATE" operation which produces a
[symlink_format_manifest](https://docs.delta.io/delta-utility/#generate-a-manifest-file) file
when needed for an external engine such as Presto or BigQuery.

The "symlink_format_manifest" is not something that has been well documented, but for
enon-partitioned tables this will generate a `_symlink_format_manifest/manifest` file next to
the `_delta_log`, for example:

```ignore
COVID-19_NYT
├── _delta_log
│   ├── 00000000000000000000.crc
│   └── 00000000000000000000.json
├── part-00000-a496f40c-e091-413a-85f9-b1b69d4b3b4e-c000.snappy.parquet
├── part-00001-9d9d980b-c500-4f0b-bb96-771a515fbccc-c000.snappy.parquet
├── part-00002-8826af84-73bd-49a6-a4b9-e39ffed9c15a-c000.snappy.parquet
├── part-00003-539aff30-2349-4b0d-9726-c18630c6ad90-c000.snappy.parquet
├── part-00004-1bb9c3e3-c5b0-4d60-8420-23261f58a5eb-c000.snappy.parquet
├── part-00005-4d47f8ff-94db-4d32-806c-781a1cf123d2-c000.snappy.parquet
├── part-00006-d0ec7722-b30c-4e1c-92cd-b4fe8d3bb954-c000.snappy.parquet
├── part-00007-4582392f-9fc2-41b0-ba97-a74b3afc8239-c000.snappy.parquet
└── _symlink_format_manifest
    └── manifest
```

For partitioned tables, a `manifest` file will be generated inside a hive-style partitioned
tree structure, e.g.:

```ignore
delta-0.8.0-partitioned
├── _delta_log
│   └── 00000000000000000000.json
├── _symlink_format_manifest
│   ├── year=2020
│   │   ├── month=1
│   │   │   └── day=1
│   │   │       └── manifest
│   │   └── month=2
│   │       ├── day=3
│   │       │   └── manifest
│   │       └── day=5
│   │           └── manifest
│   └── year=2021
│       ├── month=12
│       │   ├── day=20
│       │   │   └── manifest
│       │   └── day=4
│       │       └── manifest
│       └── month=4
│           └── day=5
│               └── manifest
├── year=2020
│   ├── month=1
│   │   └── day=1
│   │       └── part-00000-8eafa330-3be9-4a39-ad78-fd13c2027c7e.c000.snappy.parquet
│   └── month=2
│       ├── day=3
│       │   └── part-00000-94d16827-f2fd-42cd-a060-f67ccc63ced9.c000.snappy.parquet
│       └── day=5
│           └── part-00000-89cdd4c8-2af7-4add-8ea3-3990b2f027b5.c000.snappy.parquet
└── year=2021
    ├── month=12
    │   ├── day=20
    │   │   └── part-00000-9275fdf4-3961-4184-baa0-1c8a2bb98104.c000.snappy.parquet
    │   └── day=4
    │       └── part-00000-6dc763c0-3e8b-4d52-b19e-1f92af3fbb25.c000.snappy.parquet
    └── month=4
        └── day=5
            └── part-00000-c5856301-3439-4032-a6fc-22b7bc92bebb.c000.snappy.parquet
```
