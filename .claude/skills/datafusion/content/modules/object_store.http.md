# `object_store::http`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.http.json).

<a id="op-864ea065fc3b64d53597d706"></a>
## http

`module` · `object_store::http` · object_store 0.13.2

```rust
mod http
```

Source: `src/http/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An object store implementation for generic HTTP servers

This follows [rfc2518] commonly known as [WebDAV]

Basic get support will work out of the box with most HTTP servers,
even those that don't explicitly support [rfc2518]

Other operations such as list, delete, copy, etc... will likely
require server-side configuration. A list of HTTP servers with support
can be found [here](https://wiki.archlinux.org/title/WebDAV#Server)

Multipart uploads are not currently supported

[rfc2518]: https://datatracker.ietf.org/doc/html/rfc2518
[WebDAV]: https://en.wikipedia.org/wiki/WebDAV
