use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let file = File::open("audiogroup.dat")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let trk_num = u32::from_le_bytes([buffer[0x10], buffer[0x11], buffer[0x12], buffer[0x13]]);
    let ffloc = u32::from_le_bytes([buffer[0x14], buffer[0x15], buffer[0x16], buffer[0x17]]);

    println!("Number of tracks: {}", trk_num);
    println!("First Track Addr: 0x{:08x}", ffloc);

    let mut p = ffloc as usize;
    for n in 0..trk_num {
        let s2 = u32::from_le_bytes([buffer[p], buffer[p + 1], buffer[p + 2], buffer[p + 3]]) as usize;
        p += 4;

        println!("File at 0x{:08x} (Size {})... ", p, s2);
        let fname = format!("extract{:03}.ogg", n);

        let mut extract = File::create(fname)?;
        extract.write_all(&buffer[p..p + s2])?;
        println!("Extracted!");

        let rem = s2 % 4;
        p += s2 + 4 - rem;
    }

    Ok(())
}
