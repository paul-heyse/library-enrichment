# `fastmcp.utilities.version_check`

Distribution: `fastmcp`

## CACHE_TTL_SECONDS

`fastmcp.utilities.version_check.CACHE_TTL_SECONDS`

```python
CACHE_TTL_SECONDS = 60 * 60 * 12
```

**Inferred type** (`ty`, not declared in the source): `Literal[43200]`

## PYPI_URL

`fastmcp.utilities.version_check.PYPI_URL`

```python
PYPI_URL = 'https://pypi.org/pypi/fastmcp/json'
```

**Inferred type** (`ty`, not declared in the source): `Literal["https://pypi.org/pypi/fastmcp/json"]`

## REQUEST_TIMEOUT_SECONDS

`fastmcp.utilities.version_check.REQUEST_TIMEOUT_SECONDS`

```python
REQUEST_TIMEOUT_SECONDS = 2.0
```

**Inferred type** (`ty`, not declared in the source): `float*`

## logger

`fastmcp.utilities.version_check.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## _fetch_latest_version

`fastmcp.utilities.version_check._fetch_latest_version`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _fetch_latest_version(include_prereleases: bool = False) -> str | None
```

Fetch the latest version from PyPI.

Args:
    include_prereleases: If True, include pre-release versions (alpha, beta, rc).

Returns:
    The latest version string, or None if the fetch failed.


## _get_cache_path

`fastmcp.utilities.version_check._get_cache_path`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_cache_path(include_prereleases: bool = False) -> Path
```

Get the path to the version cache file.


## _read_cache

`fastmcp.utilities.version_check._read_cache`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _read_cache(include_prereleases: bool = False) -> tuple[str | None, float]
```

Read cached version info.

Returns:
    Tuple of (cached_version, cache_timestamp) or (None, 0) if no cache.


## _write_cache

`fastmcp.utilities.version_check._write_cache`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _write_cache(latest_version: str, include_prereleases: bool = False) -> None
```

Write version info to cache.


## check_for_newer_version

Import as `fastmcp.cli.cli.check_for_newer_version`  ·  defined at `fastmcp.utilities.version_check.check_for_newer_version`

```python
def check_for_newer_version() -> str | None
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Check if a newer version of FastMCP is available.

Returns:
    The latest version string if newer than current, None otherwise.


## get_latest_version

`fastmcp.utilities.version_check.get_latest_version`

```python
def get_latest_version(include_prereleases: bool = False) -> str | None
```

Get the latest version of FastMCP from PyPI, using cache when available.

Args:
    include_prereleases: If True, include pre-release versions.

Returns:
    The latest version string, or None if unavailable.


