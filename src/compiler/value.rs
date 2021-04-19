use crate::compiler::object::{GreenClosure, GreenFunction, Object};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    True,
    False,
    Nil, // TODO Does Green lang use nils???
    Obj(Object),
}

impl Value {
    pub fn string(s: String) -> Value {
        Value::Obj(Object::String(s))
    }

    pub fn closure(e: GreenClosure) -> Value {
        Value::Obj(Object::Closure(e))
    }

    pub fn function(f: GreenFunction) -> Value {
        Value::Obj(Object::Function(f))
    }
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
                Value::Number(b + a)
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
                Value::Number(b - a)
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
        match self {
            Value::Number(a) => Value::Number(-a),
            _ => todo!(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        if let Value::Number(b) = self {
            if let Value::Number(a) = other {
                b == a
            } else {
                panic!("Operand must be a number.");
            }
        } else {
            panic!("Operand must be a number.");
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Value::Number(b) = self {
            if let Value::Number(a) = other {
                b.partial_cmp(a)
            } else {
                panic!("Operand must be a number.");
            }
        } else {
            panic!("Operand must be a number.");
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::True => write!(f, "true"),
            Value::False => write!(f, "false"),
            Value::Nil => write!(f, "nil"),
            Value::Obj(obj) => write!(f, "{}", obj),
        }
    }
}

// TODO There must be a better way to convert Value::Obj(Obj::String()) to String
pub fn value_to_string(val: Value) -> String {
    match val {
        Value::Obj(s) => match s {
            Object::String(s) => s,
            _ => panic!("TODO"),
        },
        _ => panic!("TODO"),
    }
}
