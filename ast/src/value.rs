use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Eq, Hash)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Ord)]
pub enum Value {
    Int(i32),
    Str(String),
    Bool(bool),
    Bin(u32),
    Null,
}

impl Value {
    pub fn from_string(s: String) -> Self {
        if let Ok(i) = s.parse::<i32>() {
            return Value::Int(i);
        }
        Value::Str(s)
    }
}


impl Display for Value {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int (i) => write!(f, "{}", i),
            // Value::Float(fl) => write!(f, "{}", fl),
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Bin(b) => write!(f, "{:b}", b),
            Value::Null => write!(f, "null"),
        }
    }
}

pub fn compare_value(first: &Value, second: &Value) -> bool {
    match (first, second) {
        (Value::Int(i), Value::Int(j)) => i == j,
        //(Value::Float(f), Value::Float(g)) => f == g,
        (Value::Str(s), Value::Str(t)) => s == t,
        (Value::Bool(b), Value::Bool(c)) => b == c,
        _ => false,
    }
}