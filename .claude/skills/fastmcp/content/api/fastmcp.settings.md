# `fastmcp.settings`

Distribution: `fastmcp`

## DuplicateBehavior

Import as `fastmcp.server.server.DuplicateBehaviorSetting`  ·  defined at `fastmcp.settings.DuplicateBehavior`

```python
DuplicateBehavior = Literal['warn', 'error', 'replace', 'ignore']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["warn", "error", "replace", "ignore"]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## ENV_FILE

`fastmcp.settings.ENV_FILE`

```python
ENV_FILE = os.getenv('FASTMCP_ENV_FILE', '.env')
```

**Inferred type** (`ty`, not declared in the source): `str`

## LOG_LEVEL

`fastmcp.settings.LOG_LEVEL`

```python
LOG_LEVEL = Literal['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]'> ````

## MCP_LOG_LEVEL

`fastmcp.settings.MCP_LOG_LEVEL`

```python
MCP_LOG_LEVEL = Literal['debug', 'info', 'notice', 'warning', 'error', 'critical', 'alert', 'emergency']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["debug", "info", "notice", "warning", "error", ... omitted 3 literals]'> ````

## TELEMETRY_MODE

Import as `fastmcp.telemetry.TelemetryMode`  ·  defined at `fastmcp.settings.TELEMETRY_MODE`

```python
TELEMETRY_MODE = Literal['native', 'propagation_only', 'off']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["native", "propagation_only", "off"]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## TEN_MB_IN_BYTES

`fastmcp.settings.TEN_MB_IN_BYTES`

```python
TEN_MB_IN_BYTES = 1024 * 1024 * 10
```

**Inferred type** (`ty`, not declared in the source): `Literal[10485760]`

## logger

`fastmcp.settings.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## Settings

Import as `fastmcp.Settings`  ·  defined at `fastmcp.settings.Settings`

```python
class Settings(BaseSettings)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseSettings`

**Declared members (36)**

- `check_for_updates: Annotated[Literal['stable', 'prerelease', 'off'], Field(description=inspect.cleandoc('\n                Controls update checking when displaying the CLI banner.\n                - "stable": Check for stable releases only (default)\n                - "prerelease": Also check for pre-release versions (alpha, beta, rc)\n                - "off": Disable update checking entirely\n                Set via FASTMCP_CHECK_FOR_UPDATES environment variable.\n                '))] = 'stable'`  _class-attribute, instance-attribute_
- `client_disconnect_timeout: Annotated[float, Field(description='Maximum time to wait for a clean disconnect before giving up, in seconds.')] = 5`  _class-attribute, instance-attribute_
- `client_init_timeout: Annotated[float | None, Field(description="The timeout for the client's initialization handshake, in seconds. Set to None or 0 to disable.")] = None`  _class-attribute, instance-attribute_
- `client_log_level: Annotated[MCP_LOG_LEVEL | None, Field(description=inspect.cleandoc('\n                Default minimum log level for messages sent to MCP clients.\n                When set, log messages below this level are suppressed.\n                Individual clients can override this per-session using the\n                MCP logging/setLevel request.\n                '))] = None`  _class-attribute, instance-attribute_
- `client_raise_first_exceptiongroup_error: Annotated[bool, Field(description=inspect.cleandoc('\n                Many MCP components operate in anyio taskgroups, and raise\n                ExceptionGroups instead of exceptions. If this setting is True, FastMCP Clients\n                will `raise` the first error in any ExceptionGroup instead of raising\n                the ExceptionGroup as a whole. This is useful for debugging, but may\n                mask other errors.\n                '))] = True`  _class-attribute, instance-attribute_
- `debug: bool = False`  _class-attribute, instance-attribute_
- `deprecation_warnings: Annotated[bool, Field(description=inspect.cleandoc("\n                Whether to show deprecation warnings. You can completely reset\n                Python's warning behavior by running `warnings.resetwarnings()`.\n                Note this will NOT apply to deprecation warnings from the\n                settings class itself.\n                "))] = True`  _class-attribute, instance-attribute_
- `enable_rich_logging: Annotated[bool, Field(description=inspect.cleandoc('\n                If True, will use rich formatting for log output. If False,\n                will use standard Python logging without rich formatting.\n                '))] = True`  _class-attribute, instance-attribute_
- `enable_rich_tracebacks: Annotated[bool, Field(description=inspect.cleandoc('\n                If True, will use rich tracebacks for logging.\n                '))] = True`  _class-attribute, instance-attribute_
- `def get_setting(self, attr: str) -> Any`
  Get a setting. If the setting contains one or more `__`, it will be treated as a nested setting.
- `home: Path = Path(user_data_dir('fastmcp', appauthor=False))`  _class-attribute, instance-attribute_
- `host: str = '127.0.0.1'`  _class-attribute, instance-attribute_
- `http_allowed_hosts: list[str] | None = None`  _class-attribute, instance-attribute_
- `http_allowed_origins: list[str] | None = None`  _class-attribute, instance-attribute_
- `http_host_origin_protection: bool | Literal['auto'] = False`  _class-attribute, instance-attribute_
- `http_session_idle_timeout: Annotated[float | None, Field(description=inspect.cleandoc("\n                Maximum time in seconds a streamable-HTTP session may remain\n                idle before it is terminated. A session's deadline is pushed\n                forward on every request. When None (default), sessions never\n                expire from inactivity. Not supported in stateless HTTP mode.\n                Must be a positive number of seconds when set.\n                "), gt=0)] = None`  _class-attribute, instance-attribute_
- `json_response: bool = False`  _class-attribute, instance-attribute_
- `log_enabled: bool = True`  _class-attribute, instance-attribute_
- `log_level: LOG_LEVEL = 'INFO'`  _class-attribute, instance-attribute_
- `mask_error_details: Annotated[bool, Field(description=inspect.cleandoc('\n                If True, error details from user-supplied functions (tool, resource, prompt)\n                will be masked before being sent to clients. Only error messages from explicitly\n                raised ToolError, ResourceError, or PromptError will be included in responses.\n                If False (default), all error details will be included in responses, but prefixed\n                with appropriate context.\n                '))] = False`  _class-attribute, instance-attribute_
- `mcp_camelcase_compat: Annotated[bool, Field(description=inspect.cleandoc('\n                Whether to install compatibility shims that let legacy\n                camelCase reads on MCP SDK objects (e.g. `tool.inputSchema`,\n                `result.isError`) keep working after the SDK v2 rename to\n                snake_case. Each bridged read emits a\n                `FastMCPDeprecationWarning`. Set to False to disable the shims\n                entirely, in which case only the snake_case names resolve.\n                '))] = True`  _class-attribute, instance-attribute_
- `message_path: str = '/messages/'`  _class-attribute, instance-attribute_
- `mounted_components_raise_on_load_error: Annotated[bool, Field(description=inspect.cleandoc('\n                If True, errors encountered when loading mounted components (tools, resources, prompts)\n                will be raised instead of logged as warnings. This is useful for debugging\n                but will interrupt normal operation.\n                '))] = False`  _class-attribute, instance-attribute_
- `def normalize_log_level(cls, v)`  _classmethod_
- `port: int = 8000`  _class-attribute, instance-attribute_
- `server_dependencies: list[str] = Field(default_factory=list, description='List of dependencies to install in the server environment')`  _class-attribute, instance-attribute_
- `def set_setting(self, attr: str, value: Any) -> None`
  Set a setting. If the setting contains one or more `__`, it will be treated as a nested setting.
- `show_server_banner: Annotated[bool, Field(description=inspect.cleandoc('\n                If True, the server banner will be displayed when running the server.\n                This setting can be overridden by the --no-banner CLI flag or by\n                passing show_banner=False to server.run().\n                Set to False via FASTMCP_SHOW_SERVER_BANNER=false to suppress the banner.\n                '))] = True`  _class-attribute, instance-attribute_
- `sse_path: str = '/sse'`  _class-attribute, instance-attribute_
- `ssrf_trust_proxy: Annotated[bool, Field(description=inspect.cleandoc('\n                Trust an outbound HTTP proxy for SSRF-protected fetches (OAuth client\n                metadata and JWKS). When False (default), FastMCP resolves the target\n                hostname itself and refuses to connect if it maps to a private,\n                loopback, link-local, or otherwise reserved IP. When True, FastMCP\n                routes auth metadata and JWKS fetches through the configured\n                HTTPS_PROXY/ALL_PROXY and does not honor NO_PROXY; if no proxy is\n                configured the fetch is refused (raising SSRFError) rather than sent\n                direct with the blocklist disabled. Only enable this when a trusted\n                corporate proxy is the mandated egress path: it shifts SSRF trust to\n                that proxy. Scheme (HTTPS-only) and hostname checks still apply.\n                '))] = False`  _class-attribute, instance-attribute_
- `stateless_http: bool = False`  _class-attribute, instance-attribute_
- `streamable_http_path: str = '/mcp'`  _class-attribute, instance-attribute_
- `strict_input_validation: Annotated[bool, Field(description=inspect.cleandoc('\n                If True, tool inputs are strictly validated against the input\n                JSON schema. For example, providing the string "10" to an\n                integer field will raise an error. If False, compatible inputs\n                will be coerced to match the schema, which can increase\n                compatibility. For example, providing the string "10" to an\n                integer field will be coerced to 10. Defaults to False.\n                '))] = False`  _class-attribute, instance-attribute_
- `telemetry_mode: Annotated[TELEMETRY_MODE, Field(description=inspect.cleandoc("\n                Controls FastMCP's native OpenTelemetry instrumentation.\n\n                - `native` (default): FastMCP creates MCP spans and propagates\n                  trace context through request `_meta`. FastMCP uses only the\n                  OpenTelemetry API, so span creation is a no-op with negligible\n                  overhead unless an SDK and exporter are configured.\n                - `propagation_only`: FastMCP still injects and extracts trace\n                  context, and still parents downstream spans from the incoming\n                  `_meta` context, but creates none of its own MCP spans. Use\n                  this when another instrumentation layer owns the MCP span\n                  hierarchy and FastMCP's spans would duplicate it.\n                - `off`: FastMCP's span helpers become a transparent\n                  pass-through. No spans are created even when an SDK is\n                  configured, and the surrounding OTel context is left\n                  untouched — no trace context is extracted or attached.\n                "))] = 'native'`  _class-attribute, instance-attribute_
- `test_mode: bool = False`  _class-attribute, instance-attribute_
- `transport: Literal['stdio', 'http', 'sse', 'streamable-http'] = 'stdio'`  _class-attribute, instance-attribute_

FastMCP settings.


