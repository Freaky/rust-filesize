use filesize;
use std::io;

fn main() -> io::Result<()> {
    for arg in std::env::args_os() {
        let size = std::fs::symlink_metadata(&arg)?.len();
        let realsize = filesize::get_compressed_file_size(&arg)?;

        println!(
            "{}, {} bytes logical, {} bytes on-disk",
            arg.to_string_lossy(),
            size,
            realsize
        );
    }

    Ok(())
}
