mod ast;
mod dump;
mod parser;
pub use ast::{CellType, Liberty, LibraryType};
use log::info;
use nom::{error::VerboseError, IResult};
use std::fs;
use std::path::Path;
use std::time::*;
pub type LibRes<T, U> = IResult<T, U, VerboseError<T>>;
pub use parser::liberty_parser::*;
pub fn convert_lib<P>(source: P, destinate: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    info!("Starting Parse the liberty...");
    let now = SystemTime::now();
    let liberty_data: Liberty = fs::read_to_string(source)?.parse()?;
    let passed = SystemTime::now();
    info!("Finish Parsing, Time Used:{:?}", passed.duration_since(now));
    info!("Starting Dumping...");
    let now = SystemTime::now();
    if liberty_data.cell.is_empty() {
        info!("Detect library liberty file, only library json will be dumped");
        liberty_data.library.dump(&destinate)?;
    } else {
        info!("Detect complete liberty file, library and cell json will be dumped");
        liberty_data.library.dump(&destinate)?;
        for cell in liberty_data.cell {
            cell.dump(&destinate)?;
        }
    }
    let passed = SystemTime::now();
    info!("Finish Dumping, Time Used:{:?}", passed.duration_since(now));
    Ok(())
}
