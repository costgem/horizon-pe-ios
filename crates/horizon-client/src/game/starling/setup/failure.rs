//! A failed setup in the player's words: what went wrong and what to do about it, from the raw
//! error the download (accounts.rs) or the preparing (preparer.rs) gave. The raw error stays
//! under it, shortened, and whole in the log.
use horizon_client::lang;

/// Before the loader's reason when resources prepared here did not load (`step`).
pub(super) const NOT_LOADED: &str = "The prepared game files did not load: ";

/// What went wrong, by what can be done about it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cause {
    /// The device is full.
    Space,
    /// iOS cut the connection: the app left the screen while it downloaded.
    Interrupted,
    /// No connection to Mojang's servers.
    Network,
    /// A file came with other bytes than it should have.
    Damaged,
    /// Official files the preparing needs are not there.
    Missing,
    /// The app's own tables are not there: it is incomplete.
    App,
    /// The app may not write its own folder.
    Permission,
    /// Prepared, but the game did not load what was made.
    NotLoaded,
    Unknown,
}

impl Cause {
    fn key(self) -> &'static str {
        match self {
            Cause::Space => "space",
            Cause::Interrupted => "interrupted",
            Cause::Network => "network",
            Cause::Damaged => "damaged",
            Cause::Missing => "missing",
            Cause::App => "app",
            Cause::Permission => "permission",
            Cause::NotLoaded => "not_loaded",
            Cause::Unknown => "unknown",
        }
    }
}

fn cause(raw: &str) -> Cause {
    if raw.starts_with(NOT_LOADED) {
        return Cause::NotLoaded;
    }
    let raw = raw.to_lowercase();
    let any = |needles: &[&str]| needles.iter().any(|n| raw.contains(n));
    // In order: a full disk fails a download's write too, and a cut connection is a network error
    // with its own advice.
    if any(&["no space left", "os error 28", "disk full", "not enough space"]) {
        Cause::Space
    } else if any(&[
        "os error 53",
        "software caused connection abort",
        "connection abort",
    ]) {
        Cause::Interrupted
    } else if any(&[
        "dns",
        "lookup",
        "network",
        "connection",
        "timed out",
        "timeout",
        "unreachable",
        "os error 50",
        "os error 51",
        "os error 54",
        "os error 60",
        "os error 61",
        "os error 65",
        "tls",
        "certificate",
        "status code",
        "http status",
        "transport",
        "broken pipe",
        "unexpected eof",
    ]) {
        Cause::Network
    } else if any(&[
        "not the file it should be",
        "checksum",
        "hash mismatch",
        "corrupt",
        "invalid zip",
        "invalid archive",
        "damaged",
    ]) {
        Cause::Damaged
    } else if any(&[
        "game tables",
        "the app's recipe",
        "recipe names",
        "recipe lists",
        "cannot find its own files",
        "install it again",
    ]) {
        Cause::App
    } else if any(&[
        "permission denied",
        "operation not permitted",
        "os error 13",
        "os error 1)",
        "read-only file system",
    ]) {
        Cause::Permission
    } else if any(&[
        "not all there",
        "client jar",
        "no such file",
        "os error 2)",
        "lacks",
    ]) {
        Cause::Missing
    } else {
        Cause::Unknown
    }
}

/// `raw` without the folders before its file names, and every link cut to its host: the card is
/// narrow, and those parts say nothing to the player.
fn shorten(raw: &str) -> String {
    let raw = raw.strip_prefix(NOT_LOADED).unwrap_or(raw);
    let words: Vec<String> = raw
        .split(' ')
        .map(|word| {
            if let Some(rest) = word
                .strip_prefix("https://")
                .or_else(|| word.strip_prefix("http://"))
            {
                let host = rest.split('/').next().unwrap_or(rest);
                let tail = if word.ends_with(':') && !host.ends_with(':') {
                    ":"
                } else {
                    ""
                };
                format!("{host}{tail}")
            } else if word.starts_with('/') && word.len() > 1 {
                word.rsplit('/').next().unwrap_or(word).to_owned()
            } else {
                word.to_owned()
            }
        })
        .collect();
    let short = words.join(" ");
    const MAX: usize = 180;
    if short.chars().count() > MAX {
        let mut cut: String = short.chars().take(MAX).collect();
        cut.push('…');
        cut
    } else {
        short
    }
}

/// Where the player finds the whole log.
fn log_place() -> String {
    if cfg!(target_os = "ios") {
        lang::tr("starling.setup.error.log_ios", &[&horizon_client::APP_NAME])
    } else {
        lang::get("starling.setup.error.log")
    }
}

/// The card for `raw`: its title, what to do, and the line under it (the shortened reason and
/// where the log is).
pub(super) fn texts(raw: &str) -> (String, String, Option<String>) {
    let cause = cause(raw);
    let key = cause.key();
    let title = lang::get(&format!("starling.setup.error.{key}.title"));
    let advice = lang::get(&format!("starling.setup.error.{key}.advice"));
    let reason = shorten(raw);
    let note = if reason.trim().is_empty() {
        log_place()
    } else {
        format!(
            "{}\n{}",
            lang::tr("starling.setup.error.reason", &[&reason]),
            log_place()
        )
    };
    (title, advice, Some(note))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_errors_find_their_cause() {
        for (raw, want) in [
            (
                "/var/mobile/Containers/Data/Application/X/Library/res.partial/a.png: No space left on device (os error 28)",
                Cause::Space,
            ),
            (
                "https://resources.download.minecraft.net/ab/abcd: io: Software caused connection abort (os error 53)",
                Cause::Interrupted,
            ),
            (
                "https://piston-data.mojang.com/v1/objects/x/client.jar: Dns Failed: failed to lookup address information",
                Cause::Network,
            ),
            (
                "/var/x/resources.partial/assets/minecraft/lang/en_us.json is not the file it should be",
                Cause::Damaged,
            ),
            (
                "The game tables are missing from the app: install it again.",
                Cause::App,
            ),
            (
                "/var/x/versions/26.3/26.3.jar: No such file or directory (os error 2)",
                Cause::Missing,
            ),
            ("/var/x/logs: Permission denied (os error 13)", Cause::Permission),
            (
                "The prepared game files did not load: blocks.json is missing",
                Cause::NotLoaded,
            ),
            ("something nobody expected", Cause::Unknown),
        ] {
            assert_eq!(cause(raw), want, "{raw}");
        }
    }

    #[test]
    fn the_reason_loses_its_folders_and_links() {
        assert_eq!(
            shorten(
                "/var/mobile/Containers/Data/Application/X/Library/res.partial/a.png: No space left on device (os error 28)"
            ),
            "a.png: No space left on device (os error 28)"
        );
        assert_eq!(
            shorten("https://resources.download.minecraft.net/ab/abcd: Dns Failed"),
            "resources.download.minecraft.net: Dns Failed"
        );
    }
}
