# `fastmcp.server.mixins.lifespan`

Distribution: `fastmcp`

## _lifespan_root_active

`fastmcp.server.mixins.lifespan._lifespan_root_active`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_lifespan_root_active: ContextVar[bool] = ContextVar('fastmcp_lifespan_root_active', default=False)
```

## logger

`fastmcp.server.mixins.lifespan.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## LifespanMixin

Import as `fastmcp.server.mixins.LifespanMixin`  ·  defined at `fastmcp.server.mixins.lifespan.LifespanMixin`

```python
class LifespanMixin
```

**Also exported as** `fastmcp.server.mixins.LifespanMixin`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (1)**

- `docket: Docket | None`  _property_
  The Docket instance owned by this server, if the tasks extension is active.

Mixin providing lifespan infrastructure for FastMCP.


