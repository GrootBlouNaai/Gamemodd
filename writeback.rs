use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn write_to_audiogroup(trk_num: u32, ffloc: u32, files: &[&str]) -> io::Result<()> {
    let mut buffer = vec![0; ffloc as usize];
    buffer[0x10..0x14].copy_from_slice(&trk_num.to_le_bytes());
    buffer[0x14..0x18].copy_from_slice(&ffloc.to_le_bytes());

    let mut p = ffloc as usize;
    for (n, file) in files.iter().enumerate() {
        let file_path = Path::new(file);
        let mut file = File::open(file_path)?;
        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents)?;

        let s2 = file_contents.len();
        buffer.extend_from_slice(&(s2 as u32).to_le_bytes());
        buffer.extend_from_slice(&file_contents);

        let rem = s2 % 4;
        if rem != 0 {
            buffer.extend_from_slice(&vec![0; 4 - rem]);
        }
    }
    let path = Path::new("audiogroup.dat");
    let mut file = File::create(path)?;
    file.write_all(&buffer)?;
    Ok(())
}
