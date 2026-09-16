# `fastmcp.server.lifespan`

Distribution: `fastmcp`

## LifespanContextManagerFn

`fastmcp.server.lifespan.LifespanContextManagerFn`

```python
LifespanContextManagerFn = Callable[['FastMCP[Any]'], AbstractAsyncContextManager[dict[str, Any] | None]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '(FastMCP[Any], /) -> AbstractAsyncContextManager[dict[str, Any] | None, bool | None]'> ````

## LifespanFn

`fastmcp.server.lifespan.LifespanFn`

```python
LifespanFn = Callable[['FastMCP[Any]'], AsyncIterator[dict[str, Any] | None]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '(FastMCP[Any], /) -> AsyncIterator[dict[str, Any] | None]'> ````

## ComposedLifespan

`fastmcp.server.lifespan.ComposedLifespan`

```python
class ComposedLifespan(Lifespan)
```

**Bases** `Lifespan`

Two lifespans composed together.

Enters the left lifespan first, then the right. Exits in reverse order.
Results are shallow-merged into a single dict.


## ContextManagerLifespan

`fastmcp.server.lifespan.ContextManagerLifespan`

```python
class ContextManagerLifespan(Lifespan)
```

**Bases** `Lifespan`

Lifespan wrapper for already-wrapped context manager functions.

Use this for functions already decorated with @asynccontextmanager.


## Lifespan

Import as `fastmcp.server.server.Lifespan`  ·  defined at `fastmcp.server.lifespan.Lifespan`

```python
class Lifespan
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Composable lifespan wrapper.

Wraps an async generator function and enables composition via the `|` operator.
The wrapped function should yield a dict that becomes part of the lifespan context.


## lifespan

`fastmcp.server.lifespan.lifespan`

```python
def lifespan(fn: LifespanFn) -> Lifespan
```

Decorator to create a composable lifespan.

Use this decorator on an async generator function to make it composable
with other lifespans using the `|` operator.

Example:
    ```python
    @lifespan
    async def my_lifespan(server):
        # Setup
        resource = await create_resource()
        yield {"resource": resource}
        # Teardown
        await resource.close()

    mcp = FastMCP("server", lifespan=my_lifespan | other_lifespan)
    ```

Args:
    fn: An async generator function that takes a FastMCP server and yields
        a dict for the lifespan context.

Returns:
    A composable Lifespan wrapper.


