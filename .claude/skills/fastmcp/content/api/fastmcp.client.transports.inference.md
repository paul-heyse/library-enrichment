# `fastmcp.client.transports.inference`

Distribution: `fastmcp`

## _PACKAGE_ROOT

`fastmcp.client.transports.inference._PACKAGE_ROOT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_PACKAGE_ROOT = str(Path(__file__).resolve().parents[2])
```

## logger

`fastmcp.client.transports.inference.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## _external_stacklevel

`fastmcp.client.transports.inference._external_stacklevel`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _external_stacklevel() -> int
```

Stack level of the first caller outside the fastmcp package.

Public entry points reach `infer_transport` through different wrapper depths
(`Client(...)`, `create_proxy(...)`, a direct call), so a fixed stacklevel
would point the deprecation warning at internal frames for some of them.


## _is_fastmcp_server

`fastmcp.client.transports.inference._is_fastmcp_server`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_fastmcp_server(transport: object) -> bool
```

## infer_transport

Import as `fastmcp.client.transports.infer_transport`  ·  defined at `fastmcp.client.transports.inference.infer_transport`

```python
def infer_transport(transport: ClientTransport | FastMCP | SDKServer | AnyUrl | Path | MCPConfig | dict[str, Any] | str) -> ClientTransport
```

**Overloads** (the signature above is the runtime dispatcher):

- `def infer_transport(transport: ClientTransportT) -> ClientTransportT`
- `def infer_transport(transport: FastMCP) -> FastMCPTransport`
- `def infer_transport(transport: SDKServer) -> FastMCPTransport`
- `def infer_transport(transport: MCPConfig) -> MCPConfigTransport`
- `def infer_transport(transport: dict[str, Any]) -> MCPConfigTransport`
- `def infer_transport(transport: AnyUrl) -> SSETransport | StreamableHttpTransport`
- `def infer_transport(transport: str) -> PythonStdioTransport | NodeStdioTransport | SSETransport | StreamableHttpTransport`
- `def infer_transport(transport: Path) -> PythonStdioTransport | NodeStdioTransport`

**Also exported as** `fastmcp.client.transports.infer_transport`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Infer the appropriate transport type from the given transport argument.

This function attempts to infer the correct transport type from the provided
argument, handling various input types and converting them to the appropriate
ClientTransport subclass.

The function supports these input types:
- ClientTransport: Used directly without modification
- FastMCP or SDKServer: Creates an in-memory FastMCPTransport
- Path: Creates PythonStdioTransport (.py) or NodeStdioTransport (.js)
- AnyUrl or str (URL): Creates StreamableHttpTransport (default) or SSETransport (for /sse endpoints).
  A str naming an existing .py or .js file still infers a stdio transport but
  emits a FastMCPDeprecationWarning; pass a Path instead.
- MCPConfig or dict: Creates MCPConfigTransport, potentially connecting to multiple servers

For HTTP URLs, they are assumed to be Streamable HTTP URLs unless they end in `/sse`.

For MCPConfig with multiple servers, a composite client is created where each server
is mounted with its name as prefix. This allows accessing tools and resources from multiple
servers through a single unified client interface, using naming patterns like
`servername_toolname` for tools and `protocol://servername/path` for resources.
If the MCPConfig contains only one server, a direct connection is established without prefixing.

Examples:
    ```python
    # Connect to a local Python script
    transport = infer_transport(Path("my_script.py"))

    # Connect to a remote server via HTTP
    transport = infer_transport("http://example.com/mcp")

    # Connect to multiple servers using MCPConfig
    config = {
        "mcpServers": {
            "weather": {"url": "http://weather.example.com/mcp"},
            "calendar": {"url": "http://calendar.example.com/mcp"}
        }
    }
    transport = infer_transport(config)
    ```


