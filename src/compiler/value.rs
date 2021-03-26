use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::Ordering;
use crate::compiler::object::Object;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    True,
    False,
    Nil, // TODO Does Eval lang use nils???
    Obj(Object)
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        match value {
            Value::False | Value::Nil => false,
            _ => true,
        }
    }
}

impl Into<Value> for bool {
    fn into(self) -> Value {
        if self {
            Value::True
        } else {
            Value::False
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if let Value::Number(b) = self {
            if let Value::Number(a) = other {
                Value::Number(a + b)
            } else {
                panic!("Operand must be a number.");
            }
        } else {
            panic!("Operand must be a number.");
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if let Value::Number(b) = self {
            if let Value::Number(a) = other {
                Value::Number(a - b)
            } else {
                panic!("Operand must be a number.");
            }
        } else {
            panic!("Operand must be a number.");
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if let Value::Number(b) = self {
            if let Value::Number(a) = other {
                Value::Number(b * a)
            } else {
                panic!("Operand must be a number.");
            }
        } else {
            panic!("Operand must be a number.");
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if let Value::Number(b) = self {
            if let Value::Number(a) = other {
                Value::Number(b / a)
            } else {
                panic!("Operand must be a number.");
            }
        } else {
            panic!("Operand must be a number.");
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if let Value::Number(a) = self {
            Value::Number(-a)
        } else {
            todo!() // TODO
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        if let Value::Number(b) = self {
            if let Value::Number(a) = other {
                a == b
            } else {
                panic!("Operand must be a number.");
            }
        } else {
            panic!("Operand must be a number.");
        }
        // TODO Eq on object
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Value::Number(b) = self {
            if let Value::Number(a) = other {
                a.partial_cmp(b)
            } else {
                panic!("Operand must be a number.");
            }
        } else {
            panic!("Operand must be a number.");
        }
    }
}

// TODO There must be a better way to convert Value::Obj(Obj::String()) to String
pub fn value_to_string(val: Value) -> String {
    match val {
        Value::Obj(s) => {
            match s {
                Object::String(s) => s,
                _ => panic!("TODO"),
            }
        }
        _ => panic!("TODO")
    }
}