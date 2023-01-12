use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ZeroDivisionError(String);

impl Error for ZeroDivisionError {}

impl fmt::Display for ZeroDivisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    NOP,
}
#[derive(Debug)]
struct Operation {
    operator: Operator,
    left_operand: f32,
    right_operand: f32,
}

impl Operation {
    fn _new(operator: Operator, left_operand: f32, right_operand: f32) -> Self {
        Self {
            operator,
            left_operand,
            right_operand,
        }
    }

    fn parse(string: &'static str) -> Self {
        let string = string.replace(" ", "");
        let operands: Vec<&str>;
        let operator: Operator;

        if let Some(_) = string.find("+") {
            operator = Operator::Addition;
            operands = string.split("+").collect();
        } else if let Some(_) = string.find("-") {
            operator = Operator::Subtraction;
            operands = string.split("-").collect();
        } else if let Some(_) = string.find("*") {
            operator = Operator::Multiplication;
            operands = string.split("*").collect();
        } else if let Some(_) = string.find("/") {
            operator = Operator::Division;
            operands = string.split("/").collect();
        } else {
            operator = Operator::NOP;
            operands = vec!["0.0", "0.0"]
        }

        let left_operand: f32 = operands[0].parse().expect("Could not parse left operand.");
        let right_operand: f32 = operands[1].parse().expect("Could not parse right operand.");

        Self {
            left_operand: left_operand,
            right_operand: right_operand,
            operator,
        }
    }

    fn execute(&self) -> Result<f32, Box<dyn Error>> {
        match self.operator {
            Operator::Addition => Ok(self.left_operand + self.right_operand),
            Operator::Subtraction => Ok(self.left_operand - self.right_operand),
            Operator::Multiplication => Ok(self.left_operand * self.right_operand),
            Operator::Division => {
                if self.right_operand > 0 as f32 {
                    Ok(self.left_operand / self.right_operand)
                } else {
                    Err(Box::new(ZeroDivisionError("Divide by zero".to_string())))
                }
            }

            Operator::NOP => Ok(0.0),
        }
    }
}

fn p2(operations: Vec<&'static str>) -> Vec<String> {
    let mut results = Vec::new();

    for operation in operations {
        let parsed_operation = Operation::parse(operation);

        match parsed_operation.execute() {
            Ok(result) => results.push(result.to_string()),
            Err(error) => {
                if error.to_string().to_lowercase() == "divide by zero" {
                    results.push("ND".to_string())
                }
            }
        }
    }

    results
}
fn main() {
    let input = vec![
        "7/3.78",
        "1+1",
        "iii435ii",
        "3+    7",
        "22 - 8",
        "3*7",
        "64    / 2",
        "2/0",
    ];

    println!("{:?}", p2(input));
}
