//! RPC surface, worker supervision, scheduling, job state, and the CLI.
//!
//! The single writer and job owner. Supports multiple simultaneous agents without duplicate
//! builds, conflicting publications, or a language server per tool call (blueprint §2.1).
//!
//! The transport is bounded newline-delimited JSON-RPC 2.0 over a Unix-domain socket -- a third
//! thing, neither MCP nor LSP framing. See [`rpc`] for the framing and
//! `docs/adr/0006-ndjson-rpc-transport.md` for the parts §2.1 left open.

pub mod paths;
pub mod rpc;
pub mod server;
pub mod status;
pub mod validate;
