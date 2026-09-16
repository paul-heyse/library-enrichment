# `fastmcp.cli.deploy.state`

Distribution: `fastmcp`

## ModelT

`fastmcp.cli.deploy.state.ModelT`

```python
ModelT = TypeVar('ModelT', bound=BaseModel)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## _WINDOWS_ACL_SCRIPT

`fastmcp.cli.deploy.state._WINDOWS_ACL_SCRIPT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_WINDOWS_ACL_SCRIPT = '\n$ErrorActionPreference = "Stop"\n$path = $env:FASTMCP_STATE_PATH\n$sid = [System.Security.Principal.WindowsIdentity]::GetCurrent().User\n$acl = Get-Acl -LiteralPath $path\n$acl.SetAccessRuleProtection($true, $false)\nforeach ($existingRule in @($acl.Access)) {\n    $acl.RemoveAccessRuleSpecific($existingRule)\n}\n\nif ([System.IO.Directory]::Exists($path)) {\n    $inheritance = [System.Security.AccessControl.InheritanceFlags]::ContainerInherit `\n        -bor [System.Security.AccessControl.InheritanceFlags]::ObjectInherit\n    $rule = [System.Security.AccessControl.FileSystemAccessRule]::new(\n        $sid,\n        [System.Security.AccessControl.FileSystemRights]::FullControl,\n        $inheritance,\n        [System.Security.AccessControl.PropagationFlags]::None,\n        [System.Security.AccessControl.AccessControlType]::Allow\n    )\n} else {\n    $rule = [System.Security.AccessControl.FileSystemAccessRule]::new(\n        $sid,\n        [System.Security.AccessControl.FileSystemRights]::FullControl,\n        [System.Security.AccessControl.AccessControlType]::Allow\n    )\n}\n\n$acl.AddAccessRule($rule)\nSet-Acl -LiteralPath $path -AclObject $acl\n'
```

## StateFileError

Import as `fastmcp.cli.deploy.command.StateFileError`  ·  defined at `fastmcp.cli.deploy.state.StateFileError`

```python
class StateFileError(RuntimeError)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RuntimeError`

A CLI state file could not be read or written safely.


## _prepare_directory

`fastmcp.cli.deploy.state._prepare_directory`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _prepare_directory(path: Path) -> None
```

## _restrict_access

`fastmcp.cli.deploy.state._restrict_access`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _restrict_access(path: Path, directory: bool = False) -> None
```

## _restrict_windows_access

`fastmcp.cli.deploy.state._restrict_windows_access`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _restrict_windows_access(path: Path) -> None
```

## read_state

Import as `fastmcp.cli.deploy.credentials.read_state`  ·  defined at `fastmcp.cli.deploy.state.read_state`

```python
def read_state(path: Path, model: type[ModelT], secret: bool = False) -> ModelT | None
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Read and validate a versioned JSON state file.


## remove_state

Import as `fastmcp.cli.deploy.credentials.remove_state`  ·  defined at `fastmcp.cli.deploy.state.remove_state`

```python
def remove_state(path: Path) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Remove a state file when it exists.


## state_lock

Import as `fastmcp.cli.deploy.command.state_lock`  ·  defined at `fastmcp.cli.deploy.state.state_lock`

```python
def state_lock(directory: Path) -> Iterator[None]
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Lock related CLI state changes across processes.


## write_state

Import as `fastmcp.cli.deploy.credentials.write_state`  ·  defined at `fastmcp.cli.deploy.state.write_state`

```python
def write_state(path: Path, data: dict[str, Any]) -> None
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Write JSON through a restricted temporary file and atomic replacement.


