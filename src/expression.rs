use crate::locationerror::LocError;
use std::panic::Location;
enum Operation {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    And,
    Or,
    Add,
    Sub,
    Div,
    Mul,
}

impl Operation {
    // performs a specific operation on 2 arguments
    fn operate(&self, operand1: String, operand2: String) -> Result<String, LocError> {
        match self {
            Operation::Equal => {
                if operand1 == operand2 {
                    Ok(String::from("TRUE"))
                } else {
                    Ok(String::from("FALSE"))
                }
            }
            Operation::NotEqual => {
                if operand1 != operand2 {
                    Ok(String::from("TRUE"))
                } else {
                    Ok(String::from("FALSE"))
                }
            }
            Operation::GreaterThan => match Self::to_nums(operand1, operand2) {
                Ok((op1, op2)) => {
                    if op1 > op2 {
                        Ok(String::from("TRUE"))
                    } else {
                        Ok(String::from("FALSE"))
                    }
                }
                Err(e) => Err(e),
            },
            Operation::LessThan => match Self::to_nums(operand1, operand2) {
                Ok((op1, op2)) => {
                    if op1 < op2 {
                        Ok(String::from("TRUE"))
                    } else {
                        Ok(String::from("FALSE"))
                    }
                }
                Err(e) => Err(e),
            },
            Operation::And => {
                let op1 = Self::to_bool(operand1);
                let op2 = Self::to_bool(operand2);
                match (op1, op2) {
                    (Ok(op1), Ok(op2)) => {
                        if op1 && op2 {
                            Ok(String::from("TRUE"))
                        } else {
                            Ok(String::from("FALSE"))
                        }
                    }
                    _ => Err(LocError::new(
                        "AND expression wasnt given 2 valid bools",
                        *Location::caller(),
                    )),
                }
            }
            Operation::Or => {
                let op1 = Self::to_bool(operand1);
                let op2 = Self::to_bool(operand2);
                match (op1, op2) {
                    (Ok(op1), Ok(op2)) => {
                        if op1 | op2 {
                            Ok(String::from("TRUE"))
                        } else {
                            Ok(String::from("FALSE"))
                        }
                    }
                    _ => Err(LocError::new(
                        "OR expression wasnt given 2 valid bools",
                        *Location::caller(),
                    )),
                }
            }
            Operation::Add => match Self::to_nums(operand1, operand2) {
                Ok((op1, op2)) => Ok((op1 + op2).to_string()),
                Err(e) => Err(e),
            },
            Operation::Sub => match Self::to_nums(operand1, operand2) {
                Ok((op1, op2)) => Ok((op1 - op2).to_string()),
                Err(e) => Err(e),
            },
            Operation::Div => match Self::to_nums(operand1, operand2) {
                Ok((op1, op2)) => {
                    if op2 == 0.0 {
                        Err(LocError::new(
                            "Attemping to divide by 0! Naughty...",
                            *Location::caller(),
                        ))
                    } else {
                        Ok((op1 / op2).to_string())
                    }
                }
                Err(e) => Err(e),
            },
            Operation::Mul => match Self::to_nums(operand1, operand2) {
                Ok((op1, op2)) => Ok((op1 * op2).to_string()),
                Err(e) => Err(e),
            },
        }
    }

    fn to_nums(operand1: String, operand2: String) -> Result<(f32, f32), LocError> {
        let op1_conv = operand1.parse::<f32>();
        let op2_conv = operand2.parse::<f32>();

        match (op1_conv, op2_conv) {
            (Ok(op1_conv), Ok(op2_conv)) => Ok((op1_conv, op2_conv)),
            _ => Err(LocError::new(
                "Couldnt convert arguments to numbers",
                *Location::caller(),
            )),
        }
    }

    fn to_bool(operand: String) -> Result<bool, LocError> {
        if operand == *"TRUE" {
            Ok(true)
        } else if operand == *"FALSE" {
            Ok(false)
        } else {
            Err(LocError::new("Not a bool", *Location::caller()))
        }
    }
}

// takes the list of tokens, finds any valid polish expressions and evaluates them
pub fn evaluate_polish(mut tokens: Vec<String>) -> Result<Vec<String>, LocError> {
    let operators = ["EQ", "NE", "GT", "LT", "AND", "OR", "+", "-", "*", "/"];

    // find any valid polish expressions
    let mut polish_expression: Vec<String> = Vec::new();
    let (mut flag, mut start_index, mut index) = (false, 0, 0);
    let (mut ops, mut oprs) = (0, 0);
    for token in &tokens {
        if operators.contains(&token.as_str()) {
            if !flag {
                start_index += index;
                flag = true;
            }
            ops += 1;
            polish_expression.push(token.clone());
        } else if flag && oprs != (ops + 1) {
            oprs += 1;
            polish_expression.push(token.clone());
        }
        index += 1;
    }

    let exp_len = polish_expression.len();

    // actually resolve the polish expression, reverse it first cause that makes it a lot easier!
    polish_expression.reverse();
    let mut stack: Vec<String> = Vec::new();
    for token in polish_expression {
        if operators.contains(&token.as_str()) && flag {
            if stack.len() < 2 {
                return Err(LocError::new(
                    "Not enough operands, invalid expression",
                    *Location::caller(),
                ));
            }
            let operation = match token.as_str() {
                "EQ" => Operation::Equal,
                "NE" => Operation::NotEqual,
                "GT" => Operation::GreaterThan,
                "LT" => Operation::LessThan,
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "+" => Operation::Add,
                "-" => Operation::Sub,
                "*" => Operation::Mul,
                _ => Operation::Div,
            };
            let operand1 = stack.pop().unwrap();
            let operand2 = stack.pop().unwrap();
            let res = operation.operate(operand1, operand2)?;
            stack.push(res);
        } else if flag && token != "[" {
            stack.push(token);
        }
    }

    // if a polish expression was resolved, add it into the tokens
    if stack.len() == 1 {
        let mut len = 0;
        tokens = tokens
            .into_iter()
            .enumerate()
            .filter(|(index, _)| {
                len += 1;
                *index < start_index || len > (exp_len + 1)
            })
            .map(|(_, token)| token)
            .collect();
        tokens.insert(start_index, stack.pop().unwrap());
    } else if flag {
        return Err(LocError::new("Invalid expression", *Location::caller()));
    }
    Ok(tokens)
}
