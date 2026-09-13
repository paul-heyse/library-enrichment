"""Griffe extraction and isolated runtime probes.

Launched by the daemon with explicit inputs and a restricted environment. Emits
schema-versioned JSON records and cannot publish snapshots (blueprint §2.1).

The worker interpreter is never the analysed interpreter (gate P10). Griffe runs with
allow_inspection=False and explicit search_paths so static extraction never imports the
package under study (gate P02).
"""

__all__ = ["__version__"]
__version__ = "0.0.0"
