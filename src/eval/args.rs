use {
    crate::{Env, Result, Value},
    std::{ops, vec},
};

#[derive(Debug)]
pub struct Args<'e> {
    pub env: &'e mut Env,
    args: Vec<Value>,
}

impl<'e> ops::Deref for Args<'e> {
    type Target = Vec<Value>;
    fn deref(&self) -> &Self::Target {
        &self.args
    }
}

impl<'e> ops::DerefMut for Args<'e> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.args
    }
}

impl<'e> IntoIterator for Args<'e> {
    type Item = Value;
    type IntoIter = vec::IntoIter<Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.args.into_iter()
    }
}

impl<'e> Args<'e> {
    pub fn new(env: &'e mut Env, args: Vec<Value>) -> Args<'e> {
        Args { env, args }
    }

    pub fn get(&self, idx: usize) -> Option<Value> {
        self.args.get(idx).map(Value::clone)
    }

    /// Like `get()` but returns an error.
    pub fn need(&self, idx: usize) -> Result<Value> {
        self.args.get(idx).map(Value::clone).ok_or(error_kind!(
            ArgNotFound,
            "Argument Not Found: {}",
            idx
        ))
    }

    /// Errors if the arg doesn't exist or isn't a Number.
    pub fn need_number(&self, idx: usize) -> Result<f64> {
        if let Value::Number(num) = self.need(idx)? {
            Ok(num)
        } else {
            Err(error_kind!(
                WrongArgType,
                "Expected Number, got: {:?}",
                self
            ))
        }
    }

    /// Errors if the arg doesn't exist or isn't a String.
    pub fn need_string(&self, idx: usize) -> Result<&str> {
        if let Some(Value::String(s)) = self.args.get(idx) {
            Ok(s)
        } else {
            Err(error_kind!(
                WrongArgType,
                "Expected String, got: {:?}",
                self
            ))
        }
    }
}
