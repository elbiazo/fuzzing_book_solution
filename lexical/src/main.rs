use fuzzingbook::SimpleFuzzer;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::RangeInclusive;
use std::str;
use tempfile::tempfile;

fn main() -> Result<(), io::Error> {
    let sf = SimpleFuzzer::new(100, RangeInclusive::new('A' as u8, 'Z' as u8));
    // println!("{:?}", str::from_utf8(&sf.fuzz()).unwrap());

    // Write
    let mut tmpfile: File = tempfile()?;
    // write!(tmpfile, "Hello World!")?;
    tmpfile.write_all(str::from_utf8(&sf.fuzz()).unwrap().as_bytes());

    // Seek to start
    tmpfile.seek(SeekFrom::Start(0)).unwrap();

    // Read
    let mut buf = Vec::new();
    tmpfile.read_to_end(&mut buf)?;
    println!("{:?}", str::from_utf8(&buf).unwrap());

    Ok(())
}
