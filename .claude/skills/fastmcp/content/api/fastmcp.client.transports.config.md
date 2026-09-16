# `fastmcp.client.transports.config`

Distribution: `fastmcp`

## logger

`fastmcp.client.transports.config.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## MCPConfigTransport

Import as `fastmcp.client.client.MCPConfigTransport`  ·  defined at `fastmcp.client.transports.config.MCPConfigTransport`

```python
class MCPConfigTransport(ClientTransport)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ClientTransport`

**Declared members (6)**

- `async def close(self)`  _async_
- `config = config`  _instance-attribute_
- `async def connect_session(self, transport_options: TransportOptions | None = None, session_kwargs: Unpack[SessionKwargs] = {}) -> AsyncIterator[ClientSession]`  _async_
- `legacy_only: bool`  _property_
  Whether the connected composite resolved to the legacy protocol era.
- `name_as_prefix = name_as_prefix`  _instance-attribute_
- `transport = next(iter(self.config.mcpServers.values())).to_transport()`  _instance-attribute_

**Inherited (1)**

- from `fastmcp.client.transports.base.ClientTransport`: `get_session_id`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Transport for connecting to one or more MCP servers defined in an MCPConfig.

This transport provides a unified interface to multiple MCP servers defined in an MCPConfig
object or dictionary matching the MCPConfig schema. It supports two key scenarios:

1. If the MCPConfig contains exactly one server, it creates a direct transport to that server.
2. If the MCPConfig contains multiple servers, it creates a composite client by mounting
   all servers on a single FastMCP instance, with each server's name, by default, used as its mounting prefix.

In the multiserver case, tools are accessible with the prefix pattern `{server_name}_{tool_name}`
and resources with the pattern `protocol://{server_name}/path/to/resource`.

This is particularly useful for creating clients that need to interact with multiple specialized
MCP servers through a single interface, simplifying client code.

Examples:
    ```python
    from fastmcp import Client

    # Create a config with multiple servers
    config = {
        "mcpServers": {
            "weather": {
                "url": "https://weather-api.example.com/mcp",
                "transport": "http"
            },
            "calendar": {
                "url": "https://calendar-api.example.com/mcp",
                "transport": "http"
            }
        }
    }

    # Create a client with the config
    client = Client(config)

    async with client:
        # Access tools with prefixes
        weather = await client.call_tool("weather_get_forecast", {"city": "London"})
        events = await client.call_tool("calendar_list_events", {"date": "2023-06-01"})

        # Access resources with prefixed URIs
        icons = await client.read_resource("weather://weather/icons/sunny")
    ```


