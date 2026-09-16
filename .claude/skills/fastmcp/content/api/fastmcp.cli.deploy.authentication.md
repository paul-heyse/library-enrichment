# `fastmcp.cli.deploy.authentication`

Distribution: `fastmcp`

## DeviceAuthorizationDeniedError

Import as `fastmcp.cli.deploy.command.DeviceAuthorizationDeniedError`  ·  defined at `fastmcp.cli.deploy.authentication.DeviceAuthorizationDeniedError`

```python
class DeviceAuthorizationDeniedError(DeviceAuthorizationError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `DeviceAuthorizationError`

The user denied the device authorization request.


## DeviceAuthorizationError

Import as `fastmcp.cli.deploy.command.DeviceAuthorizationError`  ·  defined at `fastmcp.cli.deploy.authentication.DeviceAuthorizationError`

```python
class DeviceAuthorizationError(RuntimeError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RuntimeError`

Device authorization did not complete.


## DeviceAuthorizationExpiredError

Import as `fastmcp.cli.deploy.command.DeviceAuthorizationExpiredError`  ·  defined at `fastmcp.cli.deploy.authentication.DeviceAuthorizationExpiredError`

```python
class DeviceAuthorizationExpiredError(DeviceAuthorizationError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `DeviceAuthorizationError`

The device authorization request expired.


## authorize_device

Import as `fastmcp.cli.deploy.command.authorize_device`  ·  defined at `fastmcp.cli.deploy.authentication.authorize_device`

```python
async def authorize_device(client: HorizonClient, metadata: DeviceMetadata | None = None, on_challenge: Callable[[DeviceAuthorization], None] | None = None, open_browser: bool = False, browser_opener: Callable[[str], object] = webbrowser.open, sleep: Callable[[float], Awaitable[None]] | None = None, monotonic: Callable[[], float] = time.monotonic) -> SecretStr
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create, present, and complete a Horizon device authorization.


## poll_device_authorization

`fastmcp.cli.deploy.authentication.poll_device_authorization`

```python
async def poll_device_authorization(client: HorizonClient, authorization: DeviceAuthorization, sleep: Callable[[float], Awaitable[None]] | None = None, monotonic: Callable[[], float] = time.monotonic) -> SecretStr
```

Poll at the server interval until the device request completes.


