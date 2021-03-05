use crate::{CellType, LibraryType};
use json::JsonValue;
use std::fs;
use std::io::Result;
use std::path::Path;
impl LibraryType {
    pub fn dump<P>(&self, file_path: &P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = file_path.as_ref();
        let mut a_library = JsonValue::new_object();
        let file_name = format!("{}{}", self.name, ".library.lib.json".to_string());
        a_library[&self.name] = JsonValue::Array(self.lib_attribute.clone());
        fs::write(path.join(file_name), a_library.pretty(4))?;

        Ok(())
    }
}

impl CellType {
    pub fn dump<P>(&self, file_path: &P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = file_path.as_ref();
        let mut a_cell = JsonValue::new_object();
        a_cell[&self.name] = JsonValue::Array(self.cell_attribute.clone());
        let file_name = format!("{}{}", self.name, ".cell.lib.json".to_string());
        fs::write(path.join(file_name), a_cell.pretty(4))?;

        Ok(())
    }
}
