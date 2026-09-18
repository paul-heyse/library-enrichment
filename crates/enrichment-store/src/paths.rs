//! Where regenerable cache and retained evidence live (blueprint §2.3).
//!
//! Two roots, resolved separately: the cache root (`downloads/`, `capsules/`) and
//! the data root (`blobs/`, `snapshots/`, `contexts/`). Production resolves to XDG paths;
//! development is redirected to the gitignored `.dev-state/` by `scripts/env.sh`.
//!
//! The same rule as the daemon's socket resolver applies: **nothing resolves relative to the
//! working directory**. A relative override is an error and an empty environment is an error,
//! because the working directory of an operator or an agent is very often a repository under
//! study, and writing service state into one is the breach gate C20 exists to catch.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// Resolved state roots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatePaths {
    /// Regenerable data: downloads, unpacked archives, capsules.
    pub cache_root: PathBuf,
    /// Retained evidence: blobs, snapshots, contexts.
    pub data_root: PathBuf,
}

/// Why a root could not be resolved.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum StatePathError {
    /// Every source for this root was unset.
    #[error(
        "cannot locate the {which} directory: none of {variables} is set. Set LIBENR_HOME to an \
         absolute path."
    )]
    NoDirectory {
        /// `cache` or `data`.
        which: &'static str,
        /// The variables consulted, for the message.
        variables: &'static str,
    },
    /// A configured path was relative.
    #[error(
        "{variable} must be an absolute path, got `{value}`. A relative path resolves against \
         the working directory, which may be a repository under study (blueprint §2.3)."
    )]
    NotAbsolute {
        /// The variable at fault.
        variable: &'static str,
        /// Its value.
        value: String,
    },
}

const CACHE_SOURCES: [(&str, &str); 4] = [
    ("LIBENR_CACHE_HOME", ""),
    ("LIBENR_HOME", "cache"),
    ("XDG_CACHE_HOME", "library-enrichment"),
    ("HOME", ".cache/library-enrichment"),
];

const DATA_SOURCES: [(&str, &str); 4] = [
    ("LIBENR_DATA_HOME", ""),
    ("LIBENR_HOME", "data"),
    ("XDG_DATA_HOME", "library-enrichment"),
    ("HOME", ".local/share/library-enrichment"),
];

impl StatePaths {
    /// Resolve from the process environment.
    ///
    /// # Errors
    ///
    /// Fails if a root has no source set, or if the first source found is a relative path.
    pub fn from_env() -> Result<Self, StatePathError> {
        Self::from_lookup(|key| std::env::var_os(key))
    }

    /// Resolve from an arbitrary lookup, so tests never touch the process environment.
    ///
    /// # Errors
    ///
    /// As [`Self::from_env`].
    pub fn from_lookup(lookup: impl Fn(&str) -> Option<OsString>) -> Result<Self, StatePathError> {
        let cache_root = resolve(&lookup, &CACHE_SOURCES, "cache")?;
        let data_root = resolve(&lookup, &DATA_SOURCES, "data")?;
        Ok(Self {
            cache_root,
            data_root,
        })
    }

    /// Use explicit roots. The daemon calls this once after resolving; tests use temp dirs.
    #[must_use]
    pub fn explicit(cache_root: impl Into<PathBuf>, data_root: impl Into<PathBuf>) -> Self {
        Self {
            cache_root: cache_root.into(),
            data_root: data_root.into(),
        }
    }

    /// Raw downloads, keyed by digest.
    #[must_use]
    pub fn downloads(&self) -> PathBuf {
        self.cache_root.join("downloads")
    }

    /// Content-addressed blobs.
    #[must_use]
    pub fn blobs(&self) -> PathBuf {
        self.data_root.join("blobs")
    }
}

