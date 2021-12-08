use rand::Rng;
use bytes::Bytes;
use std::ops::RangeInclusive;

pub struct SimpleFuzzer {
    max_length : usize,
    char_range: RangeInclusive<u8>,
}

impl SimpleFuzzer {
    pub fn new(max_length: usize, char_range: RangeInclusive<u8>) -> SimpleFuzzer{
        SimpleFuzzer {
            max_length: max_length,
            char_range: char_range,
        }
    }

    pub fn fuzz(&self) -> Bytes {
        let mut s = Vec::<u8>::new();
        let mut rng = rand::thread_rng();
        for _ in 0..self.max_length {
            s.push(rng.gen_range(self.char_range.clone()));
        }
        Bytes::from(s)
    }
}
