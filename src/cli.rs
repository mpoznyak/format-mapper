use std::collections::HashMap;
use clap::{Parser, Command, ArgGroup, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "format-mapper")]
#[command(author = "Maxim Poznyak")]
#[command(version = "1.0")]
#[command(about = "Does xls conversion to json", long_about = None)]
pub struct Cli {
    /// path to file - required
    #[arg(short='f', long)]
    pub file: String,

    /// sheet name - required
    #[arg(short='s', long)]
    pub sheet: String,

    /// mapping: column_number:field_name - required
    #[arg(short='m', long)]
    pub mapping: String,

    /// number of excel rows which must be not included in parsing
    #[arg(short='n', long)]
    pub not_include: Option<usize>,

    /// trim whitespaces after parsing - default true
    #[arg(short='t', long)]
    pub trim: bool,

    /// append id field with a specified name
    #[arg(short='i', long)]
    pub id: Option<String>,

    /// id generator, required if id flag is specified
    #[arg(value_enum, short='g', long="igen")]
    pub id_generator: Option<IdGeneratorType>
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum IdGeneratorType {
    /// Autoincrement, starting from one
    Autoincrement,
    ///Generate UUID for each id field
    Uuid,
}