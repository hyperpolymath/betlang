// SPDX-License-Identifier: MIT OR Apache-2.0
//! Environment for variable bindings

use std::collections::HashMap;
use crate::types::Type;

/// Type environment for type checking
#[derive(Debug, Clone, Default)]
pub struct TypeEnv {
    bindings: HashMap<String, Type>,
    parent: Option<Box<TypeEnv>>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_parent(parent: TypeEnv) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn bind(&mut self, name: String, ty: Type) {
        self.bindings.insert(name, ty);
    }

    pub fn lookup(&self, name: &str) -> Option<&Type> {
        self.bindings
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.lookup(name)))
    }

    pub fn extend(&self) -> Self {
        Self::with_parent(self.clone())
    }
}

/// Value environment for evaluation
#[derive(Debug, Clone)]
pub struct ValueEnv<V> {
    bindings: HashMap<String, V>,
    parent: Option<Box<ValueEnv<V>>>,
}

impl<V: Clone> Default for ValueEnv<V> {
    fn default() -> Self {
        Self {
            bindings: HashMap::new(),
            parent: None,
        }
    }
}

impl<V: Clone> ValueEnv<V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind(&mut self, name: String, value: V) {
        self.bindings.insert(name, value);
    }

    pub fn lookup(&self, name: &str) -> Option<V> {
        self.bindings
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref().and_then(|p| p.lookup(name)))
    }

    pub fn extend(&self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(Box::new(self.clone())),
        }
    }
}
