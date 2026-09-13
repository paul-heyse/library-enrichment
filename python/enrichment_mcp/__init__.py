"""Thin FastMCP 4 adapter over the enrichment daemon.

Validates inputs, calls the daemon, emits MCP results, and maps structured errors. It does not
index libraries and owns no persistent state (blueprint §2.1).

stdout is the MCP protocol channel. Every log, warning and traceback goes to stderr or the
daemon log; a stray print() corrupts the session (blueprint §7.4).
"""

__all__ = ["__version__"]
__version__ = "0.0.0"
