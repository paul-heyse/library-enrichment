# `fastmcp.client.roots`

Distribution: `fastmcp`

## RootsHandler

Import as `fastmcp.client.client.RootsHandler`  ·  defined at `fastmcp.client.roots.RootsHandler`

```python
RootsHandler: TypeAlias = Callable[[RequestContext[ClientSession, LifespanContextT]], RootsList] | Callable[[RequestContext[ClientSession, LifespanContextT]], Awaitable[RootsList]]
```

**Also exported as** `fastmcp.client.client.RootsHandler`

## RootsList

Import as `fastmcp.client.client.RootsList`  ·  defined at `fastmcp.client.roots.RootsList`

```python
RootsList: TypeAlias = list[str] | list[mcp_types.Root] | list[str | mcp_types.Root]
```

**Also exported as** `fastmcp.client.client.RootsList`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _create_roots_callback_from_fn

`fastmcp.client.roots._create_roots_callback_from_fn`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _create_roots_callback_from_fn(fn: Callable[[RequestContext[ClientSession, LifespanContextT]], RootsList] | Callable[[RequestContext[ClientSession, LifespanContextT]], Awaitable[RootsList]]) -> ListRootsFnT
```

## _create_roots_callback_from_roots

`fastmcp.client.roots._create_roots_callback_from_roots`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _create_roots_callback_from_roots(roots: RootsList) -> ListRootsFnT
```

## convert_roots_list

`fastmcp.client.roots.convert_roots_list`

```python
def convert_roots_list(roots: RootsList) -> list[mcp_types.Root]
```

## create_roots_callback

Import as `fastmcp.client.client.create_roots_callback`  ·  defined at `fastmcp.client.roots.create_roots_callback`

```python
def create_roots_callback(handler: RootsList | RootsHandler) -> ListRootsFnT
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

