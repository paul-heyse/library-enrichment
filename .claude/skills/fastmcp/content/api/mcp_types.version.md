# `mcp_types.version`

Distribution: `mcp-types`

## HANDSHAKE_PROTOCOL_VERSIONS

Import as `mcp.client.client.HANDSHAKE_PROTOCOL_VERSIONS`  ·  defined at `mcp_types.version.HANDSHAKE_PROTOCOL_VERSIONS`

```python
HANDSHAKE_PROTOCOL_VERSIONS: Final[tuple[str, ...]] = ('2024-11-05', '2025-03-26', '2025-06-18', '2025-11-25')
```

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Protocol revisions reachable via the initialize handshake.


## KNOWN_PROTOCOL_VERSIONS

Import as `mcp_types.methods.KNOWN_PROTOCOL_VERSIONS`  ·  defined at `mcp_types.version.KNOWN_PROTOCOL_VERSIONS`

```python
KNOWN_PROTOCOL_VERSIONS: Final[tuple[str, ...]] = ('2024-11-05', '2025-03-26', '2025-06-18', '2025-11-25', '2026-07-28')
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Every released protocol revision, oldest to newest.


## LATEST_HANDSHAKE_VERSION

Import as `mcp.server.runner.LATEST_HANDSHAKE_VERSION`  ·  defined at `mcp_types.version.LATEST_HANDSHAKE_VERSION`

```python
LATEST_HANDSHAKE_VERSION: Final[str] = HANDSHAKE_PROTOCOL_VERSIONS[-1]
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Newest revision reachable via the ``initialize`` handshake; the client's offer and server's counter-offer default.


## LATEST_MODERN_VERSION

Import as `mcp.server.runner.LATEST_MODERN_VERSION`  ·  defined at `mcp_types.version.LATEST_MODERN_VERSION`

```python
LATEST_MODERN_VERSION: Final[str] = MODERN_PROTOCOL_VERSIONS[-1]
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Newest per-request-envelope revision; the ``server/discover`` probe default.


## LATEST_PROTOCOL_VERSION

Import as `mcp_types.LATEST_PROTOCOL_VERSION`  ·  defined at `mcp_types.version.LATEST_PROTOCOL_VERSION`

```python
LATEST_PROTOCOL_VERSION: Final[str] = KNOWN_PROTOCOL_VERSIONS[-1]
```

**Also exported as** `mcp_types.LATEST_PROTOCOL_VERSION`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Newest protocol revision this SDK speaks (any era).


## MODERN_PROTOCOL_VERSIONS

Import as `mcp.client.client.MODERN_PROTOCOL_VERSIONS`  ·  defined at `mcp_types.version.MODERN_PROTOCOL_VERSIONS`

```python
MODERN_PROTOCOL_VERSIONS: Final[tuple[str, ...]] = ('2026-07-28',)
```

_17 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Protocol revisions that use the stateless per-request envelope.


## OLDEST_SUPPORTED_VERSION

`mcp_types.version.OLDEST_SUPPORTED_VERSION`

```python
OLDEST_SUPPORTED_VERSION: Final[str] = HANDSHAKE_PROTOCOL_VERSIONS[0]
```

Oldest revision this SDK still negotiates via the ``initialize`` handshake.


## SUPPORTED_PROTOCOL_VERSIONS

`mcp_types.version.SUPPORTED_PROTOCOL_VERSIONS`

```python
SUPPORTED_PROTOCOL_VERSIONS: tuple[str, ...] = (*HANDSHAKE_PROTOCOL_VERSIONS, *MODERN_PROTOCOL_VERSIONS)
```

Deprecated: prefer HANDSHAKE_PROTOCOL_VERSIONS or MODERN_PROTOCOL_VERSIONS.

Kept as the union for v1.x compatibility.


## __all__

`mcp_types.version.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['KNOWN_PROTOCOL_VERSIONS', 'HANDSHAKE_PROTOCOL_VERSIONS', 'MODERN_PROTOCOL_VERSIONS', 'SUPPORTED_PROTOCOL_VERSIONS', 'LATEST_PROTOCOL_VERSION', 'LATEST_HANDSHAKE_VERSION', 'LATEST_MODERN_VERSION', 'OLDEST_SUPPORTED_VERSION', 'is_version_at_least']
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## is_version_at_least

Import as `mcp.server.streamable_http.is_version_at_least`  ·  defined at `mcp_types.version.is_version_at_least`

```python
def is_version_at_least(version: str, minimum: str) -> bool
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return True if `version` is a known revision at least as new as `minimum`.

Unknown `version` strings return False (treat unrecognized peers
conservatively). `minimum` must be a member of KNOWN_PROTOCOL_VERSIONS;
passing anything else is programmer error and raises ValueError.


