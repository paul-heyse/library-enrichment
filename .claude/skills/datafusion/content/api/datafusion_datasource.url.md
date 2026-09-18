# `datafusion_datasource::url`

Crate `datafusion-datasource` · 1 public items · structured records in [`model/datafusion_datasource.url.json`](../model/datafusion_datasource.url.json)

## ListingTableUrl

`struct` · `datafusion_datasource::url::ListingTableUrl`

Also reachable as `datafusion::datasource::listing::ListingTableUrl`, `datafusion_datasource::ListingTableUrl`

```rust
struct ListingTableUrl
```

**Implements**: `core::convert::AsRef`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (18)

```rust
fn as_str(&self) -> &str
fn contains(&self, path: &Path, ignore_subdirectory: bool) -> bool
fn file_extension(&self) -> Option<&str>
fn get_glob(&self) -> &Option<Pattern>
fn get_table_ref(&self) -> &Option<TableReference>
fn get_url(&self) -> &Url
fn is_collection(&self) -> bool
fn is_folder(&self) -> bool
async fn list_all_files<'a>(&'a self, ctx: &'a dyn Session, store: &'a dyn ObjectStore, file_extension: &'a str) -> Result<BoxStream<'a, Result<ObjectMeta>>>
async fn list_prefixed_files<'a>(&'a self, ctx: &'a dyn Session, store: &'a dyn ObjectStore, prefix: Option<Path>, file_extension: &'a str) -> Result<BoxStream<'a, Result<ObjectMeta>>>
fn object_store(&self) -> ObjectStoreUrl
fn parse(s: impl AsRef<str>) -> Result<Self>
fn prefix(&self) -> &Path
fn scheme(&self) -> &str
fn strip_prefix<'a, 'b: 'a>(&'a self, path: &'b Path) -> Option<impl Iterator<Item = &'b str> + 'a>
fn try_new(url: Url, glob: Option<Pattern>) -> Result<Self>
fn with_glob(self, glob: &str) -> Result<Self>
fn with_table_ref(self, table_ref: TableReference) -> Self
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
fn as_ref(&self) -> &Url
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.url.ListingTableUrl.md).


A parsed URL identifying files for a listing table, see [`ListingTableUrl::parse`]
for more information on the supported expressions

---
