use std::fs::File;
use std::io::prelude::*;

#[derive(Default)]
pub struct SmartGenerator {}

impl SmartGenerator {
    pub fn new() -> Self {
        SmartGenerator {}
    }

    pub fn next_sequence(&self, input: &[char]) -> Vec<char> {
        let mut sequence = Vec::new();
        let mut count = 0;
        let mut digit = input[0];

        for current in input.iter() {
            if *current == digit {
                count += 1;
            } else {
                sequence.push(char::from_digit(count, 10).unwrap());
                sequence.push(digit);
                digit = *current;
                count = 1;
            }
        }
        sequence.push(char::from_digit(count, 10).unwrap());
        sequence.push(digit);

        sequence
    }

    pub fn write_sequence_to_file(
        &self,
        input: Vec<char>,
        mut file: File,
    ) -> std::io::Result<File> {
        let row_str: String = input.iter().collect();
        file.write_all(row_str.as_bytes())?;
        file.write_all(b"\n")?;
        Ok(file)
    }
    // Add other methods for the smart generator here
}
