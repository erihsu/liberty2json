use clap::App;
use liberty2json::convert_lib;
fn main() {
    // from clap examples
    let matches = App::new("lib2json")
        .version("0.1.0")
        .author("Eric Hsu <zhenyuxu163@163.com>")
        .about("Convert liberty file to json")
        .args_from_usage(
            "
			<INPUT> 	'Sets the input liberty file'
			-o --output=[FOLDER] 'Sets a output folder'

			",
        )
        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let output_folder = matches.value_of("FOLDER").unwrap();
    convert_lib(input_file, output_folder).unwrap();
}
