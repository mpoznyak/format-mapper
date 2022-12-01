use std::collections::HashMap;
use std::env::Args;
use crate::Cli;
use crate::cli::{IdGeneratorType, NewLineReplacingType};

pub struct Config {
    pub file: String,
    pub sheet: String,
    pub mapping: HashMap<usize, String>,
    pub not_include: usize,
    pub trim: bool,
    pub id: Option<String>,
    pub id_generator: Option<IdGeneratorType>,
    pub new_line: NewLineReplacingType
}

impl Config {
    pub fn new(cli: Cli) -> Self {
        Self {
            file: cli.file,
            sheet: cli.sheet,
            mapping: Config::parse_mapping(cli.mapping.as_str()),
            trim: cli.trim,
            not_include: match cli.not_include {
                Some(item) => item,
                None => 0
            },
            id: cli.id,
            id_generator: cli.id_generator,
            new_line: cli.newline_replacing
        }
    }
}

impl Config {

    fn parse_mapping(arg: &str) -> HashMap<usize, String> {
        return Config::parse_keys(arg);
    }

    fn parse_keys(arg: &str) -> HashMap<usize, String> {
        let mut words = HashMap::new();
        for word in arg.split(',') {
            let parts: Vec<&str> = word.split(':').collect();
            words.insert(parts[0].parse::<usize>().unwrap(), parts[1].to_string());
        }
        return words;
    }
}

