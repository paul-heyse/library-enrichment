# `mcp.shared._callable_inspection`

Distribution: `mcp`

## AwaitableCallable

`mcp.shared._callable_inspection.AwaitableCallable`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
AwaitableCallable = Callable[..., Awaitable[T]]
```

## T

`mcp.shared._callable_inspection.T`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
T = TypeVar('T')
```

## is_async_callable

Import as `mcp.server.mcpserver.resolve.is_async_callable`  ·  defined at `mcp.shared._callable_inspection.is_async_callable`

```python
def is_async_callable(obj: Any) -> Any
```

**Overloads** (the signature above is the runtime dispatcher):

- `def is_async_callable(obj: AwaitableCallable[T]) -> TypeGuard[AwaitableCallable[T]]`
- `def is_async_callable(obj: Any) -> TypeGuard[AwaitableCallable[Any]]`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

