# Crate map

12 crates hold the pinned surface. The facade crate re-exports most of it,
so the crate an item is *reached* through is usually not the crate that defines it.
Resolve access paths through [`../index/aliases.tsv`](../index/aliases.tsv) before
attributing an item to a crate.

The envelope column matters: a crate documented under a partial feature set has a
partial surface here, and [`../index/coverage.tsv`](../index/coverage.tsv) says which
features were off and why.

| Crate | Items | Traits | Envelope | Notable extension points |
|---|---:|---:|---|---|
| `buoyant_kernel` | 378 | 52 | 9 on / 7 off | `GetData`, `RowVisitor`, `SchemaTransform`, `ToSchema` |
| `buoyant_kernel_engine` | 18 | 2 | 2 on / 2 off | `FileOpener`, `TaskExecutor` |
| `deltalake-aws` | 56 | 0 | 2 on / 2 off | — |
| `deltalake-azure` | 2 | 0 | 2 on / 2 off | — |
| `deltalake-catalog-glue` | 2 | 0 | 2 on / 1 off | — |
| `deltalake-catalog-unity` | 60 | 1 | 8 on / 1 off | `TokenCredential` |
| `deltalake-core` | 261 | 27 | 9 on / 8 off | `LogStore`, `LogStoreFactory`, `ObjectStoreFactory`, `TryUpdateKey` |
| `deltalake-gcp` | 2 | 0 | 2 on / 2 off | — |
| `deltalake-hdfs` | 2 | 0 | 2 on / 2 off | — |
| `deltalake-lakefs` | 9 | 0 | 2 on / 2 off | — |
| `deltalake-mount` | 4 | 0 | 2 on / 2 off | — |
| `deltalake-opendal` | 10 | 1 | 4 on / 16 off | `OpendalAdapter` |

