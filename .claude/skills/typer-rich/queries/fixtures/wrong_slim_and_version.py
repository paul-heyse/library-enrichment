"""Two habits that stopped paying off.

`typer_slim` has been a shallow wrapper around `typer` since 0.22.0 and pulls Rich regardless.
`rich.__version__` was removed by the lazy-loading change in 14.3.4 and now raises AttributeError
at runtime -- with no complaint from any type checker.
"""

import rich
import typer_slim


def banner() -> str:
    return f"rich {rich.__version__}"


def main() -> None:
    typer_slim.run(lambda: None)
