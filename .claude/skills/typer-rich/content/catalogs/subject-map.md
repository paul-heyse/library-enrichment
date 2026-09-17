# Three subjects, two pins

`typer._click` is indexed as its own subject although it ships inside the `typer`
distribution, because a row that cannot say which library it describes is worse than no
row -- `typer._click.Command` and `click.Command` are different classes with the same
name, and the one an agent imports is the wrong one.

| Subject | Distribution | Items | Modules |
|---|---|---|---|
| `typer` | typer 0.27.2 | 248 | 16 |
| `rich` | rich 15.0.0 | 553 | 76 |
| `click` | vendored in typer 0.27.2 | 201 | 14 |

Column 2 of `content/index/symbols.tsv` is the spelling to import; column 1 is where it
is defined. For Typer those differ constantly, and a definition path containing `_click`
is a warning rather than an address.
