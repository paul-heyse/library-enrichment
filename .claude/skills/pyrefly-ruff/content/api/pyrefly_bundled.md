# `pyrefly_bundled`

Crate `pyrefly_bundled` · 6 public items · structured records in [`model/pyrefly_bundled.json`](../model/pyrefly_bundled.json)

## BUNDLED_THIRD_PARTY_DIGEST

`constant` · `pyrefly_bundled::BUNDLED_THIRD_PARTY_DIGEST`

```rust
const BUNDLED_THIRD_PARTY_DIGEST: &[u8; 32] = b"\x0fK2Bf\x88\xec\xc9\xb6\x866\x93\xae/\x18i\xfb\x90\xe7\t\xafK!\x89\xfb\xb4\xd8\x9c\xba\x7f\xd6\x8c"
```

---

## BUNDLED_TYPESHED_DIGEST

`constant` · `pyrefly_bundled::BUNDLED_TYPESHED_DIGEST`

```rust
const BUNDLED_TYPESHED_DIGEST: &[u8; 32] = b"@\xd5\xd3\x06\xcf\xc4^\xca\xc7\xb5y\xbdA\x7f\xc8\xcb.SIwh\x95G\xb3\xd2_GG\xdb\xdb\x18S"
```

---

## BUNDLED_TYPESHED_THIRD_PARTY_DIGEST

`constant` · `pyrefly_bundled::BUNDLED_TYPESHED_THIRD_PARTY_DIGEST`

```rust
const BUNDLED_TYPESHED_THIRD_PARTY_DIGEST: &[u8; 32] = b"\xe97=7}\x8f\xda\x0b\xd1\x96\x82:=\x16\xf5\xb4\x884\xc7\xb9Q<\x87\xebPs\xd8\xbe+\xa1R^"
```

---

## bundled_third_party

`function` · `pyrefly_bundled::bundled_third_party`

```rust
fn bundled_third_party() -> anyhow::Result<starlark_map::small_map::SmallMap<std::path::PathBuf, String>>
```

Extract third-party stubs from the bundled archive.
These are stubs that are not included in typeshed (e.g., pandas-stubs, boto3-stubs).

---

## bundled_third_party_stubs

`function` · `pyrefly_bundled::bundled_third_party_stubs`

```rust
fn bundled_third_party_stubs() -> anyhow::Result<(starlark_map::small_map::SmallMap<std::path::PathBuf, String>, starlark_map::small_map::SmallMap<std::path::PathBuf, String>)>
```

---

## bundled_typeshed

`function` · `pyrefly_bundled::bundled_typeshed`

```rust
fn bundled_typeshed() -> anyhow::Result<starlark_map::small_map::SmallMap<std::path::PathBuf, String>>
```

---
