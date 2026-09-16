# `fastmcp.cli.cimd`

Distribution: `fastmcp`

## cimd_app

Import as `fastmcp.cli.auth.cimd_app`  ·  defined at `fastmcp.cli.cimd.cimd_app`

```python
cimd_app = cyclopts.App(name='cimd', help='CIMD (Client ID Metadata Document) utilities for OAuth authentication.')
```

**Inferred type** (`ty`, not declared in the source): `App`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## console

`fastmcp.cli.cimd.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## logger

`fastmcp.cli.cimd.logger`

```python
logger = get_logger('cli.cimd')
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## create_command

`fastmcp.cli.cimd.create_command`

```python
def create_command(name: Annotated[str, cyclopts.Parameter(help='Human-readable name of the client application')], redirect_uri: Annotated[list[str], cyclopts.Parameter(name=['--redirect-uri', '-r'], help='Allowed redirect URIs (can specify multiple)')], client_id: Annotated[str | None, cyclopts.Parameter(name='--client-id', help='The URL where this document will be hosted (sets client_id directly)')] = None, client_uri: Annotated[str | None, cyclopts.Parameter(name='--client-uri', help="URL of the client's home page")] = None, logo_uri: Annotated[str | None, cyclopts.Parameter(name='--logo-uri', help="URL of the client's logo image")] = None, scope: Annotated[str | None, cyclopts.Parameter(name='--scope', help='Space-separated list of scopes the client may request')] = None, output: Annotated[str | None, cyclopts.Parameter(name=['--output', '-o'], help='Output file path (default: stdout)')] = None, pretty: Annotated[bool, cyclopts.Parameter(help='Pretty-print JSON output')] = True) -> None
```

Generate a CIMD document for hosting.

Create a Client ID Metadata Document that you can host at an HTTPS URL.
The URL where you host this document becomes your client_id.

Example:
    fastmcp cimd create --name "My App" -r "http://localhost:*/callback"

After creating the document, host it at an HTTPS URL with a non-root path,
for example: https://myapp.example.com/oauth/client.json


## validate_command

`fastmcp.cli.cimd.validate_command`

```python
def validate_command(url: Annotated[str, cyclopts.Parameter(help='URL of the CIMD document to validate')], timeout: Annotated[float, cyclopts.Parameter(name=['--timeout', '-t'], help='HTTP request timeout in seconds')] = 10.0) -> None
```

Validate a hosted CIMD document.

Fetches the document from the given URL and validates:
- URL is valid CIMD URL (HTTPS, non-root path)
- Document is valid JSON
- Document conforms to CIMD schema
- client_id in document matches the URL

Example:
    fastmcp cimd validate https://myapp.example.com/oauth/client.json


