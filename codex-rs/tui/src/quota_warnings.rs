//! Client-local preference, independent of the shared app-server's config schema.
//! Absence of the marker preserves Codex's normal warning behavior.

use std::fs;
use std::io;
use std::path::Path;

const DISABLED_MARKER: &str = "quota-warnings.disabled";

pub(crate) fn enabled(codex_home: &Path) -> bool {
    !codex_home.join(DISABLED_MARKER).exists()
}

pub(crate) fn save(codex_home: &Path, enabled: bool) -> io::Result<()> {
    let marker = codex_home.join(DISABLED_MARKER);
    if enabled {
        match fs::remove_file(marker) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
    } else {
        fs::create_dir_all(codex_home)?;
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(marker)
        {
            Ok(_) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(()),
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn quota_warning_preference_defaults_on_and_survives_reload() {
        let dir = tempfile::tempdir().unwrap();
        assert!(super::enabled(dir.path()));
        for enabled in [false, false, true, true] {
            super::save(dir.path(), enabled).unwrap();
            assert_eq!(super::enabled(dir.path()), enabled);
        }
    }
}
