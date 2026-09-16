# `fastmcp.client.caching`

Distribution: `fastmcp`

## CACHEABLE_RESULT_MODELS

`fastmcp.client.caching.CACHEABLE_RESULT_MODELS`

```python
CACHEABLE_RESULT_MODELS = _cacheable_result_models()
```

**Inferred type** (`ty`, not declared in the source): `dict[str, type[CacheableResult]]`

Type tag -> model class allowlist for envelope reconstruction.


## DEFAULT_CACHE_COLLECTION

`fastmcp.client.caching.DEFAULT_CACHE_COLLECTION`

```python
DEFAULT_CACHE_COLLECTION = 'fastmcp_response_cache'
```

**Inferred type** (`ty`, not declared in the source): `Literal["fastmcp_response_cache"]`

Collection namespace owned by one adapter instance; `clear()` never reaches beyond it.


## logger

`fastmcp.client.caching.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## KeyValueResponseCacheStore

`fastmcp.client.caching.KeyValueResponseCacheStore`

```python
class KeyValueResponseCacheStore
```

**Declared members (4)**

- `async def clear(self) -> None`  _async_
  Clear this adapter's collection only.
- `async def delete(self, key: CacheKey) -> None`  _async_
- `async def get(self, key: CacheKey) -> CacheEntry | None`  _async_
- `async def set(self, key: CacheKey, entry: CacheEntry) -> None`  _async_

A `ResponseCacheStore` backed by any `AsyncKeyValue` store.

Implements the SDK client response cache contract (`get`/`set`/`delete`/
`clear`) over the key-value abstraction FastMCP already uses elsewhere, so a
distributed deployment can point every client at one shared backend (memory,
Redis, etc.). Pass an instance as `CacheConfig(store=...)`; the SDK requires
an explicit `partition` on any custom store, and FastMCP additionally
requires a `target_id`.

Each adapter instance owns one collection (`collection`), so `clear()` only
affects its own namespace and never another tenant's data. `clear()` needs
the backend to support collection destruction or key enumeration; against a
backend that supports neither it is a no-op and entries age out by TTL (a
warning is logged once).

The SDK wraps every store call defensively — a raised operation degrades to
a cache miss rather than failing the request — so this adapter does not
re-wrap its own operations.

Args:
    storage: The `AsyncKeyValue` backend. Defaults to an in-process `MemoryStore`.
    collection: Collection namespace for this adapter's entries.


## _CacheEnvelope

`fastmcp.client.caching._CacheEnvelope`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _CacheEnvelope(FastMCPBaseModel)
```

**Bases** `FastMCPBaseModel`

**Declared members (4)**

- `expires_at: float | None`  _instance-attribute_
- `scope: str`  _instance-attribute_
- `type_tag: str`  _instance-attribute_
- `value_json: str`  _instance-attribute_

Serializable form of a `CacheEntry` for a remote store.

A `CacheEntry.value` is a cacheable result model; a remote store cannot hold
it as an object, so it is serialized to `value_json` under a `type_tag`
(the model class name) and reconstructed against the allowlist on read. The
freshness/sharing metadata (`scope`, `expires_at`) round-trips alongside it.


## _cacheable_result_models

`fastmcp.client.caching._cacheable_result_models`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _cacheable_result_models() -> dict[str, type[CacheableResult]]
```

Allowlist of `{class name: model}` for every cacheable result type.

Derived from `MONOLITH_RESULTS` (the SDK's per-method result registry) so it
tracks the CACHEABLE_METHODS surface automatically. The class name is the
type tag written into the envelope; reconstruction looks the model up here
rather than importing an arbitrary name from store contents.


