# filesize

[![Cargo](https://img.shields.io/crates/v/filesize.svg)][crate]
[![Build Status](https://travis-ci.org/Freaky/rust-filesize.svg?branch=master)](https://travis-ci.org/Freaky/rust-filesize)

## Physical disk space use retrieval.

`filesize` abstracts platform-specific methods of determining the real space used
by files, taking into account filesystem compression and sparse files.

```rust
use filesize::{file_real_size, file_real_size_fast};

let realsize = file_real_size("Cargo.toml")?;

// Save a stat() on Unix
let also_realsize = file_real_size_fast(
    "Cargo.toml",
    &std::fs::symlink_metadata("Cargo.toml")?
)?;
```

Now, please stop writing `du` clones that only take apparent size into account.

### Supported Platforms

#### Unix

On Unix platforms this is a thin wrapper around [`std::fs::symlink_metadata()`]
and [`std::os::unix::fs::MetadataExt`], simply returning `blocks() * 512`.

#### Windows

On Windows this wraps [`GetCompressedFileSizeW()`].

#### Everything Else

All other platforms receive a fallback that simply returns the logical file size.


[`GetCompressedFileSizeW()`]: https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-getcompressedfilesizew
[`std::fs::symlink_metadata()`]: https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html
[`std::os::unix::fs::MetadataExt`]: https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html
[crate]: https://crates.io/crates/filesize
