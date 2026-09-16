# `fastmcp.client.transports.stdio`

Distribution: `fastmcp`

## logger

`fastmcp.client.transports.stdio.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## FastMCPStdioTransport

Import as `fastmcp.client.transports.FastMCPStdioTransport`  ·  defined at `fastmcp.client.transports.stdio.FastMCPStdioTransport`

```python
class FastMCPStdioTransport(StdioTransport)
```

**Also exported as** `fastmcp.client.transports.FastMCPStdioTransport`

**Bases** `StdioTransport`

**Declared members (1)**

- `script_path = script_path`  _instance-attribute_

**Inherited (12)**

- from `fastmcp.client.transports.base.ClientTransport`: `get_session_id`, `legacy_only`
- from `fastmcp.client.transports.stdio.StdioTransport`: `args`, `close`, `command`, `connect`, `connect_session`, `cwd`, `disconnect`, `env`, `keep_alive`, `log_file`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Transport for running FastMCP servers using the FastMCP CLI.


## NodeStdioTransport

Import as `fastmcp.client.NodeStdioTransport`  ·  defined at `fastmcp.client.transports.stdio.NodeStdioTransport`

```python
class NodeStdioTransport(StdioTransport)
```

**Also exported as** `fastmcp.client.NodeStdioTransport`, `fastmcp.client.transports.NodeStdioTransport`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `StdioTransport`

**Declared members (1)**

- `script_path = script_path`  _instance-attribute_

**Inherited (12)**

- from `fastmcp.client.transports.base.ClientTransport`: `get_session_id`, `legacy_only`
- from `fastmcp.client.transports.stdio.StdioTransport`: `args`, `close`, `command`, `connect`, `connect_session`, `cwd`, `disconnect`, `env`, `keep_alive`, `log_file`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Transport for running Node.js scripts.


## NpxStdioTransport

Import as `fastmcp.client.NpxStdioTransport`  ·  defined at `fastmcp.client.transports.stdio.NpxStdioTransport`

```python
class NpxStdioTransport(StdioTransport)
```

**Also exported as** `fastmcp.client.NpxStdioTransport`, `fastmcp.client.transports.NpxStdioTransport`

**Bases** `StdioTransport`

**Declared members (1)**

- `package = package`  _instance-attribute_

**Inherited (12)**

- from `fastmcp.client.transports.base.ClientTransport`: `get_session_id`, `legacy_only`
- from `fastmcp.client.transports.stdio.StdioTransport`: `args`, `close`, `command`, `connect`, `connect_session`, `cwd`, `disconnect`, `env`, `keep_alive`, `log_file`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Transport for running commands via the npx tool.


## PythonStdioTransport

Import as `fastmcp.client.PythonStdioTransport`  ·  defined at `fastmcp.client.transports.stdio.PythonStdioTransport`

```python
class PythonStdioTransport(StdioTransport)
```

**Also exported as** `fastmcp.client.PythonStdioTransport`, `fastmcp.client.transports.PythonStdioTransport`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `StdioTransport`

**Declared members (1)**

- `script_path = script_path`  _instance-attribute_

**Inherited (12)**

- from `fastmcp.client.transports.base.ClientTransport`: `get_session_id`, `legacy_only`
- from `fastmcp.client.transports.stdio.StdioTransport`: `args`, `close`, `command`, `connect`, `connect_session`, `cwd`, `disconnect`, `env`, `keep_alive`, `log_file`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Transport for running Python scripts.


## StdioTransport

Import as `fastmcp.client.StdioTransport`  ·  defined at `fastmcp.client.transports.stdio.StdioTransport`

```python
class StdioTransport(ClientTransport)
```

**Also exported as** `fastmcp.client.StdioTransport`, `fastmcp.client.transports.StdioTransport`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ClientTransport`

**Declared members (10)**

- `args = args`  _instance-attribute_
- `async def close(self)`  _async_
- `command = command`  _instance-attribute_
- `async def connect(self, transport_options: TransportOptions | None = None, session_kwargs: Unpack[SessionKwargs] = {}) -> ClientSession | None`  _async_
- `async def connect_session(self, transport_options: TransportOptions | None = None, session_kwargs: Unpack[SessionKwargs] = {}) -> AsyncIterator[ClientSession]`  _async_
- `cwd = cwd`  _instance-attribute_
- `async def disconnect(self)`  _async_
- `env = env`  _instance-attribute_
- `keep_alive = keep_alive`  _instance-attribute_
- `log_file = log_file`  _instance-attribute_

**Inherited (2)**

- from `fastmcp.client.transports.base.ClientTransport`: `get_session_id`, `legacy_only`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Base transport for connecting to an MCP server via subprocess with stdio.

This is a base class that can be subclassed for specific command-based
transports like Python, Node, Uvx, etc.


## UvStdioTransport

Import as `fastmcp.client.UvStdioTransport`  ·  defined at `fastmcp.client.transports.stdio.UvStdioTransport`

```python
class UvStdioTransport(StdioTransport)
```

**Also exported as** `fastmcp.client.UvStdioTransport`, `fastmcp.client.transports.UvStdioTransport`

**Bases** `StdioTransport`

**Inherited (12)**

- from `fastmcp.client.transports.base.ClientTransport`: `get_session_id`, `legacy_only`
- from `fastmcp.client.transports.stdio.StdioTransport`: `args`, `close`, `command`, `connect`, `connect_session`, `cwd`, `disconnect`, `env`, `keep_alive`, `log_file`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Transport for running commands via the uv tool.


## UvxStdioTransport

Import as `fastmcp.client.UvxStdioTransport`  ·  defined at `fastmcp.client.transports.stdio.UvxStdioTransport`

```python
class UvxStdioTransport(StdioTransport)
```

**Also exported as** `fastmcp.client.UvxStdioTransport`, `fastmcp.client.transports.UvxStdioTransport`

**Bases** `StdioTransport`

**Declared members (1)**

- `tool_name: str = tool_name`  _instance-attribute_

**Inherited (12)**

- from `fastmcp.client.transports.base.ClientTransport`: `get_session_id`, `legacy_only`
- from `fastmcp.client.transports.stdio.StdioTransport`: `args`, `close`, `command`, `connect`, `connect_session`, `cwd`, `disconnect`, `env`, `keep_alive`, `log_file`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Transport for running commands via the uvx tool.


## _stdio_transport_connect_task

`fastmcp.client.transports.stdio._stdio_transport_connect_task`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _stdio_transport_connect_task(command: str, args: list[str], env: dict[str, str] | None, cwd: str | None, log_file: Path | TextIO | None, session_kwargs: SessionKwargs, transport_options: TransportOptions, ready_event: anyio.Event, stop_event: anyio.Event, session_future: asyncio.Future[ClientSession])
```

A standalone connection task for a stdio transport. It is not a part of the StdioTransport class
to ensure that the connection task does not hold a reference to the Transport object.


