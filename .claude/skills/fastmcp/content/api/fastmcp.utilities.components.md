# `fastmcp.utilities.components`

Distribution: `fastmcp`

## T

`fastmcp.utilities.components.T`

```python
T = TypeVar('T', default=Any)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## FastMCPComponent

Import as `fastmcp.tools.base.FastMCPComponent`  ·  defined at `fastmcp.utilities.components.FastMCPComponent`

```python
class FastMCPComponent(FastMCPBaseModel)
```

_18 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPBaseModel`

**Declared members (15)**

- `KEY_PREFIX: str = ''`  _class-attribute_
- `description: str | None = Field(default=None, description='The description of the component.')`  _class-attribute, instance-attribute_
- `def disable(self) -> None`
  Removed in 3.0. Use server.disable(keys=[...]) instead.
- `def enable(self) -> None`
  Removed in 3.0. Use server.enable(keys=[...]) instead.
- `def get_meta(self) -> dict[str, Any]`
  Get the meta information about the component.
- `def get_span_attributes(self) -> dict[str, Any]`
  Return span attributes for telemetry.
- `icons: list[Icon] | None = Field(default=None, description='Optional list of icons for this component to display in user interfaces.')`  _class-attribute, instance-attribute_
- `key: str`  _property_
  The globally unique lookup key for this component.
- `def make_key(cls, identifier: str) -> str`  _classmethod_
  Construct the lookup key for this component type.
- `meta: dict[str, Any] | None = Field(default=None, description='Meta information about the component')`  _class-attribute, instance-attribute_
- `name: str = Field(description='The name of the component.')`  _class-attribute, instance-attribute_
- `tags: Annotated[set[str], BeforeValidator(_convert_set_default_none)] = Field(default_factory=set, description='Tags for the component.')`  _class-attribute, instance-attribute_
- `task_config: Annotated[TaskConfig, Field(description="Background task execution configuration (SEP-2663). Only tools support task execution; other component types always carry the default 'forbidden' config.")] = Field(default_factory=lambda: TaskConfig(mode='forbidden'))`  _class-attribute, instance-attribute_
- `title: str | None = Field(default=None, description='The title of the component for display purposes.')`  _class-attribute, instance-attribute_
- `version: Annotated[str | None, BeforeValidator(_coerce_version)] = Field(default=None, description='Optional version identifier for this component. Multiple versions of the same component (same name) can coexist.')`  _class-attribute, instance-attribute_

Base class for FastMCP tools, prompts, resources, and resource templates.


## FastMCPMeta

`fastmcp.utilities.components.FastMCPMeta`

```python
class FastMCPMeta(TypedDict)
```

**Bases** `TypedDict`

**Declared members (3)**

- `tags: list[str]`  _instance-attribute_
- `version: str`  _instance-attribute_
- `versions: list[str]`  _instance-attribute_

## _coerce_version

`fastmcp.utilities.components._coerce_version`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _coerce_version(v: str | int | float | None) -> str | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Coerce version to string, accepting int, float, or str.

Raises TypeError for non-scalar types (list, dict, set, etc.).
Raises ValueError if version contains '@' (used as key delimiter).


## _convert_set_default_none

`fastmcp.utilities.components._convert_set_default_none`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _convert_set_default_none(maybe_set: set[T] | Sequence[T] | None) -> set[T]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Convert a sequence to a set, defaulting to an empty set if None.


## get_fastmcp_metadata

Import as `fastmcp.server.providers.proxy.get_fastmcp_metadata`  ·  defined at `fastmcp.utilities.components.get_fastmcp_metadata`

```python
def get_fastmcp_metadata(meta: dict[str, Any] | None) -> FastMCPMeta
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Extract FastMCP metadata from a component's meta dict.

Handles both the current `fastmcp` namespace and the legacy `_fastmcp`
namespace for compatibility with older FastMCP servers.


