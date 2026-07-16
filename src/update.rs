use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use log::debug;
use self_update::backends::github::{Update, UpdateBuilder};
use serde::{Deserialize, Serialize};

const REPO_OWNER: &str = "van-sprundel";
const REPO_NAME: &str = "ferrisume";
/// How long a cached "latest version" result stays fresh.
const CHECK_INTERVAL: Duration = Duration::from_secs(60 * 60 * 24);
/// How long a background refresh may delay process exit.
const BACKGROUND_CHECK_GRACE: Duration = Duration::from_millis(1500);

pub const NO_UPDATE_CHECK_ENV: &str = "FERRISUME_NO_UPDATE_CHECK";

fn configure() -> UpdateBuilder {
    let mut builder = Update::configure();
    builder
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name("ferrisume")
        // cargo-dist archives contain the binary under a `ferrisume-cli-<target>/` directory
        .bin_path_in_archive("ferrisume-cli-{{ target }}/{{ bin }}")
        .current_version(env!("CARGO_PKG_VERSION"));
    builder
}

pub fn update_command(check_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    if check_only {
        match configure().build()?.is_update_available()? {
            Some(release) => {
                println!(
                    "A new version is available: v{} (current: v{})",
                    release.version(),
                    env!("CARGO_PKG_VERSION")
                );
                println!("Run `ferrisume update` to install it.");
            }
            None => println!(
                "You are on the latest version (v{})",
                env!("CARGO_PKG_VERSION")
            ),
        }
        return Ok(());
    }

    let status = configure()
        .show_download_progress(true)
        .no_confirm(true)
        .build()?
        .update()?;

    if status.is_updated() {
        println!("Updated to v{}", status.version());
    } else {
        println!(
            "Already on the latest version (v{})",
            env!("CARGO_PKG_VERSION")
        );
    }

    // A successful update means we *know* the current binary is the latest.
    let _ = write_cache(&CheckCache {
        checked_at: unix_now(),
        latest: status.version().to_string(),
    });

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct CheckCache {
    checked_at: u64,
    latest: String,
}

fn cache_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "ferrisume", "ferrisume")
        .map(|dirs| dirs.cache_dir().join("update-check.json"))
}

fn read_cache() -> Option<CheckCache> {
    let contents = std::fs::read_to_string(cache_path()?).ok()?;
    serde_json::from_str(&contents).ok()
}

fn write_cache(cache: &CheckCache) -> std::io::Result<()> {
    let Some(path) = cache_path() else {
        return Ok(());
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string(cache).unwrap_or_default())
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn is_newer(latest: &str, current: &str) -> bool {
    self_update::version::bump_is_greater(current, latest).unwrap_or(false)
}

/// Print a hint when a cached check says a newer version exists, and refresh
/// the cache in the background when it has gone stale. Never fails the
/// command it runs alongside; network errors are only logged.
pub fn update_notice() {
    if std::env::var_os(NO_UPDATE_CHECK_ENV).is_some() {
        return;
    }

    let cache = read_cache();

    if let Some(ref cache) = cache {
        if is_newer(&cache.latest, env!("CARGO_PKG_VERSION")) {
            eprintln!(
                "\nA new version of ferrisume is available: v{} (current: v{}). Run `ferrisume update` to install it.",
                cache.latest,
                env!("CARGO_PKG_VERSION")
            );
        }
    }

    let stale = cache
        .map(|c| unix_now().saturating_sub(c.checked_at) > CHECK_INTERVAL.as_secs())
        .unwrap_or(true);
    if !stale {
        return;
    }

    // Refresh in the background so fast commands aren't held up by a slow
    // network; give it a short grace period, and if it doesn't finish, the
    // next run simply tries again.
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let result = configure()
            .build()
            .and_then(|update| update.get_latest_release());
        let latest = result
            .ok()
            .and_then(|releases| releases.latest().map(|r| r.version().to_string()));
        if let Some(latest) = latest {
            let _ = write_cache(&CheckCache {
                checked_at: unix_now(),
                latest,
            });
        } else {
            debug!("background update check failed");
        }
        let _ = tx.send(());
    });
    let _ = rx.recv_timeout(BACKGROUND_CHECK_GRACE);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_newer_compares_semver() {
        assert!(is_newer("0.6.0", "0.5.4"));
        assert!(!is_newer("0.5.4", "0.5.4"));
        assert!(!is_newer("0.5.3", "0.5.4"));
    }

    /// cargo-dist archives place the binary under `ferrisume-cli-<target>/`,
    /// which `configure()` mirrors in `bin_path_in_archive`. If either side of
    /// that assumption changes, `ferrisume update` breaks at the extract step.
    #[test]
    fn extracts_binary_from_cargo_dist_archive_layout() {
        let target = self_update::get_target();
        let bin_path = format!(
            "ferrisume-cli-{}/ferrisume{}",
            target,
            std::env::consts::EXE_SUFFIX
        );

        let dir = tempfile::tempdir().unwrap();
        let archive_path = dir.path().join("release.tar.gz");
        let archive = std::fs::File::create(&archive_path).unwrap();
        let encoder = flate2::write::GzEncoder::new(archive, flate2::Compression::default());
        let mut tar = tar::Builder::new(encoder);
        let contents = b"fake binary";
        let mut header = tar::Header::new_gnu();
        header.set_size(contents.len() as u64);
        header.set_mode(0o755);
        header.set_cksum();
        tar.append_data(&mut header, &bin_path, contents.as_slice())
            .unwrap();
        tar.into_inner().unwrap().finish().unwrap();

        let extract_dir = dir.path().join("extracted");
        std::fs::create_dir(&extract_dir).unwrap();
        self_update::Extract::from_source(&archive_path)
            .extract_file(&extract_dir, &bin_path)
            .unwrap();

        assert_eq!(std::fs::read(extract_dir.join(bin_path)).unwrap(), contents);
    }
}
