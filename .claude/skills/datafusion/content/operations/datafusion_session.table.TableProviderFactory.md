# `datafusion_session::table::TableProviderFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.table.TableProviderFactory.json).

<a id="op-69227d35dfbf2f4aef45aae7"></a>
## TableProviderFactory

`trait` · `datafusion_session::table::TableProviderFactory` · datafusion-session 55.1.0

```rust
trait TableProviderFactory: Debug + Sync + Send
```

Source: `src/table.rs:566`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

A factory which creates [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e)s at runtime given a URL.

For example, this can be used to create a table "on the fly"
from a directory of files only when that name is referenced.

<a id="op-d5e17254158a8b0127ce545e"></a>
## create

`function` · `datafusion_session::table::TableProviderFactory::create` · datafusion-session 55.1.0

```rust
async fn create(&self, state: &dyn Session, cmd: &CreateExternalTable) -> Result<Arc<dyn TableProvider>>
```

Source: `src/table.rs:568`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Create a TableProvider with the given url
