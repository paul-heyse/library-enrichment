# `fastmcp.cli.discovery`

Distribution: `fastmcp`

## logger

`fastmcp.cli.discovery.logger`

```python
logger = get_logger('cli.discovery')
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## DiscoveredServer

Import as `fastmcp.cli.client.DiscoveredServer`  ·  defined at `fastmcp.cli.discovery.DiscoveredServer`

```python
class DiscoveredServer
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `config: MCPServerTypes`  _instance-attribute_
- `config_path: Path`  _instance-attribute_
- `name: str`  _instance-attribute_
- `qualified_name: str`  _property_
  Fully qualified ``source:name`` identifier.
- `source: str`  _instance-attribute_
- `transport_summary: str`  _property_
  Human-readable one-liner describing the transport.

A single MCP server found in an editor or project config.


## _normalize_server_entry

`fastmcp.cli.discovery._normalize_server_entry`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _normalize_server_entry(entry: dict[str, Any]) -> dict[str, Any]
```

Normalize editor-specific server config fields to MCPConfig format.

Handles two known differences:
- Claude Code uses ``type`` where MCPConfig uses ``transport`` for
  remote servers.
- Gemini CLI uses ``httpUrl`` where MCPConfig uses ``url``.


## _parse_mcp_config

`fastmcp.cli.discovery._parse_mcp_config`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_mcp_config(path: Path, source: str) -> list[DiscoveredServer]
```

Parse an mcpServers-style JSON file into discovered servers.


## _parse_mcp_servers

`fastmcp.cli.discovery._parse_mcp_servers`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_mcp_servers(servers_dict: dict[str, Any], source: str, config_path: Path) -> list[DiscoveredServer]
```

Parse an ``mcpServers``-style dict into discovered servers.


## _scan_claude_code

`fastmcp.cli.discovery._scan_claude_code`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _scan_claude_code(start_dir: Path) -> list[DiscoveredServer]
```

Scan ``~/.claude.json`` for global and project-scoped MCP servers.


## _scan_claude_desktop

`fastmcp.cli.discovery._scan_claude_desktop`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _scan_claude_desktop() -> list[DiscoveredServer]
```

Scan the Claude Desktop config file.


## _scan_cursor_workspace

`fastmcp.cli.discovery._scan_cursor_workspace`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _scan_cursor_workspace(start_dir: Path) -> list[DiscoveredServer]
```

Walk up from *start_dir* looking for ``.cursor/mcp.json``.


## _scan_gemini

`fastmcp.cli.discovery._scan_gemini`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _scan_gemini(start_dir: Path) -> list[DiscoveredServer]
```

Scan Gemini CLI settings for MCP servers.

Checks both user-level ``~/.gemini/settings.json`` and project-level
``.gemini/settings.json``.


## _scan_goose

`fastmcp.cli.discovery._scan_goose`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _scan_goose() -> list[DiscoveredServer]
```

Scan Goose config for MCP server extensions.

Goose uses YAML (``~/.config/goose/config.yaml``) with a different
schema — MCP servers are defined as ``extensions`` with ``type: stdio``.


## _scan_project_mcp_json

`fastmcp.cli.discovery._scan_project_mcp_json`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _scan_project_mcp_json(start_dir: Path) -> list[DiscoveredServer]
```

Check for ``mcp.json`` in *start_dir*.


## discover_servers

Import as `fastmcp.cli.client.discover_servers`  ·  defined at `fastmcp.cli.discovery.discover_servers`

```python
def discover_servers(start_dir: Path | None = None) -> list[DiscoveredServer]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Run all scanners and return the combined results.

Duplicate names across sources are preserved — callers can
use :pyattr:`DiscoveredServer.qualified_name` to disambiguate.


## resolve_name

Import as `fastmcp.cli.client.resolve_name`  ·  defined at `fastmcp.cli.discovery.resolve_name`

```python
def resolve_name(name: str, start_dir: Path | None = None) -> ClientTransport
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Resolve a server name (or ``source:name``) to a transport.

Raises :class:`ValueError` when the name is not found or is ambiguous.


