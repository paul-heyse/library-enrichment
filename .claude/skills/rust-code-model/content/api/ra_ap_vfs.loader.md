# `ra_ap_vfs::loader`

Crate `ra_ap_vfs` · 7 public items · structured records in [`model/ra_ap_vfs.loader.json`](../model/ra_ap_vfs.loader.json)

## Entry

`enum` · `ra_ap_vfs::loader::Entry`

```rust
enum Entry
```

**Variants**: `Files`, `Directories`

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn cargo_package_dependency(base: AbsPathBuf) -> Entry
fn contains_dir(&self, path: &AbsPath) -> bool
fn contains_file(&self, path: &AbsPath) -> bool
fn local_cargo_package(base: AbsPathBuf) -> Entry
fn rs_files_recursively(base: AbsPathBuf) -> Entry
```

A set of files on the file system.

---

## LoadingProgress

`enum` · `ra_ap_vfs::loader::LoadingProgress`

```rust
enum LoadingProgress
```

**Variants**: `Started`, `Progress`, `Finished`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## Message

`enum` · `ra_ap_vfs::loader::Message`

```rust
enum Message
```

**Variants**: `Progress`, `Loaded`, `Changed`

**Derives**: Debug

Message about an action taken by a [`Handle`].

---

## Config

`struct` · `ra_ap_vfs::loader::Config`

```rust
struct Config
```

**Fields**: `version`, `load`, `watch`

**Derives**: Debug

[`Handle`]'s configuration.

---

## Directories

`struct` · `ra_ap_vfs::loader::Directories`

```rust
struct Directories
```

**Fields**: `extensions`, `include`, `exclude`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn contains_dir(&self, path: &AbsPath) -> bool
fn contains_file(&self, path: &AbsPath) -> bool
```

Specifies a set of files on the file system.

A file is included if:
  * it has included extension
  * it is under an `include` path
  * it is not under `exclude` path

If many include/exclude paths match, the longest one wins.

If a path is in both `include` and `exclude`, the `exclude` one wins.

---

## Handle

`trait` · `ra_ap_vfs::loader::Handle`

```rust
trait Handle: fmt::Debug
```

**Methods** (4)

```rust
fn invalidate(&mut self, path: AbsPathBuf)
fn load_sync(&mut self, path: &AbsPath) -> Option<Vec<u8>>
fn set_config(&mut self, config: Config)
fn spawn(sender: Sender) -> Self where Self: Sized
```

Interface for reading and watching files.

---

## Sender

`type_alias` · `ra_ap_vfs::loader::Sender`

```rust
type Sender = crossbeam_channel::Sender<Message>
```

Type that will receive [`Messages`](Message) from a [`Handle`].

---
