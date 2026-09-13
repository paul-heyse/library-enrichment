//! Where the daemon's socket, lock and log live.
//!
//! Blueprint §2.3 requires an OS-aware resolver with overrides rather than hardcoded Linux
//! paths, and says only this about the socket: "Keep the socket path short and private to the
//! user." It does not name a path, so the order below is this implementation's decision --
//! recorded in `docs/adr/0006-ndjson-rpc-transport.md`.
//!
//! Nothing here ever resolves inside a repository under study. During development
//! `scripts/env.sh` sets `LIBENR_HOME` to the gitignored `.dev-state/`, so a test run cannot
//! touch real user state; `just state-leak-check` proves it.

use std::path::PathBuf;

/// The socket filename. Short on purpose: Unix socket paths are capped near 108 bytes.
const SOCKET_FILE: &str = "d.sock";

/// Resolved locations for one daemon instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaemonPaths {
    /// The Unix-domain socket adapters connect to.
    pub socket: PathBuf,
}

/// Why a location could not be resolved.
///
/// Resolution fails loudly rather than falling back to somewhere plausible. A relative path
/// would resolve against the daemon's working directory, which for an operator or an agent is
/// very often a repository under study -- and "a repository under study is never a subprocess
/// working directory or an extraction destination" (§2.3) is the invariant gate C20 exists to
/// prove. A daemon that will not start is a much better outcome than one that quietly writes a
/// socket into someone's checkout.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PathError {
    /// Every source was unset, so there is nowhere private to put the socket.
    #[error(
        "cannot locate a private runtime directory: none of LIBENR_SOCKET, LIBENR_HOME, \
         XDG_RUNTIME_DIR, XDG_CACHE_HOME or HOME is set. Set LIBENR_SOCKET to an absolute path."
    )]
    NoRuntimeDirectory,
    /// A configured path was relative, which would resolve against the working directory.
    #[error(
        "{variable} must be an absolute path, got `{value}`. A relative path resolves against \
         the working directory, which may be a repository under study (blueprint §2.3)."
    )]
    NotAbsolute {
        /// The environment variable at fault.
        variable: &'static str,
        /// What it was set to.
        value: String,
    },
}

impl DaemonPaths {
    /// Resolve from the environment.
    ///
    /// In order: `LIBENR_SOCKET` (explicit override), `LIBENR_HOME` (the development sandbox),
    /// `XDG_RUNTIME_DIR` (the correct home for a user-private socket on Linux), then
    /// `XDG_CACHE_HOME`, then `~/.cache`.
    ///
    /// # Errors
    ///
    /// Fails if every source is unset, or if one is set to a relative path. There is
    /// deliberately no fallback to the working directory -- see [`PathError`].
    pub fn from_env() -> Result<Self, PathError> {
        if let Some(socket) = env_path("LIBENR_SOCKET") {
            return if socket.is_absolute() {
                Ok(Self { socket })
            } else {
                Err(PathError::NotAbsolute {
                    variable: "LIBENR_SOCKET",
                    value: socket.display().to_string(),
                })
            };
        }
        Ok(Self::under(Self::runtime_dir()?))
    }

    /// Resolve the socket under one directory.
    #[must_use]
    pub fn under(dir: PathBuf) -> Self {
        Self {
            socket: dir.join(SOCKET_FILE),
        }
    }

    fn runtime_dir() -> Result<PathBuf, PathError> {
        for (variable, suffix) in [
            ("LIBENR_HOME", "run"),
            ("XDG_RUNTIME_DIR", "library-enrichment"),
            ("XDG_CACHE_HOME", "library-enrichment/run"),
            ("HOME", ".cache/library-enrichment/run"),
        ] {
            let Some(base) = env_path(variable) else {
                continue;
            };
            if !base.is_absolute() {
                return Err(PathError::NotAbsolute {
                    variable,
                    value: base.display().to_string(),
                });
            }
            return Ok(base.join(suffix));
        }
        Err(PathError::NoRuntimeDirectory)
    }

