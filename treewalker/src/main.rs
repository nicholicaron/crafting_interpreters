use clap::{App, Arg};

mod scanner;

// Scanner takes in raw source code as a series of characters and groups it into a series
// of chunks called tokens ("words", and "punctuation" that make up the language's grammar)

fn main() {
    let matches = App::new("treewalker")
        .version("0.1.0")
        .author("Nicholi Caron <nmcaron@protonmail.ch>")
        .about("A treewalk interpreter for the Lox language written in Rust")
        .arg(
            Arg::new("filename")
                .short('f')
                .takes_value(true)
                .value_name("FILENAME")
                .help("Name of file to be compiled")
                .required(false)
                .max_values(1),
        )
        .get_matches();

    if let Some(f) = matches.get_one::<String>("filename") {
        // Impl error handling here if file is unavailable
        let scan = scanner::run_file(f.clone());
    } else {
        let scan = scanner::run_prompt();
    }
}
