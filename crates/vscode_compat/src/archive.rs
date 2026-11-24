use std::path::{Component, Path, PathBuf};

use anyhow::{Context as _, Result, bail};
use async_zip::base::read::seek::ZipFileReader;
use futures::io::{AsyncReadExt, BufReader};

/// Hard limits to prevent ZIP bombs when extracting user-supplied `.vsix` archives.
pub const MAX_VSIX_ENTRIES: usize = 4096;
pub const MAX_VSIX_UNCOMPRESSED_BYTES: u64 = 100 * 1024 * 1024; // 100 MiB

#[derive(Debug, Clone)]
pub struct ExtractedVsix {
    pub root_dir: PathBuf,
    pub extension_dir: PathBuf,
    pub entry_count: usize,
    pub total_bytes: u64,
}

/// Validates that a ZIP entry path is strictly relative and cannot escape `destination`.
pub fn sanitize_zip_entry_path(raw: &str) -> Result<PathBuf> {
    if raw.is_empty() {
        bail!("empty zip entry path");
    }
    // Reject Windows drive prefixes and leading slashes/backslashes regardless of platform.
    if raw.starts_with('/') || raw.starts_with('\\') || raw.as_bytes().get(1) == Some(&b':') {
        bail!("absolute zip entry path rejected: {raw}");
    }

    let normalized = raw.replace('\\', "/");
    let mut clean = PathBuf::new();
    for component in Path::new(&normalized).components() {
        match component {
            Component::Normal(part) => clean.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                bail!("path traversal in zip entry rejected: {raw}");
            }
        }
    }

    if clean.as_os_str().is_empty() {
        bail!("zip entry resolved to empty path: {raw}");
    }

    Ok(clean)
}

/// Safely extracts a local `.vsix` (ZIP) archive into `destination`.
///
/// Enforces:
/// - No path traversal (`..`, absolute paths, drive letters)
/// - Maximum entry count (`MAX_VSIX_ENTRIES`)
/// - Maximum total uncompressed size (`MAX_VSIX_UNCOMPRESSED_BYTES`)
pub async fn extract_vsix_safely(vsix_path: &Path, destination: &Path) -> Result<ExtractedVsix> {
    extract_vsix_with_limits(
        vsix_path,
        destination,
        MAX_VSIX_ENTRIES,
        MAX_VSIX_UNCOMPRESSED_BYTES,
    )
    .await
}

pub async fn extract_vsix_with_limits(
    vsix_path: &Path,
    destination: &Path,
    max_entries: usize,
    max_bytes: u64,
) -> Result<ExtractedVsix> {
    std::fs::create_dir_all(destination)
        .with_context(|| format!("failed to create destination {}", destination.display()))?;

    let file = smol::fs::File::open(vsix_path)
        .await
        .with_context(|| format!("failed to open vsix {}", vsix_path.display()))?;
    let mut reader = ZipFileReader::new(BufReader::new(file))
        .await
        .with_context(|| format!("failed to read vsix zip directory {}", vsix_path.display()))?;

    let entries = reader.file().entries().to_vec();
    if entries.len() > max_entries {
        bail!("vsix exceeds maximum entry limit ({max_entries})");
    }

    let entry_count = entries.len();
    let mut total_bytes: u64 = 0;
    let mut buf = vec![0u8; 32 * 1024];

    for (index, entry) in entries.into_iter().enumerate() {
        let raw_name = entry
            .filename()
            .as_str()
            .context("vsix entry filename is not valid UTF-8")?
            .to_string();
        let is_dir = entry
            .dir()
            .with_context(|| format!("failed to inspect zip entry metadata for {raw_name}"))?;

        let rel_path = sanitize_zip_entry_path(&raw_name)?;
        let target_path = destination.join(&rel_path);

        if is_dir {
            std::fs::create_dir_all(&target_path)
                .with_context(|| format!("failed to create directory {}", target_path.display()))?;
        } else {
            if let Some(parent) = target_path.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create parent {}", parent.display()))?;
            }
            let mut out = std::fs::File::create(&target_path)
                .with_context(|| format!("failed to create file {}", target_path.display()))?;
            let mut entry_reader = reader
                .reader_with_entry(index)
                .await
                .with_context(|| format!("failed to open zip entry {raw_name}"))?;
            loop {
                let n = entry_reader.read(&mut buf).await?;
                if n == 0 {
                    break;
                }
                total_bytes = total_bytes.saturating_add(n as u64);
                if total_bytes > max_bytes {
                    drop(out);
                    let _ = std::fs::remove_dir_all(destination);
                    bail!("vsix exceeds maximum uncompressed size limit ({max_bytes} bytes)");
                }
                std::io::Write::write_all(&mut out, &buf[..n])?;
            }
        }
    }

    let extension_dir = if destination.join("extension").join("package.json").is_file() {
        destination.join("extension")
    } else if destination.join("package.json").is_file() {
        destination.to_path_buf()
    } else {
        bail!(
            "vsix archive does not contain extension/package.json or package.json"
        );
    };

    Ok(ExtractedVsix {
        root_dir: destination.to_path_buf(),
        extension_dir,
        entry_count,
        total_bytes,
    })
}
