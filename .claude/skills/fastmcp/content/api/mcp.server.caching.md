# `mcp.server.caching`

Distribution: `mcp`

## CacheableResultT

`mcp.server.caching.CacheableResultT`

```python
CacheableResultT = TypeVar('CacheableResultT', bound=types.CacheableResult)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## __all__

`mcp.server.caching.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['CACHEABLE_METHODS', 'CacheHint', 'CacheableMethod', 'apply_cache_hint', 'validate_cache_hints']
```

## CacheHint

Import as `mcp.server.CacheHint`  ·  defined at `mcp.server.caching.CacheHint`

```python
class CacheHint
```

**Also exported as** `mcp.server.CacheHint`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `scope: Literal['public', 'private'] = 'private'`  _class-attribute, instance-attribute_
- `ttl_ms: int = 0`  _class-attribute, instance-attribute_

Freshness hint for one cacheable method's results.

`ttl_ms` is how long, in milliseconds, a client may consider the result
fresh (`0` means immediately stale). `scope` is whether a cached result may
be shared across authorization contexts (`"public"`) or only reused within
the one that produced it (`"private"`).


## apply_cache_hint

Import as `mcp.server.runner.apply_cache_hint`  ·  defined at `mcp.server.caching.apply_cache_hint`

```python
def apply_cache_hint(result: CacheableResultT, hint: CacheHint) -> CacheableResultT
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Fill `ttl_ms`/`cache_scope` on `result` from `hint`.

Per-field: a field the handler set explicitly - even to its default value,
tracked via `model_fields_set` - is left alone; only unset fields take the
hint. A handler constructing results with `model_construct` bypasses that
tracking and is treated as having set nothing.


## validate_cache_hints

Import as `mcp.server.lowlevel.server.validate_cache_hints`  ·  defined at `mcp.server.caching.validate_cache_hints`

```python
def validate_cache_hints(cache_hints: Mapping[Any, Any] | None) -> dict[str, CacheHint]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Validate a `cache_hints` constructor argument into a plain dict.

The `Server`/`MCPServer` signatures already close the key set and value
type for type-checked callers; this runtime gate is deliberately loose in
its parameter so it covers everyone else (e.g. a map deserialized from
config) - a bad entry fails at construction, not on the first request to
that method.

Raises:
    ValueError: If a key is not a cacheable method.
    TypeError: If a value is not a `CacheHint`.


