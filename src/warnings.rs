use colored::Colorize;
use itertools::join;

use crate::models::WarningsCollector;

pub struct Warnings {
    configurations_count: usize,
    empty_values_configurations: Vec<String>,
}

impl Warnings {
    pub fn new(configurations_count: usize) -> Warnings {
        Warnings {
            configurations_count,
            empty_values_configurations: Vec::new(),
        }
    }
}

impl WarningsCollector for Warnings {
    fn remember_empty_value(&mut self, coniguration: &String) {
        self.empty_values_configurations.push(coniguration.clone());
    }

    fn handle_empty_values(&mut self, key: &String) {
        if !self.empty_values_configurations.is_empty() {
            if self.configurations_count == self.empty_values_configurations.len() {
                println!(
                    "{} values in all configurations is empty for key {}.",
                    "Warning:".bold().yellow(),
                    key.bold().blue(),
                );
            }
            println!(
                "{} key {} has empty values in following configurations:\n         {}",
                "Warning:".bold().yellow(),
                key.bold().blue(),
                join(&self.empty_values_configurations, ", "),
            );
            self.empty_values_configurations.clear();
        }
    }

    fn check_key_name(&mut self, key: &String, configurations: &Vec<String>) {
        let is_key_name_correct = key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
        if !is_key_name_correct {
            println!(
                "{} invalid key name {} in following configurations:\n         {}",
                "  Error:".bold().red(),
                key.bold().blue(),
                join(configurations, ", "),
            )
        }
    }

    fn duplicated_key(&self, key: &String, configuration: &String) {
        println!(
            "{} duplicated key name {} in configuration {}",
            "  Error:".bold().red(),
            key.bold().blue(),
            configuration,
        )
    }
}
