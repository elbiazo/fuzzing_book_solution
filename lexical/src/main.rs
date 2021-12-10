use fuzzingbook::SimpleFuzzer;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::RangeInclusive;
use std::process::{Command, ExitStatus};
use std::str;
use tempfile::NamedTempFile;

fn main() -> Result<(), io::Error> {
    // let sf = SimpleFuzzer::new(100, RangeInclusive::new('0' as u8, '9' as u8));
    let sf = SimpleFuzzer::new(100, RangeInclusive::new('a' as u8, 'z' as u8));
    // println!("{:?}", str::from_utf8(&sf.fuzz()).unwrap());

    // Write
    let mut tmpfile = NamedTempFile::new()?;
    tmpfile.write_all(&sf.fuzz())?;

    // Seek to start
    tmpfile.seek(SeekFrom::Start(0)).unwrap();

    // Read
    let mut buf = Vec::new();
    tmpfile.read_to_end(&mut buf)?;

    let runner = Command::new("bc").arg(tmpfile.path()).output()?;
    println!("{:?}", str::from_utf8(&runner.stdout).unwrap());
    println!("{:?}", runner.status.code());
    println!("{:?}", str::from_utf8(&runner.stderr));

    Ok(())
}
