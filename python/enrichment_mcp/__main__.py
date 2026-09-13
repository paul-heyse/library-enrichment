"""Entry point for ``library-enrichment-mcp``.

Declared in ``pyproject.toml`` as ``library-enrichment-mcp = "enrichment_mcp.__main__:main"``
and registered with a client by absolute path -- see ``docs/operations/README.md``.

Startup does nothing but build the server and serve stdio: no daemon connection, no package
fetch, no compilation, no language server (blueprint §2.2, acceptance gate C14).
"""

from __future__ import annotations

from enrichment_mcp.server import build_server


def main() -> None:
    """Serve the MCP tool catalog over stdio."""
    build_server().run()


if __name__ == "__main__":
    main()
