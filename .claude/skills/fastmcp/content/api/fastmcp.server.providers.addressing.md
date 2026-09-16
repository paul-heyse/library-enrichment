# `fastmcp.server.providers.addressing`

Distribution: `fastmcp`

## HASH_LENGTH

Import as `fastmcp.server.providers.prefab_synthesis.HASH_LENGTH`  ·  defined at `fastmcp.server.providers.addressing.HASH_LENGTH`

```python
HASH_LENGTH = 12
```

**Inferred type** (`ty`, not declared in the source): `Literal[12]`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## TOOL_HASH_META_KEY

Import as `fastmcp.server.providers.prefab_synthesis.TOOL_HASH_META_KEY`  ·  defined at `fastmcp.server.providers.addressing.TOOL_HASH_META_KEY`

```python
TOOL_HASH_META_KEY = 'tool_hash'
```

**Inferred type** (`ty`, not declared in the source): `Literal["tool_hash"]`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## hash_tool

Import as `fastmcp.server.providers.prefab_synthesis.hash_tool`  ·  defined at `fastmcp.server.providers.addressing.hash_tool`

```python
def hash_tool(app_name: str, tool_name: str) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Deterministic hex hash for a tool in an app.

Same inputs on every replica produce the same output.


## hashed_backend_name

`fastmcp.server.providers.addressing.hashed_backend_name`

```python
def hashed_backend_name(app_name: str, tool_name: str) -> str
```

Format the universal name for a backend tool: ``<hash>_<local_name>``.


## hashed_resource_uri

`fastmcp.server.providers.addressing.hashed_resource_uri`

```python
def hashed_resource_uri(app_name: str, tool_name: str) -> str
```

Per-tool Prefab renderer resource URI.


## parse_hashed_backend_name

Import as `fastmcp.server.providers.prefab_payload.parse_hashed_backend_name`  ·  defined at `fastmcp.server.providers.addressing.parse_hashed_backend_name`

```python
def parse_hashed_backend_name(name: str) -> tuple[str, str] | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Parse ``<HASH_LENGTH hex>_<rest>`` → ``(hash, local_tool_name)`` or None.


## parse_hashed_resource_uri

Import as `fastmcp.server.providers.prefab_synthesis.parse_hashed_resource_uri`  ·  defined at `fastmcp.server.providers.addressing.parse_hashed_resource_uri`

```python
def parse_hashed_resource_uri(uri: str) -> str | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Extract the hash from a Prefab renderer URI, or None.


