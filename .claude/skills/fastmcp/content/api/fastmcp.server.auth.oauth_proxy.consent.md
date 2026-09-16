# `fastmcp.server.auth.oauth_proxy.consent`

Distribution: `fastmcp`

## _CONSENT_STATE_COOKIE_BASE

`fastmcp.server.auth.oauth_proxy.consent._CONSENT_STATE_COOKIE_BASE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_CONSENT_STATE_COOKIE_BASE = 'MCP_CONSENT_STATE'
```

## _MAX_CSRF_TOKENS

`fastmcp.server.auth.oauth_proxy.consent._MAX_CSRF_TOKENS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MAX_CSRF_TOKENS = 10
```

## _MAX_REMEMBERED_CLIENTS

`fastmcp.server.auth.oauth_proxy.consent._MAX_REMEMBERED_CLIENTS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MAX_REMEMBERED_CLIENTS = 25
```

## logger

`fastmcp.server.auth.oauth_proxy.consent.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ConsentMixin

Import as `fastmcp.server.auth.oauth_proxy.proxy.ConsentMixin`  ·  defined at `fastmcp.server.auth.oauth_proxy.consent.ConsentMixin`

```python
class ConsentMixin
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Mixin class providing consent management functionality for OAuthProxy.

This mixin contains all methods related to:
- Cookie signing and verification
- Consent page rendering
- Consent approval/denial handling
- URI normalization for consent tracking


## _consent_state_base_name

`fastmcp.server.auth.oauth_proxy.consent._consent_state_base_name`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _consent_state_base_name(csrf_token: str) -> str
```

Base cookie name carrying a single issued CSRF token.

The token is hashed rather than used directly so the raw token does not end
up in a cookie name, which is far more likely to be logged than its value.


