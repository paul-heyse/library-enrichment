# Pydantic 2.13.5 — comprehensive advanced technical reference

**Reference date:** 2026-09-11  
**Stable package anchor:** `pydantic==2.13.5`  
**Core runtime anchor:** `pydantic-core==2.46.5`  
**Primary intent:** a dense, agent-oriented capability manual: every major Pydantic surface should be locatable, understood in terms of value and constraints, and usable without rediscovering the architecture from source.

This reference deliberately follows the structural conventions of the companion DataFusion/Arrow advanced reference: a version-pinned front matter, a complete capability inventory, independent deep-dive chapters, canonical syntax, decision rules, production guidance, anti-patterns, test gates, and a final source-verified reconciliation layer.

---

## Version / source anchors

Pydantic **2.13.5** is the latest stable release as of this reference date; it was released on **2026-08-28**. The `v2.13.5` package metadata requires Python **>=3.9** and pins `pydantic-core==2.46.5`. The latest stable documentation landing page still identifies its rendered documentation as **v2.13.4**, so exact release/dependency facts in this manual come from the 2.13.5 tag/PyPI metadata, while stable public API semantics come from the stable docs unless a 2.13.5 source check is specifically needed. [FM-1] [FM-2] [FM-3]

### Source precedence

Use this order when facts conflict or drift:

1. **Released `v2.13.5` tag / PyPI metadata** — package version, Python floor, exact `pydantic-core`, dependency graph, shipped source.
2. **Stable Pydantic docs** — public concepts, APIs, examples, supported semantics. The docs currently render as v2.13.4; 2.13.5 is a patch-only follow-up.
3. **Release notes / changelog for 2.13.x** — version-specific behavioral changes and migration risks.
4. **`main` / prerelease 2.14 material** — watchlist only; never silently treat as stable 2.13.5 behavior.
5. **Implementation internals** — diagnostic/extension evidence, not a stability promise unless explicitly documented.

**Agent invariant:** never generate deployable 2.13.5 code by copying an API that appears only in 2.14 prerelease/main without labeling and version-gating it.

---

## Canonical environment for this reference

| Layer | Pinned / current target | Compatibility note |
|---|---:|---|
| `pydantic` | **2.13.5** | Latest stable; released 2026-08-28 |
| `pydantic-core` | **2.46.5** | Exact dependency in Pydantic 2.13.5 |
| Python | **>=3.9** | 2.13.5 advertises 3.9–3.14; 2.14 prerelease drops 3.9 |
| `typing-extensions` | **>=4.14.1** | Direct Pydantic dependency |
| `annotated-types` | **>=0.6.0** | Direct Pydantic dependency; preferred metadata constraints |
| `typing-inspection` | **>=0.4.2** | Direct Pydantic dependency in the 2.13.5 tag |
| `pydantic-settings` | **2.15.0** | Latest settings package on PyPI; Python >=3.10; separately versioned |
| `pydantic-extra-types` | **2.11.1** | Latest non-yanked stable; 2.11.2 is yanked |
| `pydantic.v1` namespace | **1.10.26-compatible** | Shipped inside Pydantic 2.13; migration bridge, not preferred new API |

Canonical installation for a fully featured application:

```bash
uv add 'pydantic[email,timezone]==2.13.5' \
       'pydantic-settings==2.15.0' \
       'pydantic-extra-types==2.11.1'
```

Minimal application:

```bash
uv add 'pydantic==2.13.5'
```

`pydantic[email]` activates `email-validator`; the `timezone` extra supplies `tzdata` on Windows when needed. [FM-2]

---

## 2.13.5 patch ledger

The 2.13.5 release is a small but important stability patch. Its release notes list four fixes: validator reuse when plugins are configured; two `pydantic-core` garbage-collection traversal fixes; and a smart-union fix so validated model fields are counted once. [FM-1]

| Change | Why it matters | Regression / test action |
|---|---|---|
| Validator reuse with plugins | Instrumented/plugin-enabled validation can reuse validators correctly | rerun plugin/observability validation tests |
| GC traversal fixes in core structs | Prevents hidden lifetime/reference issues in core objects | rerun long-lived process / GC-sensitive suites if you embed low-level core objects |
| `GeneralFieldsSerializer` GC traversal | Serializer object graph is correctly visible to GC | rerun serializer lifecycle tests if custom core integration is used |
| Smart-union validated-field count | Corrects model-union branch scoring | rerun ambiguous/smart-union selection tests |

**Upgrade stance:** a 2.13.4 → 2.13.5 upgrade should normally be low risk, but applications depending on ambiguous smart-union selection should treat the final fix as behaviorally observable.

---

## Net 2.13 capability additions that matter

Pydantic 2.13 is not merely a patch line. The 2.13 program added or refined several agent-relevant features: **polymorphic serialization** for Pydantic model/dataclass subclasses, `exclude_if` on computed fields, `StringConstraints(ascii_only=True)`, validated-data-aware private attribute factories, `model_fields_set` tracking of extra fields added after initialization, union/serialization fixes, and an updated `pydantic.v1` namespace based on 1.10.26. [FM-4] [FM-5]

Highest-leverage implications:

* **Serialization policy is now more explicit:** choose safe annotation-based serialization, 2.13 polymorphic subclass serialization, or full “serialize as any” behavior deliberately.
* **Generated/derived fields can be conditional:** `@computed_field(exclude_if=...)` makes output policy more expressive without post-processing dicts.
* **String contracts can express ASCII-only requirements** through `StringConstraints`.
* **Smart-union behavior deserves explicit tests:** selection is algorithmic rather than a trivial first-match contract.
* **Core and V1 compatibility live in one repository/package line:** `pydantic-core` source was merged into the main repository in the 2.13 development cycle, while runtime remains a separate package dependency.

---

## 2.14 prerelease watch — do not treat as stable 2.13.5

As of 2026-09-11, **2.14.0b2** is a prerelease tagged on **2026-09-09**. The 2.14 line drops Python 3.9, adds Python 3.15-oriented work, and is developing/stabilizing features such as `frozendict`, `TypeForm`, lazy-import handling, deque/core-schema updates, temporal JSON Schema changes, and a non-experimental `MISSING` sentinel. The first 2.14 alpha explicitly dropped Python 3.9. [FM-6] [FM-7]

Rules for this manual:

```text
2.13.5 stable feature     → document as deployable
2.13 experimental feature → document under Experimental, with warnings
2.14 prerelease feature   → document only in the watch section / version notes
main-only feature         → do not recommend without an explicit source-version gate
```

---

## Capability inventory: what this reference covers

The public Pydantic surface naturally decomposes into these capability planes:

```text
Schema authoring
  BaseModel / RootModel / dataclasses / TypedDict / type aliases / generics

Field semantics
  required/default/nullability / constraints / aliases / metadata / computed fields

Validation
  Python / JSON / strings modes / strictness / conversion / validators / context

Serialization
  Python / JSON output / field+model serializers / include/exclude / subclass policy

Schema publication
  JSON Schema / OpenAPI 3.1 / validation-vs-serialization schema / custom generators

Configuration
  ConfigDict / base-model policy / model/dataclass/TypedDict/TypeAdapter/validate_call config

Arbitrary-type validation
  TypeAdapter / Annotated metadata / functional validators+serializers

Custom extension
  custom types / GetPydanticSchema / __get_pydantic_core_schema__ / core_schema

Runtime engine
  pydantic-core / SchemaValidator / SchemaSerializer / core schemas

Operations
  errors / performance / settings / secrets / deployment / testing / static tooling

Experimental
  pipeline API / partial validation / callable argument schemas / MISSING sentinel
```

The stable docs navigation itself separates concepts, public API, Pydantic Core, Settings, Extra Types, internals, examples, errors, and integrations; this reference keeps that full breadth while grouping it for agent retrieval rather than mirroring website navigation literally. [FM-3]

---

# Proposed comprehensive documentation map

## Part I — Core validation and model contracts

0. Scope, versioning, mental model, V1/V2 boundary  
1. Installation, dependency/package layout, deployment surface  
2. `BaseModel` core API  
3. Field system and `Field(...)`  
4. Validation entrypoints and input modes  
5. Type system and coercion rules  
6. Strict mode and coercion policy  
7. Field/model validators and validation context  
8. Serialization and dumping

## Part II — External shape, configuration, schemas, and extensibility

9. Aliases and external data shape  
10. `ConfigDict` configuration policy  
11. JSON Schema/OpenAPI contracts  
12. `TypeAdapter`  
13. Dataclasses, `TypedDict`, model-like types  
14. Custom types and reusable constraints  
15. `pydantic-core` and internal architecture  
16. Error model and diagnostics

## Part III — Runtime boundaries, performance, models, and deployment recipes

17. `pydantic-settings` foundations  
18. Performance engineering  
19. Advanced model features  
20. Unions/polymorphism/discriminators  
21. `@validate_call`  
22. Files/web/API/queues/database boundary recipes

## Part IV — Production hardening and source-verified capability indexes

23. Security, secrets, and sensitive-data handling  
24. Ecosystem integrations and developer tooling  
25. Experimental surfaces  
26. Plugin/instrumentation surface  
27. Migration and version-upgrade engineering  
28. Testing and QA strategy  
29. Architectural patterns and agent design rules  
30. Standard-library type capability index  
31. Pydantic/network/special type capability index  
32. Dynamic models, generics, recursive types, annotation resolution  
33. Serialization policy reconciliation for 2.13  
34. Complete `ConfigDict` option map  
35. JSON Schema contract governance  
36. `pydantic-settings` 2.15 advanced guide  
37. `pydantic-extra-types` 2.11.1 catalog  
38. Low-level core-schema extension cookbook  
39. Production deployment/performance/observability checklist  
40. Pydantic 2.13.5 source-verified capability reconciliation + 2.14 watch

Appendices: API quick index, deprecation/V3 watch, source map, agent implementation checklist.

---

## Stable upgrade / verification gate

```text
[ ] pydantic == 2.13.5 in the target environment
[ ] pydantic-core == 2.46.5 resolved exactly
[ ] Python >= 3.9 for 2.13.5; do not assume 3.9 support for 2.14+
[ ] typing-extensions >= 4.14.1
[ ] annotated-types >= 0.6.0
[ ] typing-inspection >= 0.4.2
[ ] pydantic-settings >= 2.14.2 if NestedSecretsSettingsSource is used
[ ] no new code imports pydantic.v1 unless migration is intentional
[ ] no public API depends on deprecated V1 method names
[ ] smart-union branch selection covered by tests where ambiguous
[ ] serialization tests cover base-vs-subclass sensitive-field behavior
[ ] JSON Schema tests pin semantics rather than unstable $ref formatting where possible
[ ] ValidationError tests use error type codes rather than exact prose
[ ] TypeAdapter instances reused in hot paths
[ ] experimental APIs isolated behind explicit imports/version policy
[ ] custom core-schema hooks tested against the pinned minor line
```

---

[FM-1]: https://github.com/pydantic/pydantic/releases/tag/v2.13.5 "Pydantic v2.13.5 release"
[FM-2]: https://raw.githubusercontent.com/pydantic/pydantic/v2.13.5/pyproject.toml "Pydantic 2.13.5 project metadata"
[FM-3]: https://docs.pydantic.dev/latest/ "Pydantic stable documentation"
[FM-4]: https://github.com/pydantic/pydantic/releases/tag/v2.13.0 "Pydantic v2.13.0 release"
[FM-5]: https://docs.pydantic.dev/latest/concepts/serialization/ "Pydantic serialization"
[FM-6]: https://github.com/pydantic/pydantic/releases/tag/v2.14.0b2 "Pydantic v2.14.0b2 prerelease"
[FM-7]: https://github.com/pydantic/pydantic/releases/tag/v2.14.0a1 "Pydantic v2.14.0a1 prerelease"

---
# Part I — Core validation and model contracts

# Pydantic Advanced — 0) Pydantic scope, versioning, and mental model — agent-ready deep dive

Style target follows your Cyclopts advanced reference structure. 

Release anchor for this reference: **Pydantic 2.13.5** is the latest stable package release (2026-08-28). The public “latest” documentation site still renders itself as **v2.13.4**, so this reference uses the 2.13.5 release/tag metadata for exact version and dependency facts and the 2.13.4 stable docs for public API semantics where 2.13.5 contains only patch fixes. ([Pydantic Docs][S00-1])

---

## 0.1 Scope contract: what Pydantic is

### Primary object model

```python
from pydantic import BaseModel

class User(BaseModel):
    id: int
    name: str = "John Doe"
```

**Canonical definition:** Pydantic models are classes inheriting from `BaseModel`; fields are annotated class attributes; Pydantic treats models as schema definitions for validated objects, API endpoint contracts, serialization units, and JSON Schema emitters. ([Pydantic Docs][S00-2])

### Capability envelope

Pydantic should be treated as:

```text
external/untrusted input
  → schema-directed validation/coercion
  → typed Python object
  → controlled serialization
  → optional JSON Schema contract
```

Primary validated-object surfaces:

```python
BaseModel              # structured record type
RootModel[T]           # root-wrapped arbitrary type
TypeAdapter[T]         # validation/serialization for any type, no model class required
pydantic.dataclasses   # dataclass-style validated objects
BaseSettings           # config/env/secrets model, separate package
```

Model classes expose validation, serialization, copy, schema, rebuild, and introspection methods including `model_validate()`, `model_validate_json()`, `model_construct()`, `model_dump()`, `model_dump_json()`, `model_copy()`, `model_json_schema()`, `model_fields`, `model_fields_set`, and `model_rebuild()`. ([Pydantic Docs][S00-2])

---

## 0.2 Version stance: target v2, isolate v1

### Install / pinning

```bash
pip install -U pydantic
```

Pydantic v2 is the current production release; v1 remains available, but official guidance recommends migration to v2 for improvements and new features. ([Pydantic Docs][S00-3])

### Migration bridge

```python
# V2 API
from pydantic import BaseModel

# V1 compatibility API inside V2 package
from pydantic.v1 import BaseModel as V1BaseModel
```

The v2 package exposes the v1 API through `pydantic.v1`; this exists for staged migration, not as a preferred long-term architecture. The `.v1` namespace is also available in `pydantic>=1.10.17`, enabling a compatibility import strategy across v1/v2 environments. ([Pydantic Docs][S00-3])

### Agent rule

```text
New code:        use pydantic v2 API only.
Migrating code:  isolate v1 imports behind pydantic.v1.
Library code:    do not mix v1/v2 models in generic parameters or nested model graphs.
```

Mixing v1 and v2 models is not supported for v2 generic model type parameters. ([Pydantic Docs][S00-3])

---

## 0.3 Pydantic v2 version-policy implications

### Stability rules

Pydantic v2 policy:

```text
minor v2 releases: no intentional breaking changes
deprecated features: not removed until v3
experimental features: unstable; may change or disappear
```

Pydantic explicitly says deprecated functionality will not be removed until v3, while experimental features can change during patch/minor releases or be removed with little notice. ([Pydantic Docs][S00-4])

### Contract-testing implication

Do **not** golden-test these as stable:

```text
ValidationError.msg
ValidationError.ctx
ValidationError.loc exact formatting
JSON Schema $ref formatting
__repr__ output
raw __pydantic_core_schema__ contents
```

Pydantic’s version policy says `ValidationError.type` is the stable programmatic discriminator, while `msg`, `ctx`, `loc`, JSON Schema reference formatting, new error keys, and core-schema contents may change in minor releases. ([Pydantic Docs][S00-4])

Recommended test assertion:

```python
try:
    Model.model_validate(payload)
except ValidationError as exc:
    errors = exc.errors()
    assert errors[0]["type"] == "int_parsing"   # stable-ish discriminator
    assert errors[0]["loc"] == ("id",)          # useful, but less stable than type
```

---

## 0.4 Core mental model: annotation → core schema → runtime engines

### Pipeline

```text
Python class body / type annotation
  → pydantic metaclass / schema generator
  → core schema
  → pydantic-core SchemaValidator
  → validated Python object

validated Python object
  → pydantic-core SchemaSerializer
  → Python-mode dump / JSON-mode dump

core schema
  → GenerateJsonSchema
  → JSON Schema / OpenAPI-facing contract
```

Pydantic v2 splits responsibilities: model definition is handled in the Python `pydantic` package; validation and serialization are handled in `pydantic-core`; communication between the two uses a structured, serializable “core schema” dictionary. ([Pydantic Docs][S00-5])

### Class-definition phase

```python
class Order(BaseModel):
    id: int
    total_cents: int
```

At class definition, Pydantic collects:

```text
annotations → fields
model_config
validators
serializers
private attributes
class variables
generic parametrization
```

Those collected facts become a core schema; for models, that schema is stored on `__pydantic_core_schema__`. ([Pydantic Docs][S00-5])

### Runtime validation phase

```python
order = Order.model_validate({"id": "1", "total_cents": 2500})
```

Instance validation is performed by `pydantic-core` using the prebuilt core schema. Internally, model input is sent to `SchemaValidator.validate_python`, then Pydantic populates the model instance. ([Pydantic Docs][S00-5])

### Runtime serialization phase

```python
order.model_dump()
order.model_dump_json()
```

Serialization is also handled by `pydantic-core` using `SchemaSerializer`; `model_dump()` returns Python structures, and `model_dump_json()` emits JSON. ([Pydantic Docs][S00-5])

### JSON Schema phase

```python
schema = Order.model_json_schema()
```

JSON Schema generation uses the core schema as input; Pydantic routes it through `GenerateJsonSchema`. ([Pydantic Docs][S00-5])

---

## 0.5 Core schema: what agents should and should not touch

### Useful mental representation

```python
class Model(BaseModel):
    foo: bool = Field(strict=True)
```

Conceptually, field schema resembles:

```python
{
    "type": "bool",
    "strict": True,
}
```

The architecture docs show this exact kind of core-schema representation for a strict boolean field. ([Pydantic Docs][S00-5])

### Agent rules

```text
Read core schema:       debugging, advanced custom-type work, performance reasoning.
Depend on shape:        avoid.
Mutate core schema:     avoid unless implementing documented custom hooks.
Test exact schema dict: avoid.
```

Pydantic states that custom core schema definitions are not generally user-definable because `pydantic-core` supports a fixed set of core schema types; the low-level format is not a stable public contract. ([Pydantic Docs][S00-5])

### Approved customization seams

```python
class MyType:
    @classmethod
    def __get_pydantic_core_schema__(cls, source, handler):
        ...

    @classmethod
    def __get_pydantic_json_schema__(cls, core_schema, handler):
        ...
```

Use these hooks only for custom type integration, library-level reusable types, or advanced schema/validation wrappers. Pydantic documents these as wrapper-pattern extension points around generated schemas. ([Pydantic Docs][S00-5])

---

## 0.6 What “validation” means in Pydantic

### Exact semantic stance

```text
Pydantic validates output shape, not input purity.
```

Pydantic defines validation as instantiating a model or other type that adheres to declared types and constraints. It guarantees the output’s types and constraints, not that the input was already valid; if input cannot be parsed into the target object, Pydantic raises `ValidationError`. ([Pydantic Docs][S00-2])

### Practical implication

```python
class User(BaseModel):
    id: int

User.model_validate({"id": "123"}).id
# int: 123
```

In lax mode, parsing/coercion is part of validation. Input may be `str`, output may be `int`.

### Validation failure model

```python
from pydantic import ValidationError

try:
    User.model_validate({"id": "not an int"})
except ValidationError as exc:
    print(exc.errors())
```

Pydantic raises one `ValidationError` containing all discovered validation errors, rather than one exception per field. ([Pydantic Docs][S00-2])

### Agent decision rule

```text
Use Pydantic when the desired result is:
  "produce a typed object or fail with structured errors."

Do not use Pydantic when the desired result is:
  "prove the upstream system sent semantically trustworthy data."
```

Pydantic can enforce schema-level constraints; it cannot prove business truth, database consistency, authorization, or temporal validity unless those checks are explicitly encoded.

---

## 0.7 Validation entrypoint taxonomy

### Model construction

```python
user = User(id="123")
```

Use for ergonomic internal creation when keyword arguments are already a normal Python dict-like boundary.

### Python-object validation

```python
user = User.model_validate({"id": "123"})
```

Use for dicts, ORM-like objects with `from_attributes=True`, decoded JSON, queue payloads, DB rows, or any Python object.

### JSON validation

```python
user = User.model_validate_json(b'{"id": "123"}')
```

Use when the raw input is JSON bytes/string and you want Pydantic’s JSON-mode parser/validator path.

### Stringly-typed validation

```python
user = User.model_validate_strings({"id": "123"})
```

Use for env vars, form-like values, CLI-derived maps, headers, query params. The models docs demonstrate `model_validate`, `model_validate_json`, and `model_validate_strings` as distinct validation surfaces. ([Pydantic Docs][S00-2])

---

## 0.8 Lax vs strict mode: validation policy boundary

### Lax mode

```python
class M(BaseModel):
    x: int

M.model_validate({"x": "1"})  # x=1
```

Default mode prioritizes usability: parse when safe/defined.

### Strict mode

```python
M.model_validate({"x": "1"}, strict=True)  # likely ValidationError
```

Strict mode reduces coercion. Pydantic’s top-level docs identify strict/lax mode as a core feature: strict mode avoids conversion; lax mode attempts coercion where appropriate. ([Pydantic Docs][S00-1])

### Agent deployment rule

```text
Public HTTP JSON body:         lax or selectively strict; preserve API ergonomics.
Security-sensitive boundary:   strict=True or Field(strict=True).
Internal domain object:        strict/frozen when possible.
Settings/env vars:             usually lax/stringly, because env values are strings.
LLM output parsing:             lax at ingestion, strict at post-normalization layer if needed.
```

---

## 0.9 Pydantic’s three-layer architecture

## Layer A — `pydantic`: Python authoring API

Owns:

```text
BaseModel
Field
ConfigDict
validators
serializers
TypeAdapter
RootModel
dataclasses integration
JSON Schema API
public error class exports
```

Value case:

```text
agent-readable schema code
static-analysis friendliness
IDE friendliness
concise constraints
runtime validation without hand parsers
serialization/schema generation from same source
```

Pydantic’s public docs emphasize type hints as the driver for validation and serialization, with IDE/static analysis integration as a key value proposition. ([Pydantic Docs][S00-1])

---

## Layer B — `pydantic-core`: Rust-backed execution engine

Owns:

```text
SchemaValidator
SchemaSerializer
ValidationError internals
core-schema execution
to_python / to_json mechanics
URL internals
low-level custom errors
```

Pydantic v2 moved validation and serialization execution into `pydantic-core`, a Rust-backed package introduced to improve performance, with less customizability of internal logic. ([Pydantic Docs][S00-5])

Agent rule:

```text
Default app code: do not import pydantic_core directly.
Library/custom-type code: import only when implementing custom core-schema hooks or low-level errors.
```

---

## Layer C — ecosystem packages

### `pydantic-settings`

Install:

```bash
pip install pydantic-settings
```

Use:

```python
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="APP_")
    database_url: str
```

`pydantic-settings` provides optional settings/config features for loading from environment variables or secrets files; `BaseSettings` reads missing initializer values from the environment while keeping explicit initializer overrides and defaults. ([Pydantic Docs][S00-6])

### `pydantic-extra-types`

Install only when needed:

```bash
pip install pydantic-extra-types
```

Use for domain-specific types moved out of core, e.g.:

```python
from pydantic_extra_types.color import Color
```

Pydantic v2 moved `BaseSettings` to `pydantic-settings`, and special-use types such as color and payment card types to `pydantic-extra-types`. ([Pydantic Docs][S00-3])

---

## 0.10 V2 vs V1: high-impact breaking changes

### 0.10.1 Method rename map

```text
V1                         V2
------------------------------------------------
__fields__                 model_fields
__private_attributes__     __pydantic_private__
__validators__             __pydantic_validator__
construct()                model_construct()
copy()                     model_copy()
dict()                     model_dump()
json_schema()              model_json_schema()
json()                     model_dump_json()
parse_obj()                model_validate()
update_forward_refs()      model_rebuild()
```

Pydantic v2 renamed non-deprecated `BaseModel` APIs to `model_*` or `__pydantic_*__`; old names are retained where possible but emit `DeprecationWarning`. ([Pydantic Docs][S00-3])

Agent migration rewrite:

```python
# V1
payload = model.dict()
json_s = model.json()
model = User.parse_obj(data)

# V2
payload = model.model_dump()
json_s = model.model_dump_json()
model = User.model_validate(data)
```

---

### 0.10.2 Deprecated data-loading helpers

```text
parse_raw   → model_validate_json for JSON, or load then model_validate
parse_file  → read file yourself, then model_validate/model_validate_json
from_orm    → model_validate with model_config = ConfigDict(from_attributes=True)
```

`parse_raw`, `parse_file`, and `from_orm` are deprecated in v2; `model_validate_json` replaces JSON parsing use cases, and `from_attributes=True` enables ORM-like attribute validation through `model_validate`. ([Pydantic Docs][S00-3])

---

### 0.10.3 Config rewrite

```python
from pydantic import BaseModel, ConfigDict

class Model(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        strict=True,
        from_attributes=True,
        populate_by_name=True,
    )
```

V2 uses `model_config` as a dict-like class attribute; the v1 inner `class Config:` pattern is deprecated. Config is inherited and merged across `BaseModel` subclasses, with later bases overriding earlier duplicate settings in multiple inheritance. ([Pydantic Docs][S00-3])

Removed/renamed config flags include:

```text
allow_mutation          removed → use frozen inverse
fields                  removed → use Annotated metadata
getter_dict / orm_mode  removed/renamed → from_attributes
schema_extra            renamed → json_schema_extra
validate_all            renamed → validate_default
anystr_*                renamed → str_* variants
```

Pydantic’s migration guide lists removed and renamed config settings, including `allow_mutation`, `fields`, `getter_dict`, `orm_mode → from_attributes`, and `schema_extra → json_schema_extra`. ([Pydantic Docs][S00-3])

---

### 0.10.4 Validators rewrite

```python
from pydantic import BaseModel, field_validator, model_validator

class M(BaseModel):
    x: int

    @field_validator("x")
    @classmethod
    def x_positive(cls, v: int) -> int:
        if v <= 0:
            raise ValueError("must be positive")
        return v

    @model_validator(mode="after")
    def check_model(self):
        return self
```

V1 `@validator` and `@root_validator` are deprecated; v2 uses `@field_validator` and `@model_validator`. The v2 field validator no longer has `each_item`; item validation should be attached to the container type argument, e.g. `list[Annotated[int, Field(ge=0)]]`. ([Pydantic Docs][S00-3])

Agent rewrite:

```python
# V1-ish
@validator("items", each_item=True)
def item_ok(cls, v): ...

# V2
items: list[Annotated[int, Field(ge=0)]]
```

---

### 0.10.5 Generic models

```python
from typing import Generic, TypeVar
from pydantic import BaseModel

T = TypeVar("T")

class Page(BaseModel, Generic[T]):
    items: list[T]
```

`pydantic.generics.GenericModel` is removed; v2 generic models inherit from `BaseModel` and `typing.Generic` directly. ([Pydantic Docs][S00-3])

Avoid:

```python
isinstance(page, Page[int])  # discouraged
```

Use:

```python
class IntPage(Page[int]): ...
isinstance(page, IntPage)
```

Pydantic advises against `isinstance` checks against parametrized generics; subclass parametrized variants if runtime checks are needed. ([Pydantic Docs][S00-3])

---

### 0.10.6 Root models

```python
from pydantic import RootModel

class Tags(RootModel[list[str]]):
    pass
```

V1 `__root__` custom-root models are replaced with `RootModel`; `RootModel` no longer supports `arbitrary_types_allowed`. ([Pydantic Docs][S00-3])

---

### 0.10.7 Serialization changes

```python
model.model_dump_json()
```

`.json()` is deprecated; `model_dump_json()` is the v2 replacement. JSON output may be compacted, and non-string JSON object keys are generally serialized via `str(key)`, yielding behavior changes relative to v1. ([Pydantic Docs][S00-3])

Agent test rule:

```text
Do not assert exact whitespace of model_dump_json().
Do assert parsed JSON equivalence unless compactness itself is a contract.
```

---

### 0.10.8 URL/DSN types no longer inherit from `str`

```python
url: AnyUrl
str(url)  # pass to APIs expecting str
```

In v2, URL/DSN types are built on new `Url` and `MultiHostUrl` classes and no longer inherit from `str`; use `str(url)` when passing to string-only APIs. ([Pydantic Docs][S00-3])

---

## 0.11 Migration execution plan for agents

### Mechanical pass

```bash
pip install bump-pydantic
cd /path/to/repo
bump-pydantic my_package
```

Pydantic provides the beta `bump-pydantic` tool for code transformation during v1→v2 migration. ([Pydantic Docs][S00-3])

### Required manual audit list

```text
1. Replace deprecated BaseModel APIs.
2. Replace Config inner classes with model_config.
3. Replace @validator/@root_validator.
4. Replace parse_raw/parse_file/from_orm.
5. Replace __root__ with RootModel.
6. Replace GenericModel.
7. Move BaseSettings imports to pydantic_settings.
8. Move color/payment-card imports to pydantic_extra_types.
9. Re-test serialization exactness.
10. Re-test ValidationError type values, not prose messages.
11. Re-test URL/DSN string assumptions.
12. Ensure no v1/v2 model mixing.
```

### Compatibility shim pattern

```python
# compat/pydantic_v1.py
from pydantic.v1 import BaseModel, Field, ValidationError
```

```python
# new code
from pydantic import BaseModel, Field, ValidationError
```

Agent rule:

```text
Never mix v1 and v2 BaseModel imports in the same schema graph.
Never pass v1 model classes into v2 generic model parameters.
Do not emit public library APIs accepting both v1 and v2 models unless explicitly version-gated.
```

---

## 0.12 What belongs in Pydantic vs application logic

### Belongs in Pydantic

Use Pydantic for **structural**, **syntactic**, **local**, **pure**, **deterministic** constraints:

```text
type conversion
required fields
nullable vs missing distinction
numeric bounds
string length/pattern
enum/literal membership
nested object shape
discriminated union selection
alias mapping
input normalization
serialization policy
JSON Schema metadata
settings/env parsing
secret redaction wrappers
API DTO boundaries
LLM structured-output parsing
queue/database/file payload validation
```

Good validator:

```python
class Signup(BaseModel):
    email: EmailStr
    age: int = Field(ge=13)
```

Good custom field validator:

```python
@field_validator("slug")
@classmethod
def normalize_slug(cls, v: str) -> str:
    return v.strip().lower().replace(" ", "-")
```

### Borderline: allowed but isolate

Use with caution:

```text
cross-field consistency
timezone policy
tenant-specific feature flags
soft business rules
authorization-adjacent constraints
database-backed existence checks
network-backed validation
clock-dependent validation
```

Pattern:

```python
class OrderIn(BaseModel):
    account_id: str
    amount_cents: int = Field(gt=0)

def place_order(data: OrderIn, *, actor: Actor, db: Session) -> Order:
    # business authorization + persistence here
    ...
```

### Does not belong in Pydantic

Keep outside Pydantic:

```text
database writes
network calls
authorization decisions
permission checks
non-idempotent side effects
workflow orchestration
retry logic
state-machine transitions
event publishing
audit logging
cache mutation
long-running computation
external service availability checks
```

Bad validator:

```python
@field_validator("user_id")
@classmethod
def user_must_exist(cls, v: str) -> str:
    db.insert_audit_row(...)       # side effect: bad
    requests.get(...)              # network: bad
    return v
```

Better:

```python
class Command(BaseModel):
    user_id: str

def handle(cmd: Command, repo: UserRepo) -> None:
    user = repo.require_user(cmd.user_id)
```

---

## 0.13 Deployment architecture patterns

### Pattern A — API boundary DTOs

```python
class CreateUserRequest(BaseModel):
    email: EmailStr
    age: int = Field(ge=13)

class UserResponse(BaseModel):
    id: UUID
    email: EmailStr
```

Value case:

```text
single source of truth for validation + serialization + OpenAPI schema
```

### Pattern B — message queue contracts

```python
class UserCreatedEvent(BaseModel):
    event_type: Literal["user.created"]
    user_id: UUID
    occurred_at: datetime
```

Agent rule:

```text
Validate at consumer boundary.
Serialize at producer boundary.
Version events explicitly.
```

### Pattern C — settings

```python
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="APP_", extra="ignore")
    database_url: str
    debug: bool = False
```

Value case:

```text
typed env parsing
test override via initializer
secrets/dotenv/source composition
deployment config centralization
```

`BaseSettings` reads values not passed as keyword args from environment variables, while defaults still apply when env vars are absent. ([Pydantic Docs][S00-6])

### Pattern D — LLM structured output

```python
class ToolCall(BaseModel):
    name: Literal["search", "summarize"]
    arguments: dict[str, Any]
```

Agent rule:

```text
Pydantic validates schema conformance of generated output.
Application logic still validates tool authorization, rate limits, and semantic feasibility.
```

---

## 0.14 Anti-pattern catalog

### Anti-pattern: treating `Optional[T]` as “not required”

```python
class M(BaseModel):
    x: int | None  # required, nullable
```

Correct if optional/missing allowed:

```python
class M(BaseModel):
    x: int | None = None
```

### Anti-pattern: validating business truth in field validators

```python
@field_validator("coupon")
def coupon_active(cls, v):
    return billing_api.validate_coupon(v)  # avoid
```

### Anti-pattern: relying on exact error prose

```python
assert "Input should be a valid integer" in str(exc)  # brittle
```

Prefer:

```python
assert exc.errors()[0]["type"] == "int_parsing"
```

### Anti-pattern: exact `model_dump_json()` whitespace snapshots

```python
assert model.model_dump_json() == '{"a": 1}'  # brittle
```

Prefer:

```python
assert json.loads(model.model_dump_json()) == {"a": 1}
```

### Anti-pattern: using v1 method names in new code

```python
model.dict()
model.json()
Model.parse_obj(data)
```

Prefer:

```python
model.model_dump()
model.model_dump_json()
Model.model_validate(data)
```

---

## 0.15 Agent-ready base model policy

### Strict internal base

```python
from pydantic import BaseModel, ConfigDict

class DomainModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        validate_assignment=False,
        arbitrary_types_allowed=False,
    )
```

### API input base

```python
class ApiInput(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        populate_by_name=True,
    )
```

### API output base

```python
class ApiOutput(BaseModel):
    model_config = ConfigDict(
        from_attributes=True,
        populate_by_name=True,
    )
```

### Settings base

```python
from pydantic_settings import BaseSettings, SettingsConfigDict

class AppSettings(BaseSettings):
    model_config = SettingsConfigDict(
        env_prefix="APP_",
        extra="ignore",
        validate_default=True,
    )
```

`BaseSettings` validates default values by default, unlike plain `BaseModel`; this can be disabled globally or per field. ([Pydantic Docs][S00-6])

---

## 0.16 Minimal reference implementation

```python
from __future__ import annotations

from datetime import datetime
from typing import Literal
from uuid import UUID

from pydantic import BaseModel, ConfigDict, Field, ValidationError, field_validator
from pydantic_settings import BaseSettings, SettingsConfigDict


class ApiModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        populate_by_name=True,
    )


class CreateUser(ApiModel):
    email: str
    age: int = Field(ge=13)
    source: Literal["web", "admin", "import"] = "web"

    @field_validator("email")
    @classmethod
    def normalize_email(cls, v: str) -> str:
        return v.strip().lower()


class UserCreated(ApiModel):
    id: UUID
    email: str
    created_at: datetime


class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="APP_")
    database_url: str
    debug: bool = False


def parse_request(raw: dict) -> CreateUser:
    return CreateUser.model_validate(raw)


def serialize_event(event: UserCreated) -> str:
    return event.model_dump_json()


try:
    cmd = parse_request({"email": " A@EXAMPLE.COM ", "age": "12"})
except ValidationError as exc:
    error_types = [e["type"] for e in exc.errors()]
    # stable-ish machine handling: ["greater_than_equal"]
    raise
```

---

## 0.17 Operational checklist

### New project

```text
[ ] Pin pydantic v2.
[ ] Use only model_* APIs.
[ ] Define project base models.
[ ] Decide strict/lax policy by boundary.
[ ] Keep validators pure.
[ ] Use pydantic-settings only for config/env/secrets.
[ ] Install pydantic-extra-types only for domain-specific optional types.
[ ] Assert ValidationError.errors()[*]["type"], not full message text.
[ ] Snapshot JSON Schema only where public contract requires it.
```

### Migration project

```text
[ ] Run bump-pydantic.
[ ] Replace v1 method names.
[ ] Replace Config with model_config.
[ ] Replace @validator/@root_validator.
[ ] Replace GenericModel.
[ ] Replace __root__.
[ ] Replace parse_raw/parse_file/from_orm.
[ ] Move BaseSettings.
[ ] Move color/payment card types.
[ ] Audit URL/DSN string assumptions.
[ ] Remove v1/v2 mixing.
[ ] Run strict serialization/error regression tests.
```

### Library project

```text
[ ] Do not expose pydantic-core internals as public API.
[ ] Avoid exact core-schema dict dependencies.
[ ] Version exported model behavior.
[ ] Document v2-only support or provide pydantic.v1 compatibility module.
[ ] Treat JSON Schema shape as a public compatibility surface only if users depend on it.
```

---

## 0.18 Value case summary

```text
Pydantic value = one annotation graph reused for:
  validation
  coercion
  normalized runtime objects
  serialization
  JSON Schema
  settings/env parsing
  IDE/static-analysis-friendly contracts
  structured errors
```

The strongest deployment case: place Pydantic at trust boundaries, convert untrusted/stringly/JSON data into typed Python objects once, then keep business logic explicit, side-effecting, and testable outside validators.

[S00-1]: https://docs.pydantic.dev/latest/ "Welcome to Pydantic | Pydantic Docs"
[S00-2]: https://docs.pydantic.dev/latest/concepts/models/ "Models | Pydantic Docs"
[S00-3]: https://docs.pydantic.dev/latest/migration/ "Migration Guide | Pydantic Docs"
[S00-4]: https://docs.pydantic.dev/latest/version-policy/ "Version Policy | Pydantic Docs"
[S00-5]: https://docs.pydantic.dev/latest/internals/architecture/ "Architecture | Pydantic Docs"
[S00-6]: https://docs.pydantic.dev/latest/concepts/pydantic_settings/ "Settings Management | Pydantic Docs"


# Pydantic Advanced — 1) Installation, package layout, and deployment surface — Pydantic v2

Style target: sectioned, agent-oriented, dense technical reference. 

Version anchor: current Pydantic docs describe installation through `pip install pydantic` / `uv add pydantic`; base runtime dependencies include `pydantic-core`, `typing-extensions`, and `annotated-types`; Python **3.9+** is supported per install docs. ([Pydantic Docs][S01-1])

---

## 1.0 Installation surface map

```text
pydantic
├─ required runtime deps
│  ├─ pydantic-core        # Rust-backed validation / serialization engine
│  ├─ typing-extensions    # typing backports
│  └─ annotated-types      # reusable Annotated constraints
├─ optional extras
│  ├─ pydantic[email]      # EmailStr / email validation support
│  └─ pydantic[timezone]   # tzdata fallback IANA timezone database
└─ ecosystem packages
   ├─ pydantic-settings    # BaseSettings / env / dotenv / secrets / config sources
   └─ pydantic-extra-types # Color, payment card, phone, country, currency, etc.
```

Pydantic’s install docs identify `pydantic-core` as the Rust-written core validation dependency, `typing-extensions` as a typing backport, and `annotated-types` as reusable constraints for `typing.Annotated`; optional built-in extras are currently `email` and `timezone`. ([Pydantic Docs][S01-1])

---

## 1.1 Base installation recipes

### Application install

```bash
pip install pydantic
```

```bash
uv add pydantic
```

```bash
conda install pydantic -c conda-forge
```

Use base install when the project uses only built-in types, `BaseModel`, `Field`, `TypeAdapter`, validators, serializers, JSON Schema, dataclasses, and standard Python types. Pydantic documents pip, uv, and conda-forge install paths. ([Pydantic Docs][S01-1])

### Optional extras

```bash
pip install "pydantic[email]"
pip install "pydantic[email,timezone]"
```

```bash
uv add "pydantic[email]"
uv add "pydantic[email,timezone]"
```

Use `email` when importing or validating `EmailStr` / email-aware types; use `timezone` when fallback IANA timezone data via `tzdata` is needed. Pydantic’s install page maps `email` to `email-validator` and `timezone` to `tzdata`. ([Pydantic Docs][S01-1])

### Source / main branch install

```bash
pip install "git+https://github.com/pydantic/pydantic@main"
pip install "git+https://github.com/pydantic/pydantic@main#egg=pydantic[email,timezone]"
```

Source install is for upstream testing, bug reproduction, pre-release verification, or contributor workflows; production builds should prefer released wheels and lockfiles. Pydantic documents direct repository installation but normal deployment should avoid unpinned main-branch installs. ([Pydantic Docs][S01-1])

---

## 1.2 Ecosystem package installs

### `pydantic-settings`

```bash
pip install pydantic-settings
```

```python
from pydantic_settings import BaseSettings, SettingsConfigDict
```

Use for environment variables, dotenv files, secrets directories, pyproject TOML settings, cloud secret manager sources, settings-source customization, and CLI-backed settings. `BaseSettings` was moved out of Pydantic core into `pydantic-settings`; current settings docs state that `pydantic-settings` provides optional features for loading settings/config classes from environment variables or secrets files. ([Pydantic Docs][S01-2])

### `pydantic-extra-types`

```bash
pip install pydantic-extra-types
```

```python
from pydantic_extra_types.color import Color
from pydantic_extra_types.phone_numbers import PhoneNumber
```

Use only when domain-specific types are required. Pydantic’s migration guide states that special-use types such as color and payment card types moved to `pydantic-extra-types`; the latest API docs include extra-type modules such as color, country, payment, phone numbers, routing numbers, coordinate, MAC address, ISBN, currency, language, semantic version, timezone name, and ULID. ([Pydantic Docs][S01-2])

---

## 1.3 Dependency declaration patterns

### Application `pyproject.toml`

```toml
[project]
name = "my-service"
requires-python = ">=3.11"
dependencies = [
  "pydantic[email,timezone]>=2.13,<3",
  "pydantic-settings>=2,<3",
]
```

Application stance: pin direct dependencies with lower and upper bounds, then use a lockfile for exact transitive versions. Pydantic v2 policy says deprecated features are retained until v3 and minor v2 releases avoid intentional breaking changes, so `<3` is a natural compatibility ceiling for v2-targeted code. ([Pydantic Docs][S01-3])

### Library `pyproject.toml`

```toml
[project]
name = "my-public-library"
requires-python = ">=3.9"
dependencies = [
  "pydantic>=2.7,<3",
]

[project.optional-dependencies]
email = [
  "pydantic[email]>=2.7,<3",
]
settings = [
  "pydantic-settings>=2,<3",
]
extra-types = [
  "pydantic-extra-types>=2,<3",
]
all = [
  "pydantic[email,timezone]>=2.7,<3",
  "pydantic-settings>=2,<3",
  "pydantic-extra-types>=2,<3",
]
```

Library stance: declare only hard imports in `[project.dependencies]`; put optional integrations in `[project.optional-dependencies]`. The Python packaging spec defines `dependencies` as PEP 508 strings mapped to `Requires-Dist`, and `optional-dependencies` as extras mapped to `Provides-Extra`. ([Python Packaging][S01-4])

### Dev-only dependencies

```toml
[dependency-groups]
dev = [
  "pytest",
  "mypy",
  "ruff",
  "pyright",
]
```

Keep development tools out of install-time runtime metadata. Use package extras for user-facing optional capabilities; use dependency groups or tool-specific dev dependency sections for local development/test tooling.

---

## 1.4 Pinning policy

### Application policy

```text
runtime:    pydantic>=2.x,<3
lockfile:   exact pydantic / pydantic-core / annotated-types / typing-extensions
CI matrix:  locked + latest compatible
upgrade:    test errors/schema/serialization before bump
```

Application deployment should lock exact artifacts because `pydantic-core` is a compiled dependency and because Pydantic’s version policy allows some minor-release changes that are not considered breaking: JSON Schema reference formatting, `ValidationError.msg`, `ctx`, `loc`, added error keys, added error types, `__repr__`, and low-level core schema contents. Programmatic error parsing should use `ValidationError.errors()[*]["type"]`. ([Pydantic Docs][S01-3])

### Library policy

```text
lower bound: first Pydantic minor you test
upper bound: <3
avoid:       pinning exact pydantic in libraries
avoid:       depending on pydantic-core directly
test:        oldest-supported + newest-compatible
```

Reasoning: exact pins in libraries force downstream resolver conflicts; wide unbounded ranges risk v3 incompatibility; lower bounds should reflect actually used APIs.

### Experimental-feature policy

```text
from pydantic.experimental import ...
experimental_* arguments / fields / methods
```

Treat these as unstable. Pydantic’s version policy says experimental features may change during patch/minor releases, may not be backward-compatible, and may be removed with little notice. ([Pydantic Docs][S01-3])

---

## 1.5 Runtime compatibility expectations

### Python version

```toml
[project]
requires-python = ">=3.9"
```

Pydantic install docs state Python 3.9+ is sufficient. For applications, prefer a narrower operational floor, for example `>=3.11`, if that is what production actually runs. For libraries, use the oldest Python version you test. ([Pydantic Docs][S01-1])

### Python-version drop policy

```text
Pydantic may drop a Python version after:
  1. that Python version reaches expected end-of-life
  2. less than 5% of downloads of the most recent minor release use it
```

Use this to plan support windows: if your package promises older Python support, you may need to pin older Pydantic minors longer than a normal application would. ([Pydantic Docs][S01-3])

### Compiled dependency / platform consideration

```text
pydantic → pydantic-core → Rust-backed native extension
```

Pydantic’s core validation logic is in `pydantic-core`, described by the install docs as Rust-written core validation logic. Deployment implication: prefer environments where prebuilt wheels are available; keep `pip`/build tools current; test container images and less-common CPU/OS combinations explicitly. ([Pydantic Docs][S01-1])

---

## 1.6 Package layout for applications

Recommended layout:

```text
src/myapp/
├─ __init__.py
├─ schemas/
│  ├─ __init__.py
│  ├─ base.py          # project BaseModel classes / ConfigDict policy
│  ├─ common.py        # reusable constrained aliases / shared DTO fragments
│  ├─ api.py           # request/response DTOs
│  ├─ events.py        # queue/event contracts
│  ├─ persistence.py   # DB row / persistence DTOs, if needed
│  └─ errors.py        # error response schemas
├─ settings.py         # BaseSettings only; no model imports that cause cycles
├─ types.py            # custom types / Annotated aliases / reusable validators
├─ validators.py       # pure reusable validator functions
├─ serializers.py      # pure reusable serializer functions
├─ domain/
│  └─ ...              # business/domain logic; avoid Pydantic side effects
└─ services/
   └─ ...              # orchestration, IO, DB, network
```

### Placement rules

```text
schemas/base.py       owns shared ConfigDict policy.
schemas/api.py        owns external HTTP contracts.
schemas/events.py     owns message contracts.
settings.py           owns BaseSettings classes and settings-source config.
types.py              owns reusable Annotated aliases and custom type hooks.
validators.py         owns pure validation functions only.
serializers.py        owns pure serializer functions only.
domain/               owns business invariants and side effects.
services/             owns IO, auth, DB, network, transactions.
```

Value case: import graph clarity. Pydantic model definition triggers schema collection/build behavior; separate schema modules from side-effect-heavy modules to keep imports cheap and deterministic. Pydantic architecture docs state that at model definition time the metaclass collects annotations, config, validators/serializers, private attributes, class variables, generics, etc., then communicates that collected information to `pydantic-core` via core schema. ([Pydantic Docs][S01-5])

---

## 1.7 Package layout for libraries exposing Pydantic models

Recommended public library layout:

```text
src/mylib/
├─ __init__.py
├─ py.typed
├─ models/
│  ├─ __init__.py
│  ├─ base.py
│  ├─ public.py        # stable exported models
│  └─ internal.py      # non-public models; not re-exported
├─ types.py            # public reusable types
├─ _compat.py          # version gates / import compatibility
└─ _internal/
   ├─ validators.py
   └─ serializers.py
```

### Export policy

```python
# mylib/models/__init__.py
from .public import CreateThing, Thing, ThingError

__all__ = ["CreateThing", "Thing", "ThingError"]
```

### Stability policy

```text
Public Pydantic model field name     = public API.
Public field alias                   = public API.
Public validation behavior           = public API.
Public serialization behavior        = public API.
Public JSON Schema shape             = public API if documented/generated for users.
Private validators/core schema       = not public API.
```

### Library anti-pattern

```python
# bad: hidden import side effect at package import
from .settings import Settings
settings = Settings()  # reads env at import time
```

### Library-safe pattern

```python
# good: caller controls construction
from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    ...

def load_settings(**overrides) -> Settings:
    return Settings(**overrides)
```

Do not read environment variables, files, secrets stores, or network resources at import time. `BaseSettings` construction reads missing initializer values from the environment; therefore, construction timing is a deployment decision, not a module import detail. ([Pydantic Docs][S01-6])

---

## 1.8 Shared base model placement

### Base model module

```python
# src/myapp/schemas/base.py
from pydantic import BaseModel, ConfigDict

class AppModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        validate_assignment=False,
        arbitrary_types_allowed=False,
        hide_input_in_errors=True,
    )
```

### API model base

```python
class ApiModel(AppModel):
    model_config = ConfigDict(
        extra="forbid",
        validate_by_alias=True,
        validate_by_name=True,
        serialize_by_alias=True,
    )
```

### ORM/read model base

```python
class ReadModel(AppModel):
    model_config = ConfigDict(
        from_attributes=True,
    )
```

Pydantic config can be changed globally for a project by creating a custom parent model with `model_config`; configuration is inherited by subclasses. ([Pydantic Docs][S01-7])

### Agent rule

```text
One project base model per validation regime:
  ApiInputModel
  ApiOutputModel
  EventModel
  SettingsModel
  InternalModel

Avoid:
  one universal BaseModel with every option enabled.
```

Reason: API input, API output, events, settings, and internal DTOs have different alias, extra-field, strictness, serialization, and security policies.

---

## 1.9 Custom type placement

### Reusable `Annotated` aliases

```python
# src/myapp/types.py
from typing import Annotated
from pydantic import Field

UserId = Annotated[str, Field(min_length=1, max_length=64, pattern=r"^[a-zA-Z0-9_-]+$")]
PositiveCents = Annotated[int, Field(ge=1)]
Slug = Annotated[str, Field(min_length=1, max_length=128, pattern=r"^[a-z0-9-]+$")]
```

Use aliases when the same constraint appears in multiple models.

### Custom validator functions

```python
# src/myapp/validators.py
def normalize_email(v: str) -> str:
    return v.strip().lower()
```

```python
# src/myapp/schemas/api.py
from pydantic import field_validator
from myapp.schemas.base import ApiModel
from myapp.validators import normalize_email

class Signup(ApiModel):
    email: str

    @field_validator("email")
    @classmethod
    def _email(cls, v: str) -> str:
        return normalize_email(v)
```

### Custom core-schema hooks

```python
# src/myapp/types.py
class ExternalId:
    @classmethod
    def __get_pydantic_core_schema__(cls, source, handler):
        ...
```

Only place low-level custom type hooks in `types.py` or a dedicated package such as `myapp/pydantic_types/`. Pydantic’s migration guide states that v2 custom type integration moved from `__get_validators__` to hooks such as `__get_pydantic_core_schema__`, and that `Annotated` can modify or provide those hooks for third-party types. ([Pydantic Docs][S01-2])

---

## 1.10 Settings class placement

### Basic settings module

```python
# src/myapp/settings.py
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_prefix="MYAPP_",
        env_file=".env",
        extra="ignore",
    )

    database_url: str
    log_level: str = "INFO"
```

### Construction boundary

```python
# src/myapp/main.py
from myapp.settings import Settings

def create_app(settings: Settings | None = None):
    settings = settings or Settings()
    ...
```

### Test override

```python
def test_app():
    settings = Settings(database_url="sqlite:///:memory:")
    app = create_app(settings)
```

`pydantic-settings` reads environment values for fields not passed as keyword arguments, while defaults remain in effect if no matching environment variable is set. ([Pydantic Docs][S01-6])

### Dotenv caution

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_file=".env", extra="ignore")
```

Settings docs note that dotenv handling considers `extra`; for compatibility with Pydantic 1.x `BaseSettings`, use `extra="ignore"`. They also note that dotenv values are loaded and passed to the model regardless of `env_prefix` unless dotenv filtering is used. ([Pydantic Docs][S01-6])

---

## 1.11 Import-time model creation costs

### What happens at import

```python
class User(BaseModel):
    id: int
    email: str
```

At class definition/import time, Pydantic collects annotations, model config, validators, serializers, private attributes, class variables, and generic information; it builds communication artifacts for `pydantic-core`. ([Pydantic Docs][S01-5])

### Cost drivers

```text
number of model classes
deep nested schemas
large unions
generic models
recursive/forward refs
custom validators/serializers
custom type hooks
JSON Schema generation at import
TypeAdapter construction at import
settings construction at import
```

### Defer build

```python
from pydantic import BaseModel, ConfigDict

class RarelyUsedModel(BaseModel):
    model_config = ConfigDict(defer_build=True)
    payload: dict[str, object]
```

`defer_build=True` defers model validator and serializer construction until first validation; Pydantic documents this as useful for avoiding overhead for models only used nested inside other models or when manually defining type namespaces. Since v2.10 it also applies to Pydantic dataclasses and TypeAdapters. ([Pydantic Docs][S01-8])

### TypeAdapter reuse

```python
# bad: builds validator/serializer every call
from pydantic import TypeAdapter

def parse_ids(raw):
    return TypeAdapter(list[int]).validate_python(raw)
```

```python
# good: one adapter, reused
from pydantic import TypeAdapter

_IDS = TypeAdapter(list[int])

def parse_ids(raw):
    return _IDS.validate_python(raw)
```

Pydantic performance docs state that each `TypeAdapter` instantiation constructs a new validator and serializer; instantiate once and reuse. ([Pydantic Docs][S01-9])

---

## 1.12 Deployment tradeoffs

### Serverless / cold start

```text
minimize import graph
avoid importing all schemas in __init__.py
avoid import-time Settings()
avoid import-time model_json_schema()
reuse TypeAdapter singletons
consider defer_build=True for rarely used / deeply nested schemas
```

### Web service / long-running process

```text
eager schema build acceptable if startup budget allows
prefer failure at boot for critical API models
pre-warm schema-heavy endpoints
construct Settings once at app startup
reuse TypeAdapter singletons
```

### CLI / short-lived process

```text
avoid importing API-wide schema graph for a single subcommand
lazy import command-specific schemas
avoid pydantic-settings unless env/config parsing is actually needed
avoid JSON Schema generation at runtime unless command explicitly asks
```

### Data pipeline / batch validation

```text
validate in batches
reuse TypeAdapter for list[Record] or Iterable input contract
prefer model_validate_json for raw JSON input
avoid wrap validators in hot paths unless required
```

Pydantic performance docs recommend `model_validate_json()` over `model_validate(json.loads(...))` in general because the former validates internally from JSON instead of parsing JSON in Python first, with caveats for some before/wrap validator cases. ([Pydantic Docs][S01-9])

---

## 1.13 Import graph anti-patterns

### Anti-pattern: central schema barrel imports everything

```python
# src/myapp/schemas/__init__.py
from .api import *
from .events import *
from .settings import *
from .persistence import *
```

Problem:

```text
import myapp.schemas
  → builds every model
  → imports settings
  → may import DB/domain modules
  → slow cold start / circular imports / side effects
```

Preferred:

```python
# src/myapp/schemas/__init__.py
# keep empty or export only stable, lightweight public symbols
```

### Anti-pattern: settings imported into base model module

```python
# bad
from myapp.settings import Settings
settings = Settings()

class AppModel(BaseModel):
    ...
```

Preferred:

```text
schemas/base.py does not import settings.py
settings.py may import small reusable scalar types only
application factory wires settings to services
```

### Anti-pattern: validators importing services

```python
# bad
from myapp.services.users import user_exists
```

Preferred:

```text
Pydantic validators: pure/local/synchronous/side-effect-free
Service layer: DB/network/authorization/business truth
```

---

## 1.14 Library packaging guidance for models

### Public model compatibility

```python
class User(BaseModel):
    id: str
    email: str
```

Changing any of these is a breaking or semi-breaking library change:

```text
field removal
field rename
alias rename
required → optional
optional → required
type narrowing
strictness increase
extra ignore → extra forbid
serialization alias change
JSON Schema mode/default change
validator behavior change
```

### Public schema-generation policy

```python
schema = User.model_json_schema()
```

If users consume generated JSON Schema, document whether JSON Schema is a stable API. Pydantic version policy explicitly permits JSON Schema reference-format changes in minor releases, so schema snapshots may change even without intentional breaking changes. ([Pydantic Docs][S01-3])

### Error compatibility

```python
try:
    User.model_validate(data)
except ValidationError as exc:
    for err in exc.errors():
        code = err["type"]
```

Expose error `type` values if building stable error wrappers; avoid exposing exact Pydantic prose as a public contract because Pydantic says `msg`, `ctx`, and `loc` may change, while `type` is the intended stable programmatic field. ([Pydantic Docs][S01-3])

### Type-checking marker

```text
src/mylib/py.typed
```

Include `py.typed` in typed libraries that expose Pydantic models. This advertises inline typing to type checkers.

---

## 1.15 Optional dependency design for libraries

### Direct import means direct dependency

```python
# mylib/types.py
from pydantic_extra_types.color import Color
```

Then:

```toml
[project]
dependencies = [
  "pydantic>=2.7,<3",
  "pydantic-extra-types>=2,<3",
]
```

### Optional integration means lazy import + extra

```python
def color_model_factory():
    try:
        from pydantic_extra_types.color import Color
    except ImportError as exc:
        raise RuntimeError(
            "Install mylib[color] to use color models."
        ) from exc

    class Paint(BaseModel):
        color: Color

    return Paint
```

```toml
[project.optional-dependencies]
color = ["pydantic-extra-types>=2,<3"]
```

### Agent rule

```text
If a module imports an optional package at top level,
that optional package is not optional for that module.

If users can import the package without the extra,
isolate optional imports inside:
  functions
  plugin modules
  integration modules
  optional subpackages
```

---

## 1.16 Pydantic-core dependency posture

### Default

```text
Use pydantic, not pydantic_core.
```

Pydantic architecture docs define model definition in `pydantic` and validation/serialization in `pydantic-core`; application code should normally stay at the `pydantic` API layer. ([Pydantic Docs][S01-5])

### Direct `pydantic_core` import only for

```text
custom errors
custom core-schema hooks
library-level type integration
advanced plugin/internals work
diagnostics/profiling
```

### Avoid

```python
from pydantic_core import SchemaValidator
```

unless implementing infrastructure. Direct use bypasses `BaseModel`, `Field`, `ConfigDict`, alias handling, serializers, JSON Schema conveniences, and future Pydantic API compatibility.

---

## 1.17 Deployment recipes

### FastAPI-style API service

```toml
[project]
dependencies = [
  "fastapi",
  "pydantic[email]>=2.13,<3",
  "pydantic-settings>=2,<3",
]
```

```text
schemas/api.py         request/response models
settings.py            BaseSettings
main.py                construct Settings once
tests/                 assert model_dump / ValidationError.type / OpenAPI schema if public
```

### Internal batch validator

```toml
[project]
dependencies = [
  "pydantic>=2.13,<3",
]
```

```python
from pydantic import TypeAdapter

_ROWS = TypeAdapter(list[Row])

def validate_batch(raw):
    return _ROWS.validate_python(raw)
```

### Public SDK

```toml
[project]
dependencies = [
  "pydantic>=2.7,<3",
]

[project.optional-dependencies]
email = ["pydantic[email]>=2.7,<3"]
settings = ["pydantic-settings>=2,<3"]
```

```text
Do not instantiate Settings at import.
Do not expose pydantic-core.
Do not snapshot Pydantic error prose as SDK API.
```

### Serverless function

```text
Keep handler import path lean.
Move heavy schema groups behind route/action-specific imports.
Avoid JSON Schema generation during cold start.
Use defer_build=True for rarely executed model graphs.
Prebuild critical validators during provisioned-warm startup if latency matters.
```

---

## 1.18 File placement decision matrix

```text
Thing                                         Place
---------------------------------------------------------------
BaseModel subclass policy                    schemas/base.py
API request/response DTOs                    schemas/api.py
Queue/event payload contracts                schemas/events.py
Reusable Field/Annotated aliases             types.py
Custom __get_pydantic_core_schema__ types    types.py or pydantic_types/
Pure validator functions                     validators.py
Pure serializer functions                    serializers.py
Settings/BaseSettings                        settings.py
Env/dotenv/secrets source customization      settings.py or settings_sources.py
Business rule requiring DB/network/auth      services/ or domain/
Generated JSON Schema files                  build artifact directory
Test-only models                             tests/fixtures/schemas.py
```

---

## 1.19 Agent checklist

```text
[ ] Choose runtime floor: Python >=3.9 minimum; narrower if app controls runtime.
[ ] Add pydantic dependency with >=tested,<3.
[ ] Add pydantic[email] only if email validation is used.
[ ] Add pydantic[timezone] only if tzdata fallback is required.
[ ] Add pydantic-settings only if BaseSettings/env/dotenv/secrets are used.
[ ] Add pydantic-extra-types only if importing extra domain types.
[ ] Put optional integrations in [project.optional-dependencies].
[ ] Keep Settings construction out of import time.
[ ] Keep schema package __init__.py lightweight.
[ ] Put shared ConfigDict policy in schemas/base.py.
[ ] Put reusable Annotated aliases in types.py.
[ ] Keep validators pure and side-effect-free.
[ ] Reuse TypeAdapter instances.
[ ] Consider defer_build=True for cold-start-sensitive, rarely used model graphs.
[ ] Assert ValidationError.errors()[*]["type"], not prose.
[ ] Treat JSON Schema snapshots as public only when explicitly committed.
[ ] Test oldest-supported and newest-compatible Pydantic v2 in libraries.
```

---

## 1.20 Value case

```text
Installation value:
  small base dependency set
  optional feature extras
  separate settings package
  separate extra-types package
  Rust-backed runtime execution via pydantic-core

Layout value:
  deterministic import graph
  explicit validation boundaries
  reusable base policies
  isolated settings side effects
  optional integrations without hard dependency pollution

Deployment value:
  fast hot-path validation
  controlled cold-start cost
  lockfile-friendly transitive runtime
  stable library compatibility envelope
  clear separation: schema code vs business code
```

[S01-1]: https://docs.pydantic.dev/latest/install/ "Installation | Pydantic Docs"
[S01-2]: https://docs.pydantic.dev/latest/migration/ "Migration Guide | Pydantic Docs"
[S01-3]: https://docs.pydantic.dev/latest/version-policy/ "Version Policy | Pydantic Docs"
[S01-4]: https://packaging.python.org/en/latest/specifications/pyproject-toml/ "pyproject.toml specification - Python Packaging User Guide"
[S01-5]: https://docs.pydantic.dev/latest/internals/architecture/ "Architecture | Pydantic Docs"
[S01-6]: https://docs.pydantic.dev/latest/concepts/pydantic_settings/ "Settings Management | Pydantic Docs"
[S01-7]: https://docs.pydantic.dev/latest/concepts/config/ "Configuration | Pydantic Docs"
[S01-8]: https://docs.pydantic.dev/latest/api/config/ "Configuration | Pydantic Docs"
[S01-9]: https://docs.pydantic.dev/latest/concepts/performance/ "Performance | Pydantic Docs"


# Pydantic Advanced — 2) Core model API — `BaseModel` as the main front door

Style target: dense, sectioned, agent-oriented reference. 

`BaseModel` is Pydantic’s primary schema-definition object: define a subclass, declare fields as annotated class attributes, then use that class for validation, coercion, serialization, and JSON Schema generation. Pydantic guarantees that the **resulting model instance** conforms to declared field types and constraints after validation, not that the raw input was already valid. ([Pydantic Docs][S02-1])

---

## 2.0 Core object contract

```python
from pydantic import BaseModel

class User(BaseModel):
    id: int
    name: str = "Jane Doe"
```

Semantic expansion:

```text
class User(BaseModel)
  → collect annotations / defaults / config / validators / serializers
  → build pydantic-core schema
  → expose validation constructors
  → expose serialization methods
  → expose JSON Schema method
```

Pydantic models are “classes which inherit from `BaseModel` and define fields as annotated attributes”; Pydantic analyzes the class body and type annotations to build the schema used for validation and serialization. ([Pydantic Docs][S02-1])

---

## 2.1 Defining models with annotated fields

### Minimal model

```python
from pydantic import BaseModel

class Product(BaseModel):
    sku: str
    price_cents: int
    active: bool = True
```

Field interpretation:

```text
sku          required, str
price_cents  required, int
active       not required, bool, default True
```

Pydantic treats a field without a default as required; a field with a default value is not required. The docs show `id: int` as required and `name: str = "Jane Doe"` as not required because it has a default. ([Pydantic Docs][S02-1])

---

## 2.2 Required vs optional vs nullable

### Syntax matrix

```python
from pydantic import BaseModel

class M(BaseModel):
    x1: int                 # required, cannot be None
    x2: int | None          # required, can be None
    x3: int = 1             # not required, cannot be None, default 1
    x4: int | None = None   # not required, can be None, default None
```

| Declaration             | Required? | Accepts `None`? | Default | Meaning                                         |
| ----------------------- | --------: | --------------: | ------- | ----------------------------------------------- |
| `x: int`                |       yes |              no | none    | caller must provide an integer-coercible value  |
| `x: int \| None`        |       yes |             yes | none    | caller must provide value; value may be `None`  |
| `x: int = 1`            |        no |              no | `1`     | missing field uses `1`; explicit `None` invalid |
| `x: int \| None = None` |        no |             yes | `None`  | missing field allowed; explicit `None` allowed  |

Pydantic v2 intentionally changed optional/nullable behavior to match dataclass-like semantics: `Optional[T]` or `T | None` means “nullable,” not “has default `None`.” Any explicit default makes a field not required. ([Pydantic Docs][S02-2])

---

## 2.3 “Optional” naming hazard

```python
class BadAssumption(BaseModel):
    x: int | None
```

This is **required nullable**, not optional/missing-permitted.

```python
BadAssumption.model_validate({})          # ValidationError: x missing
BadAssumption.model_validate({"x": None}) # ok
```

Correct missing-permitted declaration:

```python
class Good(BaseModel):
    x: int | None = None
```

Agent rule:

```text
nullable      = T | None
not required  = has default
optional      = ambiguous term; avoid in generated docs unless explicitly defined
```

The migration guide explicitly states that a field annotated as `typing.Optional[T]` is required and allows `None`; it does not imply a default value of `None`. ([Pydantic Docs][S02-2])

---

## 2.4 Field declaration forms

### Plain annotation

```python
class User(BaseModel):
    id: int
```

Required field, no default.

### Assignment default

```python
class User(BaseModel):
    name: str = "John Doe"
```

Not required; default used if omitted. ([Pydantic Docs][S02-3])

### `Field(...)` with metadata, still required

```python
from pydantic import Field

class User(BaseModel):
    name: str = Field(frozen=True)
```

This field is still required because `Field(frozen=True)` does not supply a default. The docs warn that assigning `Field(...)` can look like a default but still be required; `Field(..., frozen=True)` is explicit but discouraged for static type checker ergonomics. ([Pydantic Docs][S02-3])

### Default through `Field`

```python
class User(BaseModel):
    age: int = Field(default=20)
```

Equivalent requiredness to:

```python
class User(BaseModel):
    age: int = 20
```

`Field(default=...)` and normal assignment syntax both provide defaults. ([Pydantic Docs][S02-3])

---

## 2.5 Default validation behavior

```python
class M(BaseModel):
    x: int = "not an int"

M()  # default is not validated by default
```

Enable default validation:

```python
class M(BaseModel):
    x: int = Field(default="not an int", validate_default=True)
```

By default, Pydantic does **not** validate default values; use `Field(validate_default=True)` or the model-level `validate_default` config to enable default validation. ([Pydantic Docs][S02-3])

Agent rule:

```text
Trusted constant defaults:       validate_default=False acceptable.
User-controlled generated defaults: validate_default=True.
Settings models:                 prefer validating defaults.
Security-critical defaults:       validate_default=True.
```

---

## 2.6 Mutable defaults

```python
class M(BaseModel):
    items: list[dict[str, int]] = [{}]
```

Pydantic deep-copies non-hashable default values for each new model instance, so this avoids the standard Python “shared mutable default” trap for unhashable defaults. The fields docs show separate instances receiving separate copies for a list/dict default. ([Pydantic Docs][S02-3])

Deployment rule:

```text
Allowed:  simple literal defaults, small mutable defaults.
Prefer:   default_factory for dynamic / large / semantic defaults.
Avoid:    huge mutable defaults in model class body.
```

---

## 2.7 Class-level field introspection: `model_fields`

```python
field_info = User.model_fields["name"]
```

`model_fields` is a class-level mapping of field names to `FieldInfo` objects. It exposes the resolved field annotation, alias, metadata, constraints, defaults, and other field definition facts. Instance access to `model_fields` is deprecated as of v2.11 and planned for removal in v3; use `Model.model_fields`, not `model.model_fields`. ([Pydantic Docs][S02-3])

Agent pattern:

```python
for name, info in User.model_fields.items():
    print(name, info.annotation, info.default, info.alias)
```

Do not treat `model_fields` as a serialized-data source. It is schema metadata.

---

## 2.8 Construction and validation entrypoints

## 2.8.1 `Model(...)`

```python
user = User(id="123")
assert user.id == 123
```

Constructor behavior:

```text
mode:           Python mode
input shape:    keyword arguments only
validation:     yes
coercion:       yes, unless strict config/field/call behavior prevents it
custom __init__: called only for constructor path unless overridden carefully
```

Pydantic’s constructor performs parsing and validation; the docs show `User(id="123")` producing `id == 123` as an integer. Pydantic also states that Python mode is used for the `__init__()` constructor and that field values must be provided as keyword arguments. ([Pydantic Docs][S02-1])

Recommended use:

```text
internal ergonomic construction
tests
small payloads already in kwargs form
manual model creation from trusted Python data
```

Avoid:

```python
User({"id": "123"})  # wrong shape; BaseModel expects kwargs
```

---

## 2.8.2 `Model.model_validate(...)`

Signature:

```python
@classmethod
def model_validate(
    cls,
    obj: Any,
    strict: bool | None = None,
    extra: ExtraValues | None = None,
    from_attributes: bool | None = None,
    context: Any | None = None,
    by_alias: bool | None = None,
    by_name: bool | None = None,
) -> Self: ...
```

Use:

```python
user = User.model_validate({"id": "123", "name": "Ada"})
```

Use cases:

```text
decoded JSON dicts
database rows as mappings
message payloads
arbitrary object validation with from_attributes=True
explicit validation context
per-call strictness
per-call extra behavior
per-call alias/name behavior
```

`model_validate()` validates an arbitrary object and returns a model instance; parameters include strictness, extra-data behavior, `from_attributes`, context, alias usage, and field-name usage. It raises `ValidationError` when validation fails. ([Pydantic Docs][S02-4])

### Attribute extraction

```python
class CompanyModel(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: int
    public_key: str

company = CompanyModel.model_validate(company_orm)
```

`model_validate()` can extract values from arbitrary object attributes when `from_attributes` is enabled by config or per-call parameter; this replaces the old v1 “ORM mode” / `from_orm()` path. ([Pydantic Docs][S02-1])

---

## 2.8.3 `Model.model_validate_json(...)`

Signature:

```python
@classmethod
def model_validate_json(
    cls,
    json_data: str | bytes | bytearray,
    strict: bool | None = None,
    extra: ExtraValues | None = None,
    context: Any | None = None,
    by_alias: bool | None = None,
    by_name: bool | None = None,
) -> Self: ...
```

Use:

```python
user = User.model_validate_json(b'{"id": "123", "name": "Ada"}')
```

Use cases:

```text
raw HTTP body
queue message bytes
file contents already read as bytes
LLM JSON output string
high-throughput JSON validation
```

Pydantic documents `model_validate_json()` as validating JSON strings/bytes/bytearray and notes that it is generally faster for incoming JSON payloads than manually parsing JSON into a dictionary first. ([Pydantic Docs][S02-4])

Agent rule:

```text
raw JSON bytes/string → model_validate_json
already-decoded dict  → model_validate
stringly dict values  → model_validate_strings
```

---

## 2.8.4 `Model.model_validate_strings(...)`

Signature:

```python
@classmethod
def model_validate_strings(
    cls,
    obj: Any,
    strict: bool | None = None,
    extra: ExtraValues | None = None,
    context: Any | None = None,
    by_alias: bool | None = None,
    by_name: bool | None = None,
) -> Self: ...
```

Use:

```python
from datetime import datetime

class Event(BaseModel):
    id: int
    at: datetime

event = Event.model_validate_strings(
    {"id": "123", "at": "2026-05-11T12:00:00"}
)
```

Use cases:

```text
environment-style dictionaries
form data
query parameters
headers
CLI argument maps
CSV rows
string-only config maps
```

`model_validate_strings()` validates a dictionary-like object with string keys and string values in JSON mode so strings can be coerced into the correct field types. ([Pydantic Docs][S02-4])

---

## 2.9 Entry-point selection matrix

| Input                         | Recommended API                                             | Reason                       |
| ----------------------------- | ----------------------------------------------------------- | ---------------------------- |
| keyword args in Python        | `Model(...)`                                                | ergonomic constructor        |
| dict / mapping                | `Model.model_validate(obj)`                                 | explicit validation call     |
| ORM / object attributes       | `Model.model_validate(obj, from_attributes=True)` or config | attribute extraction         |
| JSON bytes/string             | `Model.model_validate_json(raw)`                            | avoids separate `json.loads` |
| env/form/query/CSV string map | `Model.model_validate_strings(obj)`                         | JSON-mode string coercion    |
| trusted prevalidated data     | `Model.model_construct(...)`                                | skips validation; dangerous  |
| arbitrary non-model type      | `TypeAdapter(T).validate_python(...)`                       | no `BaseModel` wrapper       |

Pydantic documents three validation modes: Python, JSON, and strings. Python mode is used by the constructor and `model_validate`; JSON and strings modes use the dedicated methods. ([Pydantic Docs][S02-1])

---

## 2.10 Custom `__init__` warning

```python
class Bad(BaseModel):
    x: int

    def __init__(self, x):
        self.x = x  # validation bypass / broken behavior
```

Preferred:

```python
class Good(BaseModel):
    x: int

    def model_post_init(self, context) -> None:
        ...
```

Pydantic provides a default `__init__` that delegates validation to `pydantic-core`. Defining custom `__init__` is not recommended because validation parameters such as strictness, extra behavior, and context can be lost; use validators or `model_post_init()` for post-initialization actions. ([Pydantic Docs][S02-1])

Agent rule:

```text
Never generate custom __init__ for BaseModel unless there is a strong, tested reason.
Use:
  field_validator
  model_validator
  model_post_init
  computed_field
instead.
```

---

## 2.11 `ValidationError` model

```python
from pydantic import ValidationError

try:
    User.model_validate({"id": "bad"})
except ValidationError as exc:
    errors = exc.errors()
```

Behavior:

```text
one ValidationError
contains all discovered validation errors
machine-readable .errors()
JSON-serializable .json()
human-readable str(exc)
```

Pydantic raises a single `ValidationError` whenever it finds errors in the data being validated; that one exception contains information about all validation errors. ([Pydantic Docs][S02-1])

Agent test rule:

```python
assert exc.errors()[0]["type"] in {"int_parsing", "int_type"}
```

Avoid brittle tests:

```python
assert "Input should be a valid integer" in str(exc)
```

---

## 2.12 Extra data behavior

Default:

```python
class M(BaseModel):
    x: int

m = M(x=1, y="ignored")
assert m.model_dump() == {"x": 1}
```

Configured:

```python
from pydantic import ConfigDict

class AllowExtra(BaseModel):
    model_config = ConfigDict(extra="allow")
    x: int

m = AllowExtra(x=1, y="kept")
assert m.model_dump() == {"x": 1, "y": "kept"}
```

Modes:

```text
extra="ignore"  default; discard unknown input keys
extra="forbid"  reject unknown input keys
extra="allow"   store unknown keys in __pydantic_extra__
```

The model docs define these three extra-data modes and note that validation methods can override extra behavior per validation call. ([Pydantic Docs][S02-1])

Deployment default:

```python
class ApiInput(BaseModel):
    model_config = ConfigDict(extra="forbid")
```

Agent rule:

```text
Public API input:        extra="forbid" unless forward compatibility requires ignore.
Webhook/event consumer:  extra="ignore" or allow versioned extension fields deliberately.
Internal DTO:            extra="forbid".
LLM output parser:       extra="forbid" for strict tool contracts; ignore for tolerant extraction.
```

---

## 2.13 Instance API: `.model_dump()`

Signature core:

```python
def model_dump(
    mode: Literal["json", "python"] | str = "python",
    include: IncEx | None = None,
    exclude: IncEx | None = None,
    context: Any | None = None,
    by_alias: bool | None = None,
    exclude_unset: bool = False,
    exclude_defaults: bool = False,
    exclude_none: bool = False,
    exclude_computed_fields: bool = False,
    round_trip: bool = False,
    warnings: bool | Literal["none", "warn", "error"] = True,
    fallback: Callable[[Any], Any] | None = None,
    serialize_as_any: bool = False,
    polymorphic_serialization: bool | None = None,
) -> dict[str, Any]: ...
```

Use:

```python
payload = user.model_dump()
```

`model_dump()` returns a dictionary representation of the model; `mode="python"` may contain non-JSON-serializable Python objects, while `mode="json"` produces only JSON-serializable types. It supports include/exclude, context, aliasing, unset/default/none exclusion, round-trip dumping, serialization error handling, fallback serialization, and duck-typing serialization. ([Pydantic Docs][S02-4])

### Common API response dump

```python
user.model_dump(
    mode="json",
    by_alias=True,
    exclude_none=True,
)
```

### PATCH payload dump

```python
patch.model_dump(exclude_unset=True)
```

### Persist all explicit values except nulls

```python
obj.model_dump(exclude_none=True)
```

### Round-trip dump

```python
obj.model_dump(round_trip=True)
```

Agent rule:

```text
Default internal dict:          model_dump()
JSON-compatible Python dict:    model_dump(mode="json")
Public API response:            model_dump(mode="json", by_alias=True, exclude_none=True)
PATCH/update payload:           model_dump(exclude_unset=True)
Stable revalidation payload:    model_dump(round_trip=True)
```

---

## 2.14 Instance API: `.model_dump_json()`

Signature core:

```python
def model_dump_json(
    indent: int | None = None,
    ensure_ascii: bool = False,
    include: IncEx | None = None,
    exclude: IncEx | None = None,
    context: Any | None = None,
    by_alias: bool | None = None,
    exclude_unset: bool = False,
    exclude_defaults: bool = False,
    exclude_none: bool = False,
    exclude_computed_fields: bool = False,
    round_trip: bool = False,
    warnings: bool | Literal["none", "warn", "error"] = True,
    fallback: Callable[[Any], Any] | None = None,
    serialize_as_any: bool = False,
    polymorphic_serialization: bool | None = None,
) -> str: ...
```

Use:

```python
json_payload = user.model_dump_json()
```

`model_dump_json()` returns a JSON string generated through Pydantic’s serializer; output is compact by default when `indent=None`, and `ensure_ascii=False` leaves non-ASCII characters unescaped by default. ([Pydantic Docs][S02-4])

Agent rule:

```text
Need string bytes for wire/log/cache: model_dump_json()
Need Python dict for framework response: model_dump(mode="json")
Need pretty debug output: model_dump_json(indent=2)
Need ASCII-only output: model_dump_json(ensure_ascii=True)
```

Test rule:

```python
assert json.loads(model.model_dump_json()) == expected
```

Do not snapshot compact JSON whitespace unless whitespace itself is the contract.

---

## 2.15 Instance API: `.model_copy()`

Signature:

```python
def model_copy(
    update: Mapping[str, Any] | None = None,
    deep: bool = False,
) -> Self: ...
```

Use:

```python
new_user = user.model_copy(update={"name": "Grace"})
deep_user = user.model_copy(deep=True)
```

`model_copy()` returns a shallow copy by default; `deep=True` performs a deep copy. The `update` data is **not validated** before creating the copied model, so update values must be trusted. The underlying `__dict__` is copied, which can matter if cached properties or non-field data are stored there. ([Pydantic Docs][S02-4])

Agent rule:

```text
Trusted internal patch:     model_copy(update=...)
Untrusted external patch:   Model.model_validate({**old.model_dump(), **patch})
Nested mutable structures:  model_copy(deep=True)
```

Safe untrusted update pattern:

```python
data = old.model_dump()
data.update(untrusted_patch)
new = User.model_validate(data)
```

---

## 2.16 Instance API: `.model_fields_set`

```python
class User(BaseModel):
    id: int
    name: str = "Jane Doe"
    age: int | None = None

u = User(id=1, age=None)

assert u.model_fields_set == {"id", "age"}
```

Meaning:

```text
fields explicitly provided during validation/construction
not the same as non-default fields
not the same as non-null fields
critical for PATCH semantics
```

The docs show `model_fields_set` as the way to inspect which field names were explicitly set during initialization; model instances expose `model_fields_set` as an attribute. ([Pydantic Docs][S02-1])

PATCH response pattern:

```python
patch_data = patch.model_dump(exclude_unset=True)
```

`exclude_unset=True` is driven by explicit-set tracking and excludes fields not explicitly set. ([Pydantic Docs][S02-4])

---

## 2.17 Class API: `.model_fields`

```python
for field_name, field_info in User.model_fields.items():
    ...
```

Meaning:

```text
schema metadata
class-level
field definitions, not values
FieldInfo objects
```

`model_fields` maps field names to their `FieldInfo` definitions. Pydantic also documents it in the model-methods list as a class mapping between field names and field definitions. ([Pydantic Docs][S02-3])

Common agent uses:

```text
generate docs
inspect aliases
build mapping tables
detect required fields
introspect constraints/metadata
avoid hardcoding field lists
```

Avoid:

```python
instance.model_fields  # deprecated access pattern
```

---

## 2.18 Model lifecycle

## 2.18.1 Phase 1 — class definition

```python
class Invoice(BaseModel):
    id: int
    total_cents: int
```

Lifecycle facts:

```text
Python executes class body
Pydantic collects annotations/defaults/config
field definitions become FieldInfo metadata
core schema prepared or deferred
validators/serializers bound
```

Pydantic states that when a model class is defined, it analyzes the class body, evaluates annotations, and gathers information required for validation and serialization into a core schema. ([Pydantic Docs][S02-1])

Deployment implication:

```text
Large model graphs increase import-time cost.
Avoid importing every schema module at CLI/serverless cold start.
Use lazy imports for route/command-specific schema groups when needed.
```

---

## 2.18.2 Phase 2 — schema building

```text
annotation graph
  → pydantic-core schema
  → validator / serializer
```

Forward references may prevent a model from being complete at initial class creation time. When symbols are unavailable, `model_rebuild()` may be required after all referenced types are defined. ([Pydantic Docs][S02-1])

---

## 2.18.3 Phase 3 — instance validation

```python
invoice = Invoice.model_validate({"id": "1", "total_cents": "2500"})
```

Validation modes:

```text
Python mode: constructor / model_validate
JSON mode:   model_validate_json
strings mode: model_validate_strings
```

The models docs explicitly describe Python, JSON, and strings validation modes and list the corresponding APIs. ([Pydantic Docs][S02-1])

---

## 2.18.4 Phase 4 — serialization

```python
invoice.model_dump()
invoice.model_dump(mode="json")
invoice.model_dump_json()
```

Serialization modes:

```text
Python dict:            model_dump()
JSON-compatible dict:   model_dump(mode="json")
JSON string:            model_dump_json()
```

`model_dump()` recursively serializes a model to a dictionary and supports many customization parameters; `model_dump_json()` returns a JSON string representation. ([Pydantic Docs][S02-1])

---

## 2.18.5 Phase 5 — rebuild forward references

```python
class Foo(BaseModel):
    x: "Bar"

class Bar(BaseModel):
    pass

Foo.model_rebuild()
```

`model_rebuild()` tries to rebuild the pydantic-core schema for a model. It may be needed when annotations contain unresolved `ForwardRef`s; in v2 it replaces `update_forward_refs()`. When called on the outermost model, it builds the core schema for the whole nested graph, so all nested referenced types must be ready. ([Pydantic Docs][S02-4])

Signature:

```python
@classmethod
def model_rebuild(
    cls,
    force: bool = False,
    raise_errors: bool = True,
    _parent_namespace_depth: int = 2,
    _types_namespace: MappingNamespace | None = None,
) -> bool | None: ...
```

Return behavior:

```text
None   schema already complete; rebuild not needed
True   rebuild required and successful
False  rebuild required and failed when raise_errors=False
```

The API docs define this signature and return behavior. ([Pydantic Docs][S02-4])

---

## 2.19 Forward-reference deployment patterns

### Same module, simple case

```python
class Node(BaseModel):
    name: str
    children: list["Node"] = []
```

Pydantic often resolves this automatically.

### Cross-module or dynamic loading

```python
# module_a.py
class A(BaseModel):
    b: "B"

# module_b.py
class B(BaseModel):
    value: int

# after imports are complete
A.model_rebuild(_types_namespace={"B": B})
```

Agent rule:

```text
If schema generation or validation says class is not fully defined:
  1. ensure all referenced classes are imported
  2. call model_rebuild()
  3. pass _types_namespace for non-obvious namespaces
```

Pydantic’s docs show `Foo` failing to emit JSON Schema before `Bar` exists, then succeeding after `Bar` is defined and `Foo.model_rebuild()` is called. ([Pydantic Docs][S02-1])

---

## 2.20 `model_construct()` warning

Although not in the requested API list, it is part of the core lifecycle surface:

```python
trusted = User.model_construct(id=123, name="Ada")
```

Meaning:

```text
skips validation
accepts trusted/prevalidated values
can create invalid instances if misused
```

The BaseModel API says `model_construct()` accepts trusted or prevalidated data and performs no validation; its extra-field behavior differs from validated construction, especially for `extra="forbid"`. ([Pydantic Docs][S02-4])

Agent rule:

```text
Only use model_construct for:
  deserialization from trusted cache
  internal performance-critical reconstruction
  test fixtures where invalid states are intentional
  framework internals

Never use for:
  API payloads
  LLM outputs
  env vars
  user files
  queue messages
  DB rows from untrusted sources
```

---

## 2.21 Complete syntax reference example

```python
from __future__ import annotations

from datetime import datetime
from typing import Any
from uuid import UUID

from pydantic import BaseModel, ConfigDict, Field, ValidationError


class ApiModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        validate_assignment=False,
        populate_by_name=True,
    )


class UserIn(ApiModel):
    email: str
    age: int | None = None
    tags: list[str] = Field(default_factory=list)


class UserOut(ApiModel):
    id: UUID
    email: str
    age: int | None = None
    created_at: datetime


raw_python: dict[str, Any] = {
    "email": "ada@example.com",
    "age": "42",
}

user_in = UserIn.model_validate(raw_python)
assert user_in.model_fields_set == {"email", "age"}

json_text = b"""
{
  "id": "f9de8f7c-fb2f-4e96-8b8d-1b4f3cced63f",
  "email": "ada@example.com",
  "created_at": "2026-05-11T12:00:00Z"
}
"""
user_out = UserOut.model_validate_json(json_text)

api_dict = user_out.model_dump(mode="json", exclude_none=True)
api_json = user_out.model_dump_json(exclude_none=True)

copy = user_out.model_copy(update={"email": "grace@example.com"})

try:
    UserIn.model_validate({"email": "x", "age": "not an int"})
except ValidationError as exc:
    error_types = [err["type"] for err in exc.errors()]
```

---

## 2.22 BaseModel API decision rules for programming agents

```text
Generate BaseModel when:
  object has named fields
  API/event/settings/DB-row shape matters
  JSON Schema may be useful
  serialization is needed

Use TypeAdapter instead when:
  top-level type is list[T], dict[K,V], union, primitive, tuple, TypedDict
  no named record object is needed

Use RootModel when:
  top-level value needs model identity
  schema/serialization hooks should live on a class
```

---

## 2.23 Best-practice deployment advisory

### Public API input model

```python
class CreateUser(BaseModel):
    model_config = ConfigDict(extra="forbid")

    email: str
    age: int | None = None
```

Use `extra="forbid"` for fail-fast public contracts.

### Public API output model

```python
class UserResponse(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: UUID
    email: str
```

Use `from_attributes=True` when validating ORM/domain objects into response DTOs.

### PATCH model

```python
class PatchUser(BaseModel):
    email: str | None = None
    age: int | None = None

patch = PatchUser.model_validate(payload)
update = patch.model_dump(exclude_unset=True)
```

Use `model_fields_set` / `exclude_unset=True` to distinguish omitted from explicit `null`.

### LLM output model

```python
class ToolCall(BaseModel):
    model_config = ConfigDict(extra="forbid")

    name: str
    arguments: dict[str, object]
```

Use strict extra-field policy when the model represents an executable tool contract.

---

## 2.24 Anti-patterns

### Anti-pattern: using nullable as missing-permitted

```python
class M(BaseModel):
    x: str | None  # required
```

Fix:

```python
class M(BaseModel):
    x: str | None = None
```

### Anti-pattern: treating `.model_copy(update=...)` as validation

```python
safe = existing.model_copy(update=untrusted_payload)  # update not validated
```

Fix:

```python
safe = type(existing).model_validate({
    **existing.model_dump(),
    **untrusted_payload,
})
```

### Anti-pattern: `dict(model)` for recursive serialization

```python
dict(model)
```

Fix:

```python
model.model_dump()
```

The docs note that `dict(model)` provides a dictionary, but nested fields are not recursively converted into dictionaries; `model_dump()` recursively serializes and offers customization arguments. ([Pydantic Docs][S02-1])

### Anti-pattern: custom `__init__`

```python
class M(BaseModel):
    x: int

    def __init__(self, x):
        ...
```

Fix:

```python
def model_post_init(self, context):
    ...
```

Pydantic explicitly discourages custom `__init__` because validation parameters can be lost. ([Pydantic Docs][S02-1])

---

## 2.25 Test matrix

```python
def test_required_nullable_semantics():
    class M(BaseModel):
        x1: int
        x2: int | None
        x3: int = 1
        x4: int | None = None

    assert M.model_validate({"x1": 1, "x2": None}).model_dump() == {
        "x1": 1,
        "x2": None,
        "x3": 1,
        "x4": None,
    }

    with pytest.raises(ValidationError):
        M.model_validate({"x2": None})  # x1 missing

    with pytest.raises(ValidationError):
        M.model_validate({"x1": 1})  # x2 missing
```

```python
def test_patch_semantics():
    class Patch(BaseModel):
        x: int | None = None

    p1 = Patch.model_validate({})
    p2 = Patch.model_validate({"x": None})

    assert p1.model_fields_set == set()
    assert p2.model_fields_set == {"x"}
    assert p1.model_dump(exclude_unset=True) == {}
    assert p2.model_dump(exclude_unset=True) == {"x": None}
```

```python
def test_copy_update_untrusted_requires_validation():
    class M(BaseModel):
        x: int

    m = M(x=1)
    bad = m.model_copy(update={"x": "not validated"})
    assert bad.x == "not validated"

    with pytest.raises(ValidationError):
        M.model_validate({**m.model_dump(), "x": "not validated"})
```

---

## 2.26 Value case

```text
BaseModel value =
  declarative class syntax
  runtime validation
  controlled coercion
  structured errors
  instance attribute access
  recursive nested models
  class-level schema introspection
  Python dict serialization
  JSON string serialization
  JSON Schema generation
  explicit set-field tracking
  forward-reference rebuild support
```

Operationally: use `BaseModel` at named-object boundaries, select the validation entrypoint based on input mode, preserve omitted-vs-null semantics with `model_fields_set` / `exclude_unset`, and treat serialization methods as explicit API-surface tools rather than incidental conversions.

[S02-1]: https://docs.pydantic.dev/latest/concepts/models/ "Models | Pydantic Docs"
[S02-2]: https://docs.pydantic.dev/latest/migration/ "Migration Guide | Pydantic Docs"
[S02-3]: https://docs.pydantic.dev/latest/concepts/fields/ "Fields | Pydantic Docs"
[S02-4]: https://docs.pydantic.dev/latest/api/base_model/ "BaseModel | Pydantic Docs"


# Pydantic Advanced — 3) Field system and `Field(...)` — Pydantic v2 deep dive

Style target: advanced, sectioned, agent-oriented technical reference. 

`Field()` is Pydantic’s primary per-field configuration primitive: defaults, factories, validation constraints, strictness, aliases, JSON Schema metadata, deprecation, immutability, exclusion, dataclass-specific constructor behavior, and representation control. The `Annotated[...]` pattern lets agents attach Pydantic metadata to a type without changing the static type that checkers see. ([Pydantic Docs][S03-1])

---

## 3.0 Field mental model

```text
Python annotation
  + default / Field(...)
  + Annotated metadata
  + model_config defaults
  → FieldInfo
  → core schema
  → validation behavior
  → serialization behavior
  → JSON Schema metadata
```

`FieldInfo` is the internal/public metadata object representing a model or dataclass field; it is used whether or not `Field()` appears explicitly. It exposes field information but should not be directly instantiated or mutated in normal code. ([Pydantic Docs][S03-2])

---

## 3.1 Field declaration forms

### Assignment form

```python
from pydantic import BaseModel, Field

class User(BaseModel):
    name: str = Field(frozen=True)
```

Important: assignment to `Field(...)` does **not** imply a default value. The field above is still required because no `default` or `default_factory` was supplied. Pydantic documents this explicitly and discourages using `Field(..., ...)` with ellipsis because it interacts poorly with static type checkers. ([Pydantic Docs][S03-1])

Required, explicit but type-checker-unfriendly:

```python
class User(BaseModel):
    name: str = Field(..., frozen=True)
```

Preferred required declaration with metadata:

```python
from typing import Annotated
from pydantic import BaseModel, Field

class User(BaseModel):
    name: Annotated[str, Field(frozen=True)]
```

---

### `Annotated[...]` form

```python
from typing import Annotated
from pydantic import BaseModel, Field, WithJsonSchema

class Model(BaseModel):
    name: Annotated[
        str,
        Field(strict=True, min_length=1),
        WithJsonSchema({"x-ui-widget": "text"}),
    ]
```

Static type checkers still see `name` as `str`; Pydantic consumes the metadata for validation/schema behavior. Advantages: no fake-assignment confusion, unlimited metadata stacking, reusable constrained types, item-level metadata in containers. Caveat: static type checkers use `default`, `default_factory`, and `alias` from assignment-form `Field(...)` to synthesize `__init__`; they do **not** understand those constructor-affecting arguments inside the `Annotated` pattern. ([Pydantic Docs][S03-1])

Agent rule:

```text
Use assignment form for:
  default
  default_factory
  alias important to __init__ typing

Use Annotated form for:
  constraints
  strictness
  JSON Schema metadata
  reusable constrained aliases
  item-level constraints
```

---

## 3.2 Requiredness vs default

```python
class M(BaseModel):
    a: str                         # required
    b: str = "x"                   # not required, plain default
    c: str = Field(default="x")    # not required, Field default
    d: str = Field(frozen=True)    # required, frozen metadata only
```

Default values can be supplied by normal assignment or `Field(default=...)`; both make a field not required. Pydantic v2 removed v1’s implicit `None` defaults for `Any` and `Optional`-wrapped fields. ([Pydantic Docs][S03-1])

---

## 3.3 Plain defaults

```python
class User(BaseModel):
    name: str = "John Doe"
    age: int = Field(default=20)
```

Equivalent requiredness:

```text
name: str = "John Doe"        → default literal
age: int = Field(default=20)  → default via Field
```

Deployment rule:

```text
Small immutable defaults:   direct assignment.
Semantic/default metadata:  Field(default=..., description=..., examples=...).
Dynamic defaults:           default_factory.
Validated defaults:         validate_default=True.
```

---

## 3.4 `default_factory`

### Zero-argument factory

```python
from uuid import uuid4
from pydantic import BaseModel, Field

class User(BaseModel):
    id: str = Field(default_factory=lambda: uuid4().hex)
```

`default_factory` may be a zero-argument callable called to produce a default value. ([Pydantic Docs][S03-1])

---

### Validated-data-aware factory

```python
from pydantic import BaseModel, EmailStr, Field

class User(BaseModel):
    email: EmailStr
    username: str = Field(default_factory=lambda data: data["email"])
```

A default factory can take one required argument; Pydantic passes a dictionary of already validated data. This dictionary only contains fields validated earlier according to model field order, so moving `username` above `email` breaks the factory. ([Pydantic Docs][S03-1])

Agent rule:

```text
default_factory(data) is order-sensitive.
Only read earlier fields.
Never read later fields.
Never perform DB/network IO.
Keep factories deterministic and cheap.
```

---

## 3.5 Validating defaults

```python
from pydantic import BaseModel, Field, ValidationError

class User(BaseModel):
    age: int = Field(default="twelve", validate_default=True)

try:
    User()
except ValidationError as exc:
    ...
```

By default, Pydantic trusts default values and does not validate them. Set `Field(validate_default=True)` or model config `validate_default=True` to validate defaults every time an instance is created. This is a performance/security tradeoff. ([Pydantic Docs][S03-1])

Deployment policy:

```text
Library/public models:         validate_default=True for non-trivial defaults.
Settings models:               validate defaults; env/default bugs should fail early.
Enum value extraction defaults: validate_default=True when use_enum_values is active.
Hot-path internal DTOs:         trusted defaults may skip validation.
Generated code:                validate defaults unless performance budget says otherwise.
```

---

## 3.6 Mutable default behavior

```python
from pydantic import BaseModel

class Model(BaseModel):
    item_counts: list[dict[str, int]] = [{}]
```

Unlike dataclasses, Pydantic allows mutable defaults. If the default value is not hashable, Pydantic deep-copies it when creating each model instance, avoiding shared mutable default reuse. ([Pydantic Docs][S03-1])

Preferred explicit factory:

```python
class Model(BaseModel):
    item_counts: list[dict[str, int]] = Field(default_factory=list)
```

Agent rule:

```text
Use default_factory for mutable defaults anyway.
Reason: explicit intent, cheaper for large defaults, clearer to readers, dataclass-aligned habit.
```

---

## 3.7 Numeric constraints

### Syntax

```python
from decimal import Decimal
from typing import Annotated
from pydantic import BaseModel, Field

PositiveInt = Annotated[int, Field(gt=0)]
Percent = Annotated[float, Field(ge=0.0, le=1.0)]
EvenInt = Annotated[int, Field(multiple_of=2)]

class Product(BaseModel):
    quantity: PositiveInt
    discount: Percent = 0.0
    batch_size: EvenInt
    price: Decimal = Field(gt=Decimal("0"), max_digits=8, decimal_places=2)
```

`Field()` supports numeric constraints including `gt`, `ge`, `lt`, `le`, `multiple_of`, `allow_inf_nan`, `max_digits`, and `decimal_places`; Pydantic’s API docs state that `gt/ge/lt/le/multiple_of` apply to numbers, `max_digits/decimal_places` apply to decimal-like numeric constraints, and constraints affect validation plus JSON Schema output where applicable. ([Pydantic Docs][S03-2])

### Constraint semantics

```text
gt            value > bound
ge            value >= bound
lt            value < bound
le            value <= bound
multiple_of   value % n == 0 conceptually
max_digits    max total decimal digits
decimal_places max digits after decimal point
```

### Nullable numeric constraint

```python
class M(BaseModel):
    positive_or_null: int | None = Field(gt=0)
```

When constraints are applied to a union where one member is `None`, Pydantic applies the constraint to the remaining non-`None` type(s). ([Pydantic Docs][S03-1])

---

## 3.8 String constraints

```python
from typing import Annotated
from pydantic import BaseModel, Field

Slug = Annotated[
    str,
    Field(
        min_length=1,
        max_length=128,
        pattern=r"^[a-z0-9]+(?:-[a-z0-9]+)*$",
    ),
]

class Article(BaseModel):
    slug: Slug
```

`Field()` supports `min_length`, `max_length`, and `pattern`; the API docs describe `pattern` as a string regular expression for strings, while `min_length` and `max_length` are length constraints. ([Pydantic Docs][S03-2])

Agent rule:

```text
Use Field(pattern=...) for syntactic string shape.
Use validators for semantic string checks.
Use constrained aliases for repeated string contracts.
```

Bad:

```python
class M(BaseModel):
    username: str = Field(pattern=r".*")  # too weak, false sense of validation
```

Better:

```python
Username = Annotated[str, Field(min_length=3, max_length=32, pattern=r"^[a-zA-Z0-9_]+$")]
```

---

## 3.9 Collection constraints

### Collection length

```python
from typing import Annotated
from pydantic import BaseModel, Field

class Batch(BaseModel):
    ids: list[int] = Field(min_length=1, max_length=1000)
```

`min_length` and `max_length` are documented as minimum and maximum length for iterables, not just strings. ([Pydantic Docs][S03-2])

---

### Item-level constraints

```python
class Batch(BaseModel):
    ids: list[Annotated[int, Field(gt=0)]]
```

Use `Annotated` inside the container type to constrain elements. Pydantic’s fields docs show `list[Annotated[int, Field(gt=0)]]` as the correct way to validate each integer item. ([Pydantic Docs][S03-1])

Agent rule:

```text
Field(min_length=1) on list[T]          → constrains collection length
list[Annotated[T, Field(...)]          → constrains each element
Annotated[list[T], Field(...)]         → constrains top-level list
```

Combined:

```python
PositiveIds = Annotated[
    list[Annotated[int, Field(gt=0)]],
    Field(min_length=1, max_length=1000),
]
```

---

## 3.10 Constraint placement hazards with `Annotated`

Bad:

```python
class Model(BaseModel):
    field_bad: Annotated[int, Field(deprecated=True)] | None = None
```

Good:

```python
class Model(BaseModel):
    field_ok: Annotated[int | None, Field(deprecated=True)] = None
```

Metadata must be attached to the correct layer. Pydantic warns that field-level metadata attached only to the `int` member of a union may not affect the outer field; attach field metadata to the top-level field type. ([Pydantic Docs][S03-1])

Agent rule:

```text
Type-level constraints:      Annotated[int, Field(gt=0)]
Field-level metadata:        Annotated[int | None, Field(deprecated=True)]
Container-item constraints:  list[Annotated[int, Field(gt=0)]]
Container-level metadata:    Annotated[list[int], Field(min_length=1)]
```

---

## 3.11 Strict fields

```python
class User(BaseModel):
    name: str = Field(strict=True)
    age: int = Field(strict=False)
```

`Field(strict=True)` enables strict validation for that specific field. Pydantic’s fields docs show a strict `name` field and lax `age` field, where a string `"42"` can still coerce into `age` when strict is false. ([Pydantic Docs][S03-1])

Agent rule:

```text
Security-sensitive scalar: Field(strict=True)
Public ergonomic API:      default lax or selective strict
Env/settings input:        usually lax; env vars are strings
LLM output:                lax ingestion + explicit post-normalization where useful
```

---

## 3.12 Field metadata for JSON Schema

### Core schema-facing metadata

```python
class Product(BaseModel):
    sku: str = Field(
        title="SKU",
        description="Canonical stock-keeping unit.",
        examples=["ABC-123"],
        json_schema_extra={"x-ui-widget": "sku-input"},
    )
```

Field parameters used exclusively to customize generated JSON Schema include `title`, `description`, `examples`, and `json_schema_extra`. ([Pydantic Docs][S03-1])

### API docs / OpenAPI value case

```text
title              short human-readable label
description        field-level API documentation
examples           sample values for documentation/testing
json_schema_extra  extension keywords, UI hints, vendor metadata
```

### `json_schema_extra` as dict

```python
class Product(BaseModel):
    sku: str = Field(json_schema_extra={"x-internal-index": "sku"})
```

### `json_schema_extra` as callable

```python
def remove_default(schema: dict) -> None:
    schema.pop("default", None)

class Model(BaseModel):
    a: int = Field(default=1, json_schema_extra=remove_default)
```

Pydantic JSON Schema docs show `json_schema_extra` as either a dictionary adding JSON Schema keys or a callable mutating the generated schema. Starting in v2.9, Pydantic merges `json_schema_extra` dictionaries from annotated types; mixing dict and callable specifications is not supported. ([Pydantic Docs][S03-3])

---

## 3.13 Field deprecation

```python
from typing import Annotated
from pydantic import BaseModel, Field

class Model(BaseModel):
    old_field: Annotated[int, Field(deprecated="Use new_field instead.")]
    new_field: int
```

Effects:

```text
access old_field → runtime deprecation warning
JSON Schema property → deprecated: true
```

The `deprecated` field parameter can be a string, boolean, or `warnings.deprecated` / `typing_extensions.deprecated` instance; accessing a deprecated field emits a runtime warning, and generated JSON Schema includes the `deprecated` keyword. ([Pydantic Docs][S03-1])

### Validator caveat

```python
import warnings
from typing_extensions import Self
from pydantic import BaseModel, Field, model_validator

class Model(BaseModel):
    deprecated_field: int = Field(deprecated="Use replacement.")

    @model_validator(mode="after")
    def validate_model(self) -> Self:
        with warnings.catch_warnings():
            warnings.simplefilter("ignore", DeprecationWarning)
            _ = self.deprecated_field
        return self
```

Accessing deprecated fields inside validators still emits the warning unless explicitly suppressed. Pydantic documents using `warnings.catch_warnings()` for this case. ([Pydantic Docs][S03-1])

---

## 3.14 Field-level immutability: `Field(frozen=True)`

```python
from pydantic import BaseModel, Field, ValidationError

class User(BaseModel):
    id: int = Field(frozen=True)
    name: str

user = User(id=1, name="Ada")

try:
    user.id = 2
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "frozen_field"
```

`Field(frozen=True)` prevents assignment to that field after model creation; failed assignment raises a validation error with a frozen-field error type. ([Pydantic Docs][S03-1])

Deployment rule:

```text
Use field-level frozen for:
  database IDs
  tenant IDs
  externally assigned immutable identifiers
  creation timestamps
  cryptographic fingerprints

Do not use as:
  deep immutability guarantee for nested mutable objects
```

`frozen=True` blocks rebinding the field, not mutation of every nested object reachable through it.

---

## 3.15 Model-level immutability: `ConfigDict(frozen=True)`

```python
from pydantic import BaseModel, ConfigDict

class FrozenUser(BaseModel):
    model_config = ConfigDict(frozen=True)

    id: int
    name: str
```

Model-level `frozen=True` makes models faux-immutable by disallowing `__setattr__` and generating `__hash__()` when all attributes are hashable. Default is `False`. ([Pydantic Docs][S03-4])

Field vs model policy:

```text
Field(frozen=True):
  protect one field

ConfigDict(frozen=True):
  protect all fields
  maybe hashable model
  suitable for value objects / cache keys if all fields hashable
```

---

## 3.16 Assignment validation vs frozen

```python
from pydantic import BaseModel, ConfigDict

class User(BaseModel):
    model_config = ConfigDict(validate_assignment=True)

    name: str

u = User(name="Ada")
u.name = "Grace"  # revalidated
```

Assignment validation revalidates changes after model creation. Frozen fields/models reject assignment; `validate_assignment=True` validates assignment if assignment is allowed. Pydantic config docs state that by default data is validated on creation, not on mutation, and `validate_assignment=True` enables validation when changed. ([Pydantic Docs][S03-4])

Agent rule:

```text
frozen=True              → no rebinding
validate_assignment=True → rebinding allowed but checked
neither                  → rebinding allowed and not checked
```

---

## 3.17 Exclusion: `exclude=True`

```python
class User(BaseModel):
    name: str
    password_hash: str = Field(exclude=True)

u = User(name="Ada", password_hash="...")
assert u.model_dump() == {"name": "Ada"}
```

`Field(exclude=True)` excludes a field from serialization output. Field-level `exclude` and `exclude_if` can be configured at the field level, and field-level exclusion takes priority over `include` passed to serialization methods. ([Pydantic Docs][S03-1])

### Conditional exclusion

```python
class Transaction(BaseModel):
    id: int
    private_id: int = Field(exclude=True)
    value: int = Field(ge=0, exclude_if=lambda v: v == 0)
```

`exclude_if` is a value-based exclusion callable. Pydantic’s serialization docs show `exclude_if=lambda v: v == 0`; the fields docs note `exclude_if` was added in v2.12. ([Pydantic Docs][S03-5])

Deployment rule:

```text
Secrets/internal IDs:        exclude=True
Zero/empty/no-op payloads:   exclude_if=...
API response shaping:        prefer explicit output model + exclude as defense-in-depth
```

---

## 3.18 Representation: `repr=False`

```python
class User(BaseModel):
    name: str = Field(repr=True)
    password_hash: str = Field(repr=False)

user = User(name="Ada", password_hash="...")
print(user)  # password_hash omitted from representation
```

`repr` controls whether the field appears in the string representation of the model. ([Pydantic Docs][S03-1])

Agent rule:

```text
repr=False is logging hygiene, not serialization security.
For dumps/API output, use exclude=True.
For secret masking, use SecretStr / SecretBytes.
For both, combine intentionally.
```

---

## 3.19 `computed_field`

### Basic computed field

```python
from pydantic import BaseModel, computed_field

class Box(BaseModel):
    width: float
    height: float
    depth: float

    @computed_field
    @property
    def volume(self) -> float:
        return self.width * self.height * self.depth
```

`@computed_field` includes `property` or `cached_property` values in model serialization and in JSON Schema when schema generation is in serialization mode. Pydantic does not perform validation, cache invalidation, or other additional logic on the wrapped property. ([Pydantic Docs][S03-1])

### Alias and representation

```python
from pydantic import BaseModel, computed_field

class Square(BaseModel):
    width: float

    @computed_field
    @property
    def area(self) -> float:
        return round(self.width**2, 2)

    @computed_field(alias="the magic number", repr=False)
    @property
    def random_number(self) -> int:
        return 4
```

Computed fields support aliasing and `repr` control; the API docs show `alias=...` and `repr=False` on computed fields and demonstrate `model_dump_json(by_alias=True)`. ([Pydantic Docs][S03-2])

### Exclusion

```python
class Cart(BaseModel):
    subtotal: int
    discount: int = 0

    @computed_field(exclude_if=lambda v: v == 0)
    @property
    def discount_applied(self) -> int:
        return self.discount
```

Pydantic v2.13 documents `exclude_if` support for conditionally excluding computed fields from serialization. ([Pydantic Docs][S03-1])

### Override restriction

```python
class Parent(BaseModel):
    a: str

class Child(Parent):
    @computed_field
    @property
    def a(self) -> str:
        return "new a"  # TypeError
```

A computed field cannot override a real field from a parent model; Pydantic raises a `TypeError` for that incompatible override. ([Pydantic Docs][S03-2])

---

## 3.20 Field metadata inspection

```python
from typing import Annotated
from pydantic import BaseModel, Field, WithJsonSchema

class Model(BaseModel):
    a: Annotated[
        int,
        Field(gt=1),
        WithJsonSchema({"extra": "data"}),
        Field(alias="b"),
    ] = 1

info = Model.model_fields["a"]

assert info.annotation is int
assert info.alias == "b"
print(info.metadata)
```

`model_fields` maps field names to `FieldInfo` instances; `FieldInfo.metadata` contains collected metadata such as constraints and `WithJsonSchema`. Instance access to `model_fields` is deprecated in v2.11 and removed in v3; access via the class. ([Pydantic Docs][S03-1])

Agent use cases:

```text
documentation generation
schema diffing
model transformation
form generation
LLM tool schema construction
field-level policy audit
```

Avoid mutating `FieldInfo` in place; generate derived models explicitly.

---

## 3.21 Dataclass-specific `Field` parameters

```python
from pydantic import BaseModel, Field
from pydantic.dataclasses import dataclass

@dataclass
class Foo:
    bar: str
    baz: str = Field(init_var=True)
    qux: str = Field(kw_only=True)

class Model(BaseModel):
    foo: Foo
```

Some `Field()` parameters apply only to Pydantic dataclasses: `init`, `init_var`, and `kw_only`. `init_var` fields participate in construction but are not included in serialized output. ([Pydantic Docs][S03-1])

Agent rule:

```text
BaseModel fields:       ignore init/init_var/kw_only.
Pydantic dataclasses:   use init/init_var/kw_only for constructor surface control.
```

---

## 3.22 Full syntax pattern

```python
from __future__ import annotations

from decimal import Decimal
from typing import Annotated
from uuid import uuid4

from pydantic import BaseModel, ConfigDict, Field, computed_field


Slug = Annotated[
    str,
    Field(
        min_length=1,
        max_length=128,
        pattern=r"^[a-z0-9]+(?:-[a-z0-9]+)*$",
        description="URL-safe slug.",
        examples=["hello-world"],
    ),
]

PositiveCents = Annotated[int, Field(ge=1)]


class Product(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        validate_assignment=True,
    )

    id: str = Field(
        default_factory=lambda: uuid4().hex,
        frozen=True,
        description="Immutable product identifier.",
    )

    slug: Slug

    title: str = Field(
        min_length=1,
        max_length=200,
        title="Product title",
        description="Human-readable product name.",
        examples=["Organic Cotton T-Shirt"],
    )

    price_cents: PositiveCents

    tax_rate: Decimal = Field(
        default=Decimal("0.00"),
        ge=Decimal("0.00"),
        le=Decimal("1.00"),
        max_digits=4,
        decimal_places=2,
        validate_default=True,
    )

    internal_score: float = Field(default=0.0, exclude=True, repr=False)

    legacy_code: str | None = Field(
        default=None,
        deprecated="Use slug instead.",
        json_schema_extra={"x-removal-version": "3.0"},
    )

    @computed_field
    @property
    def price_dollars(self) -> str:
        return f"{self.price_cents / 100:.2f}"
```

---

## 3.23 Deployment advisory matrix

```text
Feature                     Use case                                 Deployment note
------------------------------------------------------------------------------------------------
Field(default=...)          static fallback                          defaults not validated unless configured
default_factory             dynamic fallback                         keep deterministic; avoid IO
default_factory(data)       derived default from earlier fields       field-order-sensitive
validate_default=True       catch bad defaults                       slower; safer
gt/ge/lt/le                 numeric local constraints                use validators for semantic constraints
min_length/max_length       string/list/iterable size                 attach at correct layer
pattern                     regex string shape                       not business validation
strict=True                 reject coercion                          useful at trust/security boundaries
title/description/examples  docs/OpenAPI/tool schema                  public contract text
json_schema_extra           vendor/UI/schema metadata                 dicts merge; avoid dict+callable mixing
deprecated                  migration signaling                      warns on access; sets schema deprecated
frozen=True field           immutable identifier                     rebinding blocked only
ConfigDict(frozen=True)     value object / hashable candidate         nested mutability still possible
exclude=True                serialization suppression                 takes priority over include
repr=False                  representation/log hygiene                not serialization security
computed_field              derived serialized values                 no validation/cache invalidation
```

---

## 3.24 Best-practice rules for LLM programming agents

```text
1. Prefer Annotated aliases for reusable constraints.
2. Prefer assignment form when default/default_factory/alias must be visible to type checkers.
3. Use default_factory for mutable/dynamic defaults.
4. Use validate_default=True for public/library/settings/security-sensitive defaults.
5. Attach constraints to the correct type layer.
6. Use Field(exclude=True) for data that must not serialize.
7. Use Field(repr=False) for data that must not appear in repr/logs.
8. Use SecretStr/SecretBytes for masking, not just repr=False.
9. Use frozen=True for identifiers, not for deep immutability.
10. Keep Field metadata pure; no business logic, no IO.
11. Use computed_field for derived output, not for validated input.
12. Treat JSON Schema metadata as public API if exposed through OpenAPI/tool schemas.
```

---

## 3.25 Anti-patterns

### Fake default confusion

```python
class M(BaseModel):
    name: str = Field(frozen=True)  # required, not defaulted
```

Preferred:

```python
class M(BaseModel):
    name: Annotated[str, Field(frozen=True)]
```

---

### Wrong metadata layer

```python
field_bad: Annotated[int, Field(deprecated=True)] | None = None
```

Preferred:

```python
field_ok: Annotated[int | None, Field(deprecated=True)] = None
```

---

### Item constraint accidentally placed on container

```python
ids: list[int] = Field(gt=0)  # wrong conceptually
```

Preferred:

```python
ids: list[Annotated[int, Field(gt=0)]]
```

---

### `repr=False` mistaken for secure serialization

```python
password_hash: str = Field(repr=False)
```

Preferred:

```python
password_hash: str = Field(exclude=True, repr=False)
```

---

### IO in `default_factory`

```python
token: str = Field(default_factory=lambda: requests.get("...").text)
```

Preferred:

```python
token: str

# inject from settings/service layer
```

---

## 3.26 Test harness patterns

### Requiredness and default

```python
def test_field_requiredness():
    class M(BaseModel):
        a: str = Field(frozen=True)
        b: str = "x"

    with pytest.raises(ValidationError):
        M()

    assert M(a="ok").b == "x"
```

### Default validation

```python
def test_validate_default():
    class M(BaseModel):
        x: int = Field(default="bad", validate_default=True)

    with pytest.raises(ValidationError) as exc:
        M()

    assert exc.value.errors()[0]["type"] == "int_parsing"
```

### Mutable default isolation

```python
def test_mutable_default_isolated():
    class M(BaseModel):
        xs: list[dict[str, int]] = [{}]

    a = M()
    b = M()
    a.xs[0]["v"] = 1

    assert b.xs == [{}]
```

### Frozen field

```python
def test_frozen_field():
    class M(BaseModel):
        x: int = Field(frozen=True)

    m = M(x=1)

    with pytest.raises(ValidationError) as exc:
        m.x = 2

    assert exc.value.errors()[0]["type"] == "frozen_field"
```

### Exclusion

```python
def test_exclude_field():
    class M(BaseModel):
        public: str
        secret: str = Field(exclude=True, repr=False)

    m = M(public="ok", secret="hidden")

    assert m.model_dump() == {"public": "ok"}
    assert "secret" not in repr(m)
```

### Computed field

```python
def test_computed_field_dump():
    class Box(BaseModel):
        w: int
        h: int

        @computed_field
        @property
        def area(self) -> int:
            return self.w * self.h

    assert Box(w=2, h=3).model_dump()["area"] == 6
```

---

## 3.27 Value case

```text
Field system value =
  required/default semantics
  dynamic defaults
  validated defaults
  local constraints
  strictness override
  schema metadata
  docs/OpenAPI/tool-schema quality
  controlled serialization
  controlled representation
  deprecation signaling
  field/model immutability
  reusable constrained aliases
  computed serialization fields
```

Operationally: encode structural, syntactic, local, deterministic field rules in `Field(...)`; keep business truth, external lookups, authorization, persistence, and workflow decisions outside Pydantic.

[S03-1]: https://docs.pydantic.dev/latest/concepts/fields/ "Fields | Pydantic Docs"
[S03-2]: https://docs.pydantic.dev/latest/api/fields/ "Fields | Pydantic Docs"
[S03-3]: https://docs.pydantic.dev/latest/concepts/json_schema/ "JSON Schema | Pydantic Docs"
[S03-4]: https://docs.pydantic.dev/latest/api/config/ "Configuration | Pydantic Docs"
[S03-5]: https://docs.pydantic.dev/latest/concepts/serialization/ "Serialization | Pydantic Docs"


# Pydantic Advanced — 4) Validation entrypoints and input modes — Pydantic v2

Style target: advanced, lexical-dense, agent-oriented reference. 

Pydantic validates data in **three modes**: **Python mode**, **JSON mode**, and **strings mode**. Python mode is used by `Model(...)` and `Model.model_validate(...)`; JSON and stringly-typed validation use dedicated methods: `model_validate_json(...)` and `model_validate_strings(...)`. The `model_validate_*` methods expose per-call controls such as strictness, extra-field handling, validation context, alias/name validation, and attribute extraction. ([Pydantic Docs][S04-1])

---

## 4.0 Entrypoint taxonomy

```text
Python object / dict / model instance / attribute object
  → Model.model_validate(...)

Raw JSON str | bytes | bytearray
  → Model.model_validate_json(...)

Dict-like structure with string keys and string values
  → Model.model_validate_strings(...)
```

Entrypoint selection:

```text
Model(...)
  ergonomic constructor
  Python-mode validation
  keyword arguments only

Model.model_validate(obj)
  explicit Python-object validation
  dict / model instance / attribute object
  supports strict / extra / from_attributes / context / alias-name controls

Model.model_validate_json(json_data)
  raw JSON string/bytes/bytearray validation
  JSON-mode validation
  usually preferred over json.loads(...) + model_validate(...)

Model.model_validate_strings(obj)
  nested dict of string keys/values
  JSON-mode coercion semantics for stringly data
  useful for env/form/query/header/CSV-like sources
```

Pydantic’s docs state that `model_validate_json()` validates JSON strings or bytes and is generally considered faster for incoming JSON payloads than manually parsing into a dictionary first; `model_validate_strings()` validates a nested dictionary with string keys and values in JSON mode so strings can be coerced into target types. ([Pydantic Docs][S04-1])

---

## 4.1 `Model.model_validate(...)`: Python-object validation

### Signature

```python
@classmethod
def model_validate(
    cls,
    obj: Any,
    strict: bool | None = None,
    extra: ExtraValues | None = None,
    from_attributes: bool | None = None,
    context: Any | None = None,
    by_alias: bool | None = None,
    by_name: bool | None = None,
) -> Self: ...
```

`model_validate()` validates a Python object and returns the validated model instance; parameters include `strict`, `extra`, `from_attributes`, `context`, `by_alias`, and `by_name`, and Pydantic raises `ValidationError` if the object cannot be validated. ([Pydantic Docs][S04-2])

### Basic dict validation

```python
from pydantic import BaseModel

class User(BaseModel):
    id: int
    name: str = "Jane"

user = User.model_validate({"id": "123"})
assert user.id == 123
assert user.name == "Jane"
```

### Explicit per-call strictness

```python
from pydantic import ValidationError

try:
    User.model_validate({"id": "123"}, strict=True)
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "int_type"
```

Default lax validation attempts useful coercion, such as converting string `"123"` into integer `123`; strict mode is less lenient and generally requires values to be instances of the target type, though JSON input has looser rules for some types such as dates. ([Pydantic Docs][S04-3])

### Extra behavior override

```python
from pydantic import BaseModel, ConfigDict, ValidationError

class Payload(BaseModel):
    model_config = ConfigDict(extra="allow")
    x: int

# model default allows extra
assert Payload.model_validate({"x": 1, "y": 2}).model_dump() == {"x": 1, "y": 2}

# per-call override forbids extra
try:
    Payload.model_validate({"x": 1, "y": 2}, extra="forbid")
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "extra_forbidden"
```

The `extra` argument to validation methods overrides the model’s configured `extra` behavior for that specific validation call. ([Pydantic Docs][S04-4])

### Attribute extraction

```python
from pydantic import BaseModel, ConfigDict

class CompanyOrm:
    def __init__(self, id: int, public_key: str) -> None:
        self.id = id
        self.public_key = public_key

class CompanyModel(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: int
    public_key: str

obj = CompanyOrm(id=123, public_key="abc")
company = CompanyModel.model_validate(obj)
assert company.id == 123
```

Attribute-based validation must be enabled through `from_attributes=True` in config or by the `from_attributes` parameter on `model_validate(...)`; Pydantic’s docs show this pattern for ORM-like objects and arbitrary class instances. ([Pydantic Docs][S04-1])

---

## 4.2 `Model.model_validate_json(...)`: raw JSON validation

### Signature

```python
@classmethod
def model_validate_json(
    cls,
    json_data: str | bytes | bytearray,
    strict: bool | None = None,
    extra: ExtraValues | None = None,
    context: Any | None = None,
    by_alias: bool | None = None,
    by_name: bool | None = None,
) -> Self: ...
```

`model_validate_json()` validates JSON data supplied as `str`, `bytes`, or `bytearray`; it supports strictness, extra-field behavior, validation context, and alias/name controls, and raises `ValidationError` if the JSON input or model validation fails. ([Pydantic Docs][S04-2])

### Basic JSON validation

```python
class Event(BaseModel):
    id: int
    kind: str

event = Event.model_validate_json(b'{"id": "123", "kind": "created"}')
assert event.id == 123
```

### JSON syntax failure vs model failure

```python
from pydantic import ValidationError

try:
    Event.model_validate_json(b'{"id": 123,')  # invalid JSON
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "json_invalid"

try:
    Event.model_validate_json(b'{"id": "bad", "kind": "created"}')
except ValidationError as exc:
    assert exc.errors()[0]["type"] in {"int_parsing", "int_type"}
```

### JSON-mode strictness nuance

```python
from datetime import date
from pydantic import BaseModel

class Dated(BaseModel):
    d: date

# JSON mode may accept date strings even under strict=True.
obj = Dated.model_validate_json(b'{"d": "2026-05-11"}', strict=True)
```

Pydantic documents that strict mode is generally less lenient, but looser rules may apply to JSON input; its strict-mode docs show `TypeAdapter(date).validate_json('"2000-01-01"', strict=True)` succeeding while Python strict validation of the same string fails. ([Pydantic Docs][S04-3])

### Deployment rule

```text
Raw HTTP body             → model_validate_json
Queue message bytes       → model_validate_json
File contents as bytes    → model_validate_json
LLM JSON string           → model_validate_json
Already-decoded dict      → model_validate
```

---

## 4.3 `Model.model_validate_strings(...)`: stringly-typed data

### Signature

```python
@classmethod
def model_validate_strings(
    cls,
    obj: Any,
    strict: bool | None = None,
    extra: ExtraValues | None = None,
    context: Any | None = None,
    by_alias: bool | None = None,
    by_name: bool | None = None,
) -> Self: ...
```

`model_validate_strings()` validates an object containing string data and returns the model instance; like the other validation methods, it accepts `strict`, `extra`, `context`, `by_alias`, and `by_name`. ([Pydantic Docs][S04-2])

### Stringly dict example

```python
from datetime import datetime
from pydantic import BaseModel

class User(BaseModel):
    id: int
    name: str
    signup_ts: datetime | None = None

user = User.model_validate_strings({
    "id": "123",
    "name": "Ada",
    "signup_ts": "2026-05-11T12:00:00",
})

assert user.id == 123
assert user.signup_ts == datetime(2026, 5, 11, 12, 0, 0)
```

Pydantic’s models docs show `model_validate_strings()` coercing string values into integers and datetimes, and they show a strict strings-mode datetime parse failure when the input lacks an accepted datetime separator. ([Pydantic Docs][S04-1])

### Recommended input classes

```text
os.environ-like mapping
dotenv-derived mapping
HTTP query parameters
form data
headers
CLI argument maps
CSV row dicts
INI/TOML values already stringified
```

### Not for

```text
raw JSON text
already-typed Python objects
large heterogeneous object graphs
binary payloads
```

Use `model_validate_json()` for raw JSON and `model_validate()` for already-decoded Python objects.

---

## 4.4 Python mode vs JSON mode

### Mode definitions

```text
Python mode
  source APIs: Model(...), model_validate(...)
  input shape: Python objects
  type checks: Python-object semantics
  strict mode: usually instance-of target type

JSON mode
  source APIs: model_validate_json(...), model_validate_strings(...)
  input shape: JSON token stream or stringly dict values
  type checks: JSON-compatible semantics
  strict mode: may still accept some JSON-native string encodings
```

Pydantic states that Python and JSON modes can have different validation behavior depending on types and model configuration, especially with strictness. For non-JSON sources where JSON-mode behavior/errors are desired, the docs recommend dumping to JSON or using `model_validate_strings()` when the data is a nested dictionary of string keys and string values. ([Pydantic Docs][S04-1])

### Demonstration: date strictness

```python
from datetime import date
from pydantic import BaseModel, ValidationError

class M(BaseModel):
    d: date

# Python mode strict: string is not a date object.
try:
    M.model_validate({"d": "2026-05-11"}, strict=True)
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "date_type"

# JSON mode strict: JSON date string can be valid for date.
assert M.model_validate_json(b'{"d": "2026-05-11"}', strict=True).d == date(2026, 5, 11)
```

Strict mode is explicitly documented as looser for JSON input for some types, including date/time types. ([Pydantic Docs][S04-3])

### Agent rule

```text
If source is JSON bytes/string:
  do not json.loads first unless custom pre-parse logic is required.

If source is non-JSON but all leaves are strings:
  use model_validate_strings for JSON-mode coercion.

If source is Python objects:
  use model_validate.

If tests need parity with wire JSON behavior:
  test with model_validate_json.
```

---

## 4.5 Passing `strict=True`

### Per-call strict

```python
class M(BaseModel):
    x: int

M.model_validate({"x": "123"})                 # ok, lax
M.model_validate({"x": "123"}, strict=True)    # ValidationError
```

Pydantic’s strict mode can be enabled per validation call, at the field level, or at the configuration level; per-call strictness is available on model validation methods and TypeAdapter validation methods. ([Pydantic Docs][S04-3])

### Deployment policy

```text
strict=True useful for:
  trusted internal APIs where types should already be normalized
  security-sensitive scalar fields
  regression tests that detect accidental coercion
  library APIs that should reject ambiguous values

strict=True dangerous for:
  env vars
  HTTP query strings
  headers
  forms
  CLI strings
  CSV rows
```

### Mixed policy

```python
from typing import Annotated
from pydantic import Field

class Login(BaseModel):
    username: str
    remember_me: bool
    session_ttl_seconds: Annotated[int, Field(strict=True)]
```

Use strict mode surgically when only one field must reject coercion.

---

## 4.6 Passing `context`

### Validator access

```python
from pydantic import BaseModel, ValidationInfo, field_validator

class TextModel(BaseModel):
    text: str

    @field_validator("text", mode="after")
    @classmethod
    def remove_stopwords(cls, v: str, info: ValidationInfo) -> str:
        if isinstance(info.context, dict):
            stopwords = set(info.context.get("stopwords", ()))
            return " ".join(w for w in v.split() if w.lower() not in stopwords)
        return v

m = TextModel.model_validate(
    {"text": "This is an example document"},
    context={"stopwords": ["this", "is", "an"]},
)

assert m.text == "example document"
```

Validation methods accept a context object, and validators can read it through `ValidationInfo.context`; `ValidationInfo` also exposes current validation mode, current field name for field validators, and already-validated data for field validators. ([Pydantic Docs][S04-5])

### Constructor limitation

```python
TextModel(text="This is an example document")  # no context parameter
```

Pydantic states that context cannot currently be provided when directly instantiating a model with `Model(...)`; the docs show a `ContextVar` plus custom `__init__` workaround, but that pattern should be reserved for infrastructure-level needs. ([Pydantic Docs][S04-5])

### Context policy

```text
Good context:
  locale
  stopword list
  parse policy
  current validation phase
  normalization profile
  trusted static lookup map

Bad context:
  DB session for writes
  network clients
  authorization decision engine
  mutable global transaction state
  long-running service handles
```

### Agent rule

```text
Context should parameterize pure validation/normalization.
Context should not turn validators into service-layer orchestration.
```

---

## 4.7 Validation by alias vs by field name

### Field alias

```python
from pydantic import BaseModel, ConfigDict, Field

class User(BaseModel):
    model_config = ConfigDict(validate_by_alias=True, validate_by_name=True)

    user_id: int = Field(validation_alias="userId")
```

Accepted:

```python
User.model_validate({"userId": 1})   # alias
User.model_validate({"user_id": 1})  # field name
```

`validate_by_alias` controls whether an aliased field may be populated by alias and defaults to `True`; `validate_by_name` controls whether a field may be populated by its Python attribute name and defaults to `False`. Pydantic documents that both cannot be set to `False`, because that would make population impossible. ([Pydantic Docs][S04-4])

### Per-call override

```python
User.model_validate(
    {"user_id": 1},
    by_alias=False,
    by_name=True,
)
```

The validation methods expose `by_alias` and `by_name` parameters to control whether input data is matched by alias or by field name for that validation call. ([Pydantic Docs][S04-2])

### `populate_by_name` warning

```python
class Legacy(BaseModel):
    model_config = ConfigDict(populate_by_name=True)
```

Pydantic warns that `populate_by_name` usage is not recommended in v2.11+ and will be deprecated in v3; `validate_by_name=True` and `validate_by_alias=True` provide the same behavior with finer control. ([Pydantic Docs][S04-4])

### Deployment policies

```text
External JSON camelCase:
  validate_by_alias=True
  validate_by_name=False or True depending compatibility
  serialize_by_alias=True for responses

Internal Python DTO:
  validate_by_name=True
  validate_by_alias=False unless legacy aliases are accepted

Migration window:
  validate_by_alias=True
  validate_by_name=True
  log field-name/alias usage outside Pydantic if needed

Strict public API:
  one accepted name per field
  avoid accepting both unless backward compatibility requires it
```

---

## 4.8 Validating nested objects

### Nested model validation from dictionaries

```python
class Foo(BaseModel):
    count: int
    size: float | None = None

class Bar(BaseModel):
    apple: str = "x"
    banana: str = "y"

class Spam(BaseModel):
    foo: Foo
    bars: list[Bar]

m = Spam.model_validate({
    "foo": {"count": "4"},
    "bars": [{"apple": "x1"}, {"apple": "x2"}],
})

assert isinstance(m.foo, Foo)
assert all(isinstance(b, Bar) for b in m.bars)
```

Pydantic docs show nested models defined as annotations, where nested dictionaries and lists of dictionaries are converted into nested `BaseModel` instances. ([Pydantic Docs][S04-1])

### Nested attribute validation

```python
class PetCls:
    def __init__(self, *, name: str) -> None:
        self.name = name

class PersonCls:
    def __init__(self, *, name: str, pets: list[PetCls]) -> None:
        self.name = name
        self.pets = pets

class Pet(BaseModel):
    model_config = ConfigDict(from_attributes=True)
    name: str

class Person(BaseModel):
    model_config = ConfigDict(from_attributes=True)
    name: str
    pets: list[Pet]

person = Person.model_validate(PersonCls(name="Ada", pets=[PetCls(name="Bones")]))
assert person.pets[0].name == "Bones"
```

When validating from attributes, model instances are created from both top-level attributes and deeper nested attributes when nested models also support attribute extraction. ([Pydantic Docs][S04-1])

### Nested deployment rule

```text
Nested JSON/dict payloads:
  nested model annotations are enough.

Nested ORM/object graphs:
  set from_attributes=True on each model that should read attributes.

Mixed dict/object graphs:
  validate deliberately; test the exact input shape.

Deep graphs:
  avoid side-effecting validators; nested validation can call many validators.
```

---

## 4.9 Revalidation of existing model instances

### Default behavior

```python
class User(BaseModel):
    name: str

class Transaction(BaseModel):
    user: User

u = User(name="John")
u.name = 1  # assignment not validated by default

t = Transaction.model_validate({"user": u})
assert t.user.name == 1
```

By default, `revalidate_instances='never'`; Pydantic does not revalidate model or dataclass instances encountered during validation. This setting affects the current model to which it is applied and does not propagate automatically to referenced field models. ([Pydantic Docs][S04-4])

### Always revalidate

```python
class User(BaseModel, revalidate_instances="always"):
    name: str

class Transaction(BaseModel):
    user: User

u = User(name="John")
u.name = 1

# Raises because the nested User instance is revalidated.
Transaction.model_validate({"user": u})
```

### Subclass-only revalidation

```python
class User(BaseModel, revalidate_instances="subclass-instances"):
    name: str

class SubUser(User):
    age: int

class Transaction(BaseModel):
    user: User
```

`revalidate_instances` accepts `'never'`, `'always'`, or `'subclass-instances'`; the subclass-only mode revalidates subclass instances and can coerce them to the declared base model type. ([Pydantic Docs][S04-4])

### Deployment policy

```text
Default app DTOs:
  revalidate_instances="never" is fastest and usually expected.

Security/trust boundary:
  revalidate_instances="always" for models crossing trust domains.

Subclass-sensitive APIs:
  revalidate_instances="subclass-instances" to avoid accepting extra subclass state silently.

Mutable model graphs:
  combine validate_assignment=True and/or revalidate_instances="always".
```

---

## 4.10 Assignment validation

### Default: assignment is not validated

```python
class User(BaseModel):
    name: str

user = User(name="John")
user.name = 123
assert user.name == 123
```

Pydantic validates data when the model is created; by default, changing an attribute after creation does not trigger validation. ([Pydantic Docs][S04-4])

### Enable assignment validation

```python
from pydantic import BaseModel, ConfigDict, ValidationError

class User(BaseModel):
    model_config = ConfigDict(validate_assignment=True)
    name: str

user = User(name="John")

try:
    user.name = 123
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "string_type"
```

`validate_assignment=True` can be set via class keyword argument or `model_config`, and it validates data when model attributes are changed. ([Pydantic Docs][S04-4])

### Class keyword form

```python
class User(BaseModel, validate_assignment=True):
    name: str
```

### Policy matrix

```text
Immutable/value object:
  ConfigDict(frozen=True)

Mutable but type-safe:
  validate_assignment=True

Mutable hot-path DTO:
  validate_assignment=False, mutate only in trusted code

Patch/update workflow:
  avoid mutation; create new model via validated merge

Untrusted updates:
  never use direct assignment without validate_assignment=True
```

---

## 4.11 Interaction: `validate_assignment` vs `revalidate_instances`

```text
validate_assignment:
  protects mutation of a model instance itself.

revalidate_instances:
  protects later validation when an existing model/dataclass instance is supplied as input to another validation operation.
```

Example:

```python
class User(BaseModel, validate_assignment=True, revalidate_instances="always"):
    name: str

class Transaction(BaseModel):
    user: User
```

Recommended high-integrity configuration:

```text
validate_assignment=True
  prevents corrupting the instance after creation

revalidate_instances="always"
  prevents corrupted/preexisting instances from passing through nested validation unchecked
```

Pydantic documents that assignment is not validated unless `validate_assignment` is set and that `revalidate_instances` controls whether model/dataclass instances are revalidated during validation. ([Pydantic Docs][S04-4])

---

## 4.12 Input-mode decision matrix

| Source                             | Entrypoint                                        | Mode                | Notes                                               |
| ---------------------------------- | ------------------------------------------------- | ------------------- | --------------------------------------------------- |
| Keyword args in Python             | `Model(...)`                                      | Python              | no `context` argument                               |
| Dict / mapping                     | `Model.model_validate(obj)`                       | Python              | supports strict/context/extra/alias/name            |
| Existing model instance            | `Model.model_validate(obj)`                       | Python              | revalidation controlled by `revalidate_instances`   |
| ORM / object attributes            | `Model.model_validate(obj, from_attributes=True)` | Python              | each nested attribute model needs compatible config |
| Raw JSON bytes/string              | `Model.model_validate_json(raw)`                  | JSON                | usually faster than manual JSON parse               |
| Env/form/query/header dict         | `Model.model_validate_strings(obj)`               | strings / JSON-mode | nested string-key/value maps                        |
| Untrusted update to existing model | `Model.model_validate(merged_dict)`               | Python              | avoid `model_copy(update=...)` for untrusted data   |
| Mutating existing instance         | assignment with `validate_assignment=True`        | Python              | otherwise no validation                             |

---

## 4.13 Boundary-oriented deployment recipes

### HTTP JSON request body

```python
class CreateUser(BaseModel):
    model_config = ConfigDict(extra="forbid")
    email: str
    age: int | None = None

def parse_body(raw_body: bytes) -> CreateUser:
    return CreateUser.model_validate_json(raw_body)
```

Why:

```text
raw JSON bytes
no separate json.loads
extra fields rejected
model errors map to 400/422
```

---

### Query parameter parsing

```python
class SearchParams(BaseModel):
    q: str
    limit: int = 25
    include_archived: bool = False

params = SearchParams.model_validate_strings({
    "q": "pydantic",
    "limit": "50",
    "include_archived": "false",
})
```

Why:

```text
all inputs are strings
want JSON-mode string coercion
avoid ad hoc int/bool parsing
```

---

### ORM response projection

```python
class UserOut(BaseModel):
    model_config = ConfigDict(from_attributes=True)
    id: int
    email: str

def serialize_user(user_orm) -> dict:
    return UserOut.model_validate(user_orm).model_dump(mode="json")
```

Why:

```text
validate ORM/domain object into public response DTO
avoid exposing internal object directly
```

Attribute-based validation for ORM-like objects is supported when `from_attributes` is enabled. ([Pydantic Docs][S04-1])

---

### Context-driven normalization

```python
class LocalizedText(BaseModel):
    text: str

    @field_validator("text", mode="after")
    @classmethod
    def normalize(cls, v: str, info: ValidationInfo) -> str:
        if isinstance(info.context, dict) and info.context.get("trim"):
            return v.strip()
        return v

obj = LocalizedText.model_validate({"text": "  hi  "}, context={"trim": True})
```

Why:

```text
validator stays pure
caller controls normalization profile
no global config mutation
```

---

### High-integrity nested object graph

```python
class User(BaseModel, validate_assignment=True, revalidate_instances="always"):
    name: str

class Transaction(BaseModel):
    user: User
    amount_cents: int

u = User(name="Ada")
u.name = "Grace"  # validated

tx = Transaction.model_validate({"user": u, "amount_cents": "100"})
```

Why:

```text
assignment-time protection
nested-instance revalidation
safe across long-lived mutable object graphs
```

---

## 4.14 Anti-patterns

### Anti-pattern: JSON parse before validation

```python
data = json.loads(raw)
obj = Model.model_validate(data)
```

Preferred:

```python
obj = Model.model_validate_json(raw)
```

Use `model_validate_json()` for raw JSON payloads because Pydantic validates the JSON string/bytes directly and documents it as generally faster for incoming JSON payloads. ([Pydantic Docs][S04-1])

---

### Anti-pattern: expecting constructor context

```python
Model(x=1, context={"tenant": "a"})  # context becomes data, not validation context
```

Preferred:

```python
Model.model_validate({"x": 1}, context={"tenant": "a"})
```

Pydantic explicitly states that it is not currently possible to provide context when directly instantiating a model with `Model(...)`; context is passed through validation methods. ([Pydantic Docs][S04-5])

---

### Anti-pattern: mutating models and assuming validation

```python
m = Model.model_validate(data)
m.x = "bad"  # not validated unless validate_assignment=True
```

Preferred:

```python
class Model(BaseModel, validate_assignment=True):
    x: int
```

Assignment validation is disabled by default; set `validate_assignment=True` to validate changes after model creation. ([Pydantic Docs][S04-4])

---

### Anti-pattern: accepting both alias and name accidentally

```python
class M(BaseModel):
    model_config = ConfigDict(validate_by_alias=True, validate_by_name=True)
    internal_name: int = Field(validation_alias="externalName")
```

Use only when backward compatibility requires it. For strict external APIs, choose one accepted input spelling to avoid ambiguity.

---

### Anti-pattern: trusting mutated existing models

```python
class User(BaseModel):
    name: str

user = User(name="Ada")
user.name = 123

Wrapper.model_validate({"user": user})  # may accept unless revalidation configured
```

Preferred for trust boundaries:

```python
class User(BaseModel, revalidate_instances="always"):
    name: str
```

Pydantic defaults `revalidate_instances` to `'never'`, so existing model/dataclass instances are not revalidated unless configured otherwise. ([Pydantic Docs][S04-4])

---

## 4.15 Test matrix

### Entrypoint parity

```python
def test_json_entrypoint():
    class M(BaseModel):
        x: int

    assert M.model_validate({"x": "1"}).x == 1
    assert M.model_validate_json(b'{"x": "1"}').x == 1
    assert M.model_validate_strings({"x": "1"}).x == 1
```

### Strict behavior

```python
def test_strict_python_mode():
    class M(BaseModel):
        x: int

    with pytest.raises(ValidationError) as exc:
        M.model_validate({"x": "1"}, strict=True)

    assert exc.value.errors()[0]["type"] == "int_type"
```

### Context

```python
def test_context_validator():
    class M(BaseModel):
        text: str

        @field_validator("text", mode="after")
        @classmethod
        def normalize(cls, v: str, info: ValidationInfo) -> str:
            return v.strip() if isinstance(info.context, dict) and info.context.get("strip") else v

    assert M.model_validate({"text": " x "}, context={"strip": True}).text == "x"
```

### Alias/name policy

```python
def test_alias_only_policy():
    class M(BaseModel):
        model_config = ConfigDict(validate_by_alias=True, validate_by_name=False)
        x: int = Field(validation_alias="externalX")

    assert M.model_validate({"externalX": 1}).x == 1

    with pytest.raises(ValidationError):
        M.model_validate({"x": 1})
```

### Assignment validation

```python
def test_assignment_validation():
    class M(BaseModel, validate_assignment=True):
        x: int

    m = M(x=1)

    with pytest.raises(ValidationError):
        m.x = "bad"
```

### Revalidation

```python
def test_revalidate_instances_always():
    class User(BaseModel, revalidate_instances="always"):
        name: str

    class Box(BaseModel):
        user: User

    user = User(name="Ada")
    object.__setattr__(user, "name", 123)

    with pytest.raises(ValidationError):
        Box.model_validate({"user": user})
```

---

## 4.16 Agent checklist

```text
[ ] Raw JSON bytes/string uses model_validate_json.
[ ] Already-decoded Python objects use model_validate.
[ ] Stringly env/form/query/header maps use model_validate_strings.
[ ] strict=True used deliberately, not globally by habit.
[ ] Validators needing runtime parameters use context through model_validate_*.
[ ] Constructor Model(...) not used when context is required.
[ ] Alias/name policy is explicit with validate_by_alias / validate_by_name.
[ ] populate_by_name avoided for new code.
[ ] Nested ORM validation sets from_attributes=True where needed.
[ ] Existing model instances crossing trust boundaries use revalidate_instances="always".
[ ] Mutated models requiring type safety use validate_assignment=True.
[ ] Extra-field policy overridden per call only for intentional boundary exceptions.
[ ] Tests assert ValidationError.errors()[*]["type"], not string prose.
```

---

## 4.17 Value case

```text
Validation entrypoint value =
  correct input-mode semantics
  faster raw JSON validation path
  explicit stringly-data coercion path
  per-call strictness
  per-call extra-field policy
  per-call validation context
  alias/name compatibility control
  nested dict/object graph validation
  model-instance revalidation control
  mutation-time assignment validation
```

Operationally: choose validation entrypoints by source encoding, set strictness and alias policy at the boundary, pass context only for pure validator parameterization, and use `validate_assignment` plus `revalidate_instances` when model instances are mutable or cross trust boundaries.

[S04-1]: https://docs.pydantic.dev/latest/concepts/models/ "Models | Pydantic Docs"
[S04-2]: https://docs.pydantic.dev/latest/api/base_model/ "BaseModel | Pydantic Docs"
[S04-3]: https://docs.pydantic.dev/latest/concepts/strict_mode/ "Strict Mode | Pydantic Docs"
[S04-4]: https://docs.pydantic.dev/latest/api/config/ "Configuration | Pydantic Docs"
[S04-5]: https://docs.pydantic.dev/latest/concepts/validators/ "Validators | Pydantic Docs"


# Pydantic Advanced — 5) Type system and coercion rules — Pydantic v2 deep dive

Style target: advanced technical-doc structure for programming agents. 

Pydantic’s type system uses Python annotations to define validation and serialization behavior. Built-in and standard-library types can usually be used directly; strictness and constraints can be layered with `Field(...)`, `Strict()`, `Annotated[...]`, and `annotated-types` metadata. Pydantic’s own docs route supported built-in/stdlib behavior through the “Standard Library Types” page and summarize allowed conversions through the “Conversion Table.” ([Pydantic Docs][S05-1])

---

## 5.0 Core mental model

```text
annotation
  → core schema
  → validation mode: python | json | strings
  → coercion policy: lax | strict
  → validated runtime value
  → serialization mode: python | json
```

Agent invariant:

```text
Type annotation controls:
  accepted input shapes
  coercion rules
  runtime output type
  nested validation
  JSON Schema shape
  serialization behavior

Field / Annotated metadata controls:
  strictness
  numeric/string/container constraints
  JSON Schema metadata
  custom validation / serialization
```

Reusable constrained types should be encoded as `Annotated` aliases:

```python
from typing import Annotated
from pydantic import Field

PositiveInt = Annotated[int, Field(gt=0)]
ShortStr = Annotated[str, Field(min_length=1, max_length=64)]
PositiveIntList = Annotated[list[PositiveInt], Field(min_length=1, max_length=100)]
```

Pydantic documents this exact pattern for reusable types, including `PositiveInt = Annotated[int, Field(gt=0)]`; it also supports `annotated-types` constraints such as `Gt`, `Len`, and named type aliases for reusable JSON Schema definitions. ([Pydantic Docs][S05-1])

---

## 5.1 Validation modes and coercion modes

### Validation modes

```text
Python mode:
  Model(...)
  Model.model_validate(...)
  TypeAdapter(T).validate_python(...)

JSON mode:
  Model.model_validate_json(...)
  TypeAdapter(T).validate_json(...)

Strings mode:
  Model.model_validate_strings(...)
```

### Coercion modes

```text
lax mode:
  convert compatible input values where Pydantic defines conversion rules

strict mode:
  reject most conversions; accept only exact/near-exact runtime types
```

Strictness is type-specific and input-mode-specific. Example: `date` strict validation rejects a Python string in Python mode, but JSON strict validation may accept an RFC 3339 date string because JSON has no native `date` object. Pydantic documents that strict-mode allowances differ between Python and JSON input. ([Pydantic Docs][S05-2])

---

## 5.2 Scalar types: `bool`, `str`, `bytes`, `int`, `float`

## 5.2.1 `bool`

```python
from pydantic import BaseModel

class M(BaseModel):
    flag: bool

assert M(flag=True).flag is True
assert M(flag=1).flag is True
assert M(flag="yes").flag is True
assert M(flag="false").flag is False
```

Accepted lax inputs:

```text
bool instance
0 / 1
"0", "1"
"off", "on"
"f", "t"
"false", "true"
"n", "y"
"no", "yes"
bytes decodable to one of those strings
```

In strict mode, only actual booleans are valid; Pydantic provides `StrictBool` as a convenience. ([Pydantic Docs][S05-2])

Deployment rule:

```text
HTTP/query/env booleans: bool lax is useful.
Security-critical config: StrictBool or Field(strict=True).
Avoid bool for tri-state flags; use Literal["on", "off", "auto"].
```

---

## 5.2.2 `str`

```python
from typing import Annotated
from pydantic import BaseModel, Field, StringConstraints

Slug = Annotated[str, Field(pattern=r"^[a-z0-9-]+$", min_length=1, max_length=128)]

class M(BaseModel):
    name: str
    slug: Slug
    lower: Annotated[str, StringConstraints(to_lower=True)]
```

Lax behavior:

```text
str accepted as-is
bytes / bytearray decoded as UTF-8
enum values converted through value → str()
numbers only coerced to str if coerce_numbers_to_str is configured
```

String constraints include `pattern`, `min_length`, `max_length`, plus `StringConstraints` options such as whitespace stripping, case conversion, and ASCII-only checks. In strict mode, only strings are valid; `StrictStr` is provided as a convenience. ([Pydantic Docs][S05-2])

Agent rule:

```text
Use str for human text.
Use Literal[...] or Enum for closed vocabularies.
Use Annotated[str, Field(pattern=...)] for syntactic identifiers.
Use validators for semantic checks.
```

---

## 5.2.3 `bytes`

```python
from typing import Annotated
from pydantic import BaseModel, Field

class Blob(BaseModel):
    data: Annotated[bytes, Field(min_length=1, max_length=1024)]
```

Behavior:

```text
bytes accepted as-is
str and bytearray converted according to byte serialization config
min_length / max_length constraints supported
strict mode: only bytes accepted in Python mode
JSON mode: strictness has no effect for bytes
```

Pydantic documents `bytes` validation, length constraints, strictness, and the JSON-mode strictness exception. ([Pydantic Docs][S05-2])

Deployment rule:

```text
Binary API payload already decoded: bytes.
Base64 transport: configure val_json_bytes / serialization policy deliberately.
Do not use bytes for arbitrary JSON blobs; use JsonValue / dict / TypeAdapter.
```

---

## 5.2.4 `int`

```python
from typing import Annotated
from pydantic import BaseModel, Field

PositiveInt = Annotated[int, Field(gt=0)]
EvenInt = Annotated[int, Field(multiple_of=2)]

class M(BaseModel):
    id: int
    qty: PositiveInt
    page_size: Annotated[int, Field(ge=1, le=500)]
```

Lax behavior:

```text
int accepted as-is
numeric strings / bytes parsed as integers
float accepted only if finite and fractional part is zero
Decimal accepted if finite and denominator is one
Fraction accepted if it represents an integer
Enum converted through value
bool is explicitly forbidden as int input in conversion table strict row
```

Constraints: `gt`, `ge`, `lt`, `le`, `multiple_of`; strict mode accepts only integer values, and `StrictInt` exists as a convenience. ([Pydantic Docs][S05-2])

Agent rule:

```text
IDs from JSON strings: lax int may be OK.
Security/accounting identifiers: consider str, not int.
Pagination/counts: Annotated[int, Field(ge=..., le=...)].
Do not rely on bool-as-int; Pydantic forbids bool for int validation.
```

---

## 5.2.5 `float`

```python
from typing import Annotated
from pydantic import BaseModel, Field, FiniteFloat

class Measurement(BaseModel):
    value: float
    ratio: Annotated[float, Field(ge=0.0, le=1.0)]
    finite: FiniteFloat
```

Lax behavior:

```text
float accepted as-is
str / bytes parsed as float
objects with __float__() accepted
fallback to __index__() if __float__ absent
Decimal and Fraction can be converted
```

Constraints: `gt`, `ge`, `lt`, `le`, `multiple_of`, `allow_inf_nan`; strict mode accepts floats and values with `__float__()` / `__index__()`, and `StrictFloat` is provided. Pydantic also provides convenience aliases such as `PositiveFloat`, `NonNegativeFloat`, and `FiniteFloat`. ([Pydantic Docs][S05-2])

Deployment rule:

```text
Money: Decimal, not float.
Measurements/probabilities: float with bounds.
Model scores: FiniteFloat unless NaN/inf is intentional.
```

---

## 5.3 `Decimal`

```python
from decimal import Decimal
from typing import Annotated
from pydantic import BaseModel, Field

Money = Annotated[
    Decimal,
    Field(gt=Decimal("0"), max_digits=12, decimal_places=2),
]

class Invoice(BaseModel):
    amount: Money
```

Behavior:

```text
Decimal accepted as-is
any value accepted by Decimal constructor may validate
constraints: gt/ge/lt/le/multiple_of/allow_inf_nan/max_digits/decimal_places
strict Python mode: only Decimal instances accepted
JSON mode strictness: no effect
Python serialization: Decimal as-is
JSON serialization: string by default
```

Pydantic documents Decimal validation, precision constraints, strictness, and default JSON serialization as strings. ([Pydantic Docs][S05-2])

Agent rule:

```text
Money/accounting/tax: Decimal.
External JSON decimal contract: decide string vs numeric serializer explicitly.
Never use float for currency unless lossy arithmetic is acceptable.
```

Optional JSON numeric serializer:

```python
from decimal import Decimal
from typing import Annotated
from pydantic import BaseModel, PlainSerializer

DecimalAsFloat = Annotated[Decimal, PlainSerializer(float, when_used="json")]

class M(BaseModel):
    value: DecimalAsFloat
```

---

## 5.4 Date and time types

## 5.4.1 `datetime`

```python
from datetime import datetime
from pydantic import BaseModel, AwareDatetime, FutureDatetime

class Event(BaseModel):
    at: datetime
    aware_at: AwareDatetime
    future_at: FutureDatetime
```

Behavior:

```text
datetime accepted as-is
RFC 3339 strings/bytes accepted
Unix timestamps accepted as seconds or milliseconds
date accepted and converted to midnight naive datetime
Python mode serialization: datetime as-is
JSON mode serialization: string
```

Constraints: `gt`, `ge`, `lt`, `le`; convenience constrained types include `AwareDatetime`, `NaiveDatetime`, `PastDatetime`, and `FutureDatetime`. Strict Python mode accepts only `datetime` instances; JSON strict mode accepts RFC 3339 datetime strings or Unix timestamps. ([Pydantic Docs][S05-2])

Agent rule:

```text
Public API timestamps: AwareDatetime.
Internal scheduled jobs: FutureDatetime where appropriate.
Audit timestamps: datetime + timezone policy validator.
Do not accept naive datetimes unless explicitly intended.
```

---

## 5.4.2 `date`

```python
from datetime import date
from pydantic import BaseModel, PastDate, FutureDate

class Person(BaseModel):
    birthday: PastDate
    expires_on: FutureDate | None = None
```

Behavior:

```text
date accepted as-is
RFC 3339 date strings/bytes accepted
Unix timestamps accepted
datetime accepted only if time component is zero and naive
Python serialization: date as-is
JSON serialization: string
```

Strict Python mode accepts only `date`; JSON strict mode accepts RFC 3339 date strings or Unix timestamps. Pydantic provides `PastDate` and `FutureDate`. ([Pydantic Docs][S05-2])

---

## 5.4.3 `time`

```python
from datetime import time
from pydantic import BaseModel

class Meeting(BaseModel):
    starts_at: time
```

Behavior:

```text
time accepted as-is
RFC 3339 time strings/bytes accepted
numeric seconds accepted, max 86_399
Python serialization: time as-is
JSON serialization: string
strict Python mode: only time
JSON strict mode: RFC 3339 time strings
```

Pydantic documents validation, serialization, constraints, and strictness for `time`. ([Pydantic Docs][S05-2])

---

## 5.4.4 `timedelta`

```python
from datetime import timedelta
from typing import Annotated
from pydantic import BaseModel, Field

class RetryPolicy(BaseModel):
    timeout: Annotated[timedelta, Field(gt=timedelta(seconds=0))]
```

Behavior:

```text
timedelta accepted as-is
RFC 3339 duration strings/bytes accepted
numeric values accepted as seconds
Python serialization: timedelta as-is
JSON serialization: string
strict Python mode: only timedelta
JSON strict mode: RFC 3339 duration strings
```

Pydantic documents `timedelta` validation from strings/bytes and numeric seconds, its comparison constraints, and JSON serialization as strings. ([Pydantic Docs][S05-2])

Deployment rule:

```text
Durations in env/query strings: timedelta lax is useful.
Public JSON schema: document accepted duration format.
Config values: prefer explicit examples.
```

---

## 5.5 `UUID`

```python
from typing import Annotated
from uuid import UUID
from pydantic import BaseModel
from pydantic.types import UuidVersion, UUID7

class M(BaseModel):
    id: UUID
    event_id: UUID7
    legacy_id: Annotated[UUID, UuidVersion(4)]
```

Behavior:

```text
UUID accepted as-is
strings / bytes parsed as UUID
version constraint supported
convenience aliases: UUID1, UUID3, UUID4, UUID5, UUID6, UUID7, UUID8
strict Python mode: only UUID instances
JSON strict mode: strictness has no effect
Python serialization: UUID as-is
JSON serialization: string
```

Pydantic documents UUID validation, version constraints, strictness, convenience aliases, and serialization behavior. ([Pydantic Docs][S05-2])

Agent rule:

```text
External IDs: UUID / UUID4 / UUID7.
Database-generated UUID version: encode version if contract requires it.
Do not use str if UUID semantics matter.
```

---

## 5.6 `Path`

```python
from pathlib import Path
from pydantic import BaseModel

class FileJob(BaseModel):
    input_path: Path
```

Behavior:

```text
Path instances accepted as-is
strings passed to path constructor
os.PathLike accepted when parameterized appropriately
strict Python mode: only Path instances
JSON strict mode: strictness has no effect
Python serialization: Path as-is
JSON serialization: string
```

Pydantic documents support for `pathlib.Path`, pure/posix/windows path variants, and `os.PathLike`, with strings accepted in lax mode and path instances required in strict Python mode. ([Pydantic Docs][S05-2])

Agent rule:

```text
Syntax validation only: Path.
Existence / file / directory checks: add validator or use custom constrained type.
Security-sensitive paths: normalize + resolve + sandbox check outside simple type coercion.
```

---

## 5.7 Containers

## 5.7.1 `list[T]`

```python
from typing import Annotated
from pydantic import BaseModel, Field

PositiveInt = Annotated[int, Field(gt=0)]

class Batch(BaseModel):
    ids: Annotated[list[PositiveInt], Field(min_length=1, max_length=1000)]
```

Behavior:

```text
list / tuple / set / frozenset accepted
most non-string, non-bytes, non-mapping iterables accepted
output is list
generic item type validates every element
min_length / max_length supported
strict mode: only list accepted as outer value
strict mode does not apply to items unless item type is strict
```

Pydantic recommends built-in concrete collection types over abstract ones for performance and coercion behavior; list validation produces a list and applies generic item validation if specified. ([Pydantic Docs][S05-2])

Strict item pattern:

```python
from typing import Annotated
from pydantic import Field, Strict

StrictIntList = list[Annotated[int, Strict()]]
```

---

## 5.7.2 `tuple[...]`

```python
class Point(BaseModel):
    coords: tuple[int, float, bool]
    many_ids: tuple[int, ...]
```

Behavior:

```text
tuple / list / set / frozenset / compatible iterable accepted
output is tuple
fixed tuple validates per-position types
variable tuple validates repeated item type
min_length / max_length supported
strict mode: only tuple accepted as outer value
strictness does not apply to items unless item types are strict
```

Pydantic documents tuple validation from common iterable containers, per-element validation, tuple length constraints, and strict-mode outer-type behavior. ([Pydantic Docs][S05-2])

---

## 5.7.3 `set[T]` / `frozenset[T]`

```python
class Tags(BaseModel):
    tags: set[str]
    frozen_ids: frozenset[int]
```

Behavior:

```text
many iterable containers accepted
duplicates removed by set semantics
output is set / frozenset
generic item type validates each item
strict mode: only set/frozenset accepted as outer value
JSON serialization: arrays
```

Pydantic documents sets/frozensets with outer strictness, JSON serialization as arrays, and item validation through generic parameters. ([Pydantic Docs][S05-2])

Agent rule:

```text
Use list when order/duplicates matter.
Use set when uniqueness matters and order must not be relied on.
Use frozenset for hashable value-object fields.
```

---

## 5.7.4 `dict[K, V]`

```python
class Scores(BaseModel):
    by_user: dict[str, int]
```

Behavior:

```text
dict accepted as-is
Mapping accepted and coerced to dict
key type validates each key
value type validates each value
min_length / max_length supported
strict mode: only dict accepted as outer value
strictness does not apply to keys/values unless parameter types are strict
```

Pydantic documents dictionary validation from `dict` and mapping instances, key/value generic validation, length constraints, and strict-mode behavior. ([Pydantic Docs][S05-2])

Agent rule:

```text
Use dict[str, T] for JSON objects.
Avoid dict[Any, Any] for public contracts.
Constrain key type if keys are semantic identifiers.
```

---

## 5.8 Abstract containers and iterables

```python
from collections.abc import Sequence, Iterable
from pydantic import BaseModel

class M(BaseModel):
    xs: Sequence[str]
    stream: Iterable[str]
```

Pydantic supports built-in and abstract collection types, but recommends concrete built-ins such as `list` and `tuple` in most cases because coercion is simpler and performance is better. For `Sequence`, strings and bytes are intentionally rejected to avoid the common “string treated as character sequence” bug. Iterables are lazily validated and wrapped; item validation occurs while iterating, so use concrete containers unless infinite/lazy streams are intended. ([Pydantic Docs][S05-2])

Agent rule:

```text
Use list[T] for normal API arrays.
Use tuple[...] for positional records.
Use Iterable[T] only for lazy/infinite streams.
Use Sequence[T] only when preserving input sequence type matters.
```

---

## 5.9 `TypedDict`

```python
from typing_extensions import TypedDict
from pydantic import TypeAdapter

class UserDict(TypedDict):
    name: str
    id: int

adapter = TypeAdapter(UserDict)
value = adapter.validate_python({"name": "Ada", "id": "1"})
assert value == {"name": "Ada", "id": 1}
```

`TypedDict` validates dictionary-shaped data without creating a `BaseModel` instance. On Python 3.12 and lower, Pydantic requires `typing_extensions.TypedDict` because of runtime limitations. Strict mode requires an actual `dict` outer value, but strictness does not automatically apply to values; value types must be strict themselves. ([Pydantic Docs][S05-2])

Agent rule:

```text
Use TypedDict for:
  schema-only dict contracts
  TypeAdapter validation
  interop with code expecting plain dicts

Use BaseModel for:
  methods
  validators/serializers
  model_dump/model_dump_json
  field metadata
  object identity
```

---

## 5.10 Named tuples

```python
from typing import NamedTuple
from pydantic import BaseModel

class Point(NamedTuple):
    x: int
    y: int

class M(BaseModel):
    p: Point

assert M(p=("1", 2)).p == Point(x=1, y=2)
assert M(p={"x": "1", "y": 2}).p == Point(x=1, y=2)
```

Named tuples can validate from tuples/lists or dictionaries keyed by field names. In Python serialization mode they serialize as tuples; in JSON mode they serialize as arrays. ([Pydantic Docs][S05-2])

Agent rule:

```text
Use NamedTuple for compact positional records.
Use BaseModel when field names and JSON object representation matter.
```

---

## 5.11 Dataclasses

```python
from datetime import datetime
from pydantic.dataclasses import dataclass

@dataclass
class User:
    id: int
    name: str = "John Doe"
    signup_ts: datetime | None = None

user = User(id="42", signup_ts="2032-06-21T12:00")
assert user.id == 42
```

Pydantic dataclasses provide stdlib-dataclass-style objects with Pydantic validation, but they are not drop-in replacements for `BaseModel`. They do not expose model methods such as validation, dump, and JSON Schema generation directly; wrap them with `TypeAdapter` to validate, dump, or generate schema. Pydantic dataclasses support configuration, nesting, copied construction arguments, validators, and initialization hooks, but have different extra-field behavior and generic caveats. ([Pydantic Docs][S05-3])

TypeAdapter pattern:

```python
from pydantic import TypeAdapter

adapter = TypeAdapter(User)
u = adapter.validate_python({"id": "42"})
dumped = adapter.dump_python(u)
```

Agent rule:

```text
Use pydantic.dataclasses when:
  dataclass ergonomics are required
  BaseModel methods are not needed on the object

Use BaseModel when:
  serialization/schema/model methods are central
  model_config inheritance is important
  JSON API objects are public contracts
```

---

## 5.12 Literals

```python
from typing import Literal
from pydantic import BaseModel

class Pie(BaseModel):
    flavor: Literal["apple", "pumpkin"]
    quantity: Literal[1, 2] = 1
```

`Literal[...]` permits only specific values. Pydantic applies strict-mode-like behavior when validating literals: for example, `Literal[1, 2]` rejects the string `"1"` rather than coercing it to integer `1`. ([Pydantic Docs][S05-2])

Agent rule:

```text
Use Literal for:
  discriminators
  fixed protocol values
  finite string choices
  exact sentinel values

Do not use Literal for large dynamic vocabularies.
Use Enum for named symbolic choices shared across code.
```

---

## 5.13 Enums

```python
from enum import Enum, IntEnum
from pydantic import BaseModel

class Fruit(str, Enum):
    PEAR = "pear"
    BANANA = "banana"

class Tool(IntEnum):
    SPANNER = 1
    WRENCH = 2

class M(BaseModel):
    fruit: Fruit
    tool: Tool
```

Behavior:

```text
Enum class directly: enum instances accepted
Enum subclass annotation: enum member or matching enum value accepted
Python serialization: enum instance as-is unless use_enum_values configured
JSON serialization: enum value
```

Pydantic documents enum validation by member/value, JSON serialization using enum values, and `use_enum_values` as a config option that extracts enum values during validation. ([Pydantic Docs][S05-2])

Agent rule:

```text
Use str Enum for public JSON values.
Use IntEnum only when numeric wire values are legacy or required.
Use Literal for one-off discriminator fields.
Use Enum for reusable domain vocabularies.
```

---

## 5.14 Normal unions

```python
from typing import Union
from uuid import UUID
from pydantic import BaseModel

class User(BaseModel):
    id: int | str | UUID
```

Union validation differs from ordinary type validation: only one union member needs to validate. Pydantic supports three union approaches: left-to-right mode, smart mode, and discriminated unions. Smart mode is the default for most unions; discriminated unions are recommended because they are more predictable and performant. ([Pydantic Docs][S05-4])

---

## 5.15 Union mode: `smart` default

```python
class User(BaseModel):
    id: int | str | UUID
```

Smart-mode selection uses:

```text
for BaseModel / dataclass / TypedDict unions:
  number of valid fields set
  exactness as tiebreaker

for other unions:
  exact type match
  strict-mode success
  lax-mode success
```

Pydantic reserves the right to change smart-mode matching between minor releases; if exact match ordering is a contract, use `union_mode="left_to_right"` or a discriminated union. ([Pydantic Docs][S05-4])

Agent rule:

```text
Use smart unions for convenience.
Do not depend on obscure smart-mode edge selection.
Pin behavior with tests when ambiguity matters.
```

---

## 5.16 Union mode: `left_to_right`

```python
from typing import Union
from pydantic import BaseModel, Field

class M(BaseModel):
    id: Union[int, str] = Field(union_mode="left_to_right")
```

Behavior:

```text
try members in annotation order
first successful validation wins
all errors returned if all fail
numeric string "456" may become int if int appears before str in lax mode
```

Pydantic documents that left-to-right mode is not the default because it can produce surprising results, and it must be configured as a `Field` parameter. ([Pydantic Docs][S05-4])

Agent rule:

```text
Use left_to_right only when:
  order is the desired policy
  ambiguity is acceptable and tested
  backward compatibility requires a specific selection order

Prefer discriminators for model unions.
```

---

## 5.17 Discriminated unions: string discriminator

```python
from typing import Literal
from pydantic import BaseModel, Field

class Cat(BaseModel):
    pet_type: Literal["cat"]
    meows: int

class Dog(BaseModel):
    pet_type: Literal["dog"]
    barks: float

class Lizard(BaseModel):
    pet_type: Literal["reptile", "lizard"]
    scales: bool

class Model(BaseModel):
    pet: Cat | Dog | Lizard = Field(discriminator="pet_type")
    n: int
```

Behavior:

```text
read pet_type
select exactly one model branch
validate only selected branch
simpler errors
better performance
OpenAPI discriminator emitted in JSON Schema
```

Pydantic recommends discriminated unions because they are more performant and predictable; when a common field exists, each variant should define that field as a `Literal`, and the union field should specify `Field(discriminator="...")`. ([Pydantic Docs][S05-4])

Agent rule:

```text
For API polymorphism: always prefer discriminated unions.
Discriminator field should be required.
Discriminator value should be Literal.
Variant model names should not be used as implicit protocol tags.
```

---

## 5.18 Callable discriminators

```python
from typing import Annotated, Any, Literal
from pydantic import BaseModel, Discriminator, Tag

class Pie(BaseModel):
    time_to_cook: int
    num_ingredients: int

class ApplePie(Pie):
    fruit: Literal["apple"] = "apple"

class PumpkinPie(Pie):
    filling: Literal["pumpkin"] = "pumpkin"

def get_tag(v: Any) -> str | None:
    if isinstance(v, dict):
        return v.get("fruit", v.get("filling"))
    return getattr(v, "fruit", getattr(v, "filling", None))

Dessert = Annotated[
    Annotated[ApplePie, Tag("apple")] | Annotated[PumpkinPie, Tag("pumpkin")],
    Discriminator(get_tag),
]

class Dinner(BaseModel):
    dessert: Dessert
```

Callable discriminators are intended for unions without a uniform discriminator field. Pydantic explicitly warns that callable discriminators must handle both dictionaries and model instances because they are used during both validation and serialization. Returning `None` produces `union_tag_not_found`. ([Pydantic Docs][S05-4])

Agent rule:

```text
Callable discriminator must be:
  pure
  total over expected dict/model inputs
  side-effect-free
  stable across validation and serialization
```

Avoid:

```python
def discriminator(v):
    return requests.get(...).json()["type"]  # bad
```

---

## 5.19 Nested discriminated unions

```python
from typing import Annotated, Literal
from pydantic import BaseModel, Field

class BlackCat(BaseModel):
    pet_type: Literal["cat"]
    color: Literal["black"]
    black_name: str

class WhiteCat(BaseModel):
    pet_type: Literal["cat"]
    color: Literal["white"]
    white_name: str

Cat = Annotated[BlackCat | WhiteCat, Field(discriminator="color")]

class Dog(BaseModel):
    pet_type: Literal["dog"]
    name: str

Pet = Annotated[Cat | Dog, Field(discriminator="pet_type")]

class Model(BaseModel):
    pet: Pet
```

Only one discriminator can be set for a field, but nested `Annotated` union aliases allow multiple discriminator layers, such as first `pet_type`, then `color` inside the cat branch. ([Pydantic Docs][S05-4])

Agent rule:

```text
Use nested discriminators for:
  hierarchical protocols
  type + subtype dispatch
  large polymorphic JSON payloads

Do not flatten every discriminator into one callable unless field-based dispatch is impossible.
```

---

## 5.20 Conversion table deep dive

The conversion table is the authoritative high-level matrix for allowed conversions in strict/lax and Python/JSON modes. It explicitly marks conversions allowed under strict mode and distinguishes input source (`Python`, `JSON`, or both). ([Pydantic Docs][S05-5])

High-impact conversions:

```text
bool:
  accepts bool strictly
  lax accepts 0/1, selected strings, selected Decimals

int:
  accepts int strictly
  lax accepts numeric strings/bytes
  lax accepts exact floats only, not nan/inf
  bool explicitly forbidden

float:
  accepts float/int strictly in conversion table
  lax accepts numeric strings/bytes
  bool explicitly forbidden for float row

date/datetime:
  accepts native objects strictly in Python mode
  lax accepts RFC 3339 strings and epoch timestamps
  JSON strict may still accept strings/timestamps

list/set/tuple:
  many iterable containers accepted lax
  JSON arrays map to collection types
  strict mode applies to outer container only

dict:
  dict strict
  mapping lax
  JSON object maps to dict

namedtuple:
  dict/list/tuple/namedtuple inputs supported
```

The table’s examples include bool string values, date/datetime epoch conversion, numeric string parsing for integers/floats, exact-int requirements for float→int, collection conversions from arrays/iterables, and mapping-to-dict conversion. ([Pydantic Docs][S05-5])

Agent rule:

```text
When an input surprise occurs:
  1. identify validation mode: Python vs JSON
  2. identify strictness: strict vs lax
  3. check conversion table
  4. add Field(strict=True), Strict(), or custom validator if conversion is unwanted
```

---

## 5.21 Strict vs lax patterns

### Field strictness

```python
from typing import Annotated
from pydantic import BaseModel, Field, Strict

class M(BaseModel):
    x: int = Field(strict=True)
    y: Annotated[int, Strict()]
```

### Model strictness

```python
from pydantic import BaseModel, ConfigDict

class StrictModel(BaseModel):
    model_config = ConfigDict(strict=True)
    x: int
    xs: list[int]
```

Important: strictness on collection types applies to the outer container, not automatically to inner item types; strict inner types must be specified separately. ([Pydantic Docs][S05-2])

Correct strict list-of-int:

```python
from typing import Annotated
from pydantic import Strict

StrictIntList = list[Annotated[int, Strict()]]
```

Policy matrix:

```text
External JSON body:
  lax by default, strict for dangerous fields

Env/query/form data:
  lax/string mode; everything starts as string

Internal trusted Python API:
  strict mode useful to catch upstream type bugs

Financial/security fields:
  strict + Decimal / Literal / Enum as appropriate

Collections:
  apply strictness to both container and item types if needed
```

---

## 5.22 Complete type-system example

```python
from __future__ import annotations

from datetime import date, datetime, time, timedelta
from decimal import Decimal
from enum import Enum
from pathlib import Path
from typing import Annotated, Literal
from uuid import UUID

from pydantic import (
    AwareDatetime,
    BaseModel,
    ConfigDict,
    Field,
    FiniteFloat,
    Strict,
)
from pydantic.types import UUID7


PositiveInt = Annotated[int, Field(gt=0)]
Money = Annotated[Decimal, Field(ge=Decimal("0"), max_digits=12, decimal_places=2)]
StrictInt = Annotated[int, Strict()]
NonEmptyTags = Annotated[list[Annotated[str, Field(min_length=1)]], Field(min_length=1)]


class Status(str, Enum):
    DRAFT = "draft"
    ACTIVE = "active"


class CreateEvent(BaseModel):
    model_config = ConfigDict(extra="forbid")

    id: UUID7
    account_id: UUID
    status: Status
    kind: Literal["event.created"]

    count: PositiveInt
    score: FiniteFloat
    amount: Money

    path: Path
    created_at: AwareDatetime
    run_date: date
    run_time: time
    timeout: timedelta

    tags: NonEmptyTags
    dimensions: tuple[PositiveInt, PositiveInt]
    properties: dict[str, str]
    strict_priority: StrictInt
```

---

## 5.23 Deployment advisory matrix

```text
Need                                      Recommended type
------------------------------------------------------------------------
public event discriminator                 Literal["..."]
reusable domain status                     str Enum
numeric database ID from trusted source     int / StrictInt
opaque external ID                          str with pattern or UUID
money                                       Decimal with max_digits/decimal_places
percentage                                  Annotated[float, Field(ge=0, le=1)]
non-NaN score                               FiniteFloat
timestamp with timezone                     AwareDatetime
calendar date                               date / PastDate / FutureDate
file path syntax                            Path
validated existing file                     Path + validator
JSON array preserving order                 list[T]
unique unordered values                     set[T]
fixed positional record                     tuple[T1, T2, ...]
plain dict contract                         TypedDict + TypeAdapter
named runtime object                        BaseModel
dataclass ergonomics with validation         pydantic.dataclasses.dataclass
polymorphic JSON payload                    discriminated union
ambiguous legacy union                      union_mode="left_to_right" + tests
```

---

## 5.24 Anti-patterns

### Ambiguous untagged model union

```python
class Cat(BaseModel):
    name: str
    meows: int | None = None

class Dog(BaseModel):
    name: str
    barks: int | None = None

class M(BaseModel):
    pet: Cat | Dog  # ambiguous
```

Preferred:

```python
class Cat(BaseModel):
    pet_type: Literal["cat"]
    meows: int

class Dog(BaseModel):
    pet_type: Literal["dog"]
    barks: int

class M(BaseModel):
    pet: Cat | Dog = Field(discriminator="pet_type")
```

---

### Strict container without strict items

```python
class M(BaseModel):
    xs: list[int] = Field(strict=True)
```

This only makes the outer input require `list`; item strings like `"1"` may still coerce to integers. Pydantic documents that strict mode on collection types does not apply to inner types. ([Pydantic Docs][S05-2])

Preferred:

```python
StrictIntList = list[Annotated[int, Strict()]]
```

---

### Float for money

```python
class Invoice(BaseModel):
    amount: float
```

Preferred:

```python
class Invoice(BaseModel):
    amount: Decimal = Field(max_digits=12, decimal_places=2)
```

---

### `dict[str, object]` for public API payloads

```python
class Payload(BaseModel):
    data: dict[str, object]
```

Preferred:

```python
class UserPayload(BaseModel):
    id: UUID
    email: str
```

or:

```python
class UserPayload(TypedDict):
    id: str
    email: str
```

---

### Callable discriminator that only handles dicts

```python
def tag(v):
    return v["kind"]  # fails for model instances during serialization
```

Preferred:

```python
def tag(v):
    if isinstance(v, dict):
        return v.get("kind")
    return getattr(v, "kind", None)
```

Pydantic explicitly warns callable discriminators should handle both dictionaries and model instances because they are used in validation and serialization. ([Pydantic Docs][S05-4])

---

## 5.25 Test matrix

```python
import pytest
from decimal import Decimal
from typing import Annotated, Literal
from pydantic import BaseModel, Field, Strict, TypeAdapter, ValidationError


def test_int_lax_vs_strict():
    class M(BaseModel):
        x: int

    assert M.model_validate({"x": "1"}).x == 1

    with pytest.raises(ValidationError) as exc:
        M.model_validate({"x": "1"}, strict=True)

    assert exc.value.errors()[0]["type"] in {"int_type", "int_parsing"}


def test_collection_strict_does_not_make_items_strict():
    class M(BaseModel):
        xs: list[int] = Field(strict=True)

    assert M(xs=["1"]).xs == [1]

    StrictIntList = list[Annotated[int, Strict()]]
    ta = TypeAdapter(StrictIntList)

    with pytest.raises(ValidationError):
        ta.validate_python(["1"])


def test_literal_is_exact():
    class M(BaseModel):
        q: Literal[1, 2]

    with pytest.raises(ValidationError):
        M(q="1")


def test_decimal_money():
    class M(BaseModel):
        amount: Decimal = Field(max_digits=5, decimal_places=2)

    assert M(amount="12.34").amount == Decimal("12.34")


def test_discriminated_union():
    class Cat(BaseModel):
        kind: Literal["cat"]
        meows: int

    class Dog(BaseModel):
        kind: Literal["dog"]
        barks: int

    class Box(BaseModel):
        pet: Cat | Dog = Field(discriminator="kind")

    assert isinstance(Box(pet={"kind": "dog", "barks": "3"}).pet, Dog)
```

---

## 5.26 Agent checklist

```text
[ ] Use concrete containers unless lazy/abstract behavior is intentional.
[ ] Apply constraints with Annotated aliases for reuse.
[ ] Use Decimal for money.
[ ] Use FiniteFloat for model scores when NaN/inf forbidden.
[ ] Use AwareDatetime for public timestamps.
[ ] Use UUID/UUID4/UUID7 instead of opaque str where UUID semantics matter.
[ ] Use Literal for fixed protocol tags.
[ ] Use str Enum for reusable JSON vocabularies.
[ ] Use discriminated unions for polymorphic payloads.
[ ] Avoid ambiguous untagged model unions.
[ ] Use union_mode="left_to_right" only with tests.
[ ] Remember strict container ≠ strict items.
[ ] Use TypeAdapter for top-level list/dict/TypedDict/union validation.
[ ] Consult conversion table for coercion surprises.
[ ] Test strict/lax and Python/JSON modes separately when boundary behavior matters.
```

---

## 5.27 Value case

```text
Pydantic type-system value =
  annotation-native validation
  deterministic runtime output types
  controlled coercion
  strict/lax policy by boundary
  reusable constrained aliases
  built-in scalar/date/path/UUID/Decimal support
  recursive container item validation
  TypedDict/dataclass/namedtuple interop
  exact Literal and Enum vocabularies
  predictable discriminated polymorphism
  JSON Schema generation from the same annotation graph
```

Operationally: encode structural and syntactic contracts in types; use `Annotated` constraints for reusable local rules; use discriminated unions for polymorphism; use strict mode selectively at trust boundaries; keep business truth, authorization, persistence, and network-dependent checks outside the type system.

[S05-1]: https://docs.pydantic.dev/latest/concepts/types/ "Types | Pydantic Docs"
[S05-2]: https://docs.pydantic.dev/latest/api/standard_library_types/ "Standard Library Types | Pydantic Docs"
[S05-3]: https://docs.pydantic.dev/latest/concepts/dataclasses/ "Dataclasses | Pydantic Docs"
[S05-4]: https://docs.pydantic.dev/latest/concepts/unions/ "Unions | Pydantic Docs"
[S05-5]: https://docs.pydantic.dev/latest/concepts/conversion_table/ "Conversion Table | Pydantic Docs"


# Pydantic Advanced — 6) Strict mode and coercion policy — Pydantic v2 deep dive

Style target: advanced, sectioned, agent-oriented reference. 

Pydantic defaults to **lax mode**: it attempts to coerce compatible inputs into the declared target type. **Strict mode** can be enabled per validation call, per field, or through config; in strict mode Pydantic is less lenient and usually requires values to already be instances of the expected type, though JSON-mode validation has specific looser rules for types such as dates and times. ([Pydantic][S06-1])

---

## 6.0 Coercion-policy mental model

```text
input value
  → validation mode: python | json | strings
  → coercion policy: lax | strict
  → field/core schema
  → validated output value OR ValidationError
```

```text
lax mode:
  "123" → int(123)
  "true" → bool(True)
  "2026-05-11" → date(...)
  compatible strings/bytes/numbers converted when table allows

strict mode:
  reject most compatible-but-not-exact Python inputs
  accept exact runtime types in Python mode
  allow JSON-native representations for some non-JSON-native Python types
```

The conversion table is the authoritative matrix for allowed conversions across strict/lax and Python/JSON input sources; it explicitly marks which conversions remain legal under strict mode. ([Pydantic][S06-2])

---

## 6.1 Lax mode default behavior

### Default constructor / validation behavior

```python
from pydantic import BaseModel

class M(BaseModel):
    x: int
    enabled: bool

m = M.model_validate({"x": "123", "enabled": "yes"})

assert m.x == 123
assert m.enabled is True
```

Default mode attempts compatible coercions; Pydantic’s strict-mode docs show `"123"` becoming `123` for an `int` field in lax mode and explain that this is useful for URL parameters, HTTP headers, environment variables, dates, and user input. ([Pydantic][S06-1])

### Agent policy

```text
Use lax mode when source is stringly or JSON-like:
  HTTP query params
  HTTP headers
  form data
  environment variables
  CLI arguments
  CSV rows
  LLM JSON/text output
  legacy APIs that stringify everything

Avoid lax mode when source should already be typed:
  internal Python APIs
  trusted service-to-service DTOs
  domain-layer value objects
  security-sensitive config
  regression tests checking upstream normalization
```

---

## 6.2 Strict-mode activation surfaces

```text
validation call:
  Model.model_validate(data, strict=True)
  Model.model_validate_json(raw, strict=True)
  TypeAdapter(T).validate_python(data, strict=True)
  TypeAdapter(T).validate_json(raw, strict=True)

field level:
  Field(strict=True)
  Annotated[T, Strict()]
  StrictInt / StrictStr / StrictBool / StrictFloat / StrictBytes

model/config level:
  model_config = ConfigDict(strict=True)

dataclass / TypedDict level:
  __pydantic_config__ = ConfigDict(strict=True)
  @with_config(ConfigDict(strict=True))
```

Pydantic documents strict mode as available per validation call, per field, and at configuration level; the `Strict()` metadata class and strict convenience aliases are documented as field-level strictness mechanisms. ([Pydantic][S06-1])

---

## 6.3 Validation-call strictness

## 6.3.1 `BaseModel.model_validate(data, strict=True)`

```python
from pydantic import BaseModel, ValidationError

class M(BaseModel):
    x: int

assert M.model_validate({"x": "123"}).x == 123

try:
    M.model_validate({"x": "123"}, strict=True)
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "int_type"
```

`strict=True` at call level overrides normal lax coercion for that validation operation; for Python-mode `int` validation, a string is rejected instead of parsed. ([Pydantic][S06-1])

### Use cases

```text
one-off hard boundary
tests that verify upstream already normalized data
admin/internal endpoints
service-to-service Python DTO validation
migration audits: discover where coercion was masking bad upstream types
```

---

## 6.3.2 `BaseModel.model_validate_json(raw, strict=True)`

```python
from datetime import date
from pydantic import BaseModel

class M(BaseModel):
    d: date

# JSON strict can still accept JSON string representation for date.
m = M.model_validate_json(b'{"d": "2026-05-11"}', strict=True)
assert m.d == date(2026, 5, 11)
```

Strict mode is not identical across Python and JSON modes: Pydantic states that looser rules may apply for JSON input because JSON lacks native representations for many Python types; date/time types can allow strings even in strict mode. ([Pydantic][S06-1])

### Agent rule

```text
Strict JSON validation ≠ strict Python validation.

If a wire format is JSON:
  test strict behavior with model_validate_json, not model_validate(json.loads(...)).
```

---

## 6.3.3 `TypeAdapter(T).validate_python(data, strict=True)`

```python
from pydantic import TypeAdapter, ValidationError

ta = TypeAdapter(list[int])

assert ta.validate_python(["1", 2]) == [1, 2]

try:
    ta.validate_python(["1", 2], strict=True)
except ValidationError as exc:
    # strictness behavior depends on outer + inner schemas;
    # inspect error types, not message prose.
    print(exc.errors())
```

`TypeAdapter` exposes validation for arbitrary types that are not `BaseModel` classes; strict mode can be supplied to TypeAdapter validation methods just as it can be supplied to model validation methods. ([Pydantic][S06-1])

### Best use

```text
top-level list[T]
top-level dict[K, V]
TypedDict
Union
primitive scalar
Annotated constrained alias
dataclass
```

---

## 6.4 Field-level strictness with `Field(strict=True)`

```python
from pydantic import BaseModel, Field, ValidationError

class Payment(BaseModel):
    amount_cents: int = Field(strict=True)
    memo: str

Payment.model_validate({"amount_cents": 100, "memo": "ok"})

try:
    Payment.model_validate({"amount_cents": "100", "memo": "ok"})
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "int_type"
```

`Field(strict=True)` applies strict validation to a specific field, allowing a mixed model where some fields remain ergonomic/lax while security-sensitive or already-normalized fields reject coercion. Pydantic documents `Field(strict=True)` as a strict-mode activation mechanism for model, dataclass, or TypedDict fields. ([Pydantic][S06-1])

### Mixed strict/lax policy

```python
class SearchParams(BaseModel):
    q: str
    limit: int = 25                      # lax: query strings can parse
    tenant_id: str = Field(strict=True)  # must already be str
```

Agent rule:

```text
Prefer field-level strictness when only some fields need hard type guarantees.
Avoid ConfigDict(strict=True) if most fields are stringly boundary inputs.
```

---

## 6.5 Field-level strictness with `Annotated[T, Strict()]`

```python
from typing import Annotated
from pydantic import BaseModel, Strict

StrictUserId = Annotated[int, Strict()]

class UserRef(BaseModel):
    id: StrictUserId
```

`Strict()` is metadata for the `Annotated` pattern; Pydantic also provides strict convenience aliases implemented through the same strict-metadata mechanism. ([Pydantic][S06-1])

### Recommended reusable aliases

```python
from typing import Annotated
from pydantic import Field, Strict

StrictPositiveInt = Annotated[int, Strict(), Field(gt=0)]
StrictShortStr = Annotated[str, Strict(), Field(min_length=1, max_length=64)]
StrictNonEmptyBytes = Annotated[bytes, Strict(), Field(min_length=1)]
```

Agent rule:

```text
Use Annotated[T, Strict(), Field(...)] when:
  type strictness + constraints must be reused
  you want a named domain type
  generated code should avoid deprecated con* helpers
```

---

## 6.6 Strict convenience aliases

```python
from pydantic import StrictBool, StrictBytes, StrictFloat, StrictInt, StrictStr

class M(BaseModel):
    ok: StrictBool
    raw: StrictBytes
    score: StrictFloat
    count: StrictInt
    name: StrictStr
```

Pydantic documents strict convenience aliases for common scalar types: `StrictBool`, `StrictInt`, `StrictFloat`, `StrictStr`, and `StrictBytes`; these require values to be of the corresponding type or subtype. ([Pydantic][S06-1])

### Alias vs `Annotated` policy

```text
StrictInt:
  compact
  readable
  good for simple scalar strictness

Annotated[int, Strict(), Field(gt=0)]:
  composable
  constraint-friendly
  preferred for domain aliases
```

---

## 6.7 Model-level strictness with `ConfigDict(strict=True)`

```python
from pydantic import BaseModel, ConfigDict

class StrictModel(BaseModel):
    model_config = ConfigDict(strict=True)

    name: str
    age: int
```

`ConfigDict(strict=True)` applies strict validation to all fields on the model by default; Pydantic’s config docs describe `strict` as a model-level setting that disables normal coercion and raises when the input type does not match the field annotation. ([Pydantic][S06-3])

### Project base model

```python
class DomainModel(BaseModel):
    model_config = ConfigDict(
        strict=True,
        extra="forbid",
        frozen=True,
    )
```

Use model-level strictness for domain/value-object layers where construction should occur only from already-normalized Python values.

### Field override

```python
class MostlyStrict(BaseModel):
    model_config = ConfigDict(strict=True)

    id: int
    raw_query_limit: int = Field(strict=False)
```

Field-level strictness settings can override configuration-level strictness. Pydantic’s strict-mode docs explicitly describe model-level strictness with the possibility to override at field level. ([Pydantic][S06-1])

---

## 6.8 Dataclass and TypedDict strict config

### Stdlib dataclass

```python
from dataclasses import dataclass
from pydantic import ConfigDict, TypeAdapter

@dataclass
class User:
    __pydantic_config__ = ConfigDict(strict=True)

    id: int
    name: str

ta = TypeAdapter(User)
```

### TypedDict with config

```python
from typing_extensions import TypedDict
from pydantic import ConfigDict, TypeAdapter, with_config

@with_config(ConfigDict(strict=True))
class UserDict(TypedDict):
    id: int
    name: str

ta = TypeAdapter(UserDict)
```

Pydantic config can be applied to `BaseModel`, Pydantic dataclasses, stdlib dataclasses, and `TypedDict`; for stdlib dataclasses and TypedDicts, docs show `__pydantic_config__` and `with_config(...)` as the supported patterns. ([Pydantic Docs][S06-4])

---

## 6.9 JSON input exceptions and looser JSON behavior

### Core issue

```text
Python strict mode:
  date field wants date instance

JSON strict mode:
  date field may accept "YYYY-MM-DD" string
```

Reason: JSON has only null, bool, number, string, array, and object; it has no native `date`, `datetime`, `UUID`, `Decimal`, `Path`, or `bytes` object. Pydantic’s strict-mode docs state that looser rules may apply to JSON input and give date/time strings as examples. ([Pydantic][S06-1])

### Demonstration

```python
from datetime import date
from pydantic import BaseModel, ValidationError

class M(BaseModel):
    d: date

try:
    M.model_validate({"d": "2026-05-11"}, strict=True)
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "date_type"

assert M.model_validate_json(b'{"d": "2026-05-11"}', strict=True).d == date(2026, 5, 11)
```

### Deployment rule

```text
Wire JSON contract strictness:
  test with model_validate_json(..., strict=True)

Internal Python contract strictness:
  test with model_validate(..., strict=True)

Do not infer one from the other.
```

---

## 6.10 Conversion table high-signal rules

### Boolean

```text
bool lax accepts:
  bool
  0 / 1
  0.0 / 1.0
  selected strings: "false", "true", "no", "yes", "off", "on", etc.

bool strict:
  bool only
```

The conversion table lists bool conversions from numeric and string values in lax mode and marks strict bool validation for bool inputs. ([Pydantic][S06-2])

### Integer

```text
int lax accepts:
  int
  numeric strings / bytes
  finite floats with zero fractional part
  Decimal/Fraction values representing integers
  Enum values

int strict:
  int values
  bool explicitly not treated as int for Pydantic integer validation
```

Pydantic’s standard-library type docs describe integer conversion from numeric strings, bytes, exact finite floats, finite Decimal/Fraction values, and enum values; strict integer validation accepts integer values. ([Pydantic][S06-5])

### Float

```text
float lax accepts:
  float
  numeric strings / bytes
  values with __float__ or __index__

float strict:
  float values and compatible numeric protocol objects depending type behavior
```

Pydantic’s standard-library type docs describe float conversion from strings/bytes and from objects with `__float__()` or fallback `__index__()`. ([Pydantic][S06-5])

### Bytes

```text
bytes Python strict:
  bytes only

bytes JSON strict:
  strict mode has no effect; JSON representation is string-like
```

The standard-library type docs state that strict mode for bytes requires bytes instances in Python mode and that strict mode has no effect in JSON mode. ([Pydantic][S06-5])

---

## 6.11 Strictness and containers

### Outer container strictness

```python
from pydantic import BaseModel, Field

class M(BaseModel):
    xs: list[int] = Field(strict=True)

M(xs=["1", 2])  # outer value is list; items may still coerce
```

Strictness on collection types applies to the **outer container** and does not automatically apply to contained item types; Pydantic’s standard-library type docs state this explicitly for collection types. ([Pydantic][S06-5])

### Strict items

```python
from typing import Annotated
from pydantic import Strict

StrictIntList = list[Annotated[int, Strict()]]

class M(BaseModel):
    xs: StrictIntList
```

### Strict outer + strict inner

```python
class M(BaseModel):
    xs: Annotated[list[Annotated[int, Strict()]], Field(strict=True)]
```

Agent rule:

```text
Field(strict=True) on list[T]:
  input must be list

Annotated[T, Strict()] inside list:
  each item must satisfy strict T

Use both when both outer container type and item types must be strict.
```

---

## 6.12 Strictness and unions

```python
class M(BaseModel):
    x: int | str
```

Strictness changes which union branches can succeed. In lax mode, `"123"` can validate as `int`; in strict Python mode, `"123"` cannot validate as `int` but can validate as `str`. For ambiguous unions, use discriminated unions or pin union-mode behavior with tests.

```python
from pydantic import Field

class M(BaseModel):
    x: int | str = Field(union_mode="left_to_right")
```

Agent rule:

```text
Strict mode reduces coercive branch matches.
It does not make ambiguous union design good.
Use discriminated unions for model polymorphism.
```

Pydantic documents strict/lax conversion through the conversion table and recommends discriminated unions as more predictable and performant for polymorphic model unions. ([Pydantic][S06-2])

---

## 6.13 Strict mode for API boundary models

### External JSON body

```python
class CreateUser(BaseModel):
    model_config = ConfigDict(extra="forbid")

    email: str
    age: int
    marketing_opt_in: bool = False
```

Recommended policy:

```text
public JSON body:
  lax by default for ergonomic parsing
  extra="forbid" for contract hygiene
  Field(strict=True) only for high-risk fields
  validate with model_validate_json(raw_body)
```

Rationale: JSON request bodies often contain valid JSON strings for dates, UUIDs, and other non-JSON-native values, and Pydantic explicitly notes that strict JSON mode can still allow certain string representations. ([Pydantic][S06-1])

### Strict API variant

```python
class StrictCreateUser(BaseModel):
    model_config = ConfigDict(strict=True, extra="forbid")

    email: str
    age: int
```

Use strict API models when the API contract explicitly says clients must send correct JSON scalar types, not stringified numeric/boolean values.

---

## 6.14 Strict mode for internal domain models

```python
class DomainModel(BaseModel):
    model_config = ConfigDict(
        strict=True,
        extra="forbid",
        frozen=True,
    )

class AccountId(DomainModel):
    value: int
```

Recommended policy:

```text
internal domain/value object:
  ConfigDict(strict=True)
  ConfigDict(extra="forbid")
  frozen=True if value object
  avoid stringly coercion
  fail fast on upstream bugs
```

Pydantic config supports model-level `strict=True`, and config is inherited when a project defines a custom parent `BaseModel`. ([Pydantic][S06-3])

---

## 6.15 Strict mode for settings models

```python
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="APP_")

    port: int = 8000
    debug: bool = False
```

Recommended policy:

```text
settings/env models:
  generally lax
  env vars are strings
  validate defaults
  strict only for fields expected to be injected as typed values from non-env sources
  prefer constrained types over global strict=True
```

`BaseSettings` reads values from environment variables and secrets sources; settings docs state that defaults are validated by default, unlike plain `BaseModel`, and environment variable names/aliases/prefixes control the external source mapping. ([Pydantic][S06-6])

### Selective strict settings

```python
from pydantic import Field

class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="APP_")

    # env string "5432" should parse
    port: int = 5432

    # injected by code, not expected from env
    runtime_worker_count: int = Field(default=4, strict=True)
```

Agent rule:

```text
Do not put ConfigDict(strict=True) on BaseSettings unless every source supplies already-typed Python objects.
For env/dotenv/secrets, lax parsing is usually the feature.
```

---

## 6.16 Strict mode for CLI/env parsing models

```python
class CliArgs(BaseModel):
    path: str
    limit: int = 100
    verbose: bool = False
```

Recommended validation:

```python
args = CliArgs.model_validate_strings({
    "path": "/tmp/data.json",
    "limit": "500",
    "verbose": "true",
})
```

Policy:

```text
CLI/env/query/form inputs:
  use model_validate_strings
  keep scalar fields lax unless the CLI parser already typed them
  use constraints for range/shape
  use Strict only after CLI parser normalization
```

Pydantic’s strict docs explicitly list environment variables, URL parameters, HTTP headers, and user input as scenarios where coercion can be useful. ([Pydantic][S06-1])

---

## 6.17 Coercion-policy architecture patterns

### Pattern A — boundary lax, domain strict

```python
class CreateUserRequest(BaseModel):
    model_config = ConfigDict(extra="forbid")

    age: int
    email: str

class UserDomain(BaseModel):
    model_config = ConfigDict(strict=True, frozen=True)

    age: int
    email: str

def normalize_user(raw: bytes) -> UserDomain:
    request = CreateUserRequest.model_validate_json(raw)
    return UserDomain.model_validate(request.model_dump())
```

Value:

```text
external ergonomics
internal type hardening
clear normalization seam
```

---

### Pattern B — strict audit mode

```python
def audit_payload(data: dict) -> None:
    try:
        Model.model_validate(data, strict=True)
    except ValidationError as exc:
        report = exc.errors()
        ...
```

Value:

```text
detect coercion dependency
prepare migration to stricter APIs
identify upstream stringification bugs
```

---

### Pattern C — selective strict fields

```python
class PaymentCommand(BaseModel):
    amount_cents: int = Field(strict=True, gt=0)
    currency: Literal["USD", "EUR"]
    idempotency_key: str = Field(min_length=16, max_length=128)
```

Value:

```text
hard type guarantee where correctness matters
no unnecessary strictness for all fields
```

---

### Pattern D — reusable strict constrained aliases

```python
StrictPositiveInt = Annotated[int, Strict(), Field(gt=0)]
StrictNonEmptyStr = Annotated[str, Strict(), Field(min_length=1)]

class Command(BaseModel):
    attempts: StrictPositiveInt
    name: StrictNonEmptyStr
```

Value:

```text
single contract definition
consistent behavior across models
testable domain aliases
```

---

## 6.18 Anti-patterns

### Anti-pattern: global strict on settings

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(strict=True)

    port: int
```

Problem:

```text
env var APP_PORT="8000" is a string
global strict fights environment-variable reality
```

Preferred:

```python
class Settings(BaseSettings):
    port: Annotated[int, Field(ge=1, le=65535)]
```

---

### Anti-pattern: strict outer list only

```python
class M(BaseModel):
    xs: list[int] = Field(strict=True)
```

Problem:

```text
tuple rejected as outer input
but "1" inside list can still coerce to int
```

Preferred:

```python
class M(BaseModel):
    xs: Annotated[list[Annotated[int, Strict()]], Field(strict=True)]
```

---

### Anti-pattern: assuming strict JSON rejects all strings

```python
class M(BaseModel):
    d: date

M.model_validate_json(b'{"d": "2026-05-11"}', strict=True)  # may be valid
```

Problem:

```text
JSON has no date scalar
strict JSON mode permits certain string encodings
```

Pydantic documents looser JSON strict behavior for date/time-like types. ([Pydantic][S06-1])

---

### Anti-pattern: strict mode as business validation

```python
class Order(BaseModel):
    user_id: StrictInt
```

Problem:

```text
StrictInt proves runtime type.
It does not prove user exists, is authorized, or may place order.
```

Preferred:

```python
class OrderCommand(BaseModel):
    user_id: int = Field(strict=True)

def place_order(cmd: OrderCommand, repo: UserRepo, actor: Actor) -> Order:
    user = repo.require_user(cmd.user_id)
    authorize(actor, user)
    ...
```

---

## 6.19 Test matrix

### Lax vs strict call-level

```python
import pytest
from pydantic import BaseModel, ValidationError

def test_call_level_strict():
    class M(BaseModel):
        x: int

    assert M.model_validate({"x": "1"}).x == 1

    with pytest.raises(ValidationError) as exc:
        M.model_validate({"x": "1"}, strict=True)

    assert exc.value.errors()[0]["type"] == "int_type"
```

### Field-level strict

```python
def test_field_level_strict():
    class M(BaseModel):
        lax: int
        strict: int = Field(strict=True)

    assert M.model_validate({"lax": "1", "strict": 1}).strict == 1

    with pytest.raises(ValidationError):
        M.model_validate({"lax": "1", "strict": "1"})
```

### `Annotated[..., Strict()]`

```python
def test_annotated_strict():
    StrictPositiveInt = Annotated[int, Strict(), Field(gt=0)]

    class M(BaseModel):
        x: StrictPositiveInt

    assert M(x=1).x == 1

    with pytest.raises(ValidationError):
        M(x="1")
```

### Model-level strict

```python
def test_model_level_strict():
    class M(BaseModel):
        model_config = ConfigDict(strict=True)
        x: int

    with pytest.raises(ValidationError):
        M.model_validate({"x": "1"})
```

### JSON strict exception

```python
from datetime import date

def test_json_strict_date_string():
    class M(BaseModel):
        d: date

    with pytest.raises(ValidationError):
        M.model_validate({"d": "2026-05-11"}, strict=True)

    assert M.model_validate_json(b'{"d": "2026-05-11"}', strict=True).d == date(2026, 5, 11)
```

### Strict outer vs strict inner

```python
def test_strict_outer_not_inner():
    class OuterStrict(BaseModel):
        xs: list[int] = Field(strict=True)

    assert OuterStrict(xs=["1"]).xs == [1]

    class OuterAndInnerStrict(BaseModel):
        xs: Annotated[list[Annotated[int, Strict()]], Field(strict=True)]

    with pytest.raises(ValidationError):
        OuterAndInnerStrict(xs=["1"])
```

---

## 6.20 Deployment decision matrix

```text
Boundary / model type                  Recommended coercion policy
--------------------------------------------------------------------------------
HTTP JSON request body                  lax model + selective strict fields
HTTP query / headers / form             model_validate_strings + lax scalars
CLI args                                model_validate_strings + constraints
Environment settings                    BaseSettings lax + constrained fields
Internal domain model                   ConfigDict(strict=True)
Financial command fields                Field(strict=True) + Decimal/int cents
Security-sensitive flags                StrictBool / Field(strict=True)
LLM structured output                   lax JSON ingestion + strict post-normalization if needed
Message queue event                     lax JSON parse if producer stringifies; strict if schema guarantees typed JSON
SDK/library public model                documented policy; avoid surprising global strict
Test/audit mode                         per-call strict=True
```

---

## 6.21 Agent checklist

```text
[ ] Identify input mode: Python, JSON, or strings.
[ ] Identify trust level: untrusted boundary vs internal normalized data.
[ ] Default to lax at stringly boundaries.
[ ] Use strict at internal/domain boundaries.
[ ] Use Field(strict=True) for targeted hard fields.
[ ] Use Annotated[T, Strict(), Field(...)] for reusable strict constrained aliases.
[ ] Use StrictInt/StrictStr/etc. only for simple scalar strictness.
[ ] Use ConfigDict(strict=True) only when most fields should be strict.
[ ] Remember strict JSON can still accept date/time strings.
[ ] Remember strict collection outer type does not imply strict item types.
[ ] Avoid global strict on BaseSettings unless values are already typed.
[ ] Test strict/lax behavior with the actual validation entrypoint used in production.
[ ] Assert ValidationError.errors()[*]["type"], not message prose.
```

---

## 6.22 Value case

```text
Strict/lax policy value =
  boundary-specific coercion control
  safer internal domain objects
  gradual migration from permissive to hardened parsing
  targeted hard guarantees for high-risk fields
  predictable tests for upstream normalization
  cleaner separation between parsing convenience and business correctness
```

Operationally: keep lax parsing where data is naturally stringly or JSON-encoded, move into strict domain models after normalization, use field-level strictness for high-risk values, and test Python-mode and JSON-mode strictness separately.

[S06-1]: https://pydantic.dev/docs/validation/latest/concepts/strict_mode/?utm_source=chatgpt.com "Strict Mode | Pydantic Docs"
[S06-2]: https://pydantic.dev/docs/validation/latest/concepts/conversion_table/?utm_source=chatgpt.com "Conversion Table | Pydantic Docs"
[S06-3]: https://pydantic.dev/docs/validation/latest/api/pydantic/config/?utm_source=chatgpt.com "Configuration | Pydantic Docs"
[S06-4]: https://docs.pydantic.dev/2.7/concepts/config/?utm_source=chatgpt.com "Config | Pydantic Docs"
[S06-5]: https://pydantic.dev/docs/validation/latest/api/pydantic/standard_library_types/?utm_source=chatgpt.com "Standard Library Types"
[S06-6]: https://pydantic.dev/docs/validation/latest/concepts/pydantic_settings/?utm_source=chatgpt.com "Settings Management"


# Pydantic Advanced — 7) Validators — field-level and model-level validation

Style target: dense, sectioned, agent-oriented technical reference. 

Pydantic supports custom validation at **field** and **model** levels for constraints that are more complex than built-in type validation. Field validators can be defined through either the `Annotated[...]` metadata pattern or the `@field_validator(...)` decorator; Pydantic documents four field-validator modes: `after`, `before`, `plain`, and `wrap`. Model validators are declared with `@model_validator(...)` and support `after`, `before`, and `wrap` modes. ([Pydantic Docs][S07-1])

---

## 7.0 Validator mental model

```text
raw input
  → before validators
  → Pydantic internal validation / coercion
  → after validators
  → validated field value / model instance
```

```text
plain validator:
  raw input → user function → final value
  bypasses Pydantic internal validation

wrap validator:
  raw input → user function(handler) → optionally call internal validation → final value
```

Agent invariant:

```text
Field validators:
  operate on one field's value.
  may run before, after, instead of, or around Pydantic internal validation.

Model validators:
  operate on whole input or whole validated instance.
  best for cross-field invariants, whole-object normalization, audit hooks.
```

---

## 7.1 API surface overview

### Annotated field validators

```python
from typing import Annotated
from pydantic import BeforeValidator, AfterValidator, PlainValidator, WrapValidator

MyType = Annotated[
    int,
    BeforeValidator(...),
    AfterValidator(...),
]
```

Functional validator metadata classes:

```text
AfterValidator(func)
  apply after inner validation

BeforeValidator(func, json_schema_input_type=...)
  apply before inner validation

PlainValidator(func, json_schema_input_type=...)
  apply instead of inner validation

WrapValidator(func, json_schema_input_type=...)
  apply around inner validation through handler
```

The functional validator API describes `AfterValidator` as validation after inner validation, `BeforeValidator` as validation before inner validation, `PlainValidator` as validation instead of inner validation, and `WrapValidator` as validation around inner validation. `BeforeValidator`, `PlainValidator`, and `WrapValidator` can also carry `json_schema_input_type` for validation-mode JSON Schema input shape. ([Pydantic Docs][S07-2])

---

### Decorator field validators

```python
from pydantic import BaseModel, field_validator

class Model(BaseModel):
    x: int

    @field_validator("x", mode="after")
    @classmethod
    def validate_x(cls, value: int) -> int:
        return value
```

Decorator modes:

```text
@field_validator("x")                  # default mode="after"
@field_validator("x", mode="after")
@field_validator("x", mode="before")
@field_validator("x", mode="plain")
@field_validator("x", mode="wrap")
```

`field_validator` accepts one or more field names, a `mode`, optional `check_fields`, and `json_schema_input_type` for `before`, `plain`, or `wrap` modes; the default mode is `after`. ([Pydantic Docs][S07-2])

---

### Decorator model validators

```python
from typing_extensions import Self
from pydantic import BaseModel, model_validator

class Model(BaseModel):
    x: int

    @model_validator(mode="after")
    def validate_model(self) -> Self:
        return self
```

Decorator modes:

```text
@model_validator(mode="after")
@model_validator(mode="before")
@model_validator(mode="wrap")
```

`model_validator` requires a `mode` literal of `'wrap'`, `'before'`, or `'after'`; `after` validators are instance methods that should return the validated instance. ([Pydantic Docs][S07-2])

---

## 7.2 Validator mode taxonomy

```text
after:
  input to validator = already validated/coerced value
  safest default
  type-friendly
  should return value

before:
  input to validator = raw input
  flexible pre-parser
  must handle Any
  returned value still goes through Pydantic validation

plain:
  input to validator = raw input
  terminates validation
  Pydantic does not validate against annotation afterward
  dangerous unless intentionally replacing validation

wrap:
  input to validator = raw input + handler
  can call handler(value)
  can pre-process, post-process, catch ValidationError, retry, short-circuit
  most powerful, slowest class
```

Pydantic’s validators docs explicitly state that before validators receive raw input and should usually type their input as `Any`; plain validators terminate validation immediately so Pydantic does not validate the value against the field type; wrap validators receive an extra handler and can run code before or after Pydantic validation, catch validation errors, or avoid calling the handler. ([Pydantic Docs][S07-1])

---

## 7.3 `AfterValidator`: validated-value checks and normalization

### Annotated form

```python
from typing import Annotated
from pydantic import AfterValidator, BaseModel, ValidationError

def is_even(value: int) -> int:
    if value % 2:
        raise ValueError(f"{value} is not even")
    return value

EvenInt = Annotated[int, AfterValidator(is_even)]

class Model(BaseModel):
    n: EvenInt
```

### Decorator form

```python
from pydantic import BaseModel, field_validator

class Model(BaseModel):
    n: int

    @field_validator("n")
    @classmethod
    def is_even(cls, value: int) -> int:
        if value % 2:
            raise ValueError("must be even")
        return value
```

Use cases:

```text
range checks not expressible with Field
semantic shape checks after parsing
normalization that needs target type
cross-field field validator using info.data
cheap deterministic domain checks
```

Value case:

```text
after validator = safest custom validator mode
because input is already target type
```

Pydantic recommends after validators as generally more type-safe and easier to implement because they run after internal validation. ([Pydantic Docs][S07-1])

---

## 7.4 `BeforeValidator`: raw-input parsing / pre-normalization

### Annotated form

```python
from typing import Annotated, Any
from pydantic import BaseModel, BeforeValidator

def ensure_list(value: Any) -> Any:
    if isinstance(value, list):
        return value
    return [value]

class Model(BaseModel):
    numbers: Annotated[list[int], BeforeValidator(ensure_list)]
```

### Decorator form

```python
from typing import Any
from pydantic import BaseModel, field_validator

class Model(BaseModel):
    numbers: list[int]

    @field_validator("numbers", mode="before")
    @classmethod
    def ensure_list(cls, value: Any) -> Any:
        if isinstance(value, list):
            return value
        return [value]
```

Use cases:

```text
single value → list
legacy input shape normalization
string cleanup before parsing
dict key migration
accept old and new payload shapes
normalize raw API variants before typed validation
```

Hazards:

```text
input type is Any
must handle arbitrary objects
mutating raw input before raising can leak mutated value into other validators/unions
returned value still validated by Pydantic
```

Pydantic warns that before validators receive raw input that can be any arbitrary object and advises avoiding direct mutation if a validation error may later be raised, especially with unions. ([Pydantic Docs][S07-1])

---

## 7.5 `PlainValidator`: full validation replacement

```python
from typing import Annotated, Any
from pydantic import BaseModel, PlainValidator

def parse_or_return(value: Any) -> Any:
    if isinstance(value, int):
        return value * 2
    return value

class Model(BaseModel):
    n: Annotated[int, PlainValidator(parse_or_return)]

assert Model(n="not an int").n == "not an int"
```

Meaning:

```text
PlainValidator:
  receives raw input
  returns final field value
  prevents Pydantic type validation from running after it
```

Use cases:

```text
complete custom parser
bridge to trusted external parser
performance-critical specialized decoding
intentional non-standard field output
```

Hazards:

```text
can violate annotation
can create invalid model states
can make JSON Schema misleading unless json_schema_input_type is supplied
should be rare
```

Pydantic documents that plain validators terminate validation immediately and that Pydantic will not validate against the annotated field type afterward; the docs show an `int` field accepting an invalid string because a plain validator returned it. ([Pydantic Docs][S07-1])

---

## 7.6 `WrapValidator`: handler-controlled validation pipeline

### Annotated form

```python
from typing import Annotated, Any
from pydantic import BaseModel, Field, ValidationError, ValidatorFunctionWrapHandler, WrapValidator

def truncate(value: Any, handler: ValidatorFunctionWrapHandler) -> str:
    try:
        return handler(value)
    except ValidationError as err:
        if err.errors()[0]["type"] == "string_too_long":
            return handler(value[:5])
        raise

ShortString = Annotated[str, Field(max_length=5), WrapValidator(truncate)]

class Model(BaseModel):
    s: ShortString
```

### Decorator form

```python
from typing import Any
from pydantic import BaseModel, Field, ValidationError, ValidatorFunctionWrapHandler, field_validator

class Model(BaseModel):
    s: str = Field(max_length=5)

    @field_validator("s", mode="wrap")
    @classmethod
    def truncate(cls, value: Any, handler: ValidatorFunctionWrapHandler) -> str:
        try:
            return handler(value)
        except ValidationError as err:
            if err.errors()[0]["type"] == "string_too_long":
                return handler(value[:5])
            raise
```

Use cases:

```text
fallback validation
retry with normalized value after error
conditional short-circuit
logging failed validation
preserve input container type
advanced compatibility migrations
```

Performance warning:

```text
Wrap validators require Python materialization during validation.
Use after/before validators when wrap power is unnecessary.
```

Pydantic’s performance docs state that wrap validators are generally slower because data must be materialized in Python during validation, even though wrap validators are useful for complex logic. ([Pydantic Docs][S07-3])

---

## 7.7 Field validator decorator patterns

### Multi-field validator

```python
from pydantic import BaseModel, field_validator

class Model(BaseModel):
    first_name: str
    last_name: str

    @field_validator("first_name", "last_name", mode="before")
    @classmethod
    def strip_and_title(cls, value: str) -> str:
        return value.strip().title()
```

### All-fields validator

```python
class Model(BaseModel):
    a: str
    b: str

    @field_validator("*", mode="before")
    @classmethod
    def strip_all_strings(cls, value):
        if isinstance(value, str):
            return value.strip()
        return value
```

### Base-class validator for subclass fields

```python
class StrippingModel(BaseModel):
    @field_validator("*", mode="before", check_fields=False)
    @classmethod
    def strip_strings(cls, value):
        if isinstance(value, str):
            return value.strip()
        return value
```

`field_validator("*", ...)` applies to all fields, including subclass-defined fields; `check_fields=False` disables class-creation-time checks that named fields exist, which is useful for base classes expecting subclasses to define the fields. ([Pydantic Docs][S07-1])

---

## 7.8 Annotated pattern vs decorator pattern

### Annotated pattern: reusable type-level validators

```python
from typing import Annotated
from pydantic import AfterValidator

def is_even(v: int) -> int:
    if v % 2:
        raise ValueError("must be even")
    return v

EvenInt = Annotated[int, AfterValidator(is_even)]

class A(BaseModel):
    n: EvenInt

class B(BaseModel):
    values: list[EvenInt]  # item-level validation
```

Benefits:

```text
reusable
visible in type annotation
applies to container items when placed inside container
composable with Field constraints
good for domain aliases
```

### Decorator pattern: model-local / multi-field validators

```python
class Model(BaseModel):
    f1: str
    f2: str

    @field_validator("f1", "f2", mode="before")
    @classmethod
    def capitalize(cls, value: str) -> str:
        return value.capitalize()
```

Benefits:

```text
apply one function to multiple fields
access cls.model_fields
use inheritance/base classes
keep model-local policies near model
```

Pydantic documents the annotated pattern as useful for reusable validators and item-level validation inside containers; the decorator pattern is documented as useful for applying a function to multiple fields. ([Pydantic Docs][S07-1])

---

## 7.9 Model validators

## 7.9.1 Model `after`: cross-field invariant after full validation

```python
from typing_extensions import Self
from pydantic import BaseModel, model_validator

class Signup(BaseModel):
    password: str
    password_repeat: str

    @model_validator(mode="after")
    def passwords_match(self) -> Self:
        if self.password != self.password_repeat:
            raise ValueError("passwords do not match")
        return self
```

Use cases:

```text
cross-field invariant
post-parse whole-object consistency
derived invariant not represented as one field
instance-level validation
```

Pydantic documents model after validators as running after the whole model has been validated, defined as instance methods, and returning the validated instance. ([Pydantic Docs][S07-1])

---

## 7.9.2 Model `before`: raw input object guard / migration

```python
from typing import Any
from pydantic import BaseModel, model_validator

class User(BaseModel):
    username: str

    @model_validator(mode="before")
    @classmethod
    def reject_card_number(cls, data: Any) -> Any:
        if isinstance(data, dict) and "card_number" in data:
            raise ValueError("card_number should not be included")
        return data
```

Use cases:

```text
reject forbidden raw keys
rename raw keys
legacy envelope unwrap
version migration before typed validation
attribute-object guard when from_attributes=True
```

Hazards:

```text
data can be dict, model instance, ORM object, or arbitrary object
must handle Any
avoid mutation before raising
```

Pydantic states that model before validators run before instantiation, receive raw input that may be any object, and may receive arbitrary class instances when `from_attributes` is enabled. ([Pydantic Docs][S07-1])

---

## 7.9.3 Model `wrap`: whole-model instrumentation / fallback

```python
import logging
from typing import Any
from typing_extensions import Self
from pydantic import BaseModel, ModelWrapValidatorHandler, ValidationError, model_validator

class User(BaseModel):
    username: str

    @model_validator(mode="wrap")
    @classmethod
    def log_failed_validation(
        cls,
        data: Any,
        handler: ModelWrapValidatorHandler[Self],
    ) -> Self:
        try:
            return handler(data)
        except ValidationError:
            logging.exception("User validation failed")
            raise
```

Use cases:

```text
logging validation failures
metrics/tracing around validation
retry after model-level migration
short-circuit trusted instances
whole-model fallback
```

Model wrap validators are the most flexible model validators: they can run code before or after Pydantic validation, terminate early by returning data, or raise errors. ([Pydantic Docs][S07-1])

---

## 7.10 Model validator inheritance

```python
class BaseUser(BaseModel):
    username: str

    @model_validator(mode="after")
    def base_rule(self):
        return self

class AdminUser(BaseUser):
    role: str
```

Inheritance semantics:

```text
base class model validators run for subclass validation
subclass validator with same method name overrides base validator
```

Pydantic documents that a model validator defined in a base class is called during subclass validation and that overriding a model validator in a subclass replaces the base validator. ([Pydantic Docs][S07-1])

---

## 7.11 Validator ordering

### Annotated order

```python
from typing import Annotated
from pydantic import AfterValidator, BeforeValidator, WrapValidator

Name = Annotated[
    str,
    AfterValidator(runs_3rd),
    AfterValidator(runs_4th),
    BeforeValidator(runs_2nd),
    WrapValidator(runs_1st),
]
```

Ordering rule:

```text
before validators: right → left
wrap validators:   right → left
after validators:  left → right
```

Decorator validators are internally converted to annotated-form metadata and added after existing field metadata, so the same ordering logic applies. ([Pydantic Docs][S07-1])

### Agent rule

```text
Put raw coercion/migration closest to the right when using Annotated.
Put final semantic checks as AfterValidator toward the left.
Prefer simple single-validator aliases when order would be non-obvious.
```

---

## 7.12 ValidationInfo

### Signature pattern

```python
from pydantic import ValidationInfo, field_validator

@field_validator("x")
@classmethod
def validate_x(cls, value: int, info: ValidationInfo) -> int:
    ...
```

Available information:

```text
info.data        already validated field data; field validators only
info.context     context passed to model_validate / model_validate_json / model_validate_strings
info.mode        "python" | "json" | "strings"
info.field_name  current field name; field validators only
info.config      model config dictionary, when available
```

Pydantic documents that field and model validator callables can optionally receive `ValidationInfo`; it provides already-validated data, user-defined context, validation mode, and field name for field validators. The migration guide shows `info.config` and `info.field_name` replacing v1-style `config` and `field` arguments. ([Pydantic Docs][S07-1])

---

### `info.data`: already-validated earlier fields

```python
from pydantic import BaseModel, ValidationInfo, field_validator

class Signup(BaseModel):
    password: str
    password_repeat: str
    username: str

    @field_validator("password_repeat", mode="after")
    @classmethod
    def passwords_match(cls, value: str, info: ValidationInfo) -> str:
        if value != info.data["password"]:
            raise ValueError("passwords do not match")
        return value
```

Caution:

```text
Fields validate in field-definition order.
info.data contains only fields already validated earlier.
info.data is None for model validators.
```

Pydantic explicitly warns that validation is performed in field definition order and that `info.data` does not include fields defined later; it is `None` for model validators. ([Pydantic Docs][S07-1])

---

### `info.context`: runtime policy injection

```python
class TextModel(BaseModel):
    text: str

    @field_validator("text")
    @classmethod
    def remove_stopwords(cls, v: str, info: ValidationInfo) -> str:
        if isinstance(info.context, dict):
            stopwords = set(info.context.get("stopwords", ()))
            return " ".join(w for w in v.split() if w.lower() not in stopwords)
        return v

TextModel.model_validate(
    {"text": "This is an example document"},
    context={"stopwords": ["this", "is", "an"]},
)
```

Context limitation:

```text
Model.model_validate(..., context=...) supports context.
Model(...) constructor does not directly accept validation context.
```

Pydantic states that context can be passed to validation methods and accessed through `ValidationInfo.context`; it also states that directly instantiating with `Model(...)` cannot currently receive context without a custom workaround. ([Pydantic Docs][S07-1])

---

### `info.config` and `info.field_name`

```python
class Model(BaseModel):
    x: int

    @field_validator("x")
    @classmethod
    def val_x(cls, v: int, info: ValidationInfo) -> int:
        assert info.config is not None
        title = info.config.get("title")
        field_required = cls.model_fields[info.field_name].is_required()
        return v
```

Use cases:

```text
shared base-class validators
config-driven behavior
field metadata inspection
multi-field generic validators
```

Pydantic’s migration guide states that v1 `config` and `field` validator arguments were removed; use `info.config` and `info.field_name` plus `cls.model_fields[...]` instead. ([Pydantic Docs][S07-4])

---

## 7.13 Raising validation errors

### Allowed validator error types

```text
ValueError
AssertionError
PydanticCustomError
```

Pydantic documents these three exception classes as the intended ways to raise validation errors from validators. It warns that `assert`-based validators are skipped when Python runs with the `-O` optimization flag. ([Pydantic Docs][S07-1])

---

### `ValueError`: default choice

```python
@field_validator("age")
@classmethod
def adult(cls, v: int) -> int:
    if v < 18:
        raise ValueError("must be at least 18")
    return v
```

Use for:

```text
ordinary validation failures
human-readable messages
simple domain constraints
```

---

### `AssertionError`: concise but optimization-sensitive

```python
@field_validator("n")
@classmethod
def positive(cls, v: int) -> int:
    assert v > 0, "must be positive"
    return v
```

Avoid in production-critical validators if processes may run with `python -O`.

---

### `PydanticCustomError`: stable custom error type + context

```python
from pydantic_core import PydanticCustomError

@field_validator("x")
@classmethod
def not_answer(cls, v: int) -> int:
    if v % 42 == 0:
        raise PydanticCustomError(
            "the_answer_error",
            "{number} is the answer!",
            {"number": v},
        )
    return v
```

Use for:

```text
machine-readable error type
custom ctx payload
localized/customized error handling
API error mapping
```

Pydantic’s docs show `PydanticCustomError` producing a custom error type and interpolated context in validation output. ([Pydantic Docs][S07-1])

---

### `TypeError` is not a validation error in v2

```python
@field_validator("x")
@classmethod
def bad(cls, v: int) -> int:
    return str.lower(v)  # TypeError if v is int
```

In Pydantic v2, a `TypeError` raised inside a validator is no longer converted to `ValidationError`; it propagates as `TypeError`. Use `ValueError` or `PydanticCustomError` for intentional validation failures. ([Pydantic Docs][S07-4])

---

## 7.14 Defaults and validators

```python
from pydantic import BaseModel, Field, field_validator

class Model(BaseModel):
    x: int = Field(default=0, validate_default=True)

    @field_validator("x")
    @classmethod
    def nonzero(cls, v: int) -> int:
        if v == 0:
            raise ValueError("must not be zero")
        return v
```

Default values are not validated unless configured, and custom validators are not applied to defaults unless default validation is enabled. ([Pydantic Docs][S07-1])

Agent rule:

```text
Validators do not automatically protect default values.
Use Field(validate_default=True) or model config when default correctness matters.
```

---

## 7.15 JSON Schema interaction

### `json_schema_input_type`

```python
from typing import Any
from pydantic import BaseModel, field_validator

class Model(BaseModel):
    value: int

    @field_validator(
        "value",
        mode="before",
        json_schema_input_type=str | int,
    )
    @classmethod
    def parse_int(cls, v: Any) -> Any:
        return v
```

Meaning:

```text
json_schema_input_type:
  affects generated validation-mode JSON Schema
  allowed for before/plain/wrap field validators
  not valid for after validators
```

The functional validator API documents `json_schema_input_type` for `field_validator` and states it can only be specified for `before`, `plain`, or `wrap` modes. ([Pydantic Docs][S07-2])

---

## 7.16 Parsing vs validation

### Parsing / normalization

Use `before` or `wrap`:

```python
def ensure_list(v: Any) -> Any:
    return v if isinstance(v, list) else [v]
```

Characteristics:

```text
input may be Any
normalizes legacy/wire shape
returned value still validated, except plain validators
should be deterministic and side-effect-free
```

### Validation / invariant enforcement

Use `after`:

```python
def is_even(v: int) -> int:
    if v % 2:
        raise ValueError("must be even")
    return v
```

Characteristics:

```text
input already target type
best for semantic constraints
simplest error behavior
lowest complexity
```

Agent rule:

```text
before = parse/reshape raw input
after  = check typed value
plain  = replace validation entirely
wrap   = intercept/retry/log/fallback
```

---

## 7.17 Mutation vs pure validation

### Pure validator

```python
@field_validator("email", mode="after")
@classmethod
def normalize_email(cls, v: str) -> str:
    return v.strip().lower()
```

### Risky raw mutation

```python
@model_validator(mode="before")
@classmethod
def mutate_raw(cls, data: Any) -> Any:
    if isinstance(data, dict):
        data["x"] = "changed"  # mutates caller-owned object
        raise ValueError("bad")
    return data
```

Agent rule:

```text
Prefer returning new values.
Avoid mutating caller-owned raw objects.
If mutating for speed, never raise afterward.
With unions, mutated values may flow to other branches.
```

Pydantic warns against mutating values directly in before validators if a validation error may later be raised, because the mutated value may be passed to other validators when unions are involved. ([Pydantic Docs][S07-1])

---

## 7.18 Reusable validators

### Reusable `Annotated` alias

```python
from typing import Annotated
from pydantic import AfterValidator, Field

def normalize_slug(v: str) -> str:
    v = v.strip().lower()
    if not v:
        raise ValueError("empty slug")
    return v

Slug = Annotated[
    str,
    Field(min_length=1, max_length=128, pattern=r"^[a-z0-9-]+$"),
    AfterValidator(normalize_slug),
]
```

### Item-level reusable validator

```python
class ArticleBatch(BaseModel):
    slugs: list[Slug]
```

### Shared decorator validator

```python
class Person(BaseModel):
    first_name: str
    last_name: str

    @field_validator("first_name", "last_name", mode="before")
    @classmethod
    def normalize_name(cls, v: str) -> str:
        return v.strip().title()
```

Pydantic’s docs explicitly call out the annotated pattern as a reuse mechanism and show it being used for list item validation; the decorator pattern is highlighted for applying the same validator to multiple fields. ([Pydantic Docs][S07-1])

---

## 7.19 Performance policy

```text
Fast path:
  built-in constraints
  Field(...)
  annotated-types constraints
  after validators

Moderate:
  before validators
  decorator validators across fields

Slow / expensive:
  wrap validators
  model-level wrap validators
  Python-level fallback/retry loops
```

Performance rules:

```text
Use Field constraints before custom validators.
Use AfterValidator when data is already typed.
Avoid WrapValidator unless handler control is required.
Use discriminated unions over broad unions.
Use TypeAdapter once, reused, for non-model types.
Use Any when no validation is needed.
```

Pydantic’s performance guide recommends avoiding validation when unnecessary, using `Any` to keep values unchanged, preferring tagged unions, reusing `TypeAdapter`, and avoiding wrap validators when performance matters. ([Pydantic Docs][S07-3])

---

## 7.20 Deployment recipes

### API input model: simple field + model validators

```python
from typing_extensions import Self
from pydantic import BaseModel, ConfigDict, EmailStr, Field, field_validator, model_validator

class SignupRequest(BaseModel):
    model_config = ConfigDict(extra="forbid")

    email: EmailStr
    password: str = Field(min_length=12)
    password_repeat: str

    @field_validator("email", mode="after")
    @classmethod
    def normalize_email(cls, v: EmailStr) -> EmailStr:
        return str(v).lower()

    @model_validator(mode="after")
    def passwords_match(self) -> Self:
        if self.password != self.password_repeat:
            raise ValueError("passwords do not match")
        return self
```

Policy:

```text
Use Field constraints for local constraints.
Use field validators for typed field normalization.
Use model after validators for cross-field invariants.
```

---

### Legacy payload migration

```python
from typing import Any
from pydantic import BaseModel, model_validator

class Event(BaseModel):
    event_type: str
    user_id: str

    @model_validator(mode="before")
    @classmethod
    def unwrap_legacy_payload(cls, data: Any) -> Any:
        if isinstance(data, dict) and "payload" in data:
            payload = data["payload"]
            if isinstance(payload, dict):
                return payload
        return data
```

Policy:

```text
Use model before validators for whole-payload shape migration.
Return a new object when practical.
Keep migration deterministic and side-effect-free.
```

---

### Validation with context

```python
from pydantic import BaseModel, ValidationInfo, field_validator

class LocalizedText(BaseModel):
    text: str

    @field_validator("text")
    @classmethod
    def normalize_locale(cls, v: str, info: ValidationInfo) -> str:
        locale = (info.context or {}).get("locale") if isinstance(info.context, dict) else None
        if locale == "tr":
            return v  # placeholder for locale-aware handling
        return v.strip()

LocalizedText.model_validate({"text": " hello "}, context={"locale": "en"})
```

Policy:

```text
Use context for parse policy, locale, normalization profile.
Do not use context for DB writes, network calls, or authorization decisions.
```

---

### Error-code-stable API mapping

```python
from pydantic_core import PydanticCustomError

def valid_plan(v: str) -> str:
    allowed = {"free", "pro", "enterprise"}
    if v not in allowed:
        raise PydanticCustomError(
            "invalid_plan",
            "invalid plan: {plan}",
            {"plan": v, "allowed": sorted(allowed)},
        )
    return v

Plan = Annotated[str, AfterValidator(valid_plan)]
```

Policy:

```text
Use PydanticCustomError when API layer needs stable error code/ctx.
Use ValueError for local human-readable checks.
Avoid AssertionError for production-critical validation.
```

---

## 7.21 Anti-patterns

### Validator doing IO

```python
@field_validator("user_id")
@classmethod
def user_exists(cls, v: str) -> str:
    requests.get(f"https://users/{v}")  # bad
    return v
```

Better:

```python
class Command(BaseModel):
    user_id: str

def handle(cmd: Command, repo: UserRepo):
    repo.require_user(cmd.user_id)
```

---

### Plain validator accidentally bypassing type safety

```python
class Model(BaseModel):
    n: Annotated[int, PlainValidator(lambda v: v)]

assert Model(n="bad").n == "bad"
```

Better:

```python
class Model(BaseModel):
    n: Annotated[int, AfterValidator(validate_n)]
```

---

### TypeError as intentional validation

```python
@field_validator("x")
@classmethod
def validate_x(cls, v: int) -> int:
    raise TypeError("bad")
```

Better:

```python
raise ValueError("bad")
```

TypeError is no longer converted to `ValidationError` in Pydantic v2 validators. ([Pydantic Docs][S07-4])

---

### Accessing later fields in `info.data`

```python
class Bad(BaseModel):
    password_repeat: str
    password: str

    @field_validator("password_repeat")
    @classmethod
    def match(cls, v: str, info: ValidationInfo) -> str:
        return info.data["password"]  # password not validated yet
```

Better:

```python
class Good(BaseModel):
    password: str
    password_repeat: str
```

or use `@model_validator(mode="after")`.

---

### Wrap validator for simple check

```python
WrapValidator(lambda v, handler: handler(v))
```

Better:

```python
AfterValidator(simple_check)
```

---

## 7.22 Test matrix

### After validator

```python
def test_after_validator():
    class M(BaseModel):
        n: Annotated[int, AfterValidator(lambda v: v if v % 2 == 0 else (_ for _ in ()).throw(ValueError("odd")))]

    assert M(n=2).n == 2

    with pytest.raises(ValidationError) as exc:
        M(n=3)

    assert exc.value.errors()[0]["type"] == "value_error"
```

### Before validator

```python
def test_before_validator_single_to_list():
    class M(BaseModel):
        xs: Annotated[list[int], BeforeValidator(lambda v: v if isinstance(v, list) else [v])]

    assert M(xs=1).xs == [1]
```

### Plain validator bypass

```python
def test_plain_validator_bypasses_type():
    class M(BaseModel):
        n: Annotated[int, PlainValidator(lambda v: v)]

    assert M(n="bad").n == "bad"
```

### Wrap validator retry

```python
def test_wrap_validator_retry():
    def truncate(v, handler):
        try:
            return handler(v)
        except ValidationError as err:
            if err.errors()[0]["type"] == "string_too_long":
                return handler(v[:5])
            raise

    class M(BaseModel):
        s: Annotated[str, Field(max_length=5), WrapValidator(truncate)]

    assert M(s="abcdef").s == "abcde"
```

### Model after validator

```python
def test_model_after_validator():
    class M(BaseModel):
        a: int
        b: int

        @model_validator(mode="after")
        def ordered(self):
            if self.a > self.b:
                raise ValueError("a must be <= b")
            return self

    assert M(a=1, b=2)

    with pytest.raises(ValidationError):
        M(a=3, b=2)
```

### Context

```python
def test_context():
    class M(BaseModel):
        text: str

        @field_validator("text")
        @classmethod
        def strip_if_configured(cls, v, info: ValidationInfo):
            return v.strip() if isinstance(info.context, dict) and info.context.get("strip") else v

    assert M.model_validate({"text": " x "}, context={"strip": True}).text == "x"
```

### TypeError propagation

```python
def test_type_error_not_validation_error():
    class M(BaseModel):
        x: int

        @field_validator("x")
        @classmethod
        def bad(cls, v: int) -> int:
            return str.lower(v)

    with pytest.raises(TypeError):
        M(x=1)
```

---

## 7.23 Deployment advisory matrix

```text
Need                                      Recommended validator mechanism
-------------------------------------------------------------------------------------
simple numeric/string bound                Field(...)
reusable local semantic scalar             Annotated[T, AfterValidator(...)]
raw shape migration                         BeforeValidator / model_validator(before)
cross-field invariant                       model_validator(after)
multi-field same normalization              @field_validator("a", "b", mode="before")
all fields same normalization               @field_validator("*", mode="before")
fallback/retry on validation error          WrapValidator / field_validator(mode="wrap")
custom machine-readable error code          PydanticCustomError
runtime validation policy parameter         ValidationInfo.context
field metadata/config access                ValidationInfo.field_name / info.config
performance-sensitive hot path              Field constraints / after validators
complete replacement of validation           PlainValidator; rare, heavily tested
```

---

## 7.24 Agent checklist

```text
[ ] Use Field constraints before validators.
[ ] Use AfterValidator for typed semantic checks.
[ ] Use BeforeValidator only for raw input normalization.
[ ] Use PlainValidator only when replacing Pydantic validation intentionally.
[ ] Use WrapValidator only when handler control is required.
[ ] Prefer Annotated aliases for reusable validators.
[ ] Prefer decorator validators for multi-field/model-local logic.
[ ] Always return the validated value or model instance.
[ ] Type before/plain/wrap inputs as Any.
[ ] Do not mutate raw inputs before raising.
[ ] Do not perform DB/network/authorization side effects in validators.
[ ] Use model_validator(after) for cross-field checks unless field order is intentional.
[ ] Use info.data only for earlier fields.
[ ] Pass runtime policy through context, not globals.
[ ] Use ValueError or PydanticCustomError for intentional validation failures.
[ ] Do not raise TypeError for validation failures.
[ ] Enable validate_default=True when validators must apply to defaults.
[ ] Avoid wrap validators in hot paths.
```

---

## 7.25 Value case

```text
Validator value =
  local semantic constraints beyond Field(...)
  reusable domain aliases
  raw-input migration
  typed post-parse invariants
  cross-field model integrity
  runtime policy injection through context
  stable custom error codes
  controlled fallback/retry behavior
```

Operationally: use validators as **pure, deterministic schema-level extension points**; keep parsing/shape migration in `before`, typed checks in `after`, rare pipeline interception in `wrap`, and business truth, authorization, IO, persistence, and workflow orchestration outside Pydantic.

[S07-1]: https://docs.pydantic.dev/latest/concepts/validators/ "Validators | Pydantic Docs"
[S07-2]: https://docs.pydantic.dev/latest/api/functional_validators/ "Functional Validators | Pydantic Docs"
[S07-3]: https://docs.pydantic.dev/latest/concepts/performance/ "Performance | Pydantic Docs"
[S07-4]: https://docs.pydantic.dev/latest/migration/ "Migration Guide | Pydantic Docs"


# Pydantic Advanced — 8) Serialization and dumping — Pydantic v2 deep dive

Style target: advanced, sectioned, agent-oriented technical reference. 

Pydantic uses **serialize** and **dump** almost interchangeably: both mean converting structured values such as models, dataclasses, and adapted types into less structured Python objects or JSON-encoded output. Pydantic supports two serialization modes: **Python mode**, which may preserve non-JSON-native Python objects, and **JSON mode**, which converts values into JSON-compatible forms. ([Pydantic Docs][S08-1])

---

## 8.0 Serialization mental model

```text
validated object
  → serializer schema
  → Python mode dump
      model_dump()
      TypeAdapter.dump_python()
  → JSON-compatible Python mode
      model_dump(mode="json")
      TypeAdapter.dump_python(mode="json")
  → JSON bytes/string
      model_dump_json()       # str
      TypeAdapter.dump_json() # bytes
```

Core invariant:

```text
validation controls accepted input
serialization controls emitted output
```

Do not assume validation type equals serialized type:

```python
from datetime import datetime
from decimal import Decimal
from pydantic import BaseModel

class M(BaseModel):
    ts: datetime
    amount: Decimal

m = M(ts="2032-06-01T12:13:14", amount="2.10")

m.model_dump()
# {'ts': datetime(...), 'amount': Decimal('2.10')}

m.model_dump(mode="json")
# {'ts': '2032-06-01T12:13:14', 'amount': '2.10'}

m.model_dump_json()
# '{"ts":"2032-06-01T12:13:14","amount":"2.10"}'
```

Pydantic’s standard-library type docs state that datetime/date/time/timedelta values remain Python objects in Python mode and become strings in JSON mode; `Decimal` remains `Decimal` in Python mode and serializes as a string in JSON mode unless overridden with a serializer. ([Pydantic Docs][S08-2])

---

## 8.1 Python-mode serialization: `model_dump()`

### Signature surface

```python
def model_dump(
    mode: Literal["json", "python"] | str = "python",
    include: IncEx | None = None,
    exclude: IncEx | None = None,
    context: Any | None = None,
    by_alias: bool | None = None,
    exclude_unset: bool = False,
    exclude_defaults: bool = False,
    exclude_none: bool = False,
    exclude_computed_fields: bool = False,
    round_trip: bool = False,
    warnings: bool | Literal["none", "warn", "error"] = True,
    fallback: Callable[[Any], Any] | None = None,
    serialize_as_any: bool = False,
    polymorphic_serialization: bool | None = None,
) -> dict[str, Any]: ...
```

`model_dump()` generates a dictionary representation of a model. In `mode="python"`, output may contain non-JSON-serializable Python objects; in `mode="json"`, output contains JSON-serializable types. It also accepts include/exclude filters, serializer context, alias behavior, unset/default/none exclusion, round-trip behavior, serialization error handling, fallback serialization, duck-typing serialization, and polymorphic serialization. ([Pydantic Docs][S08-3])

### Basic Python mode

```python
from datetime import datetime
from pydantic import BaseModel, Field

class Bar(BaseModel):
    values: tuple[int, ...]

class Foo(BaseModel):
    created_at: datetime
    bar: Bar
    name: str = Field(serialization_alias="displayName")

m = Foo(created_at="2032-06-01T12:13:14", bar={"values": (1, 2)}, name="demo")

assert m.model_dump() == {
    "created_at": datetime(2032, 6, 1, 12, 13, 14),
    "bar": {"values": (1, 2)},
    "name": "demo",
}

assert m.model_dump(by_alias=True)["displayName"] == "demo"
```

Python mode recursively converts models and model-like values to dictionaries, except that Python-native non-JSON values such as tuples may remain as Python-native values. ([Pydantic Docs][S08-1])

---

## 8.2 JSON-compatible Python mode: `model_dump(mode="json")`

```python
payload = m.model_dump(mode="json", by_alias=True)
```

Use when a framework wants a Python `dict` but all values must be JSON-compatible:

```text
FastAPI response body pre-shaping
message publishing before json.dumps
OpenAPI/example generation
cache payload dicts
structured logs that accept dicts
```

Example:

```python
assert m.model_dump(mode="json") == {
    "created_at": "2032-06-01T12:13:14",
    "bar": {"values": [1, 2]},
    "name": "demo",
}
```

Pydantic’s serialization docs show that `model_dump(mode="json")` converts values such as tuples into JSON-compatible lists, while default Python mode may keep tuples. ([Pydantic Docs][S08-1])

---

## 8.3 JSON-mode serialization: `model_dump_json()`

```python
json_text = m.model_dump_json()
pretty = m.model_dump_json(indent=2)
ascii_only = m.model_dump_json(ensure_ascii=True)
```

`model_dump_json()` serializes a model directly to a JSON-encoded string, converting supported Python values such as datetimes, UUIDs, sets, and other non-stdlib-JSON values into JSON-compatible representations; unsupported values that cannot be serialized raise `PydanticSerializationError`. ([Pydantic Docs][S08-1])

Deployment rule:

```text
Need Python dict:                 model_dump()
Need JSON-compatible dict:        model_dump(mode="json")
Need JSON string from BaseModel:  model_dump_json()
Need JSON bytes from TypeAdapter: TypeAdapter.dump_json()
```

Test rule:

```python
import json

assert json.loads(model.model_dump_json()) == expected
```

Do not snapshot compact JSON whitespace unless whitespace is part of the contract.

---

## 8.4 `TypeAdapter.dump_python()` and `TypeAdapter.dump_json()`

### `dump_python`

```python
from pydantic import TypeAdapter

adapter = TypeAdapter(list[tuple[int, int]])
value = [(1, 2), (3, 4)]

adapter.dump_python(value)
# [(1, 2), (3, 4)]

adapter.dump_python(value, mode="json")
# [[1, 2], [3, 4]]
```

`TypeAdapter.dump_python()` dumps an instance of the adapted type to a Python object and supports the same major serialization controls as `model_dump()`: mode, include/exclude, aliases, unset/default/none exclusion, computed-field exclusion, round-trip, warnings, fallback, `serialize_as_any`, polymorphic serialization, and context. ([Pydantic Docs][S08-4])

### `dump_json`

```python
payload_bytes = adapter.dump_json(value)
assert isinstance(payload_bytes, bytes)
```

`TypeAdapter.dump_json()` serializes an instance of the adapted type to JSON and returns **bytes**, not `str`; it accepts indentation, `ensure_ascii`, include/exclude controls, alias behavior, exclusion flags, round-trip, warnings, fallback, duck-typing/polymorphic serialization, and context. ([Pydantic Docs][S08-4])

Agent rule:

```text
Use BaseModel serialization when value is a model instance.
Use TypeAdapter serialization when top-level type is:
  list[T]
  dict[K, V]
  tuple[...]
  union
  TypedDict
  dataclass
  primitive scalar
  Annotated constrained alias
```

---

## 8.5 Serialization controls

### Include / exclude

```python
model.model_dump(include={"id", "email"})
model.model_dump(exclude={"password_hash", "internal_notes"})
```

Nested include/exclude:

```python
model.model_dump(
    exclude={
        "user": {"password", "token"},
        "events": {"__all__": {"debug"}},
        "hobbies": {-1: {"info"}},
    }
)
```

`include` and `exclude` support sets and nested dictionaries. They can target nested models, dictionary/sequence members, negative indexes, and all sequence members via the special key `"__all__"`. Using `False` as an include/exclude inverse marker is not supported. ([Pydantic Docs][S08-1])

### Field-level exclusion

```python
from pydantic import BaseModel, Field

class Transaction(BaseModel):
    id: int
    private_id: int = Field(exclude=True)
    value: int = Field(ge=0, exclude_if=lambda v: v == 0)

Transaction(id=1, private_id=2, value=0).model_dump()
# {'id': 1}
```

Field-level `exclude=True` and `exclude_if=...` are configured through `Field(...)`; field-level exclusion takes priority over `include` passed to serialization methods. ([Pydantic Docs][S08-1])

---

## 8.6 Value-based exclusion flags

```python
model.model_dump(exclude_none=True)
model.model_dump(exclude_defaults=True)
model.model_dump(exclude_unset=True)
```

Semantics:

```text
exclude_none:
  omit fields whose value is None

exclude_defaults:
  omit fields whose value == default

exclude_unset:
  omit fields not explicitly set during model construction/validation
```

`exclude_unset` relies on `model_fields_set`, which tracks explicitly provided fields; mutating a field after creation removes it from the unset set, so it can appear in later `exclude_unset=True` dumps. ([Pydantic Docs][S08-1])

PATCH pattern:

```python
class PatchUser(BaseModel):
    email: str | None = None
    age: int | None = None

patch = PatchUser.model_validate({"email": None})

assert patch.model_fields_set == {"email"}
assert patch.model_dump(exclude_unset=True) == {"email": None}
assert patch.model_dump(exclude_none=True) == {}
```

Agent rule:

```text
PATCH / partial update:
  exclude_unset=True

Public response hiding nulls:
  exclude_none=True

Compact config output:
  exclude_defaults=True

Do not substitute exclude_none for exclude_unset.
Explicit null and omitted are different states.
```

---

## 8.7 `by_alias`

```python
from pydantic import BaseModel, Field

class User(BaseModel):
    user_id: int = Field(serialization_alias="userId")

u = User(user_id=1)

assert u.model_dump() == {"user_id": 1}
assert u.model_dump(by_alias=True) == {"userId": 1}
```

`by_alias=True` uses the field’s serialization alias as the output key when defined. ([Pydantic Docs][S08-3])

Deployment rule:

```text
Internal Python code:
  by_alias=False or default

External JSON API:
  by_alias=True if wire schema uses aliases

SDK public model dump:
  document alias policy
```

---

## 8.8 `round_trip=True`

```python
payload = model.model_dump(round_trip=True)
```

`round_trip=True` asks Pydantic to dump values so that non-idempotent types such as `Json[T]` remain valid as input for a subsequent validation pass; `exclude_computed_fields=True` can help round-trip scenarios, but Pydantic recommends `round_trip` for that purpose. ([Pydantic Docs][S08-3])

Agent rule:

```text
Use round_trip=True for:
  cache snapshots intended for revalidation
  model persistence formats
  Json[T] / non-idempotent type payloads

Avoid round_trip=True for:
  normal API responses
  human-facing output
  OpenAPI examples unless needed
```

---

## 8.9 Serialization warnings, fallback, and arbitrary types

```python
model.model_dump(
    mode="json",
    warnings="error",
    fallback=lambda value: repr(value),
)
```

`warnings` controls serialization-error handling: `False`/`"none"` ignores, `True`/`"warn"` logs, and `"error"` raises `PydanticSerializationError`; `fallback` is called when an unknown value is encountered, otherwise an unknown value raises a serialization error. ([Pydantic Docs][S08-3])

Custom arbitrary-type pattern:

```python
from typing import Annotated
from pydantic import BaseModel, ConfigDict, PlainSerializer

class Client:
    def __init__(self, name: str) -> None:
        self.name = name

SerializableClient = Annotated[
    Client,
    PlainSerializer(lambda c: {"name": c.name}, return_type=dict, when_used="json"),
]

class M(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)
    client: SerializableClient
```

Agent rule:

```text
For arbitrary types:
  validate deliberately
  serialize deliberately
  provide return_type where possible
  prefer field/type serializers over global fallback
  use fallback only as last-resort defensive serialization
```

---

## 8.10 Field serializers

### Plain field serializer

```python
from typing import Annotated, Any
from pydantic import BaseModel, PlainSerializer

def double_number(value: Any) -> Any:
    return value * 2 if isinstance(value, int) else value

DoubleInt = Annotated[int, PlainSerializer(double_number)]

class Model(BaseModel):
    number: DoubleInt

assert Model(number=4).model_dump() == {"number": 8}
```

Plain serializers are called unconditionally for the field; Pydantic’s normal serialization logic for the annotated type is not called. If `return_type` is supplied or inferred from the function annotation, Pydantic builds an extra serializer to ensure the serialized value complies with that return type. ([Pydantic Docs][S08-1])

### Decorator plain serializer

```python
from pydantic import BaseModel, field_serializer

class Student(BaseModel):
    courses: set[str]

    @field_serializer("courses", when_used="json")
    def serialize_courses(self, courses: set[str]) -> list[str]:
        return sorted(courses)
```

The `@field_serializer` decorator supports plain and wrap modes; the `when_used` setting can restrict serializers to `always`, `unless-none`, `json`, or `json-unless-none`. ([Pydantic Docs][S08-5])

### Wrap field serializer

```python
from typing import Any
from pydantic import BaseModel, SerializerFunctionWrapHandler, field_serializer

class Model(BaseModel):
    number: int

    @field_serializer("number", mode="wrap")
    def add_one(
        self,
        value: Any,
        handler: SerializerFunctionWrapHandler,
    ) -> int:
        return handler(value) + 1
```

Wrap serializers receive a mandatory handler that delegates to Pydantic’s serialization logic; the serializer can run code before/after the handler or skip the handler entirely. ([Pydantic Docs][S08-1])

### Serializer pattern choice

```text
Annotated serializer:
  reusable
  visible in field type
  applies to container items if placed inside container

Decorator serializer:
  model-local
  can target multiple fields
  supports "*" for all fields
  can disable field-existence checks for base classes
```

Pydantic’s serialization docs explicitly present the annotated pattern as reusable and useful for item-level serializers, while decorator serializers are useful for applying one function to multiple fields or all fields. ([Pydantic Docs][S08-1])

---

## 8.11 Model serializers

### Plain model serializer

```python
from pydantic import BaseModel, model_serializer

class UserModel(BaseModel):
    username: str
    password: str

    @model_serializer(mode="plain")
    def serialize_model(self) -> str:
        return f"{self.username}"
```

Plain model serializers are called unconditionally for the whole model, and may return a non-dictionary value. ([Pydantic Docs][S08-1])

### Wrap model serializer

```python
from pydantic import BaseModel, SerializerFunctionWrapHandler, model_serializer

class UserModel(BaseModel):
    username: str
    password: str

    @model_serializer(mode="wrap")
    def serialize_model(self, handler: SerializerFunctionWrapHandler) -> dict[str, object]:
        data = handler(self)
        data.pop("password", None)
        data["fields"] = list(data)
        return data
```

Wrap model serializers receive a handler that delegates to Pydantic’s model serialization; they can mutate the serialized dictionary, add/remove fields, or return another value. ([Pydantic Docs][S08-1])

Critical constraint:

```text
Only one serializer can be defined per field/model.
You cannot stack multiple plain/wrap serializers for the same field/model.
```

Pydantic’s docs state that only one serializer can be defined per field or model; plain and wrap serializers cannot be combined for the same target. ([Pydantic Docs][S08-1])

---

## 8.12 Serialization context

```python
from pydantic import BaseModel, FieldSerializationInfo, field_serializer

class Text(BaseModel):
    text: str

    @field_serializer("text", mode="plain")
    @classmethod
    def remove_stopwords(cls, v: str, info: FieldSerializationInfo) -> str:
        if isinstance(info.context, dict):
            stopwords = set(info.context.get("stopwords", ()))
            return " ".join(w for w in v.split() if w.lower() not in stopwords)
        return v

doc = Text(text="This is an example document")

doc.model_dump(context={"stopwords": ["this", "is", "an"]})
# {'text': 'example document'}
```

Field and model serializer callables can receive an `info` argument exposing context, serialization mode, serialization parameters such as `exclude_unset` and `serialize_as_any`, and the current field name for field serializers. Serialization methods accept a `context` object that serializers can read through `info.context`. ([Pydantic Docs][S08-1])

Agent rule:

```text
Good serialization context:
  locale
  redaction profile
  timezone display policy
  stopword set
  public/internal audience flag

Bad serialization context:
  DB writes
  network calls
  authorization decisions
  long-running IO
```

---

## 8.13 Subclass serialization pitfall

```python
from pydantic import BaseModel

class User(BaseModel):
    name: str

class UserLogin(User):
    password: str

class Outer(BaseModel):
    user: User

outer = Outer(user=UserLogin(name="pydantic", password="hunter2"))

assert outer.model_dump() == {"user": {"name": "pydantic"}}
```

Default v2 behavior: when a field is annotated as a model-like base class, serialization uses the schema of the annotated type, not the runtime subclass, so subclass-only fields are omitted. This differs from v1 and is intended to prevent accidental leakage of sensitive subclass fields such as passwords. ([Pydantic Docs][S08-1])

Agent rule:

```text
Default subclass omission is a security feature.
Do not disable it globally unless you own every subclass field.
```

---

## 8.14 Polymorphic serialization

```python
payload = outer.model_dump(polymorphic_serialization=True)
```

Polymorphic serialization was added in **v2.13** for Pydantic models and Pydantic dataclasses. It serializes a model/dataclass instance according to the serialization schema of the runtime instance rather than the class used in the field annotation; it can be configured at model/dataclass config level or per serialization call through `polymorphic_serialization`. ([Pydantic Docs][S08-1])

Use cases:

```text
public polymorphic response DTOs
plugin/subclass trees where subclass fields are intended output
domain hierarchies with explicitly versioned output contracts
```

Avoid:

```text
unknown subclass graphs
sensitive subclasses
security boundary output without explicit tests
```

---

## 8.15 Duck-typing / `SerializeAsAny`

### Field-level duck-typed serialization

```python
from pydantic import BaseModel, SerializeAsAny

class Outer(BaseModel):
    as_any: SerializeAsAny[User]
    as_user: User

user = UserLogin(name="pydantic", password="password")
Outer(as_any=user, as_user=user).model_dump()
# {'as_any': {'name': 'pydantic', 'password': 'password'}, 'as_user': {'name': 'pydantic'}}
```

`SerializeAsAny[T]` keeps validation and static type-checking behavior as if the field were `T`, but serializes the value as though the field were annotated as `Any`, using the runtime value’s type. ([Pydantic Docs][S08-1])

### Runtime duck-typed serialization

```python
outer.model_dump(serialize_as_any=True)
outer.model_dump_json(serialize_as_any=True)
```

`serialize_as_any=True` applies duck-typed serialization behavior to the entire serialization call; v2 default is `False`, so subclass fields are omitted unless polymorphic or duck-typed behavior is requested. Pydantic recommends preferring `SerializeAsAny` when only specific fields need this behavior. ([Pydantic Docs][S08-1])

Agent rule:

```text
Prefer:
  SerializeAsAny[field] for explicit, narrow behavior.

Avoid:
  serialize_as_any=True across whole API responses unless fully audited.
```

---

## 8.16 Secrets

```python
from pydantic import BaseModel, SecretBytes, SecretStr, field_serializer

class Credentials(BaseModel):
    password: SecretStr
    token: SecretBytes

creds = Credentials(password="secret", token=b"token")
```

`SecretStr` and `SecretBytes` are designed for sensitive values that should not appear in logging or tracebacks; nonempty secrets display as `**********` in `repr()`/`str()`, and default JSON serialization also masks them. The underlying value is available through `.get_secret_value()`. ([Pydantic][S08-6])

Plaintext secret output, only when explicitly required:

```python
class Credentials(BaseModel):
    password: SecretStr
    token: SecretBytes

    @field_serializer("password", "token", when_used="json")
    def dump_secret(self, v):
        return v.get_secret_value()
```

Pydantic’s docs show a field serializer can deliberately emit plaintext secret values for JSON serialization, which is a high-risk pattern and should be limited to trusted sinks. ([Pydantic][S08-6])

Agent rule:

```text
Default:
  keep secrets masked

Only unmask:
  explicit trusted sink
  dedicated serializer
  test coverage proving no public path uses it
```

---

## 8.17 Datetimes and timezones

```python
from datetime import datetime
from pydantic import AwareDatetime, BaseModel

class Event(BaseModel):
    dt: AwareDatetime

event = Event(dt="2032-04-23T10:20:30.400+02:30")

event.model_dump()
# {'dt': datetime.datetime(..., tzinfo=...)}

event.model_dump_json()
# {"dt":"2032-04-23T10:20:30.400000+02:30"}
```

Datetime values serialize as native `datetime` objects in Python mode and RFC 3339-like strings in JSON mode. Time values serialize as native `time` objects in Python mode and strings in JSON mode; named IANA timezones on `time` objects are not serialized, consistent with `time.isoformat()`. ([Pydantic Docs][S08-2])

UTC-normalizing serializer pattern:

```python
from datetime import timezone
from pydantic import field_serializer

class Event(BaseModel):
    dt: AwareDatetime

    @field_serializer("dt", when_used="json")
    def serialize_utc(self, dt: datetime) -> str:
        return dt.astimezone(timezone.utc).isoformat().replace("+00:00", "Z")
```

Agent rule:

```text
Public timestamps:
  use AwareDatetime
  define timezone normalization policy
  test output strings

Avoid:
  naive datetime output in public APIs unless explicitly documented
```

---

## 8.18 Decimals

```python
from decimal import Decimal
from pydantic import BaseModel

class Price(BaseModel):
    amount: Decimal

p = Price(amount="2.10")

assert p.model_dump() == {"amount": Decimal("2.10")}
assert p.model_dump_json() == '{"amount":"2.10"}'
```

`Decimal` is preserved as `Decimal` in Python mode and serialized as a string in JSON mode by default. Pydantic docs show overriding JSON Decimal serialization with `PlainSerializer(float, when_used="json")`, but that can introduce precision loss. ([Pydantic Docs][S08-2])

Safer money policy:

```python
class Price(BaseModel):
    amount: Decimal

    @field_serializer("amount", when_used="json")
    def amount_as_string(self, v: Decimal) -> str:
        return format(v, "f")
```

Agent rule:

```text
Money:
  serialize Decimal as string or integer minor units

Avoid:
  float serialization for financial values unless precision loss is acceptable
```

---

## 8.19 Bytes

```python
from pydantic import BaseModel, ConfigDict

class Blob(BaseModel):
    model_config = ConfigDict(ser_json_bytes="base64", val_json_bytes="base64")
    data: bytes
```

Bytes are validated as bytes, while strings/bytearray are converted according to the `val_json_bytes` configuration value; strict Python mode requires bytes, and strict JSON mode has no effect for bytes. ([Pydantic Docs][S08-2])

Agent rule:

```text
Binary data in JSON:
  configure val_json_bytes / ser_json_bytes explicitly

Opaque binary payload:
  avoid embedding in JSON if large
  store externally and serialize URI/checksum metadata
```

---

## 8.20 Complete serialization policy example

```python
from __future__ import annotations

from datetime import datetime, timezone
from decimal import Decimal
from typing import Annotated
from uuid import UUID

from pydantic import (
    AwareDatetime,
    BaseModel,
    ConfigDict,
    Field,
    PlainSerializer,
    SecretStr,
    SerializeAsAny,
    field_serializer,
    model_serializer,
)


MoneyJson = Annotated[
    Decimal,
    PlainSerializer(lambda v: format(v, "f"), return_type=str, when_used="json"),
]


class User(BaseModel):
    id: UUID
    email: str


class UserLogin(User):
    password: SecretStr


class ApiModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        serialize_by_alias=True,
    )


class Invoice(ApiModel):
    id: UUID
    created_at: AwareDatetime
    user: User
    user_debug: SerializeAsAny[User] | None = Field(default=None, exclude=True)
    amount: MoneyJson
    internal_score: float = Field(default=0.0, exclude=True, repr=False)
    note: str | None = None

    @field_serializer("created_at", when_used="json")
    def created_at_utc(self, v: datetime) -> str:
        return v.astimezone(timezone.utc).isoformat().replace("+00:00", "Z")

    @model_serializer(mode="wrap")
    def serialize_invoice(self, handler):
        data = handler(self)
        data["kind"] = "invoice"
        return data


invoice = Invoice(
    id="01999b2c-8353-749b-8dac-859307fae22b",
    created_at="2032-04-23T10:20:30+02:00",
    user=UserLogin(
        id="125725f3-e1b4-44e3-90c3-1a20eab12da5",
        email="a@example.com",
        password="secret",
    ),
    amount="2.10",
)

public_dict = invoice.model_dump(mode="json", exclude_none=True)
public_json = invoice.model_dump_json(exclude_none=True)
```

Expected security behavior:

```text
user.password omitted:
  field annotated as User, runtime value UserLogin

internal_score omitted:
  Field(exclude=True)

amount JSON serialized as string:
  MoneyJson serializer

created_at normalized:
  field serializer

kind injected:
  wrap model serializer
```

---

## 8.21 Deployment decision matrix

```text
Need                                         API / pattern
--------------------------------------------------------------------------------
Python dict with native Python objects         model_dump()
JSON-compatible dict                           model_dump(mode="json")
JSON text from BaseModel                       model_dump_json()
JSON bytes from non-model type                 TypeAdapter(T).dump_json(value)
Top-level list/dict/union serialization        TypeAdapter(T).dump_python / dump_json
Public API aliases                             by_alias=True / serialize_by_alias config
PATCH/update payload                           exclude_unset=True
Suppress nulls in responses                    exclude_none=True
Suppress default values                        exclude_defaults=True
Revalidation-friendly dump                     round_trip=True
Hard field suppression                         Field(exclude=True)
Conditional field suppression                  Field(exclude_if=...)
Custom field output                            @field_serializer / PlainSerializer
Custom whole-model output                      @model_serializer
Runtime output policy                          context=...
Subclass fields intended output                polymorphic_serialization=True or SerializeAsAny
Sensitive values                               SecretStr / SecretBytes + no plaintext serializer
Unknown arbitrary type                         type/field serializer; fallback as last resort
```

---

## 8.22 Anti-patterns

### `dict(model)` instead of `model_dump()`

```python
dict(model)  # nested submodels remain model instances
```

Pydantic docs note that iterating over models or calling `dict(model)` yields raw field values, so submodels are not recursively converted to dictionaries. Use `model_dump()` for recursive dumping. ([Pydantic Docs][S08-1])

---

### Global `serialize_as_any=True` on sensitive model graphs

```python
response.model_dump(serialize_as_any=True)
```

Problem:

```text
runtime subclass fields may leak
password/API-key fields added in subclasses may serialize
behavior applies broadly, not only where duck typing is relevant
```

Prefer:

```python
class Response(BaseModel):
    safe_plugin_payload: SerializeAsAny[PluginBase]
```

Pydantic warns that `serialize_as_any=True` applies to all values, and recommends using the `SerializeAsAny` annotation when specific fields require that behavior. ([Pydantic Docs][S08-1])

---

### Decimal as float for money

```python
amount: Annotated[Decimal, PlainSerializer(float, when_used="json")]
```

Problem:

```text
possible precision loss
rounding drift
bad financial contract
```

Prefer string or integer minor units.

---

### Secret plaintext serializer in public model

```python
@field_serializer("password", when_used="json")
def dump_secret(self, v):
    return v.get_secret_value()
```

Problem:

```text
turns masked secret into public JSON value
easy accidental leak through normal model_dump_json()
```

Use only in a dedicated internal/export model.

---

### Serializer doing IO

```python
@field_serializer("user_id")
def serialize_user(self, v):
    return requests.get(f"https://users/{v}").json()
```

Problem:

```text
serialization becomes nondeterministic
unexpected latency
unexpected failures during logging/API response creation
hard-to-test side effects
```

Keep serializers pure and local.

---

## 8.23 Test matrix

### Python vs JSON mode

```python
from datetime import datetime
from pydantic import BaseModel

def test_python_vs_json_mode():
    class M(BaseModel):
        ts: datetime
        values: tuple[int, ...]

    m = M(ts="2032-06-01T12:13:14", values=(1, 2))

    assert isinstance(m.model_dump()["ts"], datetime)
    assert m.model_dump()["values"] == (1, 2)
    assert m.model_dump(mode="json")["values"] == [1, 2]
    assert '"2032-06-01T12:13:14"' in m.model_dump_json()
```

### `exclude_unset` vs `exclude_none`

```python
def test_exclude_unset_vs_none():
    class Patch(BaseModel):
        x: int | None = None

    omitted = Patch()
    explicit_null = Patch(x=None)

    assert omitted.model_dump(exclude_unset=True) == {}
    assert explicit_null.model_dump(exclude_unset=True) == {"x": None}
    assert explicit_null.model_dump(exclude_none=True) == {}
```

### Subclass omission

```python
def test_subclass_fields_omitted_by_default():
    class User(BaseModel):
        name: str

    class UserLogin(User):
        password: str

    class Outer(BaseModel):
        user: User

    out = Outer(user=UserLogin(name="u", password="p"))

    assert out.model_dump() == {"user": {"name": "u"}}
    assert out.model_dump(serialize_as_any=True) == {
        "user": {"name": "u", "password": "p"}
    }
```

### Field serializer

```python
def test_field_serializer_json_only():
    class Student(BaseModel):
        courses: set[str]

        @field_serializer("courses", when_used="json")
        def courses_sorted(self, v: set[str]) -> list[str]:
            return sorted(v)

    s = Student(courses={"Math", "Chemistry"})

    assert s.model_dump()["courses"] == {"Math", "Chemistry"}
    assert s.model_dump(mode="json")["courses"] == ["Chemistry", "Math"]
```

### Serialization context

```python
def test_serialization_context():
    class Text(BaseModel):
        text: str

        @field_serializer("text")
        def ser_text(self, v: str, info):
            stop = set((info.context or {}).get("stopwords", ()))
            return " ".join(w for w in v.split() if w.lower() not in stop)

    assert Text(text="this is fine").model_dump(
        context={"stopwords": ["this", "is"]}
    ) == {"text": "fine"}
```

### TypeAdapter JSON bytes

```python
def test_type_adapter_dump_json_returns_bytes():
    ta = TypeAdapter(list[int])
    assert ta.dump_json([1, 2, 3]) == b"[1,2,3]"
```

---

## 8.24 Agent checklist

```text
[ ] Use model_dump() for recursive Python dict output.
[ ] Use model_dump(mode="json") for JSON-compatible Python dict output.
[ ] Use model_dump_json() for JSON string output from BaseModel.
[ ] Use TypeAdapter.dump_json() for JSON bytes from arbitrary top-level types.
[ ] Use exclude_unset=True for PATCH payloads.
[ ] Use exclude_none=True for null-suppressed public responses.
[ ] Use exclude_defaults=True for compact default-suppressed output.
[ ] Use by_alias=True for external wire schemas with aliases.
[ ] Use Field(exclude=True) for fields that must never serialize.
[ ] Use Field(exclude_if=...) for conditional field suppression.
[ ] Use round_trip=True for revalidation/persistence dumps.
[ ] Keep field/model serializers pure and local.
[ ] Use PlainSerializer for total replacement of one field/type serialization.
[ ] Use WrapSerializer / mode="wrap" only when handler composition is needed.
[ ] Do not stack multiple serializers on one field/model.
[ ] Pass context for audience/locale/redaction policy, not IO.
[ ] Treat subclass omission as default security behavior.
[ ] Prefer polymorphic_serialization or SerializeAsAny only when intended and tested.
[ ] Use SecretStr/SecretBytes for sensitive values.
[ ] Never add plaintext secret serializer to public response models.
[ ] Test Python mode and JSON mode separately.
[ ] Test TypeAdapter.dump_json() as bytes, not str.
```

---

## 8.25 Value case

```text
Serialization value =
  one validation schema also emits controlled output
  Python-native dumps for internal workflows
  JSON-compatible dumps for APIs/logs/messages
  direct JSON encoding for wire payloads
  top-level arbitrary type dumping through TypeAdapter
  precise include/exclude controls
  alias-driven external contracts
  round-trip-safe persistence support
  field/type/model serializer extension points
  context-sensitive output policy
  secure default subclass serialization
  explicit duck-typing/polymorphic escape hatches
  secret masking support
```

Operationally: keep validation and serialization policies separate, dump with the mode matching the target sink, treat subclass/secret behavior as security-sensitive, and encode public-output behavior with explicit serializers, aliases, and exclusion rules.

[S08-1]: https://docs.pydantic.dev/latest/concepts/serialization/ "Serialization | Pydantic Docs"
[S08-2]: https://docs.pydantic.dev/latest/api/standard_library_types/ "Standard Library Types | Pydantic Docs"
[S08-3]: https://docs.pydantic.dev/latest/api/base_model/ "BaseModel | Pydantic Docs"
[S08-4]: https://docs.pydantic.dev/latest/api/type_adapter/ "TypeAdapter | Pydantic Docs"
[S08-5]: https://docs.pydantic.dev/latest/api/functional_serializers/ "Functional Serializers | Pydantic Docs"
[S08-6]: https://pydantic.dev/docs/validation/latest/api/pydantic/types/ "Pydantic Types | Pydantic Docs"


# Part II — External shape, configuration, schemas, and extensibility

# Pydantic Advanced — 9) Aliases and external data shape control — Pydantic v2 deep dive

Style target: advanced, sectioned, agent-oriented technical reference. 

Pydantic defines an **alias** as an alternative field name used during serialization and deserialization. Alias control exists at three layers: **field-level aliases**, **model-level alias generation**, and **runtime/config alias policy**. Field-level surfaces are `alias`, `validation_alias`, and `serialization_alias`; `validation_alias` can also use `AliasPath` or `AliasChoices`; model-level `alias_generator` can be a callable or an `AliasGenerator`. ([Pydantic Docs][S09-1])

---

## 9.0 Alias mental model

```text
canonical internal field name
  = Python attribute name
  = model code / domain code / type checker name

validation alias
  = accepted inbound key(s)
  = external JSON / legacy payload / nested path / env var name

serialization alias
  = emitted outbound key
  = external JSON / API response / generated dict/json contract

alias
  = shorthand for both validation and serialization alias
  = must be str
```

Alias design separates **internal naming** from **external wire shape**:

```python
from pydantic import BaseModel, Field

class User(BaseModel):
    user_id: int = Field(validation_alias="userId", serialization_alias="userId")
```

Agent invariant:

```text
Keep Python fields snake_case.
Map external names at the boundary.
Never rename Python attributes just to match JSON/env/API casing.
```

Pydantic’s fields docs state that `Field(alias=...)` applies to both validation and serialization, while `validation_alias` and `serialization_alias` apply only to their respective use cases. ([Pydantic Docs][S09-2])

---

## 9.1 Field-level aliases: `alias`

```python
from pydantic import BaseModel, Field

class User(BaseModel):
    name: str = Field(alias="username")

user = User(username="johndoe")

assert user.name == "johndoe"
assert user.model_dump() == {"name": "johndoe"}
assert user.model_dump(by_alias=True) == {"username": "johndoe"}
```

Semantics:

```text
Field(alias="username"):
  inbound validation key: "username"
  outbound serialization key when by_alias=True: "username"
  static type checkers use alias for __init__ synthesis
```

`alias` must be a string. Pydantic uses it for validation and serialization, but serialization aliases are only emitted when `by_alias=True` or model config enables alias serialization. ([Pydantic Docs][S09-1])

Use `alias` when:

```text
same external name for inbound and outbound
legacy Python constructor should accept alias
static type checker should see alias in generated __init__
single external spelling exists
```

Avoid `alias` when:

```text
inbound and outbound names differ
you need multiple inbound legacy names
you need nested inbound paths
you want type checkers to prefer Python field names
```

---

## 9.2 Field-level aliases: `validation_alias`

```python
from pydantic import BaseModel, Field

class User(BaseModel):
    name: str = Field(validation_alias="username")

user = User(username="johndoe")

assert user.name == "johndoe"
assert user.model_dump(by_alias=True) == {"name": "johndoe"}
```

Semantics:

```text
Field(validation_alias="username"):
  inbound validation key: "username"
  outbound serialization key: Python field name unless serialization_alias/alias configured
```

`validation_alias` can be a `str`, `AliasPath`, or `AliasChoices`; unlike `alias`, it does not define the output key. ([Pydantic Docs][S09-1])

Use `validation_alias` when:

```text
inbound wire key differs from internal field
outbound key should remain internal field name
legacy compatibility only needed at input
settings/env var name differs from Python field
nested inbound payload should be flattened into a model field
```

---

## 9.3 Field-level aliases: `serialization_alias`

```python
from pydantic import BaseModel, Field

class User(BaseModel):
    name: str = Field(serialization_alias="username")

user = User(name="johndoe")

assert user.model_dump() == {"name": "johndoe"}
assert user.model_dump(by_alias=True) == {"username": "johndoe"}
```

Semantics:

```text
Field(serialization_alias="username"):
  inbound validation key: Python field name unless alias/validation_alias configured
  outbound serialization key when by_alias=True: "username"
```

`serialization_alias` must be a string and only affects serialization. ([Pydantic Docs][S09-1])

Use `serialization_alias` when:

```text
outbound API key differs from internal field
inbound payload should use Python field name
response schema has different naming than request schema
compatibility adapter emits legacy names
```

---

## 9.4 Field alias precedence

```python
class M(BaseModel):
    x: int = Field(
        alias="legacyX",
        validation_alias="inputX",
        serialization_alias="outputX",
    )

m = M.model_validate({"inputX": 1})

assert m.model_dump(by_alias=True) == {"outputX": 1}
```

Precedence:

```text
validation:
  validation_alias overrides alias

serialization:
  serialization_alias overrides alias
```

Pydantic documents that when `alias` is combined with `validation_alias` or `serialization_alias`, `validation_alias` wins for validation and `serialization_alias` wins for serialization. ([Pydantic Docs][S09-2])

---

## 9.5 Static type checker / IDE implications

### Type checkers understand `alias`

```python
class User(BaseModel):
    name: str = Field(alias="username")

User(username="johndoe")  # accepted by type checkers
```

### Type checkers do not understand `validation_alias`

```python
class User(BaseModel):
    name: str = Field(validation_alias="username")

User(username="johndoe")  # runtime ok, type checker may not recognize
```

Pydantic’s docs state that static type checkers use `alias` to synthesize the model `__init__` signature, but type checkers do not understand `validation_alias` the same way. They also describe an `Annotated[...]` workaround when the model should accept aliases at runtime while type checkers keep the Python field name. ([Pydantic Docs][S09-2])

### Type-checker-friendly runtime alias acceptance

```python
from typing import Annotated
from pydantic import BaseModel, ConfigDict, Field

class User(BaseModel):
    model_config = ConfigDict(validate_by_name=True, validate_by_alias=True)

    name: Annotated[str, Field(alias="username")]

User(name="johndoe")      # type-checker friendly
User(username="johndoe")  # runtime accepted, type checker may not accept
```

Agent rule:

```text
If generated code prioritizes static type checker constructor ergonomics:
  prefer Python field names in constructors
  use Annotated alias metadata
  validate through model_validate(...) at external boundaries

If generated code prioritizes external constructor alias ergonomics:
  use assignment-form Field(alias=...)
```

---

## 9.6 `AliasPath`: flatten nested input into fields

```python
from pydantic import BaseModel, Field, AliasPath

class User(BaseModel):
    first_name: str = Field(validation_alias=AliasPath("names", 0))
    last_name: str = Field(validation_alias=AliasPath("names", 1))
    address: str = Field(validation_alias=AliasPath("contact", "address"))

user = User.model_validate({
    "names": ["John", "Doe"],
    "contact": {"address": "221B Baker Street"},
})

assert user.first_name == "John"
assert user.last_name == "Doe"
assert user.address == "221B Baker Street"
```

Semantics:

```text
AliasPath("names", 0):
  lookup input["names"][0]

AliasPath("contact", "address"):
  lookup input["contact"]["address"]
```

`AliasPath` is a helper used by `validation_alias`; its path is a list of string or integer aliases, and it can search dictionaries for that path, returning undefined when the path is missing. ([Pydantic Docs][S09-1])

Use cases:

```text
legacy nested payload flattening
vendor payloads with arrays
extracting values from nested API envelopes
normalizing inconsistent inbound JSON without changing internal model shape
```

Avoid:

```text
deep business transformation
complex conditional extraction
multi-source aggregation
IO-backed lookup
```

AliasPath is for structural key/index extraction, not a replacement for preprocessing pipelines.

---

## 9.7 `AliasChoices`: accept multiple inbound names

```python
from pydantic import BaseModel, Field, AliasChoices

class User(BaseModel):
    first_name: str = Field(validation_alias=AliasChoices("first_name", "fname"))
    last_name: str = Field(validation_alias=AliasChoices("last_name", "lname"))

assert User.model_validate({"fname": "John", "lname": "Doe"}).first_name == "John"
assert User.model_validate({"first_name": "John", "lname": "Doe"}).first_name == "John"
```

Priority:

```text
AliasChoices("first_name", "fname"):
  first_name has higher priority than fname
  if both appear, first_name wins
```

Pydantic documents that `AliasChoices` specifies a list of aliases for validation and that earlier choices have higher priority. ([Pydantic Docs][S09-1])

Use cases:

```text
legacy payload compatibility
renamed API fields
third-party vendor variants
settings env var migration
graceful deprecation window
```

---

## 9.8 Combining `AliasChoices` and `AliasPath`

```python
from pydantic import BaseModel, Field, AliasChoices, AliasPath

class User(BaseModel):
    first_name: str = Field(
        validation_alias=AliasChoices("first_name", AliasPath("names", 0))
    )
    last_name: str = Field(
        validation_alias=AliasChoices("last_name", AliasPath("names", 1))
    )

assert User.model_validate({"first_name": "John", "last_name": "Doe"}).first_name == "John"
assert User.model_validate({"names": ["John", "Doe"]}).last_name == "Doe"
```

Value case:

```text
single internal model
multiple historical inbound shapes
no separate migration DTO required
clear priority ordering
```

Pydantic’s alias docs show `AliasChoices` can contain both strings and `AliasPath` instances. ([Pydantic Docs][S09-1])

---

## 9.9 Model-level alias generation: callable `alias_generator`

```python
from pydantic import BaseModel, ConfigDict

def to_upper(field_name: str) -> str:
    return field_name.upper()

class Tree(BaseModel):
    model_config = ConfigDict(alias_generator=to_upper)

    age: int
    height: float
    kind: str

tree = Tree.model_validate({"AGE": 12, "HEIGHT": 1.2, "KIND": "oak"})
assert tree.model_dump(by_alias=True) == {"AGE": 12, "HEIGHT": 1.2, "KIND": "oak"}
```

Semantics:

```text
alias_generator callable:
  input: Python field name
  output: alias string
  applies to all model fields unless overridden by field alias/priority
```

Pydantic supports `ConfigDict(alias_generator=...)`; built-in alias generators include `to_pascal`, `to_camel`, and `to_snake`. ([Pydantic Docs][S09-1])

---

## 9.10 Built-in alias generators

```python
from pydantic import BaseModel, ConfigDict
from pydantic.alias_generators import to_camel, to_pascal, to_snake

class ApiModel(BaseModel):
    model_config = ConfigDict(alias_generator=to_camel)

class User(ApiModel):
    user_id: int
    full_name: str
```

Expected alias forms:

```text
user_id   → userId
full_name → fullName
```

Pydantic’s alias docs list `to_pascal`, `to_camel`, and `to_snake` as built-in generators. ([Pydantic Docs][S09-1])

Agent rule:

```text
Use built-in generators for common naming conventions.
Use custom generators only when the convention is project-specific and tested.
```

---

## 9.11 Model-level alias generation: `AliasGenerator`

```python
from pydantic import AliasGenerator, BaseModel, ConfigDict

class Tree(BaseModel):
    model_config = ConfigDict(
        alias_generator=AliasGenerator(
            validation_alias=lambda field_name: field_name.upper(),
            serialization_alias=lambda field_name: field_name.title(),
        )
    )

    age: int
    height: float
    kind: str

tree = Tree.model_validate({"AGE": 12, "HEIGHT": 1.2, "KIND": "oak"})
assert tree.model_dump(by_alias=True) == {"Age": 12, "Height": 1.2, "Kind": "oak"}
```

Use cases:

```text
inbound and outbound conventions differ
loading from uppercase legacy input
emitting TitleCase / camelCase / snake_case output
incremental API migration
```

`AliasGenerator` lets a model define separate callables for `alias`, `validation_alias`, and `serialization_alias`; this is useful when loading and saving use different naming conventions. ([Pydantic Docs][S09-1])

---

## 9.12 Alias generator precedence and `alias_priority`

```python
from pydantic import BaseModel, ConfigDict, Field
from pydantic.alias_generators import to_camel

class Voice(BaseModel):
    model_config = ConfigDict(alias_generator=to_camel)

    name: str
    language_code: str = Field(alias="lang")

voice = Voice.model_validate({"name": "Filiz", "lang": "tr-TR"}, by_name=True)
assert voice.model_dump(by_alias=True)["lang"] == "tr-TR"
```

Default precedence:

```text
explicit Field(alias=...) beats generated alias
explicit Field(validation_alias=...) beats generated validation alias
explicit Field(serialization_alias=...) beats generated serialization alias
```

`alias_priority` changes this behavior:

```text
alias_priority=2:
  explicit alias will not be overridden by alias generator

alias_priority=1:
  explicit alias will be overridden by alias generator

alias_priority unset:
  if explicit alias is set, it is not overridden
  if alias is not set, generated alias applies
```

Pydantic documents that explicit field aliases take precedence over generated aliases by default and that `alias_priority` can force generated aliases to override or preserve explicit aliases; the same precedence logic applies to `validation_alias` and `serialization_alias`. ([Pydantic Docs][S09-1])

Agent rule:

```text
Default is usually correct:
  generator for convention
  explicit alias for exceptions

Use alias_priority only when generated convention must override local field metadata.
```

---

## 9.13 Validation by alias vs by field name

### Config-level policy

```python
from pydantic import BaseModel, ConfigDict, Field

class Model(BaseModel):
    model_config = ConfigDict(validate_by_alias=True, validate_by_name=False)

    my_field: str = Field(validation_alias="myAlias")

Model.model_validate({"myAlias": "foo"})
```

Config switches:

```text
validate_by_alias:
  whether aliases are accepted during validation
  default True

validate_by_name:
  whether Python field names are accepted during validation
  default False

both False:
  invalid; raises user error
```

Pydantic’s alias docs state that validation by alias defaults to `True`, validation by field name defaults to `False`, and both cannot be disabled simultaneously. ([Pydantic Docs][S09-1])

### Runtime policy

```python
Model.model_validate(
    {"my_field": "foo"},
    by_alias=False,
    by_name=True,
)
```

Runtime switches are available on:

```text
model_validate(...)
model_validate_json(...)
model_validate_strings(...)
TypeAdapter validation methods
```

Pydantic documents runtime `by_alias` and `by_name` flags for model and TypeAdapter validation methods; they default to `by_alias=True` and `by_name=False`. ([Pydantic Docs][S09-1])

---

## 9.14 Serialization by alias

```python
class Model(BaseModel):
    my_field: str = Field(serialization_alias="myAlias")

m = Model(my_field="foo")

assert m.model_dump() == {"my_field": "foo"}
assert m.model_dump(by_alias=True) == {"myAlias": "foo"}
```

Default:

```text
serialization by alias:
  default False
  enable per call: model_dump(by_alias=True), model_dump_json(by_alias=True)
  enable model-wide: ConfigDict(serialize_by_alias=True)
```

Pydantic documents that serialization by alias is disabled by default, unlike validation by alias; it notes this inconsistency and anticipates a default change in v3. ([Pydantic Docs][S09-1])

Agent rule:

```text
Always set serialization alias policy explicitly for public APIs.
Do not rely on default by_alias behavior for long-lived contracts.
```

---

## 9.15 Public API naming pattern: snake_case internal, camelCase JSON

```python
from pydantic import BaseModel, ConfigDict
from pydantic.alias_generators import to_camel

class ApiModel(BaseModel):
    model_config = ConfigDict(
        alias_generator=to_camel,
        validate_by_alias=True,
        validate_by_name=False,
        serialize_by_alias=True,
        extra="forbid",
    )

class CreateUser(ApiModel):
    user_id: int
    full_name: str

payload = CreateUser.model_validate({"userId": 1, "fullName": "Ada"})
assert payload.user_id == 1
assert payload.model_dump() == {"userId": 1, "fullName": "Ada"}
```

Policy:

```text
Python code:
  snake_case attributes

Inbound JSON:
  camelCase aliases only

Outbound JSON:
  camelCase aliases

Unknown keys:
  forbidden
```

Value case:

```text
clean Python code
stable external JSON convention
no duplicated per-field aliases
single project base class
```

`alias_generator` supports model-wide naming conventions, and `serialize_by_alias=True` enables alias output by default at the model level. ([Pydantic Docs][S09-1])

---

## 9.16 Compatibility pattern: accept snake_case and camelCase during migration

```python
class CompatApiModel(BaseModel):
    model_config = ConfigDict(
        alias_generator=to_camel,
        validate_by_alias=True,
        validate_by_name=True,
        serialize_by_alias=True,
    )

class User(CompatApiModel):
    user_id: int
```

Accepted:

```python
User.model_validate({"userId": 1})
User.model_validate({"user_id": 1})
```

Policy:

```text
Use validate_by_alias=True + validate_by_name=True only for migration windows.
Emit one canonical output shape.
Log/deprecate non-canonical input outside Pydantic if needed.
```

Pydantic supports enabling validation by alias, by name, or both through config and runtime flags. ([Pydantic Docs][S09-1])

---

## 9.17 Legacy payload compatibility with `AliasChoices`

```python
from pydantic import BaseModel, Field, AliasChoices

class User(BaseModel):
    user_id: int = Field(
        validation_alias=AliasChoices("user_id", "userId", "uid")
    )
    full_name: str = Field(
        validation_alias=AliasChoices("full_name", "fullName", "name")
    )

u = User.model_validate({"uid": 1, "name": "Ada"})
```

Policy:

```text
first choice = canonical/current key
later choices = legacy keys
serialization emits only canonical policy
```

`AliasChoices` chooses the first present alias according to its ordered choices, making it suitable for compatibility windows where multiple legacy names must be accepted. ([Pydantic Docs][S09-1])

---

## 9.18 Nested vendor payload normalization with `AliasPath`

```python
from pydantic import BaseModel, Field, AliasPath

class PaymentEvent(BaseModel):
    event_id: str = Field(validation_alias=AliasPath("meta", "id"))
    amount_cents: int = Field(validation_alias=AliasPath("data", "amount", "cents"))
    currency: str = Field(validation_alias=AliasPath("data", "amount", "currency"))

event = PaymentEvent.model_validate({
    "meta": {"id": "evt_123"},
    "data": {"amount": {"cents": "1299", "currency": "USD"}},
})
```

Policy:

```text
Use AliasPath for stable structural extraction.
Use before validators or preprocessing for conditional/algorithmic extraction.
```

`AliasPath` supports string and integer path components and is specifically designed as a `validation_alias` convenience. ([Pydantic Docs][S09-1])

---

## 9.19 Inbound/outbound naming divergence with `AliasGenerator`

```python
from pydantic import AliasGenerator, BaseModel, ConfigDict
from pydantic.alias_generators import to_camel, to_pascal

class ApiBridge(BaseModel):
    model_config = ConfigDict(
        alias_generator=AliasGenerator(
            validation_alias=to_camel,
            serialization_alias=to_pascal,
        ),
        validate_by_alias=True,
        serialize_by_alias=True,
    )

class User(ApiBridge):
    user_id: int
    full_name: str

u = User.model_validate({"userId": 1, "fullName": "Ada"})
assert u.model_dump() == {"UserId": 1, "FullName": "Ada"}
```

Use cases:

```text
legacy request format differs from new response format
third-party input convention differs from owned output convention
ETL: source-system names → target-system names
```

`AliasGenerator` supports separate validation and serialization alias callables for this exact class of differing load/save naming conventions. ([Pydantic Docs][S09-1])

---

## 9.20 Environment variable names and settings aliases

```python
from pydantic import Field, AliasChoices
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="APP_")

    auth_key: str = Field(validation_alias="MY_AUTH_KEY")
    api_key: str = Field(alias="MY_API_KEY")
    redis_dsn: str = Field(
        validation_alias=AliasChoices("SERVICE_REDIS_DSN", "REDIS_URL")
    )
```

Settings semantics:

```text
BaseSettings:
  reads missing initializer values from environment

default env name:
  same as field name

env_prefix:
  prefixes field-name-derived environment variable names

Field(validation_alias=...):
  override inbound env var name for a field

Field(alias=...):
  override env var name and serialization alias

AliasChoices:
  multiple env var names; first found wins
```

Pydantic Settings docs state that default env var names match field names, `env_prefix` sets a prefix, field aliases can override individual env var names, `validation_alias` affects env lookup, `alias` affects validation and serialization, and `AliasChoices` allows multiple env var names with the first found used. ([Pydantic Docs][S09-3])

---

## 9.21 `env_prefix` and alias interaction

```python
class FooBarSettings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="TARGET_")

    foo: str = Field(alias="FooAlias")
    bar: str
```

Default behavior:

```text
foo reads from FooAlias
bar reads from TARGET_bar / TARGET_BAR depending case policy
env_prefix is ignored for alias by default
```

Pydantic Settings documents `env_prefix_target` options:

```text
env_prefix_target="variable":
  default; apply prefix only to field-name-derived variables

env_prefix_target="all":
  apply prefix to variable names and aliases

env_prefix_target="alias":
  apply prefix only to aliases
```

It also states env vars are case-insensitive by default, with `case_sensitive=True` available, though Windows environment variables are always case-insensitive through Python’s `os` behavior. ([Pydantic Docs][S09-3])

Agent rule:

```text
Settings aliases should be explicit.
Do not assume env_prefix applies to aliases unless env_prefix_target says so.
Use AliasChoices for env var migrations.
```

---

## 9.22 CLI/env string models vs aliases

```python
class CliArgs(BaseModel):
    model_config = ConfigDict(
        alias_generator=to_camel,
        validate_by_alias=True,
        validate_by_name=True,
    )

    output_path: str
    max_items: int

CliArgs.model_validate_strings({"outputPath": "/tmp/out", "maxItems": "10"})
CliArgs.model_validate_strings({"output_path": "/tmp/out", "max_items": "10"})
```

Use for:

```text
CLI parsers that produce camelCase option maps
legacy env/config maps
forms/query params with mixed naming
```

Runtime validation methods support alias/name flags, including `model_validate_strings(...)`; Pydantic documents these runtime switches for model validation and TypeAdapter methods. ([Pydantic Docs][S09-1])

---

## 9.23 Alias locations in validation errors

```python
from pydantic import BaseModel, ConfigDict, Field, ValidationError

class M(BaseModel):
    model_config = ConfigDict(loc_by_alias=True)
    my_field: int = Field(validation_alias="myAlias")

try:
    M.model_validate({"myAlias": "bad"})
except ValidationError as exc:
    print(exc.errors()[0]["loc"])
```

Policy:

```text
Public API errors:
  loc_by_alias=True often matches client payload keys

Internal/debug errors:
  loc_by_alias=False can match Python field names
```

`loc_by_alias` is a Pydantic config setting that controls whether error locations use the actual provided alias rather than the field name. It is listed in `ConfigDict` along with alias-related settings. ([Pydantic Docs][S09-4])

---

## 9.24 Alias policy base classes

### Strict public JSON API

```python
from pydantic import BaseModel, ConfigDict
from pydantic.alias_generators import to_camel

class PublicJsonModel(BaseModel):
    model_config = ConfigDict(
        alias_generator=to_camel,
        validate_by_alias=True,
        validate_by_name=False,
        serialize_by_alias=True,
        loc_by_alias=True,
        extra="forbid",
    )
```

### Migration-compatible API

```python
class CompatJsonModel(BaseModel):
    model_config = ConfigDict(
        alias_generator=to_camel,
        validate_by_alias=True,
        validate_by_name=True,
        serialize_by_alias=True,
        loc_by_alias=True,
        extra="forbid",
    )
```

### Internal Python DTO

```python
class InternalModel(BaseModel):
    model_config = ConfigDict(
        validate_by_alias=False,
        validate_by_name=True,
        serialize_by_alias=False,
        extra="forbid",
    )
```

Agent rule:

```text
Define alias policy once in base classes.
Avoid ad hoc alias config per model unless the model is a bridge/adapter.
```

---

## 9.25 Alias inspection

```python
class User(BaseModel):
    id: int = Field(alias="userId")
    name: str = Field(validation_alias="fullName")

for name, info in User.model_fields.items():
    print(name, info.alias, info.validation_alias, info.serialization_alias)
```

Use cases:

```text
schema documentation generation
form/input mapping
client SDK generation
migration audits
alias collision detection
```

Pydantic’s fields docs show inspecting `model_fields` for field metadata such as alias and annotation; instance-level access to `model_fields` is deprecated in v2.11 and removed in v3. ([Pydantic Docs][S09-2])

---

## 9.26 Collision and ambiguity policy

Potential collisions:

```text
two fields produce same generated alias
explicit alias matches another field name
AliasChoices overlaps across fields
AliasPath extracts same source path for multiple fields
validate_by_alias=True + validate_by_name=True accepts two names for one field
```

Agent defensive pattern:

```python
def assert_no_serialization_alias_collisions(model: type[BaseModel]) -> None:
    seen: dict[str, str] = {}
    for field_name, field in model.model_fields.items():
        out = field.serialization_alias or field.alias or field_name
        if out in seen:
            raise AssertionError(f"Alias collision: {out!r} for {seen[out]!r} and {field_name!r}")
        seen[out] = field_name
```

Policy:

```text
For public API models:
  test alias collisions
  test accepted inbound spellings
  test emitted outbound keys
  test error loc policy
```

---

## 9.27 Deployment decision matrix

```text
Need                                      Mechanism
--------------------------------------------------------------------------------
same inbound/outbound external key          Field(alias="externalName")
input-only external key                     Field(validation_alias="externalName")
output-only external key                    Field(serialization_alias="externalName")
nested inbound extraction                   Field(validation_alias=AliasPath(...))
multiple legacy inbound keys                Field(validation_alias=AliasChoices(...))
global snake_case → camelCase               ConfigDict(alias_generator=to_camel)
different input/output naming conventions   AliasGenerator(validation_alias=..., serialization_alias=...)
accept aliases only                         validate_by_alias=True, validate_by_name=False
accept field names only                     validate_by_alias=False, validate_by_name=True
migration: accept both                      validate_by_alias=True, validate_by_name=True
emit aliases by default                     serialize_by_alias=True
emit aliases per call                       model_dump(by_alias=True)
settings env var override                   Field(validation_alias=...)
settings env var migration                  AliasChoices(...)
public error locations use wire keys        loc_by_alias=True
```

---

## 9.28 Anti-patterns

### Anti-pattern: renaming Python fields to camelCase

```python
class User(BaseModel):
    userId: int
    fullName: str
```

Preferred:

```python
class User(PublicJsonModel):
    user_id: int
    full_name: str
```

Value:

```text
Python remains idiomatic.
External JSON remains camelCase.
```

---

### Anti-pattern: `alias` when inbound and outbound should differ

```python
class M(BaseModel):
    x: int = Field(alias="legacyX")
```

Preferred:

```python
class M(BaseModel):
    x: int = Field(validation_alias="legacyX", serialization_alias="x")
```

---

### Anti-pattern: accepting both alias and name permanently

```python
model_config = ConfigDict(validate_by_alias=True, validate_by_name=True)
```

Problem:

```text
two inbound spellings
ambiguous client behavior
harder deprecation
harder analytics
```

Use only for migration windows unless the API explicitly supports both.

---

### Anti-pattern: relying on serialization alias default

```python
model.model_dump()
```

Problem:

```text
by_alias defaults False today
Pydantic docs anticipate v3 default change
public contracts should not depend on implicit default
```

Preferred:

```python
model.model_dump(by_alias=True)
```

or:

```python
model_config = ConfigDict(serialize_by_alias=True)
```

Pydantic notes that serialization by alias is disabled by default and that this may change in v3. ([Pydantic Docs][S09-1])

---

### Anti-pattern: overly complex `AliasPath` logic

```python
Field(validation_alias=AliasPath("a", 0, "b", 1, "c", "d", 2))
```

If shape extraction is nontrivial, define a separate input DTO or a `model_validator(mode="before")`.

---

### Anti-pattern: env alias without prefix policy awareness

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="APP_")
    token: str = Field(alias="TOKEN")
```

`APP_TOKEN` may not be read for the alias under default `env_prefix_target`; explicit `env_prefix_target` policy is required if prefix should apply to aliases. ([Pydantic Docs][S09-3])

---

## 9.29 Test matrix

### Field alias behavior

```python
def test_alias_inbound_outbound():
    class User(BaseModel):
        name: str = Field(alias="username")

    u = User.model_validate({"username": "ada"})

    assert u.name == "ada"
    assert u.model_dump() == {"name": "ada"}
    assert u.model_dump(by_alias=True) == {"username": "ada"}
```

### Validation vs serialization alias

```python
def test_validation_and_serialization_alias_split():
    class M(BaseModel):
        x: int = Field(validation_alias="inX", serialization_alias="outX")

    m = M.model_validate({"inX": "1"})

    assert m.x == 1
    assert m.model_dump(by_alias=True) == {"outX": 1}
```

### AliasChoices priority

```python
def test_alias_choices_priority():
    class M(BaseModel):
        x: int = Field(validation_alias=AliasChoices("x", "legacyX"))

    assert M.model_validate({"legacyX": 1}).x == 1
    assert M.model_validate({"x": 2, "legacyX": 1}).x == 2
```

### AliasPath extraction

```python
def test_alias_path():
    class M(BaseModel):
        first: str = Field(validation_alias=AliasPath("names", 0))
        second: str = Field(validation_alias=AliasPath("names", 1))

    m = M.model_validate({"names": ["Ada", "Lovelace"]})

    assert m.first == "Ada"
    assert m.second == "Lovelace"
```

### Generator output

```python
def test_camel_generator():
    class M(BaseModel):
        model_config = ConfigDict(
            alias_generator=to_camel,
            validate_by_alias=True,
            serialize_by_alias=True,
        )

        user_id: int

    m = M.model_validate({"userId": 1})

    assert m.user_id == 1
    assert m.model_dump() == {"userId": 1}
```

### Alias-only strict policy

```python
def test_alias_only_validation():
    class M(BaseModel):
        model_config = ConfigDict(
            alias_generator=to_camel,
            validate_by_alias=True,
            validate_by_name=False,
        )

        user_id: int

    assert M.model_validate({"userId": 1}).user_id == 1

    with pytest.raises(ValidationError):
        M.model_validate({"user_id": 1})
```

### Settings alias

```python
def test_settings_alias(monkeypatch):
    class Settings(BaseSettings):
        auth_key: str = Field(validation_alias="MY_AUTH_KEY")

    monkeypatch.setenv("MY_AUTH_KEY", "secret")

    assert Settings().auth_key == "secret"
```

---

## 9.30 Agent checklist

```text
[ ] Keep Python field names snake_case.
[ ] Use alias only when inbound and outbound external names are identical.
[ ] Use validation_alias for input-only external names.
[ ] Use serialization_alias for output-only external names.
[ ] Use AliasPath for structural inbound flattening.
[ ] Use AliasChoices for legacy inbound compatibility.
[ ] Put current/canonical alias first in AliasChoices.
[ ] Use alias_generator for model-wide naming convention.
[ ] Use built-in to_camel/to_pascal/to_snake where possible.
[ ] Use AliasGenerator when validation and serialization conventions differ.
[ ] Set validate_by_alias / validate_by_name explicitly in base models.
[ ] Avoid permanent validate_by_alias=True + validate_by_name=True unless required.
[ ] Set serialize_by_alias=True for public response base models.
[ ] Use model_dump(by_alias=True) explicitly when output contract requires aliases.
[ ] Test alias collisions for public models.
[ ] Test inbound accepted spellings and outbound emitted spellings.
[ ] For settings, remember env_prefix does not necessarily apply to aliases.
[ ] Use validation_alias / AliasChoices for env var migrations.
[ ] Use loc_by_alias=True for client-facing validation errors.
[ ] Avoid relying on alias serialization defaults because v3 may change them.
```

---

## 9.31 Value case

```text
Alias value =
  internal Python naming stays idiomatic
  external API naming stays contract-correct
  request and response shapes can differ
  legacy inbound names can coexist temporarily
  nested vendor payloads can be flattened
  model-wide naming conventions avoid per-field boilerplate
  settings env vars can have deployment-friendly names
  validation error locations can match client payload keys
  migration windows can be encoded without duplicating models
```

Operationally: define canonical Python field names once, map every external shape through explicit alias policy, keep validation aliases and serialization aliases distinct when contracts differ, and test accepted inbound keys plus emitted outbound keys as part of the public API surface.

[S09-1]: https://docs.pydantic.dev/latest/concepts/alias/ "Alias | Pydantic Docs"
[S09-2]: https://docs.pydantic.dev/latest/concepts/fields/ "Fields | Pydantic Docs"
[S09-3]: https://docs.pydantic.dev/latest/concepts/pydantic_settings/ "Settings Management | Pydantic Docs"
[S09-4]: https://docs.pydantic.dev/latest/api/config/ "Configuration | Pydantic Docs"


# Pydantic Advanced — 10) Model configuration with `ConfigDict` — Pydantic v2 deep dive

Style target: advanced, sectioned, agent-oriented technical reference. 

Pydantic v2 configuration is centered on `ConfigDict`: a typed dictionary for controlling model, dataclass, `TypeAdapter`, `TypedDict`, stdlib dataclass, and `@validate_call` behavior. On `BaseModel`, configuration can be declared through `model_config = ConfigDict(...)` or through class keyword arguments such as `class Model(BaseModel, frozen=True): ...`; the old v1 inner `Config` class remains supported but deprecated. ([Pydantic Docs][S10-1])

---

## 10.0 Configuration mental model

```text
class body
  + type annotations
  + Field metadata
  + validators / serializers
  + model_config / class keyword config
  → core schema
  → validation behavior
  → assignment behavior
  → serialization behavior
  → JSON Schema behavior
```

Configuration is **schema policy**, not business logic:

```text
ConfigDict controls:
  extra-field handling
  mutability / hashability
  strictness / coercion
  assignment validation
  nested instance revalidation
  arbitrary type allowance
  string normalization defaults
  alias validation / serialization
  JSON Schema generation options
  temporal / bytes / inf-nan serialization
  import/build performance knobs
  error display privacy
```

---

## 10.1 Primary syntax: `model_config = ConfigDict(...)`

```python
from pydantic import BaseModel, ConfigDict

class Model(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        strict=True,
        str_strip_whitespace=True,
        str_min_length=1,
    )

    name: str
    age: int
```

Equivalent plain dictionary form:

```python
class Model(BaseModel):
    model_config = {
        "extra": "forbid",
        "strict": True,
    }

    name: str
```

`model_config` is the normal v2 configuration surface; a plain dictionary can also be used, but `ConfigDict(...)` is clearer and type-checker-friendly for config keys. ([Pydantic Docs][S10-1])

Agent rule:

```text
Prefer:
  model_config = ConfigDict(...)

Avoid:
  class Config: ...
  untyped dicts for large configs
  per-model copy/paste of project policy
```

---

## 10.2 Class keyword configuration

```python
from pydantic import BaseModel

class FrozenModel(BaseModel, frozen=True):
    id: int
    name: str
```

Class keyword configuration is recognized by static type checkers more directly than `model_config`; Pydantic docs specifically note that class arguments such as `frozen=True` let type checkers flag instance mutation. ([Pydantic Docs][S10-1])

Common class keyword use:

```python
class User(BaseModel, validate_assignment=True):
    name: str

class ValueObject(BaseModel, frozen=True):
    id: int
```

Agent rule:

```text
Use class keyword config for:
  frozen=True
  validate_assignment=True
  strict=True
  revalidate_instances="always"

Use model_config for:
  many settings
  inherited project policy
  alias generators
  JSON Schema options
  string normalization
```

---

## 10.3 `extra`: unknown input fields

```python
from pydantic import BaseModel, ConfigDict, Field

class IgnoreExtra(BaseModel):
    model_config = ConfigDict(extra="ignore")
    x: int

class ForbidExtra(BaseModel):
    model_config = ConfigDict(extra="forbid")
    x: int

class AllowExtra(BaseModel):
    model_config = ConfigDict(extra="allow")
    x: int
```

Modes:

```text
extra="ignore":
  default
  unknown input keys discarded

extra="forbid":
  unknown input keys produce ValidationError type=extra_forbidden

extra="allow":
  unknown input keys stored in __pydantic_extra__
  included in model_dump()
```

`extra` defaults to `"ignore"`; `"forbid"` rejects extra data; `"allow"` stores extra values in `__pydantic_extra__`. Validation methods can also override `extra` per call. ([Pydantic Docs][S10-2])

### Typed extras

```python
class AllowTypedExtra(BaseModel):
    __pydantic_extra__: dict[str, int] = Field(init=False)

    model_config = ConfigDict(extra="allow")
    x: int

m = AllowTypedExtra.model_validate({"x": 1, "y": "2"})
assert m.__pydantic_extra__ == {"y": 2}
assert m.model_dump() == {"x": 1, "y": 2}
```

When `extra="allow"`, extra values are not validated by default, but overriding `__pydantic_extra__` with a type annotation validates extra values. ([Pydantic Docs][S10-2])

Deployment policy:

```text
Public API request model:
  extra="forbid"

External webhook consumer:
  extra="ignore" for forward compatibility
  or extra="allow" if unknown fields must be preserved

Internal domain DTO:
  extra="forbid"

LLM structured output contract:
  extra="forbid"

Flexible metadata bag:
  extra="allow" + typed __pydantic_extra__
```

---

## 10.4 `frozen`: model-level faux immutability

```python
class FrozenUser(BaseModel):
    model_config = ConfigDict(frozen=True)

    id: int
    name: str
```

Effects:

```text
frozen=True:
  blocks __setattr__
  generates __hash__ if all attributes are hashable
  makes model potentially hashable
  does not guarantee deep immutability of nested mutable objects
```

`frozen` makes models faux-immutable by disallowing `__setattr__` and generating `__hash__()` when attributes are hashable; default is `False`. ([Pydantic Docs][S10-2])

Agent rule:

```text
Use frozen=True for:
  value objects
  cache keys
  immutable command objects
  IDs / coordinates / normalized domain values

Do not use frozen=True as:
  deep freeze
  authorization protection
  concurrency lock
```

---

## 10.5 `strict`: model-wide coercion policy

```python
class StrictModel(BaseModel):
    model_config = ConfigDict(strict=True)

    name: str
    age: int
```

`strict=True` applies strict validation to all fields on the model; by default, Pydantic attempts to coerce values to the correct type when possible. Strict mode disables many coercions and raises when the input type does not match the field annotation. ([Pydantic Docs][S10-2])

Policy:

```text
API input models:
  usually not global strict
  use selective Field(strict=True)

Internal domain models:
  ConfigDict(strict=True)

Settings/env models:
  usually not global strict
  env values are strings

Test/audit mode:
  call Model.model_validate(data, strict=True)
```

---

## 10.6 `validate_assignment`: mutation-time validation

```python
from pydantic import BaseModel, ValidationError

class User(BaseModel, validate_assignment=True):
    name: str

user = User(name="Ada")

try:
    user.name = 123
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "string_type"
```

By default, Pydantic validates on model creation only; after creation, attribute assignment is not revalidated. `validate_assignment=True` revalidates values when fields are changed and can be set either with class keyword arguments or `model_config`. ([Pydantic Docs][S10-2])

Policy:

```text
Mutable but type-safe object:
  validate_assignment=True

Immutable object:
  frozen=True

High-throughput trusted DTO:
  validate_assignment=False

Patch workflow:
  prefer new validated instance over mutation
```

---

## 10.7 `revalidate_instances`: nested model/dataclass instance revalidation

```python
class User(BaseModel, revalidate_instances="always"):
    name: str

class Transaction(BaseModel):
    user: User
```

Modes:

```text
revalidate_instances="never":
  default
  existing model/dataclass instances are not revalidated

revalidate_instances="always":
  existing model/dataclass instances are revalidated

revalidate_instances="subclass-instances":
  only subclass instances are revalidated
```

`revalidate_instances` controls when models and dataclasses are revalidated during validation; default is `"never"`. This setting applies to the model it is configured on and does not propagate to referenced models. ([Pydantic Docs][S10-2])

Agent rule:

```text
Long-lived mutable model graph:
  validate_assignment=True
  revalidate_instances="always"

Plugin/subclass boundary:
  revalidate_instances="subclass-instances"

Performance-sensitive internal graph:
  revalidate_instances="never"
```

---

## 10.8 `arbitrary_types_allowed`: non-Pydantic arbitrary classes

```python
class Pet:
    def __init__(self, name: str) -> None:
        self.name = name

class Owner(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)

    pet: Pet
    owner: str
```

Behavior:

```text
arbitrary_types_allowed=True:
  value must be instance of annotated arbitrary class
  internals of that arbitrary instance are not validated
```

Pydantic validates arbitrary types with a simple instance check; it does not validate the arbitrary object’s internal attributes. ([Pydantic Docs][S10-2])

Agent rule:

```text
Use arbitrary_types_allowed=True for:
  injected runtime handles
  opaque clients
  already-validated domain objects
  dependency objects

Avoid for:
  untrusted payload shapes
  public API schemas
  objects whose internals must be validated

Prefer:
  custom type hooks
  wrapper BaseModel
  dataclass / TypedDict / TypeAdapter validation
```

---

## 10.9 String normalization options

```python
class NormalizedText(BaseModel):
    model_config = ConfigDict(
        str_strip_whitespace=True,
        str_to_lower=True,
        str_min_length=1,
        str_max_length=64,
    )

    username: str
```

String config options:

```text
str_strip_whitespace:
  strip leading/trailing whitespace

str_to_lower:
  lowercase all characters for str types

str_to_upper:
  uppercase all characters for str types

str_min_length:
  global minimum length for str fields

str_max_length:
  global maximum length for str fields
```

`ConfigDict` exposes string normalization and length options including `str_to_lower`, `str_to_upper`, `str_strip_whitespace`, `str_min_length`, and `str_max_length`. ([Pydantic Docs][S10-2])

Policy:

```text
Use global string normalization for:
  homogeneous text DTOs
  command/settings/config models
  normalized identifiers

Avoid global string normalization for:
  mixed text models
  passwords
  case-sensitive tokens
  cryptographic material
  user-generated rich text
```

Fine-grained alternative:

```python
from typing import Annotated
from pydantic import StringConstraints

Username = Annotated[
    str,
    StringConstraints(strip_whitespace=True, to_lower=True, min_length=1, max_length=64),
]
```

---

## 10.10 Alias behavior config

```python
from pydantic.alias_generators import to_camel

class ApiModel(BaseModel):
    model_config = ConfigDict(
        alias_generator=to_camel,
        validate_by_alias=True,
        validate_by_name=False,
        serialize_by_alias=True,
        loc_by_alias=True,
    )
```

Relevant config keys:

```text
alias_generator:
  generate field aliases from Python field names
  callable or AliasGenerator

validate_by_alias:
  allow input by alias
  default True

validate_by_name:
  allow input by Python field name
  default False

serialize_by_alias:
  serialize aliases by default
  default False today; expected to become True in v3

loc_by_alias:
  use actual input key / alias in error locations
  default True
```

`alias_generator` can be a callable or `AliasGenerator`; `AliasGenerator` supports different validation and serialization alias generators. `validate_by_alias`/`validate_by_name` provide fine-grained validation-name control and cannot both be false; `serialize_by_alias` controls default alias serialization and is expected to default to true in v3. ([Pydantic Docs][S10-2])

Migration note:

```text
populate_by_name=True:
  not recommended in v2.11+
  deprecated in v3
  replace with validate_by_name=True + validate_by_alias=True
```

Pydantic warns that `populate_by_name` is not recommended in v2.11+ and will be deprecated in v3; equivalent behavior is achieved through `validate_by_name=True` and `validate_by_alias=True`. ([Pydantic Docs][S10-2])

---

## 10.11 JSON Schema configuration

```python
class PublicSchemaModel(BaseModel):
    model_config = ConfigDict(
        title="PublicSchemaModel",
        json_schema_extra={"x-api-owner": "platform"},
        json_schema_serialization_defaults_required=True,
        json_schema_mode_override="validation",
    )
```

Important JSON Schema config keys:

```text
title:
  model schema title

model_title_generator:
  callable that derives model schema title

field_title_generator:
  callable that derives field title

json_schema_extra:
  dict or callable that adds/mutates schema metadata

json_schema_serialization_defaults_required:
  mark defaulted fields required in serialization schema

json_schema_mode_override:
  force validation or serialization schema mode regardless of call mode

use_attribute_docstrings:
  use attribute docstrings as field descriptions when source is available
```

`json_schema_extra` adds extra JSON Schema properties; `json_schema_serialization_defaults_required` marks fields with defaults as required in serialization schema; `json_schema_mode_override` forces a schema mode; `use_attribute_docstrings` can use immediately following attribute docstrings as descriptions when source code is available. ([Pydantic Docs][S10-2])

Deprecated / caution:

```text
json_encoders:
  deprecated v2 carryover from v1

schema_generator:
  deprecated v2.10
  private and subject to change
```

`json_encoders` is retained from v1 but deprecated and likely to be removed; `schema_generator` is deprecated in v2.10 because the referenced generation class is private and highly subject to change. ([Pydantic Docs][S10-2])

---

## 10.12 Serialization-related config

```python
class WireModel(BaseModel):
    model_config = ConfigDict(
        ser_json_temporal="iso8601",
        ser_json_bytes="base64",
        val_json_bytes="base64",
        ser_json_inf_nan="null",
        serialize_by_alias=True,
    )
```

High-impact keys:

```text
ser_json_temporal:
  datetime/date/time/timedelta JSON serialization format
  "iso8601" | "seconds" | "milliseconds"

ser_json_timedelta:
  older timedelta-only setting
  recommended replacement: ser_json_temporal
  deprecated in v3

val_temporal_unit:
  numeric datetime/date input unit policy

ser_json_bytes:
  bytes serialization policy

val_json_bytes:
  bytes validation policy

ser_json_inf_nan:
  inf/nan JSON serialization policy

serialize_by_alias:
  default alias serialization policy

polymorphic_serialization:
  serialize subclass runtime schemas for model/dataclass subclasses
```

`ser_json_temporal` was added in v2.12 and replaces `ser_json_timedelta`, which will be deprecated in v3; it controls JSON serialization for datetime/date/time/timedelta. ([Pydantic Docs][S10-2])

---

## 10.13 Error and safety config

```python
class SafeErrors(BaseModel):
    model_config = ConfigDict(
        hide_input_in_errors=True,
        protected_namespaces=("model_validate", "model_dump"),
    )

    token: str
```

Relevant keys:

```text
hide_input_in_errors:
  omit input value/type from printed ValidationError output

protected_namespaces:
  prevent fields colliding with protected BaseModel members / namespaces

validation_error_cause:
  expose underlying Python exceptions as exception group cause for debugging
```

`hide_input_in_errors=True` hides input values and types in validation-error display. `protected_namespaces` defaults changed in v2.10 from `("model_",)` to `("model_validate", "model_dump")`, allowing fields like `model_id` while still protecting core methods. ([Pydantic Docs][S10-2])

Policy:

```text
Public/user-facing validation errors:
  hide_input_in_errors=True for secret-bearing payloads

Internal debugging:
  hide_input_in_errors=False
  validation_error_cause=True selectively

Model field names:
  avoid model_validate / model_dump collisions
```

---

## 10.14 Performance/config build knobs

```python
class ColdStartOptimized(BaseModel):
    model_config = ConfigDict(
        defer_build=True,
        cache_strings="keys",
    )

    payload: dict[str, str]
```

Relevant keys:

```text
defer_build:
  delay validator/serializer construction until first validation
  useful for rarely used nested models or manual namespace rebuilds
  applies to models, pydantic dataclasses, and TypeAdapters

cache_strings:
  cache strings during validation
  True/"all" default
  "keys" caches dict keys only
  False/"none" disables
```

`defer_build` defers model validator/serializer construction until first validation and also applies to dataclasses and TypeAdapters since v2.10; `cache_strings` can improve validation performance with slight memory cost, with `"keys"` or `"none"` recommended when repeated strings are rare. ([Pydantic Docs][S10-2])

Deployment rule:

```text
Serverless / CLI cold start:
  consider defer_build=True on rarely used schema graphs

High-throughput repeated JSON keys:
  cache_strings=True or "keys"

Memory-sensitive low-repeat workloads:
  cache_strings="keys" or "none"
```

---

## 10.15 Regex engine config

```python
class RegexModel(BaseModel):
    model_config = ConfigDict(regex_engine="python-re")

    value: str = Field(pattern=r"^abc(?=def)")
```

Options:

```text
regex_engine="rust-regex":
  default
  non-backtracking
  more DDoS resistant
  not all Python regex features supported

regex_engine="python-re":
  supports Python regex features
  may be slower

compiled regex pattern:
  python-re used regardless of setting
```

Pydantic’s config docs describe `regex_engine`: default `"rust-regex"` is non-backtracking and more DDoS resistant; `"python-re"` supports more features but may be slower. ([Pydantic Docs][S10-2])

Policy:

```text
Public untrusted regex validation:
  prefer default rust-regex

Need lookarounds/backrefs/Python flags:
  regex_engine="python-re" or compiled pattern
  test performance / ReDoS exposure
```

---

## 10.16 Configuration inheritance and merging

```python
class Parent(BaseModel):
    model_config = ConfigDict(extra="allow", str_to_lower=False)

class Child(Parent):
    model_config = ConfigDict(str_to_lower=True)

    x: str

assert Child.model_config == {"extra": "allow", "str_to_lower": True}
```

Configuration is inherited. Subclass config merges with parent config, overriding duplicate keys. Pydantic currently does not follow normal Python MRO for config merging across multiple base classes, so multiple inheritance should be used carefully and tested. ([Pydantic Docs][S10-1])

Agent rule:

```text
Use single-inheritance config base classes.
Avoid config-bearing multiple inheritance.
If multiple inheritance is unavoidable:
  assert Model.model_config in tests.
```

---

## 10.17 Global policy via custom parent base model

```python
class AppModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        validate_assignment=False,
        hide_input_in_errors=True,
    )

class ApiInputModel(AppModel):
    model_config = ConfigDict(
        alias_generator=to_camel,
        validate_by_alias=True,
        validate_by_name=False,
        serialize_by_alias=True,
        loc_by_alias=True,
    )

class DomainModel(AppModel):
    model_config = ConfigDict(
        strict=True,
        frozen=True,
        revalidate_instances="always",
    )
```

Pydantic recommends changing behavior globally by creating a custom parent class with custom configuration, because configuration is inherited by subclasses. ([Pydantic Docs][S10-1])

Policy:

```text
One base model per boundary:
  ApiInputModel
  ApiOutputModel
  EventModel
  DomainModel
  SettingsModel

Do not use one universal BaseModel for all boundaries.
```

---

## 10.18 Configuration boundaries: nested Pydantic models

```python
class User(BaseModel):
    name: str

class Parent(BaseModel):
    model_config = ConfigDict(str_to_lower=True)
    user: User

p = Parent.model_validate({"user": {"name": "JOHN"}})
assert p.user.name == "JOHN"
```

For Pydantic models and Pydantic dataclasses used as field annotations, parent configuration does **not** propagate into the nested model; each model/dataclass has its own configuration boundary. ([Pydantic Docs][S10-1])

Agent rule:

```text
If nested User.name must lowercase:
  configure User itself.

Do not expect:
  Parent.model_config to change nested Pydantic model behavior.
```

---

## 10.19 Configuration propagation: stdlib dataclasses and `TypedDict`

```python
from dataclasses import dataclass
from typing_extensions import TypedDict
from pydantic import TypeAdapter, with_config

@dataclass
class UserWithoutConfig:
    name: str

@with_config(ConfigDict(str_to_lower=False))
class UserWithConfig(TypedDict):
    name: str

class Parent(BaseModel):
    model_config = ConfigDict(str_to_lower=True)

    user_1: UserWithoutConfig
    user_2: UserWithConfig
```

For stdlib dataclasses and typed dictionaries, parent configuration propagates unless the nested type has its own configuration set. Pydantic supports `__pydantic_config__` or `@with_config(...)` for stdlib dataclasses and `TypedDict`; `@with_config` avoids static type-checker issues, especially for `TypedDict`. ([Pydantic Docs][S10-1])

Agent rule:

```text
Pydantic model nested:
  config boundary; no parent propagation

stdlib dataclass / TypedDict nested:
  parent config can propagate unless nested type has config

Need stable behavior:
  configure nested type explicitly
```

---

## 10.20 Pydantic dataclass configuration

```python
from pydantic.dataclasses import dataclass

@dataclass(config=ConfigDict(str_max_length=10, validate_assignment=True))
class User:
    name: str
```

Pydantic dataclasses support configuration through the `config=` argument; the docs show `str_max_length` and `validate_assignment=True` applied to a Pydantic dataclass. ([Pydantic Docs][S10-1])

Policy:

```text
Use pydantic dataclass config when:
  dataclass ergonomics are required
  constructor/assignment behavior needs validation
  BaseModel methods are not required

Use BaseModel when:
  model_dump/model_dump_json/model_json_schema are central
  config inheritance through BaseModel hierarchy is desired
```

---

## 10.21 `TypeAdapter` configuration

```python
from pydantic import TypeAdapter

ta = TypeAdapter(
    list[str],
    config=ConfigDict(coerce_numbers_to_str=True),
)

assert ta.validate_python([1, 2]) == ["1", "2"]
```

`TypeAdapter` accepts a `config` argument, but configuration cannot be provided if the adapter directly wraps a type that itself supports configuration; in that case, Pydantic raises a usage error. ([Pydantic Docs][S10-1])

Use cases:

```text
top-level list/dict/union/primitive validation
strictness policy for adapted non-model types
string/number coercion policy
defer_build for adapter-heavy cold starts
```

---

## 10.22 `@validate_call` configuration

```python
from pydantic import validate_call

@validate_call(config=ConfigDict(strict=True), validate_return=True)
def add(x: int, y: int) -> int:
    return x + y
```

`@validate_call` accepts `config: ConfigDict | None` and `validate_return: bool = False`; it returns a wrapper that validates function arguments and optionally the return value. ([Pydantic Docs][S10-3])

Policy:

```text
Use @validate_call config for:
  public library APIs
  notebook utilities
  task entrypoints
  thin runtime guards

Avoid for:
  hot inner loops
  business logic where validation already occurred
```

---

## 10.23 Deprecated v1 `Config` class

```python
class OldStyle(BaseModel):
    class Config:
        extra = "forbid"
```

Preferred:

```python
class NewStyle(BaseModel):
    model_config = ConfigDict(extra="forbid")
```

The v1 inner `Config` class is still supported but deprecated. ([Pydantic Docs][S10-1])

Migration rule:

```text
Replace:
  class Config:
      extra = "forbid"

With:
  model_config = ConfigDict(extra="forbid")
```

---

## 10.24 v1-to-v2 config rename / removal map

Common replacements:

```text
allow_mutation             removed → use frozen inverse
fields                     removed → use Annotated / Field metadata
getter_dict / orm_mode     removed/renamed → from_attributes
schema_extra               renamed → json_schema_extra
validate_all               renamed → validate_default
anystr_lower               renamed → str_to_lower
anystr_upper               renamed → str_to_upper
anystr_strip_whitespace    renamed → str_strip_whitespace
min_anystr_length          renamed → str_min_length
max_anystr_length          renamed → str_max_length
populate_by_name           legacy-ish → prefer validate_by_name + validate_by_alias
```

Pydantic’s migration docs list v2 config removals and renames, including `allow_mutation`, `fields`, `getter_dict`, `orm_mode → from_attributes`, `schema_extra → json_schema_extra`, string option renames, and `validate_all → validate_default`. ([Pydantic Docs][S10-4])

---

## 10.25 Base policy examples

### Public JSON API input

```python
class ApiInputModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        alias_generator=to_camel,
        validate_by_alias=True,
        validate_by_name=False,
        serialize_by_alias=True,
        loc_by_alias=True,
        hide_input_in_errors=True,
    )
```

Value:

```text
strict external key contract
camelCase wire format
unknown-key rejection
client-facing error locs
secret-safe error display
```

---

### Internal domain value object

```python
class DomainModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        strict=True,
        frozen=True,
        revalidate_instances="always",
    )
```

Value:

```text
no coercive surprises
immutable-ish instances
nested instance hardening
hashable if fields are hashable
```

---

### Settings/config model

```python
class ConfigModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        str_strip_whitespace=True,
        validate_default=True,
    )
```

Value:

```text
clean strings
validated defaults
unknown-key rejection
```

For real environment-backed settings, use `pydantic-settings` `BaseSettings`; keep global strictness off unless the source supplies typed Python objects.

---

### Cold-start optimized schema group

```python
class RareSchema(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        defer_build=True,
    )

    payload: dict[str, object]
```

Value:

```text
lower import-time schema build cost
first-validation cost deferred
useful for CLI/serverless/rare routes
```

---

## 10.26 Anti-patterns

### v1 `Config` in new code

```python
class M(BaseModel):
    class Config:
        extra = "forbid"
```

Use:

```python
class M(BaseModel):
    model_config = ConfigDict(extra="forbid")
```

---

### Parent config expected to mutate nested Pydantic model

```python
class User(BaseModel):
    name: str

class Parent(BaseModel):
    model_config = ConfigDict(str_to_lower=True)
    user: User

# user.name remains unchanged by Parent config
```

Configure `User` itself.

---

### `arbitrary_types_allowed=True` as a validation shortcut

```python
class M(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)
    client: SomeComplexObject
```

Problem:

```text
only instance type checked
internal object state not validated
schema may be weak or unusable for public contracts
```

---

### Global strict on env/settings-like inputs

```python
class EnvModel(BaseModel):
    model_config = ConfigDict(strict=True)
    port: int
```

Problem:

```text
env/query/CLI values are normally strings
strict=True rejects useful string parsing
```

Prefer constrained lax parsing.

---

### Relying on `populate_by_name`

```python
model_config = ConfigDict(populate_by_name=True)
```

Prefer:

```python
model_config = ConfigDict(validate_by_name=True, validate_by_alias=True)
```

`populate_by_name` is not recommended in v2.11+ and will be deprecated in v3. ([Pydantic Docs][S10-2])

---

### `json_encoders` for new serialization policy

```python
model_config = ConfigDict(json_encoders={Decimal: str})
```

Prefer field/type/model serializers. `json_encoders` is deprecated in v2 and retained only as a carryover from v1. ([Pydantic Docs][S10-2])

---

## 10.27 Test matrix

### Config inheritance

```python
def test_config_inheritance_merge():
    class Parent(BaseModel):
        model_config = ConfigDict(extra="allow", str_to_lower=False)

    class Child(Parent):
        model_config = ConfigDict(str_to_lower=True)
        x: str

    assert Child.model_config["extra"] == "allow"
    assert Child.model_config["str_to_lower"] is True
    assert Child(x="FOO", y="bar").model_dump() == {"x": "foo", "y": "bar"}
```

### Extra forbid

```python
def test_extra_forbid():
    class M(BaseModel):
        model_config = ConfigDict(extra="forbid")
        x: int

    with pytest.raises(ValidationError) as exc:
        M.model_validate({"x": 1, "y": 2})

    assert exc.value.errors()[0]["type"] == "extra_forbidden"
```

### Assignment validation

```python
def test_validate_assignment():
    class M(BaseModel, validate_assignment=True):
        x: int

    m = M(x=1)

    with pytest.raises(ValidationError):
        m.x = "bad"
```

### Revalidate instances

```python
def test_revalidate_instances_always():
    class User(BaseModel, revalidate_instances="always"):
        name: str

    class Box(BaseModel):
        user: User

    u = User(name="Ada")
    object.__setattr__(u, "name", 123)

    with pytest.raises(ValidationError):
        Box.model_validate({"user": u})
```

### Nested config boundary

```python
def test_nested_model_config_boundary():
    class User(BaseModel):
        name: str

    class Parent(BaseModel):
        model_config = ConfigDict(str_to_lower=True)
        user: User

    assert Parent(user={"name": "JOHN"}).user.name == "JOHN"
```

### TypedDict `with_config`

```python
def test_with_config_typed_dict():
    @with_config(ConfigDict(str_to_lower=True))
    class TD(TypedDict):
        x: str

    ta = TypeAdapter(TD)

    assert ta.validate_python({"x": "ABC"}) == {"x": "abc"}
```

---

## 10.28 Deployment decision matrix

```text
Need                                             Config
--------------------------------------------------------------------------------
reject unknown public API keys                    extra="forbid"
preserve unknown metadata                         extra="allow" + typed __pydantic_extra__
forward-compatible webhook consumer               extra="ignore"
immutable value object                            frozen=True
strict internal domain model                      strict=True
mutable but checked object                        validate_assignment=True
nested instance hardening                         revalidate_instances="always"
plugin/subclass boundary hardening                revalidate_instances="subclass-instances"
opaque runtime handles                            arbitrary_types_allowed=True
normalize all string fields                       str_strip_whitespace / str_to_lower / str_to_upper
global camelCase API                              alias_generator=to_camel
accept alias only                                 validate_by_alias=True, validate_by_name=False
migration: accept alias and name                  validate_by_alias=True, validate_by_name=True
emit aliases by default                           serialize_by_alias=True
client-facing error locations                     loc_by_alias=True
hide secret-bearing inputs in errors              hide_input_in_errors=True
schema vendor metadata                            json_schema_extra={...}
force validation schema mode                      json_schema_mode_override="validation"
serialization schema defaults required            json_schema_serialization_defaults_required=True
serverless/cold-start optimization                defer_build=True
memory-sensitive string-heavy workload            cache_strings="keys" or "none"
Python regex feature support                      regex_engine="python-re"
DDoS-resistant regex default                      regex_engine="rust-regex"
```

---

## 10.29 Agent checklist

```text
[ ] Use ConfigDict, not v1 class Config.
[ ] Define one base model per boundary.
[ ] Keep public API input models extra="forbid".
[ ] Use strict=True for internal/domain models, not stringly env/query models.
[ ] Use validate_assignment=True only when mutation is part of the lifecycle.
[ ] Pair mutable nested instance flows with revalidate_instances="always" when crossing trust boundaries.
[ ] Avoid arbitrary_types_allowed in public schemas.
[ ] Use string normalization globally only for homogeneous string policy.
[ ] Use alias_generator + serialize_by_alias for external naming conventions.
[ ] Replace populate_by_name with validate_by_name + validate_by_alias.
[ ] Set loc_by_alias=True for client-facing error paths.
[ ] Use json_schema_extra for public schema metadata.
[ ] Avoid json_encoders in new code; use serializers.
[ ] Remember Pydantic models/dataclasses are config boundaries.
[ ] Configure nested Pydantic models explicitly.
[ ] Use with_config for TypedDict / stdlib dataclass config.
[ ] Use TypeAdapter(config=...) only when adapted type does not itself own config.
[ ] Use @validate_call(config=...) for function-call validation boundaries.
[ ] Test Model.model_config for inherited/base classes.
[ ] Avoid config-bearing multiple inheritance unless tested.
```

---

## 10.30 Value case

```text
ConfigDict value =
  centralized model behavior policy
  reusable boundary-specific base classes
  deterministic extra-key handling
  strict/lax coercion control
  mutability and assignment guarantees
  nested instance revalidation strategy
  arbitrary runtime type admission
  consistent string normalization
  alias and wire-format policy
  JSON Schema generation policy
  serialization/time/bytes policy
  error privacy controls
  cold-start/performance controls
  typed dataclass/TypedDict/function-call configuration
```

Operationally: treat `ConfigDict` as the **policy layer** for a schema boundary. Put repeated policy in base models, keep nested model boundaries explicit, avoid deprecated v1 config surfaces, and test config inheritance, alias behavior, extra handling, mutation behavior, and nested instance revalidation as public contract.

[S10-1]: https://docs.pydantic.dev/latest/concepts/config/ "Configuration | Pydantic Docs"
[S10-2]: https://docs.pydantic.dev/latest/api/config/ "Configuration | Pydantic Docs"
[S10-3]: https://docs.pydantic.dev/latest/api/validate_call/ "Validate Call | Pydantic Docs"
[S10-4]: https://docs.pydantic.dev/latest/migration/ "Migration Guide | Pydantic Docs"


# Pydantic Advanced — 11) JSON Schema and OpenAPI-facing contracts — Pydantic v2 deep dive

Style target: dense, agent-oriented advanced reference. 

Pydantic automatically generates JSON Schema from `BaseModel` classes and `TypeAdapter`-managed arbitrary types; generated schemas are documented as compliant with **JSON Schema Draft 2020-12** and **OpenAPI Specification v3.1.0**. Schema generation is distinct from instance serialization: `model_json_schema()` / `TypeAdapter.json_schema()` return JSON-schema dictionaries, while `model_dump_json()` / `TypeAdapter.dump_json()` serialize actual instances. ([Pydantic Docs][S11-1])

---

## 11.0 Schema-generation mental model

```text
Python annotation graph
  + Field metadata
  + ConfigDict JSON Schema config
  + validators / serializers
  + custom JSON schema hooks
  → pydantic-core schema
  → GenerateJsonSchema
  → JSON Schema dict
  → OpenAPI / docs / SDK generation / validation contract
```

Primary surfaces:

```python
BaseModel.model_json_schema(...)
TypeAdapter(T).json_schema(...)
pydantic.json_schema.models_json_schema(...)
TypeAdapter.json_schemas(...)
```

Use JSON Schema for **contract documentation**, **OpenAPI emission**, **SDK generation**, **form/UI generation**, **tool schemas**, **LLM structured-output schemas**, and **schema regression tests**. Do **not** confuse JSON Schema with runtime validation output or serialized model data. Pydantic explicitly states that schema methods return jsonable dictionaries representing schemas, while dump methods return JSON strings/bytes for instances. ([Pydantic Docs][S11-1])

---

## 11.1 `BaseModel.model_json_schema()`

### Signature

```python
@classmethod
def model_json_schema(
    cls,
    by_alias: bool = True,
    ref_template: str = DEFAULT_REF_TEMPLATE,
    schema_generator: type[GenerateJsonSchema] = GenerateJsonSchema,
    mode: JsonSchemaMode = "validation",
    union_format: Literal["any_of", "primitive_type_array"] = "any_of",
) -> dict[str, Any]: ...
```

Key parameters:

```text
by_alias:
  use field aliases as property names; default True

ref_template:
  template for generated $ref values

schema_generator:
  custom GenerateJsonSchema subclass

mode:
  "validation" | "serialization"

union_format:
  "any_of" | "primitive_type_array"
```

`model_json_schema()` generates a JSON Schema dictionary for the model class; it defaults to alias keys, validation-mode schema, `anyOf` unions, and the default `GenerateJsonSchema` implementation. ([Pydantic Docs][S11-2])

### Basic model schema

```python
from pydantic import BaseModel, Field

class Product(BaseModel):
    sku: str = Field(description="Stock keeping unit.")
    price_cents: int = Field(ge=1)
    tags: list[str] = []

schema = Product.model_json_schema()
```

Expected schema features:

```text
type: object
properties: sku, price_cents, tags
required: fields without defaults
description/title/constraints from Field metadata
array item schemas for list[str]
numeric constraints mapped to JSON Schema keywords
```

Pydantic maps field types and constraints into JSON Schema, including primitive types, arrays, object fields, string formats, numeric bounds, and constraint keywords such as `exclusiveMinimum`, `maximum`, and `multipleOf`. ([Pydantic Docs][S11-1])

---

## 11.2 `TypeAdapter.json_schema()`

### Signature

```python
def json_schema(
    by_alias: bool = True,
    ref_template: str = DEFAULT_REF_TEMPLATE,
    union_format: Literal["any_of", "primitive_type_array"] = "any_of",
    schema_generator: type[GenerateJsonSchema] = GenerateJsonSchema,
    mode: JsonSchemaMode = "validation",
) -> dict[str, Any]: ...
```

Use for top-level types that are not `BaseModel` classes:

```python
from typing import Annotated
from pydantic import Field, TypeAdapter

PositiveIds = Annotated[list[Annotated[int, Field(gt=0)]], Field(min_length=1)]

schema = TypeAdapter(PositiveIds).json_schema()
```

Use cases:

```text
top-level list[T]
top-level dict[K, V]
TypedDict
dataclass
Union
Literal / Enum
Annotated constrained alias
primitive scalar
LLM tool argument schema not represented by a BaseModel
```

`TypeAdapter.json_schema()` generates JSON Schema for an arbitrary adapted type, and Pydantic’s JSON Schema docs describe `TypeAdapter` as the replacement for v1’s deprecated `schema_of`. ([Pydantic Docs][S11-1])

---

## 11.3 Schema vs serialization

```python
from pydantic import BaseModel

class User(BaseModel):
    id: int
    name: str

schema_dict = User.model_json_schema()
instance_json = User(id=1, name="Ada").model_dump_json()
```

Meaning:

```text
model_json_schema():
  schema describing accepted/emitted shapes

model_dump_json():
  serialized JSON data for a concrete instance
```

Pydantic docs explicitly warn not to confuse `model_json_schema()` / `TypeAdapter.json_schema()` with `model_dump_json()` / `TypeAdapter.dump_json()`; the former return JSON-schema dictionaries, the latter serialize instances. ([Pydantic Docs][S11-1])

Agent rule:

```text
Need contract/documentation/tool schema:
  model_json_schema() / TypeAdapter.json_schema()

Need payload:
  model_dump() / model_dump_json() / TypeAdapter.dump_json()
```

---

## 11.4 Validation schema vs serialization schema

### Mode surface

```python
Model.model_json_schema(mode="validation")
Model.model_json_schema(mode="serialization")

TypeAdapter(T).json_schema(mode="validation")
TypeAdapter(T).json_schema(mode="serialization")
```

Modes:

```text
validation:
  schema for accepted input

serialization:
  schema for emitted output
```

Default mode is `"validation"` for both `model_json_schema()` and `TypeAdapter.json_schema()`. The docs show `Decimal` as a mode-sensitive example: validation schema can accept number or string, while serialization schema emits string. ([Pydantic Docs][S11-1])

### Example: `Decimal`

```python
from decimal import Decimal
from pydantic import BaseModel

class Money(BaseModel):
    amount: Decimal = Decimal("12.34")

validation_schema = Money.model_json_schema(mode="validation")
serialization_schema = Money.model_json_schema(mode="serialization")
```

Agent rule:

```text
Request body schema:
  mode="validation"

Response body schema:
  mode="serialization"

Bidirectional docs:
  generate both or document which one is exposed
```

---

## 11.5 JSON Schema Draft 2020-12 and OpenAPI 3.1

Pydantic’s generated schemas are documented as compliant with JSON Schema Draft 2020-12 and OpenAPI 3.1.0. This matters because OpenAPI 3.1 aligns more closely with modern JSON Schema than OpenAPI 3.0 did. ([Pydantic Docs][S11-1])

OpenAPI/FastAPI implication:

```text
FastAPI 0.99.0+:
  OpenAPI 3.1.0
  JSON Schema 2020-12
  JSON Schema examples supported as "examples"
```

FastAPI documents that `json_schema_extra={"examples": [...]}` on Pydantic models is added to generated JSON Schema and used in API docs, and notes that OpenAPI 3.1.0 support made JSON Schema `examples` the preferred standard over the older single `example` field. ([FastAPI][S11-3])

Agent rule:

```text
Modern FastAPI/OpenAPI:
  use examples=[...] / json_schema_extra={"examples": [...]}

Legacy OpenAPI 3.0 clients:
  test generated schema compatibility
  beware 2020-12 keywords and nullability differences
```

---

## 11.6 Field metadata in JSON Schema

```python
from typing import Annotated
from pydantic import BaseModel, EmailStr, Field, SecretStr

class User(BaseModel):
    age: int = Field(description="Age of the user")
    email: Annotated[EmailStr, Field(examples=["[email protected]"])]
    name: str = Field(title="Username")
    password: SecretStr = Field(
        json_schema_extra={
            "title": "Password",
            "description": "Password of the user",
            "examples": ["123456"],
        }
    )
```

Field-level JSON Schema metadata:

```text
title
description
examples
json_schema_extra
field_title_generator
```

Pydantic identifies these field parameters as schema-customization parameters, and shows them appearing directly in generated schema properties. ([Pydantic Docs][S11-1])

Constraints map into schema:

```python
class Snap(BaseModel):
    value: int = Field(gt=30, lt=50)
```

Expected keywords:

```text
exclusiveMinimum: 30
exclusiveMaximum: 50
type: integer
```

Pydantic’s examples show numeric constraints from `Field(gt=..., lt=...)` mapping to JSON Schema validation keywords. ([Pydantic Docs][S11-1])

---

## 11.7 Model-level schema metadata

```python
from pydantic import BaseModel, ConfigDict

class Model(BaseModel):
    model_config = ConfigDict(
        title="PublicUser",
        json_schema_extra={
            "examples": [{"id": 1, "name": "Ada"}],
            "x-owner": "identity",
        },
    )

    id: int
    name: str
```

Model-level JSON Schema config keys:

```text
title
json_schema_extra
json_schema_mode_override
field_title_generator
model_title_generator
json_schema_serialization_defaults_required
```

Pydantic docs list model-level configuration options relevant to JSON Schema generation and show `json_schema_extra` being used on model config to inject model-level examples. ([Pydantic Docs][S11-1])

Agent rule:

```text
Field-specific docs:
  Field(title=..., description=..., examples=...)

Model-wide docs:
  ConfigDict(title=..., json_schema_extra=...)

Project-wide naming:
  model_title_generator / field_title_generator
```

---

## 11.8 `json_schema_extra`

### Dict form

```python
from pydantic import BaseModel, ConfigDict

class Model(BaseModel):
    model_config = ConfigDict(
        json_schema_extra={
            "examples": [{"a": "Foo"}],
            "x-ui-section": "basic",
        }
    )

    a: str
```

### Callable form

```python
from pydantic import BaseModel, Field

def remove_default(schema: dict) -> None:
    schema.pop("default", None)

class Model(BaseModel):
    a: int = Field(default=1, json_schema_extra=remove_default)
```

`json_schema_extra` can be a dictionary that adds schema metadata or a callable that mutates the generated schema. Starting in v2.9, Pydantic merges `json_schema_extra` dictionaries from annotated types; mixing dictionary and callable `json_schema_extra` specifications is not supported. ([Pydantic Docs][S11-1])

Agent rule:

```text
Use dict form for:
  examples
  x-* vendor extensions
  UI metadata
  SDK hints

Use callable form for:
  removing generated defaults
  conditional schema mutation
  migration shims

Do not mix dict and callable schema-extra composition.
```

---

## 11.9 `WithJsonSchema`

```python
from typing import Annotated
from pydantic import BaseModel, WithJsonSchema

MyInt = Annotated[
    int,
    WithJsonSchema({"type": "integer", "examples": [1, 0, -1]}),
]

class Model(BaseModel):
    a: MyInt
```

`WithJsonSchema` overrides the JSON Schema for a type and is preferred over implementing `__get_pydantic_json_schema__()` for simple custom schema overrides because it is simpler and less error-prone. It replaces the whole generated schema for that type, so the override must include required schema keys such as `"type"` when needed. ([Pydantic Docs][S11-1])

Use cases:

```text
third-party type has no useful JSON Schema
schema should differ from runtime type internals
custom format keyword
LLM/tool schema requires simplified shape
UI/schema metadata belongs to reusable type alias
```

Avoid:

```text
fine-tuning validator input schema:
  prefer json_schema_input_type on validators

global schema policy:
  prefer GenerateJsonSchema subclass
```

Pydantic specifically cautions that `WithJsonSchema` is tempting for fields with validators, but recommends `json_schema_input_type` for validator input schema tuning. ([Pydantic Docs][S11-1])

---

## 11.10 `__get_pydantic_json_schema__`

```python
from typing import Any
from pydantic import GetCoreSchemaHandler, GetJsonSchemaHandler, TypeAdapter
from pydantic.json_schema import JsonSchemaValue
from pydantic_core import core_schema as cs

class Person:
    name: str
    age: int

    @classmethod
    def __get_pydantic_core_schema__(
        cls,
        source_type: Any,
        handler: GetCoreSchemaHandler,
    ) -> cs.CoreSchema:
        return cs.typed_dict_schema(
            {
                "name": cs.typed_dict_field(cs.str_schema()),
                "age": cs.typed_dict_field(cs.int_schema()),
            }
        )

    @classmethod
    def __get_pydantic_json_schema__(
        cls,
        core_schema: cs.CoreSchema,
        handler: GetJsonSchemaHandler,
    ) -> JsonSchemaValue:
        json_schema = handler(core_schema)
        json_schema = handler.resolve_ref_schema(json_schema)
        json_schema["examples"] = [{"name": "John Doe", "age": 25}]
        return json_schema

schema = TypeAdapter(Person).json_schema()
```

`__get_pydantic_json_schema__` modifies or overrides generated JSON Schema only; it does not affect core schema, validation, or serialization. Pydantic docs show calling `handler(core_schema)`, resolving referenced schemas with `handler.resolve_ref_schema(...)`, then mutating the result. ([Pydantic Docs][S11-1])

Agent rule:

```text
Use __get_pydantic_json_schema__ when:
  custom type owns its schema contract
  WithJsonSchema is insufficient
  schema needs generated base + mutation
  reusable library type must publish schema behavior

Avoid when:
  field-level json_schema_extra is enough
  WithJsonSchema is enough
  only one model field needs docs metadata
```

---

## 11.11 `__get_pydantic_core_schema__` and schema generation for custom types

```python
from typing import Any
from pydantic import BaseModel, GetCoreSchemaHandler
from pydantic_core import core_schema

class CompressedString:
    @classmethod
    def __get_pydantic_core_schema__(
        cls,
        source: type[Any],
        handler: GetCoreSchemaHandler,
    ) -> core_schema.CoreSchema:
        return core_schema.no_info_after_validator_function(
            cls._validate,
            core_schema.str_schema(),
            serialization=core_schema.plain_serializer_function_ser_schema(
                cls._serialize,
                info_arg=False,
                return_schema=core_schema.str_schema(),
            ),
        )

    @staticmethod
    def _validate(value: str) -> "CompressedString":
        ...

    @staticmethod
    def _serialize(value: "CompressedString") -> str:
        ...
```

Custom types can implement `__get_pydantic_core_schema__` to define validation and serialization behavior, which then feeds JSON Schema generation. Pydantic warns that for most custom types you should not blindly call the handler for the custom type itself, because Pydantic may not know how to generate a schema for it; for `Annotated` metadata wrappers, calling the handler is usually appropriate because there is an inner type schema to modify. ([Pydantic Docs][S11-1])

Agent rule:

```text
Custom type schema levels:
  WithJsonSchema                  simplest schema override
  __get_pydantic_json_schema__    schema-only custom hook
  __get_pydantic_core_schema__    validation/serialization core hook
  GenerateJsonSchema subclass     global generation policy
```

---

## 11.12 `$defs` and `$ref` behavior

```python
class Foo(BaseModel):
    a: int

class Model(BaseModel):
    foo: Foo

schema = Model.model_json_schema()
```

Behavior:

```text
nested submodels:
  emitted under "$defs"

field reference:
  {"$ref": "#/$defs/Foo"}
```

Pydantic adds submodels to the `$defs` JSON Schema attribute and references them according to the spec; optional fields indicate `null` is allowed; `Decimal` is exposed as a string; namedtuple identity is not preserved in JSON Schema because JSON has no namedtuple type. ([Pydantic Docs][S11-1])

Important exception:

```text
Submodel with Field modifications:
  recursively included instead of referenced
```

Pydantic notes that submodels with modifications through `Field`, such as a custom title, description, or default value, are recursively included instead of referenced. ([Pydantic Docs][S11-1])

---

## 11.13 Ref customization: `ref_template`

```python
from pydantic import BaseModel, TypeAdapter

class Foo(BaseModel):
    a: int

class Model(BaseModel):
    a: Foo

schema = TypeAdapter(Model).json_schema(
    ref_template="#/components/schemas/{model}"
)
```

`ref_template` changes generated `$ref` strings, while definitions remain under `$defs`; Pydantic explicitly calls out OpenAPI as a use case for `#/components/schemas/{model}` references. ([Pydantic Docs][S11-1])

Agent rule:

```text
Standalone JSON Schema:
  default ref_template usually fine

OpenAPI component integration:
  ref_template="#/components/schemas/{model}"

Custom schema registry:
  define stable ref_template and snapshot it
```

---

## 11.14 Schema reuse and top-level schema generation

### Multiple models

```python
from pydantic import BaseModel
from pydantic.json_schema import models_json_schema

class Foo(BaseModel):
    a: str | None = None

class Model(BaseModel):
    b: Foo

class Bar(BaseModel):
    c: int

field_schemas, top_level_schema = models_json_schema(
    [(Model, "validation"), (Bar, "validation")],
    title="My Schema",
)
```

`models_json_schema(...)` generates a top-level schema containing definitions for a list of models and related submodels under `$defs`. ([Pydantic Docs][S11-1])

### Multiple `TypeAdapter`s

```python
from pydantic import TypeAdapter

schemas, top = TypeAdapter.json_schemas(
    [
        ("ids", "validation", TypeAdapter(list[int])),
        ("names", "validation", TypeAdapter(list[str])),
    ],
    title="Adapter Schemas",
)
```

`TypeAdapter.json_schemas(...)` generates schemas for multiple adapted types and a second top-level schema containing all referenced definitions, with optional title and description. ([Pydantic Docs][S11-4])

Agent rule:

```text
Single model:
  Model.model_json_schema()

Single arbitrary type:
  TypeAdapter(T).json_schema()

Schema bundle for multiple models:
  models_json_schema(...)

Schema bundle for arbitrary adapted types:
  TypeAdapter.json_schemas(...)
```

---

## 11.15 Union schema generation

### Normal union

```python
from typing import Union
from pydantic import BaseModel

class Cat(BaseModel):
    name: str
    color: str

class Dog(BaseModel):
    name: str
    breed: str

class Wrapper(BaseModel):
    pet: Union[Cat, Dog]
```

Default schema shape:

```text
anyOf:
  - $ref Cat
  - $ref Dog
```

Pydantic’s JSON Schema docs show a `TypeAdapter(Union[Cat, Dog])` generating `$defs` for both models and an `anyOf` array referencing them. ([Pydantic Docs][S11-1])

### `union_format`

```python
schema = Model.model_json_schema(union_format="primitive_type_array")
```

Options:

```text
"any_of":
  default; use anyOf

"primitive_type_array":
  use {"type": ["string", "null", ...]} when all union members are primitive unconstrained schemas
  fallback to anyOf when constraints or non-primitive schemas appear
```

Both `BaseModel.model_json_schema()` and `TypeAdapter.json_schema()` expose `union_format` with these semantics. ([Pydantic Docs][S11-2])

Agent rule:

```text
Default anyOf:
  safest, general, explicit

primitive_type_array:
  compact nullable/primitive schemas
  use only when downstream tooling supports it
```

---

## 11.16 Discriminated unions and OpenAPI discriminator

```python
from typing import Literal
from pydantic import BaseModel, Field

class Cat(BaseModel):
    pet_type: Literal["cat"]
    meows: int

class Dog(BaseModel):
    pet_type: Literal["dog"]
    barks: float

class Model(BaseModel):
    pet: Cat | Dog = Field(discriminator="pet_type")
```

Discriminated unions validate more efficiently by selecting one member of the union based on a discriminator, reduce error proliferation, and cause generated JSON Schema to implement the OpenAPI `discriminator` attribute. ([Pydantic Docs][S11-5])

Agent rule:

```text
OpenAPI polymorphic payload:
  prefer discriminated union

Variant discriminator field:
  required
  Literal values
  stable wire tag
```

---

## 11.17 Callable discriminators and schema implications

```python
from typing import Annotated, Any, Literal
from pydantic import BaseModel, Discriminator, Tag

class ApplePie(BaseModel):
    fruit: Literal["apple"]
    time_to_cook: int

class PumpkinPie(BaseModel):
    filling: Literal["pumpkin"]
    time_to_cook: int

def tag(v: Any) -> str | None:
    if isinstance(v, dict):
        return v.get("fruit", v.get("filling"))
    return getattr(v, "fruit", getattr(v, "filling", None))

Dessert = Annotated[
    Annotated[ApplePie, Tag("apple")] | Annotated[PumpkinPie, Tag("pumpkin")],
    Discriminator(tag),
]
```

Callable discriminators are appropriate when no uniform discriminator field exists. Pydantic warns that callable discriminators should handle both dictionaries and model instances because they are used during validation and serialization; returning `None` raises `union_tag_not_found`. ([Pydantic Docs][S11-5])

Agent rule:

```text
Prefer string discriminator when possible.
Use callable Discriminator only when variant tag extraction is structurally non-uniform.
Callable must be pure, total over dict/model inputs, and side-effect-free.
```

---

## 11.18 Nested discriminated unions

```python
from typing import Annotated, Literal
from pydantic import BaseModel, Field

class BlackCat(BaseModel):
    pet_type: Literal["cat"]
    color: Literal["black"]
    black_name: str

class WhiteCat(BaseModel):
    pet_type: Literal["cat"]
    color: Literal["white"]
    white_name: str

Cat = Annotated[BlackCat | WhiteCat, Field(discriminator="color")]

class Dog(BaseModel):
    pet_type: Literal["dog"]
    name: str

Pet = Annotated[Cat | Dog, Field(discriminator="pet_type")]

class Model(BaseModel):
    pet: Pet
```

Only one discriminator can be set for a field, but multiple discriminator layers can be composed with nested `Annotated` union aliases. ([Pydantic Docs][S11-5])

Use cases:

```text
type + subtype protocols
hierarchical event types
plugin class families
OpenAPI polymorphic schemas with layered tags
```

---

## 11.19 Schema generation for custom types

### Simple schema override

```python
from typing import Annotated
from pydantic import WithJsonSchema

HexString = Annotated[
    str,
    WithJsonSchema(
        {
            "type": "string",
            "pattern": "^[0-9a-f]+$",
            "examples": ["deadbeef"],
        }
    ),
]
```

### Core + JSON schema custom type

```python
class MyType:
    @classmethod
    def __get_pydantic_core_schema__(cls, source, handler):
        ...

    @classmethod
    def __get_pydantic_json_schema__(cls, core_schema, handler):
        ...
```

Best-practice ladder:

```text
Field metadata enough:
  Field(title=..., description=..., examples=...)

Reusable type schema override:
  Annotated[T, WithJsonSchema(...)]

Schema-only library hook:
  __get_pydantic_json_schema__

Validation/serialization + schema:
  __get_pydantic_core_schema__ + optional JSON schema hook

Global schema dialect/policy:
  GenerateJsonSchema subclass
```

Pydantic describes all of these schema-customization mechanisms and notes that `WithJsonSchema` is preferred over `__get_pydantic_json_schema__()` for simpler custom type schema overrides. ([Pydantic Docs][S11-1])

---

## 11.20 Custom global schema generation: `GenerateJsonSchema`

```python
from pydantic import BaseModel
from pydantic.json_schema import GenerateJsonSchema

class MyGenerateJsonSchema(GenerateJsonSchema):
    def generate(self, schema, mode="validation"):
        json_schema = super().generate(schema, mode=mode)
        json_schema["$schema"] = self.schema_dialect
        return json_schema

class MyModel(BaseModel):
    x: int

schema = MyModel.model_json_schema(schema_generator=MyGenerateJsonSchema)
```

`GenerateJsonSchema` translates Pydantic core schema into JSON Schema and is intentionally split into overridable methods; `model_json_schema()` and related methods accept a `schema_generator` subclass when global generation behavior must change. ([Pydantic Docs][S11-1])

Use cases:

```text
add $schema everywhere
omit unsupported fields
custom reference behavior
organization-wide JSON Schema policy
tooling-specific schema dialect adjustments
```

Avoid when:

```text
one field needs metadata
one type needs a schema override
OpenAPI examples only need json_schema_extra
```

---

## 11.21 Schema sorting

Default behavior:

```text
Pydantic recursively sorts JSON Schema keys alphabetically.
Pydantic does not sort values of the properties key, preserving model field order.
```

Custom sorting:

```python
from typing import Optional
from pydantic.json_schema import GenerateJsonSchema, JsonSchemaValue

class NoSortSchema(GenerateJsonSchema):
    def sort(
        self,
        value: JsonSchemaValue,
        parent_key: Optional[str] = None,
    ) -> JsonSchemaValue:
        return value
```

Pydantic documents default recursive sorting and shows overriding `GenerateJsonSchema.sort(...)` to disable or customize sorting. ([Pydantic Docs][S11-1])

Agent rule:

```text
Human-readable schema snapshots:
  default sorting is helpful

Exact field/config insertion order snapshots:
  custom sort method

Public contract tests:
  normalize schemas before snapshotting
```

---

## 11.22 Alias keys in schema

```python
schema_alias = Model.model_json_schema()
schema_names = Model.model_json_schema(by_alias=False)
```

Default schema generation uses aliases as property keys; pass `by_alias=False` to generate schema with Python model property names instead. ([Pydantic Docs][S11-1])

Agent rule:

```text
External OpenAPI / public JSON:
  by_alias=True

Internal docs / Python-facing docs:
  by_alias=False

Never rely on implicit default in long-lived schema contracts:
  pass by_alias explicitly
```

---

## 11.23 FastAPI / OpenAPI compatibility patterns

### Pydantic model examples

```python
from pydantic import BaseModel, ConfigDict

class Item(BaseModel):
    model_config = ConfigDict(
        json_schema_extra={
            "examples": [
                {
                    "name": "Foo",
                    "price": 35.4,
                }
            ]
        }
    )

    name: str
    price: float
```

FastAPI documents that Pydantic model `json_schema_extra` is added to generated JSON Schema and used in API docs; it also recommends `examples` over the older single `example` field in OpenAPI 3.1 contexts. ([FastAPI][S11-3])

### Field examples

```python
from pydantic import BaseModel, Field

class Item(BaseModel):
    name: str = Field(examples=["Foo"])
    price: float = Field(examples=[35.4])
```

### OpenAPI ref template

```python
schema = Item.model_json_schema(
    ref_template="#/components/schemas/{model}",
    mode="serialization",
)
```

### FastAPI policy

```text
Request models:
  mode="validation"

Response models:
  mode="serialization"

Public field names:
  alias_generator=to_camel
  serialize_by_alias=True
  schema by_alias=True

Examples:
  Field(examples=[...])
  model_config.json_schema_extra={"examples": [...]}

Polymorphism:
  use discriminated unions with Literal tags

Avoid:
  arbitrary WithJsonSchema overrides that break OpenAPI clients
  callable discriminator unless necessary
  nonstandard vendor extensions without x-* prefix
```

---

## 11.24 LLM/tool-schema compatibility patterns

```python
from typing import Literal
from pydantic import BaseModel, ConfigDict, Field

class SearchToolArgs(BaseModel):
    model_config = ConfigDict(
        title="SearchToolArgs",
        extra="forbid",
        json_schema_extra={
            "examples": [{"query": "pydantic json schema", "limit": 5}]
        },
    )

    query: str = Field(min_length=1, description="Search query.")
    limit: int = Field(default=10, ge=1, le=20)
    mode: Literal["web", "files"] = "web"

schema = SearchToolArgs.model_json_schema(
    mode="validation",
    by_alias=True,
    ref_template="#/$defs/{model}",
)
```

Agent rule:

```text
Tool input schemas:
  validation mode
  extra="forbid"
  explicit descriptions
  examples
  bounded integers
  Literal/Enum for choices
  avoid ambiguous unions
  avoid unsupported custom formats unless target accepts them
```

---

## 11.25 Schema snapshot / regression testing

```python
import json

def canonical_schema(schema: dict) -> dict:
    # optional project-specific normalization:
    # remove volatile titles, sort lists where semantically orderless, etc.
    return schema

def test_public_schema_snapshot(snapshot):
    schema = SearchToolArgs.model_json_schema(
        mode="validation",
        by_alias=True,
        ref_template="#/components/schemas/{model}",
    )
    snapshot.assert_match(
        json.dumps(canonical_schema(schema), indent=2, sort_keys=True),
        "SearchToolArgs.schema.json",
    )
```

Test what matters:

```text
public property names
required list
nullable/null representation
constraints
examples
discriminator
$ref template
schema mode
alias policy
```

Avoid overfitting to:

```text
non-public title casing
key order without normalization
internal $defs naming for generic/internal models unless contractually exposed
full schema output for every private DTO
```

Pydantic supports schema sorting and custom `$ref` templates, but `$defs` and generated titles are still schema-shape details that should be intentionally pinned only when they are public contract. ([Pydantic Docs][S11-1])

---

## 11.26 Deployment decision matrix

```text
Need                                           Pydantic surface
--------------------------------------------------------------------------------
schema for named model                         BaseModel.model_json_schema()
schema for list/dict/union/TypedDict           TypeAdapter(T).json_schema()
request/input contract                         mode="validation"
response/output contract                       mode="serialization"
OpenAPI component refs                         ref_template="#/components/schemas/{model}"
camelCase public schema                        by_alias=True + alias_generator
Python-facing docs                             by_alias=False
field docs                                     Field(title, description, examples)
model docs                                     ConfigDict(title, json_schema_extra)
vendor/OpenAPI extensions                      json_schema_extra={"x-...": ...}
reusable custom type schema                    WithJsonSchema
advanced custom type schema                    __get_pydantic_json_schema__
validation + serialization custom type         __get_pydantic_core_schema__
schema bundle for many models                  models_json_schema
schema bundle for many TypeAdapters            TypeAdapter.json_schemas
schema order customization                     GenerateJsonSchema.sort
global schema-policy customization             schema_generator=CustomGenerateJsonSchema
normal union schema                            anyOf
compact primitive union schema                 union_format="primitive_type_array"
OpenAPI polymorphism                           discriminated union
hierarchical polymorphism                      nested discriminated unions
```

---

## 11.27 Anti-patterns

### Confusing schema generation with payload serialization

```python
User.model_json_schema()   # schema, not user data
user.model_dump_json()     # user data, not schema
```

Pydantic explicitly distinguishes schema-generation methods from instance serialization methods. ([Pydantic Docs][S11-1])

---

### Publishing validation schema as response schema

```python
ResponseModel.model_json_schema(mode="validation")
```

Better:

```python
ResponseModel.model_json_schema(mode="serialization")
```

Mode matters when validation and serialization shapes differ, such as for `Decimal` and custom serializers. ([Pydantic Docs][S11-1])

---

### Using `WithJsonSchema` to patch validator input shape

```python
Annotated[int, BeforeValidator(parse), WithJsonSchema(...)]
```

Better:

```python
@field_validator("x", mode="before", json_schema_input_type=str | int)
```

Pydantic recommends `json_schema_input_type` rather than `WithJsonSchema` for fields with validators whose accepted input type differs from the annotation. ([Pydantic Docs][S11-1])

---

### Complex global generator for local metadata

```python
class MyGenerateJsonSchema(GenerateJsonSchema):
    ...
```

Better for local metadata:

```python
Field(description="...")
Field(json_schema_extra={"x-ui-widget": "textarea"})
ConfigDict(json_schema_extra={"examples": [...]})
```

Use `GenerateJsonSchema` subclasses only for global generation policy. Pydantic describes field/model customizations as narrower-scope options and custom schema generator subclasses as broader-scope process overrides. ([Pydantic Docs][S11-1])

---

### Untagged polymorphic OpenAPI unions

```python
class Payload(BaseModel):
    item: Cat | Dog
```

Better:

```python
class Payload(BaseModel):
    item: Cat | Dog = Field(discriminator="kind")
```

Discriminated unions produce more efficient validation, simpler errors, and OpenAPI discriminator schema. ([Pydantic Docs][S11-5])

---

## 11.28 Test matrix

### Basic schema shape

```python
def test_schema_required_and_constraints():
    class M(BaseModel):
        x: int = Field(gt=0)
        y: str | None = None

    schema = M.model_json_schema()

    assert schema["properties"]["x"]["exclusiveMinimum"] == 0
    assert "x" in schema["required"]
    assert "y" not in schema.get("required", [])
```

### Validation vs serialization schema

```python
from decimal import Decimal

def test_decimal_schema_modes():
    class M(BaseModel):
        amount: Decimal

    validation = M.model_json_schema(mode="validation")
    serialization = M.model_json_schema(mode="serialization")

    assert validation != serialization
    assert serialization["properties"]["amount"]["type"] == "string"
```

### Alias policy

```python
def test_schema_alias_policy():
    class M(BaseModel):
        user_id: int = Field(serialization_alias="userId", validation_alias="userId")

    assert "userId" in M.model_json_schema(by_alias=True)["properties"]
    assert "user_id" in M.model_json_schema(by_alias=False)["properties"]
```

### Ref template

```python
def test_openapi_ref_template():
    class Foo(BaseModel):
        a: int

    class M(BaseModel):
        foo: Foo

    schema = M.model_json_schema(ref_template="#/components/schemas/{model}")

    assert schema["properties"]["foo"]["$ref"] == "#/components/schemas/Foo"
```

### Discriminated union schema

```python
from typing import Literal

def test_discriminated_union_schema():
    class Cat(BaseModel):
        kind: Literal["cat"]
        meows: int

    class Dog(BaseModel):
        kind: Literal["dog"]
        barks: int

    class PetBox(BaseModel):
        pet: Cat | Dog = Field(discriminator="kind")

    pet_schema = PetBox.model_json_schema()["properties"]["pet"]
    assert "discriminator" in pet_schema
```

### `json_schema_extra`

```python
def test_json_schema_extra_examples():
    class M(BaseModel):
        model_config = ConfigDict(
            json_schema_extra={"examples": [{"x": 1}]}
        )
        x: int

    assert M.model_json_schema()["examples"] == [{"x": 1}]
```

### TypeAdapter schema

```python
def test_type_adapter_schema():
    schema = TypeAdapter(list[int]).json_schema()
    assert schema == {"items": {"type": "integer"}, "type": "array"}
```

---

## 11.29 Agent checklist

```text
[ ] Use model_json_schema() for BaseModel schema contracts.
[ ] Use TypeAdapter(T).json_schema() for arbitrary top-level types.
[ ] Use mode="validation" for input/request/tool-argument schemas.
[ ] Use mode="serialization" for output/response schemas.
[ ] Pass by_alias explicitly for public schemas.
[ ] Use ref_template="#/components/schemas/{model}" for OpenAPI component refs.
[ ] Use Field(title, description, examples) for field docs.
[ ] Use ConfigDict(json_schema_extra=...) for model-level examples/extensions.
[ ] Use examples=[...] rather than deprecated single example for OpenAPI 3.1-style JSON Schema.
[ ] Use WithJsonSchema for simple reusable type schema override.
[ ] Use __get_pydantic_json_schema__ only for advanced custom type schema behavior.
[ ] Use __get_pydantic_core_schema__ only when validation/serialization behavior itself must change.
[ ] Use discriminated unions for OpenAPI polymorphism.
[ ] Avoid ambiguous untagged unions in public schemas.
[ ] Snapshot public schemas with normalization.
[ ] Test required/nullable/default/alias/discriminator/ref behavior.
[ ] Do not depend on private core-schema internals.
[ ] Do not publish validation schema as response schema without checking serialization differences.
[ ] Keep schema customization local unless global generation policy is required.
```

---

## 11.30 Value case

```text
JSON Schema value =
  one annotation graph drives validation, serialization, and contract generation
  OpenAPI 3.1-compatible schema output
  request/response schema mode separation
  field/model documentation metadata
  schema reuse through $defs / $ref
  top-level arbitrary type schemas through TypeAdapter
  custom type schema support
  discriminated polymorphism for OpenAPI
  SDK/client/docs/tooling integration
  LLM/tool schema generation
  schema regression testing
```

Operationally: treat JSON Schema as a **public contract artifact**. Generate the correct mode for the boundary, control aliases and refs explicitly, prefer local field/model metadata over global generator overrides, use discriminated unions for polymorphism, and snapshot only the schema details that downstream consumers actually rely on.

[S11-1]: https://docs.pydantic.dev/latest/concepts/json_schema/ "JSON Schema | Pydantic Docs"
[S11-2]: https://docs.pydantic.dev/latest/api/base_model/ "BaseModel | Pydantic Docs"
[S11-3]: https://fastapi.tiangolo.com/tutorial/schema-extra-example/ "Declare Request Example Data - FastAPI"
[S11-4]: https://docs.pydantic.dev/latest/api/type_adapter/ "TypeAdapter | Pydantic Docs"
[S11-5]: https://docs.pydantic.dev/latest/concepts/unions/ "Unions | Pydantic Docs"


# Pydantic Advanced — 12) `TypeAdapter`: validation without `BaseModel`

Style target: dense, sectioned, LLM-agent-oriented technical reference. 

`TypeAdapter` provides validation, serialization, and JSON Schema generation for arbitrary Pydantic-compatible Python types without requiring a `BaseModel` subclass. It exposes selected `BaseModel`-like capabilities for types that do not have model methods, including dataclasses, primitive types, collections, unions, `TypedDict`s, and constrained `Annotated` aliases. `TypeAdapter` instances are **not types** and cannot be used as field annotations. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

---

## 12.0 Mental model

```text
type annotation
  → TypeAdapter(type)
  → pydantic-core schema validator / serializer
  → validate_python / validate_json / validate_strings
  → dump_python / dump_json
  → json_schema
```

Use `TypeAdapter` when the thing you need to validate is a **type**, not a named model object:

```python
from pydantic import TypeAdapter

adapter = TypeAdapter(list[int])
value = adapter.validate_python(["1", 2, 3])
assert value == [1, 2, 3]
```

Pydantic’s type-adapter docs describe it as similar to `BaseModel.model_validate(...)` but for arbitrary Pydantic-compatible types, especially useful when the desired target type is not a direct subclass of `BaseModel`. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/concepts/type_adapter/))

---

## 12.1 When to prefer `TypeAdapter` over a one-field model

### Prefer `TypeAdapter`

```text
top-level list[T]
top-level dict[K, V]
top-level tuple[...]
top-level union
primitive scalar
Annotated constrained alias
TypedDict
dataclass
NamedTuple
Literal / Enum / constrained primitive
temporary validation boundary
performance-sensitive parser reused in loop
```

Example:

```python
from typing import Annotated
from pydantic import Field, TypeAdapter

PositiveIds = Annotated[
    list[Annotated[int, Field(gt=0)]],
    Field(min_length=1),
]

IDS = TypeAdapter(PositiveIds)

assert IDS.validate_python(["1", 2, 3]) == [1, 2, 3]
```

### Prefer `BaseModel`

```text
named record object
domain/API DTO with methods
multiple named fields
computed fields
field/model validators tied to object identity
model_config inheritance
model_copy / model_fields / model_fields_set
model_dump_json returning str
rich API docs around class object
public contract should be importable as class
```

One-field model anti-pattern:

```python
class IdsModel(BaseModel):
    ids: list[int]

IdsModel.model_validate({"ids": ["1", "2"]}).ids
```

Better:

```python
IDS = TypeAdapter(list[int])
IDS.validate_python(["1", "2"])
```

Value:

```text
no artificial wrapper key
less schema noise
direct top-level JSON array support
better conceptual match
```

---

## 12.2 Creating adapters

## 12.2.1 Collection adapter

```python
from pydantic import TypeAdapter

int_list_adapter = TypeAdapter(list[int])

assert int_list_adapter.validate_python(["1", 2]) == [1, 2]
assert int_list_adapter.validate_json(b'["1", 2]') == [1, 2]
```

Use when external payload is a top-level array, not an object.

---

## 12.2.2 Constrained `Annotated` adapter

```python
from typing import Annotated
from pydantic import Field, TypeAdapter

PositiveInt = Annotated[int, Field(gt=0)]

positive_int_adapter = TypeAdapter(PositiveInt)

assert positive_int_adapter.validate_python("3") == 3
```

Use for reusable domain scalar validation without creating a model class.

---

## 12.2.3 `TypedDict` adapter

```python
from typing_extensions import TypedDict
from pydantic import TypeAdapter

class UserDict(TypedDict):
    id: int
    name: str

USER_DICT = TypeAdapter(UserDict)

value = USER_DICT.validate_python({"id": "1", "name": "Ada"})
assert value == {"id": 1, "name": "Ada"}
assert isinstance(value, dict)
```

Use when validated output must remain a plain dictionary. Pydantic’s supported-type system allows arbitrary Pydantic-compatible field types, and `TypeAdapter` is the direct way to validate such non-model types. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/concepts/type_adapter/))

---

## 12.2.4 Dataclass adapter

```python
from dataclasses import dataclass
from pydantic import TypeAdapter

@dataclass
class User:
    id: int
    name: str

USER = TypeAdapter(User)

u = USER.validate_python({"id": "1", "name": "Ada"})
assert u == User(id=1, name="Ada")
```

When a standard-library dataclass is used within a Pydantic model, a Pydantic dataclass, or a `TypeAdapter`, Pydantic applies validation. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/concepts/dataclasses/))

---

## 12.2.5 Union adapter

```python
from typing import Literal
from pydantic import BaseModel, Field, TypeAdapter

class Cat(BaseModel):
    kind: Literal["cat"]
    meows: int

class Dog(BaseModel):
    kind: Literal["dog"]
    barks: int

Pet = Cat | Dog

PET = TypeAdapter(Pet)

assert isinstance(PET.validate_python({"kind": "dog", "barks": "3"}), Dog)
```

Discriminated version:

```python
from typing import Annotated

Pet = Annotated[Cat | Dog, Field(discriminator="kind")]
PET = TypeAdapter(Pet)
```

Use discriminated unions for predictable branch selection, cleaner errors, and better schema for OpenAPI-style polymorphism.

---

## 12.2.6 Explicit variable annotation for type checkers

```python
from typing import Union
from pydantic import TypeAdapter

ta: TypeAdapter[Union[str, int]] = TypeAdapter(Union[str, int])  # type: ignore[arg-type]
```

Pydantic notes that depending on the adapted type, mypy may complain when instantiating `TypeAdapter`; explicit variable annotation can be used as a workaround. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

---

## 12.3 Constructor parameters and attributes

### Constructor

```python
TypeAdapter(
    type,
    *,
    config: ConfigDict | None = None,
    _parent_depth: int = 2,
    module: str | None = None,
)
```

Important constructor rules:

```text
type:
  Pydantic-compatible type annotation to adapt

config:
  ConfigDict for adapted type, when allowed

config restriction:
  cannot provide config when adapted type already owns non-overridable config
  examples: BaseModel, TypedDict, dataclass
  Pydantic raises type-adapter-config-unused

_parent_depth:
  private-ish namespace lookup control for forward refs
  discourage use unless comfortable with potential future change

module:
  module passed to plugin when provided
```

Pydantic’s API docs state that `TypeAdapter` takes a type and optional `ConfigDict`, but configuration cannot be provided if the adapted type has its own config that cannot be overridden, such as `BaseModel`, `TypedDict`, or dataclass; `_parent_depth` is explicitly marked private-ish and may change. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

### Attributes

```text
core_schema:
  core schema for adapted type

validator:
  schema validator

serializer:
  schema serializer

pydantic_complete:
  whether schema was successfully built
```

Pydantic exposes these attributes on `TypeAdapter`; they are mostly diagnostics/infrastructure surfaces, not normal application APIs. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

Agent rule:

```text
Application code:
  use validate_* / dump_* / json_schema

Infrastructure/custom tooling:
  may inspect core_schema / validator / serializer

Avoid:
  mutating core_schema
  depending on exact core_schema dict shape
```

---

## 12.4 `validate_python(...)`

### Signature

```python
def validate_python(
    object: Any,
    strict: bool | None = None,
    extra: ExtraValues | None = None,
    from_attributes: bool | None = None,
    context: Any | None = None,
    experimental_allow_partial: bool | Literal["off", "on", "trailing-strings"] = False,
    by_alias: bool | None = None,
    by_name: bool | None = None,
) -> T: ...
```

`validate_python()` validates a Python object against the adapted type and returns the validated value; it supports strictness, extra-field behavior, attribute extraction, validation context, experimental partial validation, and alias/name controls. When using `TypeAdapter` with a Pydantic dataclass, `from_attributes` is not supported. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

### Example: Python list

```python
IDS = TypeAdapter(list[int])

assert IDS.validate_python(["1", 2, 3]) == [1, 2, 3]
```

### Example: strict mode

```python
from pydantic import ValidationError

try:
    IDS.validate_python(["1"], strict=True)
except ValidationError as exc:
    ...
```

### Example: context-aware validator

```python
from typing import Annotated
from pydantic import AfterValidator, TypeAdapter, ValidationInfo

def bounded(v: int, info: ValidationInfo) -> int:
    max_value = (info.context or {}).get("max", 100) if isinstance(info.context, dict) else 100
    if v > max_value:
        raise ValueError("too large")
    return v

BoundedInt = Annotated[int, AfterValidator(bounded)]
BOUND = TypeAdapter(BoundedInt)

assert BOUND.validate_python("5", context={"max": 10}) == 5
```

---

## 12.5 `validate_json(...)`

### Signature

```python
def validate_json(
    data: str | bytes | bytearray,
    strict: bool | None = None,
    extra: ExtraValues | None = None,
    context: Any | None = None,
    experimental_allow_partial: bool | Literal["off", "on", "trailing-strings"] = False,
    by_alias: bool | None = None,
    by_name: bool | None = None,
) -> T: ...
```

`validate_json()` validates JSON data supplied as `str`, `bytes`, or `bytearray` and returns the adapted value. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

### Example: top-level JSON array

```python
IDS = TypeAdapter(list[int])

assert IDS.validate_json(b'["1", 2, 3]') == [1, 2, 3]
```

### Example: top-level union JSON

```python
VALUE = TypeAdapter(int | str)

assert VALUE.validate_json(b'"x"') == "x"
assert VALUE.validate_json(b'123') == 123
```

Deployment rule:

```text
raw JSON bytes/string:
  validate_json

already parsed Python object:
  validate_python

top-level JSON array:
  TypeAdapter(list[T]).validate_json(raw)
```

---

## 12.6 `validate_strings(...)`

### Signature

```python
def validate_strings(
    obj: Any,
    strict: bool | None = None,
    extra: ExtraValues | None = None,
    context: Any | None = None,
    experimental_allow_partial: bool | Literal["off", "on", "trailing-strings"] = False,
    by_alias: bool | None = None,
    by_name: bool | None = None,
) -> T: ...
```

`validate_strings()` validates an object containing string data against the adapted type and returns the validated value. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

### Example: stringly typed dict

```python
from typing_extensions import TypedDict

class Query(TypedDict):
    limit: int
    include_archived: bool

QUERY = TypeAdapter(Query)

q = QUERY.validate_strings({
    "limit": "25",
    "include_archived": "false",
})

assert q == {"limit": 25, "include_archived": False}
```

Use for:

```text
query params
headers
form fields
CSV rows
CLI maps
env-like dictionaries
```

---

## 12.7 `dump_python(...)`

### Signature

```python
def dump_python(
    instance: T,
    mode: Literal["json", "python"] = "python",
    include: IncEx | None = None,
    exclude: IncEx | None = None,
    by_alias: bool | None = None,
    exclude_unset: bool = False,
    exclude_defaults: bool = False,
    exclude_none: bool = False,
    exclude_computed_fields: bool = False,
    round_trip: bool = False,
    warnings: bool | Literal["none", "warn", "error"] = True,
    fallback: Callable[[Any], Any] | None = None,
    serialize_as_any: bool = False,
    polymorphic_serialization: bool | None = None,
    context: Any | None = None,
) -> Any: ...
```

`dump_python()` serializes an instance of the adapted type to a Python object. It supports Python/JSON modes, include/exclude, aliases, unset/default/none/computed exclusion, round-trip output, serialization error handling, fallback serialization, duck-typing behavior, polymorphic serialization, and serializer context. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

### Example: Python vs JSON mode

```python
from datetime import datetime

EVENT = TypeAdapter(dict[str, datetime])
value = {"created_at": datetime(2032, 6, 1, 12, 13, 14)}

python_dump = EVENT.dump_python(value)
json_dump = EVENT.dump_python(value, mode="json")

assert isinstance(python_dump["created_at"], datetime)
assert json_dump["created_at"] == "2032-06-01T12:13:14"
```

Pydantic serialization docs state that Python mode may preserve non-JSON-native objects, while JSON mode converts values to JSON-compatible types. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/concepts/serialization/))

---

## 12.8 `dump_json(...)`

### Signature

```python
def dump_json(
    instance: T,
    indent: int | None = None,
    ensure_ascii: bool = False,
    include: IncEx | None = None,
    exclude: IncEx | None = None,
    by_alias: bool | None = None,
    exclude_unset: bool = False,
    exclude_defaults: bool = False,
    exclude_none: bool = False,
    exclude_computed_fields: bool = False,
    round_trip: bool = False,
    warnings: bool | Literal["none", "warn", "error"] = True,
    fallback: Callable[[Any], Any] | None = None,
    serialize_as_any: bool = False,
    polymorphic_serialization: bool | None = None,
    context: Any | None = None,
) -> bytes: ...
```

`dump_json()` serializes an adapted value to JSON and returns **bytes**. This differs from `BaseModel.model_dump_json()`, which returns `str`. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

### Example

```python
IDS = TypeAdapter(list[int])

assert IDS.dump_json([1, 2, 3]) == b"[1,2,3]"
assert IDS.dump_json([1, 2, 3], indent=2).decode()
```

Agent rule:

```text
Need bytes:
  TypeAdapter.dump_json(...)

Need str:
  TypeAdapter.dump_json(...).decode()
```

---

## 12.9 `json_schema(...)`

### Signature

```python
def json_schema(
    by_alias: bool = True,
    ref_template: str = DEFAULT_REF_TEMPLATE,
    union_format: Literal["any_of", "primitive_type_array"] = "any_of",
    schema_generator: type[GenerateJsonSchema] = GenerateJsonSchema,
    mode: JsonSchemaMode = "validation",
) -> dict[str, Any]: ...
```

`json_schema()` generates JSON Schema for the adapted type. It supports alias property names, custom `$ref` template, union schema formatting, custom schema generator, and validation/serialization schema modes. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

### Example

```python
schema = TypeAdapter(list[int]).json_schema()
assert schema == {"items": {"type": "integer"}, "type": "array"}
```

### OpenAPI component refs

```python
schema = TypeAdapter(list[User]).json_schema(
    ref_template="#/components/schemas/{model}",
    mode="serialization",
)
```

Agent rule:

```text
Use json_schema(mode="validation"):
  input/tool/request contracts

Use json_schema(mode="serialization"):
  output/response contracts

Use ref_template="#/components/schemas/{model}":
  OpenAPI integration
```

---

## 12.10 Multiple schemas: `TypeAdapter.json_schemas(...)`

```python
from pydantic import TypeAdapter

schemas, top_level = TypeAdapter.json_schemas(
    [
        ("ids", "validation", TypeAdapter(list[int])),
        ("names", "validation", TypeAdapter(list[str])),
    ],
    title="Adapter Schemas",
)
```

`TypeAdapter.json_schemas(...)` generates schemas for multiple adapted types and a top-level schema containing shared definitions; it is useful when generating a schema bundle for several non-model top-level contracts. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

---

## 12.11 Adapter-level config

### Allowed config for plain/adapted types

```python
from pydantic import ConfigDict, TypeAdapter

STR_LIST = TypeAdapter(
    list[str],
    config=ConfigDict(coerce_numbers_to_str=True),
)

assert STR_LIST.validate_python([1, 2, "x"]) == ["1", "2", "x"]
```

### Strict adapter

```python
STRICT_IDS = TypeAdapter(
    list[int],
    config=ConfigDict(strict=True),
)
```

### Config restriction

```python
from pydantic import BaseModel, TypeAdapter, ConfigDict

class User(BaseModel):
    id: int

# Invalid: BaseModel owns its config.
TypeAdapter(User, config=ConfigDict(strict=True))  # raises type-adapter-config-unused
```

Pydantic states that `TypeAdapter(config=...)` cannot be used when the adapted type has its own config that cannot be overridden, such as `BaseModel`, `TypedDict`, or dataclass; in that case Pydantic raises `type-adapter-config-unused`. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

Agent rule:

```text
Use adapter-level config for:
  primitive / list / dict / union / Annotated aliases

Do not use adapter-level config for:
  BaseModel
  TypedDict
  dataclass

Configure those types where they are declared.
```

---

## 12.12 Deferred schema building

```python
from pydantic import ConfigDict, TypeAdapter

ta = TypeAdapter("MyInt", config=ConfigDict(defer_build=True))

# later, after forward reference exists
MyInt = int

ta.rebuild()
assert ta.validate_python(1) == 1
```

When a `TypeAdapter` is initialized, Pydantic normally analyzes the type and builds a pydantic-core schema. With `ConfigDict(defer_build=True)`, schema building is deferred until first validation/serialization or until `rebuild()` is called. This is useful for forward references and expensive schema builds. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/concepts/type_adapter/))

Use cases:

```text
forward references
large recursive type aliases
rarely used adapters
serverless/CLI cold-start reduction
manually controlled initialization order
```

---

## 12.13 `rebuild(...)`

### Signature

```python
def rebuild(
    force: bool = False,
    raise_errors: bool = True,
    _parent_namespace_depth: int = 2,
    _types_namespace: MappingNamespace | None = None,
) -> bool | None: ...
```

Semantics:

```text
None:
  schema already complete; no rebuild required

True:
  rebuild required and successful

False:
  rebuild required but failed with raise_errors=False
```

`rebuild()` tries to rebuild the pydantic-core schema for the adapter’s type and may be necessary when a forward reference could not be resolved during initial schema build and automatic rebuilding fails; `_types_namespace` can supply an explicit namespace instead of parent-frame lookup. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

### Explicit namespace

```python
ta = TypeAdapter("User", config=ConfigDict(defer_build=True))

class User(BaseModel):
    id: int

ta.rebuild(_types_namespace={"User": User})
```

Agent rule:

```text
If forward refs fail:
  ensure referenced type is defined/imported
  call ta.rebuild()
  pass _types_namespace for dynamic/module-bound cases
```

---

## 12.14 Performance: reuse adapters

Bad:

```python
def parse_ids(raw: object) -> list[int]:
    return TypeAdapter(list[int]).validate_python(raw)
```

Good:

```python
IDS = TypeAdapter(list[int])

def parse_ids(raw: object) -> list[int]:
    return IDS.validate_python(raw)
```

Reason:

```text
TypeAdapter construction:
  analyzes type
  builds core schema
  builds validator
  builds serializer
  non-trivial overhead

Reuse:
  amortizes schema build
  critical in loops/hot paths
```

Pydantic explicitly states that creating a `TypeAdapter` analyzes the provided type and converts it into a pydantic-core schema, which has non-trivial overhead; create an adapter once and reuse it in loops or performance-critical code. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/concepts/type_adapter/))

Deployment rule:

```text
Module-level adapters:
  good for hot paths

Function-local adapter construction:
  acceptable for rare one-off validation
  bad in loops

Serverless cold start:
  either module-level critical adapters
  or defer_build=True for rare adapters
```

---

## 12.15 Primitives and custom aliases

### Primitive

```python
INT = TypeAdapter(int)

assert INT.validate_python("1") == 1
assert INT.validate_json(b'"1"') == 1
```

### Strict primitive alias

```python
from typing import Annotated
from pydantic import Strict

StrictPositiveInt = Annotated[int, Strict(), Field(gt=0)]
STRICT_POSITIVE_INT = TypeAdapter(StrictPositiveInt)
```

### Validator alias

```python
from pydantic import AfterValidator

def is_even(v: int) -> int:
    if v % 2:
        raise ValueError("must be even")
    return v

EvenInt = Annotated[int, AfterValidator(is_even)]
EVEN = TypeAdapter(EvenInt)
```

Agent rule:

```text
Use TypeAdapter for scalar domain aliases when:
  no model field name exists
  validator should be reused outside models
  API receives scalar top-level JSON
```

---

## 12.16 Collections

```python
IDS = TypeAdapter(list[int])
MATRIX = TypeAdapter(list[tuple[int, int]])
TAGS = TypeAdapter(set[str])
COUNTS = TypeAdapter(dict[str, int])
```

Examples:

```python
assert IDS.validate_python(["1", 2]) == [1, 2]
assert MATRIX.validate_json(b'[[1, 2], ["3", "4"]]') == [(1, 2), (3, 4)]
assert TAGS.validate_python(["a", "a", "b"]) == {"a", "b"}
assert COUNTS.validate_python({"a": "1"}) == {"a": 1}
```

Agent rule:

```text
Top-level JSON array:
  TypeAdapter(list[T])

Top-level JSON object with arbitrary keys:
  TypeAdapter(dict[str, T])

Named object with fixed fields:
  BaseModel or TypedDict
```

---

## 12.17 Dataclasses

```python
from dataclasses import dataclass
from pydantic import TypeAdapter

@dataclass
class Point:
    x: int
    y: int

POINT = TypeAdapter(Point)

p = POINT.validate_python({"x": "1", "y": "2"})
assert p == Point(1, 2)

assert POINT.dump_python(p) == {"x": 1, "y": 2}
assert POINT.dump_json(p) == b'{"x":1,"y":2}'
schema = POINT.json_schema()
```

Pydantic docs note that to perform validation or generate JSON Schema for a Pydantic dataclass, wrap the dataclass with `TypeAdapter` and use its methods; stdlib dataclasses also receive validation when used through TypeAdapter. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/concepts/dataclasses/))

Agent rule:

```text
Dataclass object needed:
  TypeAdapter(dataclass_type)

BaseModel methods needed on instance:
  BaseModel, not dataclass
```

---

## 12.18 TypedDicts

```python
from typing_extensions import NotRequired, TypedDict

class UserPayload(TypedDict):
    id: int
    name: str
    nickname: NotRequired[str]

USER_PAYLOAD = TypeAdapter(UserPayload)

data = USER_PAYLOAD.validate_python({"id": "1", "name": "Ada"})
assert data == {"id": 1, "name": "Ada"}
```

Use cases:

```text
plain dict output required
no object methods needed
lightweight request/response shape
interop with typed dict-oriented code
```

Do not wrap `TypedDict` in a one-field `BaseModel` unless an envelope key is real.

---

## 12.19 Custom aliases with serializers

```python
from decimal import Decimal
from typing import Annotated
from pydantic import PlainSerializer, TypeAdapter

Money = Annotated[
    Decimal,
    Field(ge=Decimal("0"), max_digits=12, decimal_places=2),
    PlainSerializer(lambda v: format(v, "f"), return_type=str, when_used="json"),
]

MONEY = TypeAdapter(Money)

value = MONEY.validate_python("12.30")
assert MONEY.dump_python(value) == Decimal("12.30")
assert MONEY.dump_python(value, mode="json") == "12.30"
assert MONEY.dump_json(value) == b'"12.30"'
```

Value case:

```text
single reusable scalar contract
validation + serialization + schema
no wrapper model
```

---

## 12.20 Alias/name controls with adapted model-like types

```python
from pydantic import BaseModel, Field, TypeAdapter

class User(BaseModel):
    id: int = Field(validation_alias="userId", serialization_alias="userId")

USER = TypeAdapter(User)

u = USER.validate_python({"userId": "1"}, by_alias=True)
assert USER.dump_python(u, by_alias=True) == {"userId": 1}
```

Adapter validation and serialization methods accept alias/name parameters where relevant; `validate_python`, `validate_json`, and `validate_strings` expose `by_alias`/`by_name`, while `dump_python` and `dump_json` expose `by_alias`. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

---

## 12.21 Partial validation caveat

```python
TypeAdapter(list[int]).validate_json(
    b"[1, 2",
    experimental_allow_partial=True,
)
```

`experimental_allow_partial` exists on `validate_python`, `validate_json`, and `validate_strings`, with modes `False`/`"off"`, `True`/`"on"`, and `"trailing-strings"`; it is experimental and intended for partial validation such as processing streams. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

Agent rule:

```text
Do not use experimental_allow_partial in stable production contracts unless:
  version is pinned
  behavior is tested
  partial/streaming validation is truly required
```

---

## 12.22 Deployment patterns

### Top-level API array

```python
class Item(BaseModel):
    id: int
    name: str

ITEM_LIST = TypeAdapter(list[Item])

def parse_items(raw_json: bytes) -> list[Item]:
    return ITEM_LIST.validate_json(raw_json)

def dump_items(items: list[Item]) -> bytes:
    return ITEM_LIST.dump_json(items)
```

Pydantic’s docs show `TypeAdapter(list[Item]).validate_python(...)` parsing a list of model dictionaries into a list of model instances, which is the canonical “list of models without wrapper model” use case. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/concepts/type_adapter/))

---

### CSV row parser

```python
from typing_extensions import TypedDict

class Row(TypedDict):
    id: int
    amount: Decimal
    active: bool

ROW = TypeAdapter(Row)

def parse_row(row: dict[str, str]) -> Row:
    return ROW.validate_strings(row)
```

---

### LLM scalar/tool output

```python
from typing import Literal

Action = Literal["search", "summarize", "answer"]
ACTION = TypeAdapter(Action)

def parse_action(raw_json: bytes) -> Action:
    return ACTION.validate_json(raw_json)
```

---

### Config fragment

```python
ConfigFragment = dict[str, str | int | bool]
CONFIG_FRAGMENT = TypeAdapter(ConfigFragment)

fragment = CONFIG_FRAGMENT.validate_python({"debug": "true", "workers": "4"})
```

---

### Reusable service-level validators

```python
PositivePort = Annotated[int, Field(ge=1, le=65535)]
PORT = TypeAdapter(PositivePort)

def parse_port(value: object) -> int:
    return PORT.validate_python(value)
```

---

## 12.23 Anti-patterns

### Anti-pattern: TypeAdapter as field annotation

```python
class M(BaseModel):
    x: TypeAdapter(list[int])  # wrong
```

Pydantic explicitly states that `TypeAdapter` instances are not types and cannot be used as annotations for fields. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

Correct:

```python
class M(BaseModel):
    x: list[int]
```

or:

```python
X = TypeAdapter(list[int])
```

---

### Anti-pattern: rebuilding adapter every call

```python
def parse(raw):
    return TypeAdapter(list[int]).validate_python(raw)
```

Correct:

```python
LIST_INT = TypeAdapter(list[int])

def parse(raw):
    return LIST_INT.validate_python(raw)
```

Pydantic recommends creating a TypeAdapter once and reusing it in loops or performance-critical code because construction has non-trivial schema-build overhead. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/concepts/type_adapter/))

---

### Anti-pattern: one-field model for top-level array

```python
class Items(BaseModel):
    items: list[Item]
```

If the wire payload is `[{...}, {...}]`, use:

```python
ITEMS = TypeAdapter(list[Item])
```

Use a model only when the wire payload is actually `{"items": [...]}`.

---

### Anti-pattern: adapter config for config-owning types

```python
TypeAdapter(UserModel, config=ConfigDict(strict=True))
```

Configure `UserModel` itself. `TypeAdapter` rejects config overrides for types that own config, including `BaseModel`, `TypedDict`, and dataclass types. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

---

### Anti-pattern: assuming `dump_json()` returns `str`

```python
json_text: str = TypeAdapter(list[int]).dump_json([1, 2])
```

Correct:

```python
json_bytes: bytes = TypeAdapter(list[int]).dump_json([1, 2])
json_text: str = json_bytes.decode()
```

`TypeAdapter.dump_json()` returns bytes. ([docs.pydantic.dev](https://docs.pydantic.dev/latest/api/type_adapter/))

---

## 12.24 Test matrix

### Validate top-level list

```python
def test_type_adapter_list():
    ta = TypeAdapter(list[int])
    assert ta.validate_python(["1", 2]) == [1, 2]
    assert ta.validate_json(b'["1", 2]') == [1, 2]
```

### Constrained alias

```python
def test_constrained_alias():
    PositiveInt = Annotated[int, Field(gt=0)]
    ta = TypeAdapter(PositiveInt)

    assert ta.validate_python("1") == 1

    with pytest.raises(ValidationError):
        ta.validate_python(0)
```

### TypedDict strings mode

```python
def test_typed_dict_validate_strings():
    class Query(TypedDict):
        limit: int
        active: bool

    ta = TypeAdapter(Query)
    assert ta.validate_strings({"limit": "10", "active": "false"}) == {
        "limit": 10,
        "active": False,
    }
```

### Dataclass adapter

```python
def test_dataclass_adapter():
    @dataclass
    class Point:
        x: int
        y: int

    ta = TypeAdapter(Point)
    assert ta.validate_python({"x": "1", "y": "2"}) == Point(1, 2)
```

### Dump JSON returns bytes

```python
def test_dump_json_returns_bytes():
    ta = TypeAdapter(list[int])
    assert ta.dump_json([1, 2, 3]) == b"[1,2,3]"
```

### JSON Schema

```python
def test_json_schema():
    ta = TypeAdapter(list[int])
    assert ta.json_schema() == {"items": {"type": "integer"}, "type": "array"}
```

### Reuse adapter performance guard

```python
IDS = TypeAdapter(list[int])

def test_reused_adapter():
    assert IDS.validate_python(["1"]) == [1]
```

### Rebuild forward ref

```python
def test_rebuild_forward_ref():
    ta = TypeAdapter("MyInt", config=ConfigDict(defer_build=True))
    MyInt = int
    assert ta.rebuild(_types_namespace={"MyInt": MyInt}) is True
    assert ta.validate_python("1") == 1
```

---

## 12.25 Deployment decision matrix

```text
Need                                          Preferred API
--------------------------------------------------------------------------------
top-level JSON array                           TypeAdapter(list[T]).validate_json(...)
top-level JSON object with arbitrary keys      TypeAdapter(dict[str, T]).validate_json(...)
plain dict with fixed keys                     TypeAdapter(TypedDictType)
dataclass validation                           TypeAdapter(MyDataclass)
primitive scalar boundary                      TypeAdapter(int / UUID / Decimal / Annotated...)
union validation                               TypeAdapter(A | B)
discriminated union                            TypeAdapter(Annotated[A | B, Field(discriminator=...)])
top-level type schema                          TypeAdapter(T).json_schema()
JSON bytes serialization                       TypeAdapter(T).dump_json(...)
JSON-compatible Python object                  TypeAdapter(T).dump_python(..., mode="json")
reused hot-path parser                         module-level TypeAdapter
rare/forward-ref adapter                       TypeAdapter(..., config=ConfigDict(defer_build=True))
named object with methods/state                BaseModel
one real wrapper key exists                    BaseModel with one field
```

---

## 12.26 Agent checklist

```text
[ ] Use TypeAdapter for top-level types that are not BaseModel subclasses.
[ ] Do not use TypeAdapter instances as field annotations.
[ ] Create adapters once and reuse in hot paths.
[ ] Use validate_python for Python objects.
[ ] Use validate_json for raw JSON str/bytes/bytearray.
[ ] Use validate_strings for stringly dict-like data.
[ ] Use dump_python for Python/JSON-compatible Python objects.
[ ] Remember dump_json returns bytes.
[ ] Use json_schema for top-level arbitrary type schemas.
[ ] Use TypeAdapter.json_schemas for bundles of adapted types.
[ ] Use adapter-level config only when adapted type does not own config.
[ ] Configure BaseModel / TypedDict / dataclass at declaration, not through adapter config.
[ ] Use defer_build=True for forward refs or expensive rare schemas.
[ ] Call rebuild with _types_namespace when forward refs are not automatically resolved.
[ ] Use TypeAdapter(list[Model]) for top-level arrays of models.
[ ] Use TypeAdapter(TypedDict) when output should remain dict.
[ ] Use BaseModel when object identity, methods, field tracking, or model_config inheritance matter.
[ ] Treat experimental_allow_partial as unstable unless pinned/tested.
```

---

## 12.27 Value case

```text
TypeAdapter value =
  validation without artificial BaseModel wrappers
  top-level arrays/dicts/unions/scalars
  dataclass and TypedDict validation
  reusable constrained alias validation
  BaseModel-like dump/json_schema methods for non-model types
  raw JSON validation for non-object payloads
  JSON bytes serialization for arbitrary top-level types
  adapter-level config for simple adapted types
  deferred schema build and manual rebuild for forward refs
  performance through reusable compiled validators/serializers
```

Operationally: use `TypeAdapter` as the **schema/validator/serializer handle for arbitrary type annotations**. Create it once, reuse it, choose the validation method by input encoding, use `dump_json()` with bytes semantics, and reserve `BaseModel` for named record contracts where object identity, methods, model config inheritance, field tracking, and rich model APIs matter.


# Pydantic Advanced — 13) Dataclasses, `TypedDict`s, and model-like types — Pydantic v2 deep dive

Style target: advanced, sectioned, agent-oriented technical reference. 

Pydantic supports model-like validation for `BaseModel`, Pydantic dataclasses, stdlib dataclasses, `TypedDict`, named tuples, and adapted arbitrary types. Key boundary: **`BaseModel` owns rich model APIs**; dataclasses and `TypedDict`s can be validated by Pydantic but usually require `TypeAdapter` for direct validation, dumping, and JSON Schema generation. Pydantic states that stdlib dataclasses receive validation when used inside a `BaseModel`, a Pydantic dataclass, or a `TypeAdapter`, and that stdlib and Pydantic dataclasses are functionally equivalent as field annotations for validation. ([Pydantic Docs][S13-1])

---

## 13.0 Mental model

```text
BaseModel
  named object schema
  rich model APIs
  model_config inheritance
  model_dump / model_json_schema / model_copy / validators / serializers

pydantic.dataclasses.dataclass
  dataclass ergonomics
  Pydantic validation on construction
  dataclass-style fields / ordering / frozen / init behavior
  no direct BaseModel methods

stdlib dataclass
  no Pydantic validation on direct construction
  Pydantic validation when nested/adapted
  config can be attached with __pydantic_config__ or with_config

TypedDict
  validates dictionary shape
  output remains dict
  no object methods
  best through TypeAdapter or as field annotation

NamedTuple
  validates positional or dict input
  serializes as tuple in Python mode, array in JSON mode
```

Agent invariant:

```text
Need named API DTO with dump/schema/copy/introspection:
  BaseModel

Need dataclass ergonomics and constructor validation:
  pydantic.dataclasses.dataclass

Need validated plain dict:
  TypedDict + TypeAdapter

Need validated positional record:
  NamedTuple

Need top-level list/dict/union/dataclass/TypedDict validation:
  TypeAdapter
```

---

## 13.1 `pydantic.dataclasses.dataclass`

### Basic syntax

```python
from datetime import datetime
from typing import Optional

from pydantic.dataclasses import dataclass

@dataclass
class User:
    id: int
    name: str = "John Doe"
    signup_ts: Optional[datetime] = None

user = User(id="42", signup_ts="2032-06-21T12:00")
assert user.id == 42
assert user.name == "John Doe"
assert user.signup_ts == datetime(2032, 6, 21, 12, 0)
```

Pydantic dataclasses provide stdlib-dataclass-like objects with Pydantic validation at construction time; they are explicitly **not** a replacement for `BaseModel`, and Pydantic docs say models are the better choice in some cases. ([Pydantic Docs][S13-1])

### Decorator contract

```python
from pydantic.dataclasses import dataclass

@dataclass(
    init=False,       # same dataclass-style decorator surface where supported
    repr=True,
    eq=True,
    order=False,
    frozen=False,
    config=None,     # Pydantic addition
)
class Example:
    x: int
```

Pydantic’s dataclass decorator accepts the same arguments as the stdlib decorator, plus a `config` parameter for Pydantic behavior. ([Pydantic Docs][S13-1])

---

## 13.2 Pydantic dataclass field declarations

### `dataclasses.field(...)` and `pydantic.Field(...)`

```python
import dataclasses
from typing import Optional

from pydantic import Field
from pydantic.dataclasses import dataclass

@dataclass
class User:
    id: int
    name: str = "John Doe"
    friends: list[int] = dataclasses.field(default_factory=lambda: [0])
    age: Optional[int] = dataclasses.field(
        default=None,
        metadata={"title": "The age of the user", "description": "do not lie!"},
    )
    height: Optional[int] = Field(
        default=None,
        title="The height in cm",
        ge=50,
        le=300,
    )

user = User(id="42", height="250")
assert user.id == 42
assert user.height == 250
```

Pydantic dataclasses support both stdlib `dataclasses.field()` and Pydantic `Field()`; `Field()` enables validation constraints and JSON Schema metadata such as bounds/title/description. ([Pydantic Docs][S13-1])

Agent rule:

```text
dataclasses.field:
  default_factory
  dataclass metadata
  dataclass-native behavior

pydantic.Field:
  validation constraints
  JSON Schema metadata
  Pydantic field behavior
```

---

## 13.3 Dataclass config

### Decorator config

```python
from pydantic import ConfigDict, ValidationError
from pydantic.dataclasses import dataclass

@dataclass(config=ConfigDict(str_max_length=10, validate_assignment=True))
class User:
    name: str

user = User(name="John Doe")

try:
    user.name = "x" * 20
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "string_too_long"
```

### `__pydantic_config__` attribute

```python
from pydantic import ConfigDict
from pydantic.dataclasses import dataclass

@dataclass
class User:
    id: int
    name: str

    __pydantic_config__ = ConfigDict(validate_assignment=True)
```

Pydantic dataclass configuration can be provided through `@dataclass(config=ConfigDict(...))` or through `__pydantic_config__`; assignment validation works for dataclasses when configured. ([Pydantic Docs][S13-1])

Policy:

```text
Use @dataclass(config=...):
  local visible config
  preferred for direct class declaration

Use __pydantic_config__:
  when decorator style is constrained
  when converting/augmenting existing dataclass-like classes
```

---

## 13.4 Assignment validation for dataclasses

```python
from pydantic import ConfigDict, ValidationError
from pydantic.dataclasses import dataclass

@dataclass(config=ConfigDict(validate_assignment=True))
class Point:
    x: int
    y: int

p = Point(x="1", y="2")
assert p.x == 1

try:
    p.x = "not an int"
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "int_parsing"
```

`validate_assignment=True` on a Pydantic dataclass revalidates field assignment just like model assignment validation. The config docs show assignment validation raising a `ValidationError` when a dataclass field is assigned a string exceeding configured length. ([Pydantic Docs][S13-2])

Agent rule:

```text
Mutable dataclass + type safety:
  validate_assignment=True

Immutable dataclass:
  frozen=True

Hot-path trusted dataclass:
  validate_assignment=False
```

---

## 13.5 Pydantic dataclasses vs `BaseModel`

### Pydantic dataclass missing model APIs

```python
from pydantic import TypeAdapter
from pydantic.dataclasses import dataclass

@dataclass
class Foo:
    f: int

foo = Foo(f=1)

assert TypeAdapter(Foo).dump_python(foo) == {"f": 1}
assert TypeAdapter(Foo).validate_python({"f": "1"}) == Foo(f=1)
```

Differences:

```text
Pydantic dataclass:
  no model_validate class method
  no model_dump instance method
  no model_json_schema class method
  use TypeAdapter for validation/dumping/schema
  extra="allow" data not included in serialization
  cannot customize extra values with __pydantic_extra__
  generic parameterized dataclasses are not validated as expected unless wrapped with TypeAdapter

BaseModel:
  model_validate
  model_validate_json
  model_dump
  model_dump_json
  model_json_schema
  model_copy
  model_fields / model_fields_set
  richer config inheritance and introspection
```

Pydantic docs list these differences, including the lack of model methods on dataclasses, the need to wrap dataclasses with `TypeAdapter`, different `extra` behavior, and limitations around parameterized generic dataclasses. ([Pydantic Docs][S13-1])

Decision rule:

```text
Use BaseModel when:
  public API model
  JSON Schema generation is direct requirement
  model_dump/model_dump_json are central
  field tracking / partial updates matter
  model validators/serializers and config inheritance matter

Use Pydantic dataclass when:
  dataclass semantics are central
  constructor validation is enough
  object should behave like dataclass
  existing dataclass-oriented codebase
```

---

## 13.6 Stdlib dataclasses: direct construction vs Pydantic validation

### Direct stdlib dataclass construction: no validation

```python
import dataclasses

@dataclasses.dataclass
class User:
    name: str

user = User(name=["not", "a", "string"])
assert user.name == ["not", "a", "string"]
```

### Validation when nested in `BaseModel`

```python
import dataclasses
from typing import Optional

from pydantic import BaseModel, ConfigDict, ValidationError

@dataclasses.dataclass(frozen=True)
class User:
    name: str

class Foo(BaseModel):
    model_config = ConfigDict(revalidate_instances="always")
    user: Optional[User] = None

try:
    Foo(user=User(name=["not", "a", "string"]))
except ValidationError as exc:
    assert exc.errors()[0]["loc"] == ("user", "name")
```

When a stdlib dataclass is used within a Pydantic model, Pydantic dataclass, or `TypeAdapter`, Pydantic applies validation, and using a stdlib dataclass or Pydantic dataclass as a field annotation is functionally equivalent for validation. Existing invalid dataclass instances may require `revalidate_instances='always'` to catch corrupted/preexisting attributes. ([Pydantic Docs][S13-1])

Agent rule:

```text
stdlib dataclass direct __init__:
  no Pydantic validation

stdlib dataclass inside BaseModel / Pydantic dataclass / TypeAdapter:
  Pydantic validation applies

existing dataclass instance crossing trust boundary:
  set revalidate_instances="always" on containing model if revalidation is required
```

---

## 13.7 Applying Pydantic validation to stdlib dataclasses directly

### Convert stdlib dataclass to Pydantic dataclass subclass

```python
import dataclasses
import pydantic

@dataclasses.dataclass
class A:
    a: int

PydanticA = pydantic.dataclasses.dataclass(A)

assert PydanticA(a="1").a == 1
```

Pydantic’s dataclass decorator can be applied directly to a stdlib dataclass, creating a new subclass with Pydantic validation. ([Pydantic Docs][S13-1])

### Inherit from stdlib dataclasses

```python
import dataclasses
import pydantic

@dataclasses.dataclass
class Z:
    z: int

@dataclasses.dataclass
class Y(Z):
    y: int = 0

@pydantic.dataclasses.dataclass
class X(Y):
    x: int = 0

foo = X(x=b"1", y="2", z="3")
assert foo == X(z=3, y=2, x=1)
```

Pydantic validates inherited fields from stdlib dataclasses when a subclass is decorated as a Pydantic dataclass. ([Pydantic Docs][S13-1])

---

## 13.8 Stdlib dataclasses inside `BaseModel`

```python
import dataclasses

from pydantic import BaseModel

@dataclasses.dataclass
class Address:
    street: str
    zip_code: int

class UserModel(BaseModel):
    name: str
    address: Address

u = UserModel.model_validate({
    "name": "Ada",
    "address": {"street": "Main", "zip_code": "12345"},
})

assert isinstance(u.address, Address)
assert u.address.zip_code == 12345
```

Use cases:

```text
domain dataclass nested in API DTO
config/dataclass values in model graph
dataclass library interop
transition from dataclasses to Pydantic models
```

Boundary rule:

```text
If public response needs model_dump/model_json_schema behavior on the top-level object:
  top-level BaseModel

If nested value object is dataclass-like:
  dataclass field is fine
```

---

## 13.9 Dataclass custom types and `arbitrary_types_allowed`

```python
import dataclasses

from pydantic import BaseModel, ConfigDict

class ArbitraryType:
    def __init__(self, value):
        self.value = value

@dataclasses.dataclass
class DC:
    a: ArbitraryType
    b: str

class Model(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)

    dc: DC
    other: str
```

If a stdlib dataclass used by Pydantic contains unknown arbitrary types, Pydantic may fail to generate a schema unless `arbitrary_types_allowed=True` is configured or the custom type implements Pydantic core-schema support. Pydantic docs show `arbitrary_types_allowed=True` pushing down to nested vanilla dataclasses. ([Pydantic Docs][S13-1])

Agent rule:

```text
arbitrary_types_allowed=True:
  instance check only
  no internal validation of arbitrary object state

Prefer:
  custom __get_pydantic_core_schema__
  wrapper BaseModel
  validated dataclass field types
```

---

## 13.10 Dataclass validators and initialization hooks

```python
from pydantic import field_validator
from pydantic.dataclasses import dataclass

@dataclass
class Product:
    product_id: str

    @field_validator("product_id", mode="before")
    @classmethod
    def convert_int_serial(cls, v):
        if isinstance(v, int):
            return str(v).zfill(5)
        return v

assert Product(product_id="01234").product_id == "01234"
assert Product(product_id=2468).product_id == "02468"
```

Pydantic validators work with Pydantic dataclasses; Pydantic’s docs show a dataclass field validator converting integer serial numbers to zero-padded strings before validation. ([Pydantic Docs][S13-1])

Agent rule:

```text
Use dataclass validators for:
  dataclass-local parsing/normalization
  simple invariant checks

Avoid:
  DB/network/service calls
  authorization
  persistence side effects
```

---

## 13.11 Rebuilding dataclass schema

```python
from pydantic.dataclasses import rebuild_dataclass

rebuild_dataclass(MyDataclass)
```

`rebuild_dataclass()` rebuilds the core schema for a Pydantic dataclass, analogous to rebuilding a model schema, and is useful when forward references or late-defined types are involved. ([Pydantic Docs][S13-1])

Use cases:

```text
forward refs in dataclass annotations
recursive dataclass graphs
late imports
dynamic plugin schemas
```

---

## 13.12 Detecting Pydantic dataclasses

```python
import dataclasses
import pydantic

@dataclasses.dataclass
class Std:
    id: int

PydanticStd = pydantic.dataclasses.dataclass(Std)

assert dataclasses.is_dataclass(Std) is True
assert pydantic.dataclasses.is_pydantic_dataclass(Std) is False
assert dataclasses.is_dataclass(PydanticStd) is True
assert pydantic.dataclasses.is_pydantic_dataclass(PydanticStd) is True
```

Pydantic dataclasses are still dataclasses according to `dataclasses.is_dataclass`; use `pydantic.dataclasses.is_pydantic_dataclass()` to distinguish Pydantic dataclasses from stdlib dataclasses. ([Pydantic Docs][S13-1])

---

## 13.13 `TypedDict` validation

```python
from typing_extensions import TypedDict

from pydantic import TypeAdapter, ValidationError

class User(TypedDict):
    name: str
    id: int

USER = TypeAdapter(User)

assert USER.validate_python({"name": "foo", "id": "1"}) == {
    "name": "foo",
    "id": 1,
}

try:
    USER.validate_python({"name": "foo"})
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "missing"
```

`TypedDict` declares a dictionary shape with fixed keys and value types. Pydantic validates `TypedDict` through `TypeAdapter` or when used as a field annotation; on Python 3.12 and lower, Pydantic requires `typing_extensions.TypedDict` due to runtime limitations. In strict mode, only `dict` instances are valid, and strictness does not automatically apply to values. ([Pydantic Docs][S13-3])

Agent rule:

```text
Use TypedDict when:
  output should remain plain dict
  no object methods needed
  schema is object-shaped with fixed keys
  interop code expects dict

Use BaseModel when:
  methods / serializers / validators / model_config inheritance / dump methods needed on instance
```

---

## 13.14 Optional and partial `TypedDict` keys

```python
from typing_extensions import NotRequired, Required, TypedDict
from pydantic import TypeAdapter

class UserPatch(TypedDict, total=False):
    name: str
    age: int

class MixedUser(TypedDict):
    id: Required[int]
    nickname: NotRequired[str]

PATCH = TypeAdapter(UserPatch)
MIXED = TypeAdapter(MixedUser)

assert PATCH.validate_python({"age": "42"}) == {"age": 42}
assert MIXED.validate_python({"id": "1"}) == {"id": 1}
```

`TypedDict` requiredness follows Python typing semantics: `total=False`, `Required`, and `NotRequired` drive required-key behavior, while Pydantic validates values according to their annotations.

---

## 13.15 `with_config` for `TypedDict` / stdlib types

### `TypedDict`

```python
from typing_extensions import TypedDict

from pydantic import ConfigDict, TypeAdapter, with_config

@with_config(ConfigDict(str_to_lower=True))
class UserDict(TypedDict):
    name: str

USER = TypeAdapter(UserDict)
assert USER.validate_python({"name": "ADA"}) == {"name": "ada"}
```

### Stdlib dataclass

```python
from dataclasses import dataclass

from pydantic import BaseModel, ConfigDict, with_config

@dataclass
@with_config(ConfigDict(str_to_lower=False))
class UserWithConfig:
    name: str
```

Pydantic supports configuration on stdlib dataclasses and `TypedDict`s either with `__pydantic_config__` or with `@with_config`; the docs recommend `with_config` for `TypedDict` because it avoids static type-checking issues. ([Pydantic Docs][S13-2])

Agent rule:

```text
Use with_config for:
  TypedDict config
  stdlib type config when type checker compatibility matters

Use __pydantic_config__ for:
  direct stdlib dataclass config when decorator stacking is undesirable
```

---

## 13.16 Configuration propagation boundaries

```python
from dataclasses import dataclass
from typing_extensions import TypedDict
from pydantic import BaseModel, ConfigDict, with_config

class PydanticUser(BaseModel):
    name: str

@dataclass
class StdlibUser:
    name: str

@with_config(ConfigDict(str_to_lower=False))
class ConfiguredTD(TypedDict):
    name: str

class Parent(BaseModel):
    model_config = ConfigDict(str_to_lower=True)

    pydantic_user: PydanticUser
    stdlib_user: StdlibUser
    configured_td: ConfiguredTD
```

Propagation rules:

```text
Pydantic models:
  parent config does not propagate
  each model is its own config boundary

Pydantic dataclasses:
  parent config does not propagate
  each dataclass is its own config boundary

stdlib dataclasses:
  parent config can propagate unless nested type has its own config

TypedDict:
  parent config can propagate unless nested type has its own config
```

Pydantic’s config docs explicitly distinguish configuration boundaries: Pydantic models/dataclasses do not receive parent config, while stdlib dataclasses and typed dictionaries receive propagated config unless they define their own config. ([Pydantic Docs][S13-2])

Agent rule:

```text
If nested behavior must be stable:
  configure nested type explicitly.

Do not assume parent model config changes nested Pydantic model/dataclass behavior.
```

---

## 13.17 Named tuples as model-like types

```python
from typing import NamedTuple
from pydantic import BaseModel

class Point(NamedTuple):
    x: int
    y: int

class Model(BaseModel):
    p: Point

assert Model(p=("1", 2)).p == Point(x=1, y=2)
assert Model(p={"x": "1", "y": 2}).p == Point(x=1, y=2)
assert Model(p=("1", 2)).model_dump() == {"p": (1, 2)}
```

Named tuples accept tuple/list input and dictionary input whose keys match named tuple field names; values are validated according to field definitions. In Python serialization mode they serialize as tuples, and in JSON mode they serialize as arrays. ([Pydantic Docs][S13-3])

Agent rule:

```text
Use NamedTuple when:
  positional record semantics matter
  tuple output in Python mode is acceptable
  JSON array output is acceptable

Use BaseModel when:
  JSON object output with named fields is required
  field metadata / aliases / schema docs matter
```

---

## 13.18 Revalidation of nested dataclass instances

### Problem: existing dataclass instance may already be invalid

```python
import dataclasses
from pydantic import BaseModel

@dataclasses.dataclass
class User:
    name: str

bad = User(name=["not", "a", "string"])  # no Pydantic validation here
```

### Revalidate at boundary

```python
from pydantic import ConfigDict, ValidationError

class Box(BaseModel):
    model_config = ConfigDict(revalidate_instances="always")
    user: User

try:
    Box(user=bad)
except ValidationError as exc:
    assert exc.errors()[0]["loc"] == ("user", "name")
```

Pydantic’s dataclass docs show that an invalid frozen stdlib dataclass instance can be caught when nested in a model configured with `revalidate_instances='always'`. ([Pydantic Docs][S13-1])

Policy:

```text
Nested dataclass constructed from raw dict:
  validation applies naturally

Nested dataclass already constructed elsewhere:
  use revalidate_instances="always" at trust boundary

Long-lived mutable dataclass:
  use Pydantic dataclass + validate_assignment=True
  or avoid mutation and reconstruct validated object
```

---

## 13.19 Immutability patterns

### Stdlib frozen dataclass

```python
import dataclasses

@dataclasses.dataclass(frozen=True)
class User:
    name: str
```

### Pydantic frozen dataclass

```python
from pydantic.dataclasses import dataclass

@dataclass(frozen=True)
class User:
    name: str
```

### Frozen `BaseModel`

```python
from pydantic import BaseModel, ConfigDict

class UserModel(BaseModel):
    model_config = ConfigDict(frozen=True)

    name: str
```

Policy:

```text
domain value object:
  frozen dataclass or frozen BaseModel

public API DTO:
  BaseModel, often not frozen unless value semantics matter

mutable command/config object:
  dataclass/BaseModel with validate_assignment if mutation allowed

security note:
  frozen blocks attribute reassignment
  frozen is not authorization
  frozen is not deep immutability of all nested mutable objects
```

The dataclass docs show stdlib frozen dataclass behavior preserved when nested in Pydantic, including `FrozenInstanceError` on field assignment. ([Pydantic Docs][S13-1])

---

## 13.20 Recursive structures

### Recursive dataclass

```python
from __future__ import annotations

from pydantic.dataclasses import dataclass
from pydantic.dataclasses import rebuild_dataclass

@dataclass
class Node:
    name: str
    children: list[Node] | None = None

rebuild_dataclass(Node)
```

### Recursive `BaseModel`

```python
from __future__ import annotations

from pydantic import BaseModel

class NodeModel(BaseModel):
    name: str
    children: list[NodeModel] | None = None

NodeModel.model_rebuild()
```

Guidance:

```text
Recursive public API schema:
  BaseModel usually clearer
  direct model_json_schema()
  easier forward-ref rebuild

Recursive internal value object:
  dataclass acceptable
  use rebuild_dataclass when needed
  wrap with TypeAdapter for dump/schema/validation outside constructor
```

`rebuild_dataclass()` is the dataclass equivalent of schema rebuild for forward annotations and late definitions. ([Pydantic Docs][S13-1])

---

## 13.21 Domain dataclasses vs API models

### Domain dataclass

```python
from pydantic.dataclasses import dataclass

@dataclass(frozen=True)
class Money:
    amount_cents: int
    currency: str
```

Characteristics:

```text
domain value semantics
small object
no JSON API concerns
no direct model_dump required
can be frozen
can be nested inside BaseModel
```

### API model

```python
from pydantic import BaseModel, ConfigDict, Field

class MoneyDto(BaseModel):
    model_config = ConfigDict(extra="forbid")

    amount_cents: int = Field(ge=0)
    currency: str = Field(min_length=3, max_length=3)
```

Characteristics:

```text
request/response contract
JSON Schema generation
OpenAPI compatibility
aliases / external naming
model_dump / model_dump_json
rich validation errors
```

Pattern:

```python
class InvoiceResponse(BaseModel):
    id: str
    total: Money
```

Agent rule:

```text
Use dataclass for internal value semantics.
Use BaseModel for external API boundary.
Convert explicitly when boundary policy differs.
```

---

## 13.22 Standard-library type decision matrix

```text
Need                                           Preferred type
--------------------------------------------------------------------------------
public API request/response object              BaseModel
internal immutable value object                 frozen dataclass or frozen BaseModel
constructor-validated dataclass semantics        pydantic.dataclasses.dataclass
plain dict shape                                 TypedDict + TypeAdapter
top-level array of models                        TypeAdapter(list[Model])
top-level dataclass validation/dump/schema       TypeAdapter(MyDataclass)
positional tuple-like record                     NamedTuple
nested domain value object in API DTO            dataclass nested in BaseModel
partial update dict                              TypedDict(total=False) or BaseModel patch model
runtime mutation with validation                 Pydantic dataclass/BaseModel + validate_assignment
strict revalidation of preexisting instances     revalidate_instances="always"
custom unknown runtime handle                    arbitrary_types_allowed with caution
```

---

## 13.23 Complete example: domain dataclass + API model + TypedDict input

```python
from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal
from typing_extensions import NotRequired, TypedDict

from pydantic import BaseModel, ConfigDict, Field, TypeAdapter, with_config
from pydantic.dataclasses import dataclass as pydantic_dataclass


@pydantic_dataclass(config=ConfigDict(validate_assignment=True))
class Money:
    amount: Decimal = Field(ge=Decimal("0"), max_digits=12, decimal_places=2)
    currency: str = Field(min_length=3, max_length=3)


@dataclass(frozen=True)
class ProductId:
    value: str


@with_config(ConfigDict(str_strip_whitespace=True))
class LineItemInput(TypedDict):
    product_id: str
    quantity: int
    note: NotRequired[str]


LINE_ITEM_INPUT = TypeAdapter(LineItemInput)


class LineItemResponse(BaseModel):
    model_config = ConfigDict(extra="forbid")

    product_id: str
    quantity: int = Field(ge=1)
    unit_price: Money


raw = {"product_id": " sku-1 ", "quantity": "2"}
parsed = LINE_ITEM_INPUT.validate_python(raw)

response = LineItemResponse(
    product_id=parsed["product_id"],
    quantity=parsed["quantity"],
    unit_price=Money(amount="12.30", currency="USD"),
)

assert response.model_dump(mode="json")
```

---

## 13.24 Anti-patterns

### Anti-pattern: expecting stdlib dataclass direct validation

```python
import dataclasses

@dataclasses.dataclass
class User:
    id: int

u = User(id="not an int")  # accepted by stdlib dataclass
```

Correct:

```python
USER = TypeAdapter(User)
u = USER.validate_python({"id": "1"})
```

or use `pydantic.dataclasses.dataclass`.

---

### Anti-pattern: using Pydantic dataclass when `BaseModel` methods are required

```python
@dataclass
class User:
    id: int

User.model_json_schema()  # not available
```

Correct:

```python
TypeAdapter(User).json_schema()
```

or use `BaseModel`.

---

### Anti-pattern: parameterized generic dataclass trusted for validation

```python
from typing import Generic, TypeVar
from pydantic.dataclasses import dataclass

T = TypeVar("T")

@dataclass
class Box(Generic[T]):
    item: T

Box[int](item="not an int")  # not validated as expected
```

Pydantic docs warn that parameterized generic dataclasses are generic aliases, not proper type objects for this purpose, and Pydantic treats them effectively like `Any` unless wrapped appropriately. ([Pydantic Docs][S13-1])

Correct:

```python
BOX_INT = TypeAdapter(Box[int])
BOX_INT.validate_python({"item": "1"})
```

---

### Anti-pattern: parent config assumed to affect nested Pydantic dataclass

```python
class Parent(BaseModel):
    model_config = ConfigDict(str_to_lower=True)
    user: PydanticUser  # Pydantic dataclass/model config boundary
```

Correct:

```python
@dataclass(config=ConfigDict(str_to_lower=True))
class PydanticUser:
    name: str
```

Pydantic models and Pydantic dataclasses are configuration boundaries; parent config does not propagate into them. ([Pydantic Docs][S13-2])

---

### Anti-pattern: `TypedDict` when methods/serialization hooks are needed

```python
class User(TypedDict):
    id: int
    name: str
```

If you need field serializers, computed fields, model validators, aliases, or `model_dump_json()`, use `BaseModel`.

---

## 13.25 Test matrix

### Pydantic dataclass construction validation

```python
def test_pydantic_dataclass_validates():
    @pydantic_dataclasses.dataclass
    class User:
        id: int

    assert User(id="1").id == 1

    with pytest.raises(ValidationError):
        User(id="bad")
```

### Assignment validation

```python
def test_dataclass_assignment_validation():
    @pydantic_dataclasses.dataclass(config=ConfigDict(validate_assignment=True))
    class User:
        name: str

    user = User(name="Ada")

    with pytest.raises(ValidationError):
        user.name = 123
```

### Stdlib dataclass nested validation

```python
def test_stdlib_dataclass_nested_validation():
    @dataclasses.dataclass
    class Address:
        zip_code: int

    class User(BaseModel):
        address: Address

    user = User.model_validate({"address": {"zip_code": "12345"}})

    assert isinstance(user.address, Address)
    assert user.address.zip_code == 12345
```

### Revalidation of existing invalid dataclass instance

```python
def test_revalidate_existing_dataclass_instance():
    @dataclasses.dataclass
    class User:
        name: str

    bad = User(name=["bad"])

    class Box(BaseModel):
        model_config = ConfigDict(revalidate_instances="always")
        user: User

    with pytest.raises(ValidationError):
        Box(user=bad)
```

### `TypedDict` validation

```python
def test_typed_dict_adapter():
    class User(TypedDict):
        id: int
        name: str

    ta = TypeAdapter(User)

    assert ta.validate_python({"id": "1", "name": "Ada"}) == {
        "id": 1,
        "name": "Ada",
    }

    with pytest.raises(ValidationError):
        ta.validate_python({"name": "Ada"})
```

### `with_config`

```python
def test_with_config_typed_dict():
    @with_config(ConfigDict(str_to_lower=True))
    class User(TypedDict):
        name: str

    assert TypeAdapter(User).validate_python({"name": "ADA"}) == {"name": "ada"}
```

### Named tuple behavior

```python
def test_named_tuple_validation():
    class Point(NamedTuple):
        x: int
        y: int

    class M(BaseModel):
        p: Point

    assert M(p=("1", "2")).p == Point(1, 2)
    assert M(p={"x": "1", "y": "2"}).p == Point(1, 2)
    assert M(p=("1", "2")).model_dump()["p"] == (1, 2)
    assert M(p=("1", "2")).model_dump(mode="json")["p"] == [1, 2]
```

---

## 13.26 Agent checklist

```text
[ ] Use BaseModel for public API DTOs.
[ ] Use Pydantic dataclasses for dataclass ergonomics plus validation.
[ ] Use TypeAdapter for direct dataclass validation/dump/schema.
[ ] Remember Pydantic dataclasses do not have model_dump/model_json_schema methods.
[ ] Use @dataclass(config=ConfigDict(...)) for dataclass config.
[ ] Use validate_assignment=True when dataclass mutation must be checked.
[ ] Remember stdlib dataclass direct construction does not validate.
[ ] Remember stdlib dataclasses are validated when nested/adapted.
[ ] Use revalidate_instances="always" for preexisting dataclass instances crossing trust boundaries.
[ ] Use arbitrary_types_allowed only for opaque runtime handles or custom-type escape hatches.
[ ] Use with_config for TypedDict and stdlib types where config is needed.
[ ] Use typing_extensions.TypedDict on Python 3.12 and below.
[ ] Use TypedDict for validated plain dict output.
[ ] Use BaseModel instead of TypedDict when field serializers/model methods are needed.
[ ] Configure nested Pydantic models/dataclasses explicitly; parent config does not propagate.
[ ] Test config propagation for stdlib dataclasses and TypedDicts.
[ ] Use NamedTuple only when tuple/array serialization is acceptable.
[ ] Rebuild dataclass schema for forward refs when automatic resolution fails.
```

---

## 13.27 Value case

```text
Dataclass / TypedDict / model-like type value =
  validation without forcing every object into BaseModel
  dataclass ergonomics with Pydantic parsing
  validated stdlib dataclass interop
  plain-dict contracts through TypedDict
  TypeAdapter-based validation/dump/schema for non-model types
  immutable internal value-object patterns
  recursive and nested schema support
  explicit config boundaries
  clean separation between domain dataclasses and API models
```

Operationally: use `BaseModel` for external contracts, Pydantic dataclasses for dataclass-native validated objects, stdlib dataclasses for simple domain objects that are validated at Pydantic boundaries, `TypedDict` for validated plain dictionaries, and `TypeAdapter` as the universal validation/serialization/schema handle for model-like types without `BaseModel` methods.

[S13-1]: https://docs.pydantic.dev/latest/concepts/dataclasses/ "Dataclasses | Pydantic Docs"
[S13-2]: https://docs.pydantic.dev/latest/concepts/config/ "Configuration | Pydantic Docs"
[S13-3]: https://docs.pydantic.dev/latest/api/standard_library_types/ "Standard Library Types | Pydantic Docs"


# Pydantic Advanced — 14) Custom types and reusable constraints — Pydantic v2 deep dive

Style target: dense, sectioned, LLM-agent-oriented technical reference. 

Pydantic’s preferred customization ladder is: **standard annotations → `Annotated[...]` constraints/validators/serializers/schema metadata → named aliases → `GetPydanticSchema` / marker annotations → `__get_pydantic_core_schema__` / `__get_pydantic_json_schema__` hooks**. Pydantic’s docs explicitly recommend sticking to high-level constructs such as `annotated-types`, `Field`, and functional validators where possible because direct `pydantic-core` schema hooks are lower-level and more likely to change. ([Pydantic Docs][S14-1])

---

## 14.0 Custom-type mental model

```text
Python annotation
  + Annotated metadata
  + Field constraints
  + annotated-types constraints
  + functional validators
  + functional serializers
  + JSON Schema metadata
  + optional core-schema hooks
  → pydantic-core schema
  → validation
  → serialization
  → JSON Schema
```

Customization planes:

```text
validation:
  Field(...)
  annotated-types: Gt, Ge, Lt, Le, Len, ...
  BeforeValidator
  AfterValidator
  PlainValidator
  WrapValidator
  __get_pydantic_core_schema__
  GetPydanticSchema

serialization:
  PlainSerializer
  WrapSerializer
  field_serializer / model_serializer
  core_schema serializer definitions

JSON Schema:
  Field(title, description, examples, json_schema_extra)
  WithJsonSchema
  __get_pydantic_json_schema__
  GenerateJsonSchema subclass
```

Agent rule:

```text
Use the shallowest customization layer that can express the contract.
Do not jump to pydantic-core hooks for simple constraints.
```

---

## 14.1 Reusable constrained aliases with `Annotated`

### Basic alias

```python
from typing import Annotated
from pydantic import Field, TypeAdapter

PositiveInt = Annotated[int, Field(gt=0)]

ta = TypeAdapter(PositiveInt)

assert ta.validate_python(1) == 1
```

Pydantic documents the `Annotated` pattern as the primary way to make reusable custom types across a codebase; its example defines `PositiveInt = Annotated[int, Field(gt=0)]` and validates it through `TypeAdapter`. ([Pydantic Docs][S14-2])

### Common constrained aliases

```python
from typing import Annotated
from decimal import Decimal
from pathlib import Path
from pydantic import Field

PositiveInt = Annotated[int, Field(gt=0)]
NonNegativeInt = Annotated[int, Field(ge=0)]
Percent = Annotated[float, Field(ge=0.0, le=1.0)]
Money = Annotated[Decimal, Field(ge=Decimal("0"), max_digits=12, decimal_places=2)]
Slug = Annotated[str, Field(min_length=1, max_length=128, pattern=r"^[a-z0-9]+(?:-[a-z0-9]+)*$")]
NonEmptyPath = Annotated[Path, Field()]
```

### Collection aliases

```python
PositiveIntList = Annotated[
    list[PositiveInt],
    Field(min_length=1, max_length=1000),
]

NonEmptyTags = Annotated[
    list[Annotated[str, Field(min_length=1)]],
    Field(min_length=1, max_length=100),
]
```

Agent rule:

```text
Alias local constraints:
  PositiveInt = Annotated[int, Field(gt=0)]

Alias reusable semantic domain:
  AccountId = Annotated[str, Field(pattern=...)]

Alias collection policy:
  NonEmptyList[T] via TypeAliasType or generic alias

Avoid:
  repeating Field(gt=0) in every model
```

---

## 14.2 `annotated-types` integration

```python
from typing import Annotated
from annotated_types import Gt, Ge, Lt, Le, Len, MinLen, MaxLen

PositiveInt = Annotated[int, Gt(0)]
NonNegativeInt = Annotated[int, Ge(0)]
ShortString = Annotated[str, Len(min_length=1, max_length=64)]
ShortList = Annotated[list[int], Len(max_length=4)]
```

Pydantic supports the `annotated-types` library as a Pydantic-agnostic way to express constraints; its docs show `PositiveInt = Annotated[int, Gt(0)]` and generic aliases such as `ShortList = Annotated[list[T], Len(max_length=4)]`. ([Pydantic Docs][S14-2])

### Use matrix

```text
Field(...):
  Pydantic-specific
  supports constraints + defaults + aliases + schema metadata
  good inside BaseModel field declarations

annotated-types:
  Pydantic-agnostic
  useful for shared library types
  clean domain aliases
  less tied to Pydantic imports
```

### Recommended split

```python
from typing import Annotated
from annotated_types import Gt, Len
from pydantic import Field

# reusable domain type, portable constraint
PositiveInt = Annotated[int, Gt(0)]

# Pydantic-specific field metadata
class Model(BaseModel):
    count: PositiveInt = Field(description="Number of items.")
```

Agent rule:

```text
Use annotated-types for reusable constraints.
Use Field for Pydantic-specific defaults, aliases, descriptions, examples, JSON Schema extras.
```

---

## 14.3 Combining validation, serialization, and schema metadata in one alias

```python
from typing import Annotated

from pydantic import AfterValidator, PlainSerializer, TypeAdapter, WithJsonSchema

TruncatedFloat = Annotated[
    float,
    AfterValidator(lambda x: round(x, 1)),
    PlainSerializer(lambda x: f"{x:.1e}", return_type=str),
    WithJsonSchema({"type": "string"}, mode="serialization"),
]

ta = TypeAdapter(TruncatedFloat)

assert ta.validate_python(1.02345) == 1.0
assert ta.dump_json(1.02345) == b'"1.0e+00"'
assert ta.json_schema(mode="validation") == {"type": "number"}
assert ta.json_schema(mode="serialization") == {"type": "string"}
```

Pydantic’s custom-types docs show this exact pattern: `Annotated` can attach validation, serialization, and JSON Schema metadata to an arbitrary type, producing different validation and serialization schemas. ([Pydantic Docs][S14-2])

Value case:

```text
single reusable alias
one source of validation behavior
one source of serialization behavior
one source of JSON Schema behavior
usable in BaseModel, TypeAdapter, dataclass, TypedDict fields
```

---

## 14.4 Custom validation markers

### `AfterValidator`

```python
from typing import Annotated
from pydantic import AfterValidator

def is_even(v: int) -> int:
    if v % 2:
        raise ValueError("must be even")
    return v

EvenInt = Annotated[int, AfterValidator(is_even)]
```

Use:

```text
typed semantic check
post-coercion normalization
safe default validator mode
```

### `BeforeValidator`

```python
from typing import Annotated, Any
from pydantic import BeforeValidator

def ensure_list(v: Any) -> Any:
    return v if isinstance(v, list) else [v]

IntList = Annotated[list[int], BeforeValidator(ensure_list)]
```

Use:

```text
raw shape migration
single value → list
legacy input normalization
```

### `PlainValidator`

```python
from typing import Annotated, Any
from pydantic import PlainValidator

def parse_custom(v: Any) -> int:
    return int(v)

CustomInt = Annotated[int, PlainValidator(parse_custom)]
```

Use:

```text
complete validation replacement
rare
must be heavily tested
can violate annotation if implemented badly
```

### `WrapValidator`

```python
from typing import Annotated, Any
from pydantic import Field, ValidationError, ValidatorFunctionWrapHandler, WrapValidator

def truncate(v: Any, handler: ValidatorFunctionWrapHandler) -> str:
    try:
        return handler(v)
    except ValidationError as exc:
        if exc.errors()[0]["type"] == "string_too_long":
            return handler(v[:5])
        raise

ShortStr = Annotated[str, Field(max_length=5), WrapValidator(truncate)]
```

Use:

```text
handler-controlled validation
fallback/retry
error-aware repair
advanced compatibility logic
```

Pydantic documents the same four field-validator modes: after, before, plain, and wrap; before validators receive raw input, plain validators terminate Pydantic’s internal validation, and wrap validators can call or bypass the handler. ([Pydantic Docs][S14-3])

Agent rule:

```text
AfterValidator:
  default for typed checks

BeforeValidator:
  raw input migration

PlainValidator:
  replace Pydantic validation entirely

WrapValidator:
  only when handler/error control is required
```

---

## 14.5 Custom serialization markers

### `PlainSerializer`

```python
from typing import Annotated
from pydantic import PlainSerializer

SpaceSeparatedList = Annotated[
    list[str],
    PlainSerializer(lambda xs: " ".join(xs), return_type=str),
]
```

Plain serializers replace the normal serialization output for the annotated type; `return_type` may be provided or inferred from the function annotation, and `when_used` controls whether the serializer is used always, only for JSON, unless `None`, etc. ([Pydantic Docs][S14-4])

### `WrapSerializer`

```python
from datetime import datetime, timezone
from typing import Annotated, Any

from pydantic import BaseModel, WrapSerializer

class EventDatetime(BaseModel):
    start: datetime
    end: datetime

def to_utc(value: Any, handler, info) -> dict[str, datetime | str]:
    partial = handler(value, info)
    if info.mode == "json":
        return {
            k: datetime.fromisoformat(v).astimezone(timezone.utc).isoformat().replace("+00:00", "Z")
            for k, v in partial.items()
        }
    return {k: v.astimezone(timezone.utc) for k, v in partial.items()}

UTCEventDatetime = Annotated[EventDatetime, WrapSerializer(to_utc)]
```

Wrap serializers receive the raw value plus a handler that applies standard serialization; they can modify the handler output before returning the final serialized value. ([Pydantic Docs][S14-4])

### Serializer use policy

```text
PlainSerializer:
  total replacement
  simple scalar/collection output transformation
  formatting Decimal/date/custom value

WrapSerializer:
  preserve default serialization then transform
  mode-aware output
  subclass-aware handler use
  timezone normalization
```

Agent rule:

```text
Use return_type for serializer output whenever possible.
Use when_used="json" for wire-format-only transformations.
Keep serializers pure: no DB, no network, no filesystem.
```

---

## 14.6 Custom JSON Schema: `WithJsonSchema`

```python
from typing import Annotated
from pydantic import WithJsonSchema

HexString = Annotated[
    str,
    WithJsonSchema(
        {
            "type": "string",
            "pattern": "^[0-9a-f]+$",
            "examples": ["deadbeef"],
        }
    ),
]
```

Use cases:

```text
custom format keyword
simplified schema for complex runtime type
tool/LLM schema metadata
UI hints
third-party type JSON Schema override
```

Pydantic documents `WithJsonSchema` as a simple way to add or override JSON Schema for a type; its custom-types example uses it alongside validators and serializers to provide a distinct serialization schema. ([Pydantic Docs][S14-2])

Agent rule:

```text
Use WithJsonSchema for simple reusable schema override.
Use json_schema_extra for additive metadata.
Use __get_pydantic_json_schema__ for generated-base + mutation.
```

---

## 14.7 Named type aliases

### Python 3.9+ with `TypeAliasType`

```python
from typing import Annotated
from annotated_types import Gt
from typing_extensions import TypeAliasType
from pydantic import BaseModel

PositiveIntList = TypeAliasType(
    "PositiveIntList",
    list[Annotated[int, Gt(0)]],
)

class Model(BaseModel):
    x: PositiveIntList
    y: PositiveIntList
```

### Python 3.12+ syntax

```python
from typing import Annotated
from annotated_types import Gt

type PositiveIntList = list[Annotated[int, Gt(0)]]
```

Pydantic v2.11+ supports named type aliases; unlike implicit aliases assigned to variables, named aliases can be converted into `$defs` and referenced with `$ref`, avoiding duplicate schema definitions when reused in a model. ([Pydantic Docs][S14-2])

### Do not put field-specific metadata in named aliases

Bad:

```python
from typing import Annotated
from typing_extensions import TypeAliasType
from pydantic import Field

MyAlias = TypeAliasType("MyAlias", Annotated[int, Field(default=1)])
```

Allowed:

```python
from annotated_types import Gt
MyAlias = TypeAliasType("MyAlias", Annotated[int, Gt(0)])
```

Pydantic states that named aliases cannot contain field-specific metadata such as `alias`, `default`, or `deprecated`; only metadata that applies to the annotated type itself, such as validation constraints and JSON metadata, is allowed. ([Pydantic Docs][S14-2])

Agent rule:

```text
Named alias:
  reusable type-level constraints
  reusable JSON Schema definition
  recursive aliases

Field metadata:
  keep at model field site
```

---

## 14.8 Generic aliases

```python
from typing import Annotated, TypeVar
from annotated_types import Len
from typing_extensions import TypeAliasType

T = TypeVar("T")

ShortList = TypeAliasType(
    "ShortList",
    Annotated[list[T], Len(max_length=4)],
    type_params=(T,),
)

# Python 3.12+
type ShortList312[T] = Annotated[list[T], Len(max_length=4)]
```

Generic aliases let agents define reusable parameterized constraints:

```python
ShortIntList = ShortList[int]
ShortStrList = ShortList[str]
```

Pydantic docs show generic `Annotated` aliases with `TypeVar`, including `ShortList = Annotated[list[T], Len(max_length=4)]` and a named generic alias using `TypeAliasType(..., type_params=(T,))`. ([Pydantic Docs][S14-2])

---

## 14.9 Named recursive aliases

### JSON recursive type

```python
from typing_extensions import TypeAliasType
from pydantic import TypeAdapter

Json = TypeAliasType(
    "Json",
    "dict[str, Json] | list[Json] | str | int | float | bool | None",
)

ta = TypeAdapter(Json)
schema = ta.json_schema()
```

### Python 3.12+

```python
type Json = dict[str, Json] | list[Json] | str | int | float | bool | None
```

Pydantic says named type aliases should be used whenever recursive aliases are needed because implicit recursive aliases are not supported reliably, especially across modules; named aliases are lazily evaluated and can produce `$defs` recursion. ([Pydantic Docs][S14-1])

Agent rule:

```text
Recursive alias:
  use TypeAliasType on Python <3.12
  use type statement on Python >=3.12
  validate through TypeAdapter or use in fields

Avoid:
  implicit recursive alias = ...
```

---

## 14.10 Advanced hook: `__get_pydantic_core_schema__`

### Custom subclass type

```python
from typing import Any
from pydantic import GetCoreSchemaHandler, TypeAdapter
from pydantic_core import CoreSchema, core_schema

class Username(str):
    @classmethod
    def __get_pydantic_core_schema__(
        cls,
        source_type: Any,
        handler: GetCoreSchemaHandler,
    ) -> CoreSchema:
        return core_schema.no_info_after_validator_function(cls, handler(str))

ta = TypeAdapter(Username)
res = ta.validate_python("abc")

assert isinstance(res, Username)
assert res == "abc"
```

`__get_pydantic_core_schema__` tells Pydantic how to generate a `pydantic-core` schema for a custom class. Pydantic describes it as middleware-like: the hook receives a `source_type` and a handler that can call the next `Annotated` metadata layer or Pydantic’s internal schema generation; it can modify the type, modify the returned core schema, or skip the handler entirely. ([Pydantic Docs][S14-1])

### Annotated metadata hook

```python
from dataclasses import dataclass
from typing import Annotated, Any, Callable

from pydantic import BaseModel, GetCoreSchemaHandler
from pydantic_core import CoreSchema, core_schema

@dataclass(frozen=True)
class MyAfterValidator:
    func: Callable[[Any], Any]

    def __get_pydantic_core_schema__(
        self,
        source_type: Any,
        handler: GetCoreSchemaHandler,
    ) -> CoreSchema:
        return core_schema.no_info_after_validator_function(
            self.func,
            handler(source_type),
        )

Username = Annotated[str, MyAfterValidator(str.lower)]

class Model(BaseModel):
    name: Username
```

Pydantic’s docs show marker objects implementing `__get_pydantic_core_schema__` inside `Annotated`; the marker is often a frozen dataclass so it is hashable, which matters for unions such as `Username | None`. ([Pydantic Docs][S14-1])

Agent rule:

```text
Implement on type when:
  you own the class
  runtime value should be custom subclass/type
  validation/serialization belongs to the class

Implement as Annotated metadata when:
  you do not own the class
  behavior is contextual
  base runtime type should remain unchanged
```

---

## 14.11 Advanced hook: `__get_pydantic_json_schema__`

```python
from typing import Any
from pydantic import GetCoreSchemaHandler, GetJsonSchemaHandler, TypeAdapter
from pydantic.json_schema import JsonSchemaValue
from pydantic_core import CoreSchema, core_schema

class Username(str):
    @classmethod
    def __get_pydantic_core_schema__(
        cls,
        source_type: Any,
        handler: GetCoreSchemaHandler,
    ) -> CoreSchema:
        return core_schema.no_info_after_validator_function(cls, handler(str))

    @classmethod
    def __get_pydantic_json_schema__(
        cls,
        core_schema_: CoreSchema,
        handler: GetJsonSchemaHandler,
    ) -> JsonSchemaValue:
        schema = handler(core_schema_)
        schema = handler.resolve_ref_schema(schema)
        schema["examples"] = ["ada-lovelace"]
        return schema
```

`__get_pydantic_json_schema__` customizes JSON Schema generation for a custom type or annotation marker; Pydantic’s third-party-type example returns the same schema as `int` by calling `handler(core_schema.int_schema())`. ([Pydantic Docs][S14-1])

Use:

```text
schema-only customization
examples/vendor metadata for custom type
mapping runtime custom type to simpler JSON shape
```

Avoid:

```text
simple Field(description=...)
simple WithJsonSchema(...)
global schema policy
```

---

## 14.12 `GetPydanticSchema`: lower boilerplate core-schema customization

```python
from typing import Annotated

from pydantic import BaseModel, GetPydanticSchema
from pydantic_core import core_schema

class Model(BaseModel):
    y: Annotated[
        str,
        GetPydanticSchema(
            lambda tp, handler: core_schema.no_info_after_validator_function(
                lambda x: x * 2,
                handler(tp),
            )
        ),
    ]

assert Model(y="ab").y == "abab"
```

`GetPydanticSchema` reduces boilerplate for simple core-schema customizations that would otherwise require a marker class; Pydantic’s docs explicitly present it as a shorter alternative to marker classes for simple cases. ([Pydantic Docs][S14-1])

Agent rule:

```text
Use GetPydanticSchema when:
  customization is short
  marker class would be boilerplate
  behavior belongs to one field/alias

Use marker class when:
  behavior is reused
  parameters are needed
  JSON Schema hook is also needed
  code readability beats lambda compactness
```

---

## 14.13 Third-party type integration

### Pattern: wrapper annotation, no monkeypatching

```python
from typing import Annotated, Any
from pydantic import BaseModel, GetCoreSchemaHandler, GetJsonSchemaHandler
from pydantic.json_schema import JsonSchemaValue
from pydantic_core import core_schema

class ThirdPartyType:
    x: int

    def __init__(self) -> None:
        self.x = 0

class _ThirdPartyPydantic:
    @classmethod
    def __get_pydantic_core_schema__(
        cls,
        _source_type: Any,
        _handler: GetCoreSchemaHandler,
    ) -> core_schema.CoreSchema:
        def from_int(value: int) -> ThirdPartyType:
            obj = ThirdPartyType()
            obj.x = value
            return obj

        from_int_schema = core_schema.chain_schema(
            [
                core_schema.int_schema(),
                core_schema.no_info_plain_validator_function(from_int),
            ]
        )

        return core_schema.json_or_python_schema(
            json_schema=from_int_schema,
            python_schema=core_schema.union_schema(
                [
                    core_schema.is_instance_schema(ThirdPartyType),
                    from_int_schema,
                ]
            ),
            serialization=core_schema.plain_serializer_function_ser_schema(
                lambda instance: instance.x
            ),
        )

    @classmethod
    def __get_pydantic_json_schema__(
        cls,
        _core_schema: core_schema.CoreSchema,
        handler: GetJsonSchemaHandler,
    ) -> JsonSchemaValue:
        return handler(core_schema.int_schema())

PydanticThirdPartyType = Annotated[ThirdPartyType, _ThirdPartyPydantic]

class Model(BaseModel):
    third_party_type: PydanticThirdPartyType
```

Pydantic’s docs show this exact integration pattern for third-party types: an `Annotated` wrapper with a marker class can accept either existing instances or integers, serialize instances as integers, and emit integer JSON Schema, without requiring changes to the third-party class itself. ([Pydantic Docs][S14-1])

Deployment rule:

```text
Third-party integration:
  prefer Annotated wrapper marker
  do not monkeypatch third-party class
  define validation from wire/native forms
  define serialization explicitly
  define JSON Schema explicitly
```

Examples:

```text
Pandas Timestamp / DataFrame
NumPy ndarray / scalar
domain library ID classes
custom money/currency classes
protobuf wrappers
geospatial objects
```

---

## 14.14 Custom generic classes

```python
from dataclasses import dataclass
from typing import Any, Generic, TypeVar

from pydantic import BaseModel, GetCoreSchemaHandler, ValidatorFunctionWrapHandler
from pydantic_core import CoreSchema, core_schema
from typing_extensions import get_args, get_origin

ItemT = TypeVar("ItemT")

@dataclass
class Owner(Generic[ItemT]):
    name: str
    item: ItemT

    @classmethod
    def __get_pydantic_core_schema__(
        cls,
        source_type: Any,
        handler: GetCoreSchemaHandler,
    ) -> CoreSchema:
        origin = get_origin(source_type) or source_type
        item_tp = get_args(source_type)[0] if get_origin(source_type) else Any

        item_schema = handler.generate_schema(item_tp)

        def validate_item(v: Owner[Any], item_handler: ValidatorFunctionWrapHandler) -> Owner[Any]:
            v.item = item_handler(v.item)
            return v

        python_schema = core_schema.chain_schema(
            [
                core_schema.is_instance_schema(cls),
                core_schema.no_info_wrap_validator_function(validate_item, item_schema),
            ]
        )

        return core_schema.json_or_python_schema(
            json_schema=core_schema.chain_schema(
                [
                    core_schema.typed_dict_schema(
                        {
                            "name": core_schema.typed_dict_field(core_schema.str_schema()),
                            "item": core_schema.typed_dict_field(item_schema),
                        }
                    ),
                    core_schema.no_info_before_validator_function(
                        lambda data: Owner(name=data["name"], item=data["item"]),
                        python_schema,
                    ),
                ]
            ),
            python_schema=python_schema,
        )
```

Pydantic’s docs describe custom generic classes as an advanced technique; `source_type` may differ from `cls`, so extract type parameters from `source_type` using `get_args`, and use `handler.generate_schema(...)` for generic parameters to generate an unrelated schema not affected by current metadata context. If a generic class implements `__get_pydantic_core_schema__`, `arbitrary_types_allowed` is not needed. ([Pydantic Docs][S14-1])

Agent rule:

```text
Custom generic hook:
  source_type → inspect get_origin/get_args
  generate item schema with handler.generate_schema(...)
  define python_schema for existing instances
  define json_schema for wire representation
  avoid arbitrary_types_allowed if hook exists
```

---

## 14.15 Generic container custom class

```python
from collections.abc import Sequence
from typing import Any, TypeVar

from pydantic import BaseModel, GetCoreSchemaHandler
from pydantic_core import core_schema
from typing_extensions import get_args

T = TypeVar("T")

class MySequence(Sequence[T]):
    def __init__(self, v: Sequence[T]) -> None:
        self.v = v

    def __getitem__(self, i):
        return self.v[i]

    def __len__(self) -> int:
        return len(self.v)

    @classmethod
    def __get_pydantic_core_schema__(
        cls,
        source: Any,
        handler: GetCoreSchemaHandler,
    ) -> core_schema.CoreSchema:
        args = get_args(source)
        sequence_schema = handler.generate_schema(Sequence[args[0]]) if args else handler.generate_schema(Sequence)

        return core_schema.union_schema(
            [
                core_schema.is_instance_schema(cls),
                core_schema.no_info_after_validator_function(MySequence, sequence_schema),
            ]
        )
```

Pydantic’s custom generic container example applies the same schema-generation pattern to a custom `Sequence` class: validate existing instances or validate a sequence and convert it to the custom container. ([Pydantic Docs][S14-1])

---

## 14.16 Custom-type design ladder

```text
Level 0: standard annotation
  int, str, datetime, list[T], BaseModel, TypedDict

Level 1: Field constraints
  Annotated[int, Field(gt=0)]

Level 2: annotated-types constraints
  Annotated[int, Gt(0)]

Level 3: functional validators / serializers / JSON schema markers
  AfterValidator, BeforeValidator, PlainSerializer, WithJsonSchema

Level 4: named aliases
  TypeAliasType / type statement for reuse and $defs

Level 5: marker classes / GetPydanticSchema
  reusable core-schema middleware without modifying target class

Level 6: custom type owns hooks
  __get_pydantic_core_schema__
  __get_pydantic_json_schema__

Level 7: global schema generator
  GenerateJsonSchema subclass
```

Pydantic’s own summary says high-level hooks such as `AfterValidator` and `Field` should be used when possible; lower-level direct `pydantic-core` customization is available through `GetPydanticSchema` or marker classes with `__get_pydantic_core_schema__`, and custom classes can implement the hook themselves when needed. ([Pydantic Docs][S14-1])

---

## 14.17 Field-specific vs type-specific metadata

### Type-specific metadata

Good in alias:

```python
PositiveInt = Annotated[int, Field(gt=0)]
Slug = Annotated[str, Field(pattern=r"^[a-z0-9-]+$")]
```

### Field-specific metadata

Keep at field site:

```python
class User(BaseModel):
    user_id: PositiveInt = Field(
        alias="userId",
        description="Public user identifier.",
        deprecated=False,
    )
```

Do not put field-specific metadata inside named type aliases:

```python
# Bad for named alias:
MyAlias = TypeAliasType("MyAlias", Annotated[int, Field(default=1)])
```

Pydantic explains that named aliases cannot hold field-specific metadata because Pydantic would need to eagerly inspect alias values, preventing the alias from being stored as a reusable JSON Schema definition. ([Pydantic Docs][S14-2])

---

## 14.18 Deployment recipes

### Domain scalar alias

```python
from typing import Annotated
from annotated_types import Len
from pydantic import AfterValidator, Field

def normalize_slug(v: str) -> str:
    v = v.strip().lower()
    if not v:
        raise ValueError("empty slug")
    return v

Slug = Annotated[
    str,
    Len(min_length=1, max_length=128),
    Field(pattern=r"^[a-z0-9]+(?:-[a-z0-9]+)*$"),
    AfterValidator(normalize_slug),
]
```

Policy:

```text
Reusable scalar:
  constraints + pure normalization
  no DB/network
  no field alias/default
```

---

### Money type with validation and JSON serialization

```python
from decimal import Decimal
from typing import Annotated

from pydantic import Field, PlainSerializer

Money = Annotated[
    Decimal,
    Field(ge=Decimal("0"), max_digits=12, decimal_places=2),
    PlainSerializer(lambda v: format(v, "f"), return_type=str, when_used="json"),
]
```

Policy:

```text
Money:
  Decimal runtime type
  string JSON output
  no float precision loss
```

---

### Reusable JSON value recursive alias

```python
from typing_extensions import TypeAliasType
from pydantic import TypeAdapter

Json = TypeAliasType(
    "Json",
    "dict[str, Json] | list[Json] | str | int | float | bool | None",
)

JSON_VALUE = TypeAdapter(Json)
```

Policy:

```text
Recursive general JSON:
  named alias
  TypeAdapter
  snapshot schema if public
```

Pydantic also defines `JsonValue` as a convenience type for this general JSON-value use case. ([Pydantic Docs][S14-1])

---

### Third-party ID class

```python
class ExternalId:
    def __init__(self, value: str) -> None:
        self.value = value

ExternalIdType = Annotated[
    ExternalId,
    GetPydanticSchema(
        lambda tp, handler: core_schema.no_info_after_validator_function(
            lambda v: v if isinstance(v, ExternalId) else ExternalId(str(v)),
            core_schema.union_schema(
                [
                    core_schema.is_instance_schema(ExternalId),
                    core_schema.str_schema(),
                ]
            ),
        )
    ),
]
```

Policy:

```text
Simple local integration:
  GetPydanticSchema ok

Reusable library integration:
  marker class with core + json schema hooks
```

---

## 14.19 Anti-patterns

### Anti-pattern: core-schema hook for simple constraints

```python
class PositiveInt(int):
    @classmethod
    def __get_pydantic_core_schema__(...):
        ...
```

Better:

```python
PositiveInt = Annotated[int, Field(gt=0)]
```

Reason:

```text
less code
more stable
better static type behavior
schema generation automatic
```

Pydantic recommends high-level constructs such as `annotated-types`, `Field`, and validators where possible before low-level core schema customization. ([Pydantic Docs][S14-1])

---

### Anti-pattern: `PlainValidator` accidentally bypassing type safety

```python
BadInt = Annotated[int, PlainValidator(lambda v: v)]
```

Problem:

```text
"not int" can become valid field value if returned
annotation becomes misleading
```

Prefer `BeforeValidator` or `AfterValidator`.

---

### Anti-pattern: serializer without `return_type`

```python
Money = Annotated[Decimal, PlainSerializer(lambda v: format(v, "f"))]
```

Prefer:

```python
Money = Annotated[Decimal, PlainSerializer(lambda v: format(v, "f"), return_type=str, when_used="json")]
```

Pydantic’s serializer docs note `return_type` can be provided or inferred; explicitly setting it improves clarity and gives Pydantic an output schema to check. ([Pydantic Docs][S14-4])

---

### Anti-pattern: field-specific metadata in named alias

```python
UserId = TypeAliasType("UserId", Annotated[int, Field(alias="userId")])
```

Keep alias/default/deprecation at the field declaration.

---

### Anti-pattern: third-party class monkeypatching

```python
ThirdPartyType.__get_pydantic_core_schema__ = ...
```

Prefer an `Annotated[ThirdPartyType, Marker]` wrapper so integration is explicit and local.

---

### Anti-pattern: mutating existing generic container in validator without policy

```python
def validate_item(v, handler):
    v.item = handler(v.item)
    return v
```

If mutation is unacceptable, create a new instance instead:

```python
return Owner(name=v.name, item=handler(v.item))
```

---

## 14.20 Test matrix

### Constraint alias

```python
def test_positive_int_alias():
    PositiveInt = Annotated[int, Field(gt=0)]
    ta = TypeAdapter(PositiveInt)

    assert ta.validate_python("1") == 1

    with pytest.raises(ValidationError) as exc:
        ta.validate_python(0)

    assert exc.value.errors()[0]["type"] == "greater_than"
```

### annotated-types alias

```python
def test_annotated_types_gt():
    PositiveInt = Annotated[int, Gt(0)]
    assert TypeAdapter(PositiveInt).validate_python(1) == 1
```

### Validator + serializer + schema alias

```python
def test_custom_alias_validation_serialization_schema():
    TruncatedFloat = Annotated[
        float,
        AfterValidator(lambda x: round(x, 1)),
        PlainSerializer(lambda x: f"{x:.1e}", return_type=str),
        WithJsonSchema({"type": "string"}, mode="serialization"),
    ]

    ta = TypeAdapter(TruncatedFloat)

    assert ta.validate_python(1.023) == 1.0
    assert ta.dump_json(1.023) == b'"1.0e+00"'
    assert ta.json_schema(mode="serialization") == {"type": "string"}
```

### Named alias schema reuse

```python
def test_named_alias_defs_reuse():
    PositiveIntList = TypeAliasType(
        "PositiveIntList",
        list[Annotated[int, Gt(0)]],
    )

    class M(BaseModel):
        x: PositiveIntList
        y: PositiveIntList

    schema = M.model_json_schema()
    assert "PositiveIntList" in schema["$defs"]
    assert schema["properties"]["x"]["$ref"] == "#/$defs/PositiveIntList"
```

### Recursive alias

```python
def test_recursive_json_alias():
    Json = TypeAliasType(
        "Json",
        "dict[str, Json] | list[Json] | str | int | float | bool | None",
    )

    ta = TypeAdapter(Json)
    assert ta.validate_python({"x": [1, None, "ok"]}) == {"x": [1, None, "ok"]}
    assert "$defs" in ta.json_schema()
```

### Third-party wrapper

```python
def test_third_party_type_wrapper():
    class ThirdPartyType:
        def __init__(self) -> None:
            self.x = 0

    class Marker:
        @classmethod
        def __get_pydantic_core_schema__(cls, source, handler):
            def from_int(v: int):
                obj = ThirdPartyType()
                obj.x = v
                return obj

            return core_schema.json_or_python_schema(
                json_schema=core_schema.chain_schema(
                    [
                        core_schema.int_schema(),
                        core_schema.no_info_plain_validator_function(from_int),
                    ]
                ),
                python_schema=core_schema.union_schema(
                    [
                        core_schema.is_instance_schema(ThirdPartyType),
                        core_schema.chain_schema(
                            [
                                core_schema.int_schema(),
                                core_schema.no_info_plain_validator_function(from_int),
                            ]
                        ),
                    ]
                ),
                serialization=core_schema.plain_serializer_function_ser_schema(lambda v: v.x),
            )

    T = Annotated[ThirdPartyType, Marker]

    class M(BaseModel):
        x: T

    assert M(x=1).x.x == 1
    assert M(x=1).model_dump() == {"x": 1}
```

### Generic custom class

```python
def test_custom_generic_owner():
    class Car(BaseModel):
        color: str

    class M(BaseModel):
        owner: Owner[Car]

    m = M.model_validate_json('{"owner":{"name":"Ada","item":{"color":"black"}}}')
    assert isinstance(m.owner.item, Car)
```

---

## 14.21 Deployment decision matrix

```text
Need                                           Preferred mechanism
--------------------------------------------------------------------------------
positive integer reused across models           Annotated[int, Field(gt=0)] or Gt(0)
Pydantic-agnostic reusable constraint           annotated-types: Gt, Len, etc.
reusable scalar normalization                   Annotated[T, AfterValidator(...)]
legacy raw input shape                          BeforeValidator
complete custom parser                          PlainValidator; rare
fallback/retry around validation                WrapValidator
custom JSON wire format                         PlainSerializer / WrapSerializer
simple schema override                          WithJsonSchema
named reusable JSON Schema definition           TypeAliasType / type statement
recursive alias                                 named alias only
field-specific metadata                         Field(...) at model field site
third-party type integration                    Annotated[ThirdPartyType, Marker]
simple one-off core-schema customization        GetPydanticSchema
owned custom class validation                   __get_pydantic_core_schema__ on class
custom class JSON Schema                        __get_pydantic_json_schema__
custom generic arbitrary class                  __get_pydantic_core_schema__ + get_args/source_type
global schema policy                            GenerateJsonSchema subclass
```

---

## 14.22 Agent checklist

```text
[ ] Start with standard type annotations.
[ ] Use Field / annotated-types for constraints.
[ ] Use Annotated aliases for reusable constraints.
[ ] Use TypeAliasType or type statements for schema reuse and recursion.
[ ] Keep field-specific metadata out of named aliases.
[ ] Use AfterValidator for typed semantic checks.
[ ] Use BeforeValidator for raw shape normalization.
[ ] Avoid PlainValidator unless replacing validation is intentional.
[ ] Avoid WrapValidator unless handler/error control is required.
[ ] Use PlainSerializer for total output replacement.
[ ] Use WrapSerializer when default serialization should run first.
[ ] Supply return_type for serializers.
[ ] Use WithJsonSchema for simple custom schema.
[ ] Use __get_pydantic_json_schema__ only for advanced schema mutation.
[ ] Use GetPydanticSchema for simple low-level schema customization.
[ ] Use marker classes for third-party types.
[ ] Use class hooks only when the class owns the Pydantic behavior.
[ ] For generics, inspect source_type, not cls.
[ ] Use handler.generate_schema(...) for generic type parameters.
[ ] Keep custom validators/serializers pure and deterministic.
[ ] Avoid DB/network/authorization logic in custom types.
[ ] Snapshot validation, dump, and JSON Schema behavior for public aliases.
```

---

## 14.23 Value case

```text
Custom type / reusable constraint value =
  central domain contracts
  less repeated Field metadata
  reusable validation and serialization behavior
  reusable JSON Schema definitions through named aliases
  safe recursive type aliases
  third-party type integration without monkeypatching
  custom generic class support
  TypeAdapter validation for scalar/container aliases
  schema-friendly public API contracts
  lower boilerplate with GetPydanticSchema where appropriate
```

Operationally: encode reusable **local, deterministic, structural contracts** as `Annotated` aliases and named type aliases; reserve direct core-schema hooks for custom classes, third-party types, or generic containers that cannot be modeled with built-in annotations, validators, serializers, and `WithJsonSchema`.

[S14-1]: https://docs.pydantic.dev/latest/concepts/types/ "Types | Pydantic Docs"
[S14-2]: https://docs.pydantic.dev/latest/concepts/types/?utm_source=chatgpt.com "Types | Pydantic Docs"
[S14-3]: https://docs.pydantic.dev/latest/concepts/validators/ "Validators | Pydantic Docs"
[S14-4]: https://docs.pydantic.dev/latest/api/functional_serializers/ "Functional Serializers | Pydantic Docs"


# Pydantic Advanced — 15) `pydantic-core` and internal architecture — Pydantic v2 deep dive

Style target: dense, sectioned, agent-oriented reference. 

Pydantic v2 is split into two cooperating packages: `pydantic`, which handles model/type definition and schema generation at the Python layer, and `pydantic-core`, a Rust-backed engine that performs validation and serialization from a structured “core schema.” The internals docs explicitly state that this split was introduced partly for validation/serialization performance, with the tradeoff of reduced customization/extendibility of internal logic; the same page notes that the internals section is partly contributor-targeted. ([Pydantic Docs][S15-1])

---

## 15.0 Architecture map

```text
Python user code
  → annotations / BaseModel / Field / ConfigDict / validators / serializers
  → pydantic Python package
      model metaclass
      annotation resolution
      schema generation
      public API surface
  → core schema
      structured Python dict / TypedDict-like schema description
  → pydantic-core
      Rust validation engine
      Rust serialization engine
  → validated Python objects
  → serialized Python / JSON output
  → JSON Schema via GenerateJsonSchema
```

Responsibilities:

```text
pydantic:
  model definition
  annotation collection
  model_config processing
  field metadata collection
  validator/serializer collection
  generic/private/class-var handling
  core schema generation orchestration
  JSON Schema generation API
  public Python API

pydantic-core:
  SchemaValidator
  SchemaSerializer
  validation execution
  serialization execution
  error construction
  low-level schema primitives
  JSON parsing/serialization helpers
```

The architecture docs divide Pydantic usage into model definition in `pydantic` and model validation/serialization in `pydantic-core`; when a `BaseModel` is defined, the metaclass collects annotations, `model_config`, validators/serializers, private attributes, class variables, and generic information. ([Pydantic Docs][S15-1])

---

## 15.1 Why v2 split into `pydantic-core`

```text
v1-style architecture:
  Python-heavy validation
  broad Python customization
  slower hot-path validation/serialization

v2 architecture:
  Python schema definition
  Rust execution engine
  structured core schema boundary
  faster validation/serialization
  narrower low-level customization surface
```

Pydantic’s internals docs state that v2 moved part of the codebase to Rust in the separate `pydantic-core` package partly to improve validation and serialization performance, at the cost of limited customization and extendibility of internal logic. The same docs state that validation/serialization in v2 use the previously built core schema and provide a “5 to 20” performance increase compared with Pydantic v1. ([Pydantic Docs][S15-1])

Agent rule:

```text
Application code:
  stay on pydantic public APIs.

Infrastructure/library code:
  use pydantic-core only when custom type/schema integration requires it.

Contributor/internal code:
  inspect pydantic internals, GenerateSchema, core_schema, SchemaValidator, SchemaSerializer.
```

---

## 15.2 Core schema: what it is

Core schema is the intermediate representation between the Python authoring layer and the Rust execution layer:

```text
CoreSchema =
  structured serializable Python dictionary
  represented by TypedDict definitions
  required "type" key
  type-specific keys
  optional metadata
  optional ref
  optional serialization schema
```

The architecture docs define core schema as a structured, serializable Python dictionary represented with `TypedDict` definitions; every core schema has a required `type` key, plus extra keys depending on that type. The docs also state that core schema is the main data structure used for communication between `pydantic` and `pydantic-core`. ([Pydantic Docs][S15-1])

Example conceptual schema:

```python
{
    "type": "int",
    "gt": 0,
    "strict": True,
}
```

Low-level constructor equivalent:

```python
from pydantic_core import core_schema

schema = core_schema.int_schema(gt=0, strict=True)
```

The `core_schema.int_schema(...)` helper accepts constraints such as `multiple_of`, `le`, `ge`, `lt`, `gt`, and `strict`, plus `ref`, `metadata`, and `serialization`. ([Pydantic Docs][S15-2])

---

## 15.3 Core schema: fixed type universe

```text
You can compose supported core schema types.
You cannot invent arbitrary core schema "type" values.
pydantic-core must understand every schema type.
```

Pydantic’s internals docs explicitly say it is not possible to define a custom core schema type because core schemas must be understood by `pydantic-core`; only a fixed set of core schema types is supported, and the low-level `GenerateSchema` class is not truly exposed and documented for general public customization. ([Pydantic Docs][S15-1])

Agent rule:

```text
Allowed:
  build/return supported core_schema.* primitives
  wrap existing schemas
  attach supported serializer schemas
  attach metadata that pydantic-core ignores but JSON Schema tools may use

Not allowed:
  schema = {"type": "my-new-validator"}  # pydantic-core will not understand it
```

---

## 15.4 How `BaseModel` generates core schema

```python
from pydantic import BaseModel, Field

class Model(BaseModel):
    foo: bool = Field(strict=True)
```

Conceptual field core schema:

```python
{
    "type": "bool",
    "strict": True,
}
```

At class-definition time, `pydantic` collects the model definition and builds a core schema; for a `BaseModel`, the schema is set on `__pydantic_core_schema__`. The internals docs show a `bool` field with `Field(strict=True)` mapping to a core schema like `{"type": "bool", "strict": True}`. ([Pydantic Docs][S15-1])

Inspection pattern:

```python
from pprint import pprint

pprint(Model.__pydantic_core_schema__)
```

Agent warning:

```text
Inspecting __pydantic_core_schema__ is useful for diagnostics.
Testing exact __pydantic_core_schema__ shape is brittle.
Mutating __pydantic_core_schema__ is not an application-level API.
```

---

## 15.5 Core schema and validators

High-level validators compile into function-wrapper schema nodes:

```python
from typing import Annotated
from pydantic import AfterValidator

def check(v: int) -> int:
    if v < 0:
        raise ValueError("negative")
    return v

Positive = Annotated[int, AfterValidator(check)]
```

Low-level conceptual equivalent:

```python
from pydantic_core import core_schema

schema = core_schema.no_info_after_validator_function(
    check,
    core_schema.int_schema(),
)
```

`pydantic_core.core_schema` exposes function-schema helpers for before, after, and wrap validators. For example, `no_info_before_validator_function(...)` calls a validator before validating against an inner schema; `no_info_after_validator_function(...)` calls a validator after the inner schema is validated; `no_info_wrap_validator_function(...)` provides a handler that can call inner validation, middleware-style. ([Pydantic Docs][S15-2])

Mapping:

```text
BeforeValidator(fn):
  core_schema.no_info_before_validator_function(fn, inner_schema)
  or with_info_before_validator_function(...)

AfterValidator(fn):
  core_schema.no_info_after_validator_function(fn, inner_schema)
  or with_info_after_validator_function(...)

WrapValidator(fn):
  core_schema.no_info_wrap_validator_function(fn, inner_schema)
  or with_info_wrap_validator_function(...)

PlainValidator(fn):
  plain function schema, no inner validation after returned value
```

Agent rule:

```text
Use high-level validators unless:
  custom type integration requires low-level core_schema composition
  you are building reusable infrastructure
  you need json_or_python_schema, union_schema, chain_schema, or custom serializer schemas
```

---

## 15.6 Core schema and serializers

High-level serializer:

```python
from pydantic import BaseModel, field_serializer

class Model(BaseModel):
    foo: bool

    @field_serializer("foo", mode="plain")
    def serialize_foo(self, value: bool) -> int:
        return int(value)
```

Conceptual serialization core schema fragment:

```python
{
    "type": "function-plain",
    "function": Model.serialize_foo,
    "is_field_serializer": True,
    "info_arg": False,
    "return_schema": {"type": "int"},
}
```

The internals docs show that serialization logic is also defined in core schema through a `serialization` key; a `field_serializer(..., mode="plain")` maps to a `function-plain` serialization schema including the serializer function, field-serializer flag, `info_arg`, and `return_schema`. ([Pydantic Docs][S15-1])

Low-level serializer helpers:

```python
from pydantic_core import core_schema

ser = core_schema.plain_serializer_function_ser_schema(
    lambda v: str(v),
    return_schema=core_schema.str_schema(),
)
```

`wrap_serializer_function_ser_schema(...)` creates a wrap-function serialization schema and accepts parameters such as `function`, `schema`, `return_schema`, and `when_used`; field/general serializer metadata such as `is_field_serializer` and `info_arg` can also be provided. ([Pydantic Docs][S15-2])

---

## 15.7 `SchemaValidator`

### Purpose

```text
SchemaValidator:
  Python wrapper around Rust validation logic
  owns a CombinedValidator tree
  validates Python / JSON / string inputs
  supports assignment validation
  returns validated Python objects
  raises pydantic_core.ValidationError on failure
```

The `pydantic-core` API docs define `SchemaValidator` as the Python wrapper for Rust validation logic; internally it owns one `CombinedValidator`, which may own more `CombinedValidator`s composing the full schema validator. ([Pydantic Docs][S15-3])

### Low-level example

```python
from pydantic_core import SchemaValidator, core_schema

schema = core_schema.int_schema(multiple_of=2, ge=2, le=6)
validator = SchemaValidator(schema)

assert validator.validate_python("4") == 4
```

The `int_schema(...)` docs show exactly this pattern: create a core schema, pass it to `SchemaValidator`, and validate Python input such as the string `"4"` into integer `4`. ([Pydantic Docs][S15-2])

### Main methods

```text
validate_python(input, strict=None, extra=None, from_attributes=None, context=None, self_instance=None, allow_partial=False, by_alias=None, by_name=None)
  validate Python object

validate_json(input: str | bytes | bytearray, ...)
  validate raw JSON directly

validate_strings(input, ...)
  validate string-like input for URL/query/etc. scenarios

validate_assignment(obj, field_name, field_value, ...)
  validate assignment to a model field

isinstance_python(input, ...)
  boolean validation check without ValidationError

get_default_value(strict=None, context=None)
  compute schema default value, including default validation
```

The `validate_python(...)` method validates a Python object and accepts strictness, extra behavior, attribute extraction, context, `self_instance`, partial validation, alias, and name flags; `validate_json(...)` validates raw JSON directly and is documented as significantly faster than `validate_python(json.loads(...))` because it avoids intermediate Python objects; `validate_strings(...)` is for string-but-not-JSON inputs such as URL fragments or query parameters; `validate_assignment(...)` validates assignment to a model field. ([Pydantic Docs][S15-3])

Agent rule:

```text
Normal code:
  Model.model_validate(...)
  TypeAdapter(...).validate_python(...)
  Model.model_validate_json(...)

Low-level code:
  SchemaValidator(core_schema).validate_python(...)
```

---

## 15.8 `SchemaSerializer`

### Purpose

```text
SchemaSerializer:
  Python wrapper around Rust serialization logic
  owns CombinedSerializer tree
  serializes Python objects to Python or JSON
  applies include/exclude/filtering/alias/context policies
```

The `pydantic-core` docs define `SchemaSerializer` as the Python wrapper for Rust serialization logic; internally it owns a `CombinedSerializer` tree that composes the full serializer. ([Pydantic Docs][S15-3])

### Low-level example

```python
from pydantic_core import SchemaSerializer, core_schema

serializer = SchemaSerializer(core_schema.list_schema(core_schema.int_schema()))

assert serializer.to_python([1, 2, 3]) == [1, 2, 3]
assert serializer.to_json([1, 2, 3]) == b"[1,2,3]"
```

### Main methods

```text
to_python(value, mode=None, include=None, exclude=None, by_alias=None, exclude_unset=False, exclude_defaults=False, exclude_none=False, exclude_computed_fields=False, round_trip=False, warnings=True, fallback=None, serialize_as_any=False, polymorphic_serialization=None, context=None)
  serialize to Python object

to_json(value, indent=None, ensure_ascii=False, include=None, exclude=None, by_alias=None, exclude_unset=False, exclude_defaults=False, exclude_none=False, exclude_computed_fields=False, round_trip=False, warnings=True, fallback=None, serialize_as_any=False, polymorphic_serialization=None, context=None)
  serialize to JSON bytes
```

`SchemaSerializer.to_python(...)` serializes/marshals a Python object into another Python object and supports Python vs JSON mode, include/exclude, aliases, unset/default/none exclusion, computed-field exclusion, round-trip behavior, serialization warnings, fallback, duck-typing, polymorphic serialization, and context; in JSON mode, all values are converted to JSON-compatible types. `SchemaSerializer.to_json(...)` serializes to JSON bytes. ([Pydantic Docs][S15-3])

Agent rule:

```text
BaseModel.model_dump(...)
  → high-level wrapper around SchemaSerializer.to_python(...)

BaseModel.model_dump_json(...)
  → high-level wrapper around SchemaSerializer.to_json(...), with str result

TypeAdapter.dump_python(...)
  → high-level wrapper around SchemaSerializer.to_python(...)

TypeAdapter.dump_json(...)
  → high-level wrapper around SchemaSerializer.to_json(...), bytes result
```

---

## 15.9 Relationship graph: annotations → core schema → validation

```text
annotation:
  x: Annotated[int, Field(gt=0), AfterValidator(fn)]

pydantic:
  resolve annotation
  collect Field metadata
  collect validators
  build core schema

core schema:
  function-after(
    int_schema(gt=0)
  )

pydantic-core:
  SchemaValidator.validate_python(input)

output:
  int value or ValidationError
```

Example:

```python
from typing import Annotated
from pydantic import AfterValidator, Field, TypeAdapter

def double(v: int) -> int:
    return v * 2

T = Annotated[int, Field(gt=0), AfterValidator(double)]
ta = TypeAdapter(T)

assert ta.validate_python("2") == 4
```

Pydantic’s internals docs describe the wrapper pattern for `Annotated` metadata: each `__get_pydantic_core_schema__` hook receives a `GetCoreSchemaHandler`, calls it to get the next schema, and can modify/wrap the schema; the example shows `MyStrict` and `MyGt` successively adding `strict` and `gt` behavior around an `int` schema. ([Pydantic Docs][S15-1])

---

## 15.10 Relationship graph: annotations → core schema → serialization

```text
annotation:
  x: Annotated[Decimal, PlainSerializer(str, when_used="json")]

pydantic:
  build validation schema for Decimal
  attach serialization schema

pydantic-core:
  SchemaSerializer.to_python(value, mode="python")
  SchemaSerializer.to_python(value, mode="json")
  SchemaSerializer.to_json(value)

output:
  Python Decimal OR JSON string/bytes depending mode
```

Serialization context and options are forwarded into `pydantic-core` through serializer information. The `core_schema` API exposes `SerializationInfo` attributes such as `include`, `exclude`, `context`, `mode`, `by_alias`, `exclude_unset`, `exclude_defaults`, `exclude_none`, `round_trip`, `serialize_as_any`, and `polymorphic_serialization`; `FieldSerializationInfo` adds the current `field_name`. ([Pydantic Docs][S15-2])

---

## 15.11 Relationship graph: core schema → JSON Schema

```text
core schema
  → GenerateJsonSchema.generate(core_schema)
  → JSON Schema dictionary
```

The internals docs state that JSON Schema generation is handled by `GenerateJsonSchema`; its `generate` method is given the model’s core schema. The same page notes that serializer core schema may contain a `return_schema`, which is used to generate corresponding JSON Schema. ([Pydantic Docs][S15-1])

Example:

```python
from pydantic import BaseModel, Field

class Model(BaseModel):
    x: int = Field(gt=0)

schema = Model.model_json_schema()
```

Conceptual mapping:

```text
core_schema.int_schema(gt=0)
  → JSON Schema:
      type: integer
      exclusiveMinimum: 0
```

Agent rule:

```text
Validation behavior and JSON Schema are related through core schema.
They are not identical.
Custom validators may need json_schema_input_type / WithJsonSchema / __get_pydantic_json_schema__ to keep schema honest.
```

---

## 15.12 Public high-level APIs vs internal low-level APIs

### Public high-level

```text
BaseModel
Field
ConfigDict
TypeAdapter
RootModel
pydantic.dataclasses.dataclass
field_validator / model_validator
field_serializer / model_serializer
BeforeValidator / AfterValidator / PlainValidator / WrapValidator
PlainSerializer / WrapSerializer
WithJsonSchema
GetPydanticSchema
__get_pydantic_core_schema__
__get_pydantic_json_schema__
```

### Lower-level / contributor-facing

```text
pydantic_core.SchemaValidator
pydantic_core.SchemaSerializer
pydantic_core.core_schema.*
__pydantic_core_schema__
GenerateSchema
GenerateJsonSchema subclassing
annotation resolution internals
model metaclass internals
```

Pydantic’s custom-types docs explicitly recap that high-level hooks such as `AfterValidator` and `Field` should be used when possible; direct `pydantic-core` customization is possible through `GetPydanticSchema`, marker classes implementing `__get_pydantic_core_schema__`, or implementing the hook on a custom type itself. The same docs warn that `pydantic-core` APIs are newer and are among the areas more likely to be tweaked in the future. ([Pydantic Docs][S15-4])

Agent rule:

```text
Default:
  use public high-level APIs.

Escalate to pydantic-core only when:
  a custom type cannot be expressed with Annotated validators/serializers
  third-party type integration needs custom parse/serialize/schema behavior
  generic arbitrary class integration needs type-parameter-aware validation
  framework/tooling needs direct validators/serializers
```

---

## 15.13 Customization boundaries: what can be customized

### High-level customizable

```text
field constraints:
  Field(gt=..., min_length=..., pattern=...)

strictness / config:
  ConfigDict(strict=True), Field(strict=True)

validation:
  field_validator
  model_validator
  BeforeValidator / AfterValidator / PlainValidator / WrapValidator

serialization:
  field_serializer
  model_serializer
  PlainSerializer / WrapSerializer

JSON Schema:
  Field(json_schema_extra=...)
  WithJsonSchema
  __get_pydantic_json_schema__
  GenerateJsonSchema subclass

custom type behavior:
  GetPydanticSchema
  Annotated marker class with __get_pydantic_core_schema__
  custom class __get_pydantic_core_schema__
```

### Low-level customizable through supported core-schema nodes

```text
chain_schema
union_schema
json_or_python_schema
is_instance_schema
typed_dict_schema
model_schema
dataclass_schema
no_info_before_validator_function
no_info_after_validator_function
no_info_wrap_validator_function
plain_serializer_function_ser_schema
wrap_serializer_function_ser_schema
lax_or_strict_schema
definitions_schema
definition_reference_schema
```

The `core_schema` API documents these composition helpers, including validator function schemas, `lax_or_strict_schema`, `json_or_python_schema`, `union_schema`, and definitions/reference schemas. ([Pydantic Docs][S15-2])

---

## 15.14 Customization boundaries: what cannot be customized

```text
Cannot:
  invent arbitrary core schema type names
  make pydantic-core understand schemas outside supported fixed set
  rely on undocumented GenerateSchema internals as stable API
  mutate internal model core schema safely after class creation
  assume exact core schema dict shape is public contract
  intercept every Rust-side internal operation with Python callbacks
```

Reason:

```text
pydantic-core is Rust-side execution.
It only understands fixed supported core schema node types.
The Python layer can build/wrap/combine schemas, not redefine the Rust engine's primitive vocabulary.
```

Pydantic’s internals docs are explicit that custom core schema types are not possible because schemas must be understood by `pydantic-core`; only a fixed number of core schema types is supported. The same page states that this is why `GenerateSchema` is not truly exposed and documented as a public surface. ([Pydantic Docs][S15-1])

---

## 15.15 Third-party type architecture pattern

```python
from typing import Annotated, Any
from pydantic import BaseModel, GetCoreSchemaHandler, GetJsonSchemaHandler
from pydantic.json_schema import JsonSchemaValue
from pydantic_core import core_schema

class ThirdPartyType:
    x: int

    def __init__(self) -> None:
        self.x = 0

class _ThirdPartyTypePydanticAnnotation:
    @classmethod
    def __get_pydantic_core_schema__(
        cls,
        _source_type: Any,
        _handler: GetCoreSchemaHandler,
    ) -> core_schema.CoreSchema:
        def validate_from_int(value: int) -> ThirdPartyType:
            result = ThirdPartyType()
            result.x = value
            return result

        from_int_schema = core_schema.chain_schema(
            [
                core_schema.int_schema(),
                core_schema.no_info_plain_validator_function(validate_from_int),
            ]
        )

        return core_schema.json_or_python_schema(
            json_schema=from_int_schema,
            python_schema=core_schema.union_schema(
                [
                    core_schema.is_instance_schema(ThirdPartyType),
                    from_int_schema,
                ]
            ),
            serialization=core_schema.plain_serializer_function_ser_schema(
                lambda instance: instance.x
            ),
        )

    @classmethod
    def __get_pydantic_json_schema__(
        cls,
        _core_schema: core_schema.CoreSchema,
        handler: GetJsonSchemaHandler,
    ) -> JsonSchemaValue:
        return handler(core_schema.int_schema())

PydanticThirdPartyType = Annotated[ThirdPartyType, _ThirdPartyTypePydanticAnnotation]

class Model(BaseModel):
    third_party_type: PydanticThirdPartyType
```

This pattern accepts integers as third-party instances, accepts existing third-party instances, serializes them back to integers, and emits integer JSON Schema. Pydantic’s docs present this exact style for third-party libraries such as Pandas or NumPy types, using an `Annotated` wrapper marker rather than requiring modification of the third-party class itself. ([Pydantic Docs][S15-4])

Agent rule:

```text
Third-party integration:
  Annotated[ExternalType, Marker]
  marker implements core schema
  marker implements JSON schema if public
  do not monkeypatch third-party class
  validate existing instance first if common
  define JSON/Python schema paths separately when wire/native shapes differ
```

---

## 15.16 `GetPydanticSchema` as lower-boilerplate middleware

```python
from typing import Annotated
from pydantic import BaseModel, GetPydanticSchema
from pydantic_core import core_schema

class Model(BaseModel):
    y: Annotated[
        str,
        GetPydanticSchema(
            lambda tp, handler: core_schema.no_info_after_validator_function(
                lambda x: x * 2,
                handler(tp),
            )
        ),
    ]

assert Model(y="ab").y == "abab"
```

`GetPydanticSchema` exists to reduce boilerplate for simple custom schema middleware; Pydantic’s docs show it wrapping the handler-generated schema with an after-validator function, and summarize it as a simpler alternative to marker classes when customization is small. ([Pydantic Docs][S15-4])

Agent rule:

```text
Use GetPydanticSchema when:
  customization is short
  schema hook is local
  no reusable marker class needed

Use marker class when:
  reusable behavior
  parameters
  JSON Schema customization
  clarity/testing
```

---

## 15.17 Custom generic class internals

```text
source_type:
  actual annotation being processed, e.g. Owner[Car]

cls:
  generic class object, e.g. Owner

get_args(source_type):
  extract type parameters

handler.generate_schema(T):
  generate schema for generic parameter independent of current Annotated metadata context
```

Pydantic docs call custom generic classes an advanced technique and explain that `source_type` may differ from `cls`; type parameters should be extracted from `source_type`, and schemas for type parameters should be generated using `handler.generate_schema(...)`, not `handler(...)`, to avoid unwanted influence from the current metadata context. If a generic class implements `__get_pydantic_core_schema__`, `arbitrary_types_allowed` is not required. ([Pydantic Docs][S15-4])

Agent rule:

```text
Custom generic class:
  inspect source_type
  use get_origin/get_args
  generate param schemas with handler.generate_schema
  define python path for existing instances
  define JSON path if wire representation differs
```

---

## 15.18 Core schema definitions and `$ref`-like reuse

Low-level definitions:

```python
from pydantic_core import core_schema

schema = core_schema.definitions_schema(
    schema=core_schema.definition_reference_schema("MyType"),
    definitions=[
        core_schema.int_schema(ref="MyType"),
    ],
)
```

Core schema supports definitions and definition references. The `definitions_schema(...)` helper builds a schema containing an inner schema and a list of definitions that can be referenced by the inner schema; `definition_reference_schema(...)` references a schema by `schema_ref`. ([Pydantic Docs][S15-2])

Agent rule:

```text
High-level schema reuse:
  named type aliases
  BaseModel $defs
  TypeAdapter.json_schema()

Low-level definitions_schema:
  framework/internal/custom-type work only
```

---

## 15.19 JSON vs Python core-schema paths

```python
from pydantic_core import core_schema

schema = core_schema.json_or_python_schema(
    json_schema=core_schema.int_schema(),
    python_schema=core_schema.union_schema([
        core_schema.is_instance_schema(MyType),
        core_schema.int_schema(),
    ]),
    serialization=core_schema.plain_serializer_function_ser_schema(lambda v: v.x),
)
```

`json_or_python_schema(...)` chooses one core schema for JSON inputs and another for Python inputs; its docs define `json_schema` as the schema for JSON inputs and `python_schema` as the schema for Python inputs. This is central for third-party/native object integrations where JSON wire shape differs from Python runtime shape. ([Pydantic Docs][S15-2])

Agent rule:

```text
Use json_or_python_schema when:
  JSON accepts primitive/wire representation
  Python accepts existing object instance
  serialization maps object back to wire representation
```

---

## 15.20 Strict/lax core schema paths

```python
schema = core_schema.lax_or_strict_schema(
    lax_schema=core_schema.int_schema(),
    strict_schema=core_schema.int_schema(strict=True),
    strict=None,
)
```

`lax_or_strict_schema(...)` switches between a lax and strict schema; its parameters include `lax_schema`, `strict_schema`, and `strict`, which controls whether the strict schema should be used. ([Pydantic Docs][S15-2])

Agent rule:

```text
Prefer high-level strict:
  Field(strict=True)
  Strict()
  ConfigDict(strict=True)

Use lax_or_strict_schema only when:
  implementing low-level custom type behavior
  strict and lax runtime behavior need distinct custom schemas
```

---

## 15.21 How validation context flows

```text
Model.model_validate(..., context=ctx)
  → SchemaValidator.validate_python(..., context=ctx)
  → core_schema functional validators
  → ValidationInfo.context
```

`SchemaValidator.validate_python(...)` accepts a `context` argument, and the docs state that it is passed to functional validators as `info.context`. `validate_json(...)` and `validate_strings(...)` expose the same kind of context parameter. ([Pydantic Docs][S15-3])

Low-level info object:

```text
ValidationInfo.context
ValidationInfo.config
ValidationInfo.mode
ValidationInfo.data
ValidationInfo.field_name
```

The `core_schema` API documents `ValidationInfo` as extra data used during validation, with attributes including `context` and `config`. ([Pydantic Docs][S15-2])

---

## 15.22 How serialization context flows

```text
model.model_dump(context=ctx)
  → SchemaSerializer.to_python(..., context=ctx)
  → functional serializers
  → SerializationInfo.context
```

`SchemaSerializer.to_python(...)` and `to_json(...)` accept a `context` argument, and the docs state it is passed to functional serializers as `info.context`; `SerializationInfo` includes `context`, `mode`, include/exclude, alias, exclusion flags, `round_trip`, `serialize_as_any`, and polymorphic serialization flags. ([Pydantic Docs][S15-3])

Agent rule:

```text
Context is a parameterization channel.
Do not use context to smuggle service-layer side effects into validators/serializers.
```

---

## 15.23 Contributor-level internals vs public API

### Contributor-level internals

```text
GenerateSchema
model metaclass implementation
annotation resolution internals
core schema generation order
__pydantic_core_schema__ exact layout
CombinedValidator / CombinedSerializer internals
exact core schema dict shape
private pydantic._internal modules
```

### Stable-ish public surfaces

```text
BaseModel public methods
TypeAdapter public methods
ConfigDict keys
Field parameters
functional validator/serializer APIs
core_schema helper functions
SchemaValidator / SchemaSerializer public methods
custom hook method signatures
```

Caution:

```text
Even public low-level core_schema APIs are lower stability than high-level Pydantic APIs.
Prefer high-level constructs unless low-level behavior is required.
```

Pydantic’s internals docs are partly targeted to contributors, and the custom-types docs warn that `pydantic-core` is one of the areas most likely to be tweaked in the future; Pydantic recommends built-in constructs such as `annotated-types`, `Field`, and functional validators where possible. ([Pydantic Docs][S15-1])

---

## 15.24 Direct `pydantic-core` usage patterns

### Good: test custom low-level schema

```python
from pydantic_core import SchemaValidator, SchemaSerializer, core_schema

schema = core_schema.int_schema(gt=0)
validator = SchemaValidator(schema)
serializer = SchemaSerializer(schema)

assert validator.validate_python("3") == 3
assert serializer.to_json(3) == b"3"
```

### Good: custom third-party integration

```python
PydanticExternal = Annotated[External, ExternalMarker]
```

### Bad: bypassing Pydantic model for normal DTOs

```python
validator = SchemaValidator(core_schema.typed_dict_schema(...))
```

Prefer:

```python
class DTO(BaseModel):
    ...
```

Agent rule:

```text
Direct SchemaValidator:
  infrastructure tests
  benchmarking custom core schemas
  framework adapters
  advanced custom type work

BaseModel / TypeAdapter:
  normal application validation
```

---

## 15.25 Deployment decision matrix

```text
Need                                               Recommended layer
--------------------------------------------------------------------------------
normal model validation                             BaseModel / TypeAdapter
custom scalar constraint                            Annotated + Field / annotated-types
custom field validation                             field_validator / AfterValidator
custom model invariant                              model_validator
custom serialization                                field_serializer / PlainSerializer / WrapSerializer
simple JSON Schema override                         WithJsonSchema / json_schema_extra
third-party type parse/serialize/schema             Annotated marker + core_schema hooks
owned custom class parse/serialize/schema           __get_pydantic_core_schema__ on class
generic arbitrary class                             __get_pydantic_core_schema__ + get_args/source_type
short local schema middleware                       GetPydanticSchema
global JSON Schema policy                           GenerateJsonSchema subclass
raw Rust-engine validator needed                    SchemaValidator
raw Rust-engine serializer needed                   SchemaSerializer
new primitive schema behavior                       not supported unless pydantic-core supports it
```

---

## 15.26 Anti-patterns

### Inventing a schema type

```python
schema = {"type": "my_custom_schema"}  # invalid unsupported core schema type
```

Correct:

```python
schema = core_schema.no_info_after_validator_function(
    validate,
    core_schema.str_schema(),
)
```

---

### Testing exact internal core schema

```python
assert Model.__pydantic_core_schema__ == {...}  # brittle
```

Better:

```python
assert Model.model_validate({"x": "1"}).x == 1
assert Model(x=1).model_dump(mode="json") == {"x": 1}
assert Model.model_json_schema()["properties"]["x"]["type"] == "integer"
```

---

### Direct `SchemaValidator` for application DTO

```python
validator = SchemaValidator(core_schema.typed_dict_schema(...))
```

Better:

```python
class Payload(BaseModel):
    ...
```

or:

```python
PayloadAdapter = TypeAdapter(PayloadTypedDict)
```

---

### Low-level hooks for simple constraints

```python
class PositiveInt(int):
    @classmethod
    def __get_pydantic_core_schema__(...):
        ...
```

Better:

```python
PositiveInt = Annotated[int, Field(gt=0)]
```

Pydantic recommends high-level hooks such as `Field` and `AfterValidator` where possible. ([Pydantic Docs][S15-4])

---

### Mutating handler-generated schema in fragile ways

```python
schema = handler(source)
schema["type"] = "something_else"  # risky
```

Better:

```python
return core_schema.no_info_after_validator_function(validate, handler(source))
```

Mutation can be valid for simple, understood schema edits, but wrapping is usually clearer and less brittle.

---

## 15.27 Test matrix

### Low-level validator sanity

```python
from pydantic_core import SchemaValidator, core_schema, ValidationError

def test_schema_validator_int():
    schema = core_schema.int_schema(gt=0)
    v = SchemaValidator(schema)

    assert v.validate_python("1") == 1

    with pytest.raises(ValidationError):
        v.validate_python(0)
```

### JSON vs Python schema paths

```python
def test_json_or_python_schema():
    class External:
        def __init__(self, x: int) -> None:
            self.x = x

    def from_int(x: int) -> External:
        return External(x)

    schema = core_schema.json_or_python_schema(
        json_schema=core_schema.chain_schema([
            core_schema.int_schema(),
            core_schema.no_info_plain_validator_function(from_int),
        ]),
        python_schema=core_schema.union_schema([
            core_schema.is_instance_schema(External),
            core_schema.chain_schema([
                core_schema.int_schema(),
                core_schema.no_info_plain_validator_function(from_int),
            ]),
        ]),
        serialization=core_schema.plain_serializer_function_ser_schema(lambda v: v.x),
    )

    v = SchemaValidator(schema)
    s = SchemaSerializer(schema)

    assert v.validate_json(b"1").x == 1
    assert v.validate_python(External(2)).x == 2
    assert s.to_json(External(3)) == b"3"
```

### High-level behavior instead of core schema snapshot

```python
def test_model_behavior_not_core_schema_shape():
    class M(BaseModel):
        x: int = Field(gt=0)

    assert M.model_validate({"x": "1"}).x == 1

    with pytest.raises(ValidationError):
        M.model_validate({"x": 0})

    assert M.model_dump(M(x=1)) if False else M(x=1).model_dump() == {"x": 1}
    assert M.model_json_schema()["properties"]["x"]["exclusiveMinimum"] == 0
```

### Custom hook behavior

```python
def test_custom_type_hook_contract():
    class Username(str):
        @classmethod
        def __get_pydantic_core_schema__(cls, source, handler):
            return core_schema.no_info_after_validator_function(cls, handler(str))

    class M(BaseModel):
        username: Username

    m = M(username="ada")
    assert isinstance(m.username, Username)
    assert m.username == "ada"
```

---

## 15.28 Agent checklist

```text
[ ] Treat pydantic as schema-authoring layer.
[ ] Treat pydantic-core as validation/serialization execution layer.
[ ] Understand core schema as the communication contract between them.
[ ] Do not invent custom core schema type names.
[ ] Do not mutate __pydantic_core_schema__ in application code.
[ ] Prefer Field / annotated-types / validators / serializers before core hooks.
[ ] Use GetPydanticSchema for small local low-level customizations.
[ ] Use Annotated marker classes for third-party type integration.
[ ] Use __get_pydantic_core_schema__ on custom classes only when the class owns behavior.
[ ] Implement __get_pydantic_json_schema__ when public schema differs from default.
[ ] Use json_or_python_schema when Python/native and JSON/wire inputs differ.
[ ] Use handler.generate_schema(...) for generic type parameters.
[ ] Use SchemaValidator / SchemaSerializer only for infrastructure, tests, benchmarks, or framework integration.
[ ] Test behavior: validate, dump, dump_json, schema.
[ ] Avoid exact internal core-schema snapshots.
[ ] Remember internals docs are partly contributor-targeted.
```

---

## 15.29 Value case

```text
pydantic-core architecture value =
  Rust-backed validation execution
  Rust-backed serialization execution
  unified core schema representation
  faster v2 hot path
  high-level Python authoring API preserved
  validation and serialization defined from one schema graph
  JSON Schema generated from same core schema source
  clear extension hooks for custom types
  sharp boundary between stable public modeling APIs and contributor-level internals
```

Operationally: use Pydantic’s high-level APIs for application schemas, reach into `pydantic-core` only for custom type infrastructure or framework work, treat core schema as a fixed-vocabulary execution plan rather than an open plugin language, and verify custom integrations through behavior-level tests rather than internal schema snapshots.

[S15-1]: https://docs.pydantic.dev/latest/internals/architecture/ "Architecture | Pydantic Docs"
[S15-2]: https://docs.pydantic.dev/latest/api/pydantic_core_schema/ "pydantic_core.core_schema | Pydantic Docs"
[S15-3]: https://docs.pydantic.dev/latest/api/pydantic_core/ "pydantic_core | Pydantic Docs"
[S15-4]: https://docs.pydantic.dev/latest/concepts/types/ "Types | Pydantic Docs"


# Pydantic Advanced — 16) Error model and diagnostics — Pydantic v2 deep dive

Style target: dense, sectioned, agent-oriented technical reference. 

Pydantic raises `ValidationError` when validated input data cannot be converted into the declared schema; the exception aggregates **all discovered validation errors** and exposes structured error records through `.errors()`, `.json()`, `.error_count()`, and `str(e)`. Validation code should generally raise `ValueError`, `AssertionError`, or `PydanticCustomError`, not instantiate `ValidationError` directly. ([Pydantic Docs][S16-1])

---

## 16.0 Error model mental map

```text
raw input
  → pydantic-core validation
  → one of:

  success:
    validated object

  data validation failure:
    ValidationError
      .errors()
      .json()
      .error_count()
      str(e)

  developer / schema / API misuse:
    PydanticUserError
    PydanticSchemaGenerationError
    SchemaError
    TypeError from bad validators
    other usage-time exceptions
```

Operational distinction:

```text
ValidationError:
  user/input/data problem
  convert to 400/422/API validation response
  expected at trust boundaries

PydanticUserError / schema errors:
  developer/programming/configuration problem
  fail fast
  fix code/schema/config
  should not be converted into normal user validation response
```

Pydantic separates validation errors from usage errors: validation errors happen during data validation, while usage errors occur when Pydantic itself is used incorrectly, such as undefined forward references, invalid validator declarations, invalid discriminator configuration, schema-generation misuse, or invalid `TypeAdapter` config usage. ([Pydantic Docs][S16-1])

---

## 16.1 `ValidationError` anatomy

### Public methods

```python
try:
    Model.model_validate(data)
except ValidationError as exc:
    exc.error_count()
    exc.errors()
    exc.json()
    str(exc)
```

Surface:

```text
error_count() -> int
  number of validation errors

errors(
  include_url=True,
  include_context=True,
  include_input=True,
) -> list[ErrorDetails]
  structured list of error dictionaries

json(
  indent=None,
  include_url=True,
  include_context=True,
  include_input=True,
) -> str
  JSON string representation of errors()

str(exc)
  human-readable display
```

`ValidationError.errors()` returns a list of `ErrorDetails`; `.json()` returns the same error list as JSON; both methods can exclude URLs, context, and input values through `include_url`, `include_context`, and `include_input`. ([Pydantic Docs][S16-2])

---

## 16.2 `ErrorDetails` fields

```python
{
    "type": "greater_than",
    "loc": ("age",),
    "msg": "Input should be greater than 0",
    "input": -1,
    "ctx": {"gt": 0},
    "url": "https://errors.pydantic.dev/2/v/greater_than",
}
```

Field semantics:

```text
type:
  machine-readable error identifier
  stable discriminator for programmatic handling

loc:
  tuple/list path to the failing input location
  field names, aliases, list indexes, union branch labels, discriminator tags

msg:
  human-readable message
  useful for display, not stable enough for logic

input:
  offending input at this location
  can be omitted by errors(include_input=False)

ctx:
  optional rendering/context data
  e.g. {"gt": 42}, {"expected_schemes": "'http' or 'https'"}

url:
  optional documentation URL for built-in errors
  absent for PydanticCustomError
```

Pydantic documents `ErrorDetails` as a dictionary with `type`, `loc`, `msg`, `input`, optional `ctx`, and optional `url`; the first item of `loc` is the field where the error occurred, and nested models/lists add subsequent location components. ([Pydantic Docs][S16-1])

---

## 16.3 Minimal error example

```python
from pydantic import BaseModel, Field, ValidationError

class Location(BaseModel):
    lat: float
    lng: float

class Model(BaseModel):
    required_float: float
    gt_int: int = Field(gt=42)
    list_of_ints: list[int]
    location: Location

data = {
    "gt_int": 21,
    "list_of_ints": ["1", 2, "bad"],
    "location": {"lat": 4.2, "lng": "New York"},
}

try:
    Model.model_validate(data)
except ValidationError as exc:
    for err in exc.errors():
        print(err["type"], err["loc"], err.get("ctx"))
```

Expected classes of errors:

```text
missing
  required_float omitted

greater_than
  gt_int <= 42, ctx contains gt

int_parsing
  list_of_ints[2] cannot parse as int

float_parsing
  location.lng cannot parse as float
```

Pydantic’s error-handling docs show exactly this style of aggregation: a single `ValidationError` contains multiple field, list-index, custom-validator, and nested-model errors. ([Pydantic Docs][S16-1])

---

## 16.4 Error locations: `loc`

### Field

```python
("email",)
```

### Nested model

```python
("address", "zip_code")
```

### List item

```python
("items", 1, "value")
```

### Union branch

```python
("id", "str")
("id", "int")
```

### Discriminated union branch

```python
("pet", "dog", "barks")
("pet", "cat", "black", "black_name")
```

Pydantic `loc` values are tuples by default. Nested lists use integer indexes; nested models add field names; union errors append branch labels; discriminated unions use selected tags/branch names, making errors shorter than ordinary untagged union errors. ([Pydantic Docs][S16-1])

---

## 16.5 Location rendering customization

```python
from typing import Any

from pydantic import ValidationError

def loc_to_dot_sep(loc: tuple[str | int, ...]) -> str:
    path = ""
    for item in loc:
        if isinstance(item, str):
            if path:
                path += "."
            path += item
        elif isinstance(item, int):
            path += f"[{item}]"
        else:
            raise TypeError(f"Unexpected loc item: {item!r}")
    return path

def convert_error_locations(exc: ValidationError) -> list[dict[str, Any]]:
    errors = exc.errors()
    for error in errors:
        error["loc"] = loc_to_dot_sep(error["loc"])
    return errors
```

Use cases:

```text
API clients expecting JSON Pointer
frontend forms expecting dot/bracket paths
CLI output
spreadsheet row/column diagnostics
LLM repair prompts
```

Pydantic’s docs show this exact post-processing pattern: call `e.errors()`, transform each `loc`, and emit the modified list. ([Pydantic Docs][S16-1])

---

## 16.6 Error types as stable programmatic contract

Recommended branching:

```python
try:
    Model.model_validate(data)
except ValidationError as exc:
    for error in exc.errors():
        match error["type"]:
            case "missing":
                ...
            case "extra_forbidden":
                ...
            case "int_parsing" | "int_type":
                ...
            case "greater_than":
                ...
```

Do **not** branch on `msg`:

```python
# brittle
if "Input should be a valid integer" in error["msg"]:
    ...
```

Pydantic’s version policy explicitly says that `ValidationError.msg`, `ctx`, and `loc` may change in minor releases, while `type` is the field to use for programmatic parsing; new keys and new error types may also be added. ([Pydantic Docs][S16-3])

---

## 16.7 Error URL and versioning

Built-in errors often include URLs like:

```text
https://errors.pydantic.dev/2/v/int_parsing
https://errors.pydantic.dev/2/v/missing
https://errors.pydantic.dev/2/v/greater_than
```

Operational semantics:

```text
url:
  documentation pointer for humans
  useful for debugging and developer tools
  not guaranteed for custom errors
  can be excluded with include_url=False

type:
  stable machine discriminator

msg:
  display string; do not parse
```

`ErrorDetails.url` is documented as the URL giving information about the error, and Pydantic notes no URL is available for `PydanticCustomError`. The `.errors()` and `.json()` methods can omit URLs with `include_url=False`. ([Pydantic Docs][S16-2])

---

## 16.8 `.errors()` redaction options

```python
safe_errors = exc.errors(
    include_input=False,
    include_context=True,
    include_url=False,
)
```

Redaction profiles:

```text
developer diagnostics:
  include_input=True
  include_context=True
  include_url=True

public API response:
  include_input=False
  include_context=True or False
  include_url=False

logs with sensitive payloads:
  include_input=False
  include_context=False
  include_url=False

test assertions:
  include_input=False
  include_url=False
```

`ValidationError.errors()` and `.json()` support `include_url`, `include_context`, and `include_input`, allowing callers to omit sensitive input values and reduce output size. ([Pydantic Docs][S16-2])

---

## 16.9 `.json()` error output

```python
try:
    Model.model_validate(data)
except ValidationError as exc:
    compact = exc.json(include_input=False, include_url=False)
    pretty = exc.json(indent=2, include_input=False)
```

`.json()` returns a JSON string representation of `.errors()`, with the same options for URL, context, and input inclusion. ([Pydantic Docs][S16-2])

Agent rule:

```text
Use .errors() when:
  Python code will transform/map errors

Use .json() when:
  returning diagnostics as JSON string
  logging serialized error payloads
  writing error artifacts

Prefer:
  framework-native JSON response from .errors()
over:
  embedding .json() string inside another JSON document
```

---

## 16.10 `str(e)` display

```python
try:
    Model.model_validate(data)
except ValidationError as exc:
    print(str(exc))
```

`str(e)` is human-readable:

```text
5 validation errors for Model
is_required
  Field required [type=missing, input_value=..., input_type=dict]
...
```

Use:

```text
developer console
CLI diagnostics
test failure debugging
interactive notebooks
```

Avoid:

```text
API response contract
machine-readable branching
snapshot tests across Pydantic versions
```

Pydantic documents `str(e)` as the human-readable error representation and `.errors()` / `.json()` as structured representations. ([Pydantic Docs][S16-1])

---

## 16.11 Redacting sensitive input from printed errors

### Model-level config

```python
from pydantic import BaseModel, ConfigDict

class Login(BaseModel):
    model_config = ConfigDict(hide_input_in_errors=True)

    username: str
    password: str
```

Effect:

```text
printed ValidationError:
  includes type
  omits input_value and input_type
```

`hide_input_in_errors=True` hides input value and input type when printing validation errors; the default is `False`. ([Pydantic Docs][S16-4])

### Structured redaction

```python
errors = exc.errors(include_input=False)
json_errors = exc.json(include_input=False)
```

Agent rule:

```text
hide_input_in_errors:
  protects human-readable printed errors

include_input=False:
  protects structured errors and JSON errors

Use both for secret-bearing public boundaries.
```

---

## 16.12 Error location by alias

```python
from pydantic import BaseModel, ConfigDict, Field, ValidationError

class Model(BaseModel):
    model_config = ConfigDict(loc_by_alias=True)

    user_id: int = Field(validation_alias="userId")

try:
    Model.model_validate({"userId": "bad"})
except ValidationError as exc:
    assert exc.errors()[0]["loc"] == ("userId",)
```

`loc_by_alias` controls whether error locations use the actual key/alias provided in the input rather than the Python field name; it defaults to `True`. ([Pydantic Docs][S16-4])

Policy:

```text
Public API:
  loc_by_alias=True
  clients see wire keys

Internal validation:
  loc_by_alias=False can be easier for Python code

Mixed:
  preserve Pydantic loc and add separate normalized path
```

---

## 16.13 Usage errors vs validation errors

### Validation error

```python
from pydantic import BaseModel, ValidationError

class M(BaseModel):
    x: int

try:
    M.model_validate({"x": "bad"})
except ValidationError as exc:
    ...
```

Meaning:

```text
schema valid
Pydantic usage valid
input data invalid
expected at runtime boundary
```

### Usage error

```python
from typing import ForwardRef
from pydantic import BaseModel, PydanticUserError

UndefinedType = ForwardRef("UndefinedType")

class M(BaseModel):
    x: UndefinedType

try:
    M(x=1)
except PydanticUserError as exc:
    assert exc.code == "class-not-fully-defined"
```

Meaning:

```text
schema/config/decorator/use is invalid
developer action required
should fail fast
should not be reported as user input validation failure
```

Pydantic usage errors include `class-not-fully-defined`, invalid JSON Schema customization, invalid validator/serializer decorator use, invalid discriminator configuration, missing annotations, `TypeAdapter` config misuse, invalid `validate_call` signatures, and related developer-facing issues. ([Pydantic Docs][S16-5])

Agent policy:

```text
ValidationError:
  catch at API/data boundaries

PydanticUserError:
  fail startup/tests/import
  fix code
  do not return as 422 validation response
```

---

## 16.14 Validator-raised errors

### Recommended

```python
from pydantic import BaseModel, field_validator

class Model(BaseModel):
    x: int

    @field_validator("x")
    @classmethod
    def positive(cls, v: int) -> int:
        if v <= 0:
            raise ValueError("must be positive")
        return v
```

### Custom machine-readable error

```python
from pydantic import BaseModel, field_validator
from pydantic_core import PydanticCustomError

class Model(BaseModel):
    x: int

    @field_validator("x")
    @classmethod
    def not_answer(cls, v: int) -> int:
        if v % 42 == 0:
            raise PydanticCustomError(
                "the_answer_error",
                "{number} is the answer!",
                {"number": v},
            )
        return v
```

Recommended validator exception types:

```text
ValueError:
  ordinary validation failure

AssertionError:
  concise, but skipped under python -O

PydanticCustomError:
  custom error type + message template + ctx
```

Pydantic’s validators docs state that validators should raise `ValueError`, `AssertionError`, or `PydanticCustomError`; `PydanticCustomError` lets validators control the error type, message template, and context. ([Pydantic Docs][S16-6])

---

## 16.15 `PydanticCustomError`

```python
from pydantic_core import PydanticCustomError

raise PydanticCustomError(
    "invalid_plan",
    "Invalid plan: {plan}; expected one of {allowed}",
    {"plan": value, "allowed": ["free", "pro"]},
)
```

Produces error shape like:

```python
{
    "type": "invalid_plan",
    "loc": ("plan",),
    "msg": "Invalid plan: enterprise; expected one of ['free', 'pro']",
    "input": "enterprise",
    "ctx": {"plan": "enterprise", "allowed": ["free", "pro"]},
}
```

`PydanticCustomError` is a `ValueError` subclass intended for flexible Pydantic validator errors; constructor parameters are `error_type`, `message_template`, and optional `context`, and the generated custom error has no documentation URL. ([Pydantic Docs][S16-2])

Use cases:

```text
API-stable custom error codes
translation/localization
domain-specific validation
frontend error mapping
LLM repair logic
```

---

## 16.16 `PydanticKnownError`

```python
from pydantic_core import PydanticKnownError

raise PydanticKnownError("greater_than", {"gt": 0})
```

`PydanticKnownError` mimics built-in Pydantic errors with a known error type and context; unlike `PydanticCustomError`, its `error_type` must be a known `ErrorType`. ([Pydantic Docs][S16-2])

Policy:

```text
Prefer PydanticCustomError for:
  domain-specific custom codes

Use PydanticKnownError for:
  advanced libraries emulating built-in errors

Most application validators:
  ValueError is enough
```

---

## 16.17 `TypeError` in validators

```python
from pydantic import BaseModel, field_validator

class Model(BaseModel):
    x: int

    @field_validator("x")
    @classmethod
    def bad(cls, v: int) -> int:
        return str.lower(v)  # TypeError
```

Pydantic v2 no longer converts `TypeError` raised inside validators into `ValidationError`; such `TypeError`s propagate as programming errors. This is intentional so accidental validator bugs are not hidden as user input failures. ([Pydantic Docs][S16-6])

Agent rule:

```text
Intentional validation failure:
  raise ValueError or PydanticCustomError

Bug / wrong API call:
  TypeError should propagate
```

---

## 16.18 Custom error messages by post-processing

```python
from pydantic import BaseModel, HttpUrl, ValidationError
from pydantic_core import ErrorDetails

CUSTOM_MESSAGES = {
    "int_parsing": "This is not an integer.",
    "url_scheme": "Expected one of: {expected_schemes}.",
}

def convert_errors(
    exc: ValidationError,
    custom_messages: dict[str, str],
) -> list[ErrorDetails]:
    new_errors = []
    for error in exc.errors():
        custom = custom_messages.get(error["type"])
        if custom:
            ctx = error.get("ctx")
            error["msg"] = custom.format(**ctx) if ctx else custom
        new_errors.append(error)
    return new_errors
```

Pydantic’s docs recommend custom error handlers that transform `.errors()` output to change messages, translate errors, or reformat locations. ([Pydantic Docs][S16-1])

Agent rule:

```text
Do not monkeypatch Pydantic global error messages.
Transform structured errors at the boundary:
  API adapter
  CLI renderer
  localization layer
  LLM repair prompt builder
```

---

## 16.19 Aggregated errors

```python
try:
    Model.model_validate(bad_payload)
except ValidationError as exc:
    assert exc.error_count() >= 1
    errors = exc.errors()
```

Aggregation behavior:

```text
one ValidationError
many ErrorDetails
nested locs
all discovered field/list/model errors where possible
```

Pydantic explicitly states that `ValidationError` contains information about all validation errors and provides `.error_count()` to return the number of errors. ([Pydantic Docs][S16-1])

Boundary policy:

```text
API validation response:
  return full error list unless payload is huge/sensitive

CLI:
  print human summary + optionally full structured details in verbose mode

LLM repair loop:
  include reduced error list: loc, type, ctx, maybe msg

Logging:
  include count, types, locs; omit input
```

---

## 16.20 Union error reporting

### Untagged union

```python
from typing import Union
from pydantic import BaseModel, Field, ValidationError

class User(BaseModel):
    id: Union[str, int] = Field(union_mode="left_to_right")

try:
    User(id=[])
except ValidationError as exc:
    print(exc)
```

Typical locs:

```text
id.str
id.int
```

When union validation fails, Pydantic includes errors from all union members that failed. This can be verbose, especially with recursive models; Pydantic recommends discriminated unions where possible because they are more predictable, more performant, and produce fewer irrelevant errors. ([Pydantic Docs][S16-7])

Policy:

```text
Public API polymorphism:
  use discriminated unions

Ambiguous legacy union:
  use union_mode="left_to_right" only with tests

LLM repair:
  prefer discriminated union to reduce error ambiguity
```

---

## 16.21 Discriminated-union error reporting

### String discriminator

```python
from typing import Literal
from pydantic import BaseModel, Field, ValidationError

class Cat(BaseModel):
    pet_type: Literal["cat"]
    meows: int

class Dog(BaseModel):
    pet_type: Literal["dog"]
    barks: float

class Model(BaseModel):
    pet: Cat | Dog = Field(discriminator="pet_type")
```

If discriminator matches a branch but branch data is invalid:

```text
pet.dog.barks
  Field required [type=missing, ...]
```

If discriminator tag is invalid:

```text
pet
  Input tag 'red' ... does not match expected tags [type=union_tag_invalid, ...]
```

Discriminated unions validate only the selected branch based on the discriminator, which avoids proliferation of errors; invalid tags produce `union_tag_invalid`, and callable discriminators that return `None` produce `union_tag_not_found`. ([Pydantic Docs][S16-7])

---

## 16.22 Callable discriminator diagnostics

```python
from typing import Annotated, Any, Union
from pydantic import BaseModel, Discriminator, Tag

class SpecialValue(BaseModel):
    value: int

def discr(v: Any) -> str | None:
    if isinstance(v, int):
        return "int"
    if isinstance(v, dict) and "value" in v:
        return "model"
    if isinstance(v, SpecialValue):
        return "model"
    return None

Adapted = Annotated[
    Union[Annotated[int, Tag("int")], Annotated[SpecialValue, Tag("model")]],
    Discriminator(discr),
]
```

Diagnostic rules:

```text
callable returns known tag:
  selected branch validates

callable returns unknown tag:
  union_tag_invalid

callable returns None:
  union_tag_not_found
```

Pydantic warns callable discriminators must handle both dictionaries and model instances because they are used for validation and serialization; returning `None` raises `union_tag_not_found`. ([Pydantic Docs][S16-7])

---

## 16.23 Custom discriminator error type/message/context

```python
from typing import Annotated, Union
from pydantic import BaseModel, Discriminator, Tag

class Model(BaseModel):
    x: Annotated[
        Union[
            Annotated[str, Tag("str")],
            Annotated["Model", Tag("model")],
        ],
        Discriminator(
            lambda v: "model" if isinstance(v, dict) else "str" if isinstance(v, str) else None,
            custom_error_type="invalid_union_member",
            custom_error_message="Input is not a valid union member",
            custom_error_context={"allowed": "str or model"},
        ),
    ]
```

Pydantic’s union docs state that `Discriminator` can customize the error type, message, and context for discriminator failures. ([Pydantic Docs][S16-7])

Use when:

```text
API needs stable domain error code
LLM repair prompt should avoid verbose branch errors
frontend should show one concise discriminator error
```

---

## 16.24 API boundary error handling

### Recommended adapter

```python
from pydantic import BaseModel, ConfigDict, ValidationError

class ApiInput(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        loc_by_alias=True,
        hide_input_in_errors=True,
    )

def to_problem_details(exc: ValidationError) -> dict:
    errors = []
    for err in exc.errors(include_input=False, include_url=False):
        errors.append(
            {
                "code": err["type"],
                "path": list(err["loc"]),
                "message": err["msg"],
                "context": err.get("ctx", {}),
            }
        )

    return {
        "type": "https://example.com/problems/validation-error",
        "title": "Invalid request body",
        "status": 422,
        "errors": errors,
    }
```

Boundary policy:

```text
HTTP request body:
  catch ValidationError
  return 400/422 per framework convention
  include loc/type/msg
  omit input
  optionally omit url

Internal service call:
  either raise ValidationError
  or map to typed domain error

CLI:
  print str(exc)
  maybe print exc.json(indent=2, include_input=False) under --verbose

LLM repair:
  include type, loc, ctx, minimal msg
  omit raw input unless safe
```

Pydantic exposes enough structured data for API mapping through `.errors()` and supports hiding printed inputs with `hide_input_in_errors` plus omitting structured input values through `include_input=False`. ([Pydantic Docs][S16-1])

---

## 16.25 Redaction profiles

### Secret-bearing payload

```python
errors = exc.errors(
    include_input=False,
    include_context=False,
    include_url=False,
)
```

### Internal debug

```python
errors = exc.errors(
    include_input=True,
    include_context=True,
    include_url=True,
)
```

### Public API default

```python
errors = exc.errors(
    include_input=False,
    include_context=True,
    include_url=False,
)
```

### Printed-error config

```python
class PublicModel(BaseModel):
    model_config = ConfigDict(hide_input_in_errors=True)
```

Agent rule:

```text
Never include raw input in:
  auth payload errors
  secrets/settings validation errors
  public API error responses
  multi-tenant logs
  LLM traces containing private data
```

---

## 16.26 Debugging validator causes

```python
from pydantic import BaseModel, ConfigDict

class Model(BaseModel):
    model_config = ConfigDict(validation_error_cause=True)
```

`validation_error_cause=True` shows Python exceptions that were part of validation failure as an exception group cause; Pydantic documents it as useful for debugging and defaults it to `False`. ([Pydantic Docs][S16-4])

Policy:

```text
Dev/debug:
  validation_error_cause=True can help locate validator bugs

Production/public:
  keep False unless logs are private and redacted
```

---

## 16.27 Usage-error codes as developer diagnostics

Examples of `PydanticUserError.code`:

```text
class-not-fully-defined
custom-json-schema
decorator-invalid-fields
decorator-missing-arguments
decorator-missing-field
discriminator-no-field
discriminator-alias-type
discriminator-needs-literal
discriminator-alias
schema-for-unknown-type
type-adapter-config-unused
validate-by-alias-and-name-false
```

Usage-error handling:

```python
from pydantic import PydanticUserError

try:
    build_models()
except PydanticUserError as exc:
    raise RuntimeError(f"Pydantic schema bug: {exc.code}") from exc
```

Pydantic’s usage-errors page lists common developer errors and shows assertions against `exc_info.code`, for example `class-not-fully-defined`, `custom-json-schema`, `decorator-invalid-fields`, `decorator-missing-field`, and discriminator-related codes. ([Pydantic Docs][S16-5])

Agent rule:

```text
Usage errors belong in:
  import/startup tests
  schema build tests
  CI failures
  developer diagnostics

Not in:
  ordinary user-input validation response
```

---

## 16.28 Testing error behavior without overfitting

### Good

```python
def test_age_validation_error():
    with pytest.raises(ValidationError) as exc_info:
        User.model_validate({"age": -1})

    errors = exc_info.value.errors(include_input=False, include_url=False)

    assert errors == [
        {
            "type": "greater_than",
            "loc": ("age",),
            "msg": "Input should be greater than 0",
            "ctx": {"gt": 0},
        }
    ]
```

### More stable

```python
def test_age_validation_error_stable():
    with pytest.raises(ValidationError) as exc_info:
        User.model_validate({"age": -1})

    err = exc_info.value.errors(include_input=False, include_url=False)[0]

    assert err["type"] == "greater_than"
    assert err["loc"] == ("age",)
    assert err.get("ctx", {}).get("gt") == 0
```

### Most stable for version-agnostic tests

```python
def test_age_validation_error_type_only():
    with pytest.raises(ValidationError) as exc_info:
        User.model_validate({"age": -1})

    assert {e["type"] for e in exc_info.value.errors()} == {"greater_than"}
```

Pydantic’s version policy says `type` is the field to use for programmatic parsing; `msg`, `ctx`, and `loc` may change in minor releases, and new error keys or new error types may be added. ([Pydantic Docs][S16-3])

---

## 16.29 Test helpers

### Find error by location

```python
def find_error(exc: ValidationError, loc: tuple) -> dict:
    for err in exc.errors(include_url=False):
        if err["loc"] == loc:
            return err
    raise AssertionError(f"No error at {loc!r}; got {exc.errors(include_url=False)}")
```

### Assert type at location

```python
def assert_error_type(exc: ValidationError, loc: tuple, type_: str) -> None:
    err = find_error(exc, loc)
    assert err["type"] == type_
```

### Normalize error payload for snapshots

```python
def normalize_errors(exc: ValidationError) -> list[dict]:
    normalized = []
    for err in exc.errors(include_input=False, include_url=False):
        normalized.append(
            {
                "type": err["type"],
                "loc": list(err["loc"]),
                "ctx": err.get("ctx", {}),
            }
        )
    return normalized
```

Snapshot policy:

```text
Snapshot:
  type
  loc if API contract requires exact paths
  selected ctx keys

Avoid snapshotting:
  msg
  input
  url
  full str(exc)
```

---

## 16.30 Error handling patterns by boundary

```text
HTTP JSON API:
  catch ValidationError
  emit code/path/message/context
  omit input
  loc_by_alias=True
  hide_input_in_errors=True

CLI:
  show str(exc)
  use --json-errors to print exc.json(include_input=False)
  use --verbose to include URL/context

Batch pipeline:
  collect record index + errors(include_input=False)
  do not fail whole batch unless policy says so
  preserve input separately in secure storage if needed

Message queue:
  reject / dead-letter with compact normalized errors
  include schema version + error types
  omit raw message if sensitive

LLM repair:
  include loc/type/ctx/minimal msg
  avoid raw private input
  prefer discriminated unions to reduce noisy branch errors

Library API:
  raise ValidationError unchanged unless public abstraction requires mapping
  document stable error types only if promising compatibility
```

---

## 16.31 Deployment decision matrix

```text
Need                                            Mechanism
--------------------------------------------------------------------------------
structured errors                               exc.errors()
JSON error string                               exc.json()
error count                                     exc.error_count()
human-readable diagnostics                      str(exc)
hide input in printed errors                    ConfigDict(hide_input_in_errors=True)
hide input in structured errors                 exc.errors(include_input=False)
omit docs URLs                                  exc.errors(include_url=False)
omit ctx                                        exc.errors(include_context=False)
client-facing alias paths                       ConfigDict(loc_by_alias=True)
custom domain error code                        PydanticCustomError
custom built-in-like error                      PydanticKnownError
translate/change messages                       post-process exc.errors()
dot/bracket loc paths                           post-process loc tuple
debug underlying validator exceptions           ConfigDict(validation_error_cause=True)
reduce union noise                              discriminated unions
custom discriminator failure                    Discriminator(custom_error_type=...)
developer misuse detection                      PydanticUserError.code
stable tests                                    assert error["type"]
```

---

## 16.32 Anti-patterns

### Raising `ValidationError` inside validators

```python
@field_validator("x")
@classmethod
def validate_x(cls, v):
    raise ValidationError(...)  # bad
```

Preferred:

```python
raise ValueError("x is invalid")
```

Pydantic states validation code should raise `ValueError`, `AssertionError`, or related subclasses, which are caught and used to populate the final `ValidationError`. ([Pydantic Docs][S16-1])

---

### Branching on message text

```python
if "valid integer" in err["msg"]:
    ...
```

Preferred:

```python
if err["type"] in {"int_type", "int_parsing"}:
    ...
```

---

### Public API returns raw inputs

```python
return {"errors": exc.errors()}
```

Preferred:

```python
return {"errors": exc.errors(include_input=False, include_url=False)}
```

---

### Converting `PydanticUserError` into 422

```python
except Exception as exc:
    return {"status": 422, "detail": str(exc)}
```

Preferred:

```python
except ValidationError as exc:
    return validation_response(exc)
```

Usage errors indicate schema/developer misuse, not bad user input.

---

### Using untagged recursive unions in public schemas

```python
class M(BaseModel):
    x: str | "M"
```

Problem:

```text
verbose recursive union branch errors
hard for clients and LLMs to repair
```

Prefer discriminated unions where possible.

---

### Logging `str(exc)` for secrets

```python
logger.warning("bad payload: %s", exc)
```

Preferred:

```python
logger.warning(
    "bad payload",
    extra={"errors": exc.errors(include_input=False, include_url=False)},
)
```

---

## 16.33 Test matrix

### `.errors()` structure

```python
def test_errors_structure():
    class M(BaseModel):
        x: int = Field(gt=0)

    with pytest.raises(ValidationError) as exc_info:
        M.model_validate({"x": -1})

    err = exc_info.value.errors()[0]

    assert err["type"] == "greater_than"
    assert err["loc"] == ("x",)
    assert err["ctx"]["gt"] == 0
    assert "input" in err
    assert "url" in err
```

### Redacted errors

```python
def test_errors_redacted():
    class M(BaseModel):
        password: str

    with pytest.raises(ValidationError) as exc_info:
        M.model_validate({"password": 123})

    err = exc_info.value.errors(include_input=False, include_url=False)[0]

    assert "input" not in err
    assert "url" not in err
    assert err["type"] == "string_type"
```

### Printed input hidden

```python
def test_hide_input_in_printed_error():
    class M(BaseModel):
        model_config = ConfigDict(hide_input_in_errors=True)
        password: str

    with pytest.raises(ValidationError) as exc_info:
        M(password=123)

    assert "input_value=123" not in str(exc_info.value)
    assert "input_type=int" not in str(exc_info.value)
```

### Custom error type

```python
def test_custom_error_type():
    class M(BaseModel):
        x: int

        @field_validator("x")
        @classmethod
        def custom(cls, v):
            raise PydanticCustomError("my_error", "bad {value}", {"value": v})

    with pytest.raises(ValidationError) as exc_info:
        M(x=1)

    err = exc_info.value.errors(include_url=False)[0]

    assert err["type"] == "my_error"
    assert err["ctx"] == {"value": 1}
```

### Usage error

```python
def test_usage_error_code():
    UndefinedType = ForwardRef("UndefinedType")

    class M(BaseModel):
        x: UndefinedType

    with pytest.raises(PydanticUserError) as exc_info:
        M(x=1)

    assert exc_info.value.code == "class-not-fully-defined"
```

### Untagged union branch errors

```python
def test_union_errors_are_branch_labeled():
    class M(BaseModel):
        x: str | int = Field(union_mode="left_to_right")

    with pytest.raises(ValidationError) as exc_info:
        M(x=[])

    locs = {err["loc"] for err in exc_info.value.errors()}
    assert ("x", "str") in locs
    assert ("x", "int") in locs
```

### Discriminator tag not found

```python
def test_discriminator_tag_not_found():
    class A(BaseModel):
        value: int

    def discr(v):
        return "a" if isinstance(v, dict) and "value" in v else None

    T = Annotated[Annotated[A, Tag("a")] | Annotated[int, Tag("int")], Discriminator(discr)]

    adapter = TypeAdapter(T)

    with pytest.raises(ValidationError) as exc_info:
        adapter.validate_python("bad")

    assert exc_info.value.errors()[0]["type"] == "union_tag_not_found"
```

---

## 16.34 Agent checklist

```text
[ ] Catch ValidationError at data/API boundaries.
[ ] Do not catch PydanticUserError as user validation failure.
[ ] Use exc.errors() for structured mapping.
[ ] Use exc.json() only when a JSON string is directly needed.
[ ] Use exc.error_count() for summaries.
[ ] Use error["type"] for programmatic branching.
[ ] Avoid parsing error["msg"].
[ ] Avoid exact full error snapshots.
[ ] Use include_input=False for public/logged errors.
[ ] Use include_url=False unless docs links are useful.
[ ] Use hide_input_in_errors=True for secret-bearing models.
[ ] Use loc_by_alias=True for client-facing APIs.
[ ] Convert loc tuples to API path format at boundary if needed.
[ ] Raise ValueError or PydanticCustomError in validators.
[ ] Do not raise ValidationError manually inside validators.
[ ] Treat TypeError in validators as a bug.
[ ] Use PydanticCustomError for stable custom domain codes.
[ ] Prefer discriminated unions over untagged unions for public polymorphism.
[ ] Customize Discriminator error type/message/context when one concise union error is required.
[ ] Enable validation_error_cause only for private debugging.
[ ] Test error types and selected ctx, not prose.
```

---

## 16.35 Value case

```text
Pydantic error model value =
  one aggregated validation exception
  structured machine-readable errors
  stable error type identifiers
  nested location paths
  optional input/context/url payloads
  custom validator error codes
  API-boundary redaction controls
  discriminated-union diagnostics
  developer usage-error separation
  reliable test assertions without prose coupling
```

Operationally: treat `ValidationError` as a structured diagnostic envelope. Map it at system boundaries, redact raw input by default, use `type` as the stable programmatic discriminator, keep usage errors as developer failures, and design polymorphic schemas with discriminators to reduce noisy branch-error output.

[S16-1]: https://docs.pydantic.dev/latest/errors/errors/ "Error Handling | Pydantic Docs"
[S16-2]: https://docs.pydantic.dev/latest/api/pydantic_core/ "pydantic_core | Pydantic Docs"
[S16-3]: https://docs.pydantic.dev/latest/version-policy/ "Version Policy | Pydantic Docs"
[S16-4]: https://docs.pydantic.dev/latest/api/config/ "Configuration | Pydantic Docs"
[S16-5]: https://docs.pydantic.dev/latest/errors/usage_errors/ "Usage Errors | Pydantic Docs"
[S16-6]: https://docs.pydantic.dev/latest/concepts/validators/ "Validators | Pydantic Docs"
[S16-7]: https://docs.pydantic.dev/latest/concepts/unions/ "Unions | Pydantic Docs"


# Part III — Runtime boundaries, performance, models, and deployment recipes

# Pydantic Advanced — 17) Settings management with `pydantic-settings`

Style target: advanced, sectioned, agent-oriented technical reference. 

`pydantic-settings` provides Pydantic’s settings/config loading layer. A `BaseSettings` subclass behaves like a validated Pydantic model whose missing initializer values are filled from configured sources such as environment variables, dotenv files, secrets directories, CLI arguments, and custom sources. Defaults are still used when no configured source provides a value. ([Pydantic Docs][S17-1])

---

## 17.0 Installation and imports

```bash
pip install pydantic-settings
```

```python
from pydantic import AliasChoices, BaseModel, Field, PostgresDsn, RedisDsn, SecretStr
from pydantic_settings import BaseSettings, SettingsConfigDict
```

`pydantic-settings` is a separate package that provides optional Pydantic features for loading settings/config classes from environment variables and secrets files. ([Pydantic Docs][S17-1])

---

## 17.1 Mental model

```text
Settings class definition
  → BaseSettings subclass
  → fields + defaults + SettingsConfigDict
  → settings sources queried in priority order
  → merged input mapping
  → Pydantic validation
  → typed immutable-or-mutable config object
```

Default source priority, highest to lowest:

```text
1. CLI args, if cli_parse_args is enabled
2. __init__ keyword arguments
3. environment variables
4. dotenv files
5. secrets directory files
6. field defaults
```

The settings docs define this priority order and note that `settings_customise_sources` can override it. ([Pydantic Docs][S17-1])

Agent invariant:

```text
BaseSettings is a boundary object.
It validates deployment configuration.
It should be constructed at application startup or test setup.
It should not be constructed at import time.
```

---

## 17.2 `BaseSettings`

### Basic settings class

```python
from pydantic import PostgresDsn, RedisDsn
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="APP_")

    database_url: PostgresDsn
    redis_url: RedisDsn = "redis://localhost:6379/0"
    debug: bool = False
    workers: int = 4
```

Environment mapping:

```text
APP_DATABASE_URL → database_url
APP_REDIS_URL    → redis_url
APP_DEBUG        → debug
APP_WORKERS      → workers
```

By default, environment variable names match field names; `env_prefix` changes the prefix for all environment variables, and `_env_prefix` can override it at construction time. ([Pydantic Docs][S17-1])

---

## 17.3 Construction behavior

```python
settings = Settings(database_url="postgres://user:pass@localhost:5432/app")
```

Semantics:

```text
explicit __init__ kwarg:
  highest normal priority unless CLI is enabled and ranked above init

missing kwarg:
  read from env / dotenv / secrets / custom sources

missing everywhere:
  use default if defined

missing required field:
  ValidationError
```

`BaseSettings` reads values for fields not passed as keyword arguments from the environment; explicit initializer values are useful for manual overrides and unit tests. ([Pydantic Docs][S17-1])

---

## 17.4 `SettingsConfigDict`

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_prefix="APP_",
        case_sensitive=False,
        env_file=".env",
        env_file_encoding="utf-8",
        secrets_dir="/run/secrets",
        env_nested_delimiter="__",
        validate_default=True,
        extra="ignore",
    )

    database_url: str
```

High-value settings keys:

```text
env_prefix
case_sensitive
env_prefix_target
env_file
env_file_encoding
dotenv_filtering
secrets_dir
env_nested_delimiter
env_nested_max_split
nested_model_default_partial_update
env_ignore_empty
enable_decoding
cli_parse_args
validate_default
extra
```

`SettingsConfigDict` extends `ConfigDict` and settings-specific sources include environment, dotenv, secrets, TOML/JSON/YAML/pyproject sources, and CLI support. ([Pydantic Docs][S17-2])

---

## 17.5 Environment variable names

### Default field-name matching

```python
class Settings(BaseSettings):
    redis_host: str = "localhost"
```

```bash
export REDIS_HOST=redis.internal
```

By default, environment variable names are case-insensitive, so `redis_host`, `REDIS_HOST`, and similar case variants can match unless `case_sensitive=True` is configured. ([Pydantic Docs][S17-1])

### `env_prefix`

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="MYAPP_")

    auth_key: str
```

```bash
export MYAPP_AUTH_KEY=secret
```

Default `env_prefix` is the empty string; `env_prefix` applies to env vars, dotenv files, secrets, and other settings sources. ([Pydantic Docs][S17-1])

---

## 17.6 `env_prefix_target`

```python
from pydantic import Field

class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_prefix="APP_",
        env_prefix_target="variable",  # default
    )

    foo: str = Field(alias="FooAlias")
    bar: str
```

Modes:

```text
env_prefix_target="variable":
  prefix field-name-derived variables only
  aliases are read without prefix

env_prefix_target="alias":
  prefix aliases only

env_prefix_target="all":
  prefix aliases and variable names
```

Pydantic settings documents `env_prefix_target` as the control for whether `env_prefix` applies only to variable names, only to aliases, or to both. ([Pydantic Docs][S17-1])

Agent rule:

```text
If aliases are used for env vars:
  explicitly decide env_prefix_target.

Do not assume env_prefix applies to aliases.
```

---

## 17.7 Case sensitivity

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(case_sensitive=True)

    redis_host: str = "localhost"
```

Semantics:

```text
case_sensitive=False:
  default
  env names case-insensitive

case_sensitive=True:
  env variable names must match field names / aliases exactly, aside from prefix

nested models:
  case_sensitive setting applies recursively

Windows:
  Python os.environ is always case-insensitive, so case_sensitive has no practical effect
```

Pydantic settings documents default case-insensitive matching, `case_sensitive=True`, nested-model propagation, and the Windows limitation. ([Pydantic Docs][S17-1])

Policy:

```text
Linux production:
  prefer uppercase env aliases for human ops ergonomics.

Cross-platform tests:
  do not rely on case-sensitive env behavior on Windows.

Strict env contract:
  set case_sensitive=True and use explicit validation_alias names.
```

---

## 17.8 Field aliases for env vars

```python
from pydantic import AliasChoices, Field
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="APP_")

    auth_key: str = Field(validation_alias="MY_AUTH_KEY")
    api_key: str = Field(alias="MY_API_KEY")
    redis_dsn: str = Field(
        default="redis://localhost:6379/0",
        validation_alias=AliasChoices("SERVICE_REDIS_DSN", "REDIS_URL"),
    )
```

Semantics:

```text
validation_alias:
  input/env-only alias
  does not change serialization alias

alias:
  validation + serialization alias

AliasChoices:
  multiple accepted env names
  first found value wins
```

The docs show `validation_alias` overriding env variable name, `alias` applying to validation and serialization, and `AliasChoices` allowing multiple environment variable names for one field with the first found value used. ([Pydantic Docs][S17-1])

Deployment pattern:

```text
new canonical env:
  SERVICE_REDIS_DSN

legacy env:
  REDIS_URL

field:
  redis_dsn: RedisDsn = Field(validation_alias=AliasChoices("SERVICE_REDIS_DSN", "REDIS_URL"))
```

---

## 17.9 Validation of defaults

```python
class Settings(BaseSettings):
    foo: int = "bad"
```

Unlike `BaseModel`, `BaseSettings` validates default values by default. You can disable this globally or per field:

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(validate_default=False)

    foo: int = "bad"
```

```python
class Settings(BaseSettings):
    foo: int = Field("bad", validate_default=False)
```

Pydantic settings explicitly documents that defaults are validated by default for `BaseSettings`, unlike `BaseModel`. ([Pydantic Docs][S17-1])

Policy:

```text
Production settings:
  keep validate_default=True

Generated defaults / test stubs:
  validate defaults unless intentionally allowing raw values

Migration from permissive configs:
  temporary validate_default=False only with tests and TODO
```

---

## 17.10 Parsing environment variable values

### Simple types

```python
class Settings(BaseSettings):
    port: int
    debug: bool
```

```bash
export PORT=8000
export DEBUG=true
```

Simple field types such as `int`, `float`, and `str` are parsed from the env value as if the string had been passed directly to model initialization. ([Pydantic Docs][S17-1])

### Complex types as JSON

```python
class Settings(BaseSettings):
    domains: set[str]
    thresholds: dict[str, int]
```

```bash
export DOMAINS='["example.com", "api.example.com"]'
export THRESHOLDS='{"low": 1, "high": 10}'
```

Complex types such as `list`, `set`, `dict`, and submodels are populated from environment variables by treating the env value as a JSON-encoded string. ([Pydantic Docs][S17-1])

---

## 17.11 Disable / force JSON decoding

### Per-field `NoDecode`

```python
from typing import Annotated
from pydantic import field_validator
from pydantic_settings import BaseSettings, NoDecode

class Settings(BaseSettings):
    numbers: Annotated[list[int], NoDecode]

    @field_validator("numbers", mode="before")
    @classmethod
    def decode_numbers(cls, v: str) -> list[int]:
        return [int(x) for x in v.split(",")]
```

### Global `enable_decoding=False`

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(enable_decoding=False)

    numbers: list[int]

    @field_validator("numbers", mode="before")
    @classmethod
    def decode_numbers(cls, v: str) -> list[int]:
        return [int(x) for x in v.split(",")]
```

### Per-field `ForceDecode`

```python
from typing import Annotated
from pydantic_settings import ForceDecode

class Settings(BaseSettings):
    model_config = SettingsConfigDict(enable_decoding=False)

    numbers: Annotated[list[int], ForceDecode]
```

By default, complex env values are parsed as JSON. `NoDecode` disables JSON parsing for a field, `enable_decoding=False` disables JSON parsing globally, and `ForceDecode` forces JSON parsing for a field even when global decoding is disabled. ([Pydantic Docs][S17-1])

---

## 17.12 Nested settings: JSON env values

```python
from pydantic import BaseModel

class LLMConfig(BaseModel):
    provider: str = "openai"
    api_key: str
    api_version: str = "2024-01-01"

class Settings(BaseSettings):
    llm: LLMConfig
```

```bash
export LLM='{"provider": "anthropic", "api_key": "sk-...", "api_version": "2026-01-01"}'
```

Complex submodels are parsed from JSON strings when provided as env values. Pydantic settings notes that submodels used this way should inherit from `pydantic.BaseModel`; otherwise initialization can produce unexpected results. ([Pydantic Docs][S17-1])

Agent rule:

```text
Nested settings object:
  make it BaseModel, not plain dataclass, unless tested.

Env JSON:
  good for single complex object override.

Env nested delimiter:
  better for ops-managed individual variables.
```

---

## 17.13 Nested settings: `env_nested_delimiter`

```python
from pydantic import BaseModel

class DeepSubModel(BaseModel):
    v4: str

class SubModel(BaseModel):
    v1: str
    v2: bytes
    v3: int
    deep: DeepSubModel

class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_nested_delimiter="__")

    v0: str
    sub_model: SubModel
```

```bash
export V0=0
export SUB_MODEL='{"v1": "json-1", "v2": "json-2"}'
export SUB_MODEL__V2=nested-2
export SUB_MODEL__V3=3
export SUB_MODEL__DEEP__V4=v4
```

Semantics:

```text
SUB_MODEL__DEEP__V4=v4
  → {"sub_model": {"deep": {"v4": "v4"}}}

Multiple nested variables:
  merged

Nested env var vs top-level JSON:
  nested env var wins
```

The docs state that `env_nested_delimiter` explodes env var names into nested dictionaries, merged across variables, and that nested variables take precedence over top-level JSON env values. ([Pydantic Docs][S17-1])

---

## 17.14 `env_nested_max_split`

```python
class LLMConfig(BaseModel):
    provider: str = "openai"
    api_key: str
    api_version: str = "2024-03-15"

class GenerationConfig(BaseSettings):
    model_config = SettingsConfigDict(
        env_prefix="GENERATION_",
        env_nested_delimiter="_",
        env_nested_max_split=1,
    )

    llm: LLMConfig
```

```bash
export GENERATION_LLM_PROVIDER=anthropic
export GENERATION_LLM_API_KEY=your-api-key
export GENERATION_LLM_API_VERSION=2024-03-15
```

`env_nested_max_split` limits nested splitting depth; without it, a variable like `GENERATION_LLM_API_KEY` could be parsed as `llm.api.key` instead of `llm.api_key`. ([Pydantic Docs][S17-1])

Agent rule:

```text
If delimiter is "_":
  almost always set env_nested_max_split for nested settings.

If delimiter is "__":
  usually safe for deep nesting.
```

---

## 17.15 Nested model default partial updates

```python
class SubModel(BaseModel):
    val: int = 0
    flag: bool = False

class SettingsPartialUpdate(BaseSettings):
    model_config = SettingsConfigDict(
        env_nested_delimiter="__",
        nested_model_default_partial_update=True,
    )

    nested_model: SubModel = SubModel(val=1)
```

```bash
export NESTED_MODEL__FLAG=True
```

Semantics:

```text
nested_model_default_partial_update=True:
  existing default SubModel(val=1) is partially updated
  result: {"val": 1, "flag": True}

nested_model_default_partial_update=False:
  new SubModel is instantiated from partial env data
  result may use nested model defaults for omitted fields
```

The docs state that nested model default partial updates are disabled by default and can be enabled with `nested_model_default_partial_update=True`. ([Pydantic Docs][S17-1])

---

## 17.16 Dotenv `.env` files

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
    )

    database_url: str
```

Runtime override:

```python
settings = Settings(_env_file="prod.env", _env_file_encoding="utf-8")
```

Semantics:

```text
env_file:
  path to dotenv file

_env_file:
  instance-level override

multiple files:
  tuple/list loaded in order
  later files override earlier files

real environment variables:
  always take priority over dotenv values

path lookup:
  if filename is provided, current working directory only; parent directories are not searched
```

Pydantic supports dotenv loading through `env_file` / `_env_file`; environment variables override dotenv values; multiple dotenv files are loaded in order with later files overriding earlier ones; and a relative `env_file` name is checked only in the current working directory. ([Pydantic Docs][S17-1])

### Dotenv `extra`

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_file=".env",
        extra="ignore",
    )
```

Dotenv values are loaded and passed to the model regardless of `env_prefix` unless `dotenv_filtering` is used. With the default `extra="forbid"`, unknown dotenv entries can raise `ValidationError`; for v1 compatibility, use `extra="ignore"`. ([Pydantic Docs][S17-1])

### Dotenv filtering

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_file=".env",
        env_prefix="APP_",
        dotenv_filtering="match_prefix",
    )
```

Modes:

```text
match_prefix:
  pass only dotenv variables matching env_prefix

only_existing:
  pass only variables corresponding to model fields

default:
  all dotenv variables passed to model
```

Pydantic settings documents `dotenv_filtering="match_prefix"` and `"only_existing"` as ways to avoid unknown dotenv entries being passed to the settings model. ([Pydantic Docs][S17-1])

---

## 17.17 Secrets directories

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(secrets_dir="/run/secrets")

    database_password: str
```

File layout:

```text
/run/secrets/database_password
  super_secret_database_password
```

Runtime override:

```python
settings = Settings(_secrets_dir="/run/secrets")
```

Semantics:

```text
file name:
  setting key

file content:
  setting value

nonexistent directory:
  warning, ignored

path is file:
  error

multiple directories:
  tuple/list accepted
  later paths override earlier paths

environment / dotenv:
  take priority over secrets directory
```

Pydantic settings supports loading secret files from `secrets_dir` or `_secrets_dir`; env vars and dotenv values take priority over secrets; multiple secrets directories can be supplied, with later paths overriding earlier ones; missing dirs are warned/ignored and file paths error. ([Pydantic Docs][S17-1])

### Docker secrets

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(secrets_dir="/run/secrets")

    my_secret_data: str
```

Docker secrets commonly mount files into `/run/secrets`; the settings docs include Docker secrets as a first-class use case through `secrets_dir="/run/secrets"`. ([Pydantic Docs][S17-1])

### Kubernetes secrets

Kubernetes Secrets can be exposed to containers as environment variables or mounted as data volumes. For file-mounted secrets, point `secrets_dir` to the mount directory; for env-injected secrets, use normal env names or `validation_alias`. ([Kubernetes][S17-3])

---

## 17.18 CLI settings source

```python
import sys
from pydantic import BaseModel
from pydantic_settings import BaseSettings, SettingsConfigDict

class DeepSubModel(BaseModel):
    v4: str

class SubModel(BaseModel):
    v1: str
    v2: bytes
    v3: int
    deep: DeepSubModel

class Settings(BaseSettings):
    model_config = SettingsConfigDict(cli_parse_args=True)

    v0: str
    sub_model: SubModel

sys.argv = [
    "example.py",
    "--v0=0",
    '--sub_model={"v1": "json-1", "v2": "json-2"}',
    "--sub_model.v2=nested-2",
    "--sub_model.v3=3",
    "--sub_model.deep.v4=v4",
]

settings = Settings()
```

Pydantic settings includes integrated CLI support; setting `cli_parse_args=True` enables parsing `sys.argv` using nested dot syntax for submodels, and the CLI source is the topmost source by default when enabled unless source priority is customized. ([Pydantic Docs][S17-1])

Policy:

```text
Settings CLI good for:
  simple config overrides
  internal scripts
  small utilities
  config inspection tools

Dedicated CLI framework better for:
  subcommands
  rich help UX
  shell completion
  command dispatch
  interactive features
```

---

## 17.19 Custom settings sources

```python
from typing import Any
from pydantic.fields import FieldInfo
from pydantic_settings import (
    BaseSettings,
    EnvSettingsSource,
    PydanticBaseSettingsSource,
)

class CsvEnvSource(EnvSettingsSource):
    def prepare_field_value(
        self,
        field_name: str,
        field: FieldInfo,
        value: Any,
        value_is_complex: bool,
    ) -> Any:
        if field_name == "numbers":
            return [int(x) for x in value.split(",")]
        return super().prepare_field_value(field_name, field, value, value_is_complex)

class Settings(BaseSettings):
    numbers: list[int]

    @classmethod
    def settings_customise_sources(
        cls,
        settings_cls: type[BaseSettings],
        init_settings: PydanticBaseSettingsSource,
        env_settings: PydanticBaseSettingsSource,
        dotenv_settings: PydanticBaseSettingsSource,
        file_secret_settings: PydanticBaseSettingsSource,
    ) -> tuple[PydanticBaseSettingsSource, ...]:
        return (CsvEnvSource(settings_cls),)
```

`settings_customise_sources` returns the source callables to use, and a custom `EnvSettingsSource` can override `prepare_field_value` to parse custom formats. ([Pydantic Docs][S17-1])

### Changing priority

```python
class Settings(BaseSettings):
    database_dsn: PostgresDsn

    @classmethod
    def settings_customise_sources(
        cls,
        settings_cls: type[BaseSettings],
        init_settings: PydanticBaseSettingsSource,
        env_settings: PydanticBaseSettingsSource,
        dotenv_settings: PydanticBaseSettingsSource,
        file_secret_settings: PydanticBaseSettingsSource,
    ) -> tuple[PydanticBaseSettingsSource, ...]:
        return env_settings, init_settings, file_secret_settings
```

The order returned by `settings_customise_sources` determines priority; first returned source has highest priority. ([Pydantic Docs][S17-1])

### Adding sources

```python
import json
from pathlib import Path
from typing import Any
from pydantic.fields import FieldInfo
from pydantic_settings import BaseSettings, PydanticBaseSettingsSource

class JsonConfigSettingsSource(PydanticBaseSettingsSource):
    def get_field_value(
        self,
        field: FieldInfo,
        field_name: str,
    ) -> tuple[Any, str, bool]:
        data = json.loads(Path("config.json").read_text())
        return data.get(field_name), field_name, False

    def prepare_field_value(
        self,
        field_name: str,
        field: FieldInfo,
        value: Any,
        value_is_complex: bool,
    ) -> Any:
        return value

    def __call__(self) -> dict[str, Any]:
        result = {}
        for field_name, field in self.settings_cls.model_fields.items():
            value, key, value_is_complex = self.get_field_value(field, field_name)
            value = self.prepare_field_value(field_name, field, value, value_is_complex)
            if value is not None:
                result[key] = value
        return result
```

Custom source classes should inherit from `PydanticBaseSettingsSource`, implement `get_field_value`, and return a dictionary from `__call__`; Pydantic’s docs show a JSON file source using exactly this pattern. ([Pydantic Docs][S17-1])

---

## 17.20 Source state in custom sources

```python
class MySource(PydanticBaseSettingsSource):
    def __call__(self) -> dict[str, Any]:
        current_state = self.current_state
        settings_sources_data = self.settings_sources_data
        ...
```

Custom settings sources can access `current_state`, the aggregate output from previous sources, and `settings_sources_data`, the per-source data from earlier sources. ([Pydantic Docs][S17-1])

Use cases:

```text
derived source based on earlier source
audit/debug source precedence
conditional file lookup based on init/env value
fallback only if no previous value exists
```

---

## 17.21 TOML / pyproject sources

```python
from pydantic_settings import (
    BaseSettings,
    PydanticBaseSettingsSource,
    PyprojectTomlConfigSettingsSource,
    SettingsConfigDict,
)

class Settings(BaseSettings):
    field: str

    @classmethod
    def settings_customise_sources(
        cls,
        settings_cls: type[BaseSettings],
        init_settings: PydanticBaseSettingsSource,
        env_settings: PydanticBaseSettingsSource,
        dotenv_settings: PydanticBaseSettingsSource,
        file_secret_settings: PydanticBaseSettingsSource,
    ) -> tuple[PydanticBaseSettingsSource, ...]:
        return (PyprojectTomlConfigSettingsSource(settings_cls),)
```

Default `pyproject.toml` table:

```toml
[tool.pydantic-settings]
field = "value"
```

`PyprojectTomlConfigSettingsSource` loads from `pyproject.toml`, defaulting to the `[tool.pydantic-settings]` table. `pyproject_toml_depth` can search parent directories, and an explicit file path can also be passed when the source is instantiated. ([Pydantic Docs][S17-1])

Agent rule:

```text
Use pyproject source for:
  local tool/library configuration
  developer defaults
  non-secret project config

Avoid for:
  production secrets
  per-deploy mutable config
```

---

## 17.22 Settings class layout

Recommended layout:

```text
src/myapp/
├─ settings.py
├─ settings_sources.py
├─ config/
│  └─ defaults.py
└─ main.py
```

```python
# settings.py
from functools import lru_cache
from pydantic import Field, PostgresDsn, SecretStr
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_prefix="MYAPP_",
        env_file=".env",
        env_file_encoding="utf-8",
        secrets_dir="/run/secrets",
        env_nested_delimiter="__",
        extra="ignore",
    )

    environment: str = "development"
    database_url: PostgresDsn
    redis_url: str = "redis://localhost:6379/0"
    secret_key: SecretStr
    log_level: str = "INFO"

@lru_cache
def get_settings() -> Settings:
    return Settings()
```

Usage:

```python
# main.py
from myapp.settings import get_settings

def create_app():
    settings = get_settings()
    ...
```

Agent rule:

```text
Define Settings class at import time.
Construct Settings object at app startup or through cached factory.
Never run Settings() as an import-time module global in libraries.
```

---

## 17.23 Avoiding import-time settings construction

Anti-pattern:

```python
# bad
settings = Settings()
```

Problems:

```text
reads environment at import
breaks tests that monkeypatch env after import
makes imports side-effecting
can fail import if env missing
harder for CLI/tests to override
harder for multi-tenant/multi-instance configs
```

Preferred:

```python
from functools import lru_cache

@lru_cache
def get_settings() -> Settings:
    return Settings()
```

Test reset:

```python
def test_config(monkeypatch):
    get_settings.cache_clear()
    monkeypatch.setenv("MYAPP_DATABASE_URL", "postgres://...")
    settings = get_settings()
```

This follows the documented behavior that initializer kwargs and environment variables are evaluated when the settings class is instantiated, not when the class is defined. ([Pydantic Docs][S17-1])

---

## 17.24 Testing with env monkeypatching

```python
def test_settings_from_env(monkeypatch):
    monkeypatch.setenv("MYAPP_DATABASE_URL", "postgres://user:pass@localhost:5432/test")
    monkeypatch.setenv("MYAPP_DEBUG", "true")

    settings = Settings()

    assert str(settings.database_url).startswith("postgres://")
    assert settings.debug is True
```

### Testing aliases

```python
def test_alias_choices(monkeypatch):
    class Settings(BaseSettings):
        redis_dsn: str = Field(validation_alias=AliasChoices("SERVICE_REDIS_DSN", "REDIS_URL"))

    monkeypatch.setenv("REDIS_URL", "redis://localhost:6379/0")

    assert Settings().redis_dsn == "redis://localhost:6379/0"
```

### Testing dotenv

```python
def test_dotenv(tmp_path):
    env_file = tmp_path / ".env"
    env_file.write_text("APP_PORT=8000\n")

    class Settings(BaseSettings):
        model_config = SettingsConfigDict(env_prefix="APP_")
        port: int

    settings = Settings(_env_file=env_file)

    assert settings.port == 8000
```

### Testing secrets directory

```python
def test_secrets_dir(tmp_path):
    (tmp_path / "database_password").write_text("secret")

    class Settings(BaseSettings):
        database_password: str

    settings = Settings(_secrets_dir=tmp_path)

    assert settings.database_password == "secret"
```

Agent testing rule:

```text
Use monkeypatch.setenv for env tests.
Use monkeypatch.delenv to prove missing behavior.
Use tmp_path for dotenv/secrets tests.
Construct Settings after env/files are prepared.
Clear cached get_settings() between tests.
```

---

## 17.25 12-factor deployment policy

The Twelve-Factor App config principle says deploy-varying config should be stored in environment variables because env vars can change between deploys without code changes and are language/OS agnostic. ([Twelve-Factor App][S17-4])

Settings mapping:

```text
12-factor env config:
  BaseSettings + env vars

local dev convenience:
  dotenv file

secrets manager / Docker / Kubernetes:
  secrets_dir or env-injected secrets

CI overrides:
  environment variables or explicit Settings(...)
```

Policy:

```text
Do:
  keep deploy-varying config out of code
  validate config at startup
  fail fast on missing required config
  keep defaults only for non-secret safe values
  use explicit aliases for public env names

Do not:
  commit production secrets in .env
  instantiate settings at library import time
  silently ignore required secrets
  use untyped global os.environ access across codebase
```

---

## 17.26 Docker / Kubernetes deployment

### Docker secrets

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(secrets_dir="/run/secrets")

    database_password: SecretStr
```

Docker secret file:

```text
/run/secrets/database_password
```

Pydantic settings documents `/run/secrets` as the Docker secrets use case. ([Pydantic Docs][S17-1])

### Kubernetes

Options:

```text
Secret as env var:
  normal env source
  Field(validation_alias=...)

Secret as volume file:
  secrets_dir="/mounted/secret/path"
```

Kubernetes supports mounting Secrets as volumes or exposing them as environment variables. ([Kubernetes][S17-3])

Agent rule:

```text
Prefer mounted secret files for high-sensitivity secrets when platform policy supports it.
Use env vars for non-secret deploy configuration and simple CI overrides.
Keep field names / aliases aligned with platform secret keys.
```

---

## 17.27 CI override patterns

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="APP_")

    database_url: str
    environment: str = "test"
```

CI environment:

```bash
export APP_DATABASE_URL=postgres://user:pass@localhost:5432/test
export APP_ENVIRONMENT=ci
```

Test override:

```python
settings = Settings(database_url="sqlite:///:memory:", environment="test")
```

Priority consequence:

```text
default priority:
  init kwargs beat environment
  environment beats dotenv
  dotenv beats secrets
  secrets beat defaults
```

If CI should override explicit kwargs, customize source order so `env_settings` precedes `init_settings`. Pydantic documents source priority customization and shows env settings being moved before init settings. ([Pydantic Docs][S17-1])

---

## 17.28 Complete production pattern

```python
from functools import lru_cache
from typing import Literal

from pydantic import AliasChoices, Field, PostgresDsn, RedisDsn, SecretStr
from pydantic_settings import BaseSettings, SettingsConfigDict

class DatabaseSettings(BaseModel):
    url: PostgresDsn
    pool_size: int = Field(default=10, ge=1, le=100)

class RedisSettings(BaseModel):
    url: RedisDsn = "redis://localhost:6379/0"

class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_prefix="MYAPP_",
        env_prefix_target="variable",
        case_sensitive=False,
        env_file=".env",
        env_file_encoding="utf-8",
        dotenv_filtering="match_prefix",
        secrets_dir="/run/secrets",
        env_nested_delimiter="__",
        env_nested_max_split=2,
        extra="ignore",
        validate_default=True,
    )

    environment: Literal["development", "staging", "production", "test"] = "development"

    database: DatabaseSettings
    redis: RedisSettings = RedisSettings()

    secret_key: SecretStr = Field(
        validation_alias=AliasChoices("MYAPP_SECRET_KEY", "SECRET_KEY")
    )

    log_level: Literal["DEBUG", "INFO", "WARNING", "ERROR"] = "INFO"

@lru_cache
def get_settings() -> Settings:
    return Settings()
```

Example env:

```bash
export MYAPP_ENVIRONMENT=production
export MYAPP_DATABASE__URL=postgres://user:pass@db:5432/app
export MYAPP_DATABASE__POOL_SIZE=20
export MYAPP_SECRET_KEY=super-secret
```

---

## 17.29 Deployment decision matrix

```text
Need                                      Mechanism
--------------------------------------------------------------------------------
read env vars                             BaseSettings
prefix all variables                      SettingsConfigDict(env_prefix="APP_")
one field custom env name                 Field(validation_alias="CUSTOM_ENV")
multiple legacy env names                 AliasChoices("NEW", "OLD")
case-sensitive env names                  SettingsConfigDict(case_sensitive=True)
complex object from one env var           JSON string env value
individual nested env vars                env_nested_delimiter="__"
limit nested split depth                  env_nested_max_split=n
dotenv local dev                          env_file=".env"
filter shared dotenv                      dotenv_filtering="match_prefix" / "only_existing"
file-mounted secrets                      secrets_dir="/run/secrets"
override secrets path per instance        Settings(_secrets_dir=...)
CLI override                              cli_parse_args=True
custom config source                      settings_customise_sources(...)
pyproject config                          PyprojectTomlConfigSettingsSource
TOML/YAML/JSON config                     built-in config source classes
comma-separated custom parsing            NoDecode + before validator or custom EnvSettingsSource
validate default config                   default BaseSettings behavior
skip default validation                   validate_default=False
avoid import-time env reads               cached get_settings() factory
```

---

## 17.30 Anti-patterns

### Import-time construction

```python
# bad
settings = Settings()
```

Preferred:

```python
@lru_cache
def get_settings() -> Settings:
    return Settings()
```

---

### Scattered `os.environ`

```python
database_url = os.environ["DATABASE_URL"]
```

Preferred:

```python
settings = Settings()
settings.database_url
```

---

### Production secrets in `.env`

```text
.env committed to repo with SECRET_KEY=...
```

Preferred:

```text
local .env for non-production dev convenience
production env vars or secrets_dir / platform secret manager
```

---

### Global strict settings model

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(strict=True)
    port: int
```

Problem:

```text
env vars are strings
strict=True rejects normal env parsing
```

Preferred:

```python
class Settings(BaseSettings):
    port: int = Field(ge=1, le=65535)
```

---

### Unknown dotenv values with default `extra="forbid"`

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_file=".env")
```

Problem:

```text
shared .env contains unrelated variables
settings construction raises ValidationError
```

Preferred:

```python
model_config = SettingsConfigDict(
    env_file=".env",
    extra="ignore",
)
```

or:

```python
model_config = SettingsConfigDict(
    env_file=".env",
    env_prefix="APP_",
    dotenv_filtering="match_prefix",
)
```

Pydantic settings documents that dotenv loads all values unless filtering is enabled, and unknown entries can error with `extra="forbid"`. ([Pydantic Docs][S17-1])

---

## 17.31 Test matrix

### Env override

```python
def test_env_override(monkeypatch):
    class Settings(BaseSettings):
        model_config = SettingsConfigDict(env_prefix="APP_")
        port: int = 8000

    monkeypatch.setenv("APP_PORT", "9000")

    assert Settings().port == 9000
```

### Init beats env by default

```python
def test_init_beats_env(monkeypatch):
    class Settings(BaseSettings):
        x: int

    monkeypatch.setenv("X", "1")

    assert Settings(x=2).x == 2
```

### AliasChoices

```python
def test_alias_choices(monkeypatch):
    class Settings(BaseSettings):
        redis_url: str = Field(validation_alias=AliasChoices("SERVICE_REDIS_DSN", "REDIS_URL"))

    monkeypatch.setenv("REDIS_URL", "redis://localhost:6379/0")

    assert Settings().redis_url == "redis://localhost:6379/0"
```

### Nested delimiter

```python
def test_nested_env(monkeypatch):
    class DB(BaseModel):
        url: str
        pool_size: int = 10

    class Settings(BaseSettings):
        model_config = SettingsConfigDict(env_nested_delimiter="__")
        database: DB

    monkeypatch.setenv("DATABASE__URL", "postgres://user:pass@localhost:5432/test")
    monkeypatch.setenv("DATABASE__POOL_SIZE", "20")

    s = Settings()

    assert s.database.pool_size == 20
```

### Dotenv

```python
def test_dotenv(tmp_path):
    env = tmp_path / ".env"
    env.write_text("APP_PORT=9000\n")

    class Settings(BaseSettings):
        model_config = SettingsConfigDict(env_prefix="APP_")
        port: int

    assert Settings(_env_file=env).port == 9000
```

### Secrets dir

```python
def test_secrets_dir(tmp_path):
    (tmp_path / "secret_key").write_text("secret")

    class Settings(BaseSettings):
        secret_key: str

    assert Settings(_secrets_dir=tmp_path).secret_key == "secret"
```

### Cached factory reset

```python
def test_cached_settings(monkeypatch):
    get_settings.cache_clear()
    monkeypatch.setenv("MYAPP_DATABASE_URL", "postgres://user:pass@localhost:5432/test")
    settings = get_settings()
    assert settings.database_url
    get_settings.cache_clear()
```

---

## 17.32 Agent checklist

```text
[ ] Install pydantic-settings separately.
[ ] Define one Settings class per application boundary.
[ ] Keep Settings class import-safe; construct Settings at startup.
[ ] Use env_prefix for project namespacing.
[ ] Use validation_alias for explicit env names.
[ ] Use AliasChoices for env migrations.
[ ] Keep BaseSettings defaults validated unless intentionally disabled.
[ ] Use env_nested_delimiter for nested settings.
[ ] Set env_nested_max_split when delimiter may appear in field names.
[ ] Use dotenv for local/dev only; do not rely on it for production secrets.
[ ] Set extra="ignore" or dotenv_filtering when .env may contain unrelated keys.
[ ] Use secrets_dir for file-mounted Docker/Kubernetes secrets.
[ ] Remember env and dotenv override secrets_dir.
[ ] Use cli_parse_args only when settings-as-CLI is appropriate.
[ ] Customize sources only when default priority is wrong.
[ ] Use custom sources for JSON/TOML/YAML/cloud secrets as needed.
[ ] Test source priority explicitly.
[ ] Use monkeypatch.setenv and tmp_path in tests.
[ ] Clear cached settings factories between tests.
[ ] Avoid global os.environ reads outside Settings.
[ ] Avoid strict=True globally for env-string settings.
```

---

## 17.33 Value case

```text
pydantic-settings value =
  typed deployment configuration
  one validated config object
  explicit env var contract
  aliases for operational naming and migrations
  nested config from JSON or delimiter-expanded env vars
  dotenv support for local development
  secrets directory support for container platforms
  CLI/config source composition
  source priority control
  default validation
  testable override surfaces
  12-factor-friendly environment configuration
```

Operationally: centralize all deployment-varying configuration in a `BaseSettings` subclass, keep settings construction out of import time, use env aliases and nested delimiters to match platform naming, treat dotenv as a local-development convenience, map file-mounted secrets through `secrets_dir`, and test source priority and redaction-sensitive settings paths explicitly.

[S17-1]: https://docs.pydantic.dev/latest/concepts/pydantic_settings/ "Settings Management | Pydantic Docs"
[S17-2]: https://docs.pydantic.dev/latest/api/pydantic_settings/ "Pydantic Settings | Pydantic Docs"
[S17-3]: https://kubernetes.io/docs/concepts/configuration/secret/?utm_source=chatgpt.com "Secrets"
[S17-4]: https://12factor.net/config?utm_source=chatgpt.com "Store config in the environment"


# Pydantic Advanced — 18) Performance engineering — Pydantic v2

Style target: advanced, sectioned, agent-oriented reference. 

Pydantic’s official performance guidance starts with a constraint: in most applications, Pydantic will not be the bottleneck; apply these optimizations only after measuring. The highest-leverage official recommendations are: prefer `model_validate_json()` for raw JSON, instantiate `TypeAdapter` once, prefer concrete containers over abstract containers, avoid unnecessary validation with `Any`, prefer tagged unions, prefer `TypedDict` over nested models in hot paths, avoid wrap validators when performance matters, and use `FailFast` when first-error-only sequence validation is acceptable. ([Pydantic Docs][S18-1])

---

## 18.0 Performance mental model

```text
cost center =
  schema build cost
  + JSON parsing cost
  + validation traversal cost
  + Python callback cost
  + object construction cost
  + error aggregation cost
  + serialization cost
```

Primary optimization levers:

```text
schema build:
  reuse TypeAdapter
  avoid per-call adapter construction
  defer_build for cold-start-sensitive rare schemas

input decoding:
  use model_validate_json for raw JSON
  avoid json.loads + model_validate unless measured faster for before/wrap-heavy schemas

schema shape:
  concrete list/dict over Sequence/Mapping
  tagged/discriminated union over broad Union
  TypedDict over nested BaseModel when object methods are not needed
  Any for values that do not require validation

callback cost:
  prefer Field constraints / annotated-types
  prefer after validators over wrap validators
  avoid wrap validators in hot paths

error cost:
  FailFast for long sequences when only first error matters
  strict early rejection when useful
  avoid collecting irrelevant union branch errors with discriminators
```

---

## 18.1 Baseline rule: measure first

```text
Do not pre-optimize every model.
Do optimize:
  hot validation loops
  high-throughput JSON ingestion
  serverless cold start
  streaming/batch pipelines
  repeated TypeAdapter use
  large nested payloads
  large unions
  sequence-heavy validation
```

Official stance:

```text
Pydantic usually is not the bottleneck.
Follow performance guidance when profiling indicates validation/serialization cost matters.
```

The performance docs explicitly say not to follow the performance guide unless you are sure Pydantic is the bottleneck. ([Pydantic Docs][S18-1])

---

## 18.2 Reuse `TypeAdapter` instances

### Bad

```python
from pydantic import TypeAdapter

def parse_ids(raw: object) -> list[int]:
    adapter = TypeAdapter(list[int])
    return adapter.validate_python(raw)
```

### Good

```python
from pydantic import TypeAdapter

IDS_ADAPTER = TypeAdapter(list[int])

def parse_ids(raw: object) -> list[int]:
    return IDS_ADAPTER.validate_python(raw)
```

Reason:

```text
TypeAdapter construction:
  builds core schema
  builds validator
  builds serializer

Per-call construction:
  repeats schema/validator/serializer setup

Module-level reuse:
  pays schema build once
```

Pydantic documents this directly: each `TypeAdapter` instantiation constructs a new validator and serializer; instantiate once and reuse when used repeatedly. ([Pydantic Docs][S18-1])

### Generic cache pattern

```python
from functools import lru_cache
from typing import Any

from pydantic import TypeAdapter

@lru_cache(maxsize=None)
def adapter_for(type_repr: str) -> TypeAdapter[Any]:
    # Only use if type_repr maps deterministically to a known type.
    return TypeAdapter(TYPE_REGISTRY[type_repr])
```

Policy:

```text
Known static type:
  module-level TypeAdapter

Many dynamic but finite types:
  cache by stable key

Unbounded runtime-generated types:
  avoid dynamic adapter generation if possible
```

---

## 18.3 Prefer concrete containers

### Slower / broader

```python
from collections.abc import Mapping, Sequence
from pydantic import BaseModel

class Model(BaseModel):
    xs: Sequence[int]
    meta: Mapping[str, int]
```

### Faster / narrower

```python
class Model(BaseModel):
    xs: list[int]
    meta: dict[str, int]
```

Reason:

```text
Sequence:
  isinstance(value, Sequence)
  tries multiple sequence-like paths
  broader accepted input surface

Mapping:
  broader accepted input surface

list / tuple / dict:
  direct concrete validation
  less branch work
  more precise contract
```

Pydantic’s performance docs recommend using `list` or `tuple` instead of `Sequence` when the input is known, and `dict` instead of `Mapping` when the input is known to be a dictionary. ([Pydantic Docs][S18-1])

Agent rule:

```text
Public JSON arrays:
  list[T]

Fixed positional record:
  tuple[T1, T2, ...]

Object mapping:
  dict[str, T]

Only use Sequence/Mapping when abstraction is part of the contract.
```

---

## 18.4 `model_validate_json()` vs `json.loads(...) + model_validate(...)`

### Preferred for raw JSON

```python
class Event(BaseModel):
    id: int
    kind: str

event = Event.model_validate_json(raw_bytes)
```

### Usually avoid

```python
import json

event = Event.model_validate(json.loads(raw_bytes))
```

Reason:

```text
json.loads + model_validate:
  parse JSON in Python
  allocate dict/list tree
  validate Python object

model_validate_json:
  parse and validate internally
  avoids separate Python JSON parse step
```

Pydantic’s performance docs recommend `model_validate_json()` instead of `model_validate(json.loads(...))` in general, because the latter parses JSON in Python and then validates the resulting dictionary; the docs also note a few special cases where two-step parsing may be faster, specifically with model-level `before` or `wrap` validators. ([Pydantic Docs][S18-1])

### Decision rule

```text
Raw JSON + ordinary model:
  model_validate_json

Raw JSON + heavy model before/wrap validator:
  benchmark both

Already-decoded dict:
  model_validate

Non-JSON stringly map:
  model_validate_strings
```

### Benchmark harness

```python
import json
from timeit import timeit

def bench_json(model: type[BaseModel], raw: bytes, n: int = 100_000) -> dict[str, float]:
    return {
        "model_validate_json": timeit(lambda: model.model_validate_json(raw), number=n),
        "json_loads_model_validate": timeit(
            lambda: model.model_validate(json.loads(raw)),
            number=n,
        ),
    }
```

---

## 18.5 Avoid unnecessary wrap validators

### Expensive shape

```python
from typing import Any
from pydantic import BaseModel, ValidatorFunctionWrapHandler, field_validator

class Model(BaseModel):
    x: int

    @field_validator("x", mode="wrap")
    @classmethod
    def validate_x(cls, v: Any, handler: ValidatorFunctionWrapHandler) -> int:
        value = handler(v)
        if value < 0:
            raise ValueError("negative")
        return value
```

### Cheaper shape

```python
from pydantic import BaseModel, field_validator

class Model(BaseModel):
    x: int

    @field_validator("x")
    @classmethod
    def validate_x(cls, value: int) -> int:
        if value < 0:
            raise ValueError("negative")
        return value
```

### Cheapest shape when expressible

```python
from typing import Annotated
from pydantic import BaseModel, Field

PositiveInt = Annotated[int, Field(ge=0)]

class Model(BaseModel):
    x: PositiveInt
```

Cost hierarchy:

```text
Field constraints / annotated-types:
  fastest and schema-native

after validator:
  Python callback after typed validation

before validator:
  Python callback before typed validation

wrap validator:
  Python callback with handler
  materializes data in Python
  generally slower
```

Pydantic’s performance docs explicitly warn that wrap validators are generally slower because they require data to be materialized in Python during validation; wrap validators remain useful for complex logic but should be avoided in performance-sensitive paths when simpler validators suffice. ([Pydantic Docs][S18-1])

Agent rule:

```text
Use wrap only for:
  retry/fallback after error
  handler short-circuit
  instrumentation around inner validation
  preserving/restoring complex structure

Do not use wrap for:
  simple range checks
  simple normalization
  basic typed semantic checks
```

---

## 18.6 Avoid validating known-good data repeatedly

### Bad

```python
def process_user(user: User) -> None:
    user = User.model_validate(user)  # usually unnecessary
    ...
```

### Better

```python
def process_user(user: User) -> None:
    ...
```

### Boundary-only validation

```python
def handle_request(raw: bytes) -> None:
    command = Command.model_validate_json(raw)
    service.handle(command)  # do not revalidate at every internal call
```

### Use `Any` for intentionally opaque data

```python
from typing import Any
from pydantic import BaseModel

class EventEnvelope(BaseModel):
    event_id: str
    payload: Any  # keep unchanged; no validation cost
```

Pydantic’s performance docs recommend using `Any` when a value does not need validation so the value is kept unchanged. ([Pydantic Docs][S18-1])

Agent rule:

```text
Validate:
  at ingress
  at trust boundary
  before persistence if schema matters
  after untrusted deserialization

Do not revalidate:
  every service-layer call
  already-normalized objects
  trusted internal DTOs
  cached validated models
```

---

## 18.7 `model_construct()` tradeoffs

```python
user = User.model_construct(id=1, name="Ada")
```

Meaning:

```text
model_construct:
  creates model instance from trusted/prevalidated data
  skips validation
  skips coercion
  may create invalid model states
  may be useful for trusted cache hydration or test fixtures
```

Use cases:

```text
trusted database row already normalized
internal cache rehydration
deserialization from your own previously validated format
test fixture requiring invalid state
framework internals
```

Avoid:

```text
HTTP bodies
LLM output
env vars
CLI input
queue messages
third-party API payloads
user files
anything crossing trust boundary
```

Important nuance:

```text
Pydantic v2 narrowed the performance gap between validated construction and model_construct.
For simple models, model_construct may be slower than normal validation.
Benchmark before replacing validation.
```

`model_construct()` is explicitly a no-validation construction path for trusted/prevalidated data; its behavior differs from normal validation, including how extra values are handled. The BaseModel API documents it as trusted-data construction, not a general performance shortcut. ([Pydantic Docs][S18-2])

### Safe wrapper pattern

```python
from typing import TypeVar

M = TypeVar("M", bound=BaseModel)

def construct_from_trusted_cache(model: type[M], data: dict) -> M:
    # Use only when cache content is produced by this service from validated data.
    return model.model_construct(**data)
```

### Benchmark decision

```python
from timeit import timeit

def compare_construct(model: type[BaseModel], data: dict, n: int = 100_000) -> dict[str, float]:
    return {
        "validate": timeit(lambda: model.model_validate(data), number=n),
        "construct": timeit(lambda: model.model_construct(**data), number=n),
    }
```

---

## 18.8 `FailFast` for sequences

```python
from typing import Annotated

from pydantic import FailFast, TypeAdapter, ValidationError

BOOLS = TypeAdapter(Annotated[list[bool], FailFast()])

try:
    BOOLS.validate_python([True, "invalid", False, "also invalid"])
except ValidationError as exc:
    print(exc.errors())
```

Semantics:

```text
without FailFast:
  validate entire sequence
  collect all item errors

with FailFast:
  stop at first invalid item
  return only first item error
  trade error visibility for speed
```

`FailFast` was introduced in Pydantic v2.8+ for sequence types; it stops sequence validation at the first failing item, so later item errors are not collected. ([Pydantic Docs][S18-1])

Use cases:

```text
large arrays
batch ingestion where first error rejects whole batch
low-latency validation
DoS-resistance when pathological payloads contain many invalid items
LLM/tool arguments where first error is enough
```

Avoid:

```text
UX requiring all row/item errors
batch reports needing full diagnostics
data cleaning tools
spreadsheet import with full error report
```

### Pattern

```python
from typing import Annotated
from pydantic import FailFast, Field

PositiveIntBatch = Annotated[
    list[Annotated[int, Field(gt=0)]],
    FailFast(),
]
```

---

## 18.9 Deferred schema building

```python
from pydantic import BaseModel, ConfigDict

class RareModel(BaseModel):
    model_config = ConfigDict(defer_build=True)

    x: int
    y: list[str]
```

Semantics:

```text
defer_build=True:
  delay validator/serializer construction until first validation
  reduce import/cold-start cost
  first validation pays build cost
```

`ConfigDict(defer_build=True)` delays construction of validators and serializers until first use; current docs note it applies to models, Pydantic dataclasses, and TypeAdapters. ([Pydantic Docs][S18-3])

### `TypeAdapter` deferred build

```python
from pydantic import ConfigDict, TypeAdapter

RARE_ADAPTER = TypeAdapter(
    list[dict[str, int]],
    config=ConfigDict(defer_build=True),
)
```

Use cases:

```text
serverless cold start
large optional schema graphs
rare admin endpoints
rare CLI subcommands
plugin systems
forward-reference resolution control
```

Avoid:

```text
hot request path where first-request latency matters
startup validation should fail fast
schemas that must be prewarmed
```

### Prewarm pattern

```python
def prewarm_validation() -> None:
    CriticalModel.model_validate(SAMPLE_CRITICAL_PAYLOAD)
    CRITICAL_ADAPTER.validate_python(SAMPLE_CRITICAL_VALUE)
```

Policy:

```text
Cold start priority:
  defer rare schemas
  prewarm critical schemas

Steady-state latency priority:
  build critical schemas at startup
```

---

## 18.10 Concrete schema shape choices

### Prefer tagged unions

```python
from typing import Literal
from pydantic import BaseModel, Field

class Cat(BaseModel):
    kind: Literal["cat"]
    meows: int

class Dog(BaseModel):
    kind: Literal["dog"]
    barks: int

class PetBox(BaseModel):
    pet: Cat | Dog = Field(discriminator="kind")
```

Avoid broad untagged union:

```python
class PetBox(BaseModel):
    pet: Cat | Dog
```

Reason:

```text
untagged union:
  tries multiple branches
  may collect multiple branch errors
  ambiguity cost

discriminated union:
  read tag
  validate selected branch
  clearer errors
  better performance
```

Pydantic’s performance docs recommend tagged/discriminated unions over ordinary unions, and the unions docs describe discriminated unions as more predictable and performant. ([Pydantic Docs][S18-1])

---

## 18.11 Prefer `TypedDict` over nested models in hot paths

### Faster dict-shaped schema

```python
from typing_extensions import TypedDict
from pydantic import TypeAdapter

class AddressTD(TypedDict):
    city: str
    zip_code: int

class UserTD(TypedDict):
    name: str
    address: AddressTD

USER_TD = TypeAdapter(UserTD)
```

### Richer but heavier model object

```python
class Address(BaseModel):
    city: str
    zip_code: int

class User(BaseModel):
    name: str
    address: Address
```

Tradeoff:

```text
TypedDict:
  plain dict output
  generally faster
  less object construction
  no model methods
  no computed fields
  no model serializers

BaseModel:
  object instances
  methods
  validators/serializers
  model_dump/model_json_schema
  field tracking
```

Pydantic’s performance docs recommend `TypedDict` over nested models for performance-sensitive paths and show a simple benchmark where `TypedDict` validation was about 2.5x faster than nested models. ([Pydantic Docs][S18-1])

Agent rule:

```text
Use TypedDict when:
  data stays dict-shaped
  no object methods needed
  hot nested validation path

Use BaseModel when:
  model identity and methods matter
  serialization/custom hooks matter
  public DTO class matters
```

---

## 18.12 Avoid primitive subclasses for extra state

### Bad

```python
class CompletedStr(str):
    def __init__(self, s: str) -> None:
        self.done = False
```

### Better

```python
from pydantic import BaseModel

class CompletedModel(BaseModel):
    s: str
    done: bool = False
```

Pydantic’s performance docs recommend avoiding extra information via subclasses of primitives and representing additional data explicitly in a model instead. ([Pydantic Docs][S18-1])

Reason:

```text
primitive subclass:
  validation/serialization complexity
  hidden state
  harder schema
  weaker explicitness

model:
  explicit fields
  schema-visible state
  predictable validation
```

---

## 18.13 Batch validation patterns

### Top-level list adapter

```python
from pydantic import TypeAdapter

EVENTS = TypeAdapter(list[Event])

def parse_events(raw: bytes) -> list[Event]:
    return EVENTS.validate_json(raw)
```

Use when:

```text
wire payload is JSON array
batch should fail as a unit
all items use same schema
```

### Sequence with `FailFast`

```python
from typing import Annotated
from pydantic import FailFast, TypeAdapter

EVENTS_FAST = TypeAdapter(Annotated[list[Event], FailFast()])
```

Use when:

```text
first invalid item rejects entire batch
full error report is unnecessary
```

### Per-record partial success

```python
from pydantic import ValidationError

def parse_records(records: list[dict]) -> tuple[list[Event], list[dict]]:
    ok: list[Event] = []
    bad: list[dict] = []

    for i, record in enumerate(records):
        try:
            ok.append(Event.model_validate(record))
        except ValidationError as exc:
            bad.append(
                {
                    "index": i,
                    "errors": exc.errors(include_input=False, include_url=False),
                }
            )

    return ok, bad
```

Use when:

```text
batch import should collect row-level errors
UX requires all invalid rows
bad records can be skipped/quarantined
```

### Batch design matrix

```text
Fail whole batch + full diagnostics:
  TypeAdapter(list[T])

Fail whole batch + first error only:
  TypeAdapter(Annotated[list[T], FailFast()])

Partial success:
  per-record validation loop

Maximum throughput with plain dict output:
  TypeAdapter(list[TypedDictType])

Maximum object features:
  TypeAdapter(list[BaseModelSubclass])
```

---

## 18.14 Profiling validation hotspots

### Microbenchmark

```python
from timeit import timeit

def bench(name: str, fn, number: int = 100_000) -> tuple[str, float]:
    return name, timeit(fn, number=number)
```

### JSON path benchmark

```python
import json

raw = b'{"id": "1", "name": "Ada"}'
data = json.loads(raw)

print(bench("model_validate_json", lambda: User.model_validate_json(raw)))
print(bench("model_validate", lambda: User.model_validate(data)))
print(bench("json.loads + validate", lambda: User.model_validate(json.loads(raw))))
```

### Adapter construction benchmark

```python
print(bench("new adapter", lambda: TypeAdapter(list[int]).validate_python(["1", "2"])))

LIST_INT = TypeAdapter(list[int])
print(bench("reused adapter", lambda: LIST_INT.validate_python(["1", "2"])))
```

### Profiling strategy

```text
Measure:
  schema build time
  validation time
  serialization time
  JSON parse + validation
  hot validators
  union branch behavior
  error-heavy invalid inputs
  batch success vs batch failure

Tools:
  timeit for micro
  py-spy / scalene / cProfile for app
  pydantic-logfire or observability around validation boundaries
```

Agent rule:

```text
Benchmark with:
  representative payload size
  representative valid/invalid ratio
  actual validation entrypoint
  actual validators/serializers
  warm and cold cases separately
```

---

## 18.15 Performance vs error-detail tradeoffs

```text
More diagnostics:
  validate full sequences
  untagged unions may report many branch errors
  collect all batch row errors
  include context and locs
  richer validators

More speed:
  FailFast
  discriminated unions
  typed dicts
  concrete containers
  fewer Python callbacks
  Any for opaque fields
  per-record early exit
```

Decision matrix:

```text
Public form UX:
  full diagnostics preferred

Batch ETL report:
  per-record full diagnostics preferred

API request rejection:
  first error may be enough only if documented

High-throughput ingestion:
  FailFast + compact errors often acceptable

LLM repair loop:
  concise discriminated/first-error diagnostics often better than huge error trees
```

---

## 18.16 Serialization performance notes

### Avoid repeated dumps if cached output is acceptable

```python
payload = model.model_dump(mode="json")
json_text = model.model_dump_json()
```

Policy:

```text
Need Python dict:
  model_dump(mode="json")

Need JSON string:
  model_dump_json()

Need JSON bytes for top-level non-model:
  TypeAdapter(T).dump_json(...)
```

`model_dump_json()` returns a JSON string, while `TypeAdapter.dump_json()` returns bytes and exposes the same kinds of include/exclude/round-trip/fallback/context controls. ([Pydantic Docs][S18-2])

### Avoid serializer callbacks for simple formats

Prefer built-in config/standard serialization over custom Python callbacks when possible:

```text
datetime ISO:
  built-in JSON mode

bytes base64/hex:
  ConfigDict ser_json_bytes / val_json_bytes

Decimal string:
  built-in JSON behavior

custom serializer:
  use when output differs from built-in policy
```

Python serializer callbacks are flexible but add Python call overhead; treat them like validators in hot paths.

---

## 18.17 Configuration knobs with performance implications

```python
from pydantic import BaseModel, ConfigDict

class Model(BaseModel):
    model_config = ConfigDict(
        defer_build=True,
        cache_strings="keys",
        regex_engine="rust-regex",
    )
```

Relevant config:

```text
defer_build:
  reduce import/cold-start schema construction
  first validation pays schema build

cache_strings:
  cache repeated strings during validation
  "all" / True default
  "keys" useful when only object keys repeat
  "none" / False saves memory when repetition is low

regex_engine:
  "rust-regex" default
  non-backtracking, DDoS-resistant
  fewer Python regex features
  "python-re" supports more features but may be slower
```

Pydantic’s config docs describe `defer_build`, `cache_strings`, and `regex_engine`; the default Rust regex engine is non-backtracking and more DDoS resistant, while Python `re` supports more regex features but may be slower. ([Pydantic Docs][S18-3])

---

## 18.18 High-throughput API ingestion pattern

```python
from typing import Literal

from pydantic import BaseModel, ConfigDict, Field, TypeAdapter

class ClickEvent(BaseModel):
    model_config = ConfigDict(extra="forbid")

    event_type: Literal["click"]
    user_id: int
    ts_ms: int
    path: str

Batch = TypeAdapter(Annotated[list[ClickEvent], FailFast()])

def parse_batch(raw: bytes) -> list[ClickEvent]:
    return Batch.validate_json(raw)
```

Why:

```text
raw JSON array:
  validate_json

top-level list:
  TypeAdapter

first invalid item rejects batch:
  FailFast

concrete list:
  direct collection validation

Literal tag:
  stable event type
```

---

## 18.19 High-throughput dict-only pattern

```python
from typing_extensions import TypedDict
from pydantic import TypeAdapter

class ClickEventTD(TypedDict):
    event_type: Literal["click"]
    user_id: int
    ts_ms: int
    path: str

CLICK_BATCH = TypeAdapter(Annotated[list[ClickEventTD], FailFast()])

def parse_batch(raw: bytes) -> list[ClickEventTD]:
    return CLICK_BATCH.validate_json(raw)
```

Use when:

```text
downstream consumes dicts
no model methods needed
serialization hooks unnecessary
hot path matters
```

---

## 18.20 Hot-path validator rewrite examples

### From wrap validator to after validator

```python
# before
@field_validator("x", mode="wrap")
@classmethod
def x_non_negative(cls, v, handler):
    x = handler(v)
    if x < 0:
        raise ValueError("negative")
    return x
```

```python
# after
@field_validator("x")
@classmethod
def x_non_negative(cls, x: int) -> int:
    if x < 0:
        raise ValueError("negative")
    return x
```

```python
# best, if local numeric bound is enough
x: int = Field(ge=0)
```

### From model validator to field constraints

```python
# before
@model_validator(mode="after")
def check_page_size(self):
    if not (1 <= self.page_size <= 500):
        raise ValueError("bad page_size")
    return self
```

```python
# after
page_size: int = Field(ge=1, le=500)
```

Agent rule:

```text
If validator can be expressed as Field:
  use Field

If validator needs typed value:
  after validator

If validator needs raw legacy shape:
  before validator

If validator needs handler/error retry:
  wrap validator
```

---

## 18.21 `Any` as a deliberate optimization boundary

```python
from typing import Any
from pydantic import BaseModel

class Envelope(BaseModel):
    id: str
    payload: Any
```

Meaning:

```text
payload:
  no validation
  no recursive traversal
  kept unchanged
```

Use cases:

```text
opaque JSON blob
plugin-owned payload
log metadata not inspected by this service
large nested field forwarded unchanged
partially validated envelope pattern
```

Risks:

```text
no type safety inside payload
no schema detail
no nested error reporting
downstream must validate before using
```

Official docs recommend `Any` when you do not need to validate a value, because it keeps the value unchanged. ([Pydantic Docs][S18-1])

---

## 18.22 Avoid repeated JSON/schema generation in hot paths

Bad:

```python
def handle():
    schema = Model.model_json_schema()
    ...
```

Better:

```python
MODEL_SCHEMA = Model.model_json_schema()

def handle():
    ...
```

Policy:

```text
Generate schema:
  at startup
  build time
  docs route
  cached property

Do not generate schema:
  per request
  per row
  per validation
```

Even though schema generation is not the same as validation, it traverses schema structures and should not be repeated in hot paths unless required.

---

## 18.23 Serverless/cold-start pattern

```python
class CriticalRequest(BaseModel):
    model_config = ConfigDict(extra="forbid")
    ...

class RareAdminRequest(BaseModel):
    model_config = ConfigDict(extra="forbid", defer_build=True)
    ...
```

Startup strategy:

```text
critical request models:
  build/prewarm at startup if latency matters

rare admin/CLI/plugin models:
  defer_build=True

large optional TypeAdapters:
  TypeAdapter(T, config=ConfigDict(defer_build=True))

heavy schema modules:
  lazy import by route/command/plugin when possible
```

Tradeoff:

```text
defer_build:
  lower import/cold-start cost
  first-use latency spike
```

---

## 18.24 Performance anti-patterns

### Per-call `TypeAdapter`

```python
def parse(raw):
    return TypeAdapter(list[int]).validate_python(raw)
```

Fix:

```python
LIST_INT = TypeAdapter(list[int])
```

### Raw JSON parsed twice

```python
Model.model_validate(json.loads(raw))
```

Fix:

```python
Model.model_validate_json(raw)
```

Benchmark before exception:

```text
model-level before/wrap validators may be special case
```

### Abstract container in hot path

```python
xs: Sequence[int]
meta: Mapping[str, int]
```

Fix:

```python
xs: list[int]
meta: dict[str, int]
```

### Wrap validator for simple check

```python
@field_validator("x", mode="wrap")
```

Fix:

```python
x: int = Field(ge=0)
```

### Nested model when dict output is enough

```python
class Nested(BaseModel): ...
```

Fix:

```python
class NestedTD(TypedDict): ...
```

### Full error collection when first error rejects payload

```python
TypeAdapter(list[Item])
```

Fix:

```python
TypeAdapter(Annotated[list[Item], FailFast()])
```

### Validating known-good nested blob

```python
payload: dict[str, Any]
```

If truly opaque:

```python
payload: Any
```

---

## 18.25 Test and benchmark matrix

### Adapter reuse

```python
from timeit import timeit
from pydantic import TypeAdapter

def test_adapter_reuse_benchmark():
    reused = TypeAdapter(list[int])

    new_each_time = timeit(
        lambda: TypeAdapter(list[int]).validate_python(["1", "2"]),
        number=10_000,
    )
    reused_time = timeit(
        lambda: reused.validate_python(["1", "2"]),
        number=10_000,
    )

    assert reused_time < new_each_time
```

### JSON path benchmark

```python
import json
from timeit import timeit

def test_json_validation_benchmark():
    raw = b'{"id": "1", "name": "Ada"}'
    data = json.loads(raw)

    t_json = timeit(lambda: User.model_validate_json(raw), number=10_000)
    t_py = timeit(lambda: User.model_validate(data), number=10_000)

    # Do not assert absolute ratio in CI unless environment is controlled.
    assert t_json >= 0
    assert t_py >= 0
```

### FailFast behavior

```python
from typing import Annotated
from pydantic import FailFast, TypeAdapter, ValidationError

def test_fail_fast_only_first_sequence_error():
    ta = TypeAdapter(Annotated[list[bool], FailFast()])

    with pytest.raises(ValidationError) as exc_info:
        ta.validate_python([True, "bad", False, "also bad"])

    errors = exc_info.value.errors()
    assert len(errors) == 1
    assert errors[0]["loc"] == (1,)
```

### TypedDict vs model benchmark harness

```python
from timeit import timeit
from typing_extensions import TypedDict
from pydantic import BaseModel, TypeAdapter

class A(TypedDict):
    a: str
    b: int

class TypedPayload(TypedDict):
    a: A

class B(BaseModel):
    a: str
    b: int

class ModelPayload(BaseModel):
    b: B

TYPED = TypeAdapter(TypedPayload)

payload_td = {"a": {"a": "x", "b": 2}}
payload_model = {"b": {"a": "x", "b": 2}}

typed_time = timeit(lambda: TYPED.validate_python(payload_td), number=10_000)
model_time = timeit(lambda: ModelPayload.model_validate(payload_model), number=10_000)
```

---

## 18.26 Deployment decision matrix

```text
Need / situation                              Preferred performance choice
--------------------------------------------------------------------------------
raw JSON body                                 model_validate_json
raw top-level JSON array                      TypeAdapter(list[T]).validate_json
frequent non-model validation                 module-level TypeAdapter
known list input                              list[T], not Sequence[T]
known dict input                              dict[K, V], not Mapping[K, V]
opaque value                                  Any
simple numeric/string check                   Field constraints
custom typed semantic check                   after validator
raw shape migration                           before validator
complex fallback/retry                        wrap validator only if necessary
large union/polymorphism                      discriminated union
nested dict output sufficient                 TypedDict + TypeAdapter
full object methods needed                    BaseModel
batch fail whole payload                      TypeAdapter(list[T])
batch first error enough                      TypeAdapter(Annotated[list[T], FailFast()])
batch partial success                         per-record validation loop
trusted cache hydration                       model_construct, benchmark first
serverless rare schema                        ConfigDict(defer_build=True)
critical endpoint first-use latency           prewarm model/adapters at startup
full diagnostics required                     no FailFast, collect errors
throughput more important than diagnostics    FailFast / discriminators / compact errors
```

---

## 18.27 Agent checklist

```text
[ ] Profile before optimizing.
[ ] Reuse TypeAdapter instances.
[ ] Use model_validate_json for raw JSON.
[ ] Benchmark json.loads + model_validate only for before/wrap-heavy models.
[ ] Use concrete list/dict/tuple when exact container type is known.
[ ] Use Any for deliberately opaque values.
[ ] Prefer Field constraints over validators.
[ ] Prefer after validators over wrap validators.
[ ] Avoid wrap validators in hot paths unless handler control is required.
[ ] Use discriminated unions for polymorphism.
[ ] Use TypedDict over nested models when output can remain dict-shaped.
[ ] Avoid primitive subclasses carrying extra state.
[ ] Use FailFast for sequence validation when first error is enough.
[ ] Use model_construct only for trusted/prevalidated data.
[ ] Benchmark model_construct; do not assume it is faster.
[ ] Use defer_build=True for rare/cold-start-sensitive schemas.
[ ] Prewarm critical models/adapters if first-use latency matters.
[ ] Cache JSON Schema output if used repeatedly.
[ ] Test performance with real payload sizes and valid/invalid ratios.
[ ] Separate cold-start benchmarks from steady-state benchmarks.
```

---

## 18.28 Value case

```text
Pydantic performance engineering value =
  lower schema build overhead through adapter reuse and defer_build
  lower JSON ingestion cost through model_validate_json
  lower validation branching through concrete containers and discriminated unions
  lower Python callback overhead through constraints and after validators
  lower nested object cost through TypedDict when objects are unnecessary
  lower error aggregation cost through FailFast where appropriate
  safer hot-path shortcuts through Any and model_construct when data is trusted
  explicit latency/error-detail tradeoffs for batch and API boundaries
```

Operationally: validate once at ingress, choose the validation entrypoint that matches the input encoding, reuse compiled validators/serializers, keep schema shapes concrete, reserve wrap validators and `model_construct()` for measured cases, and decide explicitly whether each path optimizes for throughput, cold start, or complete diagnostics.

[S18-1]: https://docs.pydantic.dev/latest/concepts/performance/ "Performance | Pydantic Docs"
[S18-2]: https://docs.pydantic.dev/latest/api/base_model/ "BaseModel | Pydantic Docs"
[S18-3]: https://docs.pydantic.dev/latest/api/config/ "Configuration | Pydantic Docs"


# Pydantic Advanced — 19) Advanced model features — Pydantic v2

Style target: dense, agent-oriented technical reference. 

This section covers model-level features beyond basic fields/validation: generics, `RootModel`, dynamic model creation, private attributes, class variables, abstract base classes, faux immutability, pattern matching, signatures, field order, copying, forward refs, and recursive models. Pydantic models are `BaseModel` subclasses whose fields are annotated attributes; model definition produces validation, serialization, and JSON Schema behavior from the annotation graph. ([Pydantic][S19-1])

---

## 19.0 Feature map

```text
Advanced model feature                  Primary surface
--------------------------------------------------------------------
Generic models                          class M(BaseModel, Generic[T])
Root/custom-root models                 RootModel[T]
Dynamic model classes                    create_model(...)
Private instance state                   PrivateAttr / leading underscore attrs
Class-level constants                    ClassVar[T]
Abstract model contracts                 class M(BaseModel, abc.ABC)
Faux immutability                        ConfigDict(frozen=True)
Structural pattern matching              match model: case Model(...)
Generated constructor signature          inspect.signature(Model)
Field order control                      source-order field declaration
Copying                                  model_copy(...)
Trusted no-validation construction       model_construct(...)
Forward reference rebuilding             model_rebuild(...)
Recursive models                         forward annotations / future annotations
```

---

## 19.1 Generic models

### Basic syntax

```python
from typing import Generic, TypeVar

from pydantic import BaseModel

T = TypeVar("T")


class Page(BaseModel, Generic[T]):
    items: list[T]
    total: int


class User(BaseModel):
    id: int
    name: str


page = Page[User](
    items=[{"id": "1", "name": "Ada"}],
    total="1",
)

assert isinstance(page.items[0], User)
assert page.total == 1
```

Pydantic generic models inherit from both `BaseModel` and `typing.Generic`; parametrized generic models inherit configuration, validation, serialization logic, methods, and attributes from the generic base. Pydantic creates runtime subclasses for parametrized generic models and caches them, so normal use has minimal overhead. ([Pydantic][S19-1])

---

### Generic subclass preserving generic status

```python
from typing import Generic, TypeVar

from pydantic import BaseModel

T = TypeVar("T")


class BaseBox(BaseModel, Generic[T]):
    value: T


class ChildBox(BaseBox[T], Generic[T]):
    label: str


box = ChildBox[int](value="123", label="x")
assert box.value == 123
```

If a subclass should remain generic, it must also inherit from `Generic[...]`; Pydantic supports partial or full replacement of type variables in subclasses. ([Pydantic][S19-1])

---

### Partial parametrization

```python
from typing import Generic, TypeVar

from pydantic import BaseModel

X = TypeVar("X")
Y = TypeVar("Y")
Z = TypeVar("Z")


class Pair(BaseModel, Generic[X, Y]):
    x: X
    y: Y


class IntPairPlus(Pair[int, Y], Generic[Y, Z]):
    z: Z


m = IntPairPlus[str, bool](x="1", y="hello", z=1)
assert m.x == 1
assert m.y == "hello"
assert m.z is True
```

---

### Custom parametrized model name

```python
from typing import Any, Generic, TypeVar

from pydantic import BaseModel

DataT = TypeVar("DataT")


class Response(BaseModel, Generic[DataT]):
    data: DataT

    @classmethod
    def model_parametrized_name(cls, params: tuple[type[Any], ...]) -> str:
        return f"{params[0].__name__.title()}Response"


assert repr(Response[int](data=1)).startswith("IntResponse")
```

Use `model_parametrized_name(...)` when generated model names appear in logs, error messages, schema titles, or debugging output. Pydantic documents this method for overriding concrete generic subclass names. ([Pydantic][S19-1])

---

### Nested generic relationships

```python
from typing import Generic, TypeVar

from pydantic import BaseModel

T = TypeVar("T")


class Inner(BaseModel, Generic[T]):
    inner: T


class Outer(BaseModel, Generic[T]):
    outer: T
    nested: Inner[T]


valid = Outer[int](outer=1, nested=Inner[int](inner="2"))
assert valid.nested.inner == 2
```

Using the same `TypeVar` across nested generic models enforces relationships between field types. If `Outer[int]` receives `str` data in both `outer` and `nested.inner`, both locations fail integer validation. ([Pydantic][S19-1])

---

### Avoid `isinstance(obj, GenericModel[int])`

```python
# discouraged
isinstance(page, Page[User])

# safer
class UserPage(Page[User]):
    pass

isinstance(page, UserPage)
```

Pydantic strongly advises against `isinstance()` checks against parametrized generic models; use the unparametrized origin or create a concrete subclass for runtime checks. ([Pydantic][S19-1])

---

### Unparametrized generic pitfalls

```python
from typing import Generic, TypeVar

from pydantic import BaseModel

class ItemBase(BaseModel):
    pass

class IntItem(ItemBase):
    value: int

ItemT = TypeVar("ItemT", bound=ItemBase)


class ItemHolder(BaseModel, Generic[ItemT]):
    item: ItemT


# Data loss risk: validates against ItemBase if not parametrized.
holder = ItemHolder.model_validate({"item": {"value": 1}})
assert holder.item == ItemBase()

# Correct: explicitly parametrize.
holder_int = ItemHolder[IntItem].model_validate({"item": {"value": 1}})
assert holder_int.item == IntItem(value=1)
```

When type variables are left unparametrized, Pydantic falls back to the type variable’s bound, constraints, default, or `Any`; if the bound is a base model with fewer fields, input data can be lost. Explicit parametrization prevents this class of silent narrowing. ([Pydantic][S19-1])

---

### Generic serialization pitfall

```python
from typing import Generic, TypeVar

from pydantic import BaseModel, SerializeAsAny

class ErrorDetails(BaseModel):
    foo: str

class MyErrorDetails(ErrorDetails):
    bar: str

ErrorT = TypeVar("ErrorT", default=ErrorDetails)


class Error(BaseModel, Generic[ErrorT]):
    message: str
    details: ErrorT


class AnyError(BaseModel, Generic[ErrorT]):
    message: str
    details: SerializeAsAny[ErrorT]
```

If a type variable has a default or constraints and is not parametrized, serialization may use that default/constraint schema and omit subclass fields; `SerializeAsAny` can preserve runtime-subclass fields when that behavior is intended. ([Pydantic][S19-1])

---

### Generic model deployment rules

```text
Use generic models for:
  paginated responses
  result envelopes
  error envelopes
  resource wrappers
  typed cache entries
  command/result protocols

Always:
  parametrize public models explicitly
  avoid isinstance(obj, Generic[T])
  test validation + serialization for unparametrized/default/bound TypeVar behavior
  use SerializeAsAny only when subclass fields are safe and intended to serialize
```

---

## 19.2 `RootModel`

### Basic syntax

```python
from pydantic import RootModel

Pets = RootModel[list[str]]
PetsByName = RootModel[dict[str, str]]

pets = Pets(["dog", "cat"])
assert pets.root == ["dog", "cat"]
assert pets.model_dump_json() == '["dog","cat"]'
assert Pets.model_validate(["dog", "cat"]).root == ["dog", "cat"]
```

`RootModel[T]` defines a Pydantic model whose payload is a single root value rather than an object with named fields; the root type can be any Pydantic-supported type, and the root value can be passed to `__init__` or `model_validate(...)` as the first and only argument. ([Pydantic][S19-1])

---

### Root model with behavior

```python
from pydantic import RootModel


class Pets(RootModel[list[str]]):
    def __iter__(self):
        return iter(self.root)

    def __getitem__(self, item):
        return self.root[item]

    def describe(self) -> str:
        return f"Pets: {', '.join(self.root)}"


pets = Pets.model_validate(["dog", "cat"])
assert pets[0] == "dog"
assert list(pets) == ["dog", "cat"]
assert pets.describe() == "Pets: dog, cat"
```

If direct item access or iteration is desired, implement `__iter__` and `__getitem__`; Pydantic also supports subclassing a parametrized root model directly to add domain behavior. ([Pydantic][S19-1])

---

### `RootModel` vs `TypeAdapter`

```python
from pydantic import RootModel, TypeAdapter

TagsModel = RootModel[list[str]]
TagsAdapter = TypeAdapter(list[str])
```

```text
Use RootModel when:
  top-level type needs a named class
  JSON Schema title/class identity matters
  methods/properties should attach to wrapper
  domain object should carry behavior
  public API wants importable named schema

Use TypeAdapter when:
  validation/serialization/schema for arbitrary type is enough
  no wrapper behavior needed
  no model identity needed
  hot-path top-level array validation
```

---

### Root model notes

```text
RootModel[list[T]].model_dump()
  returns the root payload shape, not {"root": ...}

RootModel[list[T]].model_dump_json()
  serializes to JSON array

RootModel[dict[str, T]].model_dump_json()
  serializes to JSON object

RootModel.model_construct(root)
  accepts root positionally
```

The `RootModel` API states that `root` is the root object, that `__pydantic_root_model__` marks the class as a root model, and that `RootModel.model_construct(root, _fields_set=None)` creates a new instance from the root object. ([Pydantic][S19-2])

---

## 19.3 Dynamic model creation: `create_model`

### Basic syntax

```python
from pydantic import BaseModel, create_model

DynamicFoobarModel = create_model(
    "DynamicFoobarModel",
    foo=str,
    bar=(int, 123),
)

# Equivalent to:
class StaticFoobarModel(BaseModel):
    foo: str
    bar: int = 123
```

Field definitions in `create_model(...)` are keyword arguments. A field definition can be a single type annotation or a two-tuple of `(type, default_or_Field)`. Pydantic changed single-element field definitions in v2.11 so any type can be used. ([Pydantic][S19-1])

---

### Field metadata and private attributes

```python
from typing import Annotated

from pydantic import Field, PrivateAttr, create_model

DynamicModel = create_model(
    "DynamicModel",
    foo=(str, Field(alias="FOO")),
    bar=Annotated[str, Field(description="Bar field")],
    _private=(int, PrivateAttr(default=1)),
)
```

`create_model(...)` supports `Field(...)`, `Annotated[...]` field metadata, and private attributes via `PrivateAttr(...)`. ([Pydantic][S19-1])

---

### Extending a base model

```python
from pydantic import BaseModel, create_model

class FooModel(BaseModel):
    foo: str
    bar: int = 123


BarModel = create_model(
    "BarModel",
    apple=(str, "russet"),
    banana=(str, "yellow"),
    __base__=FooModel,
)

assert list(BarModel.model_fields) == ["foo", "bar", "apple", "banana"]
```

Special keyword arguments such as `__base__` and `__config__` customize the dynamically created class; `__base__` extends a base model with additional fields. ([Pydantic][S19-1])

---

### Dynamic validators

```python
from pydantic import ValidationError, create_model, field_validator

def alphanum(cls, v: str) -> str:
    assert v.isalnum(), "must be alphanumeric"
    return v

validators = {
    "username_validator": field_validator("username")(alphanum),
}

UserModel = create_model(
    "UserModel",
    username=(str, ...),
    __validators__=validators,
)

UserModel(username="scolvin")

try:
    UserModel(username="scolvi%n")
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "assertion_error"
```

Validator names passed through `__validators__` must not clash with field names because Pydantic gathers members into a namespace much like normal class creation. ([Pydantic][S19-1])

---

### Pickling dynamic models

```python
UserModel = create_model(
    "UserModel",
    username=(str, ...),
    __module__=__name__,
)
```

To pickle a dynamically created model, define it globally and pass `__module__`. Pydantic also warns that `create_model(...)` may execute arbitrary code contained in field annotations when string references need evaluation, so do not create models from untrusted annotations. ([Pydantic][S19-1])

---

### Dynamic model deployment rules

```text
Use create_model for:
  generated schemas
  optional/partial variants
  plugin-defined DTOs
  database/reflection-driven models
  test models
  schema migration adapters

Avoid create_model for:
  ordinary static domain models
  untrusted field annotations
  models that should be easy for IDE/type checkers to understand
  public SDK models where static class definitions improve readability
```

---

## 19.4 Private attributes: `PrivateAttr`

### Basic private state

```python
from datetime import datetime
from random import randint
from typing import Any

from pydantic import BaseModel, PrivateAttr

class TimeAwareModel(BaseModel):
    _processed_at: datetime = PrivateAttr(default_factory=datetime.now)
    _secret_value: int

    def model_post_init(self, context: Any) -> None:
        self._secret_value = randint(1, 5)


m = TimeAwareModel()
assert isinstance(m._processed_at, datetime)
assert 1 <= m._secret_value <= 5
```

Attributes with leading underscores are not treated as model fields, are not included in model schema, and are converted into private attributes that are not validated or set during `__init__`, `model_validate`, etc.; dunder names such as `__attr__` are not supported and are ignored. ([Pydantic][S19-1])

---

### Private attr use cases

```text
Use PrivateAttr for:
  caches
  timestamps
  memoized expensive resources
  non-serialized runtime state
  internal random/session values
  computed indexes
  locks / handles when not part of schema

Do not use PrivateAttr for:
  user input
  public data
  serialized API fields
  values needing validation
  security boundaries
```

### Private attr and `model_construct`

Private attributes are populated under `model_construct()` similarly to validated construction; no validation still occurs for normal fields. The BaseModel docs note `model_construct()` populates `__pydantic_private__` like validated construction. ([Pydantic][S19-1])

---

## 19.5 Class variables: `ClassVar`

```python
from typing import ClassVar

from pydantic import BaseModel

class Model(BaseModel):
    x: ClassVar[int] = 1
    y: int = 2


m = Model()
assert m.model_dump() == {"y": 2}
assert Model.x == 1
assert "x" not in Model.model_fields
```

Attributes annotated with `ClassVar` are treated as class variables and do not become Pydantic fields on model instances. ([Pydantic][S19-1])

Agent rule:

```text
Use ClassVar for:
  constants
  registries
  class-level metadata
  version strings
  discriminator maps
  static lookup tables

Do not use ClassVar for:
  instance input fields
  values expected in model_dump()
  values expected in JSON Schema
```

---

## 19.6 Abstract base classes

```python
import abc

from pydantic import BaseModel

class RepositoryConfig(BaseModel, abc.ABC):
    name: str

    @abc.abstractmethod
    def connect_uri(self) -> str:
        ...
```

Pydantic models can be combined with Python abstract base classes by inheriting from `BaseModel` and `abc.ABC`. ([Pydantic][S19-1])

### Pattern

```python
class StorageConfig(BaseModel, abc.ABC):
    bucket: str

    @abc.abstractmethod
    def kind(self) -> str:
        ...


class S3Config(StorageConfig):
    region: str

    def kind(self) -> str:
        return "s3"
```

Use cases:

```text
validated configuration contracts
plugin interfaces
model families with required behavior
domain value objects with abstract methods
```

Agent rule:

```text
ABC enforces Python method implementation.
Pydantic enforces field validation.
Do not rely on ABCs for payload polymorphism; use discriminated unions for wire data.
```

---

## 19.7 Faux immutability

### Model-level frozen

```python
from pydantic import BaseModel, ConfigDict, ValidationError

class FooBarModel(BaseModel):
    model_config = ConfigDict(frozen=True)

    a: str
    b: dict


foobar = FooBarModel(a="hello", b={"apple": "pear"})

try:
    foobar.a = "different"
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "frozen_instance"

foobar.b["apple"] = "grape"
assert foobar.b == {"apple": "grape"}
```

`ConfigDict(frozen=True)` blocks rebinding instance attributes, replacing v1’s deprecated `allow_mutation=False`; it is “faux” immutability because nested mutable objects such as dictionaries can still be mutated. ([Pydantic][S19-1])

### Policy

```text
Use frozen=True for:
  value objects
  cache keys if all fields hashable
  stable validated command objects
  identifiers / normalized domain records

Do not assume:
  deep immutability
  thread-safety
  authorization protection
  mutation-proof nested dict/list state
```

### Deep immutability pattern

```python
from typing import Mapping
from types import MappingProxyType

from pydantic import BaseModel, ConfigDict, field_validator

class FrozenDeepish(BaseModel):
    model_config = ConfigDict(frozen=True)

    data: tuple[tuple[str, int], ...]

    @field_validator("data", mode="before")
    @classmethod
    def dict_to_tuple(cls, v):
        if isinstance(v, dict):
            return tuple(sorted(v.items()))
        return v
```

For true deep immutability, prefer immutable field types such as `tuple`, `frozenset`, immutable dataclasses, or custom conversion.

---

## 19.8 Structural pattern matching

```python
from pydantic import BaseModel

class Pet(BaseModel):
    name: str
    species: str


a = Pet(name="Bones", species="dog")

match a:
    case Pet(species="dog", name=dog_name):
        result = f"{dog_name} is a dog"
    case _:
        result = "No dog matched"

assert result == "Bones is a dog"
```

Pydantic supports Python 3.10 structural pattern matching for models; a `match` statement does not create a new model, it is syntactic sugar for attribute access, comparison, and binding. ([Pydantic][S19-1])

### Use cases

```text
domain branching after validation
command dispatch on validated object shape
test assertions
small decision trees
```

### Avoid

```text
complex API polymorphism:
  use discriminated unions during validation

business workflow engine:
  use explicit dispatch table or methods

matching on unvalidated raw data:
  validate first
```

---

## 19.9 Model signatures

```python
import inspect

from pydantic import BaseModel, Field

class FooModel(BaseModel):
    id: int
    name: str = None
    description: str = "Foo"
    apple: int = Field(alias="pear")


sig = inspect.signature(FooModel)
assert "pear" in str(sig)
```

Pydantic synthesizes model `__init__` signatures from fields; aliases are prioritized over field names when valid Python identifiers, and signatures respect custom `__init__` functions. Accurate signatures support introspection-heavy libraries such as FastAPI and Hypothesis. ([Pydantic][S19-1])

### Signature rules

```text
field alias valid Python identifier:
  used in signature before field name

alias invalid but field name valid:
  field name may be used

alias and field name invalid:
  **data added

extra="allow":
  **data always present

custom __init__:
  signature respects custom __init__
```

Pydantic documents these exact signature-generation rules, including `**data` behavior for invalid aliases/names and `extra="allow"`. ([Pydantic][S19-1])

### Agent rule

```text
For public model constructors:
  choose valid identifier aliases when constructor ergonomics matter

For external JSON aliases with invalid identifiers:
  use model_validate(...), not positional/constructor convenience

Avoid custom __init__ unless necessary:
  prefer validators/model_post_init
```

---

## 19.10 Field ordering

```python
from pydantic import BaseModel, ValidationError

class Model(BaseModel):
    a: int
    b: int = 2
    c: int = 1
    d: int = 0
    e: float


assert list(Model.model_fields) == ["a", "b", "c", "d", "e"]
assert Model(e=2, a=1).model_dump() == {"a": 1, "b": 2, "c": 1, "d": 0, "e": 2.0}
```

Field declaration order is preserved in model JSON Schema, validation error ordering, and serialization output. ([Pydantic][S19-1])

### Order-sensitive features

```text
field order affects:
  model_fields order
  model_dump order
  ValidationError error order
  JSON Schema property order
  default_factory(data) availability
  info.data availability in field validators
```

### Policy

```text
Place dependency-providing fields before dependent fields.
Place stable public fields in desired serialization/schema order.
Avoid relying on alphabetical field order.
Test order when schemas or dumps are public artifacts.
```

---

## 19.11 Copying models: `model_copy(...)`

### Syntax

```python
new_model = model.model_copy()
patched = model.model_copy(update={"name": "Grace"})
deep_copy = model.model_copy(deep=True)
```

`model_copy(update=None, deep=False)` returns a copy of the model. The underlying `__dict__` is copied, which can matter if cached properties or other non-field values are stored there; `update` data is **not validated** and must be trusted. ([Pydantic Docs][S19-3])

### Safe vs unsafe update

```python
# trusted internal patch only
patched = user.model_copy(update={"name": "Grace"})

# untrusted patch: revalidate
data = user.model_dump()
data.update(untrusted_patch)
patched = type(user).model_validate(data)
```

### Shallow vs deep

```python
class M(BaseModel):
    xs: list[int]

m1 = M(xs=[1, 2])
m2 = m1.model_copy()
m3 = m1.model_copy(deep=True)

m2.xs.append(3)
assert m1.xs == [1, 2, 3]

m3.xs.append(4)
assert m1.xs == [1, 2, 3]
```

### Policy

```text
Use model_copy(update=...) for:
  trusted internal transformations
  persistent immutable-ish workflows
  functional-style updates

Use revalidation for:
  API patches
  user input
  LLM output
  file/config/message updates

Use deep=True when:
  nested mutable values must not alias
```

---

## 19.12 Creating models without validation: `model_construct(...)`

```python
user = User.model_construct(id=1, name="Ada")
```

`model_construct(...)` creates a model from trusted or prevalidated data, respects defaults, sets `__dict__` and `__pydantic_fields_set__`, but performs no validation; for `extra="forbid"`, extra values are ignored rather than rejected because validation is not run. ([Pydantic Docs][S19-3])

Use cases:

```text
trusted cache hydration
deserialization from your own validated storage
test fixtures requiring invalid internal state
framework internals
non-idempotent validators should not rerun
```

Avoid:

```text
untrusted API data
settings/env data
queue messages
LLM outputs
user files
database rows from untrusted/mixed sources
```

Performance caveat:

```text
Do not assume model_construct is faster.
Pydantic v2 narrowed validation-vs-construct performance gap.
Benchmark before replacing normal validation.
```

Pydantic warns that `model_construct()` can create invalid models and should only be used with already validated or trusted data; it also notes validation may be faster for simple models in v2. ([Pydantic][S19-1])

---

## 19.13 Rebuilding models and forward refs: `model_rebuild(...)`

### Basic forward reference

```python
from pydantic import BaseModel, PydanticUserError

class Foo(BaseModel):
    x: "Bar"

try:
    Foo.model_json_schema()
except PydanticUserError as exc:
    assert exc.code == "class-not-fully-defined"

class Bar(BaseModel):
    pass

Foo.model_rebuild()
```

`model_rebuild()` replaces v1 `update_forward_refs()`. It is used when annotations reference symbols not yet defined at class creation; calling it on the outermost model builds the core schema for the whole nested graph, so all types at all nested levels must be ready before it is called. ([Pydantic][S19-1])

### Signature

```python
@classmethod
def model_rebuild(
    cls,
    *,
    force: bool = False,
    raise_errors: bool = True,
    _parent_namespace_depth: int = 2,
    _types_namespace: MappingNamespace | None = None,
) -> bool | None:
    ...
```

Return behavior:

```text
None:
  schema already complete; rebuild not required

True:
  rebuild required and succeeded

False:
  rebuild required and failed with raise_errors=False
```

The BaseModel API lists `model_rebuild()` as the method for rebuilding a model schema when forward references could not be resolved automatically. ([Pydantic Docs][S19-3])

### Explicit namespace pattern

```python
class Foo(BaseModel):
    x: "Bar"

class Bar(BaseModel):
    y: int

Foo.model_rebuild(_types_namespace={"Bar": Bar})
```

### Deployment rules

```text
Use model_rebuild when:
  forward refs remain unresolved
  dynamic imports define referenced models late
  recursive generic model schemas fail to build
  plugin schemas are assembled after import

Call on:
  outermost model

Before calling:
  import/define all nested referenced types

Avoid:
  repeated rebuild calls in hot paths
  calling rebuild before every referenced model is imported
```

---

## 19.14 Forward annotations and recursive models

### Self-referencing model

```python
from __future__ import annotations

from pydantic import BaseModel

class Node(BaseModel):
    id: int
    children: list[Node] = []


node = Node.model_validate({"id": "1", "children": [{"id": "2"}]})
assert node.children[0].id == 2
```

Forward annotations can be explicit strings or enabled with `from __future__ import annotations`; Pydantic supports self-referencing models and resolves such annotations during model creation when possible. ([Pydantic Docs][S19-4])

---

### Mutability-safe recursive default

```python
from __future__ import annotations

from pydantic import BaseModel, Field

class Node(BaseModel):
    id: int
    children: list[Node] = Field(default_factory=list)
```

Prefer `default_factory=list` over `children: list[Node] = []` for clarity and dataclass-style habit, even though Pydantic handles many mutable-default cases safely.

---

### Cross-referencing models

```python
from __future__ import annotations

from pydantic import BaseModel

class ModelA(BaseModel):
    b: ModelB | None = None

class ModelB(BaseModel):
    a: ModelA | None = None

ModelA.model_rebuild()
ModelB.model_rebuild()
```

In multi-module cross-referencing graphs, avoid rebuild until the full graph is imported/defined. Prefer a central module that imports all model classes and rebuilds public outermost schemas once.

---

## 19.15 Cyclic input references

```python
from __future__ import annotations

from pydantic import BaseModel, ValidationError

class ModelA(BaseModel):
    b: ModelB | None = None

class ModelB(BaseModel):
    a: ModelA | None = None


cyclic_data = {}
cyclic_data["a"] = {"b": cyclic_data}

try:
    ModelB.model_validate(cyclic_data)
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "recursion_loop"
```

When validating recursive models, cyclic input data can produce a `recursion_loop` `ValidationError` instead of a Python `RecursionError`, allowing normal error handling without exceeding maximum recursion depth. Serialization cycles are also detected early and can raise a `ValueError` for circular references. ([Pydantic Docs][S19-4])

### Cycle handling policy

```text
Tree data:
  recursive model fields are fine

Graph data with cycles:
  avoid full object recursion in schema
  use reference IDs
  use NodeReference pattern
  use wrap validators/serializers only if deliberate cycle pruning is needed

ORM backrefs:
  prefer DTO projections
  avoid validating raw cyclic ORM graph directly
```

---

## 19.16 Recursive serialization cycle handling

```python
from __future__ import annotations

from pydantic import BaseModel, field_serializer

class Node(BaseModel):
    id: int
    children: list[Node] = []

    @field_serializer("children", mode="wrap")
    def serialize_children(self, children, handler):
        try:
            return handler(children)
        except ValueError as exc:
            if not str(exc).startswith("Circular reference"):
                raise
            return [{"id": child.id} for child in children]
```

Pydantic documents both validation-time cycle detection and serialization-time circular-reference detection; wrap serializers can be used to degrade cyclic child objects into reference summaries when that is the intended output contract. ([Pydantic Docs][S19-4])

---

## 19.17 Attribute copies

```python
from pydantic import BaseModel

class C2(BaseModel):
    arr: list[int]


arr_orig = [1, 9, 10, 3]
c2 = C2(arr=arr_orig)

assert id(c2.arr) != id(arr_orig)
```

Pydantic often copies constructor arguments during validation/coercion, so model values may not alias the original input. Pydantic does not always copy, especially when passing model instances; set `revalidate_instances='always'` if nested model instances must be revalidated/copied through validation. ([Pydantic][S19-1])

Agent rule:

```text
Do not rely on input object identity after validation.
Do not mutate original input expecting model to update.
Do not mutate model expecting original input to update.
For nested models, understand revalidate_instances.
```

---

## 19.18 Advanced feature decision matrix

```text
Need                                         Recommended feature
--------------------------------------------------------------------------------
typed response/data envelope                  Generic BaseModel[T]
top-level array/object as named contract       RootModel[list[T]] / RootModel[dict[str, T]]
top-level type without class behavior          TypeAdapter(T)
runtime-generated DTO                          create_model(...)
derived optional/partial model                 create_model + existing model fields
private runtime cache/state                    PrivateAttr
class-level constants/registry                 ClassVar
abstract behavior contract                     BaseModel + abc.ABC
model-level rebinding prohibition              ConfigDict(frozen=True)
deep immutability                              immutable field types + validators
Python branching on validated model            structural pattern matching
framework introspection                        generated model signature
public ordering in schema/dump/errors          declaration field order
trusted internal update                        model_copy(update=...)
untrusted update                               model_validate(merged_data)
trusted no-validation hydration                model_construct(...)
forward refs unresolved                        model_rebuild(...)
self-referencing tree                          forward annotations + recursive fields
cyclic graph                                   reference IDs or explicit cycle handling
```

---

## 19.19 Complete advanced model example

```python
from __future__ import annotations

import abc
from typing import Any, ClassVar, Generic, TypeVar
from uuid import UUID

from pydantic import BaseModel, ConfigDict, Field, PrivateAttr, RootModel, create_model

T = TypeVar("T")


class ApiModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        validate_assignment=True,
    )


class Resource(ApiModel, Generic[T]):
    api_version: ClassVar[str] = "v1"

    id: UUID
    payload: T
    _loaded_from_cache: bool = PrivateAttr(default=False)

    def mark_cached(self) -> None:
        self._loaded_from_cache = True


class TreeNode(ApiModel):
    id: int
    children: list[TreeNode] = Field(default_factory=list)


class ResourceList(RootModel[list[Resource[Any]]]):
    def __iter__(self):
        return iter(self.root)

    def __getitem__(self, index: int):
        return self.root[index]


class AbstractProcessor(ApiModel, abc.ABC):
    name: str

    @abc.abstractmethod
    def process(self, node: TreeNode) -> TreeNode:
        ...


PartialNode = create_model(
    "PartialNode",
    id=(int | None, None),
    children=(list[TreeNode] | None, None),
    __base__=ApiModel,
    __module__=__name__,
)

TreeNode.model_rebuild()
```

---

## 19.20 Anti-patterns

### Parametrized generic `isinstance`

```python
isinstance(obj, Page[User])  # discouraged
```

Use:

```python
class UserPage(Page[User]):
    pass

isinstance(obj, UserPage)
```

---

### Unparametrized generic public API

```python
def handle(page: Page):
    ...
```

Prefer:

```python
def handle(page: Page[User]):
    ...
```

Unparametrized generics can validate against bounds/defaults/`Any` and may cause data loss or serialization surprises. ([Pydantic][S19-1])

---

### `RootModel` when `TypeAdapter` is enough

```python
Ids = RootModel[list[int]]
```

If no class identity/method/schema title is needed:

```python
IDS = TypeAdapter(list[int])
```

---

### `create_model` from untrusted annotations

```python
create_model("M", x=("SomeUntrustedAnnotation", ...))
```

Pydantic warns that `create_model(...)` may execute arbitrary code in field annotations when string references must be evaluated. ([Pydantic][S19-1])

---

### PrivateAttr for validated data

```python
class Bad(BaseModel):
    _token: str = PrivateAttr()
```

If `_token` comes from user/config/API input and needs validation, make it a field:

```python
class Good(BaseModel):
    token: str
```

---

### Frozen model with mutable field assumed deep-immutable

```python
class M(BaseModel):
    model_config = ConfigDict(frozen=True)
    data: dict
```

Use immutable structures:

```python
class M(BaseModel):
    model_config = ConfigDict(frozen=True)
    data: tuple[tuple[str, int], ...]
```

---

### `model_copy(update=...)` for untrusted data

```python
new = old.model_copy(update=request_json)
```

Use:

```python
new = type(old).model_validate({**old.model_dump(), **request_json})
```

`model_copy(update=...)` does not validate update data. ([Pydantic Docs][S19-3])

---

## 19.21 Test matrix

### Generic model

```python
def test_generic_page():
    T = TypeVar("T")

    class Page(BaseModel, Generic[T]):
        items: list[T]

    class User(BaseModel):
        id: int

    page = Page[User].model_validate({"items": [{"id": "1"}]})
    assert page.items == [User(id=1)]
```

### Unparametrized generic data-loss guard

```python
def test_parametrize_generic_to_avoid_data_loss():
    ItemT = TypeVar("ItemT", bound=BaseModel)

    class ItemBase(BaseModel):
        pass

    class IntItem(ItemBase):
        value: int

    class Holder(BaseModel, Generic[ItemT]):
        item: ItemT

    assert Holder[IntItem].model_validate({"item": {"value": 1}}).item == IntItem(value=1)
```

### RootModel

```python
def test_root_model_list():
    Pets = RootModel[list[str]]
    pets = Pets.model_validate(["dog", "cat"])

    assert pets.root == ["dog", "cat"]
    assert pets.model_dump_json() == '["dog","cat"]'
```

### Dynamic model

```python
def test_create_model():
    Dynamic = create_model("Dynamic", x=str, y=(int, 1))

    assert Dynamic(x=123).model_dump() == {"x": "123", "y": 1}
```

### Private attributes

```python
def test_private_attr_not_dumped():
    class M(BaseModel):
        x: int
        _cache: dict = PrivateAttr(default_factory=dict)

    m = M(x=1)
    m._cache["k"] = "v"

    assert m.model_dump() == {"x": 1}
    assert "_cache" not in M.model_fields
```

### ClassVar

```python
def test_class_var_not_field():
    class M(BaseModel):
        kind: ClassVar[str] = "m"
        x: int

    assert M.kind == "m"
    assert "kind" not in M.model_fields
```

### Frozen shallow immutability

```python
def test_frozen_is_shallow():
    class M(BaseModel):
        model_config = ConfigDict(frozen=True)
        x: int
        data: dict

    m = M(x=1, data={"a": 1})

    with pytest.raises(ValidationError):
        m.x = 2

    m.data["a"] = 2
    assert m.data == {"a": 2}
```

### Pattern matching

```python
def test_structural_pattern_matching():
    class Pet(BaseModel):
        name: str
        species: str

    pet = Pet(name="Bones", species="dog")

    match pet:
        case Pet(species="dog", name=name):
            assert name == "Bones"
        case _:
            raise AssertionError("expected dog")
```

### Signature

```python
def test_signature_uses_alias():
    class M(BaseModel):
        x: int = Field(alias="external_x")

    sig = str(inspect.signature(M))
    assert "external_x" in sig
```

### Field order

```python
def test_field_order():
    class M(BaseModel):
        a: int
        b: int = 2
        c: float

    assert list(M.model_fields) == ["a", "b", "c"]
    assert list(M(a=1, c=2).model_dump()) == ["a", "b", "c"]
```

### Copy safety

```python
def test_model_copy_update_not_validated():
    class M(BaseModel):
        x: int

    m = M(x=1)
    copied = m.model_copy(update={"x": "bad"})

    assert copied.x == "bad"

    with pytest.raises(ValidationError):
        M.model_validate({**m.model_dump(), "x": "bad"})
```

### Forward refs

```python
def test_model_rebuild_forward_ref():
    class A(BaseModel):
        b: "B"

    class B(BaseModel):
        x: int

    assert A.model_rebuild() in {None, True}
    assert A.model_validate({"b": {"x": "1"}}).b.x == 1
```

### Recursive cycle detection

```python
def test_recursive_cycle_detection():
    class A(BaseModel):
        b: "B | None" = None

    class B(BaseModel):
        a: A | None = None

    A.model_rebuild()
    B.model_rebuild()

    cyclic = {}
    cyclic["a"] = {"b": cyclic}

    with pytest.raises(ValidationError) as exc:
        B.model_validate(cyclic)

    assert exc.value.errors()[0]["type"] == "recursion_loop"
```

---

## 19.22 Deployment checklist

```text
[ ] Parametrize generic models explicitly in public APIs.
[ ] Do not use parametrized generic models in isinstance checks.
[ ] Test generic validation and serialization separately.
[ ] Use RootModel only when a named root wrapper is valuable.
[ ] Use TypeAdapter for unnamed top-level types.
[ ] Use create_model only for runtime-generated schemas; static classes remain preferred.
[ ] Pass __module__ for picklable dynamic models.
[ ] Never build dynamic models from untrusted annotations.
[ ] Use PrivateAttr only for unvalidated private runtime state.
[ ] Use ClassVar for class constants and registries.
[ ] Combine BaseModel with abc.ABC for abstract behavior contracts.
[ ] Treat frozen=True as shallow faux immutability.
[ ] Use immutable nested types for deep immutability.
[ ] Use structural pattern matching after validation, not before.
[ ] Verify generated signatures if framework introspection matters.
[ ] Order fields intentionally for schema/dump/error order and field dependency.
[ ] Use model_copy(update=...) only with trusted updates.
[ ] Use model_construct only with trusted/prevalidated data.
[ ] Call model_rebuild after all forward-referenced types are imported/defined.
[ ] Rebuild outermost models for nested forward-ref graphs.
[ ] Use forward annotations for recursive models.
[ ] Avoid raw cyclic ORM/object graphs; prefer reference DTOs or explicit cycle handling.
[ ] Test recursion_loop behavior when recursive inputs are possible.
```

---

## 19.23 Value case

```text
Advanced model feature value =
  generic reusable model families
  named top-level root schemas
  runtime model generation when field sets are dynamic
  private runtime state without schema pollution
  class constants that do not become fields
  abstract validated interfaces
  shallow immutable/value-object patterns
  Python-native match/case ergonomics
  introspection-friendly signatures for frameworks
  deterministic schema/error/dump ordering
  explicit copy and trusted-construction workflows
  robust forward-reference and recursive model support
```

Operationally: use advanced model features to encode **schema identity**, **runtime ergonomics**, and **deployment-specific behavior** deliberately. Keep generics explicitly parametrized, use `RootModel` only when named root behavior matters, keep private attributes out of validated data paths, treat frozen models as shallow, validate untrusted updates instead of copying them, and rebuild forward-reference graphs only after every referenced type is available.

[S19-1]: https://pydantic.dev/docs/validation/latest/concepts/models/ "Models | Pydantic Docs"
[S19-2]: https://pydantic.dev/docs/validation/latest/api/pydantic/root_model/?utm_source=chatgpt.com "RootModel | Pydantic Docs"
[S19-3]: https://docs.pydantic.dev/latest/api/base_model/ "BaseModel | Pydantic Docs"
[S19-4]: https://docs.pydantic.dev/latest/concepts/forward_annotations/ "Forward Annotations | Pydantic Docs"


# Pydantic Advanced — 20) Unions, polymorphism, and discriminators — Pydantic v2

Style target: dense, agent-oriented technical reference. 

Unions are fundamentally different from ordinary annotations: instead of requiring every field/item/value to validate, a union requires **one member** to validate. Pydantic supports three main union strategies: **left-to-right mode**, **smart mode**, and **discriminated/tagged unions**; Pydantic recommends discriminated unions in general because they are more predictable and performant than untagged unions. ([Pydantic Docs][S20-1])

---

## 20.0 Union mental model

```text
input value
  → union dispatcher
  → candidate branch selection
  → branch validation
  → selected branch output OR aggregated branch errors
```

Union strategies:

```text
plain union, smart mode:
  default for most unions
  evaluate candidates and choose "best" match

plain union, left_to_right mode:
  try candidates in declaration order
  first success wins

discriminated union:
  read tag/discriminator
  validate only selected branch
  most predictable
  best error locality
  best polymorphic API shape
```

Syntax variants:

```python
# Python 3.10+
value: int | str

# Older / explicit typing syntax
from typing import Union
value: Union[int, str]
```

---

## 20.1 Plain unions: `A | B` / `Union[A, B]`

```python
from uuid import UUID
from pydantic import BaseModel

class User(BaseModel):
    id: int | str | UUID
    name: str
```

Use plain unions when:

```text
small number of scalar alternatives
legacy payload accepts multiple simple shapes
closed model branches do not share a stable discriminator
ambiguity is acceptable and tested
```

Avoid plain untagged model unions when:

```text
branch models overlap heavily
public API contract must be deterministic
OpenAPI output should be clean
error messages should be concise
performance matters
LLM repair prompts should be small
```

Pydantic notes that untagged union validation involves deciding which members to validate and which errors to raise; discriminated unions validate only one member based on a discriminator, which is more efficient and avoids a proliferation of errors. ([Pydantic Docs][S20-1])

---

## 20.2 Union validation modes

```text
left_to_right:
  each member tried in order
  first successful validation returned
  all branch errors returned if none succeed

smart:
  default in Pydantic >=2 for most unions
  tries to select best match
  algorithm can change between minor releases

discriminated:
  discriminator chooses branch
  only selected branch validated
```

Pydantic documents exactly these three approaches. Left-to-right mode tries members in order; smart mode is default and attempts a better match; discriminated unions choose one branch from a discriminator. ([Pydantic Docs][S20-1])

---

## 20.3 `union_mode="left_to_right"`

### Syntax

```python
from typing import Union
from pydantic import BaseModel, Field, ValidationError

class User(BaseModel):
    id: Union[str, int] = Field(union_mode="left_to_right")

assert User(id=123).id == 123
assert User(id="hello").id == "hello"
```

### Order-sensitive coercion

```python
class User(BaseModel):
    id: Union[int, str] = Field(union_mode="left_to_right")

u = User(id="456")
assert isinstance(u.id, int)
assert u.id == 456
```

Because lax `int` validation can parse a numeric string, `Union[int, str]` with left-to-right mode returns `int` for `"456"` if `int` is first. Pydantic explicitly calls this a surprising result and explains why left-to-right is not the default. ([Pydantic Docs][S20-1])

### Use cases

```text
legacy compatibility where branch priority is part of contract
migration path where old format must win
scalar unions with explicit order semantics
specialized non-discriminated behavior with tests
```

### Rules

```text
If branch order matters:
  use union_mode="left_to_right"

If branch order must not matter:
  avoid left_to_right

If model polymorphism is public:
  prefer discriminated unions
```

---

## 20.4 Smart union mode

```python
from uuid import UUID
from pydantic import BaseModel

class User(BaseModel):
    id: int | str | UUID
    name: str
```

Smart mode scoring uses:

```text
model / dataclass / TypedDict unions:
  number of valid fields set
  exactness as tiebreaker

other unions:
  exact type match
  strict-mode success
  lax-mode success
```

Pydantic documents that smart mode attempts to select the best match, that its algorithm may change between minor releases, and that if exact matching behavior is required, use `union_mode="left_to_right"` or discriminated unions. It also documents the scoring concepts of valid-field count and exactness. ([Pydantic Docs][S20-1])

### Agent rules

```text
Use smart mode for:
  ergonomic scalar unions
  internal non-public convenience
  low-ambiguity unions

Do not rely on:
  exact smart-mode selection for ambiguous inputs
  stable tie-breaking across minor versions

Pin with tests if:
  ambiguous input shape can match multiple members
```

---

## 20.5 Plain union error reporting

```python
from typing import Union
from pydantic import BaseModel, Field, ValidationError

class User(BaseModel):
    id: Union[str, int] = Field(union_mode="left_to_right")

try:
    User(id=[])
except ValidationError as exc:
    print(exc.errors())
```

Typical location shape:

```text
("id", "str")
("id", "int")
```

Semantics:

```text
all branches fail:
  include errors from all union members

branch labels:
  appended to loc

large recursive/plain model union:
  error output can become noisy
```

Pydantic shows failed left-to-right union validation producing branch-labeled errors such as `id.str` and `id.int`. ([Pydantic Docs][S20-1])

---

## 20.6 Discriminated unions: field discriminator

### Basic syntax

```python
from typing import Literal
from pydantic import BaseModel, Field

class Cat(BaseModel):
    pet_type: Literal["cat"]
    meows: int

class Dog(BaseModel):
    pet_type: Literal["dog"]
    barks: float

class Lizard(BaseModel):
    pet_type: Literal["reptile", "lizard"]
    scales: bool

class Model(BaseModel):
    pet: Cat | Dog | Lizard = Field(discriminator="pet_type")
    n: int

m = Model.model_validate({"pet": {"pet_type": "dog", "barks": 3.14}, "n": 1})
assert isinstance(m.pet, Dog)
```

Requirements:

```text
each branch model:
  has common discriminator field

discriminator field:
  typed as Literal[...] or compatible literal root model support

union field:
  Field(discriminator="field_name")
```

Pydantic documents string discriminators using a common field such as `pet_type`, where every variant defines that field as one or more `Literal` values, and the union field specifies `Field(discriminator="pet_type")`. ([Pydantic Docs][S20-1])

---

## 20.7 Literal-based discriminators

```python
from typing import Literal
from pydantic import BaseModel, Field

class CreateUser(BaseModel):
    kind: Literal["user.create"]
    email: str

class DeleteUser(BaseModel):
    kind: Literal["user.delete"]
    user_id: int

class Event(BaseModel):
    payload: CreateUser | DeleteUser = Field(discriminator="kind")
```

Design rules:

```text
tag field:
  required
  stable
  explicit
  low-cardinality
  versionable if needed

tag value:
  Literal string is preferred for JSON APIs
  avoid using class names as protocol tags unless frozen by contract
  avoid numeric tags unless legacy protocol requires them
```

Value case:

```text
one field read
one branch validated
concise errors
OpenAPI discriminator generated
safe polymorphic contract
```

Discriminated unions validate more efficiently by selecting one branch and also cause generated JSON Schema to implement the OpenAPI `discriminator` attribute. ([Pydantic Docs][S20-1])

---

## 20.8 Multiple literal tags per variant

```python
class Lizard(BaseModel):
    pet_type: Literal["reptile", "lizard"]
    scales: bool
```

Use cases:

```text
legacy tag value:
  "reptile"

new tag value:
  "lizard"

same branch:
  Lizard
```

Policy:

```text
Use multi-Literal branch tags for:
  aliasing old/new discriminator values
  compatibility windows
  protocol tag migrations

Avoid:
  broad tag sets that hide semantically different cases
```

Pydantic’s discriminator example includes `Lizard` with two literal tags, `"reptile"` and `"lizard"`. ([Pydantic Docs][S20-1])

---

## 20.9 Discriminated-union error locality

```python
try:
    Model.model_validate({"pet": {"pet_type": "dog"}, "n": 1})
except ValidationError as exc:
    print(exc.errors())
```

Typical location:

```text
("pet", "dog", "barks")
```

Meaning:

```text
pet:
  union field

dog:
  selected discriminator tag / branch

barks:
  missing field inside selected Dog branch
```

Pydantic’s docs show a missing `barks` field in the dog branch producing one error at `pet.dog.barks`, rather than errors for every possible union member. ([Pydantic Docs][S20-1])

---

## 20.10 Callable discriminators

### Basic syntax

```python
from typing import Annotated, Any, Literal
from pydantic import BaseModel, Discriminator, Tag

class Pie(BaseModel):
    time_to_cook: int
    num_ingredients: int

class ApplePie(Pie):
    fruit: Literal["apple"] = "apple"

class PumpkinPie(Pie):
    filling: Literal["pumpkin"] = "pumpkin"

def get_discriminator_value(v: Any) -> str | None:
    if isinstance(v, dict):
        return v.get("fruit", v.get("filling"))
    return getattr(v, "fruit", getattr(v, "filling", None))

Dessert = Annotated[
    Annotated[ApplePie, Tag("apple")] | Annotated[PumpkinPie, Tag("pumpkin")],
    Discriminator(get_discriminator_value),
]

class ThanksgivingDinner(BaseModel):
    dessert: Dessert
```

Use callable discriminator when:

```text
no uniform discriminator field exists
tag may live under different field names per branch
union includes primitives and models
legacy formats are structurally inconsistent
```

Pydantic documents callable `Discriminator` for cases where no uniform discriminator field exists and emphasizes that callable discriminators must handle both dictionaries and model instances because Pydantic also uses them during serialization. ([Pydantic Docs][S20-1])

---

## 20.11 Callable discriminator with primitives + models

```python
from typing import Annotated, Any
from pydantic import BaseModel, Discriminator, Tag

def model_x_discriminator(v: Any) -> str | None:
    if isinstance(v, int):
        return "int"
    if isinstance(v, (dict, BaseModel)):
        return "model"
    return None

class SpecialValue(BaseModel):
    value: int

DiscriminatedValue = Annotated[
    Annotated[int, Tag("int")] | Annotated[SpecialValue, Tag("model")],
    Discriminator(model_x_discriminator),
]

class DiscriminatedModel(BaseModel):
    value: DiscriminatedValue
```

Policy:

```text
Callable discriminator function:
  pure
  total over expected inputs
  accepts dict
  accepts already-validated model instances
  accepts primitive branch inputs if primitives are in union
  returns known tag string or None
```

Pydantic’s callable discriminator docs show a union of `int` and `SpecialValue`, with a callable returning `"int"`, `"model"`, or `None`. ([Pydantic Docs][S20-1])

---

## 20.12 Callable discriminator failure modes

```text
callable returns known tag:
  selected branch validates

callable returns unknown tag:
  union_tag_invalid

callable returns None:
  union_tag_not_found
```

Design rules:

```text
Return None only when:
  discriminator truly not found

Do not:
  raise for normal unknown shape
  perform IO
  mutate input
  rely on serialization-only attributes missing from dict inputs
```

If callable discriminators do not handle both dict and model inputs, Pydantic warns they may produce serialization warnings or runtime validation errors. ([Pydantic Docs][S20-1])

---

## 20.13 Custom discriminator error design

```python
from typing import Annotated
from pydantic import BaseModel, Discriminator, Tag

class Node(BaseModel):
    kind: str

def get_tag(v):
    if isinstance(v, dict):
        return v.get("kind")
    return getattr(v, "kind", None)

NodeUnion = Annotated[
    Annotated[int, Tag("int")] | Annotated[Node, Tag("node")],
    Discriminator(
        get_tag,
        custom_error_type="invalid_node_union",
        custom_error_message="Input is not a valid node union member",
        custom_error_context={"allowed": "int or node"},
    ),
]
```

Use when:

```text
public API should expose one stable domain error code
LLM repair prompt should avoid internal branch names
frontend form mapper needs one concise code
legacy clients expect a specific error shape
```

Pydantic supports `custom_error_type`, `custom_error_message`, and `custom_error_context` on callable `Discriminator`, letting applications control discriminator failure diagnostics. ([Pydantic Docs][S20-1])

---

## 20.14 Nested discriminated unions

### Type + subtype

```python
from typing import Annotated, Literal
from pydantic import BaseModel, Field

class BlackCat(BaseModel):
    pet_type: Literal["cat"]
    color: Literal["black"]
    black_name: str

class WhiteCat(BaseModel):
    pet_type: Literal["cat"]
    color: Literal["white"]
    white_name: str

Cat = Annotated[
    BlackCat | WhiteCat,
    Field(discriminator="color"),
]

class Dog(BaseModel):
    pet_type: Literal["dog"]
    name: str

Pet = Annotated[
    Cat | Dog,
    Field(discriminator="pet_type"),
]

class Model(BaseModel):
    pet: Pet
```

Meaning:

```text
outer discriminator:
  pet_type
  chooses cat vs dog

inner discriminator:
  color
  chooses black cat vs white cat
```

Pydantic documents nested discriminated unions using nested `Annotated` types and notes that only one discriminator can be set for a field, so multiple discriminator layers require nesting. ([Pydantic Docs][S20-1])

### Use cases

```text
event_type + event_version
resource_type + action
provider + provider_specific_type
message_family + message_kind
animal_type + subtype
```

Agent rule:

```text
Use nested discriminators when polymorphism is hierarchical.
Do not collapse stable hierarchy into one giant callable discriminator unless fields are structurally inconsistent.
```

---

## 20.15 Discriminators with `RootModel` literal root tags

```python
from typing import Literal
from pydantic import RootModel

class CatTag(RootModel[Literal["cat"]]):
    pass
```

Pydantic v2.13 adds support for root models with literal root types in place of `Literal` types in discriminator contexts. ([Pydantic Docs][S20-1])

Use case:

```text
tag type has class identity
tag schema/title needs naming
tag participates in reusable root model hierarchy
```

---

## 20.16 OpenAPI schema implications

Discriminated unions affect JSON Schema / OpenAPI:

```text
plain union:
  anyOf / oneOf-like branch schema shape depending generator/details
  no discriminator metadata

discriminated union:
  OpenAPI discriminator attribute emitted
  branch mapping based on literal tags
  cleaner generated contract for clients
```

Pydantic states that adding a discriminator to unions means the generated JSON Schema implements the OpenAPI `discriminator` attribute. ([Pydantic Docs][S20-1])

### OpenAPI-oriented pattern

```python
class EventBase(BaseModel):
    event_type: str

class UserCreated(BaseModel):
    event_type: Literal["user.created"]
    user_id: int

class UserDeleted(BaseModel):
    event_type: Literal["user.deleted"]
    user_id: int

Event = Annotated[
    UserCreated | UserDeleted,
    Field(discriminator="event_type"),
]
```

API schema rules:

```text
tag name:
  public, stable, documented

tag values:
  semantic protocol values
  not arbitrary Python class names unless locked

branch schemas:
  should have non-overlapping required fields where possible

versioning:
  include version in tag or add separate version discriminator when needed
```

---

## 20.17 Performance implications

```text
plain untagged union:
  may attempt multiple branches
  may collect errors from multiple branches
  cost grows with branch count and model depth

smart union:
  may evaluate multiple successful branches to select best match
  model/dataclass/TypedDict branches can use valid-field-count heuristic

left_to_right union:
  stops at first success
  deterministic order
  can be surprising in lax mode

discriminated union:
  reads tag
  validates one branch
  avoids branch proliferation
  recommended for performance and predictability
```

Pydantic’s performance docs explicitly recommend tagged/discriminated unions over ordinary unions, and the union docs say discriminated unions are both more performant and predictable because they control which member is validated. ([Pydantic Docs][S20-2])

Agent rule:

```text
High-throughput polymorphic payload:
  discriminated union

Small scalar convenience union:
  smart union often fine

Legacy order-sensitive scalar union:
  left_to_right + tests

Large untagged model union:
  avoid or benchmark
```

---

## 20.18 Error-message design for polymorphic payloads

### Bad public error shape: untagged union

```python
class Payload(BaseModel):
    pet: Cat | Dog | Lizard
```

Potential problems:

```text
many branch errors
irrelevant branch diagnostics
hard for API clients
large LLM repair prompts
less deterministic for ambiguous payloads
```

### Better public error shape: discriminator

```python
class Payload(BaseModel):
    pet: Cat | Dog | Lizard = Field(discriminator="pet_type")
```

Better diagnostics:

```text
wrong tag:
  one union_tag_invalid / union_tag_not_found error

right tag but bad branch:
  one branch-local error, e.g. pet.dog.barks

missing discriminator:
  concise discriminator-not-found error
```

Design rules:

```text
For API consumers:
  require discriminator
  document tag values
  emit alias-based locs
  keep tags stable
  use custom discriminator error code for domain UX when needed

For LLM repair:
  include expected tag field and allowed tags
  avoid full branch-error explosion
```

---

## 20.19 Serialization implications for callable discriminators

Callable discriminator functions are used during both validation and serialization. Therefore:

```text
During validation:
  input may be dict
  input may be primitive
  input may be arbitrary object

During serialization:
  input may be already validated BaseModel subclass
  input may be dataclass/model instance
```

Required function shape:

```python
def discriminator(v: Any) -> str | None:
    if isinstance(v, dict):
        ...
    return getattr(v, "tag_field", None)
```

Pydantic explicitly warns that callable discriminators should handle both dict and model inputs because failing to do so can cause warnings during serialization or runtime errors during validation. ([Pydantic Docs][S20-1])

Agent rule:

```text
Callable discriminator is a serializer path dependency.
Test:
  model_validate(...)
  model_dump(...)
  model_dump_json(...)
```

---

## 20.20 Strictness and unions

```python
class M(BaseModel):
    value: int | str
```

Behavior considerations:

```text
lax mode:
  "123" can validate as int
  branch selection depends on union mode and smart scoring

strict mode:
  "123" cannot validate as int
  but can validate as str
```

Policy:

```text
Ambiguous scalar union:
  test lax and strict behavior separately

API accepts stringified numbers:
  document that output branch may be int or str depending mode/order

Need deterministic "string stays string":
  prefer smart mode with exactness
  or avoid int | str ambiguity
  or use discriminated shape
```

Smart mode exactness scoring includes exact type matches, strict-mode successes, and lax-mode successes. ([Pydantic Docs][S20-1])

---

## 20.21 Union design patterns

### Result envelope

```python
class Success(BaseModel):
    status: Literal["success"]
    data: dict[str, object]

class Failure(BaseModel):
    status: Literal["failure"]
    error_code: str
    message: str

Result = Annotated[Success | Failure, Field(discriminator="status")]
```

### Event bus

```python
class UserCreated(BaseModel):
    type: Literal["user.created"]
    user_id: int
    email: str

class UserDeleted(BaseModel):
    type: Literal["user.deleted"]
    user_id: int

Event = Annotated[UserCreated | UserDeleted, Field(discriminator="type")]
```

### Versioned payload

```python
class PaymentV1(BaseModel):
    version: Literal["v1"]
    amount_cents: int

class PaymentV2(BaseModel):
    version: Literal["v2"]
    amount: str
    currency: str

Payment = Annotated[PaymentV1 | PaymentV2, Field(discriminator="version")]
```

### Hierarchical event

```python
class UserCreatedV1(BaseModel):
    event_type: Literal["user"]
    version: Literal["v1"]
    email: str

class UserCreatedV2(BaseModel):
    event_type: Literal["user"]
    version: Literal["v2"]
    email: str
    display_name: str

UserEvent = Annotated[UserCreatedV1 | UserCreatedV2, Field(discriminator="version")]

class OrderCreated(BaseModel):
    event_type: Literal["order"]
    order_id: int

DomainEvent = Annotated[UserEvent | OrderCreated, Field(discriminator="event_type")]
```

---

## 20.22 Anti-patterns

### Ambiguous model union

```python
class A(BaseModel):
    name: str
    value: int | None = None

class B(BaseModel):
    name: str
    count: int | None = None

class M(BaseModel):
    item: A | B
```

Problem:

```text
same shape can match both
smart-mode choice can be non-obvious
error output can include multiple branch failures
```

Preferred:

```python
class A(BaseModel):
    kind: Literal["a"]
    name: str
    value: int | None = None

class B(BaseModel):
    kind: Literal["b"]
    name: str
    count: int | None = None

class M(BaseModel):
    item: A | B = Field(discriminator="kind")
```

---

### `left_to_right` without branch-order tests

```python
id: int | str = Field(union_mode="left_to_right")
```

Required test:

```python
assert isinstance(Model(id="123").id, int)
```

---

### Callable discriminator only handling dicts

```python
def get_tag(v):
    return v["kind"]
```

Preferred:

```python
def get_tag(v):
    if isinstance(v, dict):
        return v.get("kind")
    return getattr(v, "kind", None)
```

---

### Discriminator field not typed as `Literal`

```python
class Cat(BaseModel):
    kind: str
```

Preferred:

```python
class Cat(BaseModel):
    kind: Literal["cat"]
```

Discriminator fields should be literal-constrained so Pydantic can map tag values to branches. ([Pydantic Docs][S20-1])

---

### Hidden class-name tags

```python
kind: Literal["CatModel"]
```

Prefer protocol tags:

```python
kind: Literal["cat"]
```

Unless class names are explicitly part of the public protocol.

---

## 20.23 Test matrix

### Smart scalar union

```python
def test_smart_union_string_exact_match():
    class M(BaseModel):
        x: int | str

    assert M(x="123").x == "123"
    assert M(x=123).x == 123
```

### Left-to-right order

```python
def test_left_to_right_order():
    class M(BaseModel):
        x: int | str = Field(union_mode="left_to_right")

    assert M(x="123").x == 123
    assert isinstance(M(x="123").x, int)
```

### Plain union errors

```python
def test_union_branch_errors():
    class M(BaseModel):
        x: str | int = Field(union_mode="left_to_right")

    with pytest.raises(ValidationError) as exc:
        M(x=[])

    locs = {e["loc"] for e in exc.value.errors()}
    assert ("x", "str") in locs
    assert ("x", "int") in locs
```

### Discriminated union branch selection

```python
def test_discriminated_union_selection():
    class Cat(BaseModel):
        kind: Literal["cat"]
        meows: int

    class Dog(BaseModel):
        kind: Literal["dog"]
        barks: int

    class M(BaseModel):
        pet: Cat | Dog = Field(discriminator="kind")

    m = M.model_validate({"pet": {"kind": "dog", "barks": "3"}})
    assert isinstance(m.pet, Dog)
    assert m.pet.barks == 3
```

### Discriminated union error locality

```python
def test_discriminated_union_error_locality():
    class Cat(BaseModel):
        kind: Literal["cat"]
        meows: int

    class Dog(BaseModel):
        kind: Literal["dog"]
        barks: int

    class M(BaseModel):
        pet: Cat | Dog = Field(discriminator="kind")

    with pytest.raises(ValidationError) as exc:
        M.model_validate({"pet": {"kind": "dog"}})

    err = exc.value.errors()[0]
    assert err["loc"] == ("pet", "dog", "barks")
    assert err["type"] == "missing"
```

### Invalid discriminator tag

```python
def test_invalid_discriminator_tag():
    class Cat(BaseModel):
        kind: Literal["cat"]
        meows: int

    class Dog(BaseModel):
        kind: Literal["dog"]
        barks: int

    class M(BaseModel):
        pet: Cat | Dog = Field(discriminator="kind")

    with pytest.raises(ValidationError) as exc:
        M.model_validate({"pet": {"kind": "bird"}})

    assert exc.value.errors()[0]["type"] == "union_tag_invalid"
```

### Callable discriminator handles dict and model

```python
def test_callable_discriminator_validation_and_serialization():
    class ApplePie(BaseModel):
        fruit: Literal["apple"] = "apple"
        time_to_cook: int

    class PumpkinPie(BaseModel):
        filling: Literal["pumpkin"] = "pumpkin"
        time_to_cook: int

    def get_tag(v: Any) -> str | None:
        if isinstance(v, dict):
            return v.get("fruit", v.get("filling"))
        return getattr(v, "fruit", getattr(v, "filling", None))

    Dessert = Annotated[
        Annotated[ApplePie, Tag("apple")] | Annotated[PumpkinPie, Tag("pumpkin")],
        Discriminator(get_tag),
    ]

    class Dinner(BaseModel):
        dessert: Dessert

    d = Dinner.model_validate({"dessert": {"fruit": "apple", "time_to_cook": "60"}})
    assert isinstance(d.dessert, ApplePie)
    assert d.model_dump()["dessert"]["fruit"] == "apple"
```

### Nested discriminated union

```python
def test_nested_discriminated_union():
    class BlackCat(BaseModel):
        pet_type: Literal["cat"]
        color: Literal["black"]
        black_name: str

    class WhiteCat(BaseModel):
        pet_type: Literal["cat"]
        color: Literal["white"]
        white_name: str

    Cat = Annotated[BlackCat | WhiteCat, Field(discriminator="color")]

    class Dog(BaseModel):
        pet_type: Literal["dog"]
        name: str

    Pet = Annotated[Cat | Dog, Field(discriminator="pet_type")]

    class M(BaseModel):
        pet: Pet

    m = M.model_validate({"pet": {"pet_type": "cat", "color": "black", "black_name": "Midnight"}})
    assert isinstance(m.pet, BlackCat)
```

### OpenAPI discriminator schema smoke test

```python
def test_discriminated_union_schema_has_discriminator():
    class Cat(BaseModel):
        kind: Literal["cat"]
        meows: int

    class Dog(BaseModel):
        kind: Literal["dog"]
        barks: int

    class M(BaseModel):
        pet: Cat | Dog = Field(discriminator="kind")

    pet_schema = M.model_json_schema()["properties"]["pet"]
    assert "discriminator" in pet_schema
```

---

## 20.24 Deployment decision matrix

```text
Need / scenario                              Recommended union design
--------------------------------------------------------------------------------
small scalar convenience                      smart union: int | str | UUID
specific ordered legacy priority              Field(union_mode="left_to_right") + tests
public API polymorphism                       discriminated union
OpenAPI client generation                     discriminated union with Literal tags
event bus / message queue                     Field(discriminator="type")
versioned payloads                            Field(discriminator="version")
hierarchical protocol                         nested discriminated unions
no uniform tag field                          callable Discriminator + Tag
primitive + model polymorphism                callable Discriminator + Tag
LLM structured output                         discriminated union with clear tag field
performance-sensitive polymorphism            discriminated union
verbose union error problem                   discriminated union / custom Discriminator error
ambiguous model branches                      redesign with tag
recursive polymorphism                        discriminated where possible; custom validator only if unavoidable
```

---

## 20.25 Agent checklist

```text
[ ] Use A | B syntax on Python 3.10+; Union[A, B] when compatibility/readability requires it.
[ ] Treat untagged unions as ambiguous unless proven otherwise.
[ ] Use discriminated unions for public polymorphic payloads.
[ ] Put a required Literal discriminator field on every branch.
[ ] Use Field(discriminator="kind") for uniform tag fields.
[ ] Use callable Discriminator only when no uniform tag field exists.
[ ] Callable discriminator must handle dict and model instance inputs.
[ ] Callable discriminator must be pure and side-effect-free.
[ ] Use Tag(...) for callable-discriminator branches.
[ ] Use nested Annotated unions for multiple discriminator layers.
[ ] Use union_mode="left_to_right" only with branch-order tests.
[ ] Do not depend on exact smart-mode behavior for ambiguous inputs.
[ ] Test invalid tag, missing tag, valid tag with invalid branch data.
[ ] Test model_dump/model_dump_json for callable discriminators.
[ ] Use custom discriminator error type/message/context when API UX requires one concise error.
[ ] Snapshot OpenAPI discriminator schema for public APIs.
[ ] Prefer discriminated unions for performance and concise errors.
```

---

## 20.26 Value case

```text
Union / polymorphism value =
  one field can accept multiple typed shapes
  scalar compatibility without ad hoc parsing
  deterministic legacy branch priority through left_to_right mode
  smart default matching for convenience unions
  tagged/discriminated validation for public polymorphism
  callable tag extraction for inconsistent legacy formats
  nested discriminator hierarchies for type/subtype protocols
  OpenAPI discriminator output for client generation
  lower validation cost through one-branch dispatch
  clearer error messages for clients, CLIs, and LLM repair loops
```

Operationally: use plain unions sparingly for small, low-ambiguity scalar alternatives; use `left_to_right` only when ordering is a tested contract; use discriminated unions for API, event, LLM, and plugin polymorphism; design discriminator tags as public protocol fields; and test validation, serialization, errors, and generated schema for every polymorphic boundary.

[S20-1]: https://docs.pydantic.dev/latest/concepts/unions/ "Unions | Pydantic Docs"
[S20-2]: https://docs.pydantic.dev/latest/concepts/performance/ "Performance | Pydantic Docs"


# Pydantic Advanced — 21) Functional validation of callables with `@validate_call`

Style target: advanced, sectioned, agent-oriented technical reference. 

`@validate_call` decorates a function so its **arguments are parsed/coerced/validated from the function signature and annotations before the original function body runs**; return-value validation is optional and disabled by default. It can be used as `@validate_call` or `@validate_call(config=..., validate_return=...)`. ([Pydantic][S21-1])

---

## 21.0 Mental model

```text
function signature
  + type annotations
  + Field / Annotated metadata
  + ConfigDict
  + validate_return flag
  → generated call validator
  → runtime call:
      bind args/kwargs
      validate/coerce arguments
      call original function
      optionally validate return value
```

Use `@validate_call` when the validation target is a **call boundary**, not a persistent data object.

```python
from pydantic import validate_call

@validate_call
def repeat(s: str, count: int, *, separator: bytes = b"") -> bytes:
    b = s.encode()
    return separator.join(b for _ in range(count))

assert repeat("x", "4", separator=b" ") == b"x x x x"
```

Unannotated parameters are treated as `Any`, so they are accepted without type validation/coercion beyond normal binding. Pydantic’s docs show this with an unannotated `include_equal=False` parameter. ([Pydantic][S21-1])

---

## 21.1 API surface

```python
from pydantic import validate_call

@validate_call
def f(...):
    ...

@validate_call(config=ConfigDict(...), validate_return=True)
def g(...):
    ...
```

Formal API:

```python
def validate_call(
    func: AnyCallableT | None = None,
    /,
    *,
    config: ConfigDict | None = None,
    validate_return: bool = False,
) -> AnyCallableT | Callable[[AnyCallableT], AnyCallableT]:
    ...
```

Parameters:

```text
func:
  function to decorate

config:
  ConfigDict controlling call validation behavior

validate_return:
  validate annotated return value after function execution
  default False
```

The API docs state that `validate_call` returns a decorated wrapper around the function that validates arguments and optionally the return value. ([Pydantic][S21-2])

---

## 21.2 Decorating functions

### Plain decorator

```python
from pydantic import validate_call

@validate_call
def add(x: int, y: int) -> int:
    return x + y

assert add("1", "2") == 3
```

### Decorator with config

```python
from pydantic import ConfigDict, validate_call

@validate_call(config=ConfigDict(strict=True))
def add_strict(x: int, y: int) -> int:
    return x + y
```

### Decorator with return validation

```python
from pydantic import validate_call

@validate_call(validate_return=True)
def get_count() -> int:
    return "1"

assert get_count() == 1
```

Return validation is disabled by default and enabled with `validate_return=True`. ([Pydantic][S21-1])

---

## 21.3 Argument validation: positional and keyword parameters

```python
from pydantic import validate_call

@validate_call
def pos_or_kw(a: int, b: int = 2) -> str:
    return f"a={a} b={b}"

assert pos_or_kw("1") == "a=1 b=2"
assert pos_or_kw(1, b="3") == "a=1 b=3"
```

Supported parameter categories:

```text
positional-or-keyword
keyword-only
positional-only
varargs: *args
varkwargs: **kwargs
defaults
mixed signatures
```

Pydantic documents support for all standard Python parameter configurations, including positional-or-keyword, keyword-only, positional-only, `*args`, and `**kwargs`. ([Pydantic][S21-1])

---

## 21.4 Argument validation: keyword-only parameters

```python
from pydantic import validate_call

@validate_call
def kw_only(*, a: int, b: int = 2) -> str:
    return f"a={a} b={b}"

assert kw_only(a="1") == "a=1 b=2"
assert kw_only(a=1, b="3") == "a=1 b=3"
```

Use keyword-only parameters for public APIs where positional order should not become a compatibility contract.

---

## 21.5 Argument validation: positional-only parameters

```python
from pydantic import validate_call

@validate_call
def pos_only(a: int, b: int = 2, /) -> str:
    return f"a={a} b={b}"

assert pos_only("1") == "a=1 b=2"
```

Pydantic validates positional-only arguments by positional index in the validation error location. Missing/wrong values produce `ValidationError`, not Python’s normal `TypeError`. ([Pydantic][S21-1])

---

## 21.6 Argument validation: `*args`

```python
from pydantic import validate_call

@validate_call
def var_args(*args: int) -> tuple[int, ...]:
    return args

assert var_args("1", 2, 3.0) == (1, 2, 3)
```

Validation semantics:

```text
*args: int
  every extra positional argument validates as int
  output passed to function is normal Python tuple
```

Pydantic’s docs show `*args: int` validating every positional argument. ([Pydantic][S21-1])

---

## 21.7 Argument validation: `**kwargs`

```python
from pydantic import validate_call

@validate_call
def var_kwargs(**kwargs: int) -> dict[str, int]:
    return kwargs

assert var_kwargs(a="1", b=2) == {"a": 1, "b": 2}
```

Validation semantics:

```text
**kwargs: int
  every arbitrary keyword value validates as int
```

Pydantic’s docs show `**kwargs: int` validating arbitrary keyword values. ([Pydantic][S21-1])

---

## 21.8 Argument validation: typed `**kwargs` with `Unpack[TypedDict]`

```python
from typing_extensions import TypedDict, Unpack
from pydantic import validate_call

class Point(TypedDict):
    x: int
    y: int

@validate_call
def add_coords(**kwargs: Unpack[Point]) -> int:
    return kwargs["x"] + kwargs["y"]

assert add_coords(x="1", y=2) == 3
```

`Unpack[TypedDict]` can annotate variable keyword parameters, giving `**kwargs` a fixed required/optional key structure rather than validating every keyword value as the same type. Pydantic documents this pattern and references PEP 692. ([Pydantic][S21-1])

---

## 21.9 Complex mixed signature

```python
from pydantic import validate_call

@validate_call
def command(
    a: int,
    /,
    b: int,
    *c: int,
    d: int,
    e: int | None = None,
    **f: int,
) -> tuple[int, int, tuple[int, ...], int, int | None, dict[str, int]]:
    return a, b, c, d, e, f

assert command(1, "2", "3", "4", d="5", e="6", extra="7") == (
    1,
    2,
    (3, 4),
    5,
    6,
    {"extra": 7},
)
```

Use this sparingly. For public library APIs, prefer clear keyword-only parameters and avoid mixing positional-only, varargs, and arbitrary kwargs unless the Python API genuinely requires it.

---

## 21.10 `Field(...)` and `Annotated[...]` for function parameters

```python
from typing import Annotated
from pydantic import Field, validate_call

@validate_call
def how_many(num: Annotated[int, Field(gt=10)]) -> int:
    return num

assert how_many("42") == 42
```

Use `Annotated[T, Field(...)]` when adding constraints or metadata without pretending a default exists:

```python
PositiveCount = Annotated[int, Field(gt=0)]

@validate_call
def retry(count: PositiveCount) -> int:
    return count
```

Pydantic’s docs recommend the `Annotated` pattern when using `Field()` without `default` or `default_factory`, so type checkers still infer the parameter as required. ([Pydantic][S21-1])

---

## 21.11 Default values with `Field(...)`

```python
from pydantic import Field, validate_call

@validate_call
def return_value(value: str = Field(default="default value")) -> str:
    return value

assert return_value() == "default value"
```

Use direct defaults for simple cases:

```python
@validate_call
def f(limit: int = 100) -> int:
    return limit
```

Use `Field(default=..., ge=..., description=...)` when the default and validation metadata belong together.

---

## 21.12 Parameter aliases

```python
from typing import Annotated
from pydantic import Field, validate_call

@validate_call
def how_many(num: Annotated[int, Field(gt=10, alias="number")]) -> int:
    return num

assert how_many(number="42") == 42
```

Aliases work with `@validate_call` similarly to model fields; use them when external call sites or compatibility layers use a different keyword name. ([Pydantic][S21-1])

Agent rule:

```text
Function aliases are useful for:
  migration compatibility
  external plugin/config calls
  CLI/generated-call layers

Avoid aliases for:
  ordinary internal Python APIs
  code where signature readability matters more than compatibility
```

---

## 21.13 Return validation

### Disabled by default

```python
from pydantic import validate_call

@validate_call
def f() -> int:
    return "1"

assert f() == "1"
```

### Enabled

```python
@validate_call(validate_return=True)
def g() -> int:
    return "1"

assert g() == 1
```

### Failure

```python
from pydantic import ValidationError

@validate_call(validate_return=True)
def h() -> int:
    return "bad"

try:
    h()
except ValidationError as exc:
    assert exc.errors()[0]["type"] in {"int_parsing", "int_type"}
```

Return validation is opt-in with `validate_return=True`; without it, Pydantic validates arguments only. ([Pydantic][S21-1])

Deployment rule:

```text
Use validate_return=True for:
  public library APIs
  plugin hooks
  task queue functions
  notebook utilities where outputs become downstream inputs
  boundary functions returning parsed external data

Avoid validate_return=True for:
  hot inner loops
  functions already statically/test-verified
  performance-critical service internals
```

---

## 21.14 Custom configuration

```python
from pydantic import ConfigDict, ValidationError, validate_call

class Foobar:
    def __init__(self, v: str) -> None:
        self.v = v

@validate_call(config=ConfigDict(arbitrary_types_allowed=True))
def add_foobars(a: Foobar, b: Foobar) -> str:
    return f"{a.v}+{b.v}"

assert add_foobars(Foobar("a"), Foobar("b")) == "a+b"

try:
    add_foobars(1, 2)
except ValidationError as exc:
    assert exc.errors()[0]["type"] == "is_instance_of"
```

`config=ConfigDict(...)` lets the decorator use Pydantic configuration such as `arbitrary_types_allowed`, `strict`, string normalization, alias behavior, and other relevant config options. Pydantic documents `arbitrary_types_allowed=True` with `@validate_call` for arbitrary class instances. ([Pydantic][S21-1])

Common config patterns:

```python
@validate_call(config=ConfigDict(strict=True))
def strict_api(x: int) -> int:
    return x

@validate_call(config=ConfigDict(arbitrary_types_allowed=True))
def accepts_client(client: SomeClient) -> None:
    ...

@validate_call(config=ConfigDict(str_strip_whitespace=True))
def normalize_name(name: str) -> str:
    return name
```

---

## 21.15 Async functions

```python
from pydantic import PositiveInt, validate_call

@validate_call
async def get_user_email(user_id: PositiveInt) -> str:
    return f"user-{user_id}@example.com"

email = await get_user_email("123")
```

`@validate_call` can decorate async functions; argument validation occurs before the coroutine body executes, and validation failures raise `ValidationError` before awaiting user code. ([Pydantic][S21-1])

Agent rule:

```text
Async service boundary:
  @validate_call can guard coroutine arguments.

But:
  it does not validate awaited side effects.
  it does not replace authorization.
  it does not make database/network operations safe.
```

---

## 21.16 Error model

```python
from pydantic import ValidationError, validate_call

@validate_call
def f(x: int) -> int:
    return x

try:
    f("bad")
except ValidationError as exc:
    err = exc.errors()[0]
    assert err["loc"] == (0,)
    assert err["type"] == "int_parsing"
```

Important behavior:

```text
validation failure:
  raises pydantic.ValidationError

missing required argument:
  raises ValidationError, not TypeError

error loc:
  positional args use integer index
  keyword args use keyword name
  nested input uses nested paths
```

Pydantic documents this limitation explicitly: validation failure raises a standard Pydantic `ValidationError`, including cases like missing required arguments where Python would normally raise `TypeError`. ([Pydantic][S21-1])

Boundary mapping:

```text
Public API / CLI / task worker:
  catch ValidationError
  map to structured validation response

Internal programmer error:
  let TypeError and unexpected exceptions propagate

Validator-intent failures:
  raise ValueError / PydanticCustomError inside validators
```

---

## 21.17 Accessing the original function: `.raw_function`

```python
from pydantic import validate_call

@validate_call
def repeat(s: str, count: int, *, separator: bytes = b"") -> bytes:
    b = s.encode()
    return separator.join(b for _ in range(count))

safe = repeat("hello", 3)
fast = repeat.raw_function("hello", 3, separator=b" ")
```

`.raw_function` bypasses validation and calls the original decorated function directly. Pydantic documents this as useful when inputs are trusted and maximum call efficiency matters; type checkers may not recognize `.raw_function` because of current Python type-system limitations. ([Pydantic][S21-1])

Agent rule:

```text
Use raw_function only when:
  arguments are already validated
  hot path matters
  bypass is documented and tested

Never use raw_function for:
  user input
  API payloads
  task queue messages
  plugin calls from untrusted sources
```

---

## 21.18 Validate arguments before expensive execution

Pattern:

```python
from collections.abc import Callable
from pydantic import validate_call

@validate_call
def validate_foo(a: int, b: int) -> Callable[[], int]:
    def foo() -> int:
        return a + b
    return foo

validated_callable = validate_foo(a="1", b="2")
result = validated_callable()
```

Pydantic documents this workaround for separating validation from a costly function call: make the decorated function validate inputs and return a callable that performs the expensive work later. ([Pydantic][S21-1])

Use cases:

```text
preflight validation
job scheduling
task queue enqueue validation
expensive computation setup
delayed execution
authorization step after validation but before execution
```

---

## 21.19 Service-layer guards

```python
from typing import Annotated
from pydantic import Field, validate_call

UserId = Annotated[int, Field(gt=0)]
Email = Annotated[str, Field(min_length=3)]

@validate_call(config=ConfigDict(strict=False))
def create_user(user_id: UserId, email: Email, *, send_email: bool = True) -> None:
    ...
```

Use for:

```text
thin service boundary
script entrypoints
manual/admin tools
integration points where raw data may be stringly
non-framework code where BaseModel DTO would be overkill
```

Avoid:

```text
core hot loops
functions called millions of times internally
business methods where caller already holds validated DTO
functions whose true contract needs a named object
```

---

## 21.20 Public library APIs

```python
from pathlib import Path
from typing import Annotated
from pydantic import Field, validate_call

@validate_call(config=ConfigDict(strict=True))
def load_config(
    path: Path,
    *,
    retries: Annotated[int, Field(ge=0, le=10)] = 3,
) -> dict:
    ...
```

Benefits:

```text
runtime guard for untyped users
clear validation errors
coercion policy configurable
parameter constraints near signature
minimal boilerplate
```

Risks:

```text
runtime overhead per call
ValidationError instead of TypeError
decorator may surprise users expecting normal Python call semantics
type checkers do not see raw_function
```

Pydantic notes the decorator preserves the decorated function’s signature and should be compatible with type checkers, but attributes such as `.raw_function` may not be recognized. ([Pydantic][S21-1])

---

## 21.21 Task queues

```python
from typing import Literal
from pydantic import BaseModel, validate_call

class JobOptions(BaseModel):
    priority: Literal["low", "normal", "high"] = "normal"
    retries: int = 3

@validate_call(validate_return=True)
def enqueue_email_job(
    recipient: str,
    template_id: str,
    options: JobOptions = JobOptions(),
) -> str:
    job_id = "job_123"
    return job_id
```

Patterns:

```text
validate before enqueue
validate job handler args
validate return job_id/status
avoid constructing invalid jobs
convert stringly scheduler payloads
```

Better for complex task payloads:

```python
class EmailJob(BaseModel):
    recipient: str
    template_id: str
    options: JobOptions

def enqueue(job: EmailJob) -> str:
    ...
```

Agent rule:

```text
Use @validate_call for simple task function boundaries.
Use BaseModel DTOs for task payloads that need persistence, schema, serialization, or versioning.
```

---

## 21.22 Notebook utilities

```python
from typing import Annotated
from pydantic import Field, validate_call

@validate_call
def sample_rows(
    path: str,
    n: Annotated[int, Field(gt=0, le=10_000)] = 100,
) -> list[dict]:
    ...
```

Use cases:

```text
interactive notebooks
data science helper functions
quick scripts
manual admin utilities
```

Value:

```text
clear early error
coercion from notebook widget strings
fewer ad hoc asserts
constraints inline with signature
```

---

## 21.23 Relationship to `BaseModel`

```python
class CreateUser(BaseModel):
    user_id: int
    email: str

def create_user(cmd: CreateUser) -> None:
    ...
```

`BaseModel` is better when:

```text
data object has identity
payload is reused across functions
serialization/schema required
field tracking needed
multiple validation entrypoints needed
complex nested contract
OpenAPI/JSON Schema contract exists
```

`@validate_call` is better when:

```text
function signature is the contract
no DTO object should survive
boundary is a Python call
minimal boilerplate desired
parameter validation only
```

Under the hood, Pydantic says the decorator uses the same general approach as model creation/initialization, but it exposes validation directly at function-call boundaries with minimal boilerplate. ([Pydantic][S21-1])

---

## 21.24 Relationship to `TypeAdapter`

```python
from pydantic import TypeAdapter

ARGS = TypeAdapter(tuple[int, int])

def add(x: int, y: int) -> int:
    x, y = ARGS.validate_python((x, y))
    return x + y
```

`TypeAdapter` is better when:

```text
you need to validate a standalone type
you want manual control over when validation happens
you need top-level list/dict/union/scalar validation
you want reusable cached validator
you want to validate args without decorating function
```

`@validate_call` is better when:

```text
signature binding matters
positional/keyword/default semantics matter
function itself should enforce validation
minimal wrapper code desired
```

Manual adapter pattern for pre-validated calls:

```python
from typing_extensions import TypedDict
from pydantic import TypeAdapter

class AddKwargs(TypedDict):
    x: int
    y: int

ADD_KWARGS = TypeAdapter(AddKwargs)

def add_impl(x: int, y: int) -> int:
    return x + y

def add_from_payload(payload: dict) -> int:
    kwargs = ADD_KWARGS.validate_python(payload)
    return add_impl(**kwargs)
```

---

## 21.25 Performance considerations

```text
decorator setup:
  function inspection performed once

each call:
  argument binding/validation overhead remains
  return validation adds more overhead if enabled

raw_function:
  bypasses validation overhead
  only for trusted inputs
```

Pydantic documents that function inspection is performed once, but every decorated function call still has performance overhead compared with calling the original function; the decorator is not equivalent to function definitions in strongly typed languages. ([Pydantic][S21-1])

Performance policy:

```text
Use @validate_call:
  boundary functions
  low/medium call frequency
  public APIs
  developer-facing utilities
  task entrypoints

Avoid @validate_call:
  inner loops
  tight numerical code
  serializers/parsers called per item in huge batches
  already validated service internals
```

Benchmark harness:

```python
from timeit import timeit
from pydantic import validate_call

def raw_add(x: int, y: int) -> int:
    return x + y

@validate_call
def checked_add(x: int, y: int) -> int:
    return x + y

print(timeit(lambda: raw_add(1, 2), number=1_000_000))
print(timeit(lambda: checked_add(1, 2), number=1_000_000))
print(timeit(lambda: checked_add.raw_function(1, 2), number=1_000_000))
```

---

## 21.26 Limitations

```text
ValidationError, not TypeError:
  validation failures and missing args raise Pydantic ValidationError

Runtime overhead:
  every call validates unless raw_function is used

No separate public "validate args only" API:
  documented workaround returns a callable

Type checker limitations:
  signature preserved, but raw_function attribute may not be recognized

Not a replacement for static typing:
  runtime validation is not equivalent to compiler-enforced typed functions

Not a DTO/schema object:
  no model_fields, model_dump, model_json_schema for function call itself
```

These limitations are directly documented: failures raise `ValidationError`; missing arguments also raise `ValidationError`; decorated calls carry overhead; and `@validate_call` is not equivalent to strongly typed language function definitions. ([Pydantic][S21-1])

---

## 21.27 Error handling patterns

### API/task boundary

```python
from pydantic import ValidationError

try:
    result = task_handler(payload)
except ValidationError as exc:
    errors = exc.errors(include_input=False, include_url=False)
    ...
```

### Library boundary

```python
def public_api(...):
    try:
        return _validated_public_api(...)
    except ValidationError as exc:
        raise ValueError("Invalid public API arguments") from exc
```

### CLI boundary

```python
try:
    main(...)
except ValidationError as exc:
    print(exc)
    raise SystemExit(2)
```

Agent rule:

```text
For machine handling:
  use exc.errors()[*]["type"]

For human CLI:
  str(exc) acceptable

For public logs/API:
  include_input=False
```

---

## 21.28 Deployment decision matrix

```text
Need / situation                              Recommended surface
--------------------------------------------------------------------------------
validate Python call args                      @validate_call
validate return value                          @validate_call(validate_return=True)
strict call boundary                           @validate_call(config=ConfigDict(strict=True))
arbitrary runtime handles                      @validate_call(config=ConfigDict(arbitrary_types_allowed=True))
parameter constraints                          Annotated[T, Field(...)]
parameter alias                                Annotated[T, Field(alias="...")]
async function guard                           @validate_call on async def
call original trusted fast path                decorated_fn.raw_function(...)
preflight expensive call                       decorated validator returning callable
reusable payload object                        BaseModel DTO
standalone top-level type validation           TypeAdapter(T)
hot inner loop                                 avoid @validate_call or use raw_function after validation
public schema/OpenAPI needed                   BaseModel, not @validate_call
task payload persistence/versioning            BaseModel, not only @validate_call
```

---

## 21.29 Anti-patterns

### Decorating hot inner loops

```python
@validate_call
def dot(x: float, y: float) -> float:
    return x * y
```

Better:

```python
def dot(x: float, y: float) -> float:
    return x * y
```

Validate once at boundary, not per arithmetic operation.

---

### Using `@validate_call` instead of a DTO for reusable payloads

```python
@validate_call
def handle(user_id: int, email: str, role: str) -> None:
    ...
```

If the same payload is stored, serialized, queued, versioned, or passed through multiple functions:

```python
class UserCommand(BaseModel):
    user_id: int
    email: str
    role: str

def handle(cmd: UserCommand) -> None:
    ...
```

---

### Expecting Python `TypeError`

```python
@validate_call
def f(x: int) -> int:
    return x

f()  # ValidationError, not TypeError
```

Pydantic explicitly documents this limitation. ([Pydantic][S21-1])

---

### Using `.raw_function` on untrusted input

```python
checked.raw_function(user_input)
```

Only bypass validation for trusted, already validated values.

---

### Hiding business logic in validators/config

```python
@validate_call
def place_order(user_id: int, amount: int) -> None:
    ...
```

`@validate_call` proves type/shape. It does not prove:

```text
user exists
actor is authorized
balance is sufficient
idempotency is satisfied
transaction is safe
```

Keep business checks in service logic.

---

## 21.30 Test matrix

### Coercion

```python
def test_validate_call_coerces_args():
    @validate_call
    def add(x: int, y: int) -> int:
        return x + y

    assert add("1", "2") == 3
```

### Constraint failure

```python
def test_validate_call_field_constraint():
    @validate_call
    def f(x: Annotated[int, Field(gt=0)]) -> int:
        return x

    with pytest.raises(ValidationError) as exc:
        f(0)

    assert exc.value.errors()[0]["type"] == "greater_than"
```

### Missing argument produces `ValidationError`

```python
def test_missing_arg_validation_error():
    @validate_call
    def f(x: int) -> int:
        return x

    with pytest.raises(ValidationError) as exc:
        f()

    assert exc.value.errors()[0]["type"] == "missing_argument"
```

### `*args`

```python
def test_var_args():
    @validate_call
    def f(*xs: int) -> tuple[int, ...]:
        return xs

    assert f("1", 2, 3.0) == (1, 2, 3)
```

### `**kwargs`

```python
def test_var_kwargs():
    @validate_call
    def f(**xs: int) -> dict[str, int]:
        return xs

    assert f(a="1", b=2) == {"a": 1, "b": 2}
```

### `Unpack[TypedDict]`

```python
def test_unpack_typed_dict_kwargs():
    class Point(TypedDict):
        x: int
        y: int

    @validate_call
    def f(**kwargs: Unpack[Point]) -> int:
        return kwargs["x"] + kwargs["y"]

    assert f(x="1", y=2) == 3
```

### Return validation

```python
def test_validate_return():
    @validate_call(validate_return=True)
    def f() -> int:
        return "1"

    assert f() == 1
```

### Config strict

```python
def test_validate_call_strict_config():
    @validate_call(config=ConfigDict(strict=True))
    def f(x: int) -> int:
        return x

    with pytest.raises(ValidationError):
        f("1")
```

### Raw function bypass

```python
def test_raw_function_bypasses_validation():
    @validate_call
    def f(x: int):
        return x

    assert f("1") == 1
    assert f.raw_function("1") == "1"  # type: ignore[attr-defined]
```

### Async function

```python
@pytest.mark.asyncio
async def test_async_validate_call():
    @validate_call
    async def f(x: int) -> int:
        return x

    assert await f("1") == 1
```

---

## 21.31 Agent checklist

```text
[ ] Use @validate_call for Python call-boundary validation.
[ ] Keep BaseModel for reusable DTOs, schemas, persistence, and serialization.
[ ] Keep TypeAdapter for standalone type validation.
[ ] Annotate every parameter that should be validated.
[ ] Remember unannotated parameters are Any.
[ ] Use Annotated[T, Field(...)] for constraints.
[ ] Use validate_return=True only where return validation is worth the overhead.
[ ] Use ConfigDict(strict=True) selectively for strict function boundaries.
[ ] Use arbitrary_types_allowed=True when function accepts opaque runtime handles.
[ ] Use keyword-only parameters for stable public API ergonomics.
[ ] Use Unpack[TypedDict] for structured **kwargs.
[ ] Catch ValidationError, not TypeError, for call-validation failures.
[ ] Use .raw_function only after prior validation or trusted construction.
[ ] Avoid @validate_call in hot loops.
[ ] Benchmark decorated vs raw calls for performance-sensitive paths.
[ ] Do not treat @validate_call as authorization, existence, or workflow validation.
[ ] Test positional, keyword, varargs, kwargs, return validation, and strict config paths.
```

---

## 21.32 Value case

```text
@validate_call value =
  runtime validation for ordinary Python function calls
  minimal boilerplate compared with explicit DTOs
  signature-native parsing/coercion
  support for positional, keyword-only, positional-only, *args, **kwargs
  support for Field constraints and aliases
  optional return-value validation
  custom ConfigDict policy
  async function compatibility
  raw_function escape hatch for trusted hot paths
```

Operationally: use `@validate_call` at **function boundaries** where a full model object would be unnecessary; use `BaseModel` when the data itself needs identity, persistence, serialization, or schema; use `TypeAdapter` when the boundary is a standalone type rather than a callable.

[S21-1]: https://pydantic.dev/docs/validation/latest/concepts/validation_decorator/ "Validation Decorator | Pydantic Docs"
[S21-2]: https://pydantic.dev/docs/validation/latest/api/pydantic/validate_call/ "Validate Call | Pydantic Docs"


# Pydantic Advanced — 22) Files, web/API requests, queues, databases, and boundary recipes

Style target: advanced, sectioned, agent-oriented technical reference. 

Pydantic’s strongest deployment pattern is **boundary-first validation**: validate untrusted input at ingress, convert to typed internal objects, keep business logic explicit, then serialize through outbound schemas at egress. Pydantic’s docs explicitly present models as API endpoint requirements, note that untrusted data can be passed into a model, and state that Pydantic guarantees the resulting model instance conforms to declared field types and constraints after parsing/validation. ([Pydantic Docs][S22-1])

---

## 22.0 Boundary-first architecture

```text
external source
  → inbound DTO validation
  → typed internal/domain object
  → service / business logic
  → outbound DTO validation/projection
  → serialization
  → external sink
```

Boundary classes:

```text
HTTP request body          → Inbound DTO
HTTP response body         → Outbound DTO
queue message              → Event DTO
database row / ORM object  → Persistence DTO / read model
CSV/JSON/YAML file row     → Import DTO
settings/config            → BaseSettings
domain object              → Domain model / dataclass / value object
```

Core rule:

```text
Validate at ingress.
Do not scatter raw dict access.
Do not run business logic on unvalidated external payloads.
Do not expose internal domain/ORM objects directly at egress.
Serialize through explicit outbound contracts.
```

---

## 22.1 Model layering

```text
Inbound DTO
  external input shape
  aliases / compatibility / extra forbid
  lax parsing if external source is stringly/JSON
  no persistence/business side effects

Domain model
  internal business object
  stricter types
  usually no external aliases
  may be frozen / value-object-like
  business invariants outside Pydantic when DB/auth/IO needed

Persistence model
  DB row / ORM projection shape
  from_attributes=True for ORM objects
  aliases for reserved DB/ORM names
  schema separate from table definition unless using integrated ORM library

Outbound schema
  external response shape
  serialization aliases
  excludes secrets/internal fields
  datetimes/decimals/bytes serialized deliberately

Event / message schema
  versioned external contract
  discriminated by type/version
  stable JSON serialization
  idempotency metadata where needed
```

---

## 22.2 Project layout

```text
src/myapp/
├─ schemas/
│  ├─ base.py              # ApiInputModel, ApiOutputModel, EventModel, DomainModel
│  ├─ inbound.py           # request DTOs
│  ├─ outbound.py          # response DTOs
│  ├─ events.py            # queue/message DTOs
│  ├─ persistence.py       # DB row / ORM projection models
│  ├─ imports.py           # CSV/JSON/YAML import models
│  └─ versions.py          # versioned contract aliases/registries
├─ domain/
│  ├─ models.py            # domain objects/value objects
│  └─ services.py          # business logic, DB/auth/IO orchestration
├─ repositories/
│  └─ users.py             # database IO
├─ adapters/
│  ├─ http.py              # request/response mapping
│  ├─ queues.py            # producer/consumer mapping
│  └─ files.py             # file parsing/import mapping
└─ settings.py             # BaseSettings only
```

Rule:

```text
schemas/:
  structural contracts only

domain/:
  semantic/business decisions

adapters/:
  source-specific parsing/serialization

repositories/:
  database IO

settings.py:
  deployment configuration
```

---

## 22.3 Base model policies

```python
from pydantic import BaseModel, ConfigDict
from pydantic.alias_generators import to_camel


class ApiInputModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        alias_generator=to_camel,
        validate_by_alias=True,
        validate_by_name=False,
        loc_by_alias=True,
        hide_input_in_errors=True,
    )


class ApiOutputModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        alias_generator=to_camel,
        serialize_by_alias=True,
        from_attributes=True,
    )


class EventModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        validate_by_alias=True,
        serialize_by_alias=True,
        hide_input_in_errors=True,
    )


class DomainModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        strict=True,
        frozen=True,
    )
```

Boundary policy:

```text
Input:
  reject unknown keys
  use external aliases
  hide raw inputs in errors

Output:
  serialize aliases
  support ORM/domain object projection if needed
  exclude internal fields explicitly

Events:
  forbid unknown message fields
  include schema version and event type

Domain:
  strict, frozen where appropriate
  no external naming policy
```

---

## 22.4 Validating API request payloads

### Raw JSON body

```python
from pydantic import EmailStr, Field, ValidationError


class CreateUserRequest(ApiInputModel):
    email: EmailStr
    age: int | None = Field(default=None, ge=13)
    marketing_opt_in: bool = False


def parse_create_user_body(raw_body: bytes) -> CreateUserRequest:
    return CreateUserRequest.model_validate_json(raw_body)
```

Use `model_validate_json(...)` for raw JSON bytes/strings. Pydantic’s docs list `model_validate_json()` as the model method for validating JSON data, while `model_validate()` validates Python objects. ([Pydantic Docs][S22-1])

### Already decoded request object

```python
def parse_create_user_dict(data: dict) -> CreateUserRequest:
    return CreateUserRequest.model_validate(data)
```

### Query/header/form data

```python
class SearchQuery(ApiInputModel):
    q: str = Field(min_length=1)
    limit: int = Field(default=25, ge=1, le=100)
    include_archived: bool = False


def parse_query_params(params: dict[str, str]) -> SearchQuery:
    return SearchQuery.model_validate_strings(params)
```

Policy:

```text
raw JSON body:
  model_validate_json

decoded framework dict:
  model_validate

query/header/form/CLI-style string map:
  model_validate_strings

top-level JSON array:
  TypeAdapter(list[T]).validate_json
```

---

## 22.5 HTTP client response validation

```python
import httpx
from pydantic import BaseModel, EmailStr, TypeAdapter


class RemoteUser(BaseModel):
    id: int
    name: str
    email: EmailStr


def fetch_user(client: httpx.Client, user_id: int) -> RemoteUser:
    response = client.get(f"https://api.example.com/users/{user_id}")
    response.raise_for_status()
    return RemoteUser.model_validate(response.json())


USERS = TypeAdapter(list[RemoteUser])


def fetch_users(client: httpx.Client) -> list[RemoteUser]:
    response = client.get("https://api.example.com/users")
    response.raise_for_status()
    return USERS.validate_python(response.json())
```

The official web/API example uses `httpx`, calls `response.raise_for_status()`, validates a single response with `User.model_validate(response.json())`, and validates a list response with `TypeAdapter(list[User]).validate_python(response.json())`. ([Pydantic Docs][S22-2])

Policy:

```text
Validate remote responses:
  before trusting them
  before persisting them
  before passing to business logic

Do not:
  trust third-party JSON because status code is 200
  pass response.json() around raw
```

---

## 22.6 Validating response payloads

### Domain → outbound DTO

```python
from datetime import datetime
from uuid import UUID


class UserDomain(DomainModel):
    id: UUID
    email: str
    created_at: datetime
    password_hash: str


class UserResponse(ApiOutputModel):
    id: UUID
    email: str
    created_at: datetime


def user_to_response(user: UserDomain) -> dict:
    dto = UserResponse.model_validate(user)
    return dto.model_dump(mode="json", by_alias=True)
```

`model_dump()` recursively converts Pydantic models into dictionaries in Python mode, and `model_dump(mode="json")` ensures JSON-compatible values; `model_dump_json()` serializes directly to a JSON string. ([Pydantic Docs][S22-3])

### Response JSON string

```python
def user_to_json(user: UserDomain) -> str:
    return UserResponse.model_validate(user).model_dump_json(by_alias=True)
```

Policy:

```text
Do not serialize domain objects directly.
Do not return ORM objects directly.
Do not include secrets in output DTOs.
Use dedicated response schemas.
```

---

## 22.7 API error mapping

```python
from pydantic import ValidationError


def validation_problem(exc: ValidationError) -> dict:
    return {
        "type": "https://example.com/problems/validation-error",
        "title": "Invalid request payload",
        "status": 422,
        "errors": [
            {
                "code": err["type"],
                "path": list(err["loc"]),
                "message": err["msg"],
                "context": err.get("ctx", {}),
            }
            for err in exc.errors(include_input=False, include_url=False)
        ],
    }
```

Policy:

```text
Public API:
  include type/code
  include loc/path
  include message if acceptable
  omit raw input
  omit Pydantic docs URL unless developer-facing

Internal logs:
  include error_count, types, locs
  omit raw input for secret-bearing payloads
```

---

## 22.8 Validating queue messages

### Producer

```python
from typing import Literal
from uuid import UUID


class UserCreated(EventModel):
    event_type: Literal["user.created"]
    version: Literal["v1"] = "v1"
    event_id: UUID
    user_id: UUID
    email: str


def serialize_event(event: UserCreated) -> bytes:
    return event.model_dump_json().encode()
```

### Consumer

```python
def parse_user_created(raw: bytes) -> UserCreated:
    return UserCreated.model_validate_json(raw)
```

The official queue examples serialize Pydantic models with `model_dump_json()` before pushing to a queue and validate popped queue data with `model_validate_json(...)`. ([Pydantic Docs][S22-4])

### Polymorphic event bus

```python
from typing import Annotated, Literal
from pydantic import Field, TypeAdapter


class UserDeleted(EventModel):
    event_type: Literal["user.deleted"]
    version: Literal["v1"] = "v1"
    event_id: UUID
    user_id: UUID


Event = Annotated[
    UserCreated | UserDeleted,
    Field(discriminator="event_type"),
]

EVENT_ADAPTER = TypeAdapter(Event)


def parse_event(raw: bytes) -> Event:
    return EVENT_ADAPTER.validate_json(raw)


def dump_event(event: Event) -> bytes:
    return EVENT_ADAPTER.dump_json(event)
```

Policy:

```text
Queue producer:
  validate before enqueue
  serialize from DTO
  include event_type
  include version
  include idempotency/event_id

Queue consumer:
  validate immediately after dequeue
  reject/dead-letter invalid payloads
  do not process raw dicts

Event polymorphism:
  discriminated union
  stable Literal tags
  version field or versioned tags
```

---

## 22.9 Queue dead-letter recipe

```python
from pydantic import ValidationError


def consume(raw: bytes) -> None:
    try:
        event = EVENT_ADAPTER.validate_json(raw)
    except ValidationError as exc:
        dead_letter(
            raw=raw,
            reason="validation_error",
            errors=exc.errors(include_input=False, include_url=False),
        )
        return

    handle_event(event)
```

Policy:

```text
Dead-letter payload:
  original raw message if allowed by privacy policy
  normalized validation errors
  schema version expected
  consumer version
  timestamp
  queue metadata
```

---

## 22.10 Validating database rows and ORM objects

### SQLAlchemy-style ORM projection

```python
from pydantic import BaseModel, ConfigDict, Field


class DbRowDTO(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    metadata: dict[str, str] = Field(alias="metadata_")


def from_orm_row(row: object) -> DbRowDTO:
    return DbRowDTO.model_validate(row)
```

Pydantic’s database example shows `ConfigDict(from_attributes=True)` for validating SQLAlchemy ORM objects, uses an alias to avoid a reserved SQLAlchemy field name, and notes that aliases have priority over field names for population. ([Pydantic Docs][S22-5])

### Persistence DTO vs domain object

```python
from datetime import datetime
from uuid import UUID


class UserRowDTO(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: UUID
    email: str
    password_hash: str
    created_at: datetime


class User(DomainModel):
    id: UUID
    email: str
    created_at: datetime


def row_to_domain(row: object) -> User:
    dto = UserRowDTO.model_validate(row)
    return User.model_validate(dto.model_dump(exclude={"password_hash"}))
```

Policy:

```text
DB read boundary:
  validate ORM/row object into persistence DTO

Domain boundary:
  convert persistence DTO to domain model

Response boundary:
  convert domain model to outbound DTO

Do not:
  expose ORM rows directly to API
  expose persistence-only fields by default
  let DB column naming leak into domain model unless intentional
```

---

## 22.11 Database write validation

```python
class CreateUserCommand(DomainModel):
    email: str
    password_hash: str


class UserInsertRow(BaseModel):
    email: str
    password_hash: str


def insert_user(cmd: CreateUserCommand, repo) -> UUID:
    row = UserInsertRow.model_validate(cmd.model_dump())
    return repo.insert_user(row.model_dump())
```

Policy:

```text
Validate before write:
  shape
  local constraints
  nullable/missing semantics

Keep outside Pydantic:
  transaction handling
  uniqueness checks
  foreign-key existence
  authorization
  retry/idempotency
```

---

## 22.12 Validating JSON files

### Single object

```python
from pathlib import Path
from pydantic import BaseModel, EmailStr, PositiveInt, ValidationError


class Person(BaseModel):
    name: str
    age: PositiveInt
    email: EmailStr


def load_person_json(path: Path) -> Person:
    return Person.model_validate_json(path.read_text())
```

The official file-data example validates a `.json` file by reading text and passing it to `Person.model_validate_json(...)`; invalid files raise one `ValidationError` containing all discovered issues. ([Pydantic Docs][S22-6])

### Top-level list

```python
from pydantic import TypeAdapter

PEOPLE = TypeAdapter(list[Person])


def load_people_json(path: Path) -> list[Person]:
    return PEOPLE.validate_json(path.read_text())
```

The same file example validates a top-level list with `TypeAdapter(list[Person]).validate_json(...)`. ([Pydantic Docs][S22-6])

---

## 22.13 Validating JSON Lines files

```python
def load_people_jsonl(path: Path) -> tuple[list[Person], list[dict]]:
    ok: list[Person] = []
    bad: list[dict] = []

    for line_number, line in enumerate(path.read_text().splitlines(), start=1):
        if not line.strip():
            continue

        try:
            ok.append(Person.model_validate_json(line))
        except ValidationError as exc:
            bad.append(
                {
                    "line": line_number,
                    "errors": exc.errors(include_input=False, include_url=False),
                }
            )

    return ok, bad
```

The official JSON Lines example reads lines from a `.jsonl` file and validates each line with `Person.model_validate_json(line)`. ([Pydantic Docs][S22-6])

Policy:

```text
JSONL:
  validate per line
  attach line number to errors
  decide fail-fast vs collect-all
  avoid loading huge files entirely when streaming is needed
```

---

## 22.14 Validating CSV files

```python
import csv
from pathlib import Path


def load_people_csv(path: Path) -> tuple[list[Person], list[dict]]:
    ok: list[Person] = []
    bad: list[dict] = []

    with path.open(newline="") as f:
        reader = csv.DictReader(f)
        for row_number, row in enumerate(reader, start=2):
            try:
                ok.append(Person.model_validate(row))
            except ValidationError as exc:
                bad.append(
                    {
                        "row": row_number,
                        "errors": exc.errors(include_input=False, include_url=False),
                    }
                )

    return ok, bad
```

Pydantic’s file-data example uses Python’s `csv.DictReader` and validates each row with `Person.model_validate(row)`. ([Pydantic Docs][S22-6])

CSV policy:

```text
CSV rows are stringly dicts.
Use model_validate(row) or model_validate_strings(row).
Attach row number to errors.
Normalize headers before validation if needed.
Use aliases for column names.
```

---

## 22.15 CSV with aliases

```python
from pydantic import AliasChoices, Field


class PersonImport(BaseModel):
    name: str = Field(validation_alias=AliasChoices("name", "Name", "full_name"))
    age: PositiveInt = Field(validation_alias=AliasChoices("age", "Age"))
    email: EmailStr = Field(validation_alias=AliasChoices("email", "Email Address"))


def parse_person_row(row: dict[str, str]) -> PersonImport:
    return PersonImport.model_validate(row)
```

Policy:

```text
Column compatibility:
  AliasChoices for legacy headers
  canonical first
  log deprecated headers outside Pydantic if needed

Header normalization:
  strip whitespace
  map known variants
  reject unknown required headers early
```

---

## 22.16 Validating TOML files

```python
import tomllib
from pathlib import Path


def load_person_toml(path: Path) -> Person:
    with path.open("rb") as f:
        data = tomllib.load(f)
    return Person.model_validate(data)
```

The official file-data example uses `tomllib.load(...)` for TOML, then validates the resulting dictionary with `Person.model_validate(data)`. ([Pydantic Docs][S22-6])

Policy:

```text
TOML config:
  if deployment settings, prefer pydantic-settings source support
  if domain data file, load with tomllib then model_validate
```

The file-data docs explicitly note that if these formats are being used for configuration/settings, `pydantic-settings` may be more appropriate because it has built-in support for parsing settings data. ([Pydantic Docs][S22-6])

---

## 22.17 Validating YAML files

```python
from pathlib import Path

import yaml


def load_person_yaml(path: Path) -> Person:
    with path.open() as f:
        data = yaml.safe_load(f)
    return Person.model_validate(data)
```

The official file-data example loads YAML with `yaml.safe_load(...)` and validates the resulting data with `Person.model_validate(data)`. ([Pydantic Docs][S22-6])

YAML policy:

```text
Use safe_load, not unsafe loaders.
Validate after parsing.
Do not treat YAML as trusted code/config.
Avoid YAML for untrusted public uploads unless parser/config is tightly controlled.
```

---

## 22.18 Validating INI/XML and other file data

```python
import configparser
import xml.etree.ElementTree as ET
from pathlib import Path


def load_person_ini(path: Path) -> Person:
    config = configparser.ConfigParser()
    config.read(path)
    return Person.model_validate(config["PERSON"])


def load_person_xml(path: Path) -> Person:
    root = ET.parse(path).getroot()
    data = {child.tag: child.text for child in root}
    return Person.model_validate(data)
```

Pydantic’s file-data examples include INI via `configparser` and XML via `xml.etree.ElementTree`, converting parsed structures into dictionaries and then using `model_validate(...)`. ([Pydantic Docs][S22-6])

Agent rule:

```text
Pydantic validates parsed data.
It does not replace file parser security.
Choose safe parser configuration first.
Then validate structure with Pydantic.
```

---

## 22.19 Batch import pattern

```python
from dataclasses import dataclass


@dataclass(frozen=True)
class ImportResult[T]:
    ok: list[T]
    errors: list[dict]


def validate_rows[T](
    rows: list[dict],
    model: type[T],
) -> ImportResult[T]:
    ok: list[T] = []
    errors: list[dict] = []

    for index, row in enumerate(rows):
        try:
            ok.append(model.model_validate(row))  # type: ignore[attr-defined]
        except ValidationError as exc:
            errors.append(
                {
                    "index": index,
                    "errors": exc.errors(include_input=False, include_url=False),
                }
            )

    return ImportResult(ok=ok, errors=errors)
```

Policy:

```text
Batch import:
  decide fail-fast vs collect-all
  preserve source position: file, line, row, message offset
  normalize error format
  avoid raw input in logs when sensitive
  validate all rows before mutating database if atomic import is required
```

---

## 22.20 Boundary-first service recipe

```python
class CreateUserRequest(ApiInputModel):
    email: EmailStr
    age: int | None = Field(default=None, ge=13)


class CreateUserCommand(DomainModel):
    email: EmailStr
    age: int | None = None


class UserRecord(BaseModel):
    id: UUID
    email: str
    age: int | None
    password_hash: str
    created_at: datetime


class UserResponse(ApiOutputModel):
    id: UUID
    email: str
    age: int | None
    created_at: datetime


def endpoint(raw_body: bytes, service) -> dict:
    req = CreateUserRequest.model_validate_json(raw_body)
    cmd = CreateUserCommand.model_validate(req.model_dump())
    user = service.create_user(cmd)
    return UserResponse.model_validate(user).model_dump(mode="json", by_alias=True)
```

Flow:

```text
raw_body:
  untrusted JSON

CreateUserRequest:
  inbound API contract

CreateUserCommand:
  internal typed command

service.create_user:
  business logic / DB / auth

UserResponse:
  outbound public contract
```

---

## 22.21 Versioning external contracts

### Event versioning

```python
class UserCreatedV1(EventModel):
    event_type: Literal["user.created"]
    version: Literal["v1"] = "v1"
    user_id: UUID
    email: str


class UserCreatedV2(EventModel):
    event_type: Literal["user.created"]
    version: Literal["v2"] = "v2"
    user_id: UUID
    email: str
    display_name: str | None = None


UserCreatedEvent = Annotated[
    UserCreatedV1 | UserCreatedV2,
    Field(discriminator="version"),
]
```

### API versioning

```python
class CreateUserV1(ApiInputModel):
    email: EmailStr


class CreateUserV2(ApiInputModel):
    email: EmailStr
    age: int | None = Field(default=None, ge=13)
```

Policy:

```text
External contract versioning:
  never silently change required fields
  never silently rename wire keys
  add optional fields first
  deprecate with overlap window
  use AliasChoices for inbound compatibility
  emit one canonical outbound shape
  version queue/event payloads explicitly
  snapshot public JSON Schema
```

Pydantic can generate JSON Schema from `BaseModel.model_json_schema()` and `TypeAdapter.json_schema()`, and its generated schemas are jsonable dictionaries suitable for contract/docs/tooling workflows. ([Pydantic Docs][S22-7])

---

## 22.22 JSON Schema contract snapshots

```python
import json


def schema_snapshot(model: type[BaseModel]) -> str:
    return json.dumps(
        model.model_json_schema(
            mode="validation",
            by_alias=True,
            ref_template="#/components/schemas/{model}",
        ),
        indent=2,
        sort_keys=True,
    )
```

Snapshot targets:

```text
public request DTOs
public response DTOs
event payloads
file import schemas
LLM/tool input schemas
SDK-exposed models
```

Avoid snapshotting:

```text
private domain models
temporary adapter models
internal persistence DTOs unless DB contract needs it
```

---

## 22.23 Inbound vs outbound schemas

```python
class UserIn(ApiInputModel):
    email: EmailStr
    password: str = Field(min_length=12)


class UserOut(ApiOutputModel):
    id: UUID
    email: EmailStr
    created_at: datetime
```

Rule:

```text
Inbound:
  may contain secrets
  may accept legacy aliases
  may be lax/stringly
  should not be returned

Outbound:
  no password/secret fields
  canonical aliases
  serialization-safe types
  may differ from inbound
```

Do not reuse one model for both if:

```text
input has password
output has id/created_at
input accepts legacy fields
output must emit canonical fields
domain has internal-only fields
serialization differs from validation
```

---

## 22.24 Persistence mapping patterns

### Explicit mapping

```python
def domain_to_row(user: UserDomain, password_hash: str) -> UserInsertRow:
    return UserInsertRow(
        email=user.email,
        password_hash=password_hash,
    )


def row_to_response(row: object) -> UserResponse:
    return UserResponse.model_validate(row)
```

### Avoid implicit full dump

```python
# risky: may include internal fields
repo.insert(user.model_dump())
```

Preferred:

```python
repo.insert(UserInsertRow.model_validate(user.model_dump()).model_dump())
```

Policy:

```text
DB writes:
  use insert/update DTOs
  exclude internal/computed fields
  validate nullable/missing semantics
  avoid dumping full domain object blindly
```

---

## 22.25 Security and secrets at boundaries

```python
from pydantic import SecretStr


class LoginRequest(ApiInputModel):
    email: EmailStr
    password: SecretStr


class LoginResponse(ApiOutputModel):
    access_token: SecretStr
    token_type: Literal["bearer"] = "bearer"
```

Policy:

```text
Secret input:
  SecretStr / SecretBytes
  hide_input_in_errors=True
  errors(include_input=False)

Secret output:
  never expose default SecretStr masked value as actual token response unless intended
  use dedicated internal serializer only for trusted token issuing path
  do not log model_dump_json containing secrets
```

---

## 22.26 Boundary recipe matrix

```text
Boundary                      Validate with                         Serialize with
------------------------------------------------------------------------------------------------
HTTP request JSON object       Model.model_validate_json(raw)         n/a
HTTP request decoded dict       Model.model_validate(data)            n/a
HTTP query params               Model.model_validate_strings(params)  n/a
HTTP response DTO               Model.model_validate(obj)             model_dump(mode="json", by_alias=True)
HTTP response JSON              Model.model_validate(obj)             model_dump_json(by_alias=True)
HTTP client response object     Model.model_validate(response.json()) n/a
HTTP client list response       TypeAdapter(list[T]).validate_python  n/a
Queue producer                  EventModel(...) / model_validate      model_dump_json / TypeAdapter.dump_json
Queue consumer JSON bytes       Model.model_validate_json(raw)        n/a
Queue polymorphic consumer      TypeAdapter(DiscriminatedUnion).validate_json(raw) n/a
DB ORM read                     Model.model_validate(row), from_attributes=True    model_dump as needed
DB write                        InsertDTO.model_validate(data)        model_dump
JSON file object                Model.model_validate_json(text)       n/a
JSON file list                  TypeAdapter(list[T]).validate_json(text) n/a
JSONL file                      per-line model_validate_json          n/a
CSV row                         Model.model_validate(row) / strings   n/a
TOML file                       tomllib.load → model_validate         n/a
YAML file                       safe_load → model_validate            n/a
Settings/config                 BaseSettings                          n/a
```

---

## 22.27 Anti-patterns

### Raw dicts past ingress

```python
def endpoint(raw_body: bytes):
    data = json.loads(raw_body)
    service.create_user(data)
```

Preferred:

```python
req = CreateUserRequest.model_validate_json(raw_body)
service.create_user(CreateUserCommand.model_validate(req.model_dump()))
```

---

### One model for request, domain, DB, and response

```python
class User(BaseModel):
    email: str
    password_hash: str
    created_at: datetime
```

Problem:

```text
password_hash leaks risk
DB concerns leak into API
response and request semantics diverge
versioning becomes hard
```

Preferred:

```text
CreateUserRequest
CreateUserCommand
UserRowDTO
UserDomain
UserResponse
```

---

### Trusting third-party API JSON

```python
return response.json()["email"]
```

Preferred:

```python
remote_user = RemoteUser.model_validate(response.json())
return remote_user.email
```

---

### Queue consumer processes raw JSON

```python
data = json.loads(raw)
handle(data)
```

Preferred:

```python
event = EVENT_ADAPTER.validate_json(raw)
handle(event)
```

---

### ORM object serialized directly

```python
return orm_user.__dict__
```

Preferred:

```python
return UserResponse.model_validate(orm_user).model_dump(mode="json", by_alias=True)
```

---

### Unsafe YAML load

```python
data = yaml.load(f)
```

Preferred:

```python
data = yaml.safe_load(f)
model = ImportModel.model_validate(data)
```

---

### Import-time file/settings validation in libraries

```python
CONFIG = Settings()
IMPORT_DATA = ImportModel.model_validate_json(Path("data.json").read_text())
```

Preferred:

```python
def load_settings() -> Settings: ...
def load_import_data(path: Path) -> ImportModel: ...
```

---

## 22.28 Test matrix

### API request validation

```python
def test_create_user_request_json():
    raw = b'{"email":"ada@example.com","age":"13","marketingOptIn":"true"}'
    req = CreateUserRequest.model_validate_json(raw)

    assert req.email == "ada@example.com"
    assert req.age == 13
    assert req.marketing_opt_in is True
```

### API response excludes secrets

```python
def test_user_response_excludes_password_hash():
    user = UserDomain(
        id=UUID("00000000-0000-0000-0000-000000000001"),
        email="ada@example.com",
        created_at=datetime(2026, 1, 1),
        password_hash="hash",
    )

    payload = UserResponse.model_validate(user).model_dump(mode="json", by_alias=True)

    assert "passwordHash" not in payload
    assert "password_hash" not in payload
```

### Queue event round trip

```python
def test_event_round_trip():
    event = UserCreated(
        event_type="user.created",
        event_id=UUID("00000000-0000-0000-0000-000000000001"),
        user_id=UUID("00000000-0000-0000-0000-000000000002"),
        email="ada@example.com",
    )

    raw = event.model_dump_json().encode()
    parsed = UserCreated.model_validate_json(raw)

    assert parsed == event
```

### Polymorphic event dispatch

```python
def test_event_discriminator():
    raw = b'{"event_type":"user.deleted","version":"v1","event_id":"00000000-0000-0000-0000-000000000001","user_id":"00000000-0000-0000-0000-000000000002"}'

    event = EVENT_ADAPTER.validate_json(raw)

    assert isinstance(event, UserDeleted)
```

### ORM projection

```python
def test_orm_projection():
    class Row:
        id = 1
        metadata_ = {"key": "val"}

    model = DbRowDTO.model_validate(Row())

    assert model.metadata == {"key": "val"}
```

### CSV row validation

```python
def test_csv_row_validation():
    row = {"name": "Ada", "age": "30", "email": "ada@example.com"}

    person = Person.model_validate(row)

    assert person.age == 30
```

### JSON file list adapter

```python
def test_json_list_adapter(tmp_path):
    path = tmp_path / "people.json"
    path.write_text('[{"name":"Ada","age":30,"email":"ada@example.com"}]')

    people = PEOPLE.validate_json(path.read_text())

    assert len(people) == 1
    assert people[0].name == "Ada"
```

### Invalid file errors include source location

```python
def test_jsonl_error_line_number(tmp_path):
    path = tmp_path / "people.jsonl"
    path.write_text('{"name":"Ada","age":30,"email":"ada@example.com"}\n{"age":-1}\n')

    ok, bad = load_people_jsonl(path)

    assert len(ok) == 1
    assert bad[0]["line"] == 2
    assert {err["type"] for err in bad[0]["errors"]} >= {"missing", "greater_than"}
```

### Schema snapshot

```python
def test_public_schema_snapshot(snapshot):
    schema = UserCreated.model_json_schema(
        mode="validation",
        by_alias=True,
        ref_template="#/components/schemas/{model}",
    )

    snapshot.assert_match(
        json.dumps(schema, indent=2, sort_keys=True),
        "UserCreated.schema.json",
    )
```

---

## 22.29 Deployment checklist

```text
[ ] Validate raw HTTP JSON with model_validate_json.
[ ] Validate decoded HTTP client responses before use.
[ ] Use TypeAdapter(list[T]) for top-level lists.
[ ] Validate queue messages immediately after dequeue.
[ ] Serialize queue messages only from event DTOs.
[ ] Include event_type and version in external messages.
[ ] Use discriminated unions for polymorphic messages.
[ ] Use from_attributes=True for ORM/domain projections.
[ ] Keep persistence DTOs separate from API DTOs.
[ ] Validate DB rows before exposing or transforming.
[ ] Validate CSV rows with row-numbered errors.
[ ] Validate JSONL line-by-line with line-numbered errors.
[ ] Use safe YAML loading before Pydantic validation.
[ ] Use pydantic-settings for deployment config instead of ad hoc file parsing.
[ ] Keep secrets out of response DTOs.
[ ] Use SecretStr/SecretBytes and include_input=False for sensitive validation errors.
[ ] Snapshot JSON Schema for public contracts.
[ ] Version external APIs/events/files explicitly.
[ ] Use AliasChoices for inbound compatibility windows.
[ ] Emit one canonical outbound shape.
[ ] Avoid import-time settings/file validation in reusable libraries.
```

---

## 22.30 Value case

```text
Boundary recipe value =
  raw external data converted once into typed objects
  service logic receives stable internal contracts
  outbound data shaped through explicit public schemas
  database/ORM details isolated from API contracts
  queue messages versioned and validated at both producer/consumer edges
  file imports produce structured row/line errors
  JSON Schema snapshots protect external contracts
  secret-bearing inputs can be redacted consistently
  legacy aliases and versioned DTOs support safe migration
```

Operationally: put Pydantic at every trust boundary, but keep business truth outside Pydantic. Use DTO layers to prevent API, queue, file, database, and domain concerns from collapsing into one brittle model.

[S22-1]: https://docs.pydantic.dev/latest/concepts/models/ "Models | Pydantic Docs"
[S22-2]: https://docs.pydantic.dev/latest/examples/requests/ "Web and API Requests | Pydantic Docs"
[S22-3]: https://docs.pydantic.dev/latest/concepts/serialization/ "Serialization | Pydantic Docs"
[S22-4]: https://docs.pydantic.dev/latest/examples/queues/ "Queues | Pydantic Docs"
[S22-5]: https://docs.pydantic.dev/latest/examples/orms/ "Databases | Pydantic Docs"
[S22-6]: https://docs.pydantic.dev/latest/examples/files/ "Validating File Data | Pydantic Docs"
[S22-7]: https://docs.pydantic.dev/latest/concepts/json_schema/ "JSON Schema | Pydantic Docs"



# Part IV — Production hardening and source-verified capability indexes

# Pydantic Advanced — 23) Security, secrets, and sensitive-data handling

## 23.0 Security mental model

Pydantic is a **validation and serialization layer**, not an authorization, sandboxing, or taint-tracking system. Its security value is strongest when it constrains external shape, rejects unexpected data, prevents accidental serialization of sensitive fields, and centralizes secret/config ingestion policy.

```text
untrusted bytes / mappings
  → parser + Pydantic schema
  → normalized typed object
  → application authorization / business checks
  → controlled serialization
```

Use Pydantic to narrow the shape of data; use the service/domain layer to decide whether the data is permitted or true.

## 23.1 Defensive model defaults

A conservative public-input baseline:

```python
from pydantic import BaseModel, ConfigDict

class PublicInput(BaseModel):
    model_config = ConfigDict(
        extra='forbid',
        hide_input_in_errors=True,
        validation_error_cause=False,
    )
```

High-integrity internal boundary:

```python
class DomainInput(BaseModel):
    model_config = ConfigDict(
        extra='forbid',
        strict=True,
        revalidate_instances='always',
        validate_assignment=True,
    )
```

Do not make `strict=True` a universal default for environment variables, query strings, forms, or CLI values: those boundaries are intrinsically stringly typed.

## 23.2 Secrets

Use `SecretStr`, `SecretBytes`, or generic `Secret[T]` for values whose representation should be masked. Masking is not encryption and does not make the underlying value inaccessible to application code.

```python
from pydantic import BaseModel, SecretStr

class Credentials(BaseModel):
    username: str
    password: SecretStr

c = Credentials(username='svc', password='hunter2')
assert 'hunter2' not in repr(c)
raw = c.password.get_secret_value()
```

Combine controls deliberately:

| Need | Mechanism |
|---|---|
| hide from `repr` | secret type and/or `Field(repr=False)` |
| omit from serialized model | `Field(exclude=True)` or serializer policy |
| avoid raw inputs in validation errors | `ConfigDict(hide_input_in_errors=True)` |
| source secret values from files/cloud secret managers | `pydantic-settings` |
| encrypt at rest/in transit | external secret/KMS/TLS mechanism; not Pydantic |

## 23.3 Subclass serialization is a security boundary

Pydantic V2 defaults to serializing model-like fields according to the **annotated type**, even when the runtime object is a subclass. This intentionally avoids surprising leakage of newly added subclass fields. Pydantic 2.13 adds explicit polymorphic serialization, which can expose subclass fields and therefore must be treated as a conscious data-exposure decision. [S23-1]

```python
class User(BaseModel):
    name: str

class UserLogin(User):
    password: str

class Envelope(BaseModel):
    user: User

obj = Envelope(user=UserLogin(name='a', password='secret'))
assert obj.model_dump() == {'user': {'name': 'a'}}
# Enabling polymorphic serialization may include password.
```

**Agent rule:** never enable `polymorphic_serialization` or `serialize_as_any` merely to “make serialization work”; audit whether subclass-only fields include secrets or internal state.

## 23.4 Error privacy

Validation errors can include input values. At public boundaries:

```python
class Input(BaseModel):
    model_config = ConfigDict(hide_input_in_errors=True)
```

Also sanitize logs. `hide_input_in_errors` affects rendered validation errors; it does not automatically redact every application log, trace, serializer, or custom exception.

## 23.5 Settings-secret advisory

`pydantic-settings` versions `>=2.12.0,<2.14.2` had a moderate advisory affecting `NestedSecretsSettingsSource` with nested subdirectories: symlinks under `secrets_dir` could escape the configured tree and bypass the size cap. The patched floor is **2.14.2**; this reference’s current companion target is **2.15.0**. [S23-2]

Production rule:

```text
NestedSecretsSettingsSource in production:
  [ ] pydantic-settings >= 2.14.2
  [ ] secrets directory owned by the application/deployment system
  [ ] secrets_dir_max_size configured where appropriate
  [ ] no attacker-writable secret directory entries
  [ ] cloud/Kubernetes/Docker secret mounts treated as privileged inputs
```

## 23.6 What Pydantic does not secure

* SQL/NoSQL injection after validated strings are interpolated unsafely.
* Path traversal if a validated `Path` is later joined or opened without policy checks.
* SSRF merely because a value is an `HttpUrl`.
* Authorization, tenancy, ownership, or row-level security.
* Deserialization of unsafe Python pickle objects.
* Secret confidentiality after application code calls `get_secret_value()`.
* Denial of service from application-specific pathological validators or huge accepted payloads unless you add explicit limits.

## 23.7 Security anti-patterns

* `extra='ignore'` on a privileged command when unknown keys should be rejected.
* `arbitrary_types_allowed=True` as a broad workaround on external API models.
* Network/database calls from field validators.
* Enabling polymorphic/any serialization globally without a data-leak audit.
* Logging `exc.errors()` blindly when inputs contain credentials or PII.
* Treating `SecretStr` as encrypted storage.
* Accepting an `HttpUrl` and assuming the destination is safe to fetch.
* Using `model_construct()` on untrusted data.

## 23.8 Security test gate

```text
[ ] unexpected fields rejected where contract is closed
[ ] secrets masked in repr
[ ] secrets excluded from wire output where required
[ ] hidden inputs confirmed in error rendering
[ ] subclass secret field cannot leak under default serialization
[ ] polymorphic serialization has an explicit allow/test case
[ ] settings secret source meets patched version floor
[ ] strict/coercion behavior tested for security-sensitive identifiers
[ ] model_construct absent from untrusted ingress paths
```

[S23-1]: https://docs.pydantic.dev/latest/concepts/serialization/ "Serialization and subclass behavior"
[S23-2]: https://github.com/pydantic/pydantic-settings/security/advisories/GHSA-4xgf-cpjx-pc3j "Nested secrets symlink advisory"

---

# Pydantic Advanced — 24) Ecosystem integrations and developer tooling

## 24.0 Ecosystem boundary

Pydantic’s stable docs explicitly maintain integration guidance for LLMs, Logfire, mypy, Pyrefly, PyCharm, Hypothesis, VS Code, datamodel-code-generator, devtools, Rich, linting/documentation tooling, and AWS Lambda. Treat these as **integration surfaces around Pydantic**, not core validation semantics. [S24-1]

## 24.1 Integration map

| Integration | Pydantic value | Recommended boundary |
|---|---|---|
| FastAPI / Django Ninja / API frameworks | request/response validation + OpenAPI | expose explicit API models, not domain internals |
| SQLModel / ORM workflows | shared type-driven DTOs | separate persistence and wire contracts when behavior diverges |
| Pydantic AI / LLM structured output | runtime schema + JSON Schema | validate generated output; separately authorize tool/action semantics |
| Logfire | validation observability | instrument without logging secrets/raw inputs indiscriminately |
| mypy | static checking + optional plugin | CI against supported Pydantic minor range |
| Pyrefly | static analysis integration | useful for large typed codebases; keep runtime tests authoritative |
| PyCharm / VS Code | editor support | ordinary type hints + plugin/tooling where helpful |
| Hypothesis | property-based tests | derive/generated test data plus explicit edge-case strategies |
| datamodel-code-generator | schema → models | generated code is a starting point; review config/validators/aliases |
| devtools / Rich | inspection and display | debugging only; not wire serialization |
| AWS Lambda | serverless deployment | manage cold-start/model-build cost |

## 24.2 LLM structured output

Pydantic is well suited to LLM boundaries because one annotation graph can drive validation and JSON Schema. The high-value pattern is:

```text
Pydantic model / TypeAdapter
  → JSON Schema supplied to model/tool API
  → model output
  → model_validate_json / TypeAdapter.validate_json
  → application semantic / authorization checks
```

Never equate “Pydantic-valid” with “factually correct” or “authorized to execute.”

## 24.3 Static analysis

Pydantic deliberately uses normal Python annotations so static analysis remains useful. Prefer `Annotated[T, ...]` metadata over constructor-style constrained types (`constr`, `conint`, etc.) in new reusable APIs because the `Annotated` form composes better with type checkers; current type docs explicitly discourage `constr` in favor of `StringConstraints` and plan deprecation in Pydantic 3. [S24-2]

Recommended CI:

```text
ruff / lint
mypy or pyright / pyrefly
pytest runtime validation tests
JSON Schema contract tests where public
oldest-supported + newest-compatible Pydantic for public libraries
```

## 24.4 Framework-coupling rule

If a framework consumes Pydantic models, distinguish:

```text
Pydantic semantic contract
  field types / defaults / aliases / validators / serialization / JSON Schema

Framework contract
  HTTP status / dependency injection / route metadata / ORM session / response encoding
```

Do not place framework-only side effects into model validators merely because the framework happens to call Pydantic.

## 24.5 Agent checklist

```text
[ ] Identify whether feature belongs to Pydantic or surrounding framework.
[ ] Keep public models importable without booting DB/network clients.
[ ] Validate LLM output and separately validate action semantics.
[ ] Use static tooling, but keep runtime validation tests authoritative.
[ ] Treat generated models as reviewed source, not infallible output.
[ ] Keep display/debug integrations out of serialization contracts.
```

[S24-1]: https://docs.pydantic.dev/latest/ "Pydantic docs navigation and integrations"
[S24-2]: https://docs.pydantic.dev/latest/api/types/ "Pydantic Types"

---

# Pydantic Advanced — 25) Experimental surfaces

## 25.0 Stability contract

Experimental APIs are intentionally outside the normal Pydantic V2 compatibility promise. Pydantic’s version policy permits experimental features to change in patch/minor releases or disappear with limited notice. Isolate them behind dedicated modules and exact-version tests. [S25-1]

Current 2.13 stable experimental surfaces documented prominently include:

* Pipeline API.
* Partial validation.
* Callable argument-schema generation.
* `MISSING` sentinel.

## 25.1 Pipeline API

Introduced in 2.8, the experimental pipeline API composes validation, transformation, constraints, and predicates in `Annotated` metadata. [S25-2]

```python
from typing import Annotated
from pydantic import BaseModel
from pydantic.experimental.pipeline import validate_as

class User(BaseModel):
    name: Annotated[str, validate_as(str).str_lower()]
    age: Annotated[int, validate_as(int).gt(0)]
```

Use when a chain reads much more clearly than multiple validators and you accept experimental API churn. Do not build a public library ABI around it without a strict pin.

## 25.2 Partial validation

Partial validation is a proof-of-concept feature introduced in 2.10 for incomplete/streamed data, especially LLM JSON. It is enabled only through the three `TypeAdapter` validation methods using `experimental_allow_partial`. Supported values are `False/'off'`, `True/'on'`, and `'trailing-strings'`. [S25-2]

```python
from pydantic import TypeAdapter

ta = TypeAdapter(list[int])
value = ta.validate_json('[1, 2, 3', experimental_allow_partial=True)
```

Critical limitations:

* It is **TypeAdapter-only**; `BaseModel.model_validate_*` does not expose the flag.
* Only a subset of collection validators propagate partial semantics (`list`, `set`, `frozenset`, `dict`, and non-required `TypedDict` fields).
* The implementation deliberately ignores **all errors in the last element** under partial mode, not only errors provably caused by truncation.
* Nested `BaseModel` structures do not currently propagate partial behavior in the general case.

**Agent rule:** partial validation is for progressive display/parsing, not for final authoritative validation. Revalidate the completed payload normally before side effects.

## 25.3 Callable argument schemas

`pydantic.experimental.arguments_schema.generate_arguments_schema()` can create a core schema for a callable’s arguments **without calling the function**; the result can be executed with `pydantic_core.SchemaValidator`. [S25-2]

```python
from pydantic_core import SchemaValidator
from pydantic.experimental.arguments_schema import generate_arguments_schema

def f(flag: bool, *items: str, **meta: int): ...

validator = SchemaValidator(generate_arguments_schema(func=f))
args, kwargs = validator.validate_json('{"flag": true, "args": ["a"]}')
```

Use cases: RPC/tool argument preflight, LLM tool-call validation, job-envelope validation. Prefer `@validate_call` when the desired behavior is simply “validate and then call this Python function.”

## 25.4 `MISSING` sentinel in stable 2.13

In 2.13, import it from the experimental namespace:

```python
from pydantic.experimental.missing_sentinel import MISSING
```

It distinguishes “not provided” from explicit `None`, is automatically excluded from serialization, and does not appear in generated JSON Schema. It is experimental in the stable 2.13 docs, relies on sentinel work related to PEP 661, has static-analysis limitations, and models containing it are not pickleable. [S25-2]

```python
from typing import Union
from pydantic import BaseModel
from pydantic.experimental.missing_sentinel import MISSING

class Patch(BaseModel):
    timeout: Union[int, None, MISSING] = MISSING
```

**2.14 watch:** the 2.14 prerelease program is stabilizing `MISSING`; do not use the future top-level import in 2.13.5-targeted code.

## 25.5 Experimental deployment wrapper

```python
# experimental_pydantic.py
from pydantic import VERSION

EXPECTED = (2, 13)

def assert_supported_minor() -> None:
    major, minor, *_ = map(int, VERSION.split('.'))
    if (major, minor) != EXPECTED:
        raise RuntimeError('re-audit experimental Pydantic APIs before upgrade')
```

## 25.6 Test gate

```text
[ ] experimental import isolated in one package/module
[ ] exact Pydantic minor tested
[ ] fallback behavior defined
[ ] completed partial payload revalidated normally
[ ] no production side effect based only on partial validation
[ ] JSON Schema behavior for MISSING tested
[ ] upgrade CI fails loudly if experimental API disappears
```

[S25-1]: https://docs.pydantic.dev/latest/version-policy/ "Pydantic version policy"
[S25-2]: https://docs.pydantic.dev/latest/concepts/experimental/ "Experimental features"

---

# Pydantic Advanced — 26) Plugin and validation-instrumentation surface

## 26.0 Status

Pydantic 2.13.5 still exposes `ConfigDict.plugin_settings`, and the 2.13.5 patch notes include a fix for validator reuse when plugins are configured. However, the current stable documentation navigation no longer exposes the older dedicated plugin concept page as a first-class concept. Treat the plugin mechanism as **experimental/infrastructure-level**, verify against the pinned package source before implementing a new plugin, and prefer ordinary validators/serializers for application behavior. [S26-1] [S26-2]

## 26.1 Architectural intent

Historically, Pydantic plugins are Python package entry points under the `pydantic` group that can instrument the three validation entrypoints (`validate_python`, `validate_json`, `validate_strings`) around `SchemaValidator` execution. They are best thought of as cross-cutting validation instrumentation rather than schema business logic.

Suitable uses:

```text
observability / tracing
validation metrics
cross-cutting debugging
controlled enterprise instrumentation
```

Poor uses:

```text
field-specific business rules
normal data transformations
authorization
network/database writes
model serialization policy
```

## 26.2 `plugin_settings`

`ConfigDict.plugin_settings` provides plugin-specific configuration payloads. Keep plugin configuration namespaced and treat it as private between application/plugin versions.

```python
class Model(BaseModel):
    model_config = ConfigDict(
        plugin_settings={
            'my_plugin': {'sample_rate': 0.1}
        }
    )
```

Exact plugin hooks are lower-level and should be source-verified for the pinned Pydantic version rather than copied from historical docs.

## 26.3 Plugin safety rules

* Plugin callbacks should not change validation semantics unless that behavior is intentional and exhaustively tested.
* Avoid retaining validated input objects indefinitely; instrumentation must not create hidden memory leaks.
* Redact sensitive values.
* Keep plugin failure policy explicit: fail-open vs fail-closed for observability tooling.
* Test validator reuse because 2.13.5 specifically fixes a reuse issue under plugins.
* Prefer Logfire or framework-native tracing when it meets the need without custom plugin maintenance.

## 26.4 Agent checklist

```text
[ ] Confirm plugin API in installed 2.13.5 source before coding.
[ ] Prefer high-level validators/serializers for schema behavior.
[ ] Namespace plugin_settings.
[ ] Redact inputs/errors.
[ ] Test validation with plugin enabled and disabled.
[ ] Test repeated validator/TypeAdapter use.
[ ] Pin Pydantic minor for infrastructure plugins.
```

[S26-1]: https://docs.pydantic.dev/latest/api/config/ "ConfigDict API"
[S26-2]: https://github.com/pydantic/pydantic/releases/tag/v2.13.5 "2.13.5 plugin validator reuse fix"

---

# Pydantic Advanced — 27) Migration and version-upgrade engineering

## 27.0 Three distinct migration problems

```text
V1 → V2           API and semantic migration
V2 minor → minor  supported evolution, but observable behavior can change
patch → patch     bug-fix behavior can still alter edge-case outcomes
```

Do not use one checklist for all three.

## 27.1 V1 → V2 core map

| V1 | V2 |
|---|---|
| `dict()` | `model_dump()` |
| `json()` | `model_dump_json()` |
| `parse_obj()` | `model_validate()` |
| `construct()` | `model_construct()` |
| `copy()` | `model_copy()` |
| `json_schema()` / `schema()` patterns | `model_json_schema()` |
| `update_forward_refs()` | `model_rebuild()` |
| `@validator` | `@field_validator` |
| `@root_validator` | `@model_validator` |
| inner `class Config` | `model_config = ConfigDict(...)` |
| `orm_mode` / `from_orm` | `from_attributes=True` + `model_validate` |
| `__root__` | `RootModel` |
| `GenericModel` | `BaseModel, Generic[T]` |
| `BaseSettings` in `pydantic` | `pydantic-settings` |

The inherited chapters contain the deep syntax and semantic differences; use this table as the compile-pass index.

## 27.2 V2 stability is not “nothing changes”

Pydantic’s V2 policy avoids intentional breaking changes in minors and defers removal of deprecated features to V3, but it explicitly permits certain observable changes, including bug fixes, error message/context/location evolution, and JSON Schema reference formatting. [S27-1]

Test stable semantics, not incidental formatting:

```python
# prefer
assert exc.errors()[0]['type'] == 'int_parsing'

# avoid as primary contract
assert str(exc) == 'exact long prose...'
```

## 27.3 2.12 → 2.13 audit

Re-run tests around:

* subclass/model serialization and `polymorphic_serialization`;
* `computed_field(exclude_if=...)`;
* ASCII-only string constraints if adopted;
* smart unions and ambiguous model branches;
* private-attribute default factories;
* `model_fields_set` if extras are allowed and mutated;
* fixed-length tuple serialization warnings;
* V1 compatibility namespace users.

## 27.4 2.13.4 → 2.13.5 audit

Focus narrowly on:

```text
plugin-enabled validator reuse
smart-union branch selection
long-lived core validator/serializer GC behavior
```

## 27.5 2.13 → 2.14 future migration watch

The 2.14 prerelease line drops Python 3.9. Plan the runtime floor before upgrading. Also re-audit `MISSING` imports/semantics and newly supported Python 3.15 typing/runtime features. [S27-2]

## 27.6 Public-library compatibility matrix

```text
CI lane A: minimum supported Pydantic minor
CI lane B: locked production minor
CI lane C: latest stable <3
CI lane D: optional prerelease canary (allowed to fail, but investigated)
```

For libraries, avoid exact `pydantic==...` pins unless you have a compelling runtime reason; express a tested range and use CI to enforce it.

## 27.7 Upgrade test gate

```text
[ ] model validation success/failure corpus
[ ] strict/lax conversion corpus
[ ] ValidationError.type corpus
[ ] serializer output corpus (Python + JSON modes)
[ ] subclass/polymorphic serialization corpus
[ ] JSON Schema semantic snapshots
[ ] aliases / validation_alias / serialization_alias
[ ] discriminated and smart-union selection
[ ] settings source precedence
[ ] custom core-schema hooks
[ ] TypeAdapter hot paths
[ ] static checker lanes
[ ] deprecation warnings treated as actionable in CI
```

[S27-1]: https://docs.pydantic.dev/latest/version-policy/ "Pydantic version policy"
[S27-2]: https://github.com/pydantic/pydantic/releases/tag/v2.14.0a1 "2.14 alpha: Python 3.9 dropped"

---

# Pydantic Advanced — 28) Testing and QA strategy

## 28.0 Test the contract by plane

A complete Pydantic test suite should separately test:

```text
validation
serialization
JSON Schema
error codes/locations
aliases
configuration
mutation/revalidation
settings sources
performance-sensitive adapters
custom extensions
```

## 28.1 Validation matrix

For each public model/type alias:

```text
[ ] minimal valid input
[ ] maximal/representative valid input
[ ] missing required field
[ ] explicit None vs omitted field
[ ] unknown fields
[ ] wrong scalar type
[ ] boundary constraint values
[ ] strict and lax behavior where exposed
[ ] nested failure location
[ ] alias vs field-name population
```

## 28.2 Serialization matrix

```python
def assert_round_shape(model):
    assert isinstance(model.model_dump(), dict)
    assert isinstance(model.model_dump(mode='json'), dict)
    assert isinstance(model.model_dump_json(), str)
```

Also test:

* `exclude_unset`, `exclude_defaults`, `exclude_none`;
* field `exclude` / `exclude_if`;
* computed fields;
* aliases;
* serializers with context;
* subclass security behavior;
* round-trip behavior for non-idempotent/special types.

## 28.3 Error assertions

Prefer structured errors:

```python
with pytest.raises(ValidationError) as ei:
    Model.model_validate(bad)

errs = ei.value.errors()
assert errs[0]['type'] == 'int_parsing'
assert errs[0]['loc'] == ('field',)
```

Treat `msg`, full rendered string, and some location/context details as less stable than `type` under Pydantic’s version policy.

## 28.4 JSON Schema assertions

If schema is a public contract, normalize before snapshotting:

```text
semantic properties / required / discriminator / types / constraints: stable target
$defs ordering / $ref formatting / title ordering: normalize or assert selectively
```

Validate emitted schema with a JSON Schema validator in CI if downstream systems depend on it.

## 28.5 Property-based testing

Hypothesis is a documented integration. High-value targets:

* numeric constraints;
* nested unions;
* aliases and round-trip serialization;
* date/time boundaries;
* custom validators/core schemas;
* arbitrary Unicode/ASCII-only strings;
* settings parsing from environment-style strings.

## 28.6 Performance regression harness

Only benchmark measured hot paths. Separate:

```text
schema build / import time
validation throughput
JSON validation throughput
serialization throughput
error-path cost
memory retention
```

Warm adapters before steady-state benchmarks if production reuses them.

## 28.7 Golden test policy

Good golden targets:

```text
public wire JSON
public API JSON Schema after normalization
error type codes
migration fixtures
```

Brittle golden targets:

```text
repr()
exact ValidationError prose
internal core-schema dict
private serializer/validator repr
unordered JSON Schema formatting
```

## 28.8 Agent test checklist

```text
[ ] tests are organized by contract plane
[ ] omission vs explicit None tested
[ ] alias input and alias output tested independently
[ ] subclass serialization leak test exists
[ ] TypeAdapter reuse path covered
[ ] experimental feature tests version-gated
[ ] custom type tests cover validation + dump + schema
[ ] deprecations surfaced in CI
```

[S28-1]: https://docs.pydantic.dev/latest/version-policy/ "Version policy"
[S28-2]: https://docs.pydantic.dev/latest/integrations/hypothesis/ "Hypothesis integration"

---

# Pydantic Advanced — 29) Architectural patterns and agent design rules

## 29.0 Boundary-first architecture

Recommended flow:

```text
raw external representation
  → boundary Pydantic model / TypeAdapter
  → normalized application/domain representation
  → business logic
  → explicit outbound Pydantic model
  → serialized wire representation
```

Do not make one omnipotent model serve database persistence, public input, internal domain state, and public output when those contracts differ.

## 29.1 Project model taxonomy

```python
class ApiInput(BaseModel): ...
class ApiOutput(BaseModel): ...
class EventContract(BaseModel): ...
class PersistenceDTO(BaseModel): ...
class InternalDTO(BaseModel): ...
```

For domain value objects, a frozen dataclass or other typed object may be preferable; Pydantic can validate it at the boundary through `TypeAdapter` or nested schema support.

## 29.2 Reusable type aliases over repeated validators

Preferred:

```python
from typing import Annotated
from pydantic import Field

TenantId = Annotated[str, Field(min_length=1, max_length=64, pattern=r'^[A-Za-z0-9_-]+$')]
```

Use low-level core hooks only when high-level metadata cannot express the behavior.

## 29.3 Pure-validator rule

Validators and serializers should normally be:

```text
local
pure
fast
deterministic
side-effect-free
```

Do DB/network/auth/workflow operations outside Pydantic. This improves retry safety, testability, performance, and reasoning about when callbacks execute.

## 29.4 Trust-level policy

| Boundary | Default posture |
|---|---|
| public JSON request | `extra='forbid'`, selective strictness, explicit aliases |
| public response | explicit output model, annotation-based safe serialization |
| queue/event | versioned discriminator, `extra` policy deliberate |
| environment/settings | string-friendly coercion, `BaseSettings`, no import-time construction |
| internal typed API | stricter config acceptable |
| LLM structured output | closed schema + final validation + semantic post-check |
| persistence row | separate persistence DTO if DB representation differs |

## 29.5 Agent code-generation rules

```text
1. Prefer standard annotations + Annotated metadata.
2. Prefer BaseModel for named records, TypeAdapter for arbitrary types.
3. Prefer discriminated unions for model polymorphism.
4. Prefer model_validate_json for raw JSON.
5. Reuse TypeAdapter.
6. Do not use model_construct on untrusted input.
7. Do not add arbitrary_types_allowed to make an error disappear without understanding it.
8. Do not use custom __init__ unless framework internals require it.
9. Keep ConfigDict boundary-specific; do not create a universal “everything enabled” base.
10. Treat serialization as an explicit policy plane, especially with subclasses/secrets.
11. Keep experimental imports isolated.
12. Use error type codes in programmatic logic/tests.
13. Treat JSON Schema as public only if downstream systems consume it.
14. Version event/tool contracts explicitly.
15. Add migration tests before Pydantic minor upgrades.
```

## 29.6 Anti-pattern inventory

* Inheriting API output directly from a persistence model to save lines of code.
* A validator that queries a database to “check existence.”
* `Any` everywhere to silence validation.
* `arbitrary_types_allowed=True` on all models.
* Recreating TypeAdapter inside a tight loop.
* `serialize_as_any=True` globally.
* Depending on the raw shape of `__pydantic_core_schema__` as a public contract.
* Using Pydantic models as mutable application state without assignment/revalidation policy.

---

# Pydantic Advanced — 30) Standard-library type capability index

## 30.0 Purpose

The standard-library types page is the authoritative type-level map for validation inputs, constraints, strictness, and serialization. The following index is designed for fast agent routing; use the conversion table or type page for exact coercion matrices. [S30-1]

## 30.1 Scalar and numeric types

| Family | Types / examples | High-value controls |
|---|---|---|
| Boolean | `bool`, `StrictBool` | strict vs accepted 0/1/string forms |
| Strings | `str` | length, regex, trim/case metadata, ASCII-only via `StringConstraints` |
| Bytes | `bytes`, `StrictBytes` | length, `val_json_bytes` / `ser_json_bytes` |
| Integer | `int`, `StrictInt`, positive/negative aliases | `gt/ge/lt/le/multiple_of` |
| Float | `float`, `StrictFloat`, `FiniteFloat` | bounds, multiple, inf/nan |
| Decimal | `decimal.Decimal` | bounds, `max_digits`, `decimal_places` |
| Complex | `complex` | strict/lax conversion and serialization |
| Fraction | `fractions.Fraction` | numeric conversion/serialization |
| Integer enum | `enum.IntEnum` | enum member/value coercion |

## 30.2 Time and date

```text
datetime.datetime
datetime.date
datetime.time
datetime.timedelta
PastDate / FutureDate
AwareDatetime / NaiveDatetime
PastDatetime / FutureDatetime
```

Important distinction: Python-mode strict validation and JSON-mode strict validation can differ because JSON represents time-like values as strings/numbers.

## 30.3 Collection and mapping families

```text
list[T]
tuple[...]
NamedTuple
set[T]
frozenset[T]
collections.deque[T]
Sequence[T]
dict[K, V]
TypedDict
Iterable[T]
```

Prefer concrete collection types (`list`, `dict`) in performance-sensitive schemas when the input contract can be concrete. `Iterable[T]` can produce lazy validation semantics; do not assume it materializes like `list[T]`.

## 30.4 Structural/typing types

```text
Callable
Type / type[T]
Literal
Any
Hashable
Pattern / regex
Path / pathlib types
Enum
None / NoneType
UUID
IP address/interface/network types
```

## 30.5 String constraints in 2.13

`StringConstraints` supports `strip_whitespace`, `to_upper`, `to_lower`, `strict`, `min_length`, `max_length`, `pattern`, and **`ascii_only`**. The latter is new in the 2.13 program. [S30-2]

```python
from typing import Annotated
from pydantic import StringConstraints

AsciiToken = Annotated[
    str,
    StringConstraints(min_length=1, max_length=64, ascii_only=True),
]
```

## 30.6 Conversion-table rule

Do not re-implement Pydantic’s coercion logic from memory. When an exact accepted-source-type question matters, consult the version-matched conversion table and test the concrete boundary. Strictness is mode-dependent and JSON-mode exceptions are intentional.

## 30.7 Type-selection rules

```text
Need named record + methods        → BaseModel
Need plain mapping shape           → TypedDict + TypeAdapter
Need arbitrary collection/scalar   → TypeAdapter[T]
Need local reusable constraint     → Annotated[T, metadata...]
Need closed polymorphism           → discriminated union
Need no validation of a payload    → Any (only intentionally)
Need streaming iterable semantics  → Iterable[T], with explicit lifecycle awareness
```

[S30-1]: https://docs.pydantic.dev/latest/api/standard_library_types/ "Standard library types"
[S30-2]: https://docs.pydantic.dev/latest/api/types/#stringconstraints "StringConstraints"

---

# Pydantic Advanced — 31) Pydantic, network, and special type capability index

## 31.0 Built-in Pydantic metadata/types

Current type API includes:

```text
Strict
AllowInfNan
StringConstraints
ImportString
UuidVersion
Json
Secret / SecretStr / SecretBytes
PaymentCardNumber
ByteSize
PastDate / FutureDate
AwareDatetime / NaiveDatetime / PastDatetime / FutureDatetime
EncodedBytes / EncodedStr
Base64Encoder / Base64UrlEncoder
GetPydanticSchema
Tag / Discriminator
FailFast
```

It also exports convenience aliases for positive/negative/strict numeric types; UUID versions **1, 3, 4, 5, 6, 7, 8**; path predicates (`FilePath`, `DirectoryPath`, `NewPath`, `SocketPath`); Base64/Base64URL bytes/strings; `JsonValue`; and `OnErrorOmit`. [S31-1]

## 31.1 Network and DSN types

Current network API exposes: [S31-2]

```text
UrlConstraints
AnyUrl
AnyHttpUrl
HttpUrl
AnyWebsocketUrl
WebsocketUrl
FileUrl
FtpUrl

PostgresDsn
CockroachDsn
AmqpDsn
RedisDsn
MongoDsn
KafkaDsn
NatsDsn
MySQLDsn
MariaDBDsn
ClickHouseDsn
SnowflakeDsn

EmailStr
NameEmail
IPvAnyAddress
IPvAnyInterface
IPvAnyNetwork
validate_email
```

Some DSNs are multi-host and some have default-host/port semantics. Read the specific type contract before assuming host/user/database requirements.

## 31.2 URL security warning

`HttpUrl` validates URL shape/scheme; it does not establish whether the host is safe to fetch. If you use validated URLs for outbound network requests, separately enforce SSRF policy (DNS/IP ranges, schemes, ports, redirects, credentials, and network egress policy).

## 31.3 Email extra

`EmailStr` and related email validation require the `email` optional extra / `email-validator` dependency:

```bash
uv add 'pydantic[email]==2.13.5'
```

## 31.4 UUID and ID contracts

Use UUID-version aliases when version is part of the domain contract:

```python
from pydantic import BaseModel, UUID7

class Event(BaseModel):
    id: UUID7
```

Do not use string regexes for UUIDs when a UUID type provides the actual semantic parser.

## 31.5 `FailFast`

`FailFast()` tells sequence validation to stop at the first error, trading complete error aggregation for speed. [S31-3]

```python
from typing import Annotated
from pydantic import FailFast

Ids = Annotated[list[int], FailFast()]
```

Use only when first-error behavior is acceptable to callers.

## 31.6 Encoded types

`EncodedBytes` / `EncodedStr` make encoding/decoding policy explicit, with built-in Base64 and URL-safe Base64 aliases. These are preferable to sprinkling ad hoc encode/decode calls across validators.

## 31.7 Deprecated-style constrained constructors

Functions such as `constr()` remain available in V2, but current docs explicitly discourage `constr` in favor of `Annotated[str, StringConstraints(...)]` and target deprecation in V3. Apply the same architectural preference to other “con*” constructor-returned types when an `Annotated` form exists. [S31-1]

[S31-1]: https://docs.pydantic.dev/latest/api/types/ "Pydantic Types API"
[S31-2]: https://docs.pydantic.dev/latest/api/networks/ "Network Types API"
[S31-3]: https://docs.pydantic.dev/latest/api/types/#failfast "FailFast"

---

# Pydantic Advanced — 32) Dynamic models, generics, recursive types, and annotation resolution

## 32.0 Dynamic models

`create_model()` is the model-factory counterpart to a normal class definition. Use it when fields are genuinely discovered/generated at runtime; prefer explicit classes when the schema is statically known.

```python
from pydantic import create_model

Dynamic = create_model(
    'Dynamic',
    id=(int, ...),
    label=(str, 'default'),
)
```

High-value uses: schema-driven adapters, model transformations, plugin-defined payloads, generated SDK surfaces.

## 32.1 Generic models

V2 uses ordinary `BaseModel` + `typing.Generic`:

```python
from typing import Generic, TypeVar
from pydantic import BaseModel

T = TypeVar('T')

class Page(BaseModel, Generic[T]):
    items: list[T]
    total: int
```

Avoid runtime `isinstance(x, Page[int])` patterns; create a concrete subclass if runtime identity is needed.

## 32.2 Forward/recursive annotations

Typical recursive model:

```python
from __future__ import annotations
from pydantic import BaseModel

class Node(BaseModel):
    name: str
    children: list[Node] = []
```

If types cannot be resolved during class creation (dynamic modules, plugin loading, complex forward refs), use `model_rebuild()` after the relevant namespace exists.

## 32.3 Rebuild policy

```text
normal static modules:
  let Pydantic resolve automatically

dynamic/plugin namespace:
  call model_rebuild(_types_namespace=...)

TypeAdapter deferred build:
  use adapter.rebuild(...) when namespace resolution is intentionally delayed
```

Avoid repeatedly rebuilding models in request paths.

## 32.4 Cyclic input detection

Pydantic detects cyclic references during recursive validation and reports validation errors rather than recursing indefinitely in ordinary supported model graphs. Test actual ORM/object cycles if `from_attributes=True` is involved.

## 32.5 Dynamic-model anti-patterns

* `create_model()` for every request when the schema is actually static.
* Mutating `model_fields` after class creation and assuming validators/schema update automatically.
* Circular imports fixed by arbitrary late imports rather than deliberate `model_rebuild()`/module design.
* Generic models with runtime type assumptions that Python erasure does not support.
* Building JSON Schema before all forward references are resolvable.

## 32.6 Test gate

```text
[ ] parametrized generic model validates expected type
[ ] forward references resolve in clean process startup
[ ] model_json_schema succeeds after rebuild
[ ] recursive invalid data reports finite structured errors
[ ] dynamic model factory produces deterministic fields/config
[ ] no per-request schema rebuild hot loop
```

[S32-1]: https://docs.pydantic.dev/latest/concepts/models/ "Models"
[S32-2]: https://docs.pydantic.dev/latest/concepts/forward_annotations/ "Forward annotations"

---

# Pydantic Advanced — 33) Serialization policy reconciliation for Pydantic 2.13

## 33.0 Three subclass policies

Pydantic’s subclass serialization now has three important conceptual levels:

```text
annotation-based default
  serialize according to declared field schema
  safest / predictable

polymorphic_serialization (2.13)
  for Pydantic models and Pydantic dataclasses
  use runtime subclass's Pydantic serialization schema

serialize_as_any / SerializeAsAny
  broader runtime-type / "as Any" behavior
  applies beyond the Pydantic-model-only polymorphic case
```

## 33.1 Default safety behavior

```python
class User(BaseModel):
    name: str

class UserLogin(User):
    password: str

class Wrapper(BaseModel):
    user: User

assert Wrapper(user=UserLogin(name='x', password='p')).model_dump() == {
    'user': {'name': 'x'}
}
```

This V2 behavior was chosen specifically to make sensitive subclass additions less likely to leak unexpectedly. [S33-1]

## 33.2 Polymorphic serialization (new 2.13)

Configuration-level:

```python
class PolyUser(BaseModel):
    model_config = ConfigDict(polymorphic_serialization=True)
    name: str
```

Runtime override:

```python
payload = obj.model_dump(polymorphic_serialization=True)
```

It applies to Pydantic models and Pydantic dataclasses, not arbitrary standard-library dataclass subclass graphs. [S33-1]

## 33.3 “As Any” behavior

Use `SerializeAsAny[T]` or runtime `serialize_as_any=True` only when serialization should intentionally ignore the declared schema and inspect runtime values more broadly. This is less constrained than Pydantic-model polymorphic serialization.

## 33.4 Field inclusion/exclusion

Serialization policy stack:

```text
Field(exclude=True / exclude_if=...)
computed_field(exclude_if=...)
model_dump include/exclude
exclude_unset
exclude_defaults
exclude_none
exclude_computed_fields
aliases
context-aware serializers
polymorphic_serialization / serialize_as_any
```

Field-level `exclude` has strong precedence. Put security-critical exclusions close to field declarations, and use explicit output DTOs when data exposure rules are important.

## 33.5 Serializer context

Validation context and serialization context are separate call-time channels. A field/model serializer can read `SerializationInfo.context` to parameterize output without global mutation.

Use for locale/format/policy profiles, not authorization side effects.

## 33.6 Temporal, bytes, and non-finite-number policy

`ConfigDict` separates serialization policy for temporal values, bytes, and infinity/NaN. Keep encode/decode settings symmetrical when round-trip behavior matters:

```text
ser_json_temporal / ser_json_timedelta
val_temporal_unit
ser_json_bytes / val_json_bytes
ser_json_inf_nan
```

## 33.7 Serialization test gate

```text
[ ] Python-mode dump
[ ] JSON-mode dump
[ ] JSON string dump
[ ] aliases
[ ] exclude_unset/defaults/none
[ ] field and computed exclude_if
[ ] context serializer
[ ] base/subclass default output
[ ] explicit polymorphic output
[ ] secret subclass cannot leak accidentally
[ ] bytes/temporal/nonfinite values match wire contract
```

[S33-1]: https://docs.pydantic.dev/latest/concepts/serialization/ "Serialization"
[S33-2]: https://docs.pydantic.dev/latest/api/config/ "ConfigDict serialization settings"

---

# Pydantic Advanced — 34) Complete `ConfigDict` option map

## 34.0 Configuration as policy plane

The current stable configuration API exposes a broad set of fields. Organize them by concern rather than alphabetically. [S34-1]

## 34.1 Schema/title/documentation

```text
title
model_title_generator
field_title_generator
json_schema_extra
json_schema_serialization_defaults_required
json_schema_mode_override
```

## 34.2 String normalization

```text
str_to_lower
str_to_upper
str_strip_whitespace
str_min_length
str_max_length
```

Use only when the policy really applies model-wide; field/type aliases are clearer for heterogeneous string semantics.

## 34.3 Input acceptance / object construction

```text
extra
strict
arbitrary_types_allowed
from_attributes
ignored_types
allow_inf_nan
coerce_numbers_to_str
regex_engine
```

## 34.4 Mutation / instance trust

```text
frozen
validate_assignment
revalidate_instances
validate_default
validate_return
```

`validate_return` is relevant to callable validation/config contexts; do not confuse it with ordinary model-field output serialization.

## 34.5 Alias and naming policy

```text
alias_generator
loc_by_alias
validate_by_alias
validate_by_name
serialize_by_alias
populate_by_name  # older compatibility surface; avoid in new code
```

`populate_by_name` is not recommended in newer V2 releases; use explicit `validate_by_name` + `validate_by_alias`. `serialize_by_alias` is a distinct output policy.

## 34.6 JSON/temporal/bytes serialization

```text
ser_json_timedelta
ser_json_temporal
val_temporal_unit
ser_json_bytes
val_json_bytes
ser_json_inf_nan
polymorphic_serialization
```

## 34.7 Errors / diagnostics / security

```text
hide_input_in_errors
validation_error_cause
protected_namespaces
```

## 34.8 Build/performance/infrastructure

```text
defer_build
cache_strings
plugin_settings
```

## 34.9 Deprecated / migration-sensitive configuration

```text
json_encoders    # deprecated carry-over; prefer serializers
schema_generator # deprecated as a ConfigDict setting in newer V2
populate_by_name # moving toward explicit validation-name switches
```

Always check the exact API docs before writing new code around a deprecated setting.

## 34.10 Model-wide baseline examples

Public input:

```python
class ApiInput(BaseModel):
    model_config = ConfigDict(
        extra='forbid',
        validate_by_alias=True,
        validate_by_name=False,
        hide_input_in_errors=True,
    )
```

Internal immutable value object:

```python
class ValueObject(BaseModel):
    model_config = ConfigDict(
        frozen=True,
        extra='forbid',
        strict=True,
    )
```

Attribute-backed output:

```python
class ReadModel(BaseModel):
    model_config = ConfigDict(
        from_attributes=True,
        serialize_by_alias=True,
    )
```

## 34.11 Configuration inheritance rule

Config is inherited/merged across model bases. Multiple inheritance with competing configuration can become hard to reason about; test resolved `Model.model_config` explicitly. Parent model config does not magically become policy for every nested model object — nested Pydantic model/dataclass types are their own configuration boundaries.

## 34.12 Agent checklist

```text
[ ] Choose a config per boundary, not one global mega-config.
[ ] Use validate_by_name/alias explicitly.
[ ] Separate validation alias policy from serialization alias policy.
[ ] Avoid arbitrary_types_allowed unless types are intentionally opaque.
[ ] Pair mutation strategy with validate_assignment/revalidate_instances.
[ ] Set hide_input_in_errors on sensitive boundaries.
[ ] Reuse serializers instead of json_encoders in new code.
[ ] Treat plugin_settings as infrastructure-level.
[ ] Measure before changing cache_strings/defer_build for performance.
```

[S34-1]: https://docs.pydantic.dev/latest/api/config/ "Configuration API"

---

# Pydantic Advanced — 35) JSON Schema contract governance

## 35.0 Schema is a separate product surface

Runtime validation and emitted JSON Schema usually align, but they are not identical artifacts. Decide whether JSON Schema is merely documentation or a versioned contract consumed by clients, LLM APIs, code generators, or gateways.

## 35.1 Generation surfaces

```python
Model.model_json_schema()
TypeAdapter(T).json_schema()
TypeAdapter.json_schemas(...)
models_json_schema(...)
```

Key controls include:

```text
validation vs serialization mode
aliases
ref_template
union_format
custom GenerateJsonSchema subclass
WithJsonSchema
Field(json_schema_extra=...)
__get_pydantic_json_schema__
```

## 35.2 Validation schema vs serialization schema

Generate both when the wire input and output differ because of serializers, computed fields, aliases, or defaults.

```python
validation = Model.model_json_schema(mode='validation')
serialization = Model.model_json_schema(mode='serialization')
```

## 35.3 Reference governance

Pydantic’s version policy explicitly does not treat every `$ref` formatting change as breaking. If downstream consumers key on exact reference strings, normalize or provide your own `ref_template`/generator and treat that as your layer’s contract. [S35-1]

## 35.4 Custom schema ladder

Prefer in this order:

```text
Field title/description/examples/json_schema_extra
WithJsonSchema metadata
custom type __get_pydantic_json_schema__
GenerateJsonSchema subclass
```

Do not mutate internal core-schema dictionaries just to alter presentation schema.

## 35.5 LLM/tool-schema guidance

For LLM structured outputs/tool calls:

* prefer closed objects (`extra='forbid'`) when the provider honors `additionalProperties`;
* prefer discriminated unions over ambiguous `anyOf` model branches;
* keep descriptions concise but semantically useful;
* avoid implementation-only fields;
* version tool payloads when evolution matters;
* test provider-specific schema subsets separately from Pydantic’s full Draft 2020-12 capabilities.

## 35.6 OpenAPI guidance

Pydantic emits JSON Schema designed for modern OpenAPI 3.1 interoperability. The web framework may still transform/deduplicate schemas, so test the **framework’s final OpenAPI** if that is the public artifact, not only `model_json_schema()`.

## 35.7 Contract test gate

```text
[ ] validation schema generated
[ ] serialization schema generated
[ ] aliases as intended
[ ] required/default/nullability correct
[ ] discriminator correct
[ ] computed fields appear only in intended mode
[ ] secrets/internal fields omitted where required
[ ] downstream JSON Schema validator accepts schema
[ ] framework/provider final schema integration tested
[ ] $ref formatting not over-snapshotted unless intentionally stabilized
```

[S35-1]: https://docs.pydantic.dev/latest/version-policy/ "Version policy"
[S35-2]: https://docs.pydantic.dev/latest/concepts/json_schema/ "JSON Schema"

---

# Pydantic Advanced — 36) `pydantic-settings` 2.15 advanced guide

## 36.0 Separate package, separate lifecycle

`pydantic-settings` is not version-locked to Pydantic itself. The latest PyPI release at this reference date is **2.15.0 (2026-08-07)**. It requires Python >=3.10, so an application using Pydantic 2.13.5 on Python 3.9 cannot also use the latest settings release without raising its Python floor. [S36-1]

```bash
uv add 'pydantic-settings==2.15.0'
```

## 36.1 Source stack

The settings system can combine:

```text
initializer kwargs
environment variables
dotenv files
file secrets / nested secrets
CLI arguments
pyproject.toml / other file sources
custom sources
cloud secret managers
```

Source priority is customizable through `settings_customise_sources`.

## 36.2 Environment parsing

Core controls include:

```text
env_prefix
case_sensitive
env_nested_delimiter
env_nested_max_split
env_ignore_empty
env_parse_none_str
env_parse_enums
nested_model_default_partial_update
```

Use aliases (`alias`, `validation_alias`, `AliasChoices`) when deployment-variable names must differ from Python field names.

## 36.3 Dotenv

```python
class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_file='.env',
        env_file_encoding='utf-8',
        extra='ignore',
    )
```

For public libraries, avoid implicitly reading project-local `.env` files at import time. Let the application choose construction and source locations.

## 36.4 CLI support

Current settings docs include a substantial CLI subsystem: lists/dicts/literals/enums, aliases, variadic options, subcommands, positional args, async commands, help generation, serialization, mutually exclusive groups, kebab-case naming, short options, unknown-argument handling, environment-variable help, custom prefix characters, and integration with existing parsers. [S36-2]

Use `CliApp` when a settings model itself should define a typed CLI. For large applications with sophisticated command semantics, compare against a dedicated CLI framework rather than forcing every workflow through settings.

## 36.5 Secret sources

Current docs cover:

* Docker/file secrets;
* `NestedSecretsSettingsSource`;
* AWS Secrets Manager;
* AWS Systems Manager Parameter Store;
* Azure Key Vault;
* Google Cloud Secret Manager.

The package provides optional extras for cloud providers and file formats. [S36-1] [S36-2]

## 36.6 Nested secret security

Minimum safe version for the 2026 symlink advisory is `2.14.2`. The current 2.15.0 target is patched. [S36-3]

## 36.7 Other settings sources and operations

Current documentation also includes:

```text
pyproject.toml source
field value priority
debugging settings sources
changing / adding / removing sources
accessing previous-source results
in-place reloading
caching settings
async environments
```

This means `BaseSettings` can serve as a configurable source orchestration layer, not merely an `os.environ` parser.

## 36.8 Construction rule

Bad:

```python
# settings.py
settings = Settings()  # import-time environment read
```

Preferred:

```python
def load_settings() -> Settings:
    return Settings()
```

Construct once at the application composition boundary, inject into services, and override explicitly in tests.

## 36.9 Deployment checklist

```text
[ ] Python floor compatible with chosen pydantic-settings version
[ ] settings object constructed explicitly, not as hidden import side effect
[ ] source priority documented
[ ] env naming/alias policy documented
[ ] dotenv policy disabled or explicit in production
[ ] secrets source patched >=2.14.2
[ ] cloud secret extras installed only when used
[ ] tests isolate environment variables
[ ] CLI/settings source collisions tested
[ ] reload/cache behavior explicit
```

[S36-1]: https://pypi.org/project/pydantic-settings/ "pydantic-settings 2.15.0"
[S36-2]: https://docs.pydantic.dev/latest/concepts/pydantic_settings/ "Settings management"
[S36-3]: https://github.com/pydantic/pydantic-settings/security/advisories/GHSA-4xgf-cpjx-pc3j "Nested secret source security advisory"

---

# Pydantic Advanced — 37) `pydantic-extra-types` 2.11.1 catalog

## 37.0 Version stance

The latest non-yanked stable package is **2.11.1 (2026-03-16)**. Version 2.11.2 was yanked because multiple features were mistakenly merged before release; maintainers explicitly state that the yank was **not** due to known security risks. [S37-1] [S37-2]

```bash
uv add 'pydantic-extra-types==2.11.1'
```

## 37.1 Current documented module families

The stable Pydantic documentation navigation exposes these Extra Types families:

```text
Color
Country
Payment
Phone Numbers
Routing Numbers
Coordinate
Mac Address
ISBN
Pendulum
Currency
Language
Script Code
Semantic Version
Timezone Name
ULID
```

Optional extras include domain dependencies such as `pendulum`, `phonenumbers`, `pycountry`, `python-ulid`, `semver`, and others depending on the module. [S37-2]

## 37.2 Use when the type is domain-specific but broadly reusable

Good:

```python
from pydantic_extra_types.color import Color
from pydantic_extra_types.phone_numbers import PhoneNumber
```

Avoid reinventing complex standardized parsing (ISBN, country codes, phone numbers, currency codes) as regex validators unless the package semantics do not match your domain.

## 37.3 Dependency isolation

If only one optional module requires a heavy dependency, expose it as an application/library extra rather than making every installation pay for it.

```toml
[project.optional-dependencies]
phones = ["pydantic-extra-types[phonenumbers]==2.11.1"]
```

## 37.4 Contract tests

Treat extra-type parsing/serialization as a third-party contract:

```text
[ ] representative valid inputs
[ ] invalid inputs
[ ] model_dump Python mode
[ ] JSON mode
[ ] JSON Schema
[ ] package extra installed in CI lane
```

[S37-1]: https://pypi.org/project/pydantic-extra-types/2.11.1/ "pydantic-extra-types 2.11.1"
[S37-2]: https://pypi.org/project/pydantic-extra-types/2.11.2/ "Yanked 2.11.2 release"
[S37-3]: https://docs.pydantic.dev/latest/ "Pydantic docs extra-types navigation"

---

# Pydantic Advanced — 38) Low-level core-schema extension cookbook

## 38.0 Extension ladder

Always start at the highest abstraction that works:

```text
standard type annotation
  ↓
Field / annotated-types metadata
  ↓
functional validator / serializer
  ↓
named Annotated alias / TypeAliasType
  ↓
ValidateAs / GetPydanticSchema
  ↓
annotation marker class
  ↓
__get_pydantic_core_schema__ / __get_pydantic_json_schema__
  ↓
direct pydantic_core SchemaValidator / SchemaSerializer
```

The lower you go, the more power you gain and the more upgrade surface you own.

## 38.1 `GetPydanticSchema`

Use to reduce boilerplate for custom annotation behavior when a full marker class is unnecessary.

Conceptual pattern:

```python
from typing import Annotated
from pydantic import GetPydanticSchema
from pydantic_core import core_schema

Doubled = Annotated[
    str,
    GetPydanticSchema(
        lambda tp, handler: core_schema.no_info_after_validator_function(
            lambda x: x * 2,
            handler(tp),
        )
    ),
]
```

## 38.2 Class-owned core schema

```python
class CustomType:
    @classmethod
    def __get_pydantic_core_schema__(cls, source_type, handler):
        ...

    @classmethod
    def __get_pydantic_json_schema__(cls, core_schema, handler):
        ...
```

Use when the class itself owns its Pydantic representation and you control the type.

## 38.3 Third-party type marker

For types you do not own, attach a marker via `Annotated[ThirdPartyType, Marker()]` or `GetPydanticSchema`; do not monkey-patch the external class.

## 38.4 Generic custom types

When building core schema for a generic container, inspect `source_type` to recover concrete type arguments. Generate child schemas through the handler’s supported generic-schema path instead of recursively invoking your own class hook incorrectly.

## 38.5 `SchemaValidator` and `SchemaSerializer`

Direct core execution is appropriate for infrastructure that already owns a `core_schema.CoreSchema`:

```python
from pydantic_core import SchemaValidator, SchemaSerializer, core_schema

schema = core_schema.list_schema(core_schema.int_schema())
validator = SchemaValidator(schema)
serializer = SchemaSerializer(schema)

value = validator.validate_python(['1', 2])
wire = serializer.to_json(value)
```

Application code should normally use `TypeAdapter(list[int])`, which wraps schema generation and public compatibility behavior.

## 38.6 Stability rule

Pydantic documents core schema as the communication format with `pydantic-core`, but low-level schema contents are not promised to remain byte-for-byte structurally stable across minors. Test behavior, not raw dict equality.

## 38.7 Core extension test matrix

```text
[ ] validate_python
[ ] validate_json
[ ] dump_python
[ ] dump_json / serializer.to_json
[ ] JSON Schema
[ ] strict behavior
[ ] null behavior
[ ] nested model/container behavior
[ ] wrong-type errors
[ ] reusable TypeAdapter
[ ] oldest/newest supported Pydantic minor if library code
```

## 38.8 Anti-patterns

* Direct core schema because a simple `AfterValidator` would work.
* Returning an invalid core schema and debugging it through downstream model errors.
* Reusing one handler incorrectly across generic type arguments.
* Depending on exact `__pydantic_core_schema__` dict layout.
* Exposing `pydantic_core` types as public API of an ordinary application library.

[S38-1]: https://docs.pydantic.dev/latest/concepts/types/ "Custom types"
[S38-2]: https://docs.pydantic.dev/latest/internals/architecture/ "Pydantic architecture"
[S38-3]: https://docs.pydantic.dev/latest/api/pydantic_core/ "Pydantic Core"

---

# Pydantic Advanced — 39) Production deployment, performance, observability, and reliability checklist

## 39.0 Production model

```text
startup
  import model graph
  build validators/serializers (unless deferred)
  construct settings once
  warm critical adapters if latency-sensitive

request/job
  validate at ingress
  run business logic
  serialize at egress
  record structured errors/metrics without leaking sensitive values
```

## 39.1 Performance controls

Official guidance: optimize only after measurement. Highest-value knobs/patterns include:

```text
model_validate_json for raw JSON
reuse TypeAdapter instances
concrete collections over abstract ones in hot paths
Any when validation is intentionally unnecessary
discriminated unions over broad untagged unions
TypedDict where plain dict output is enough
avoid wrap validators in hot paths
FailFast for large sequences when first error is enough
defer_build for cold-start-sensitive, rarely used schemas
cache_strings tuning only after measurement
```

## 39.2 Cold-start vs fail-fast startup

```text
serverless / CLI:
  minimize schema imports
  defer rare model builds
  avoid schema generation at import
  avoid Settings() at import

long-running service:
  eagerly build critical schemas
  fail boot if critical model/settings config is invalid
  pre-warm hot adapters
```

## 39.3 Observability

Record:

```text
model/type name
validation mode (Python/JSON/strings)
success/failure counts
error type codes
validation latency
payload size (not raw payload by default)
serializer latency
schema-build/cold-start time where relevant
```

Logfire is Pydantic’s documented observability integration. Whether using Logfire or another tracer, treat raw inputs and error context as potentially sensitive.

## 39.4 Resource controls

Pydantic is fast but not a resource firewall. Apply upstream/downstream caps:

```text
HTTP body size
JSON nesting / application-specific depth
collection max_length constraints
string max_length constraints
queue message size
request timeout
custom validator timeout policy (prefer no IO at all)
```

## 39.5 Long-lived processes

2.13.5 includes GC traversal fixes in `pydantic-core`. For services with massive model/adapter churn or low-level core objects, include memory/GC regression tests around version upgrades. Normal applications should reuse model classes/adapters instead of dynamically rebuilding schemas continuously.

## 39.6 Production checklist

```text
Dependencies
  [ ] pydantic 2.13.5 pinned/locked for application
  [ ] pydantic-core 2.46.5 resolved
  [ ] settings/extra-types versions intentional

Schema construction
  [ ] no per-request create_model / TypeAdapter construction unless required
  [ ] forward refs rebuilt before serving traffic
  [ ] critical models pre-warmed if latency-sensitive

Validation
  [ ] raw JSON uses model_validate_json / TypeAdapter.validate_json
  [ ] public extra-key policy explicit
  [ ] strictness policy explicit
  [ ] discriminated unions used for public polymorphic payloads

Serialization
  [ ] public output DTOs explicit
  [ ] subclass/polymorphic behavior tested
  [ ] secrets/internal fields excluded
  [ ] bytes/time/NaN wire policy explicit

Settings
  [ ] no hidden import-time Settings()
  [ ] source priority explicit
  [ ] nested secrets security patch floor met

Observability
  [ ] structured error type metrics
  [ ] inputs redacted
  [ ] validation/serialization latency measured where material

Upgrade
  [ ] deprecation warnings reviewed
  [ ] newest stable canary CI
  [ ] experimental APIs version-gated
```

[S39-1]: https://docs.pydantic.dev/latest/concepts/performance/ "Performance"
[S39-2]: https://github.com/pydantic/pydantic/releases/tag/v2.13.5 "2.13.5 fixes"

---

# Pydantic Advanced — 40) Pydantic 2.13.5 source-verified capability reconciliation

## 40.0 Purpose and precedence

This chapter is the **version-sensitive precedence layer** for the rest of the reference. When an older inherited chapter and this chapter conflict on a 2.13.5-sensitive fact, this chapter wins.

## 40.1 Exact 2.13.5 package contract

From the released tag:

```text
requires-python: >=3.9

runtime dependencies:
  typing-extensions >=4.14.1
  annotated-types >=0.6.0
  pydantic-core ==2.46.5
  typing-inspection >=0.4.2

extras:
  email    -> email-validator >=2.0.0
  timezone -> tzdata on Windows
```

This is more precise than the current docs landing page, which still labels itself v2.13.4. [S40-1] [S40-2]

## 40.2 Stable public capability index

The current stable documentation explicitly covers:

```text
Concepts:
  Models, Fields, JSON Schema, JSON, Types, Unions, Alias, Configuration,
  Serialization, Validators, Dataclasses, Forward Annotations, Strict Mode,
  Type Adapter, Validation Decorator, Conversion Table, Settings Management,
  Performance, Experimental

Public API:
  BaseModel, RootModel, Pydantic dataclasses, TypeAdapter, validate_call,
  fields, aliases, configuration, JSON Schema, errors, functional validators,
  functional serializers, standard library types, Pydantic types, network types,
  version information, annotated handlers

Core:
  pydantic-core, core_schema

Ecosystem documentation:
  pydantic-settings, pydantic-extra-types, Logfire, LLMs, static/dev tooling,
  AWS Lambda and common boundary examples
```

This reference maps every one of those categories to at least one dedicated chapter or index. [S40-2]

## 40.3 Current `ConfigDict` inventory

The current stable API lists these fields (grouped elsewhere in §34):

```text
title, model_title_generator, field_title_generator,
str_to_lower, str_to_upper, str_strip_whitespace, str_min_length, str_max_length,
extra, frozen, populate_by_name, use_enum_values, validate_assignment,
arbitrary_types_allowed, from_attributes, loc_by_alias, alias_generator,
ignored_types, allow_inf_nan, json_schema_extra, json_encoders, strict,
revalidate_instances, ser_json_timedelta, ser_json_temporal, val_temporal_unit,
ser_json_bytes, val_json_bytes, ser_json_inf_nan, validate_default,
validate_return, protected_namespaces, hide_input_in_errors, defer_build,
plugin_settings, schema_generator, json_schema_serialization_defaults_required,
json_schema_mode_override, coerce_numbers_to_str, regex_engine,
validation_error_cause, use_attribute_docstrings, cache_strings,
validate_by_alias, validate_by_name, serialize_by_alias,
url_preserve_empty_path, polymorphic_serialization
```

`polymorphic_serialization` defaults to `False`. [S40-3]

## 40.4 Current standard and special type indexes

Stable docs enumerate the standard-library families summarized in §30 and the Pydantic/network families summarized in §31. Notable 2.13-era details include `StringConstraints.ascii_only`, UUID aliases through UUID8, and the network DSN set including Postgres, Cockroach, AMQP, Redis, MongoDB, Kafka, NATS, MySQL, MariaDB, ClickHouse, and Snowflake. [S40-4] [S40-5]

## 40.5 Stable experimental boundary

As rendered by the current stable docs:

```text
Pipeline API             experimental
Partial validation       experimental
arguments schema         experimental
MISSING sentinel         experimental (2.13 stable)
```

Partial validation is TypeAdapter-only, propagates through a limited set of collections, and can ignore every error in the last element. Final payloads must be revalidated normally before side effects. [S40-6]

## 40.6 Companion packages

Current companion anchors:

```text
pydantic-settings 2.15.0     current PyPI stable; Python >=3.10
pydantic-extra-types 2.11.1  current non-yanked stable
```

Do not assume companion package versions track Pydantic’s own minor numbering. [S40-7] [S40-8]

## 40.7 2.14 prerelease watch

At this reference date, 2.14.0b2 is tagged 2026-09-09. Treat the following as migration/watch items rather than 2.13.5 APIs:

```text
Python 3.9 dropped in 2.14 line
Python 3.15 support work
MISSING stabilization / sentinel updates
frozendict support
TypeForm adoption
lazy-import support work
deque/core-schema evolution
temporal JSON Schema evolution
```

The 2.14 line should receive a fresh source reconciliation before this document’s stable anchor is moved forward. [S40-9] [S40-10]

## 40.8 Final agent invariants

```text
Invariant 1 — Pydantic validates/normalizes output objects; it does not prove business truth.
Invariant 2 — BaseModel is for named records; TypeAdapter is the universal arbitrary-type handle.
Invariant 3 — validation, serialization, and JSON Schema are related but distinct policy planes.
Invariant 4 — strictness depends on input mode; JSON strict behavior can differ from Python strict behavior.
Invariant 5 — discriminated unions are the preferred public polymorphism mechanism.
Invariant 6 — subclass serialization is security-sensitive; default, polymorphic, and "as any" are distinct.
Invariant 7 — reuse TypeAdapter; do not rebuild schemas in hot loops.
Invariant 8 — keep validators/serializers local and side-effect-free.
Invariant 9 — do not depend on raw core-schema dict layout as a stable public contract.
Invariant 10 — experimental APIs require explicit version gates.
Invariant 11 — settings and extra-types are separate packages with separate version/runtime floors.
Invariant 12 — use released tag/PyPI facts when the stable docs version label lags a patch release.
```

## 40.9 Reconciliation test gate

```text
[ ] package/version lock matches canonical stack
[ ] public API usage appears in stable docs or 2.13.5 source
[ ] no 2.14-only imports in stable code
[ ] ConfigDict options checked against current API
[ ] type names checked against current standard/Pydantic/network indexes
[ ] experimental calls isolated
[ ] companion package runtime floors compatible with application Python
[ ] smart-union and subclass-serialization edge cases regression-tested
```

[S40-1]: https://raw.githubusercontent.com/pydantic/pydantic/v2.13.5/pyproject.toml "2.13.5 pyproject metadata"
[S40-2]: https://docs.pydantic.dev/latest/ "Stable docs navigation (renders v2.13.4)"
[S40-3]: https://docs.pydantic.dev/latest/api/config/ "ConfigDict API"
[S40-4]: https://docs.pydantic.dev/latest/api/standard_library_types/ "Standard library types"
[S40-5]: https://docs.pydantic.dev/latest/api/networks/ "Network types"
[S40-6]: https://docs.pydantic.dev/latest/concepts/experimental/ "Experimental features"
[S40-7]: https://pypi.org/project/pydantic-settings/ "pydantic-settings"
[S40-8]: https://pypi.org/project/pydantic-extra-types/2.11.1/ "pydantic-extra-types 2.11.1"
[S40-9]: https://github.com/pydantic/pydantic/releases/tag/v2.14.0b2 "Pydantic 2.14.0b2"
[S40-10]: https://github.com/pydantic/pydantic/releases/tag/v2.14.0a1 "Pydantic 2.14.0a1"

---

# Appendix A — Fast API / capability index

| Need | Primary surface |
|---|---|
| Named validated object | `BaseModel` |
| Single root value with model identity | `RootModel[T]` |
| Arbitrary type validation | `TypeAdapter[T]` |
| Dataclass ergonomics + validation | `pydantic.dataclasses.dataclass` |
| Plain validated dict shape | `TypedDict` + `TypeAdapter` |
| Field constraints/defaults/metadata | `Field`, `Annotated`, `annotated-types` |
| Field validation | `@field_validator`, functional validators |
| Whole-model invariant | `@model_validator` |
| Field serialization | `@field_serializer`, `PlainSerializer`, `WrapSerializer` |
| Whole-model serialization | `@model_serializer` |
| Derived serialized property | `@computed_field` |
| External field naming | aliases / `AliasPath` / `AliasChoices` / `AliasGenerator` |
| Model policy | `ConfigDict` |
| JSON Schema | `model_json_schema`, `TypeAdapter.json_schema` |
| Validate callable invocation | `@validate_call` |
| Settings/env/CLI/secrets | `pydantic-settings` |
| Domain-specialized types | `pydantic-extra-types` |
| Custom third-party type | `Annotated` marker / `GetPydanticSchema` |
| Deep custom type | `__get_pydantic_core_schema__` |
| Low-level execution | `SchemaValidator`, `SchemaSerializer` |
| Streaming incomplete data | experimental TypeAdapter partial validation |

---

# Appendix B — Deprecation and Pydantic 3 watchlist

Re-audit before Pydantic 3:

```text
V1 method aliases still present through deprecations
v1 Config class compatibility
@validator / @root_validator compatibility
constr and other constrained constructor style where Annotated is preferred
populate_by_name
json_encoders
schema_generator ConfigDict setting
instance-level access patterns deprecated in newer V2 (e.g. model_fields via instance)
pydantic.v1 compatibility dependency
experimental pipeline / partial / MISSING APIs
```

Do not mechanically replace deprecations during a major upgrade without checking semantic differences.

---

# Appendix C — Source map

Core stable sources:

* https://docs.pydantic.dev/latest/
* https://docs.pydantic.dev/latest/concepts/models/
* https://docs.pydantic.dev/latest/concepts/fields/
* https://docs.pydantic.dev/latest/concepts/validators/
* https://docs.pydantic.dev/latest/concepts/serialization/
* https://docs.pydantic.dev/latest/concepts/json_schema/
* https://docs.pydantic.dev/latest/concepts/strict_mode/
* https://docs.pydantic.dev/latest/concepts/unions/
* https://docs.pydantic.dev/latest/concepts/performance/
* https://docs.pydantic.dev/latest/concepts/experimental/
* https://docs.pydantic.dev/latest/api/config/
* https://docs.pydantic.dev/latest/api/types/
* https://docs.pydantic.dev/latest/api/networks/
* https://docs.pydantic.dev/latest/api/standard_library_types/
* https://docs.pydantic.dev/latest/internals/architecture/
* https://docs.pydantic.dev/latest/version-policy/

Version anchors:

* https://github.com/pydantic/pydantic/releases/tag/v2.13.5
* https://raw.githubusercontent.com/pydantic/pydantic/v2.13.5/pyproject.toml
* https://pypi.org/project/pydantic/

Companion packages:

* https://docs.pydantic.dev/latest/concepts/pydantic_settings/
* https://pypi.org/project/pydantic-settings/
* https://pypi.org/project/pydantic-extra-types/

Future watch:

* https://github.com/pydantic/pydantic/releases/tag/v2.14.0b2

---

# Appendix D — LLM-agent implementation checklist

```text
Before coding
[ ] Identify exact boundary: API / settings / event / DB / internal / LLM / CLI.
[ ] Determine stable Pydantic target version.
[ ] Choose BaseModel vs TypeAdapter vs dataclass vs TypedDict.
[ ] Decide strict/lax and Python/JSON/strings input mode.
[ ] Decide alias policy and extra-key policy.

While coding
[ ] Use standard annotations + Annotated metadata first.
[ ] Keep validators/serializers pure.
[ ] Prefer discriminated unions for model alternatives.
[ ] Reuse TypeAdapter.
[ ] Keep external IO out of validators.
[ ] Separate validation from serialization policy.
[ ] Preserve omission vs explicit None when it matters.
[ ] Never use model_construct for untrusted input.

Before shipping
[ ] Validation success/failure tests.
[ ] Serialization tests in Python and JSON modes.
[ ] Error type-code tests.
[ ] JSON Schema tests if public.
[ ] Subclass serialization/secret leak tests.
[ ] Performance profile only if validation is material.
[ ] Settings source/security audit.
[ ] Deprecation warnings reviewed.
[ ] Experimental APIs version-gated.
[ ] Upgrade/canary CI for latest stable.
```
