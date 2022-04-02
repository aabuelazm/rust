#[derive(Debug, Copy, Clone)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut outputs: Vec<i32> = Vec::new();

    for &input in inputs {
        match input {
            CalculatorInput::Add => {
                if let Some(y) = outputs.pop() {
                    if let Some(x) = outputs.pop() {
                        outputs.push(x + y);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } // add arm

            CalculatorInput::Subtract => {
                if let Some(y) = outputs.pop() {
                    if let Some(x) = outputs.pop() {
                        outputs.push(x - y);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } // subtract arm

            CalculatorInput::Multiply => {
                if let Some(y) = outputs.pop() {
                    if let Some(x) = outputs.pop() {
                        outputs.push(x * y);
                    }
                }
            } // multiply arm

            CalculatorInput::Divide => {
                if let Some(y) = outputs.pop() {
                    if let Some(x) = outputs.pop() {
                        outputs.push(x / y);
                    }
                }
            } // divide arm

            CalculatorInput::Value(number) => outputs.push(number), // value arm
        } // match
    } // for loop

    match outputs {
        mut result if result.len() == 1 => result.pop(),
        _ => None,
    }
}
