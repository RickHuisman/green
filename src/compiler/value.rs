use crate::compiler::object::{GreenClosure, GreenFunction, Instance, Class};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::vm::obj::Gc;
use crate::vm::vm::RunResult;
use crate::vm::errors::RuntimeError;

#[derive(Clone)] // TODO Implement Copy
pub enum Value {
    Number(f64),
    True,
    False,
    Nil, // TODO Does Green lang use nils???
    String(String),
    Array(Vec<Value>), // TODO u32? Vec?
    Closure(Gc<GreenClosure>),
    Function(Gc<GreenFunction>),
    Class(Gc<Class>),
    Instance(Gc<Instance>),
}

impl Value {
    pub fn string(s: String) -> Value {
        Value::String(s)
    }

    // TODO Find a better way to convert Value::Obj(Obj::String()) to String
    // TODO Use Result<T>
    pub fn as_string(&self) -> &String {
        match self {
            Value::String(s) => s,
            _ => panic!("TODO"),
        }
    }

    // pub fn as_function(self) -> GreenFunction {
    //     // FIXME
    //     match self {
    //         Value::Function(fun) => fun,
    //         _ => panic!("TODO"), // TODO
    //     }
    // }
    //
    pub fn as_instance(self) -> RunResult<Gc<Instance>> {
        match self {
            Value::Instance(i) => Ok(i),
            _ => Err(RuntimeError::ArgumentTypes),
        }
    }
    //
    // pub fn as_closure(self) -> GreenClosure {
    //     // FIXME
    //     match self {
    //         Value::Obj(obj) => match obj {
    //             Object::Closure(c) => c,
    //             _ => panic!("Can only call functions"), // TODO
    //         },
    //         _ => panic!("Can only call functions"), // TODO
    //     }
    // }

    pub fn as_number(self) -> f64 {
        match self {
            Value::Number(n) => n,
            _ => panic!("TODO"), // TODO
        }
    }

    pub fn as_array(self) -> Vec<Value> {
        // FIXME
        match self {
            Value::Array(a) => a,
            _ => panic!("TODO"), // TODO
        }
    }

    pub fn is_instance(&self) -> bool {
        match self {
            Value::Instance(_) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({})", n),
            Value::True => write!(f, "True"),
            Value::False => write!(f, "False"),
            Value::Nil => write!(f, "Nil"),
            Value::String(s) => write!(f, "String({})", s),
            Value::Array(_) => todo!(),
            Value::Closure(clos) => write!(f, "Closure({:?})", clos),
            Value::Function(fun) => write!(f, "Function({})", **fun),
            Value::Class(c) => write!(f, "Class({})", **c),
            Value::Instance(i) => write!(f, "Instance({:?})", i),
        }
    }
}

impl From<&Value> for bool {
    fn from(value: &Value) -> Self {
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
