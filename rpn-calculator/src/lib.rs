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
            CalculatorInput::Value(x) => stack.push(*x),
            x => {
                if stack.len() < 2 {
                    return None;
                }
                let n = stack.pop().unwrap();
                let m = stack.pop().unwrap();
                match x {
                    CalculatorInput::Add => stack.push(n + m),
                    CalculatorInput::Subtract => stack.push(m - n),
                    CalculatorInput::Multiply => stack.push(n * m),
                    CalculatorInput::Divide => stack.push(m / n),
                    _ => return None,
                }
            }
        }
    }
    if stack.len() != 1 {
        return None;
    }

    stack.pop()
}
