use std::collections::HashMap;
use std::fs::File;

use colored::Colorize;

use crate::models::{OutputConsumer, WarningsCollector};
use crate::strings::reader::AndroidStringsReader;
use crate::strings::resolver;

const MAX_ITERATIONS: i32 = 1_000_000;

pub fn strings_to_csv<O, W>(res_path: &String, output: &mut O, warnings: &mut W)
where
    O: OutputConsumer,
    W: WarningsCollector,
{
    // Get strings.xml for all available language configurations.
    let configurations = resolver::resolve_res_dir(res_path);
    if configurations.is_empty() {
        println!(
            "No available configurations found. Make sure that:\n\
            - you have specified correct path to `res` directory,\n\
            - you have strings.xml resources placed in `res/values` and `res/values-XX` directories."
        );
        return;
    }

    output.append(
        &"key".to_string(),
        &configurations.iter().map(|it| it.name.clone()).collect(),
        false,
    );

    // Notify user about all configurations we have found
    println!("\nFollowing configurations were found:");
    configurations
        .iter()
        .enumerate()
        .for_each(|(num, configuration)| println!("{}. {}", num + 1, configuration.name));

    // Create resources working pool to not to not to load all the strings.xml into the RAM
    // [StringsKey to [Configuration to Value]]
    let mut pool: HashMap<String, HashMap<String, String>> = HashMap::new();
    // [Configuration to AndroidStringsReader]
    let mut readers: HashMap<String, AndroidStringsReader> = HashMap::new();
    let mut unfinished: Vec<String> = Vec::new();

    configurations.iter().for_each(|configuration| {
        let strings_xml_file = File::open(&configuration.strings_path).unwrap();
        let reader = AndroidStringsReader::from_file(strings_xml_file);
        readers.insert(configuration.name.clone(), reader);
        unfinished.push(configuration.name.clone());
    });

    // Start reading iterations
    let mut i = 0;
    loop {
        read_entry_for_each_configuration(&mut unfinished, &mut readers, &mut pool);
        save_fully_defined_entries(&unfinished, output, &mut pool, warnings);
        if unfinished.is_empty() {
            break;
        }
        if i > MAX_ITERATIONS {
            break;
        }
        i += 1;
    }
    if i - 1 == MAX_ITERATIONS {
        println!("\n{}\n", "Completed due to iteration limit".bold().yellow());
    } else {
        println!("\n{}\n", "Done!".bold().green());
    }
}

fn read_entry_for_each_configuration(
    configurations: &mut Vec<String>,
    readers: &mut HashMap<String, AndroidStringsReader>,
    pool: &mut HashMap<String, HashMap<String, String>>,
) {
    configurations.retain(|configuration| {
        let reader = readers.get_mut(configuration).unwrap();
        match reader.next() {
            Some(entry) => {
                if !pool.contains_key(&entry.key) {
                    pool.insert(entry.key.clone(), HashMap::new());
                }
                let entries = pool.get_mut(&entry.key).unwrap();
                entries.insert(configuration.clone(), entry.value);

                true
            }
            None => {
                // Release RAM resources
                reader.clear();

                false
            }
        }
    });
}

fn save_fully_defined_entries<O, W>(
    configurations: &Vec<String>,
    output: &mut O,
    pool: &mut HashMap<String, HashMap<String, String>>,
    warnings: &mut W,
) where
    O: OutputConsumer,
    W: WarningsCollector,
{
    // Get all strings key names with fully defined configurations
    let mut keys_for_removing: Vec<String> = Vec::new();
    pool.iter().for_each(|(key, entries)| {
        let configurations_fully_defined = configurations.iter().all(|c| entries.contains_key(c));
        if configurations_fully_defined {
            // Save fully defined strings entries to CSV
            let data_to_save = configurations
                .iter()
                .map(|c| {
                    let value = entries.get(c).unwrap().clone();
                    if value.is_empty() {
                        warnings.remember_empty_value(c);
                    }
                    value
                })
                .collect::<Vec<String>>();
            output.append(key, &data_to_save, true);
            // Keep saved keys in memory to clear pool later
            keys_for_removing.push(key.clone());
            warnings.handle_empty_values(key);
            warnings.check_key_name(key, configurations);
        }
    });
    // Remove fully defined and saved keys from pool
    keys_for_removing.iter().for_each(|k| {
        pool.remove(k);
    });
}
