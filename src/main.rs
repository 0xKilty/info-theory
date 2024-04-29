use clap::{App, Arg, ArgGroup};
use std::error::Error;
use std::fs;
use std::io;

#[derive(Debug)]
enum OperationMode {
    Compression,
    Decompression,
}

fn get_flags() -> Result<(String, String, OperationMode), Box<dyn Error>> {
    let matches = App::new("Rust file compressor")
        .version("1.0")
        .author("Ian Kilty")
        .about("Information theory in Rust.")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true)
        )
        .arg(
            Arg::new("compression")
                .short('c')
                .long("compress")
                .conflicts_with("decompression")
        )
        .arg(
            Arg::new("decompression")
                .short('d')
                .long("decompress")
                .conflicts_with("compression")
        )
        .group(
            ArgGroup::new("mode")
                .args(&["compression", "decompression"])
                .required(true)
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap().to_string();
    let output_file = matches
        .value_of("output")
        .map(|o| o.to_string())
        .unwrap_or_else(|| format!("{}_compressed", input_file));

    let mode = if matches.is_present("compression") {
        OperationMode::Compression
    } else {
        OperationMode::Decompression
    };

    Ok((input_file, output_file, mode))
}

fn read_file_and_handle<F>(file_path: &str, handle_contents: F)
where
    F: FnOnce(String),
{
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            handle_contents(contents);
        }
        Err(err) => {
            eprintln!("Error reading input file: {}", err);
            std::process::exit(1);
        }
    }
}

fn compress(input_file: String, output_file: String) {
    read_file_and_handle(&input_file, |contents| {
        println!("File contents:\n{}", contents);
        // Add your compression logic here
    });
}

fn decompress(input_file: String, output_file: String) {
    read_file_and_handle(&input_file, |contents| {
        println!("File contents:\n{}", contents);
        // Add your compression logic here
    });
}

fn main() -> io::Result<()> {
    match get_flags() {
        Ok((input_file, output_file, mode)) => {
            match mode {
                OperationMode::Compression => { compress(input_file, output_file); }
                OperationMode::Decompression => { decompress(input_file, output_file); }
            }
            
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}
