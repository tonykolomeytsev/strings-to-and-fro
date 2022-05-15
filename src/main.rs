mod models;

mod strings {
    pub mod reader;
    pub mod resolver;
    pub mod writer;
}

mod csv {
    pub mod reader;
    pub mod writer;
}

mod features {
    pub mod strings_to_csv;
}

use clap::Parser;

use colored::Colorize;
use csv::writer::CsvWriter;
use itertools::join;
use models::{OutputConsumer, WarningsCollector};
use strings::resolver;

/// Simple util to convert Android strings.xml to CSV and back.
///
/// ```bash
/// ./strings-to-and-fro -I./app/src/main/res -O./gen/output.csv
/// ```
#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
struct Args {
    /// Path to `res` dir of your android project module
    path: String,
    /// Output CSV file to convert to
    #[clap(short = 'O', long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    // Get strings.xml for all available language configurations.
    let configurations = resolver::resolve_res_dir(&args.path);
    if configurations.is_empty() {
        println!(
            "No available configurations found. Make sure that:\n\
            - you have specified correct path to `res` directory,\n\
            - you have strings.xml resources placed in `res/values` and `res/values-XX` directories."
        );
        return;
    }

    let mut warnings = Warnings::new(configurations.len());
    if args.output.is_some() {
        let mut csv_writer = CsvWriter::new(&args.output.unwrap());
        features::strings_to_csv::strings_to_csv(&args.path, &mut csv_writer, &mut warnings);
    } else {
        features::strings_to_csv::strings_to_csv(&args.path, &mut NoOpWriter(()), &mut warnings);
    }
}

struct NoOpWriter(());

impl OutputConsumer for NoOpWriter {
    fn append(&mut self, _: &String, _: &Vec<String>, _: bool) {
        /* no-op */
    }
}

struct Warnings {
    configurations_count: usize,
    empty_values_configurations: Vec<String>,
}

impl Warnings {
    fn new(configurations_count: usize) -> Warnings {
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

    fn check_key_name(&mut self, key: &String, configuration: &String) {
        let is_key_name_correct = key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
        if !is_key_name_correct {
            println!(
                "{} invalid key name {} in configuration {}",
                "  Error:".bold().red(),
                key.bold().blue(),
                configuration,
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
