# `mcp.client.caching`

Distribution: `mcp`

## CacheMode

Import as `mcp.client.CacheMode`  ·  defined at `mcp.client.caching.CacheMode`

```python
CacheMode = Literal['use', 'refresh', 'bypass']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["use", "refresh", "bypass"]'> ``` --- Per-call cache behavior: `"use"` serves and stores, `"refresh"` stores without serving, `"bypass"` skips the cache entirely.`

**Also exported as** `mcp.client.CacheMode`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Per-call cache behavior: `"use"` serves and stores, `"refresh"` stores
without serving, `"bypass"` skips the cache entirely.


## MAX_TTL_MS

`mcp.client.caching.MAX_TTL_MS`

```python
MAX_TTL_MS: Final[int] = 24 * 60 * 60 * 1000
```

Cap on any entry's time-to-live (24 hours, in milliseconds); larger `ttlMs` values are clamped down.


## _GENERATION_MAP_CAP

`mcp.client.caching._GENERATION_MAP_CAP`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_GENERATION_MAP_CAP: Final[int] = 4096
```

Cap on the generation map; at the cap the oldest key's eviction-race guard is dropped (FIFO).


## _STORE_CLEANUP_TIMEOUT

`mcp.client.caching._STORE_CLEANUP_TIMEOUT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_STORE_CLEANUP_TIMEOUT: Final[float] = 5
```

Bound for must-complete store cleanup deletes (mirrors the dispatcher's final-write bound);
a wedged store delete must not hold client teardown uncancellably.


## __all__

`mcp.client.caching.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['MAX_TTL_MS', 'CacheConfig', 'CacheEntry', 'CacheKey', 'CacheMode', 'InMemoryResponseCacheStore', 'ResponseCacheStore']
```

## logger

`mcp.client.caching.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## CacheConfig

Import as `mcp.client.CacheConfig`  ·  defined at `mcp.client.caching.CacheConfig`

```python
class CacheConfig
```

**Also exported as** `mcp.client.CacheConfig`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `clock: Callable[[], float] = time.time`  _class-attribute, instance-attribute_
  Wall-clock source returning epoch seconds; injectable for expiry tests.
- `default_ttl_ms: int = 0`  _class-attribute, instance-attribute_
  TTL in milliseconds for results carrying no `ttlMs` hint; the default `0` leaves them uncached.
- `partition: str = ''`  _class-attribute, instance-attribute_
  Authorization-context identifier isolating `"private"`-scoped entries within a shared store. Derive it from a verified credential - never from request-supplied data or the server URL. Fixed for the `Client`'s lifetime: construct a new `Cli…
- `share_public: bool = False`  _class-attribute, instance-attribute_
  Serve server-marked `"public"` entries across every partition in the store.
- `store: ResponseCacheStore | None = None`  _class-attribute, instance-attribute_
  Backing store; `None` means a per-client `InMemoryResponseCacheStore`. A custom store requires an explicit `partition`.
- `target_id: str | None = None`  _class-attribute, instance-attribute_
  Server-identity override for custom transports and proxies where the SDK cannot derive one from a URL; must be non-empty when provided.

Configuration for a `Client`'s response cache.

Raises:
    ValueError: On a custom `store` without `partition`, an empty `target_id`, or a negative `default_ttl_ms`.


## CacheEntry

Import as `mcp.client.CacheEntry`  ·  defined at `mcp.client.caching.CacheEntry`

```python
class CacheEntry
```

**Also exported as** `mcp.client.CacheEntry`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `expires_at: float | None`  _instance-attribute_
  Epoch seconds after which the entry is stale; `None` is never fresh.
- `scope: Literal['public', 'private']`  _instance-attribute_
  Server-asserted `cacheScope`: only `"public"` entries may be shared across authorization contexts.
- `value: Any`  _instance-attribute_
  The cached result; the SDK deep-copies on write and on serve, so a store may hold it as-is.

One cached response with its freshness and sharing metadata.


## CacheKey

Import as `mcp.client.CacheKey`  ·  defined at `mcp.client.caching.CacheKey`

```python
class CacheKey
```

**Also exported as** `mcp.client.CacheKey`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `method: str`  _instance-attribute_
- `params_key: str = ''`  _class-attribute, instance-attribute_
  Result-affecting params discriminator: the uri for `resources/read`, `""` for the list methods.
- `partition: str = ''`  _class-attribute, instance-attribute_
  Coordinator-computed arm identifier; opaque to stores.

Identity of one cached response; compare as the field tuple, never a flattened string (collision hazard).


## ClientResponseCache

Import as `mcp.client.client.ClientResponseCache`  ·  defined at `mcp.client.caching.ClientResponseCache`

```python
class ClientResponseCache
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `def capture(self, method: str, params_key: str) -> int`
  Register the key for eviction-race detection before the fetch; `write` takes the returned generation.
- `async def evict_for_notification(self, notification: ServerNotification) -> None`  _async_
  Map a server notification to the entries it makes stale.
- `async def evict_key(self, method: str, params_key: str) -> None`  _async_
  Evict one key from both arms.
- `async def evict_method(self, method: str) -> None`  _async_
  Evict the method's cursor-less entry.
- `async def read(self, method: str, params_key: str) -> CacheableResult | None`  _async_
  Serve a fresh entry for the key, or `None`; the served result is a deep copy.
- `async def write(self, method: str, params_key: str, result: CacheableResult, gen_at_capture: int, mode: Literal['use', 'refresh']) -> None`  _async_
  Store a fetched result under the arm its resolved scope selects.

Coordinates the `Client` caching verbs with a `ResponseCacheStore`: keys, era gate, TTL/scope, eviction.


## InMemoryResponseCacheStore

Import as `mcp.client.InMemoryResponseCacheStore`  ·  defined at `mcp.client.caching.InMemoryResponseCacheStore`

```python
class InMemoryResponseCacheStore
```

**Also exported as** `mcp.client.InMemoryResponseCacheStore`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `async def clear(self) -> None`  _async_
- `async def delete(self, key: CacheKey) -> None`  _async_
- `async def get(self, key: CacheKey) -> CacheEntry | None`  _async_
- `async def set(self, key: CacheKey, entry: CacheEntry) -> None`  _async_

Default in-process `ResponseCacheStore`.

Method bodies are synchronous, so concurrent tasks never observe a torn
write. `max_entries` caps the whole store, evicting least-recently-used
at the cap (`0` disables it); `get` and `set` both refresh recency, so a
hot entry survives churn from other keys.

Raises:
    ValueError: If `max_entries` is negative.


## ResponseCacheStore

Import as `mcp.client.ResponseCacheStore`  ·  defined at `mcp.client.caching.ResponseCacheStore`

```python
class ResponseCacheStore(Protocol)
```

**Also exported as** `mcp.client.ResponseCacheStore`

**Bases** `Protocol`

**Declared members (4)**

- `async def clear(self) -> None`  _async_
- `async def delete(self, key: CacheKey) -> None`  _async_
- `async def get(self, key: CacheKey) -> CacheEntry | None`  _async_
- `async def set(self, key: CacheKey, entry: CacheEntry) -> None`  _async_

Storage contract for the client response cache.

Each `Client` calls its store from a single event loop; per-operation
atomicity is the implementation's responsibility. Operations may raise -
the SDK degrades to a miss rather than failing the call. A serializing
store must round-trip `value` back to the result model object (a
wrong-shape entry is a miss, never an error). A lookup may issue two
sequential `get` calls (private arm, then public).


