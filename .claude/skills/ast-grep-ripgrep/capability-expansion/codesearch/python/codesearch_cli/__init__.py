"""codesearch -- a Typer front end over the Rust query binary.

This process never opens a Delta table. It shells out to ``codesearch-query``, which emits JSON,
and renders the result. Keeping the two apart means the CLI cannot hold a stale handle on tables a
build is rewriting, and it keeps the storage stack out of the Python dependency graph entirely.
"""

__all__ = ["app"]

from codesearch_cli.main import app