fn resolve(
    lookup: &impl Fn(&str) -> Option<OsString>,
    sources: &[(&'static str, &str)],
    which: &'static str,
) -> Result<PathBuf, StatePathError> {
    for (variable, suffix) in sources {
        let Some(value) = lookup(variable).filter(|v| !v.is_empty()) else {
            continue;
        };
        let base = PathBuf::from(value);
        if !base.is_absolute() {
            return Err(StatePathError::NotAbsolute {
                variable,
                value: base.display().to_string(),
            });
        }
        return Ok(if suffix.is_empty() {
            base
        } else {
            base.join(suffix)
        });
    }
    Err(StatePathError::NoDirectory {
        which,
        variables: if which == "cache" {
            "LIBENR_CACHE_HOME, LIBENR_HOME, XDG_CACHE_HOME or HOME"
        } else {
            "LIBENR_DATA_HOME, LIBENR_HOME, XDG_DATA_HOME or HOME"
        },
    })
}

/// Whether `path` is inside `root` after lexical normalization. Used to keep every write
/// inside the resolved roots.
#[must_use]
pub fn is_within(root: &Path, path: &Path) -> bool {
    path.starts_with(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolve_with(vars: &[(&str, &str)]) -> Result<StatePaths, StatePathError> {
        StatePaths::from_lookup(|key| {
            vars.iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| OsString::from(v))
        })
    }

    #[test]
    fn libenr_home_yields_both_roots() {
        let paths = resolve_with(&[("LIBENR_HOME", "/srv/libenr")]).expect("resolves");
        assert_eq!(paths.cache_root, PathBuf::from("/srv/libenr/cache"));
        assert_eq!(paths.data_root, PathBuf::from("/srv/libenr/data"));
        assert_eq!(paths.blobs(), PathBuf::from("/srv/libenr/data/blobs"));
    }

    #[test]
    fn the_specific_overrides_win_over_libenr_home() {
        let paths = resolve_with(&[
            ("LIBENR_HOME", "/srv/libenr"),
            ("LIBENR_CACHE_HOME", "/fast/cache"),
            ("LIBENR_DATA_HOME", "/durable/data"),
        ])
        .expect("resolves");
        assert_eq!(paths.cache_root, PathBuf::from("/fast/cache"));
        assert_eq!(paths.data_root, PathBuf::from("/durable/data"));
    }

    #[test]
    fn xdg_then_home_are_the_production_fallbacks() {
        let paths = resolve_with(&[
            ("XDG_CACHE_HOME", "/home/u/.cache"),
            ("XDG_DATA_HOME", "/home/u/.local/share"),
        ])
        .expect("resolves");
        assert_eq!(
            paths.cache_root,
            PathBuf::from("/home/u/.cache/library-enrichment")
        );
        assert_eq!(
            paths.data_root,
            PathBuf::from("/home/u/.local/share/library-enrichment")
        );

        let paths = resolve_with(&[("HOME", "/home/u")]).expect("resolves");
        assert_eq!(
            paths.cache_root,
            PathBuf::from("/home/u/.cache/library-enrichment")
        );
        assert_eq!(
            paths.data_root,
            PathBuf::from("/home/u/.local/share/library-enrichment")
        );
    }

    #[test]
    fn a_relative_root_is_refused() {
        assert_eq!(
            resolve_with(&[("LIBENR_HOME", ".dev-state")]),
            Err(StatePathError::NotAbsolute {
                variable: "LIBENR_HOME",
                value: ".dev-state".to_owned(),
            })
        );
    }

    #[test]
    fn an_empty_environment_fails_rather_than_using_the_working_directory() {
        assert!(matches!(
            resolve_with(&[]),
            Err(StatePathError::NoDirectory { which: "cache", .. })
        ));
    }

    #[test]
    fn an_empty_value_is_treated_as_unset() {
        let paths = resolve_with(&[("LIBENR_CACHE_HOME", ""), ("LIBENR_HOME", "/x")]).expect("ok");
        assert_eq!(paths.cache_root, PathBuf::from("/x/cache"));
    }
}
