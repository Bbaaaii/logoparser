use crate::Turtle;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::{collections::HashMap, panic::Location};

use crate::locationerror::LocError;

#[derive(Debug)]
pub struct Command {
    pub arg: Vec<String>,
}

impl Command {
    // Create a new command struct.
    pub fn new(command: Vec<String>) -> Self {
        Command { arg: command }
    }

    // Checks that the command has the correct num of arguments, useful as it allows use of unwrap() later with certainty.
    pub fn check_command(&self) -> Result<String, LocError> {
        let operators = ["EQ", "NE", "GT", "LT", "AND", "OR", "+", "-", "*", "/"];
        // First token will always exist since commands must be non-empty.
        let first_arg: &str = self.first_token();
        match first_arg {
            "PENUP" | "PENDOWN" | "END" if self.arg.len() != 1 => Err(LocError::new(
                "Incorrect num arguments",
                *Location::caller(),
            )),
            "TO" if self.arg.len() < 2 => Err(LocError::new(
                "Incorrect num arguments",
                *Location::caller(),
            )),
            "IF" | "WHILE" => {
                if self.arg.len() >= 3 {
                    if let Some(last) = self.arg.last() {
                        if last.as_str() == "[" {
                            return Ok(String::from("Expression"));
                        }
                    }
                    Err(LocError::new("Missing [", *Location::caller()))
                } else {
                    Err(LocError::new(
                        "Incorrect num arguments",
                        *Location::caller(),
                    ))
                }
            }
            "FORWARD" | "BACK" | "LEFT" | "RIGHT" | "SETPENCOLOR" | "TURN" | "SETHEADING"
            | "SETX" | "SETY"
                if self.arg.len() != 2 =>
            {
                if let Some(v) = self.arg.get(1) {
                    if operators.contains(&v.as_str()) {
                        return Ok(String::from("Expression"));
                    }
                }
                Err(LocError::new(
                    "Incorrect num arguments",
                    *Location::caller(),
                ))
            }
            "MAKE" | "ADDASSIGN" if self.arg.len() != 3 => {
                if let Some(v) = self.arg.get(2) {
                    if operators.contains(&v.as_str()) {
                        return Ok(String::from("Expression"));
                    }
                }
                Err(LocError::new(
                    "Incorrect num arguments",
                    *Location::caller(),
                ))
            }
            _ => Ok(String::from("No Expression")),
        }
    }

    // Takes a vec of arguments from a command and formats them by removing the first element and substituting variables for values.
    pub fn get_tokens(
        &self,
        turtle: &Turtle,
        vars: &HashMap<String, String>,
    ) -> Result<Vec<String>, LocError> {
        // Check if this command is a comment, if it is "invalid" tokens are allowed
        let mut function_name = false;
        if self.first_token() == "//" {
            return Ok(Vec::new());
        } else if self.first_token() == "TO" {
            function_name = true;
        }

        let tokens: Result<Vec<String>, LocError> = self
            .arg
            .iter()
            .skip(1)
            .map(|s| {
                let full_str = s;
                let mut s = s.chars();
                let first = s.next().unwrap();
                if first == ':' {
                    // Deals with variables, either extracts the variable
                    match vars.get(&s.collect::<String>()) {
                        Some(variable) => Ok(variable.to_string()),
                        None => Err(LocError::new("Variable not found", *Location::caller())),
                    }
                } else if first == '"' {
                    Ok(s.collect::<String>())
                } else {
                    // queries
                    match full_str.as_str() {
                        "XCOR" => Ok(turtle.coords.0.to_string()),
                        "YCOR" => Ok(turtle.coords.1.to_string()),
                        "HEADING" => Ok(turtle.heading.to_string()),
                        "COLOR" => Ok(turtle.colour.to_string()),
                        "EQ" | "NE" | "GT" | "LT" | "AND" | "OR" | "+" | "-" | "*" | "/" | "[" => {
                            Ok(full_str.to_string())
                        }
                        _ => {
                            if function_name {
                                function_name = false;
                                return Ok(full_str.to_string());
                            }
                            return Err(LocError::new("Invalid token given", *Location::caller()));
                        }
                    }
                }
            })
            .collect::<Result<_, _>>();

        tokens
    }

    pub fn first_token(&self) -> &str {
        // get the first token -> unformatted
        self.arg.first().unwrap()
    }
}

/*
    Checks that we dont have any loose "TO"'s or "END"'s floating about in the file.
    The num of each must be equal or return an appropriate error message.
*/
pub fn check_procedures(commands: &[Command]) -> Result<(), LocError> {
    let to = commands.iter().filter(|s| s.first_token() == "TO").count();
    let end = commands.iter().filter(|s| s.first_token() == "END").count();

    match to.cmp(&end) {
        Equal => Ok(()),
        Greater => Err(LocError::new(
            "too many toos, not enough ends",
            *Location::caller(),
        )),
        Less => Err(LocError::new(
            "too many ends, not enough toos",
            *Location::caller(),
        )),
    }
}
