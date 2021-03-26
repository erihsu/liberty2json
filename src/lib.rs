use log::info;
use nom::{error::VerboseError, IResult};
use serde_json::Value;
use std::{fs, path::Path, time::*};

pub use ast::Liberty;
pub use parser::liberty_parser::*;

mod ast;
mod dump;
mod parser;

pub fn convert_lib<P>(source: P, destinate: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    info!("Starting Parse the liberty...");
    let now = SystemTime::now();
    let liberty_data: Liberty = fs::read_to_string(source)?.parse().unwrap();
    let passed = SystemTime::now();
    info!("Finish Parsing, Time Used:{:?}", passed.duration_since(now));
    info!("Starting Dumping...");
    let now = SystemTime::now();
    if liberty_data.cell.is_empty() {
        info!("Detect library liberty file, only library json will be dumped");
        liberty_data.dump_library(&destinate)?;
    } else {
        info!("Detect complete liberty file, library and cell json will be dumped");
        liberty_data.dump_cell(&destinate)?;
    }
    let passed = SystemTime::now();
    info!("Finish Dumping, Time Used:{:?}", passed.duration_since(now));
    Ok(())
}

pub type LibertyJson = Value;
pub type LibRes<T, U> = IResult<T, U, VerboseError<T>>;
