"""A Typer CLI written the way it worked before Typer 0.26.0.

Every line here type-checks. `import click` resolves to whatever copy is installed for some other
reason, which is not the Click that Typer vendored and runs, so the plugin never fires.
"""

import click
import typer
from click import Command

app = typer.Typer()


@app.command()
def build(target: str) -> None:
    """Build [target]."""
    typer.echo(f"building {target}")


def as_click_command() -> Command:
    return typer.main.get_command(app)
