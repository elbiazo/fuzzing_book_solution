use fuzzingbook::SimpleFuzzer;
use std::ops::RangeInclusive;
use tempfile::tempfile;
use std::io::{self, Write, Read};

fn main() -> Result<(), io::Error> {
    let sf = SimpleFuzzer::new(100, RangeInclusive::new(255, 255));
    println!("{:?}", sf.fuzz());

    let mut file = tempfile()?;
    writeln!(file, "Hello, world!")?;
    println!("{:?}", file);


    Ok(())
}
