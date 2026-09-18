# `buoyant_kernel::scan::scan_row_schema`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.scan_row_schema.json).

<a id="op-e319333cbaa70c0f9bc8fb21"></a>
## scan_row_schema

`function` · `buoyant_kernel::scan::scan_row_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn scan_row_schema() -> schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L1273).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:1273`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the base schema that scan rows (from [`Scan::scan_metadata`](../operations/buoyant_kernel.scan.Scan.md#op-d91f731ebc1cdf3b0f6a19d8)) will be returned with.

This is the base shape; engines may add trailing `*_parsed` columns by opting in via
[`StatsOptions`](../operations/buoyant_kernel.scan.StatsOptions.md#op-df28d0423c58aeff877d5915) (`stats_parsed`) or [`PartitionValuesOptions`](../operations/buoyant_kernel.scan.PartitionValuesOptions.md#op-c151f7b66cbf5134df7c2604) (`partitionValues_parsed`).

It is:
```ignored
{
   path: string,
   size: long,
   modificationTime: long,
   stats: string,
   deletionVector: {
     storageType: string,
     pathOrInlineDv: string,
     offset: int,
     sizeInBytes: int,
     cardinality: long,
   },
   fileConstantValues: {
     partitionValues: map<string, string>,
     tags: map<string, string>,
     baseRowId: long,
     defaultRowCommitVersion: long,
     clusteringProvider: string,
   }
}
```
