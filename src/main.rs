use clap::{App, Arg};
use std::error::Error;
use std::fs;
use std::io;

fn get_file_names() -> Result<(String, Option<String>), Box<dyn Error>> {
    let matches = App::new("Rust file compressor")
        .version("1.0")
        .author("Ian Kilty")
        .about("Information theory in Rust.")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap().to_string();
    let output_file = matches.value_of("output").map(String::from);

    Ok((input_file, output_file))
}

fn main() -> io::Result<()> {
    match get_file_names() {
        Ok((input_file, output_file)) => {
            if let Some(output_file) = output_file {
                println!("Output file: {}", output_file);

                if let Ok(contents) = fs::read_to_string(input_file) {
                    println!("Input file contents: {}", contents);
                } else {
                    eprintln!("Error reading input file");
                    std::process::exit(1);
                }
            } else {
                println!("No output file specified.");
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}
