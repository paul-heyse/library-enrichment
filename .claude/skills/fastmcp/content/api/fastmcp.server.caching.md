# `fastmcp.server.caching`

Distribution: `fastmcp`

## CacheScope

`fastmcp.server.caching.CacheScope`

```python
CacheScope = Literal['public', 'private']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["public", "private"]'> ``` --- Whether a cached result may be shared across authorization contexts (`"public"`) or reused only within the one that produced it (`"private"`).`

Whether a cached result may be shared across authorization contexts
(`"public"`) or reused only within the one that produced it (`"private"`).


## build_cache_hints

Import as `fastmcp.server.server.build_cache_hints`  ·  defined at `fastmcp.server.caching.build_cache_hints`

```python
def build_cache_hints(cache_ttl: int | None, cache_scope: CacheScope | None) -> dict[CacheableMethod, CacheHint] | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Build the per-method `CacheHint` map for the SDK low-level server.

`cache_ttl` is in seconds and is converted to the wire's milliseconds. When
`cache_ttl` is `None` the server emits no hint, so its wire output is
identical to a server that never set one; a `cache_scope` given without a
`cache_ttl` is meaningless (the client gates caching on the presence of a
TTL) and is rejected rather than silently ignored.

Returns `None` when no hint is set, or a map applying the same hint to every
SDK-cacheable method otherwise.

Raises:
    ValueError: If `cache_ttl` is not positive, or if `cache_scope` is set
        without `cache_ttl`.


