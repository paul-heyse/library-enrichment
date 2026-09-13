//! RPC surface, worker supervision, scheduling, job state, and the CLI.
//!
//! The single writer and job owner. Supports multiple simultaneous agents without duplicate
//! builds, conflicting publications, or a language server per tool call (blueprint §2.1).
