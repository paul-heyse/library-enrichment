# `fastmcp.cli.generate`

Distribution: `fastmcp`

## _JSON_SCHEMA_TYPE_LABELS

`fastmcp.cli.generate._JSON_SCHEMA_TYPE_LABELS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_JSON_SCHEMA_TYPE_LABELS: dict[str, str] = {'string': 'string', 'integer': 'integer', 'number': 'number', 'boolean': 'boolean', 'null': 'null', 'array': 'array', 'object': 'object'}
```

## _SIMPLE_TYPES

`fastmcp.cli.generate._SIMPLE_TYPES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SIMPLE_TYPES = {'string', 'integer', 'number', 'boolean', 'null'}
```

## console

`fastmcp.cli.generate.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## logger

`fastmcp.cli.generate.logger`

```python
logger = get_logger('cli.generate')
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## _derive_server_name

`fastmcp.cli.generate._derive_server_name`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _derive_server_name(server_spec: str) -> str
```

Derive a human-friendly name from a server spec.


## _format_schema_for_help

`fastmcp.cli.generate._format_schema_for_help`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _format_schema_for_help(schema: dict[str, Any]) -> str
```

Format a JSON schema for display in help text.


## _is_simple_array

`fastmcp.cli.generate._is_simple_array`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_simple_array(schema: dict[str, Any]) -> tuple[bool, str | None]
```

Check if schema is an array of simple types.

Returns (is_simple_array, item_type_str).


## _is_simple_type

`fastmcp.cli.generate._is_simple_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_simple_type(schema: dict[str, Any]) -> bool
```

Check if a schema represents a simple (non-complex) type.


## _param_to_cli_flag

`fastmcp.cli.generate._param_to_cli_flag`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _param_to_cli_flag(prop_name: str) -> str
```

Convert a JSON Schema property name to its CLI flag form.

Replicates cyclopts' default_name_transform: camelCase → snake_case,
lowercase, underscores → hyphens, strip leading/trailing hyphens.


## _schema_to_python_type

`fastmcp.cli.generate._schema_to_python_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _schema_to_python_type(schema: dict[str, Any]) -> tuple[str, bool]
```

Convert a JSON Schema to a Python type annotation.

Returns (type_annotation, needs_json_parsing).


## _schema_type_label

`fastmcp.cli.generate._schema_type_label`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _schema_type_label(prop_schema: dict[str, Any]) -> str
```

Return a human-readable type label for a property schema.


## _to_python_identifier

`fastmcp.cli.generate._to_python_identifier`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _to_python_identifier(name: str) -> str
```

Sanitize a string into a valid Python identifier.


## _tool_function_source

`fastmcp.cli.generate._tool_function_source`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _tool_function_source(tool: mcp_types.Tool) -> str
```

Generate the source for a single ``@call_tool_app.command`` function.


## _tool_skill_section

`fastmcp.cli.generate._tool_skill_section`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _tool_skill_section(tool: mcp_types.Tool, cli_filename: str) -> str
```

Generate a SKILL.md section for a single tool.


## generate_cli_command

Import as `fastmcp.cli.cli.generate_cli_command`  ·  defined at `fastmcp.cli.generate.generate_cli_command`

```python
async def generate_cli_command(server_spec: Annotated[str, cyclopts.Parameter(help='Server URL, Python file, MCPConfig JSON, discovered name, or .js file')], output: Annotated[str, cyclopts.Parameter(help='Output file path (default: cli.py)')] = 'cli.py', force: Annotated[bool, cyclopts.Parameter(name=[-f, --force], help='Overwrite output file if it exists')] = False, timeout: Annotated[float | None, cyclopts.Parameter(--timeout, help='Connection timeout in seconds')] = None, auth: Annotated[str | None, cyclopts.Parameter(--auth, help="Auth method: 'oauth', a bearer token string, or 'none' to disable")] = None, no_skill: Annotated[bool, cyclopts.Parameter(--no - skill, help='Skip generating a SKILL.md agent skill alongside the CLI')] = False) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Generate a standalone CLI script from an MCP server.

Connects to the server, reads its tools/resources/prompts, and writes
a Python script that can invoke them directly. Also generates a SKILL.md
agent skill file unless --no-skill is passed.

Examples:
    fastmcp generate-cli weather
    fastmcp generate-cli weather my_cli.py
    fastmcp generate-cli http://localhost:8000/mcp
    fastmcp generate-cli server.py output.py -f
    fastmcp generate-cli weather --no-skill


## generate_cli_script

`fastmcp.cli.generate.generate_cli_script`

```python
def generate_cli_script(server_name: str, server_spec: str, transport_code: str, extra_imports: set[str], tools: list[mcp_types.Tool]) -> str
```

Generate the full CLI script source code.


## generate_skill_content

`fastmcp.cli.generate.generate_skill_content`

```python
def generate_skill_content(server_name: str, cli_filename: str, tools: list[mcp_types.Tool]) -> str
```

Generate a SKILL.md file for a generated CLI script.


## serialize_transport

`fastmcp.cli.generate.serialize_transport`

```python
def serialize_transport(resolved: str | dict[str, Any] | ClientTransport) -> tuple[str, set[str]]
```

Serialize a resolved transport to a Python expression string.

Returns ``(expression, extra_imports)`` where *extra_imports* is a set of
import lines needed by the expression.


