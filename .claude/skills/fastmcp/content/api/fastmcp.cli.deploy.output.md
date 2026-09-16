# `fastmcp.cli.deploy.output`

Distribution: `fastmcp`

## CommandName

Import as `fastmcp.cli.deploy.command.CommandName`  ·  defined at `fastmcp.cli.deploy.output.CommandName`

```python
CommandName = Literal['login', 'logout', 'whoami']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["login", "logout", "whoami"]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## ErrorCategory

Import as `fastmcp.cli.deploy.command.ErrorCategory`  ·  defined at `fastmcp.cli.deploy.output.ErrorCategory`

```python
ErrorCategory = Literal['authentication_invalid', 'authentication_required', 'authorization_denied', 'authorization_expired', 'authorization_failed', 'horizon_error', 'horizon_unavailable', 'invalid_host', 'remote_revocation_failed', 'state_error']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["authentication_invalid", "authentication_required", "authorization_denied", "authorization_expired", "authorization_failed", ... omitted 5 literals]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## console

`fastmcp.cli.deploy.output.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## error_console

`fastmcp.cli.deploy.output.error_console`

```python
error_console = Console(stderr=True)
```

**Inferred type** (`ty`, not declared in the source): `Console`

## _account_panel

`fastmcp.cli.deploy.output._account_panel`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _account_panel(user: HorizonUser, title: str, message: str) -> Panel
```

## _banner

`fastmcp.cli.deploy.output._banner`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _banner(title: str, style: str) -> Panel
```

## _format_duration

`fastmcp.cli.deploy.output._format_duration`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _format_duration(seconds: int) -> str
```

## _write_json

`fastmcp.cli.deploy.output._write_json`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _write_json(payload: object, stderr: bool = False) -> None
```

## emit_device_challenge

Import as `fastmcp.cli.deploy.command.emit_device_challenge`  ·  defined at `fastmcp.cli.deploy.output.emit_device_challenge`

```python
def emit_device_challenge(authorization: DeviceAuthorization, json_output: bool) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Show a device challenge before polling starts.


## emit_environment_logout

Import as `fastmcp.cli.deploy.command.emit_environment_logout`  ·  defined at `fastmcp.cli.deploy.output.emit_environment_logout`

```python
def emit_environment_logout(json_output: bool) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Explain why logout cannot change an environment credential.


## emit_error

Import as `fastmcp.cli.deploy.command.emit_error`  ·  defined at `fastmcp.cli.deploy.output.emit_error`

```python
def emit_error(command: CommandName, category: ErrorCategory, message: str, json_output: bool, details: dict[str, object] | None = None) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Show a stable expected command failure.


## emit_identity

Import as `fastmcp.cli.deploy.command.emit_identity`  ·  defined at `fastmcp.cli.deploy.output.emit_identity`

```python
def emit_identity(command: Literal['login', 'whoami'], user: HorizonUser, json_output: bool) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Show the authenticated user.


## emit_logout

Import as `fastmcp.cli.deploy.command.emit_logout`  ·  defined at `fastmcp.cli.deploy.output.emit_logout`

```python
def emit_logout(remote_revoked: bool, json_output: bool) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Show a successful local logout result.


## start_device_approval_status

Import as `fastmcp.cli.deploy.command.start_device_approval_status`  ·  defined at `fastmcp.cli.deploy.output.start_device_approval_status`

```python
def start_device_approval_status(json_output: bool) -> Status | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Start the terminal spinner while the browser approval is pending.


## stop_device_approval_status

Import as `fastmcp.cli.deploy.command.stop_device_approval_status`  ·  defined at `fastmcp.cli.deploy.output.stop_device_approval_status`

```python
def stop_device_approval_status(status: Status | None) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Stop a device approval spinner when one is active.


