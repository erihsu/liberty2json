use crate::{CellType, LibraryType};

use std::fs;
use std::io::Result;
use std::path::Path;
impl LibraryType {
    pub fn dump<P>(&self, file_path: &P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = file_path.as_ref();
        let file_name = format!("{}{}", self.name, ".library.lib.json".to_string());
        fs::write(path.join(file_name), self.lib_attribute.pretty(4))?;

        Ok(())
    }
}

impl CellType {
    pub fn dump<P>(&self, file_path: &P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = file_path.as_ref();
        let file_name = format!("{}{}", self.name, ".cell.lib.json".to_string());
        fs::write(path.join(file_name), self.cell_attribute.pretty(4))?;

        Ok(())
    }
}
