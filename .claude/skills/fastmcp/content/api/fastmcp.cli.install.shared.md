# `fastmcp.cli.install.shared`

Distribution: `fastmcp`

## _SAFE_NAME_RE

`fastmcp.cli.install.shared._SAFE_NAME_RE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SAFE_NAME_RE = re.compile('^[\\w\\-. ]+$')
```

## logger

`fastmcp.cli.install.shared.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## open_deeplink

Import as `fastmcp.cli.install.goose.open_deeplink`  ·  defined at `fastmcp.cli.install.shared.open_deeplink`

```python
def open_deeplink(url: str, expected_scheme: str) -> bool
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Attempt to open a deeplink URL using the system's default handler.

Args:
    url: The deeplink URL to open.
    expected_scheme: The URL scheme to validate (e.g. "cursor", "goose").

Returns:
    True if the command succeeded, False otherwise.


## parse_env_var

`fastmcp.cli.install.shared.parse_env_var`

```python
def parse_env_var(env_var: str) -> tuple[str, str]
```

Parse environment variable string in format KEY=VALUE.


## process_common_args

Import as `fastmcp.cli.install.goose.process_common_args`  ·  defined at `fastmcp.cli.install.shared.process_common_args`

```python
async def process_common_args(server_spec: str, server_name: str | None, with_packages: list[str] | None, env_vars: list[str] | None, env_file: Path | None) -> tuple[Path, str | None, str, list[str], dict[str, str] | None]
```

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Process common arguments shared by all install commands.

Handles both fastmcp.json config files and traditional file.py:object syntax.


## validate_server_name

Import as `fastmcp.cli.install.gemini_cli.validate_server_name`  ·  defined at `fastmcp.cli.install.shared.validate_server_name`

```python
def validate_server_name(name: str) -> str
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Validate that a server name is safe for use as a subprocess argument.

Raises SystemExit if the name contains shell metacharacters.


