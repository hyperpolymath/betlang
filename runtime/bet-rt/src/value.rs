// SPDX-License-Identifier: MIT OR Apache-2.0
//! Runtime values for Betlang

use im::{HashMap, Vector};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::sync::Arc;

/// Serde helper modules for Arc<T> types
mod arc_serde {
    use super::*;

    pub mod arc_string {
        use super::*;

        pub fn serialize<S>(arc: &Arc<String>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            arc.as_ref().serialize(serializer)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Arc<String>, D::Error>
        where
            D: Deserializer<'de>,
        {
            String::deserialize(deserializer).map(Arc::new)
        }
    }

    pub mod arc_vec_u8 {
        use super::*;

        pub fn serialize<S>(arc: &Arc<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            arc.as_ref().serialize(serializer)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Arc<Vec<u8>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            Vec::<u8>::deserialize(deserializer).map(Arc::new)
        }
    }

    pub mod arc_vec_value {
        use super::*;

        pub fn serialize<S>(arc: &Arc<Vec<super::Value>>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            arc.as_ref().serialize(serializer)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Arc<Vec<super::Value>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            Vec::<super::Value>::deserialize(deserializer).map(Arc::new)
        }
    }
}

/// Ternary logic value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ternary {
    True,
    False,
    Unknown,
}

impl Ternary {
    /// Convert to numeric (Kleene logic): 0.0, 0.5, 1.0
    pub fn to_f64(self) -> f64 {
        match self {
            Ternary::False => 0.0,
            Ternary::Unknown => 0.5,
            Ternary::True => 1.0,
        }
    }

    /// Ternary AND (minimum)
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Ternary::False, _) | (_, Ternary::False) => Ternary::False,
            (Ternary::True, x) | (x, Ternary::True) => x,
            _ => Ternary::Unknown,
        }
    }

    /// Ternary OR (maximum)
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Ternary::True, _) | (_, Ternary::True) => Ternary::True,
            (Ternary::False, x) | (x, Ternary::False) => x,
            _ => Ternary::Unknown,
        }
    }

    /// Ternary NOT
    pub fn not(self) -> Self {
        match self {
            Ternary::True => Ternary::False,
            Ternary::False => Ternary::True,
            Ternary::Unknown => Ternary::Unknown,
        }
    }

    /// Ternary XOR
    pub fn xor(self, other: Self) -> Self {
        match (self, other) {
            (Ternary::Unknown, _) | (_, Ternary::Unknown) => Ternary::Unknown,
            (Ternary::True, Ternary::False) | (Ternary::False, Ternary::True) => Ternary::True,
            _ => Ternary::False,
        }
    }

    /// Majority of three values
    pub fn majority(a: Self, b: Self, c: Self) -> Self {
        let sum = a.to_f64() + b.to_f64() + c.to_f64();
        if sum >= 2.0 {
            Ternary::True
        } else if sum <= 1.0 {
            Ternary::False
        } else {
            Ternary::Unknown
        }
    }
}

impl fmt::Display for Ternary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ternary::True => write!(f, "true"),
            Ternary::False => write!(f, "false"),
            Ternary::Unknown => write!(f, "unknown"),
        }
    }
}

/// Runtime value representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    /// Unit value
    Unit,

    /// Boolean (binary)
    Bool(bool),

    /// Ternary logic value
    Ternary(Ternary),

    /// 64-bit signed integer
    Int(i64),

    /// 64-bit floating point
    Float(f64),

    /// UTF-8 string
    #[serde(with = "arc_serde::arc_string")]
    String(Arc<String>),

    /// Byte array
    #[serde(with = "arc_serde::arc_vec_u8")]
    Bytes(Arc<Vec<u8>>),

    /// Immutable list/array
    List(Vector<Value>),

    /// Immutable map
    Map(HashMap<String, Value>),

    /// Immutable set (represented as map to unit)
    Set(HashMap<Value, ()>),

    /// Tuple (fixed-size, heterogeneous)
    #[serde(with = "arc_serde::arc_vec_value")]
    Tuple(Arc<Vec<Value>>),

    /// Distribution (lazy probabilistic value)
    #[serde(skip)]
    Dist(Arc<Distribution>),

    /// Function closure
    #[serde(skip)]
    Closure(Arc<Closure>),

    /// Native function
    #[serde(skip)]
    Native(NativeFunction),

    /// File handle
    #[serde(skip)]
    File(Arc<FileHandle>),

    /// Error value
    #[serde(with = "arc_serde::arc_string")]
    Error(Arc<String>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Unit, Value::Unit) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Ternary(a), Value::Ternary(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bytes(a), Value::Bytes(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Map(a), Value::Map(b)) => a == b,
            (Value::Set(a), Value::Set(b)) => a == b,
            (Value::Tuple(a), Value::Tuple(b)) => a == b,
            (Value::Error(a), Value::Error(b)) => a == b,
            // Non-comparable types (contain function pointers)
            (Value::Dist(_), Value::Dist(_)) => false,
            (Value::Closure(_), Value::Closure(_)) => false,
            (Value::Native(a), Value::Native(b)) => a == b,
            (Value::File(_), Value::File(_)) => false,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Value::Unit => {}
            Value::Bool(b) => b.hash(state),
            Value::Ternary(t) => t.hash(state),
            Value::Int(i) => i.hash(state),
            Value::Float(f) => f.to_bits().hash(state),
            Value::String(s) => s.hash(state),
            Value::Bytes(b) => b.hash(state),
            Value::List(l) => {
                for v in l.iter() {
                    v.hash(state);
                }
            }
            Value::Tuple(t) => {
                for v in t.iter() {
                    v.hash(state);
                }
            }
            _ => {} // Non-hashable types
        }
    }
}

