# `cargo_metadata::messages`

Crate `cargo_metadata` · 19 public items · structured records in [`model/cargo_metadata.messages.json`](../model/cargo_metadata.messages.json)

## ArtifactBuilderError

`enum` · `cargo_metadata::messages::ArtifactBuilderError`

```rust
enum ArtifactBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

Error type for ArtifactBuilder

---

## ArtifactDebuginfo

`enum` · `cargo_metadata::messages::ArtifactDebuginfo`

Also reachable as `cargo_metadata::ArtifactDebuginfo`

```rust
enum ArtifactDebuginfo
```

**Variants**: `None`, `LineDirectivesOnly`, `LineTablesOnly`, `Limited`, `Full`, `UnknownInt`, `UnknownString`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(d: D) -> Result<ArtifactDebuginfo, D::Error> where D: de::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ser::Serializer
```

The kind of debug information included in the artifact.

---

## ArtifactProfileBuilderError

`enum` · `cargo_metadata::messages::ArtifactProfileBuilderError`

```rust
enum ArtifactProfileBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

Error type for ArtifactProfileBuilder

---

## BuildFinishedBuilderError

`enum` · `cargo_metadata::messages::BuildFinishedBuilderError`

```rust
enum BuildFinishedBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

Error type for BuildFinishedBuilder

---

## BuildScriptBuilderError

`enum` · `cargo_metadata::messages::BuildScriptBuilderError`

```rust
enum BuildScriptBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

Error type for BuildScriptBuilder

---

## CompilerMessageBuilderError

`enum` · `cargo_metadata::messages::CompilerMessageBuilderError`

```rust
enum CompilerMessageBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

Error type for CompilerMessageBuilder

---

## Message

`enum` · `cargo_metadata::messages::Message`

Also reachable as `cargo_metadata::Message`

```rust
enum Message
```

**Variants**: `CompilerArtifact`, `CompilerMessage`, `BuildScriptExecuted`, `BuildFinished`, `TextLine`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn parse_stream<R: Read>(input: R) -> MessageIter<R>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A cargo message

---

## parse_messages

`function` · `cargo_metadata::messages::parse_messages`

> **Deprecated** — Use Message::parse_stream instead

Also reachable as `cargo_metadata::parse_messages`

```rust
fn parse_messages<R: Read>(input: R) -> serde_json::StreamDeserializer<'static, serde_json::de::IoRead<R>, Message>
```

Creates an iterator of Message from a Read outputting a stream of JSON
messages. For usage information, look at the top-level documentation.

---

## Artifact

`struct` · `cargo_metadata::messages::Artifact`

Also reachable as `cargo_metadata::Artifact`

```rust
struct Artifact
```

**Fields**: `package_id`, `manifest_path`, `target`, `profile`, `features`, `filenames`, `executable`, `fresh`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A compiler-generated file.

---

## ArtifactBuilder

`struct` · `cargo_metadata::messages::ArtifactBuilder`

Also reachable as `cargo_metadata::ArtifactBuilder`

```rust
struct ArtifactBuilder
```

**Derives**: Default

**Methods** (9)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<Artifact, ArtifactBuilderError>
fn executable<VALUE: ::derive_builder::export::core::convert::Into<Option<Utf8PathBuf>>>(self, value: VALUE) -> Self
fn features<VALUE: ::derive_builder::export::core::convert::Into<Vec<String>>>(self, value: VALUE) -> Self
fn filenames<VALUE: ::derive_builder::export::core::convert::Into<Vec<Utf8PathBuf>>>(self, value: VALUE) -> Self
fn fresh<VALUE: ::derive_builder::export::core::convert::Into<bool>>(self, value: VALUE) -> Self
fn manifest_path<VALUE: ::derive_builder::export::core::convert::Into<Utf8PathBuf>>(self, value: VALUE) -> Self
fn package_id<VALUE: ::derive_builder::export::core::convert::Into<PackageId>>(self, value: VALUE) -> Self
fn profile<VALUE: ::derive_builder::export::core::convert::Into<ArtifactProfile>>(self, value: VALUE) -> Self
fn target<VALUE: ::derive_builder::export::core::convert::Into<Target>>(self, value: VALUE) -> Self
```

Builder for [`Artifact`](struct.Artifact.html).

---

## ArtifactProfile

`struct` · `cargo_metadata::messages::ArtifactProfile`

Also reachable as `cargo_metadata::ArtifactProfile`

```rust
struct ArtifactProfile
```

**Fields**: `opt_level`, `debuginfo`, `debug_assertions`, `overflow_checks`, `test`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Profile settings used to determine which compiler flags to use for a
target.

---

## ArtifactProfileBuilder

`struct` · `cargo_metadata::messages::ArtifactProfileBuilder`

Also reachable as `cargo_metadata::ArtifactProfileBuilder`

```rust
struct ArtifactProfileBuilder
```

**Derives**: Default

**Methods** (6)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<ArtifactProfile, ArtifactProfileBuilderError>
fn debug_assertions<VALUE: ::derive_builder::export::core::convert::Into<bool>>(self, value: VALUE) -> Self
fn debuginfo<VALUE: ::derive_builder::export::core::convert::Into<ArtifactDebuginfo>>(self, value: VALUE) -> Self
fn opt_level<VALUE: ::derive_builder::export::core::convert::Into<String>>(self, value: VALUE) -> Self
fn overflow_checks<VALUE: ::derive_builder::export::core::convert::Into<bool>>(self, value: VALUE) -> Self
fn test<VALUE: ::derive_builder::export::core::convert::Into<bool>>(self, value: VALUE) -> Self
```

