// Scanner takes in raw source code as a series of characters and groups it into a series
// of chunks called tokens ("words", and "punctuation" that make up the language's grammar)

use std::fs;
use std::io;

// When do we return io::Error??
// REPL
pub fn run_file(path: String) -> Result<(), io::Error> {
    let file = fs::read_to_string(path).expect("Failed to read file.");
    // read file as bytes??

    Ok(run(file))
}

pub fn run_prompt() -> Result<(), io::Error> {
    let mut null_input_inc: u8 = 0;
    let mut input = String::new();
    println!("Please enter the file to be compiled: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get user input.\n");

    loop {
        if !input.trim().is_empty() {
            return Ok(run(input.clone()));
        } else if null_input_inc >= 5 {
            let error = io::Error::new(io::ErrorKind::Other, "Failed to receive input >5 times");
            return Err(error);
        } else {
            null_input_inc += 1;
            println!("Failed to get input, let's try again:\n");
        }
    }
}

// When do we return io::Error??
// Should arg be Byte String, Byte Array, or normal string??
fn run(source: String) {
    // source file as byte vector
    let source_str = fs::read_to_string(source).expect("Failed to read file");
    // maybe consider rayon::par_iter somewhere? Depending on file size. Could be future optimization
    let source_bv = source_str.as_bytes();
}