/// A probability distribution (lazy)
pub struct Distribution {
    pub sampler: Box<dyn Fn() -> Value + Send + Sync>,
    pub name: String,
}

impl fmt::Debug for Distribution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dist({})", self.name)
    }
}

/// A function closure
pub struct Closure {
    pub params: Vec<String>,
    pub body: Box<dyn Fn(Vec<Value>) -> Value + Send + Sync>,
    pub name: Option<String>,
}

impl fmt::Debug for Closure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Closure({:?})", self.name)
    }
}

/// Native function pointer
#[derive(Clone)]
pub struct NativeFunction {
    pub name: &'static str,
    pub arity: usize,
    pub func: fn(Vec<Value>) -> Result<Value, String>,
}

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Native({})", self.name)
    }
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

/// File handle wrapper
pub struct FileHandle {
    pub path: String,
    pub mode: FileMode,
    // Actual handle managed by tokio
}

impl fmt::Debug for FileHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File({}, {:?})", self.path, self.mode)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileMode {
    Read,
    Write,
    Append,
    ReadWrite,
}

impl Value {
    /// Check if value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Unit => false,
            Value::Bool(b) => *b,
            Value::Ternary(t) => *t == Ternary::True,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Bytes(b) => !b.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Map(m) => !m.is_empty(),
            Value::Set(s) => !s.is_empty(),
            Value::Tuple(t) => !t.is_empty(),
            Value::Error(_) => false,
            _ => true,
        }
    }

    /// Get type name
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Unit => "Unit",
            Value::Bool(_) => "Bool",
            Value::Ternary(_) => "Ternary",
            Value::Int(_) => "Int",
            Value::Float(_) => "Float",
            Value::String(_) => "String",
            Value::Bytes(_) => "Bytes",
            Value::List(_) => "List",
            Value::Map(_) => "Map",
            Value::Set(_) => "Set",
            Value::Tuple(_) => "Tuple",
            Value::Dist(_) => "Dist",
            Value::Closure(_) => "Function",
            Value::Native(_) => "Native",
            Value::File(_) => "File",
            Value::Error(_) => "Error",
        }
    }

    /// Create a uniform ternary bet distribution
    pub fn bet(a: Value, b: Value, c: Value) -> Value {
        use rand::Rng;
        let choices = Arc::new([a, b, c]);
        Value::Dist(Arc::new(Distribution {
            sampler: Box::new(move || {
                let idx = rand::thread_rng().gen_range(0..3);
                choices[idx].clone()
            }),
            name: "bet".to_string(),
        }))
    }

    /// Create a weighted ternary bet distribution
    pub fn weighted_bet(a: Value, wa: f64, b: Value, wb: f64, c: Value, wc: f64) -> Value {
        use rand::Rng;
        let total = wa + wb + wc;
        let choices = Arc::new([(a, wa / total), (b, wb / total), (c, wc / total)]);
        Value::Dist(Arc::new(Distribution {
            sampler: Box::new(move || {
                let r: f64 = rand::thread_rng().gen();
                if r < choices[0].1 {
                    choices[0].0.clone()
                } else if r < choices[0].1 + choices[1].1 {
                    choices[1].0.clone()
                } else {
                    choices[2].0.clone()
                }
            }),
            name: "weighted_bet".to_string(),
        }))
    }

    /// Sample from a distribution
    pub fn sample(&self) -> Result<Value, String> {
        match self {
            Value::Dist(d) => Ok((d.sampler)()),
            _ => Err(format!("Cannot sample from {}", self.type_name())),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Unit => write!(f, "()"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Ternary(t) => write!(f, "{}", t),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Bytes(b) => write!(f, "<bytes len={}>", b.len()),
            Value::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Map(m) => {
                write!(f, "{{")?;
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Set(s) => {
                write!(f, "Set{{")?;
                for (i, (k, _)) in s.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", k)?;
                }
                write!(f, "}}")
            }
            Value::Tuple(t) => {
                write!(f, "(")?;
                for (i, v) in t.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, ")")
            }
            Value::Dist(d) => write!(f, "<{}>", d.name),
            Value::Closure(c) => write!(f, "<fn {:?}>", c.name),
            Value::Native(n) => write!(f, "<native {}>", n.name),
            Value::File(h) => write!(f, "<file {}>", h.path),
            Value::Error(e) => write!(f, "Error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_logic() {
        assert_eq!(Ternary::True.and(Ternary::True), Ternary::True);
        assert_eq!(Ternary::True.and(Ternary::Unknown), Ternary::Unknown);
        assert_eq!(Ternary::True.and(Ternary::False), Ternary::False);
        assert_eq!(Ternary::Unknown.not(), Ternary::Unknown);
    }

    #[test]
    fn test_bet_sampling() {
        let dist = Value::bet(Value::Int(1), Value::Int(2), Value::Int(3));
        for _ in 0..100 {
            let sample = dist.sample().unwrap();
            match sample {
                Value::Int(n) => assert!(n >= 1 && n <= 3),
                _ => panic!("Expected Int"),
            }
        }
    }

    #[test]
    fn test_majority() {
        use Ternary::*;
        assert_eq!(Ternary::majority(True, True, False), True);
        assert_eq!(Ternary::majority(False, False, True), False);
        assert_eq!(Ternary::majority(True, Unknown, False), Unknown);
    }
}
