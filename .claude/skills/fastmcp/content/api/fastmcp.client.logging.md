# `fastmcp.client.logging`

Distribution: `fastmcp`

## LogHandler

Import as `fastmcp.client.client.LogHandler`  ·  defined at `fastmcp.client.logging.LogHandler`

```python
LogHandler: TypeAlias = Callable[[LogMessage], Awaitable[None]]
```

**Also exported as** `fastmcp.client.client.LogHandler`

## LogMessage

Import as `fastmcp.server.providers.proxy.LogMessage`  ·  defined at `fastmcp.client.logging.LogMessage`

```python
LogMessage: TypeAlias = LoggingMessageNotificationParams
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## from_server_logger

`fastmcp.client.logging.from_server_logger`

```python
from_server_logger: Logger = get_logger(name='fastmcp.client.from_server')
```

## logger

`fastmcp.client.logging.logger`

```python
logger: Logger = get_logger(name=__name__)
```

## create_log_callback

Import as `fastmcp.client.client.create_log_callback`  ·  defined at `fastmcp.client.logging.create_log_callback`

```python
def create_log_callback(handler: LogHandler | None = None) -> LoggingFnT
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## default_log_handler

Import as `fastmcp.client.client.default_log_handler`  ·  defined at `fastmcp.client.logging.default_log_handler`

```python
async def default_log_handler(message: LogMessage) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Default handler that properly routes server log messages to appropriate log levels.


