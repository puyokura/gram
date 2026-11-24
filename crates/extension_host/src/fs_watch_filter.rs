use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};
use std::time::{Duration, Instant};

/// Directories excluded by default from plugin file-watch notifications to prevent
/// event storms when AI agents or build tools run inside the integrated terminal.
pub const DEFAULT_IGNORED_DIR_NAMES: &[&str] = &["target", ".git", "node_modules", "tmp"];

/// Default debounce window before flushing a deduplicated batch to plugin hosts.
pub const DEFAULT_WATCH_DEBOUNCE: Duration = Duration::from_millis(200);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WatchEventKind {
    Created,
    Modified,
    Removed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WatchEvent {
    pub path: PathBuf,
    pub kind: WatchEventKind,
}

/// Returns `true` if `path` traverses any default-ignored directory
/// (`target/`, `.git/`, `node_modules/`, `tmp/`).
pub fn should_ignore_watch_path(path: &Path) -> bool {
    should_ignore_watch_path_with(path, DEFAULT_IGNORED_DIR_NAMES)
}

pub fn should_ignore_watch_path_with(path: &Path, ignored_dirs: &[&str]) -> bool {
    for component in path.components() {
        if let Component::Normal(name) = component {
            let Some(name_str) = name.to_str() else {
                continue;
            };
            if ignored_dirs
                .iter()
                .any(|ignored| name_str.eq_ignore_ascii_case(ignored))
            {
                return true;
            }
        }
    }
    false
}

/// Coalesces rapid file-system notifications into deduplicated batches while dropping
/// events from ignored build/VCS/temp directories.
pub struct DebouncedWatchBatcher {
    debounce: Duration,
    ignored_dirs: Vec<String>,
    pending: BTreeMap<PathBuf, WatchEventKind>,
    last_event_at: Option<Instant>,
}

impl DebouncedWatchBatcher {
    pub fn new(debounce: Duration) -> Self {
        Self {
            debounce,
            ignored_dirs: DEFAULT_IGNORED_DIR_NAMES
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
            pending: BTreeMap::new(),
            last_event_at: None,
        }
    }

    /// Records an incoming event at `now`. Returns `false` if the path was ignored.
    pub fn push(&mut self, event: WatchEvent, now: Instant) -> bool {
        let ignored_refs: Vec<&str> = self.ignored_dirs.iter().map(String::as_str).collect();
        if should_ignore_watch_path_with(&event.path, &ignored_refs) {
            return false;
        }
        self.pending
            .entry(event.path)
            .and_modify(|existing| {
                // Removed overrides Created/Modified; otherwise keep Created if newly created.
                *existing = match (*existing, event.kind) {
                    (_, WatchEventKind::Removed) => WatchEventKind::Removed,
                    (WatchEventKind::Created, WatchEventKind::Modified) => WatchEventKind::Created,
                    (_, next) => next,
                };
            })
            .or_insert(event.kind);
        self.last_event_at = Some(now);
        true
    }

    /// Flushes the deduplicated batch if the debounce window has elapsed since `last_event_at`.
    pub fn flush_if_ready(&mut self, now: Instant) -> Option<Vec<WatchEvent>> {
        let last = self.last_event_at?;
        if now.saturating_duration_since(last) < self.debounce {
            return None;
        }
        self.last_event_at = None;
        let events = std::mem::take(&mut self.pending)
            .into_iter()
            .map(|(path, kind)| WatchEvent { path, kind })
            .collect();
        Some(events)
    }

    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_default_storm_directories() {
        assert!(should_ignore_watch_path(Path::new("target/debug/gram.exe")));
        assert!(should_ignore_watch_path(Path::new("crates/foo/target/tmp.o")));
        assert!(should_ignore_watch_path(Path::new(".git/objects/aa/bbcc")));
        assert!(should_ignore_watch_path(Path::new("web/node_modules/lodash/index.js")));
        assert!(should_ignore_watch_path(Path::new("tmp/agent-scratch.txt")));

        assert!(!should_ignore_watch_path(Path::new("crates/i18n/src/lib.rs")));
        assert!(!should_ignore_watch_path(Path::new("docs/development/plugin-architecture.md")));
    }

    #[test]
    fn debounces_and_deduplicates_rapid_events() {
        let mut batcher = DebouncedWatchBatcher::new(Duration::from_millis(200));
        let t0 = Instant::now();

        // Ignored paths are dropped immediately
        assert!(!batcher.push(
            WatchEvent {
                path: PathBuf::from("target/debug/build.o"),
                kind: WatchEventKind::Modified,
            },
            t0
        ));
        assert!(!batcher.push(
            WatchEvent {
                path: PathBuf::from(".git/index.lock"),
                kind: WatchEventKind::Created,
            },
            t0
        ));
        assert_eq!(batcher.pending_len(), 0);

        // Rapid edits to the same source file coalesce into a single event
        assert!(batcher.push(
            WatchEvent {
                path: PathBuf::from("src/main.rs"),
                kind: WatchEventKind::Created,
            },
            t0
        ));
        assert!(batcher.push(
            WatchEvent {
                path: PathBuf::from("src/main.rs"),
                kind: WatchEventKind::Modified,
            },
            t0 + Duration::from_millis(50)
        ));
        assert!(batcher.push(
            WatchEvent {
                path: PathBuf::from("src/lib.rs"),
                kind: WatchEventKind::Modified,
            },
            t0 + Duration::from_millis(80)
        ));

        // Not ready before debounce window elapses
        assert!(batcher.flush_if_ready(t0 + Duration::from_millis(200)).is_none());

        // Ready once 200ms passes after the last event (t0 + 80 + 200)
        let batch = batcher
            .flush_if_ready(t0 + Duration::from_millis(280))
            .expect("expected debounced batch");
        assert_eq!(batch.len(), 2);
        assert_eq!(
            batch[0],
            WatchEvent {
                path: PathBuf::from("src/lib.rs"),
                kind: WatchEventKind::Modified,
            }
        );
        assert_eq!(
            batch[1],
            WatchEvent {
                path: PathBuf::from("src/main.rs"),
                kind: WatchEventKind::Created,
            }
        );
        assert_eq!(batcher.pending_len(), 0);
    }
}
