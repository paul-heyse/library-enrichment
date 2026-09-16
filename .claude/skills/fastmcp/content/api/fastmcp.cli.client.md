# `fastmcp.cli.client`

Distribution: `fastmcp`

## _JSON_SCHEMA_TYPE_MAP

`fastmcp.cli.client._JSON_SCHEMA_TYPE_MAP`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_JSON_SCHEMA_TYPE_MAP: dict[str, str] = {'string': 'str', 'integer': 'int', 'number': 'float', 'boolean': 'bool', 'array': 'list', 'object': 'dict', 'null': 'None'}
```

## console

`fastmcp.cli.client.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## logger

`fastmcp.cli.client.logger`

```python
logger = get_logger('cli.client')
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## _build_client

`fastmcp.cli.client._build_client`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _build_client(resolved: str | dict[str, Any] | ClientTransport, timeout: float | None = None, auth: str | None = None) -> Client
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Build a ``Client`` from a resolved server spec.

Applies ``auth='oauth'`` automatically for HTTP-based targets unless
the caller explicitly passes ``--auth none`` to disable it.

``auth=None`` means "not specified" (use default), ``auth="none"``
means "explicitly disabled".


## _build_stdio_from_command

`fastmcp.cli.client._build_stdio_from_command`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _build_stdio_from_command(command_str: str) -> StdioTransport
```

Shell-split a command string into a ``StdioTransport``.


## _call_result_to_dict

`fastmcp.cli.client._call_result_to_dict`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _call_result_to_dict(result: CallToolResult) -> dict[str, Any]
```

Serialize a ``CallToolResult`` to a JSON-safe dict.


## _content_block_to_dict

`fastmcp.cli.client._content_block_to_dict`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _content_block_to_dict(block: mcp_types.ContentBlock) -> dict[str, Any]
```

Serialize a single content block to a JSON-safe dict.


## _format_call_result_text

`fastmcp.cli.client._format_call_result_text`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _format_call_result_text(result: CallToolResult) -> None
```

Pretty-print a tool call result to the console.


## _handle_prompt

`fastmcp.cli.client._handle_prompt`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _handle_prompt(client: Client, prompt_name: str, arguments: tuple[str, ...], input_json: str | None, json_output: bool) -> None
```

Handle a prompt get within an open client session.


## _handle_resource

`fastmcp.cli.client._handle_resource`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _handle_resource(client: Client, uri: str, json_output: bool) -> None
```

Handle a resource read within an open client session.


## _handle_tool_call

`fastmcp.cli.client._handle_tool_call`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _handle_tool_call(client: Client, tool_name: str, arguments: tuple[str, ...], input_json: str | None, json_output: bool) -> None
```

Handle a tool call within an open client session.


## _is_http_target

`fastmcp.cli.client._is_http_target`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_http_target(resolved: str | dict[str, Any] | ClientTransport) -> bool
```

Return True if the resolved target will use an HTTP-based transport.

MCPConfig dicts are excluded because ``MCPConfigTransport`` manages
individual server transports internally and does not support top-level auth.


## _json_schema_type_to_str

`fastmcp.cli.client._json_schema_type_to_str`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _json_schema_type_to_str(schema: dict[str, Any]) -> str
```

Produce a short Python-style type string from a JSON-Schema fragment.


## _print_schema

`fastmcp.cli.client._print_schema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _print_schema(label: str, schema: dict[str, Any]) -> None
```

Print a JSON schema with a label.


## _resolve_json_spec

`fastmcp.cli.client._resolve_json_spec`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _resolve_json_spec(path: Path) -> str | dict[str, Any]
```

Disambiguate a ``.json`` server spec.


## _sanitize_untrusted_text

`fastmcp.cli.client._sanitize_untrusted_text`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _sanitize_untrusted_text(value: str) -> str
```

Escape rich markup and encode control chars for terminal-safe output.


## _terminal_elicitation_handler

`fastmcp.cli.client._terminal_elicitation_handler`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _terminal_elicitation_handler(message: str, response_type: type[Any] | None, params: Any, context: Any) -> ElicitResult[dict[str, Any]]
```

Prompt the user on the terminal for elicitation responses.

Prints the server's message and prompts for each field in the schema.
The user can type 'decline' or 'cancel' instead of a value to abort.


## _tools_to_json

`fastmcp.cli.client._tools_to_json`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _tools_to_json(tools: list[mcp_types.Tool]) -> list[dict[str, Any]]
```

Serialize a list of tools to JSON-safe dicts.


## call_command

Import as `fastmcp.cli.cli.call_command`  ·  defined at `fastmcp.cli.client.call_command`

