use crate::Liberty;
use serde_json::json;
use std::{fs, io::Result, path::Path};
impl Liberty {
    pub fn dump_library<P>(&self, file_path: &P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = file_path.as_ref();
        let file_name = format!("{}{}", self.name, ".library.lib.json".to_string());
        let data = json!({
            "library":self.name,
            "single_attribute":self.single_attribute,
            "group_attribute":self.group_attribute,

        });
        fs::write(path.join(file_name), data.to_string())?;

        Ok(())
    }
    pub fn dump_cell<P>(&self, file_path: &P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = file_path.as_ref();
        for (k, v) in &self.cell {
            let file_name = format!("{}{}", self.name, ".cell.lib.json".to_string());
            let data = json!({
                "cell":k,
                "attribute": v,
            });
            fs::write(path.join(file_name), data.to_string())?;
        }

        Ok(())
    }
}
