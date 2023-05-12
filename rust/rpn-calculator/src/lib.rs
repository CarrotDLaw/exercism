use std::vec;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];
    for input in inputs {
        match input {
            CalculatorInput::Value(number) => stack.push(*number),
            operator => {
                if stack.len() < 2 {
                    return None;
                }
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();
                let result = match operator {
                    CalculatorInput::Add => v1 + v2,
                    CalculatorInput::Subtract => v1 - v2,
                    CalculatorInput::Multiply => v1 * v2,
                    CalculatorInput::Divide => v1 / v2,
                    CalculatorInput::Value(_) => return None,
                };
                stack.push(result);
            }
        }
    }
    if stack.len() == 1 {
        return stack.pop();
    } else {
        return None;
    }
}
