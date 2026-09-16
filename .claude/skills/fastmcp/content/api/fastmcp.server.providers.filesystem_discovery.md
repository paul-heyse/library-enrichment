# `fastmcp.server.providers.filesystem_discovery`

Distribution: `fastmcp`

## logger

`fastmcp.server.providers.filesystem_discovery.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## DiscoveryResult

`fastmcp.server.providers.filesystem_discovery.DiscoveryResult`

```python
class DiscoveryResult
```

**Declared members (2)**

- `components: list[tuple[Path, FastMCPComponent]] = field(default_factory=list)`  _class-attribute, instance-attribute_
- `failed_files: dict[Path, str] = field(default_factory=dict)`  _class-attribute, instance-attribute_

Result of filesystem discovery.


## _compute_module_name

`fastmcp.server.providers.filesystem_discovery._compute_module_name`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _compute_module_name(file_path: Path, package_root: Path) -> str
```

Compute the dotted module name for a file within a package.

Args:
    file_path: Path to the Python file.
    package_root: Root directory of the package.

Returns:
    Dotted module name (e.g., "mcp.tools.greet").


## _find_package_root

`fastmcp.server.providers.filesystem_discovery._find_package_root`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _find_package_root(file_path: Path, stop_at: Path | None = None) -> Path | None
```

Find the root of the package containing this file.

Walks up the directory tree until we find a directory without __init__.py,
but never above stop_at (the provider root). This prevents escaping into
ancestor packages when the provider is nested inside a larger Python project.

Args:
    file_path: Path to the Python file.
    stop_at: Do not walk above this directory. Typically the provider root.

Returns:
    The package root directory, or None if not in a package.


## _is_package_dir

`fastmcp.server.providers.filesystem_discovery._is_package_dir`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_package_dir(directory: Path) -> bool
```

Check if a directory is a Python package (has __init__.py).


## _package_path_matches

`fastmcp.server.providers.filesystem_discovery._package_path_matches`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _package_path_matches(module: ModuleType, package_root: Path) -> bool
```

Check whether a package module's __path__ points at package_root.

Used to tell whether a top-level package name already present in
sys.modules belongs to this provider (same directory) or to a different
provider that happens to share the package name.


## _private_package_prefix

`fastmcp.server.providers.filesystem_discovery._private_package_prefix`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _private_package_prefix(directory: Path) -> str
```

Compute a collision-safe synthetic package name anchored at a directory.


## discover_and_import

Import as `fastmcp.server.providers.filesystem.discover_and_import`  ·  defined at `fastmcp.server.providers.filesystem_discovery.discover_and_import`

```python
def discover_and_import(root: Path) -> DiscoveryResult
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Discover files, import modules, and extract components.

This is the main entry point for filesystem-based discovery.

Args:
    root: Root directory to scan.

Returns:
    DiscoveryResult with components and any failed files.

Note:
    Files that fail to import are tracked in failed_files, not logged.
    The caller is responsible for logging/handling failures.
    Files with no components are silently skipped.


## discover_files

`fastmcp.server.providers.filesystem_discovery.discover_files`

```python
def discover_files(root: Path) -> list[Path]
```

Recursively discover all Python files under a directory.

Excludes __init__.py files (they're for package structure, not components).

Args:
    root: Root directory to scan.

Returns:
    List of .py file paths, sorted for deterministic order.


## extract_components

`fastmcp.server.providers.filesystem_discovery.extract_components`

```python
def extract_components(module: ModuleType) -> list[FastMCPComponent]
```

Extract all MCP components from a module.

Scans all module attributes for instances of Tool, Resource,
ResourceTemplate, or Prompt objects created by standalone decorators,
or functions decorated with @tool/@resource/@prompt that have __fastmcp__ metadata.

Args:
    module: The imported module to scan.

Returns:
    List of component objects (Tool, Resource, ResourceTemplate, Prompt).


## import_module_from_file

`fastmcp.server.providers.filesystem_discovery.import_module_from_file`

```python
def import_module_from_file(file_path: Path, provider_root: Path | None = None) -> ModuleType
```

Import a Python file as a module.

If the file is part of a package (directory has __init__.py), imports
it as a proper package member (relative imports work). Otherwise,
imports directly using spec_from_file_location.

sys.path is modified only for the duration of the import and restored
immediately after, so no permanent pollution occurs.

Args:
    file_path: Path to the Python file.
    provider_root: The provider's root directory. Prevents package root
        discovery from walking above this boundary into ancestor packages.

Returns:
    The imported module.

Raises:
    ImportError: If the module cannot be imported.


