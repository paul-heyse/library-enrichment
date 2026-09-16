# `mcp.server.mcpserver.utilities.logging`

Distribution: `mcp`

## configure_logging

Import as `mcp.server.mcpserver.server.configure_logging`  ·  defined at `mcp.server.mcpserver.utilities.logging.configure_logging`

```python
def configure_logging(level: Literal['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL'] = 'INFO') -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Configure logging for MCP.

Args:
    level: The log level to use.


## get_logger

Import as `mcp.cli.cli.get_logger`  ·  defined at `mcp.server.mcpserver.utilities.logging.get_logger`

```python
def get_logger(name: str) -> logging.Logger
```

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get a logger nested under MCP namespace.

Args:
    name: The name of the logger.

Returns:
    A configured logger instance.


