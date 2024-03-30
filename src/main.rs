mod command;
mod expression;
mod interpreter;
mod locationerror;
mod turtle;

use clap::Parser;
use locationerror::LocError;
use unsvg::Image;

use crate::command::{check_procedures, Command};
use crate::interpreter::execute;
use crate::turtle::Turtle;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::panic::Location;

/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Tokens {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}

fn main() -> Result<(), LocError> {
    let tokens: Tokens = Tokens::parse();

    // Access the parsed arguments
    let file_path = tokens.file_path;
    let image_path = tokens.image_path;
    let height = tokens.height;
    let width = tokens.width;

    // Format the file into a vec of commands, empty lines are excluded
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            return Err(LocError::new(
                "couldn't access file path",
                *Location::caller(),
            ))
        }
    };
    let reader = BufReader::new(file);
    let mut commands: Vec<Command> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if !line.trim().is_empty() {
            let line: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
            commands.push(Command::new(line));
        }
    }

    // Check that all procedures are valid (every TO has an END).
    check_procedures(&commands)?;

    // execute the functionality of all the commands.
    let turtle = Turtle::new((height as f32 / 2.0, width as f32 / 2.0));
    let mut image = Image::new(width, height);
    execute(commands, &mut image, turtle)?;

    // save the image to the correct path - its updated here
    match image_path.extension().and_then(|s| s.to_str()) {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if res.is_err() {
                return Err(LocError::new(
                    "couldn't save to image path",
                    *Location::caller(),
                ));
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if res.is_err() {
                return Err(LocError::new(
                    "couldn't save to image path",
                    *Location::caller(),
                ));
            }
        }
        _ => {
            return Err(LocError::new(
                "File extension not supported",
                *Location::caller(),
            ));
        }
    }
    Ok(())
}
