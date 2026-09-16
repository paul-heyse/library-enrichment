# `fastmcp.client.elicitation`

Distribution: `fastmcp`

## ElicitationHandler

Import as `fastmcp.client.client.ElicitationHandler`  ·  defined at `fastmcp.client.elicitation.ElicitationHandler`

```python
ElicitationHandler: TypeAlias = Callable[[str, type[T] | None, ElicitRequestParams, RequestContext[ClientSession, LifespanContextT]], Awaitable[T | dict[str, Any] | ElicitResult[T | dict[str, Any]]]]
```

**Also exported as** `fastmcp.client.client.ElicitationHandler`

## T

`fastmcp.client.elicitation.T`

```python
T = TypeVar('T', default=Any)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## __all__

`fastmcp.client.elicitation.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['ElicitRequestParams', 'ElicitResult', 'ElicitationHandler']
```

## ElicitResult

Import as `fastmcp.cli.client.ElicitResult`  ·  defined at `fastmcp.client.elicitation.ElicitResult`

```python
class ElicitResult(MCPElicitResult, Generic[T])
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPElicitResult`, `Generic[T]`

**Declared members (1)**

- `content: T | None = None`  _class-attribute, instance-attribute_

## create_elicitation_callback

Import as `fastmcp.client.client.create_elicitation_callback`  ·  defined at `fastmcp.client.elicitation.create_elicitation_callback`

```python
def create_elicitation_callback(elicitation_handler: ElicitationHandler) -> ElicitationFnT
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

