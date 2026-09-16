# `fastmcp.client.progress`

Distribution: `fastmcp`

## ProgressHandler

Import as `fastmcp.client.client.ProgressHandler`  ·  defined at `fastmcp.client.progress.ProgressHandler`

```python
ProgressHandler: TypeAlias = ProgressFnT
```

**Also exported as** `fastmcp.client.client.ProgressHandler`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## logger

`fastmcp.client.progress.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## default_progress_handler

Import as `fastmcp.client.client.default_progress_handler`  ·  defined at `fastmcp.client.progress.default_progress_handler`

```python
async def default_progress_handler(progress: float, total: float | None, message: str | None) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Default handler for progress notifications.

Logs progress updates at debug level, properly handling missing total or message values.

Args:
    progress: Current progress value
    total: Optional total expected value
    message: Optional status message


