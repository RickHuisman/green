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

    // TODO Find a better way to convert Value::Obj(Obj::String()) to String
    pub fn as_string(self) -> String {
        match self {
            Value::Obj(s) => match s {
                Object::String(s) => s,
                _ => panic!("TODO"),
            },
            _ => panic!("TODO"),
        }
    }

    pub fn as_function(self) -> GreenFunction {
        // FIXME
        match self {
            Value::Obj(obj) => match obj {
                Object::Function(fun) => fun,
                _ => panic!("TODO"), // TODO
            },
            _ => panic!("TODO"), // TODO
        }
    }

    pub fn as_closure(self) -> GreenClosure {
        // FIXME
        match self {
            Value::Obj(obj) => match obj {
                Object::Closure(c) => c,
                _ => panic!("Can only call functions"), // TODO
            },
            _ => panic!("Can only call functions"), // TODO
        }
    }

    pub fn as_number(self) -> f64 {
        match self {
            Value::Number(n) => n,
            _ => panic!("TODO"), // TODO
        }
    }

    pub fn as_array(self) -> Vec<Value> {
        // FIXME
        match self {
            Value::Obj(object) => {
                match object {
                    Object::Array(a) => a,
                    _ => panic!("TODO"), // TODO
                }
            },
            _ => panic!("TODO"), // TODO
        }
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