//! `enr_fixture` — a deterministic fixture for library-enrichment acceptance gates.
//!
//! Every item here exists to exercise one gate:
//!
//! - [`Widget`] is re-exported from [`inner`] (gate R07: one definition, two public paths).
//! - [`Shape`] is a trait with an impl for [`Widget`] (relationships).
//! - [`extra_only`] is gated behind the `extra` feature (gate R05).
//! - [`unix_only`] is gated behind `cfg(unix)` (gate R06).
//! - [`inner::Widget::create`] is deprecated.
//! - [`perimeter`] was added in 0.2.0 without breaking anything (an additive change).
//!
//! # Example
//!
//! ```
//! use enr_fixture::{Shape, Widget};
//! let w = Widget::new(3);
//! assert_eq!(w.area(), 9);
//! ```

/// Types that are also reachable at the crate root through a re-export.
pub mod inner {
    /// A square widget.
    ///
    /// Reachable as both `enr_fixture::Widget` and `enr_fixture::inner::Widget`; the two paths
    /// name one definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Widget {
        /// Side length.
        pub size: u32,
    }

    impl Widget {
        /// Construct a widget with the given side length.
        pub fn new(size: u32) -> Self {
            Self { size }
        }

        /// Older constructor, kept for compatibility.
        #[deprecated(since = "0.1.0", note = "use `Widget::new`")]
        pub fn create(size: u32) -> Self {
            Self::new(size)
        }
    }
}

pub use inner::Widget;

/// Anything with an area.
pub trait Shape {
    /// The area in square units.
    fn area(&self) -> u32;
}

impl Shape for Widget {
    fn area(&self) -> u32 {
        self.size * self.size
    }
}

/// Describe a widget in one line.
///
/// Since 0.2.0 the description is trimmed of surrounding whitespace. The signature is
/// unchanged; the behaviour is not (a behaviour-only release note).
pub fn describe(widget: &Widget) -> String {
    format!("Widget of size {}", widget.size).trim().to_owned()
}

/// The perimeter of a widget. Added in 0.2.0.
pub fn perimeter(widget: &Widget) -> u32 {
    widget.size * 4
}

/// Only present when the `extra` feature is enabled.
///
/// docs.rs builds this crate with `all-features = true`, so hosted documentation shows this
/// function even though a project with default features does not have it.
#[cfg(feature = "extra")]
pub fn extra_only() -> &'static str {
    "extra"
}

/// Only present on Unix targets.
#[cfg(unix)]
pub fn unix_only() -> bool {
    true
}
