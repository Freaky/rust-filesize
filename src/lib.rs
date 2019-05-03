//!
//! Get the real on-disk size of a file, taking into account compression and
//! sparse allocation.
//!
//! Returns the logical size on platforms other than Unix and Windows.
//!
//! ```rust
//! # fn main() -> std::io::Result<()> {
//! let realsize = filesize::get_compressed_file_size("Cargo.toml")?;
//! # Ok(())
//! # }

use std::path::Path;

#[cfg(unix)]
pub fn get_compressed_file_size<P: AsRef<Path>>(p: P) -> std::io::Result<u64> {
    use std::os::unix::fs::MetadataExt;

    Ok(std::fs::symlink_metadata(p)?.blocks() * 512)
}

#[cfg(windows)]
pub fn get_compressed_file_size<P: AsRef<Path>>(p: P) -> std::io::Result<u64> {
    use std::os::windows::ffi::OsStrExt;
    use winapi::shared::winerror::NO_ERROR;
    use winapi::um::fileapi::{GetCompressedFileSizeW, INVALID_FILE_SIZE};

    let p = std::fs::canonicalize(p)?.into_os_string();
    let mut path: Vec<u16> = Vec::with_capacity(p.len() + 1);
    path.extend(p.encode_wide());
    path.push(0);

    let mut high: u32 = 0;
    let low = unsafe { GetCompressedFileSizeW(path.as_ptr(), &mut high) };

    if low == INVALID_FILE_SIZE {
        let err = std::io::Error::last_os_error();
        if err.raw_os_error().map(|e| e as u32).unwrap_or(NO_ERROR) != NO_ERROR {
            return Err(err);
        }
    }

    Ok(u64::from(high) << 32 | u64::from(low))
}

#[cfg(not(any(windows, unix)))]
pub fn get_compressed_file_size<P: AsRef<Path>>(p: P) -> std::io::Result<u64> {
    std::fs::symlink_metadata(p)?.len()
}

#[test]
fn it_seems_to_work() {
    assert!(
        get_compressed_file_size("Cargo.toml").expect("get_compressed_file_size") > 0
    );
}
