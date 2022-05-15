use std::fs::{self, DirEntry};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Configuration {
    pub name: String,
    pub strings_path: String,
}

pub fn resolve_res_dir(res_path: &String) -> Vec<Configuration> {
    if let Ok(read_dir) = fs::read_dir(&res_path) {
        read_dir
            .filter(|entry| is_dir(entry.as_ref().unwrap()))
            .filter_map(|entry| as_configuration_or_none(entry.as_ref().unwrap()))
            .sorted_by(|a, b| Ord::cmp(&a.name, &b.name))
            .collect::<Vec<Configuration>>()
    } else {
        panic!(
            "Cannot access path `{}`. \nMake sure that:\n\
            - the path exists, \n\
            - the path leads to the directory, \n\
            - the program has access to view the contents of the directory.",
            &res_path,
        )
    }
}

fn is_dir(entry: &DirEntry) -> bool {
    entry
        .metadata()
        .expect("Something went wrong while getting files metadata in `res` dir")
        .is_dir()
}

fn as_configuration_or_none(entry: &DirEntry) -> Option<Configuration> {
    let dir_name = entry
        .file_name()
        .to_str()
        .map(|s| String::from(s))
        .expect("Something went wrong while getting subdirectory filename");
    if dir_name.starts_with("values") {
        if let Some(strings_path) = find_strings_or_none(entry) {
            Some(Configuration {
                name: dir_name,
                strings_path: strings_path,
            })
        } else {
            None
        }
    } else {
        None
    }
}

fn find_strings_or_none(entry: &DirEntry) -> Option<String> {
    let mut strings_path = entry.path().clone();
    strings_path.push("strings.xml");
    if strings_path.exists() {
        Some(strings_path.to_str().map(|s| String::from(s)).unwrap())
    } else {
        None
    }
}
