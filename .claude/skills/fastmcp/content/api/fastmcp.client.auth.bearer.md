# `fastmcp.client.auth.bearer`

Distribution: `fastmcp`

## __all__

`fastmcp.client.auth.bearer.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['BearerAuth']
```

## logger

`fastmcp.client.auth.bearer.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## BearerAuth

Import as `fastmcp.client.BearerAuth`  ·  defined at `fastmcp.client.auth.bearer.BearerAuth`

```python
class BearerAuth(httpx2.Auth)
```

**Also exported as** `fastmcp.client.BearerAuth`, `fastmcp.client.auth.BearerAuth`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `httpx2.Auth`

**Declared members (2)**

- `def auth_flow(self, request)`
- `token = SecretStr(token)`  _instance-attribute_