Builder for [`ArtifactProfile`](struct.ArtifactProfile.html).

---

## BuildFinished

`struct` · `cargo_metadata::messages::BuildFinished`

Also reachable as `cargo_metadata::BuildFinished`

```rust
struct BuildFinished
```

**Fields**: `success`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Final result of a build.

---

## BuildFinishedBuilder

`struct` · `cargo_metadata::messages::BuildFinishedBuilder`

Also reachable as `cargo_metadata::BuildFinishedBuilder`

```rust
struct BuildFinishedBuilder
```

**Derives**: Default

**Methods** (2)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<BuildFinished, BuildFinishedBuilderError>
fn success<VALUE: ::derive_builder::export::core::convert::Into<bool>>(self, value: VALUE) -> Self
```

Builder for [`BuildFinished`](struct.BuildFinished.html).

---

## BuildScript

`struct` · `cargo_metadata::messages::BuildScript`

Also reachable as `cargo_metadata::BuildScript`

```rust
struct BuildScript
```

**Fields**: `package_id`, `linked_libs`, `linked_paths`, `cfgs`, `env`, `out_dir`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Output of a build script execution.

---

## BuildScriptBuilder

`struct` · `cargo_metadata::messages::BuildScriptBuilder`

Also reachable as `cargo_metadata::BuildScriptBuilder`

```rust
struct BuildScriptBuilder
```

**Derives**: Default

**Methods** (7)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<BuildScript, BuildScriptBuilderError>
fn cfgs<VALUE: ::derive_builder::export::core::convert::Into<Vec<String>>>(self, value: VALUE) -> Self
fn env<VALUE: ::derive_builder::export::core::convert::Into<Vec<(String, String)>>>(self, value: VALUE) -> Self
fn linked_libs<VALUE: ::derive_builder::export::core::convert::Into<Vec<Utf8PathBuf>>>(self, value: VALUE) -> Self
fn linked_paths<VALUE: ::derive_builder::export::core::convert::Into<Vec<Utf8PathBuf>>>(self, value: VALUE) -> Self
fn out_dir<VALUE: ::derive_builder::export::core::convert::Into<Utf8PathBuf>>(self, value: VALUE) -> Self
fn package_id<VALUE: ::derive_builder::export::core::convert::Into<PackageId>>(self, value: VALUE) -> Self
```

Builder for [`BuildScript`](struct.BuildScript.html).

---

## CompilerMessage

`struct` · `cargo_metadata::messages::CompilerMessage`

Also reachable as `cargo_metadata::CompilerMessage`

```rust
struct CompilerMessage
```

**Fields**: `package_id`, `target`, `message`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Message left by the compiler

---

## CompilerMessageBuilder

`struct` · `cargo_metadata::messages::CompilerMessageBuilder`

Also reachable as `cargo_metadata::CompilerMessageBuilder`

```rust
struct CompilerMessageBuilder
```

**Derives**: Default

**Methods** (4)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<CompilerMessage, CompilerMessageBuilderError>
fn message<VALUE: ::derive_builder::export::core::convert::Into<Diagnostic>>(self, value: VALUE) -> Self
fn package_id<VALUE: ::derive_builder::export::core::convert::Into<PackageId>>(self, value: VALUE) -> Self
fn target<VALUE: ::derive_builder::export::core::convert::Into<Target>>(self, value: VALUE) -> Self
```

Builder for [`CompilerMessage`](struct.CompilerMessage.html).

---

## MessageIter

`struct` · `cargo_metadata::messages::MessageIter`

Also reachable as `cargo_metadata::MessageIter`

```rust
struct MessageIter<R>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

An iterator of Messages.

---
