use std::fs::File;
use std::io::BufReader;

use crate::models::AndroidStringsEntry;
use quick_xml::events::Event;
use quick_xml::Reader;

pub struct AndroidStringsReader {
    xml_reader: Reader<BufReader<File>>,
    buffer: Vec<u8>,
}

const BUF_SIZE: usize = 4096;

impl AndroidStringsReader {
    pub fn from_file(file: File) -> AndroidStringsReader {
        let reader = BufReader::new(file);
        AndroidStringsReader {
            xml_reader: Reader::from_reader(reader),
            buffer: Vec::with_capacity(BUF_SIZE),
        }
    }

    pub fn next(&mut self) -> Option<AndroidStringsEntry> {
        let mut key = String::new();
        let mut waiting_for_text = false;
        loop {
            let event = &self.xml_reader.read_event(&mut self.buffer);
            match event {
                Ok(Event::Eof) => return None,
                Ok(Event::Start(ref e)) => match e.name() {
                    b"string" => {
                        let name_attr = e
                            .attributes()
                            .find(|e| e.as_ref().unwrap().key == b"name")
                            .expect("Unexpected `strings` tag without `name` attrubute")
                            .unwrap();

                        key = String::from_utf8(Vec::from(name_attr.value)).unwrap();
                        waiting_for_text = true;
                    }
                    _ => (),
                },
                Ok(Event::Text(ref e)) => {
                    if waiting_for_text {
                        return Some(AndroidStringsEntry {
                            key: key,
                            value: String::from_utf8(Vec::from(e.escaped())).unwrap(),
                        });
                    }
                }
                Err(e) => panic!(
                    "Error while reading strings.xml at position {}: {:?}",
                    &self.xml_reader.buffer_position(),
                    e,
                ),
                _ => (),
            };
        }
    }

    /// Free up resources
    pub fn clear(&mut self) {
        self.buffer.clear()
    }
}
