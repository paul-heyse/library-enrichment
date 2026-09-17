//! `codesearch-bridge` -- everything that extends DataFusion, in one place.
//!
//! This crate is the answer to "what have we added to the engine?". Every UDF, extension type,
//! provider wrapper, optimizer rule and session registration lives here and nowhere else, so the
//! inventory of custom functionality has a directory as its answer rather than a wiki page.
//!
//! Load-bearing facts about the host libraries, each established by an executed probe rather than
//! recalled, and each with a comment at its call site:
//!
//! - a Delta write needs delta-rs's own session, not a bare `SessionContext` (PB07);
//! - a builder handed a session silently discards it unless the fallback policy says otherwise
//!   (PB05);
//! - `ScalarUDFImpl` has no `as_any` at this DataFusion version, and requires `Eq + Hash`;
//! - a UDF that is not `Immutable` loses its filter entirely, with no diagnostic (PB06).

pub mod identity;
pub mod interaction;
pub mod lattice;
pub mod lazy;
pub mod projection;
pub mod provider;
pub mod rules;
pub mod session;
