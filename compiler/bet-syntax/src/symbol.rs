// SPDX-License-Identifier: MIT OR Apache-2.0
//! Interned symbols for efficient identifier handling

use std::fmt;
use std::sync::OnceLock;
use indexmap::IndexSet;
use std::sync::RwLock;

/// An interned string symbol
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Symbol(u32);

/// Global symbol interner
static INTERNER: OnceLock<RwLock<Interner>> = OnceLock::new();

fn interner() -> &'static RwLock<Interner> {
    INTERNER.get_or_init(|| RwLock::new(Interner::new()))
}

#[derive(Default)]
struct Interner {
    symbols: IndexSet<String>,
}

impl Interner {
    fn new() -> Self {
        let mut interner = Self::default();
        // Pre-intern common symbols
        interner.intern("bet");
        interner.intern("let");
        interner.intern("in");
        interner.intern("fun");
        interner.intern("match");
        interner.intern("if");
        interner.intern("then");
        interner.intern("else");
        interner.intern("do");
        interner.intern("return");
        interner.intern("true");
        interner.intern("false");
        interner.intern("unknown");
        interner
    }

    fn intern(&mut self, s: &str) -> Symbol {
        if let Some(idx) = self.symbols.get_index_of(s) {
            Symbol(idx as u32)
        } else {
            let idx = self.symbols.len();
            self.symbols.insert(s.to_owned());
            Symbol(idx as u32)
        }
    }

    fn resolve(&self, sym: Symbol) -> &str {
        self.symbols.get_index(sym.0 as usize).map(|s| s.as_str()).unwrap_or("<invalid>")
    }
}

impl Symbol {
    /// Intern a string, returning a symbol
    pub fn intern(s: &str) -> Self {
        interner().write().unwrap().intern(s)
    }

    /// Get the string representation of a symbol
    pub fn as_str(&self) -> String {
        interner().read().unwrap().resolve(*self).to_owned()
    }

    /// Get the numeric index of this symbol
    pub fn index(&self) -> u32 {
        self.0
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Symbol({:?})", self.as_str())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for Symbol {
    fn from(s: &str) -> Self {
        Self::intern(s)
    }
}

impl From<String> for Symbol {
    fn from(s: String) -> Self {
        Self::intern(&s)
    }
}

/// Well-known symbols for fast comparison
pub mod kw {
    use super::Symbol;

    macro_rules! define_symbols {
        ($($name:ident => $str:expr),* $(,)?) => {
            $(
                pub fn $name() -> Symbol {
                    Symbol::intern($str)
                }
            )*
        };
    }

    define_symbols! {
        bet => "bet",
        let_ => "let",
        in_ => "in",
        fun => "fun",
        match_ => "match",
        if_ => "if",
        then => "then",
        else_ => "else",
        do_ => "do",
        return_ => "return",
        true_ => "true",
        false_ => "false",
        unknown => "unknown",
        type_ => "type",
        module => "module",
        import => "import",
        export => "export",
        infer => "infer",
        sample => "sample",
        observe => "observe",
        parallel => "parallel",
        weighted => "weighted",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interning() {
        let s1 = Symbol::intern("hello");
        let s2 = Symbol::intern("hello");
        let s3 = Symbol::intern("world");

        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
        assert_eq!(s1.as_str(), "hello");
    }
}
