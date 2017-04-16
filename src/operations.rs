use value::*;
use ast::{BinaryOp, UnaryOp};
use runtime::RuntimeError;

pub fn unary_minus(a: Value) -> Result<Value, RuntimeError> {
    match a {
        Value::Number(x) => Ok(Value::Number(-x)),
        x => Err(RuntimeError::UnaryTypeError(UnaryOp::Minus, x.get_type())),
    }
}

pub fn add(a: Value, b: Value) -> Result<Value, RuntimeError> {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
        (Value::Tuple(mut a), Value::Tuple(mut b)) => {
            a.append(&mut b);
            Ok(Value::Tuple(a))
        }
        (Value::String(sa), Value::String(sb)) => Ok(Value::String(sa + &sb)),
        (Value::String(s), other) => Ok(Value::String(s + &other.to_string())),
        (other, Value::String(s)) => Ok(Value::String(other.to_string() + &s)),
        (a, b) => Err(RuntimeError::BinaryTypeError(BinaryOp::Add, a.get_type(), b.get_type())),
    }
}

pub fn subtract(a: Value, b: Value) -> Result<Value, RuntimeError> {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
        (a, b) => Err(RuntimeError::BinaryTypeError(BinaryOp::Sub, a.get_type(), b.get_type())),
    }
}

pub fn multiply(a: Value, b: Value) -> Result<Value, RuntimeError> {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
        (a, b) => Err(RuntimeError::BinaryTypeError(BinaryOp::Mul, a.get_type(), b.get_type())),
    }
}

pub fn divide(a: Value, b: Value) -> Result<Value, RuntimeError> {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
        (a, b) => Err(RuntimeError::BinaryTypeError(BinaryOp::Div, a.get_type(), b.get_type())),
    }
}

pub fn floor_divide(a: Value, b: Value) -> Result<Value, RuntimeError> {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.floor_div(&b))),
        (a, b) => {
            Err(RuntimeError::BinaryTypeError(BinaryOp::FloorDiv, a.get_type(), b.get_type()))
        }
    }
}

pub fn less_than(a: Value, b: Value) -> Result<Value, RuntimeError> {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
        (a, b) => {
            Err(RuntimeError::BinaryTypeError(BinaryOp::LessThan, a.get_type(), b.get_type()))
        }
    }
}

pub fn less_than_or_equal(a: Value, b: Value) -> Result<Value, RuntimeError> {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
        (a, b) => {
            Err(RuntimeError::BinaryTypeError(BinaryOp::LessThanOrEqual,
                                                  a.get_type(),
                                                  b.get_type()))
        }
    }
}
pub fn greater_than(a: Value, b: Value) -> Result<Value, RuntimeError> {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
        (a, b) => {
            Err(RuntimeError::BinaryTypeError(BinaryOp::GreaterThan,
                                                  a.get_type(),
                                                  b.get_type()))
        }
    }
}

pub fn greater_than_or_equal(a: Value, b: Value) -> Result<Value, RuntimeError> {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
        (a, b) => {
            Err(RuntimeError::BinaryTypeError(BinaryOp::GreaterThanOrEqual,
                                                  a.get_type(),
                                                  b.get_type()))
        }
    }
}
