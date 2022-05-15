use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::fs::File;

use crate::models::AndroidStringsEntry;
use crate::strings::{reader::AndroidStringsReader, resolver::Configuration};

pub struct StringsPool {
    /// [Configuration Name] to [[Key] to [Value]]
    entries: HashMap<String, HashMap<String, String>>,
    /// [Configuration Name] to [AndroidStringsReader]
    readers: HashMap<String, AndroidStringsReader>,
    /// Configurations that are not done yet
    unfinished: HashSet<String>,
}

impl StringsPool {
    pub fn new() -> StringsPool {
        StringsPool {
            entries: HashMap::new(),
            readers: HashMap::new(),
            unfinished: HashSet::new(),
        }
    }

    pub fn create(&mut self, configuration: &Configuration) {
        self.entries
            .insert(configuration.name.clone(), HashMap::new());
        self.readers.insert(
            configuration.name.clone(),
            AndroidStringsReader::from_file(File::open(&configuration.strings_path).unwrap()),
        );
        self.unfinished.insert(configuration.name.clone());
    }

    pub fn reader_for(&mut self, configuration: &Configuration) -> &mut AndroidStringsReader {
        self.readers.get(&configuration.name).unwrap()
    }

    pub fn entries_for(&self, configuration: &Configuration) -> &HashMap<String, String> {
        self.entries.get(&configuration.name).unwrap()
    }

    pub fn has_unfinished_configurations(&self) -> bool {
        !self.unfinished.is_empty()
    }
}
