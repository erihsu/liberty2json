mod ast;
mod dump;
mod parser;
pub use ast::LibraryType;
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
    info!("[Lib2Json] INFO Starting Parse the liberty...");
    let now = SystemTime::now();
    let lib_data: LibraryType = fs::read_to_string(source)?.parse()?;
    let passed = SystemTime::now();
    info!(
        "[Lib2Json] INFO Finish Parsing, Time Used:{:?}",
        passed.duration_since(now)
    );
    info!("[Lib2Json] INFO Starting Dumping...");
    let now = SystemTime::now();
    if lib_data.cell.is_empty() {
        info!("[Lib2Json] INFO detect library liberty file, only library json will be dumped");
        lib_data.dump_library(&destinate)?;
    } else {
        info!("[Lib2Json] INFO detect complete liberty file, library and cell json will be dumped");
        lib_data.dump_library(&destinate)?;
        lib_data.dump_cell(&destinate)?;
    }
    let passed = SystemTime::now();
    info!(
        "[Lib2Json] INFO Finish Dumping, Time Used:{:?}",
        passed.duration_since(now)
    );
    Ok(())
}
