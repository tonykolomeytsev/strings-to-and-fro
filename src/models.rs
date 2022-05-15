#[derive(Debug)]
pub struct AndroidStringsEntry {
    pub key: String,
    pub value: String,
}

pub trait OutputConsumer {
    fn append(&mut self, key: &String, items: &Vec<String>, quotes: bool);
}

pub trait WarningsCollector {
    fn remember_empty_value(&mut self, coniguration: &String);
    fn handle_empty_values(&mut self, key: &String);
    fn check_key_name(&mut self, key: &String, configuration: &String);
    fn duplicated_key(&self, key: &String, configuration: &String);
}
