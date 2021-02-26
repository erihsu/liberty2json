mod ast;
mod dump;
mod parser;
pub use ast::LibraryType;
use log::info;
use nom::{error::VerboseError, IResult};
use std::fs;
use std::path::Path;
pub type LibRes<T, U> = IResult<T, U, VerboseError<T>>;

pub fn convert_lib<P>(source: P, destinate: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let lib_data: LibraryType = fs::read_to_string(source)?.parse()?;
    if lib_data.cell.is_empty() {
        info!("[Lib2Json] INFO detect library liberty file, only library json will be dumped");
        lib_data.dump_library(&destinate)?;
    } else {
        info!("[Lib2Json] INFO detect complete liberty file, library and cell json will be dumped");
        lib_data.dump_library(&destinate)?;
        lib_data.dump_cell(&destinate)?;
    }
    Ok(())
}
