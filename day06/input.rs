use std::str::FromStr;

#[derive(Debug)]
pub enum Operation {
    Add,
    Multiply,
}
impl Operation {
    pub fn call(&self, a: usize, b: usize) -> usize {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}
impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => Err(format!("Unrecognized operation, '{s}'")),
        }
    }
}
impl TryFrom<char> for Operation {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Multiply),
            _ => Err(format!("Unrecognized operation, '{value}'")),
        }
    }
}

#[derive(Debug)]
pub struct Expression {
    pub operation: Operation,
    pub inputs: Vec<usize>,
}

pub type Input = Vec<Expression>;
