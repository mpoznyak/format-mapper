use std::collections::HashMap;
use calamine::{DataType, Range};
use std::io::Write;
use std::ptr::write;
use crate::cli::{IdGeneratorType, NewLineReplacingType};
use crate::id_generator::{IdGeneratorAutoincrement, IdGeneratorUuid};

const NULL: &str = "null";

pub struct RangeWriter {}

impl RangeWriter {
    pub fn write<W: Write>(dest: &mut W, range: &Range<DataType>, mapping: &HashMap<usize, String>,
                           not_include: usize, trim: bool, id: Option<&String>,
                           id_generator_type: Option<&IdGeneratorType>,
                           new_line: NewLineReplacingType) -> std::io::Result<()> {
        write!(dest, "[").unwrap();
        let size = range.get_size().1 - 1;
        let max_key = mapping.keys().max().expect("Provide schema");
        let mut range_peek = range.rows().skip(not_include).peekable();
        let mut autoin_generator = IdGeneratorAutoincrement::new();
        while let Some(r) = range_peek.next() {
            write!(dest, "{{");
            if let Some(id_column) = id {
                if let Some(gen_type) = id_generator_type {
                    match gen_type {
                        IdGeneratorType::Autoincrement => {
                            write!(dest, "{}", format!("\"{}\":\"{}\",", id_column, autoin_generator.inc()));
                        }
                        IdGeneratorType::Uuid => {
                            let mut generator = IdGeneratorUuid::new();
                            write!(dest, "{}", format!("\"{}\":\"{}\",", id_column, generator.gen()));
                        }
                    }
                } else {
                    panic!("id flag requires generator type, specify it via, ex. '-g autoincrement'")
                }
            }
            for (i, val) in r.iter().enumerate() {
                if !mapping.contains_key(&i) {
                    continue;
                }
                write_key_value_pair(dest, &match mapping.get(&i) {
                    Some(key) => key,
                    _ => panic!("Key not found")
                }, val, trim, new_line);
                if i != size && i != *max_key {
                    write!(dest, ",")?;
                }
            }
            write!(dest, "}}").unwrap();
            if !range_peek.peek().is_none() {
                write!(dest, ",\r\n")?;
            }
        }
        write!(dest, "]")?;
        Ok(())
    }
}

fn write_key_value_pair<W: Write>(dest: &mut W, key: &str, value: &DataType, trim: bool,
                                  new_line: NewLineReplacingType) {
    write!(dest, "{}", format!("\"{}\" : ", key)).unwrap();
    write_value(dest, value, trim, new_line);
}

fn write_value<W: Write>(dest: &mut W, val: &DataType, trim: bool, new_line: NewLineReplacingType) {

    match *val {
        DataType::Empty => write!(dest, "{}", NULL),
        DataType::String(ref s) => match trim {
            true => write!(dest, "\"{}\"", prepare_string(s, trim, new_line)),
            false => write!(dest, "\"{}\"", prepare_string(s, trim, new_line)),
        }
        DataType::Float(ref f) => write!(dest, "\"{}\"", f),
        DataType::DateTime(ref f) => write!(dest, "{}", f),
        DataType::Int(ref i) => write!(dest, "{}", i),
        DataType::Error(ref e) => write!(dest, "{:?}", e),
        DataType::Bool(ref b) => write!(dest, "{}", b),
    }.unwrap();
}


fn prepare_string(val: &String, trim: bool, new_line: NewLineReplacingType) -> String {
    let cloned_val = val.clone();
    let result = match new_line {
        NewLineReplacingType::Ignore => cloned_val,
        NewLineReplacingType::Blank => cloned_val.replace("\n", ""),
        NewLineReplacingType::Whitespace => cloned_val.replace("\n", " "),
    };
    let result = match trim {
        true => result.trim().to_string(),
        false => result
    };
    return result;
}