    /// Create the containing directory, private to the user.
    ///
    /// Mode `0700`: the socket carries research requests and the daemon is the single writer.
    pub fn ensure_dir(&self) -> std::io::Result<()> {
        let Some(dir) = self.socket.parent() else {
            return Ok(());
        };
        std::fs::create_dir_all(dir)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700))?;
        }
        Ok(())
    }
}

fn env_path(key: &str) -> Option<PathBuf> {
    match std::env::var_os(key) {
        Some(value) if !value.is_empty() => Some(PathBuf::from(value)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    /// `std::env` is process-global, so these tests must not interleave.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    /// Resolve with exactly the given variables set and every other source cleared.
    fn resolve_with(vars: &[(&str, &str)]) -> Result<DaemonPaths, PathError> {
        const ALL: [&str; 5] = [
            "LIBENR_SOCKET",
            "LIBENR_HOME",
            "XDG_RUNTIME_DIR",
            "XDG_CACHE_HOME",
            "HOME",
        ];
        let _guard = ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let saved: Vec<_> = ALL.iter().map(|k| (*k, std::env::var_os(k))).collect();

        // SAFETY: serialized by ENV_LOCK, and every variable is restored below.
        unsafe {
            for key in ALL {
                std::env::remove_var(key);
            }
            for (key, value) in vars {
                std::env::set_var(key, value);
            }
        }
        let result = DaemonPaths::from_env();
        unsafe {
            for (key, value) in saved {
                match value {
                    Some(v) => std::env::set_var(key, v),
                    None => std::env::remove_var(key),
                }
            }
        }
        result
    }

    #[test]
    fn the_socket_sits_under_the_given_directory() {
        let paths = DaemonPaths::under(PathBuf::from("/run/libenr"));
        assert_eq!(paths.socket, PathBuf::from("/run/libenr/d.sock"));
    }

    #[test]
    fn a_relative_socket_override_is_refused() {
        // Would otherwise resolve against the working directory, which may be a repository
        // under study. Blueprint §2.3, gate C20.
        let err = DaemonPaths::under(PathBuf::from("relative/dir"));
        assert!(
            !err.socket.is_absolute(),
            "this helper does not validate; from_env does"
        );

        assert_eq!(
            resolve_with(&[("LIBENR_SOCKET", "relative/d.sock")]),
            Err(PathError::NotAbsolute {
                variable: "LIBENR_SOCKET",
                value: "relative/d.sock".to_owned(),
            })
        );
    }

    #[test]
    fn a_relative_runtime_base_is_refused() {
        assert_eq!(
            resolve_with(&[("LIBENR_HOME", ".dev-state")]),
            Err(PathError::NotAbsolute {
                variable: "LIBENR_HOME",
                value: ".dev-state".to_owned(),
            })
        );
    }

    #[test]
    fn an_empty_environment_fails_rather_than_using_the_working_directory() {
        // The regression this replaces: a `PathBuf::from(".library-enrichment/run")` fallback
        // put a live socket inside whatever directory the daemon happened to start in.
        assert_eq!(resolve_with(&[]), Err(PathError::NoRuntimeDirectory));
    }

    #[test]
    fn every_resolved_path_is_absolute() {
        for source in [
            "LIBENR_SOCKET",
            "LIBENR_HOME",
            "XDG_RUNTIME_DIR",
            "XDG_CACHE_HOME",
            "HOME",
        ] {
            let value = if source == "LIBENR_SOCKET" {
                "/tmp/libenr/d.sock"
            } else {
                "/tmp/libenr"
            };
            let resolved = resolve_with(&[(source, value)])
                .unwrap_or_else(|err| panic!("{source} should resolve: {err}"));
            assert!(
                resolved.socket.is_absolute(),
                "{source} produced a relative socket: {}",
                resolved.socket.display()
            );
        }
    }

    #[test]
    fn the_socket_path_stays_well_under_the_sun_path_limit() {
        // `sockaddr_un.sun_path` is 108 bytes on Linux, 104 on macOS. A long XDG_RUNTIME_DIR
        // plus a verbose filename is a real way to produce an unbindable socket.
        let paths = DaemonPaths::under(PathBuf::from("/run/user/1000/library-enrichment"));
        assert!(
            paths.socket.as_os_str().len() < 104,
            "socket path is too long to bind: {}",
            paths.socket.display()
        );
    }
}
