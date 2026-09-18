# `parquet::file::metadata::footer_tail`

Crate `parquet` · 1 public items · structured records in [`model/parquet.file.metadata.footer_tail.json`](../model/parquet.file.metadata.footer_tail.json)

## FooterTail

`struct` · `parquet::file::metadata::footer_tail::FooterTail`

Also reachable as `parquet::file::metadata::FooterTail`

```rust
struct FooterTail
```

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn is_encrypted_footer(&self) -> bool
fn metadata_length(&self) -> usize
fn try_new(slice: &[u8; 8]) -> Result<FooterTail>
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: [u8; 8]) -> Result<Self>
fn try_from(value: &[u8]) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/parquet.file.metadata.footer_tail.FooterTail.md).


Parsed Parquet footer tail (last 8 bytes of a Parquet file)

There are 8 bytes at the end of the Parquet footer with the following layout:
* 4 bytes for the metadata length
* 4 bytes for the magic bytes 'PAR1' or 'PARE' (encrypted footer)

```text
+-----+------------------+
| len | 'PAR1' or 'PARE' |
+-----+------------------+
```

# Examples
```
# use parquet::file::metadata::FooterTail;
// a non encrypted footer with 28 bytes of metadata
let last_8_bytes: [u8; 8] = [0x1C, 0x00, 0x00, 0x00, b'P', b'A', b'R', b'1'];
let footer_tail = FooterTail::try_from(last_8_bytes).unwrap();
assert_eq!(footer_tail.metadata_length(), 28);
assert_eq!(footer_tail.is_encrypted_footer(), false);
```

```
# use parquet::file::metadata::FooterTail;
// an encrypted footer with 512 bytes of metadata
let last_8_bytes = vec![0x00, 0x02, 0x00, 0x00, b'P', b'A', b'R', b'E'];
let footer_tail = FooterTail::try_from(&last_8_bytes[..]).unwrap();
assert_eq!(footer_tail.metadata_length(), 512);
assert_eq!(footer_tail.is_encrypted_footer(), true);
```

---
