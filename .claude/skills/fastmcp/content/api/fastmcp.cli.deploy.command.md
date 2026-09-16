# `fastmcp.cli.deploy.command`

Distribution: `fastmcp`

## HostOption

`fastmcp.cli.deploy.command.HostOption`

```python
HostOption = Annotated[str | None, Parameter(name='--host', help='Use and save a different Horizon host URL')]
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'typing.Annotated[str | None, <metadata>]'> ````

## JsonOption

`fastmcp.cli.deploy.command.JsonOption`

```python
JsonOption = Annotated[bool, Parameter(name='--json', help='Write one final JSON result to stdout', negative=())]
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'typing.Annotated[bool, <metadata>]'> ````

## _can_open_browser

`fastmcp.cli.deploy.command._can_open_browser`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _can_open_browser() -> bool
```

## _device_metadata

`fastmcp.cli.deploy.command._device_metadata`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _device_metadata() -> DeviceMetadata
```

## _fail

`fastmcp.cli.deploy.command._fail`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _fail(command: CommandName, category: ErrorCategory, message: str, json_output: bool, details: dict[str, object] | None = None) -> NoReturn
```

## _fail_for_expected_error

`fastmcp.cli.deploy.command._fail_for_expected_error`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _fail_for_expected_error(command: CommandName, error: Exception, json_output: bool) -> NoReturn
```

## _get_user

`fastmcp.cli.deploy.command._get_user`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _get_user(api_origin: str, credential: ResolvedCredential) -> HorizonUser
```

## _load_session_snapshot

`fastmcp.cli.deploy.command._load_session_snapshot`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _load_session_snapshot(credentials: CredentialStore) -> tuple[HorizonConfiguration, ResolvedCredential | None]
```

## login

Import as `fastmcp.cli.cli.login`  ·  defined at `fastmcp.cli.deploy.command.login`

```python
async def login(host: HostOption = None, json_output: JsonOption = False) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Sign in to Prefect Horizon.


## logout

Import as `fastmcp.cli.cli.logout`  ·  defined at `fastmcp.cli.deploy.command.logout`

```python
async def logout(json_output: JsonOption = False) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Revoke the current Horizon key and remove the local credential.


## whoami

Import as `fastmcp.cli.cli.whoami`  ·  defined at `fastmcp.cli.deploy.command.whoami`

```python
async def whoami(json_output: JsonOption = False) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Show the current Prefect Horizon user.


