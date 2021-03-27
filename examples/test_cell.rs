use liberty2json::cell_parser;
fn main() {
    use std::{fs, io::Read};
    let mut f = fs::File::open("latch_cell.lib").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();
    let (_, _) = cell_parser(&data).unwrap();
}
