# `fastmcp.utilities.token_cache`

Distribution: `fastmcp`

## DEFAULT_MAX_CACHE_SIZE

`fastmcp.utilities.token_cache.DEFAULT_MAX_CACHE_SIZE`

```python
DEFAULT_MAX_CACHE_SIZE = 10000
```

**Inferred type** (`ty`, not declared in the source): `Literal[10000]`

## _CLEANUP_INTERVAL

`fastmcp.utilities.token_cache._CLEANUP_INTERVAL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_CLEANUP_INTERVAL = 60
```

## logger

`fastmcp.utilities.token_cache.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## TokenCache

Import as `fastmcp.server.auth.providers.github.TokenCache`  ·  defined at `fastmcp.utilities.token_cache.TokenCache`

```python
class TokenCache
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `enabled: bool`  _property_
  Return whether caching is active.
- `def get(self, token: str) -> tuple[bool, AccessToken | None]`
  Look up a cached verification result.
- `def set(self, token: str, result: AccessToken) -> None`
  Store a *successful* verification result.

TTL-based in-memory cache for ``AccessToken`` objects.

Features:
- SHA-256 hashed cache keys (fixed size, regardless of token length).
- Per-entry TTL that respects both the configured ``ttl_seconds`` and the
  token's own ``expires_at`` claim (whichever is sooner).
- Bounded size with FIFO eviction when the cache is full.
- Periodic cleanup of expired entries to prevent unbounded growth.
- Defensive deep copies on both store and retrieve to prevent
  callers from mutating cached values.

Caching is disabled when ``ttl_seconds`` is ``None`` or ``0``, or
when ``max_size`` is ``0``.  Negative values raise ``ValueError``.


## _CacheEntry

`fastmcp.utilities.token_cache._CacheEntry`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _CacheEntry
```

**Declared members (2)**

- `expires_at: float`  _instance-attribute_
- `result: AccessToken`  _instance-attribute_

A cached token result with its absolute expiration timestamp.


