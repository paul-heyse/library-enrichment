# `fastmcp.utilities.async_utils`

Distribution: `fastmcp`

## T

`fastmcp.utilities.async_utils.T`

```python
T = TypeVar('T')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## call_sync_fn_in_threadpool

Import as `fastmcp.server.dependencies.call_sync_fn_in_threadpool`  ·  defined at `fastmcp.utilities.async_utils.call_sync_fn_in_threadpool`

```python
async def call_sync_fn_in_threadpool(fn: Callable[..., Any], args: Any = (), kwargs: Any = {}) -> Any
```

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Call a sync function in a threadpool to avoid blocking the event loop.

Uses anyio.to_thread.run_sync which properly propagates contextvars,
making this safe for functions that depend on context (like dependency injection).


## gather

Import as `fastmcp.client.group.gather`  ·  defined at `fastmcp.utilities.async_utils.gather`

```python
async def gather(awaitables: Iterable[Awaitable[T]], return_exceptions: bool = False) -> list[T] | list[T | BaseException]
```

**Overloads** (the signature above is the runtime dispatcher):

- `async def gather(awaitables: Iterable[Awaitable[T]], return_exceptions: Literal[True]) -> list[T | BaseException]`
- `async def gather(awaitables: Iterable[Awaitable[T]], return_exceptions: Literal[False] = ...) -> list[T]`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Run awaitables concurrently and return results in order.

Uses anyio TaskGroup for structured concurrency.

``awaitables`` is consumed lazily, one item at a time, right before each
is handed to the task group. Callers with a dynamic number of awaitables
should pass a generator expression (e.g. ``gather(f(x) for x in xs)``)
rather than a list or list comprehension: a list comprehension calls
every ``f(x)`` up front, creating a batch of coroutine objects before
this function even starts, whereas a generator expression creates each
coroutine only as this function's own scheduling loop asks for it. That
matters because coroutine creation and scheduling can be interrupted
between any two bytecode instructions by a synchronous signal handler
(for example pytest-timeout's SIGALRM-based per-test timeout). If that
happens while a whole batch of coroutines is sitting unscheduled, they
are silently abandoned and eventually trigger a "coroutine was never
awaited" warning attributed to whatever unrelated code happens to be
running when the garbage collector gets to them. Lazy consumption keeps
the window in which a created-but-unscheduled coroutine can exist as
small as possible.

Args:
    awaitables: Iterable of awaitables to run concurrently.
    return_exceptions: If True, exceptions are returned in results.
                      If False, first exception cancels all and raises.

Returns:
    List of results in the same order as input awaitables.


## is_coroutine_function

Import as `fastmcp.utilities.tasks.is_coroutine_function`  ·  defined at `fastmcp.utilities.async_utils.is_coroutine_function`

```python
def is_coroutine_function(fn: Any) -> bool
```

_9 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Check if a callable is a coroutine function, unwrapping functools.partial.

``inspect.iscoroutinefunction`` returns ``False`` for
``functools.partial`` objects wrapping an async function on Python < 3.12.
This helper unwraps any layers of ``partial`` before checking.


