# `fastmcp.utilities.exceptions`

Distribution: `fastmcp`

## _catch_handlers

`fastmcp.utilities.exceptions._catch_handlers`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_catch_handlers: Mapping[type[BaseException] | Iterable[type[BaseException]], Callable[[BaseExceptionGroup[Any]], Any]] = {Exception: _exception_handler}
```

## _exception_handler

`fastmcp.utilities.exceptions._exception_handler`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _exception_handler(group: BaseExceptionGroup)
```

## _is_legacy_httpx_exception

`fastmcp.utilities.exceptions._is_legacy_httpx_exception`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_legacy_httpx_exception(exc: BaseException, exception_type: str) -> bool
```

Check a legacy-httpx exception without importing the legacy package.


## get_catch_handlers

Import as `fastmcp.client.client.get_catch_handlers`  ·  defined at `fastmcp.utilities.exceptions.get_catch_handlers`

```python
def get_catch_handlers() -> Mapping[type[BaseException] | Iterable[type[BaseException]], Callable[[BaseExceptionGroup[Any]], Any]]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## get_http_status_code

Import as `fastmcp.server.server.get_http_status_code`  ·  defined at `fastmcp.utilities.exceptions.get_http_status_code`

```python
def get_http_status_code(exc: BaseException) -> int | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return the response status code from a recognized HTTP status error.


## is_http_status_error

`fastmcp.utilities.exceptions.is_http_status_error`

```python
def is_http_status_error(exc: BaseException) -> bool
```

Return whether an exception is an httpx2 or legacy-httpx status error.


## is_request_error

Import as `fastmcp.server.providers.openapi.components.is_request_error`  ·  defined at `fastmcp.utilities.exceptions.is_request_error`

```python
def is_request_error(exc: BaseException) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return whether an exception is an httpx2 or legacy-httpx request error.


## is_timeout_error

Import as `fastmcp.server.server.is_timeout_error`  ·  defined at `fastmcp.utilities.exceptions.is_timeout_error`

```python
def is_timeout_error(exc: BaseException) -> bool
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return whether an exception is an httpx2 or legacy-httpx timeout.


## iter_exc

`fastmcp.utilities.exceptions.iter_exc`

```python
def iter_exc(group: BaseExceptionGroup)
```

**Inferred type** (`ty`, not declared in the source): `def iter_exc(group: BaseExceptionGroup[BaseException]) -> Unknown`

