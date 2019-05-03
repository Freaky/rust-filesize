use filesize;
use std::io;

fn main() -> io::Result<()> {
    println!("{:>9} {:>9} {:>9} {}", "Logical", "Physical", "Ratio", "Path");

    for path in std::env::args_os().skip(1) {
        let meta = std::fs::symlink_metadata(&path)?;
        let logical = meta.len();
        let physical = filesize::file_real_size_fast(&path, &meta)?;

        println!(
            "{:>9} {:>9} {:>9.2}x {}",
            logical,
            physical,
            physical as f64 / logical as f64,
            path.to_string_lossy()
        );
    }

    Ok(())
}
