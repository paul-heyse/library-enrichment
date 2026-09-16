# `fastmcp.utilities.logging`

Distribution: `fastmcp`

## _level_to_no

`fastmcp.utilities.logging._level_to_no`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_level_to_no: dict[Literal['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL'] | None, int | None] = {'DEBUG': logging.DEBUG, 'INFO': logging.INFO, 'WARNING': logging.WARNING, 'ERROR': logging.ERROR, 'CRITICAL': logging.CRITICAL, None: None}
```

## _ClampedLogFilter

`fastmcp.utilities.logging._ClampedLogFilter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _ClampedLogFilter(logging.Filter)
```

**Bases** `logging.Filter`

**Declared members (3)**

- `def filter(self, record: logging.LogRecord) -> bool`
- `max_level: tuple[int, str] | None = None`  _instance-attribute_
- `min_level: tuple[int, str] | None = None`  _instance-attribute_

## _clamp_logger

`fastmcp.utilities.logging._clamp_logger`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _clamp_logger(logger: logging.Logger, min_level: Literal['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL'] | None = None, max_level: Literal['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL'] | None = None) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Clamp the logger to a minimum and maximum level.

If min_level is provided, messages logged at a lower level than `min_level` will have their level increased to `min_level`.
If max_level is provided, messages logged at a higher level than `max_level` will have their level decreased to `max_level`.

Args:
    min_level: The lower bound of the clamp
    max_level: The upper bound of the clamp


## _get_package_path

`fastmcp.utilities.logging._get_package_path`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_package_path(package: str) -> str | None
```

Return a package directory without importing the package.


## _unclamp_logger

`fastmcp.utilities.logging._unclamp_logger`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _unclamp_logger(logger: logging.Logger) -> None
```

Remove all clamped log filters from the logger.


## configure_logging

`fastmcp.utilities.logging.configure_logging`

```python
def configure_logging(level: Literal['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL'] | int = 'INFO', logger: logging.Logger | None = None, enable_rich_tracebacks: bool | None = None, rich_kwargs: Any = {}) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Configure logging for FastMCP.

Args:
    logger: the logger to configure
    level: the log level to use
    rich_kwargs: the parameters to use for creating RichHandler


## get_logger

Import as `fastmcp.settings.get_logger`  ·  defined at `fastmcp.utilities.logging.get_logger`

```python
def get_logger(name: str) -> logging.Logger
```

_99 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get a logger nested under FastMCP namespace.

Args:
    name: the name of the logger, which will be prefixed with 'FastMCP.'

Returns:
    a configured logger instance


## temporary_log_level

Import as `fastmcp.server.mixins.transport.temporary_log_level`  ·  defined at `fastmcp.utilities.logging.temporary_log_level`

```python
def temporary_log_level(level: str | None, logger: logging.Logger | None = None, enable_rich_tracebacks: bool | None = None, rich_kwargs: Any = {})
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Context manager to temporarily set log level and restore it afterwards.

Args:
    level: The temporary log level to set (e.g., "DEBUG", "INFO")
    logger: Optional logger to configure (defaults to FastMCP logger)
    enable_rich_tracebacks: Whether to enable rich tracebacks
    **rich_kwargs: Additional parameters for RichHandler

Usage:
    with temporary_log_level("DEBUG"):
        # Code that runs with DEBUG logging
        pass
    # Original log level is restored here


