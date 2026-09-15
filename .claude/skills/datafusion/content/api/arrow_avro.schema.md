# `arrow_avro::schema`

Crate `arrow-avro` · 15 public items · structured records in [`model/arrow_avro.schema.json`](../model/arrow_avro.schema.json)

## AVRO_DOC_METADATA_KEY

`constant` · `arrow_avro::schema::AVRO_DOC_METADATA_KEY`

```rust
const AVRO_DOC_METADATA_KEY: &str = "avro.doc"
```

Metadata key used to store the documentation for a type in an Avro schema.

---

## AVRO_ENUM_SYMBOLS_METADATA_KEY

`constant` · `arrow_avro::schema::AVRO_ENUM_SYMBOLS_METADATA_KEY`

```rust
const AVRO_ENUM_SYMBOLS_METADATA_KEY: &str = "avro.enum.symbols"
```

Metadata key used to represent Avro enum symbols in an Arrow schema.

---

## AVRO_FIELD_DEFAULT_METADATA_KEY

`constant` · `arrow_avro::schema::AVRO_FIELD_DEFAULT_METADATA_KEY`

```rust
const AVRO_FIELD_DEFAULT_METADATA_KEY: &str = "avro.field.default"
```

Metadata key used to store the default value of a field in an Avro schema.

---

## AVRO_NAMESPACE_METADATA_KEY

`constant` · `arrow_avro::schema::AVRO_NAMESPACE_METADATA_KEY`

```rust
const AVRO_NAMESPACE_METADATA_KEY: &str = "avro.namespace"
```

Metadata key used to store the name of a type in an Avro schema.

---

## AVRO_NAME_METADATA_KEY

`constant` · `arrow_avro::schema::AVRO_NAME_METADATA_KEY`

```rust
const AVRO_NAME_METADATA_KEY: &str = "avro.name"
```

Metadata key used to store the name of a type in an Avro schema.

---

## AVRO_ROOT_RECORD_DEFAULT_NAME

`constant` · `arrow_avro::schema::AVRO_ROOT_RECORD_DEFAULT_NAME`

```rust
const AVRO_ROOT_RECORD_DEFAULT_NAME: &str = "topLevelRecord"
```

Default name for the root record in an Avro schema.

---

## CONFLUENT_MAGIC

`constant` · `arrow_avro::schema::CONFLUENT_MAGIC`

```rust
const CONFLUENT_MAGIC: [u8; 1] = _
```

The Confluent "magic" byte (`0x00`)

---

## MAX_PREFIX_LEN

`constant` · `arrow_avro::schema::MAX_PREFIX_LEN`

```rust
const MAX_PREFIX_LEN: usize = 34
```

The maximum possible length of a prefix.
SHA256 (32) + single-object magic (2)

---

## SCHEMA_METADATA_KEY

`constant` · `arrow_avro::schema::SCHEMA_METADATA_KEY`

```rust
const SCHEMA_METADATA_KEY: &str = "avro.schema"
```

The metadata key used for storing the JSON encoded `Schema`

---

## SINGLE_OBJECT_MAGIC

`constant` · `arrow_avro::schema::SINGLE_OBJECT_MAGIC`

```rust
const SINGLE_OBJECT_MAGIC: [u8; 2] = _
```

The Avro single‑object encoding “magic” bytes (`0xC3 0x01`)

---

## Fingerprint

`enum` · `arrow_avro::schema::Fingerprint`

```rust
enum Fingerprint
```

**Variants**: `Rabin`, `Id`, `Id64`, `MD5`, `SHA256`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn load_fingerprint_id(id: u32) -> Self
fn load_fingerprint_id64(id: u64) -> Self
```

**via `core::convert::From`**

```rust
fn from(s: FingerprintAlgorithm) -> Self
fn from(s: &FingerprintStrategy) -> Self
fn from(s: FingerprintStrategy) -> Self
```

A schema fingerprint in one of the supported formats.

This is used as the key inside `SchemaStore` `HashMap`. Each `SchemaStore`
instance always stores only one variant, matching its configured
`FingerprintAlgorithm`, but the enum makes the API uniform.

<https://avro.apache.org/docs/1.11.1/specification/#schema-fingerprints>
<https://docs.confluent.io/platform/current/schema-registry/fundamentals/serdes-develop/index.html#wire-format>

---

## FingerprintAlgorithm

`enum` · `arrow_avro::schema::FingerprintAlgorithm`

```rust
enum FingerprintAlgorithm
```

**Variants**: `Rabin`, `Id`, `Id64`, `MD5`, `SHA256`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(s: &FingerprintStrategy) -> Self
fn from(s: FingerprintStrategy) -> Self
fn from(fp: &Fingerprint) -> Self
```

Supported fingerprint algorithms for Avro schema identification.
For use with Confluent Schema Registry IDs, set to None.

---

## FingerprintStrategy

`enum` · `arrow_avro::schema::FingerprintStrategy`

```rust
enum FingerprintStrategy
```

**Variants**: `Rabin`, `Id`, `Id64`, `MD5`, `SHA256`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(f: &Fingerprint) -> Self
fn from(f: FingerprintAlgorithm) -> Self
fn from(f: Fingerprint) -> Self
```

Defines the strategy for generating the per-record prefix for an Avro binary stream.

---

## AvroSchema

`struct` · `arrow_avro::schema::AvroSchema`

```rust
struct AvroSchema
```

**Fields**: `json_string`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn fingerprint(&self, hash_type: FingerprintAlgorithm) -> Result<Fingerprint, ArrowError>
fn new(json_string: String) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(schema: &ArrowSchema) -> Result<Self, Self::Error>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A wrapper for an Avro schema in its JSON string representation.

---

## SchemaStore

`struct` · `arrow_avro::schema::SchemaStore`

```rust
struct SchemaStore
```

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Debug, Default

**Methods** (6)

```rust
fn fingerprints(&self) -> Vec<Fingerprint>
fn lookup(&self, fingerprint: &Fingerprint) -> Option<&AvroSchema>
fn new() -> Self
fn new_with_type(fingerprint_algorithm: FingerprintAlgorithm) -> Self
fn register(&mut self, schema: AvroSchema) -> Result<Fingerprint, ArrowError>
fn set(&mut self, fingerprint: Fingerprint, schema: AvroSchema) -> Result<Fingerprint, ArrowError>
```

**via `core::convert::TryFrom`**

```rust
fn try_from(schemas: HashMap<Fingerprint, AvroSchema>) -> Result<Self, Self::Error>
```

An in-memory cache of Avro schemas, indexed by their fingerprint.

`SchemaStore` provides a mechanism to store and retrieve Avro schemas efficiently.
Each schema is associated with a unique [`Fingerprint`], which is generated based
on the schema's canonical form and a specific hashing algorithm.

A `SchemaStore` instance is configured to use a single [`FingerprintAlgorithm`] such as Rabin,
MD5 (not yet supported), or SHA256 (not yet supported) for all its operations.
This ensures consistency when generating fingerprints and looking up schemas.
All schemas registered will have their fingerprint computed with this algorithm, and
lookups must use a matching fingerprint.

# Examples

```no_run
// Create a new store with the default Rabin fingerprinting.
use arrow_avro::schema::{AvroSchema, SchemaStore};

let mut store = SchemaStore::new();
let schema = AvroSchema::new("\"string\"".to_string());
// Register the schema to get its fingerprint.
let fingerprint = store.register(schema.clone()).unwrap();
// Use the fingerprint to look up the schema.
let retrieved_schema = store.lookup(&fingerprint).cloned();
assert_eq!(retrieved_schema, Some(schema));
```

---
