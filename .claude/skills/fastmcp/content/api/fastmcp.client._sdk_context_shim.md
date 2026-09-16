# `fastmcp.client._sdk_context_shim`

Distribution: `fastmcp`

## LifespanContextT

Import as `fastmcp.client.roots.LifespanContextT`  ·  defined at `fastmcp.client._sdk_context_shim.LifespanContextT`

```python
LifespanContextT = TypeVar('LifespanContextT')
```

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _SessionT

`fastmcp.client._sdk_context_shim._SessionT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SessionT = TypeVar('_SessionT')
```

## RequestContext

Import as `fastmcp.client.sampling.RequestContext`  ·  defined at `fastmcp.client._sdk_context_shim.RequestContext`

```python
class RequestContext(Generic[_SessionT, LifespanContextT])
```

**Also exported as** `fastmcp.client.sampling.RequestContext`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Generic[_SessionT, LifespanContextT]`

Placeholder for the removed SDK ``RequestContext`` generic.

Subscriptable with two type parameters to match existing client handler
annotations. Not instantiated anywhere; exists only so module imports and
annotation evaluation succeed until the Phase C client port lands.


