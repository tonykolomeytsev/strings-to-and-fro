mod models;
mod warnings;

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

use crate::strings::resolver;
use crate::warnings::Warnings;

use clap::Parser;
use csv::writer::CsvWriter;
use models::OutputConsumer;

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
