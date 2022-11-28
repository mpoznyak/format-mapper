use std::collections::HashMap;
use calamine::{DataType, Range};
use std::io::Write;
use std::ptr::write;
use crate::cli::IdGeneratorType;
use crate::id_generator::{IdGeneratorAutoincrement, IdGeneratorUuid};

const NULL: &str = "null";

pub struct RangeWriter {}

impl RangeWriter {
    pub fn write<W: Write>(dest: &mut W, range: &Range<DataType>, mapping: &HashMap<usize, String>,
                           not_include: usize, trim: bool, id: Option<&String>, id_generator_type: Option<&IdGeneratorType>) -> std::io::Result<()> {

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
                }, val, trim);
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

fn write_key_value_pair<W: Write>(dest: &mut W, key: &str, value: &DataType, trim: bool) {
    write!(dest, "{}", format!("\"{}\" : ", key)).unwrap();
    write_value(dest, value, trim);
}

fn write_value<W: Write>(dest: &mut W, val: &DataType, trim: bool) {
    match *val {
        DataType::Empty => write!(dest, "{}", NULL),
        DataType::String(ref s) => match trim {
            true => write!(dest, "\"{}\"", s.trim()),
            false => write!(dest, "\"{}\"", s),
        }
        DataType::Float(ref f) => write!(dest, "\"{}\"", f),
        DataType::DateTime(ref f) => write!(dest, "{}", f),
        DataType::Int(ref i) => write!(dest, "{}", i),
        DataType::Error(ref e) => write!(dest, "{:?}", e),
        DataType::Bool(ref b) => write!(dest, "{}", b),
    }.unwrap();
}
