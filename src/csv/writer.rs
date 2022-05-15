use std::fs::{self, File};
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use crate::models::OutputConsumer;

pub struct CsvWriter {
    writer: BufWriter<File>,
}

impl CsvWriter {
    pub fn new(csv_path: &String) -> CsvWriter {
        let path: PathBuf = csv_path
            .parse()
            .expect(&format!("Cannot parse path: {}", csv_path));
        let file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .expect(&format!("Cannot open CSV file for append: {}", csv_path));
        let writer = BufWriter::new(file);
        CsvWriter { writer: writer }
    }
}

impl OutputConsumer for CsvWriter {
    fn append(&mut self, key: &String, items: &Vec<String>, quotes: bool) {
        let mut i = 0;
        let mut iter = items.iter();
        loop {
            match iter.next() {
                Some(item) => {
                    if i == 0 {
                        write!(self.writer, "{}, ", key).unwrap();
                    } else {
                        write!(self.writer, ", ").unwrap();
                    }
                    if quotes {
                        write!(self.writer, "\"{}\"", item).unwrap();
                    } else {
                        write!(self.writer, "{}", item).unwrap();
                    }
                }
                None => break,
            }
            i += 1;
        }
        writeln!(self.writer, "").unwrap();
    }
}
