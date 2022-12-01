extern crate core;

mod config;
mod writer_json;
mod cli;
pub mod id_generator;

use calamine::{open_workbook_auto, DataType, Range, Reader};
use std::env;
use std::env::Args;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use crate::cli::Cli;
use crate::config::{Config};
use crate::writer_json::RangeWriter;
use clap::Parser;

fn main() {
    xls_to_json();
}

fn xls_to_json() {
    let cli: Cli = Cli::parse();
    println!("{:?}", cli);
    let config: Config = Config::new(cli);
    let path_buf = PathBuf::from(&config.file);
    validate(&path_buf);

    let dest = path_buf.with_extension("json");
    let mut dest = BufWriter::new(File::create(dest).unwrap());
    let mut xl = open_workbook_auto(&path_buf).unwrap();
    let range = xl.worksheet_range(&config.sheet).unwrap().unwrap();

    RangeWriter::write(&mut dest,
                       &range,
                       &config.mapping,
                       config.not_include,
                       config.trim,
                       config.id.as_ref(),
                       config.id_generator.as_ref(),
                       config.new_line).unwrap()
}

fn validate(path_buf: &PathBuf) {
    match path_buf.extension().and_then(|s| s.to_str()) {
        Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => (),
        _ => panic!("Expecting an excel file")
    }
}