```python
async def call_command(server_spec: Annotated[str | None, cyclopts.Parameter(help='Server URL, Python file, MCPConfig JSON, or .js file')] = None, target: Annotated[str, cyclopts.Parameter(help='Tool name, resource URI, or prompt name (with --prompt)')] = '', arguments: str = (), command: Annotated[str | None, cyclopts.Parameter(--command, help="Stdio command to connect to (e.g. 'npx -y @mcp/server')")] = None, transport: Annotated[Literal['http', 'sse'] | None, cyclopts.Parameter(name=[--transport, -t], help='Force transport type for URL targets (http or sse)')] = None, prompt: Annotated[bool, cyclopts.Parameter(--prompt, help='Treat target as a prompt name')] = False, input_json: Annotated[str | None, cyclopts.Parameter(--input - json, help='JSON string of arguments (merged with key=value args)')] = None, json_output: Annotated[bool, cyclopts.Parameter(--json, help='Output raw JSON result')] = False, timeout: Annotated[float | None, cyclopts.Parameter(--timeout, help='Connection timeout in seconds')] = None, auth: Annotated[str | None, cyclopts.Parameter(--auth, help="Auth method: 'oauth', a bearer token string, or 'none' to disable")] = None) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Call a tool, read a resource, or get a prompt on an MCP server.

By default the target is treated as a tool name. If the target
contains ``://`` it is treated as a resource URI. Pass ``--prompt``
to treat it as a prompt name.

Arguments are passed as key=value pairs. Use --input-json for complex
or nested arguments.

Examples:
    ```
    fastmcp call server.py greet name=World
    fastmcp call server.py resource://docs/readme
    fastmcp call server.py analyze --prompt data='[1,2,3]'
    fastmcp call http://server/mcp create --input-json '{"tags": ["a","b"]}'
    ```


## coerce_value

`fastmcp.cli.client.coerce_value`

```python
def coerce_value(raw: str, schema: dict[str, Any]) -> Any
```

Coerce a string CLI value according to a JSON-Schema type hint.


## discover_command

Import as `fastmcp.cli.cli.discover_command`  ·  defined at `fastmcp.cli.client.discover_command`

```python
async def discover_command(source: Annotated[list[str] | None, cyclopts.Parameter(--source, help='Only show servers from these sources (e.g. claude-code, cursor, gemini)')] = None, json_output: Annotated[bool, cyclopts.Parameter(--json, help='Output as JSON')] = False) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Discover MCP servers configured in editor and project configs.

Scans Claude Desktop, Claude Code, Cursor, Gemini CLI, Goose, and
project-level mcp.json files for MCP server definitions.

Discovered server names can be used directly with ``fastmcp list``
and ``fastmcp call`` instead of specifying a URL or file path.

Examples:
    fastmcp discover
    fastmcp discover --source claude-code
    fastmcp discover --source cursor --source gemini --json
    fastmcp list weather
    fastmcp call cursor:weather get_forecast city=London


## format_tool_signature

`fastmcp.cli.client.format_tool_signature`

```python
def format_tool_signature(tool: mcp_types.Tool) -> str
```

Build ``name(param: type, ...) -> return_type`` from a tool's JSON schemas.


## list_command

Import as `fastmcp.cli.cli.list_command`  ·  defined at `fastmcp.cli.client.list_command`

```python
async def list_command(server_spec: Annotated[str | None, cyclopts.Parameter(help='Server URL, Python file, MCPConfig JSON, or .js file')] = None, command: Annotated[str | None, cyclopts.Parameter(--command, help="Stdio command to connect to (e.g. 'npx -y @mcp/server')")] = None, transport: Annotated[Literal['http', 'sse'] | None, cyclopts.Parameter(name=[--transport, -t], help='Force transport type for URL targets (http or sse)')] = None, resources: Annotated[bool, cyclopts.Parameter(--resources, help='Also list resources')] = False, prompts: Annotated[bool, cyclopts.Parameter(--prompts, help='Also list prompts')] = False, input_schema: Annotated[bool, cyclopts.Parameter(--input - schema, help='Show full input schemas')] = False, output_schema: Annotated[bool, cyclopts.Parameter(--output - schema, help='Show full output schemas')] = False, json_output: Annotated[bool, cyclopts.Parameter(--json, help='Output as JSON')] = False, timeout: Annotated[float | None, cyclopts.Parameter(--timeout, help='Connection timeout in seconds')] = None, auth: Annotated[str | None, cyclopts.Parameter(--auth, help="Auth method: 'oauth', a bearer token string, or 'none' to disable")] = None) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

List tools available on an MCP server.

Examples:
    fastmcp list http://localhost:8000/mcp
    fastmcp list server.py
    fastmcp list mcp.json --json
    fastmcp list --command 'npx -y @mcp/server' --resources
    fastmcp list http://server/mcp --transport sse


## parse_tool_arguments

`fastmcp.cli.client.parse_tool_arguments`

```python
def parse_tool_arguments(raw_args: tuple[str, ...], input_json: str | None, input_schema: dict[str, Any]) -> dict[str, Any]
```

Build a tool-call argument dict from CLI inputs.

A single JSON object argument is treated as the full argument dict.
``--input-json`` provides the base dict; ``key=value`` pairs override.
Values are coerced using the tool's ``inputSchema``.


## resolve_server_spec

Import as `fastmcp.cli.generate.resolve_server_spec`  ·  defined at `fastmcp.cli.client.resolve_server_spec`

```python
def resolve_server_spec(server_spec: str | None, command: str | None = None, transport: str | None = None) -> str | dict[str, Any] | ClientTransport
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Turn CLI inputs into something ``Client()`` accepts.

Exactly one of ``server_spec`` or ``command`` should be provided.

Resolution order for ``server_spec``:
1. URLs (``http://``, ``https://``) — passed through as-is.
   If ``--transport`` is ``sse``, the URL is rewritten to end with ``/sse``
   so ``infer_transport`` picks the right transport.
2. Existing file paths, or strings ending in ``.py``/``.js``/``.json``.
3. Anything else — name-based resolution via ``resolve_name``.

When ``command`` is provided, the string is shell-split into a
``StdioTransport(command, args)``.


