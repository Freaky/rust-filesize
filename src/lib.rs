//! Get the real on-disk size of a file, taking into account compression and
//! sparse allocation.
//!
//! On Unix this uses `std::fs::symlink_metadata()` and `std::os::unix::fs::MetadataExt`.
//!
//! On Windows it uses `GetCompressedFileSizeW()`.
//!
//! Returns the logical size on platforms other than Unix and Windows.

use std::fs::Metadata;
use std::path::Path;

#[cfg(unix)]
mod imp {
    use std::fs::Metadata;
    use std::os::unix::fs::MetadataExt;
    use std::path::Path;

    pub fn file_real_size<P: AsRef<Path>>(path: P) -> std::io::Result<u64> {
        Ok(std::fs::symlink_metadata(path)?.blocks() * 512)
    }

    pub fn file_real_size_fast<P: AsRef<Path>>(_path: P, metadata: &Metadata) -> std::io::Result<u64> {
        Ok(metadata.blocks() * 512)
    }
}

#[cfg(windows)]
mod imp {
    use std::fs::Metadata;
    use std::os::windows::ffi::OsStrExt;
    use std::path::Path;

    use winapi::shared::winerror::NO_ERROR;
    use winapi::um::fileapi::{GetCompressedFileSizeW, INVALID_FILE_SIZE};

    pub fn file_real_size<P: AsRef<Path>>(path: P) -> std::io::Result<u64> {
        let path = std::fs::canonicalize(path)?.into_os_string();
        let mut pathw: Vec<u16> = Vec::with_capacity(path.len() + 1);
        pathw.extend(path.encode_wide());
        pathw.push(0);

        let mut high: u32 = 0;
        let low = unsafe { GetCompressedFileSizeW(pathw.as_ptr(), &mut high) };

        if low == INVALID_FILE_SIZE {
            let err = std::io::Error::last_os_error();
            if err.raw_os_error().map(|e| e as u32).unwrap_or(NO_ERROR) != NO_ERROR {
                return Err(err);
            }
        }

        Ok(u64::from(high) << 32 | u64::from(low))
    }

    pub fn file_real_size_fast<P: AsRef<Path>>(path: P, _metadata: &Metadata) -> std::io::Result<u64> {
        file_real_size(path)
    }
}

#[cfg(not(any(windows, unix)))]
mod imp {
    use std::fs::Metadata;
    use std::path::Path;

    pub fn file_real_size<P: AsRef<Path>>(path: P) -> std::io::Result<u64> {
        std::fs::symlink_metadata(path)?.len()
    }

    pub fn file_real_size_fast<P: AsRef<Path>>(_path: P, metadata: &Metadata) -> std::io::Result<u64> {
        Ok(metadata.len())
    }
}

/// Get the on-disk size of the file at the given `path`.
///
/// ```rust
/// # fn main() -> std::io::Result<()> {
/// let realsize = filesize::file_real_size("Cargo.toml")?;
/// # Ok(())
/// # }
/// ```
pub fn file_real_size<P: AsRef<Path>>(path: P) -> std::io::Result<u64> {
    self::imp::file_real_size(path)
}

/// Get the on-disk size of the file at the given `path`, using a reference to
/// a previously-fetched `Metadata` struct if necessary.
///
/// This avoids an unnecessary `stat()` call on Unix if `Metadata` has already
/// been retrieved.
///
/// ```rust
/// # fn main() -> std::io::Result<()> {
/// let realsize = filesize::file_real_size_fast(
///     "Cargo.toml",
///      &std::fs::symlink_metadata("Cargo.toml")?
/// )?;
/// # Ok(())
/// # }
/// ```
pub fn file_real_size_fast<P: AsRef<Path>>(path: P, metadata: &Metadata) -> std::io::Result<u64> {
    self::imp::file_real_size_fast(path, metadata)
}

#[test]
fn it_seems_to_work() {
    assert!(
        file_real_size("Cargo.toml").expect("file_real_size")
            == file_real_size_fast(
                "Cargo.toml",
                &std::fs::symlink_metadata("Cargo.toml").expect("stat")
            )
            .expect("file_real_size_fast")
    );
}
