use crate::ast::LibraryType;
use std::fs;
use std::io::Result;
use std::path::Path;
impl LibraryType {
    pub fn dump_library<P>(&self, file_path: &P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = file_path.as_ref();
        let mut library_json = json::JsonValue::new_object();
        let file_name = format!("{}{}", self.name, "_library.lib.json".to_string());
        library_json["name"] = self.name.clone().into();
        library_json["library"] = self.library.clone();
        fs::write(path.join(file_name), library_json.pretty(4))?;

        Ok(())
    }
    pub fn dump_cell<P>(&self, file_path: &P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = file_path.as_ref();
        let cells = &self.cell;
        for a_cell in cells {
            let file_name = format!("{}{}", a_cell["name"], "_cell.lib.json".to_string());
            fs::write(path.join(file_name), a_cell.pretty(4))?;
        }

        Ok(())
    }
    // pub fn dump_all<P>(&self, file_path: P) -> Result<()>
    // where
    //     P: AsRef<Path>,
    // {
    //     let path = file_path.as_ref();
    //     let mut json_file = json::JsonValue::new_object();
    //     let file_name = format!("{}{}", self.name, ".lib.json".to_string());
    //     json_file["name"] = self.name.clone().into();
    //     json_file["attribute"] = self.attribute.clone();
    //     json_file["group"] = self.group.clone();
    //     json_file["cell"] = self.cell.clone().into();
    //     fs::write(path.join(file_name), json_file.dump())?;
    //     Ok(())
    // }
}
