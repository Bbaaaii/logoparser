use crate::expression::evaluate_polish;
use crate::turtle::{PenState, Turtle};
use crate::Command;
use crate::Image;
use crate::LocError;

use std::collections::HashMap;
use std::panic::Location;

struct Function {
    func_name: String,
    func_vars: Vec<String>,
    func_index: usize,
}

pub fn execute(
    commands: Vec<Command>,
    image: &mut Image,
    mut turtle: Turtle,
) -> Result<(), LocError> {
    // Create the collections of variables and functions!
    let mut variables: HashMap<String, String> = HashMap::new();
    let mut functions: Vec<Function> = Vec::new();

    // Control flow variables.
    let mut index = 0;
    let mut while_if_stack: Vec<usize> = Vec::new();
    let mut func_stack: Vec<usize> = Vec::new();

    // Execute through the vec of commands
    loop {
        // Exit if all commands have been executed; otherwise get the next command and execute on it
        if index >= commands.len() {
            break;
        }

        // Get the command to execute on this loop
        let command = commands.get(index).unwrap();

        // Check command for valid num of arguments, allows for unwrapping later!
        command.check_command()?;

        // get tokens for this specific command and evaluate any potential polish expressions
        let mut tokens = command.get_tokens(&turtle, &variables)?;
        tokens = evaluate_polish(tokens)?;

        // Execute the specific behaviour of the command.
        match command.first_token() {
            "//" => (),
            "PENUP" => turtle.change_penstate(PenState::Up),
            "PENDOWN" => turtle.change_penstate(PenState::Down),
            "SETPENCOLOR" => match tokens.first().unwrap().parse::<usize>() {
                Ok(code @ 0..=16) => turtle.change_colour(code),
                _ => {
                    return Err(LocError::new(
                        "Invalid colour, colour must be an integer between 0 and 16",
                        *Location::caller(),
                    ));
                }
            },
            first_arg @ ("FORWARD" | "BACK" | "LEFT" | "RIGHT") => {
                let direction = match first_arg {
                    "BACK" => 180,
                    "RIGHT" => 90,
                    "LEFT" => 270,
                    _ => 0,
                };
                match tokens.first().unwrap().parse::<f32>() {
                    Ok(distance) => turtle.draw(image, direction, distance),
                    Err(_) => {
                        return Err(LocError::new(
                            "Unable to convert to a float!",
                            *Location::caller(),
                        ));
                    }
                }
            }
            "TURN" => match tokens.first().unwrap().parse::<i32>() {
                Ok(turn) => turtle.turn(turn),
                Err(_) => {
                    return Err(LocError::new(
                        "Unable to convert to a number!",
                        *Location::caller(),
                    ));
                }
            },
            "SETHEADING" => match tokens.first().unwrap().parse::<i32>() {
                Ok(new_heading) => turtle.change_heading(new_heading),
                Err(_) => {
                    return Err(LocError::new(
                        "Unable to convert to a number!",
                        *Location::caller(),
                    ));
                }
            },
            "SETX" => match tokens.first().unwrap().parse::<f32>() {
                Ok(x) => turtle.change_x(x),
                Err(_) => {
                    return Err(LocError::new(
                        "Unable to convert to a float!",
                        *Location::caller(),
                    ))
                }
            },
            "SETY" => match tokens.first().unwrap().parse::<f32>() {
                Ok(y) => turtle.change_y(y),
                Err(_) => {
                    return Err(LocError::new(
                        "Unable to convert to a float!",
                        *Location::caller(),
                    ))
                }
            },
            "MAKE" => {
                // Create a variable with the given name and value.
                let name: String = tokens.first().unwrap().to_string();
                match tokens.get(1).unwrap().parse::<f32>() {
                    Ok(value) => {
                        variables.insert(name.clone(), value.to_string());
                    }
                    Err(_) => {
                        // If the variable could not be passed as a
                        if let Some(value) = tokens.get(1) {
                            if value.as_str() == "TRUE" || value.as_str() == "FALSE" {
                                variables.insert(name.clone(), value.clone());
                            } else {
                                return Err(LocError::new(
                                    "Unable to convert to a float!",
                                    *Location::caller(),
                                ));
                            }
                        } else {
                            return Err(LocError::new(
                                "Unable to convert to a float!",
                                *Location::caller(),
                            ));
                        }
                    }
                }
            }
            "ADDASSIGN" => {
                // Find the variable to add to
                let var = match variables.get_mut(tokens.first().unwrap()) {
                    Some(variable) => variable,
                    None => {
                        return Err(LocError::new(
                            "No variable with that name!",
                            *Location::caller(),
                        ))
                    }
                };

                // Extract the value to add to the found variable
                match (var.parse::<f32>(), tokens.get(1).unwrap().parse::<f32>()) {
                    (Ok(var_val), Ok(val_to_add)) => {
                        variables.insert(
                            tokens.first().unwrap().clone(),
                            (var_val + val_to_add).to_string(),
                        );
                    }
                    _ => {
                        return Err(LocError::new(
                            "Unable to convert to a float!",
                            *Location::caller(),
                        ))
                    }
                }
            }
            "IF" => {
                let condition = tokens.first().unwrap();

                // Find the index of the if statements "]".
                let mut if_end = 0;
                let found = find_conditional_end(&mut if_end, &index, &commands);

                if condition.as_str() == "FALSE" {
                    // Jump to the next "]" on a false.
                    if !found {
                        return Err(LocError::new(
                            "No end of if statement conditional!",
                            *Location::caller(),
                        ));
                    }
                    index += if_end - index;
                } else if condition.as_str() == "TRUE" {
                    // Push the line after the "]" to the stack to pass the "]" command checker.
                    while_if_stack.push(if_end + 1);
                }
            }
            "WHILE" => {
                let condition = tokens.first().unwrap();

                // Find the index of the while statements "]".
                let mut while_end = 0;
                let found = find_conditional_end(&mut while_end, &index, &commands);

                if condition.as_str() == "FALSE" {
                    // Jump to the next "]" on a false.
                    if !found {
                        return Err(LocError::new(
                            "No end of while statement conditional!",
                            *Location::caller(),
                        ));
                    }
                    index += while_end - index;
                } else if condition.as_str() == "TRUE" {
                    // Otherwise prepare to jump to the start of the while loop!
                    while_if_stack.push(index);
                }
            }
            "]" => {
                // Pop an index from the while-if stack and jump to it!
                if let Some(stack_index) = while_if_stack.pop() {
                    index = stack_index - 1;
                }
            }
            "TO" => {
                // Collect func_vars
                let mut func_vars: Vec<String> = Vec::new();
                for var_name in tokens.iter().skip(1) {
                    func_vars.push(var_name.to_string());
                }

                // Add the function to the list of functions.
                functions.push(Function {
                    func_name: tokens.first().unwrap().to_string(),
                    func_vars,
                    func_index: index,
                });

                let mut func_end = 0;
                if find_func_end(&mut func_end, &index, &commands) {
                    index += func_end - index;
                } else {
                    return Err(LocError::new("No end to procedure!", *Location::caller()));
                }
            }
            "END" => {
                // Pop an index from the func stack and jump to it!
                if let Some(stack_index) = func_stack.pop() {
                    index = stack_index;
                }
            }
            value => {
                // Checks if the func exists, and returns or throws an error accordingly
                let func = match &functions.iter().find(|v| v.func_name == value) {
                    Some(func) => *func,
                    None => {
                        return Err(LocError::new(
                            "No function with that name found",
                            *Location::caller(),
                        ));
                    }
                };

                // Bind variables to the new names
                for (index, var_name) in func.func_vars.iter().enumerate() {
                    // For
                    let value = match tokens.get(index) {
                        Some(value) => value,
                        None => {
                            return Err(LocError::new(
                                "Missing procedure argument",
                                *Location::caller(),
                            ))
                        }
                    };
                    variables.insert(var_name.to_string(), value.to_string());
                }
                func_stack.push(index);
                index = func.func_index;
            }
        };
        index += 1;
    }

    if !while_if_stack.is_empty() {
        return Err(LocError::new(
            "Stack not empty at end of program",
            *Location::caller(),
        ));
    }
    Ok(())
}

// Finds the ending "]" of a conditional (IF/WHILE)
fn find_conditional_end(curr_index: &mut usize, index: &usize, commands: &[Command]) -> bool {
    let mut found = false;
    let mut stack = 0;
    for command in commands {
        let first = command.first_token();
        if *curr_index > *index && ((first == "WHILE") | (first == "IF")) {
            stack += 1;
        } else if *curr_index > *index && first == "]" {
            if stack == 0 {
                found = true;
                break;
            }
            stack -= 1;
        }
        *curr_index += 1;
    }
    found
}

// Finds the ending "END" of a function
fn find_func_end(curr_index: &mut usize, index: &usize, commands: &[Command]) -> bool {
    let mut found = false;
    for command in commands {
        if *curr_index > *index && command.first_token() == "END" {
            found = true;
            break;
        }
        *curr_index += 1;
    }
    found
}
