// SPDX-License-Identifier: MIT OR Apache-2.0
//! Abstract Syntax Tree for Betlang
//!
//! The AST represents the structure of betlang programs after parsing.
//! The core innovation is the ternary `bet` expression as the fundamental
//! probabilistic primitive.

use crate::span::{Span, Spanned};
use crate::symbol::Symbol;
use smallvec::SmallVec;

/// A complete betlang module/file
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Module {
    pub name: Option<Symbol>,
    pub items: Vec<Spanned<Item>>,
    pub span: Span,
}

/// Top-level items in a module
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    /// Function/value definition: `let name = expr` or `let name : type = expr`
    Let(LetDef),

    /// Type definition: `type Name = Type`
    TypeDef(TypeDef),

    /// Import: `import Module` or `import Module.{a, b, c}`
    Import(Import),

    /// Expression at top level (for scripts/REPL)
    Expr(Expr),
}

/// A let binding definition
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LetDef {
    pub name: Spanned<Symbol>,
    pub params: Vec<Spanned<Pattern>>,
    pub type_ann: Option<Spanned<Type>>,
    pub body: Spanned<Expr>,
    pub is_rec: bool,
}

/// A type definition
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeDef {
    pub name: Spanned<Symbol>,
    pub params: Vec<Spanned<Symbol>>,
    pub body: Spanned<Type>,
}

/// An import statement
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Import {
    pub path: Vec<Spanned<Symbol>>,
    pub items: Option<Vec<Spanned<Symbol>>>, // None = import all
}

/// Expressions - the heart of betlang
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expr {
    // === Literals ===

    /// Integer literal: `42`
    Int(i64),

    /// Float literal: `3.14`
    Float(f64),

    /// String literal: `"hello"`
    String(String),

    /// Boolean literal: `true` or `false`
    Bool(bool),

    /// Ternary logic literal: `true`, `false`, or `unknown`
    Ternary(TernaryValue),

    /// Unit value: `()`
    Unit,

    // === Core Ternary Primitive ===

    /// Uniform ternary bet: `bet { a, b, c }`
    Bet(BetExpr),

    /// Weighted bet: `bet { a @ w1, b @ w2, c @ w3 }`
    WeightedBet(WeightedBetExpr),

    /// Conditional bet: `bet_if cond { a } else { b, c, d }`
    ConditionalBet(ConditionalBetExpr),

    // === Variables and Application ===

    /// Variable reference: `x`
    Var(Symbol),

    /// Function application: `f x y`
    App(Box<Spanned<Expr>>, Vec<Spanned<Expr>>),

    /// Lambda: `fun x -> e` or `fun x y z -> e`
    Lambda(LambdaExpr),

    // === Binding Forms ===

    /// Let binding: `let x = e1 in e2`
    Let(LetExpr),

    /// Monadic do-notation: `do { x <- e1; e2 }`
    Do(DoExpr),

    // === Control Flow ===

    /// Conditional: `if c then t else e`
    If(IfExpr),

    /// Pattern matching: `match e { p1 -> e1, p2 -> e2, p3 -> e3 }`
    Match(MatchExpr),

    // === Data Structures ===

    /// Tuple: `(a, b, c)`
    Tuple(Vec<Spanned<Expr>>),

    /// List: `[a, b, c]`
    List(Vec<Spanned<Expr>>),

    /// Record: `{ x = 1, y = 2 }`
    Record(Vec<(Spanned<Symbol>, Spanned<Expr>)>),

    /// Field access: `e.field`
    Field(Box<Spanned<Expr>>, Spanned<Symbol>),

    /// Index access: `e[i]`
    Index(Box<Spanned<Expr>>, Box<Spanned<Expr>>),

    // === Operators ===

    /// Binary operator: `a + b`
    BinOp(BinOp, Box<Spanned<Expr>>, Box<Spanned<Expr>>),

    /// Unary operator: `-x` or `not x`
    UnOp(UnOp, Box<Spanned<Expr>>),

    // === Probabilistic Operations ===

    /// Sample from distribution: `sample dist`
    Sample(Box<Spanned<Expr>>),

    /// Observe/condition: `observe dist value`
    Observe(Box<Spanned<Expr>>, Box<Spanned<Expr>>),

    /// Inference: `infer method { params } model`
    Infer(InferExpr),

    /// Parallel sampling: `parallel n { expr }`
    Parallel(Box<Spanned<Expr>>, Box<Spanned<Expr>>),

    // === Type Annotations ===

    /// Type annotation: `e : T`
    Annotate(Box<Spanned<Expr>>, Spanned<Type>),

    // === Special ===

    /// Hole/placeholder: `_` or `?name`
    Hole(Option<Symbol>),

    /// Error recovery node
    Error,
}

/// The core ternary bet expression
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BetExpr {
    /// Exactly three alternatives (ternary choice)
    pub alternatives: [Box<Spanned<Expr>>; 3],
}

/// Weighted bet expression
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WeightedBetExpr {
    /// Alternatives with weights: (expr, weight)
    pub alternatives: [(Box<Spanned<Expr>>, Box<Spanned<Expr>>); 3],
}

/// Conditional bet expression
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConditionalBetExpr {
    /// Condition to check
    pub condition: Box<Spanned<Expr>>,
    /// Value if true
    pub if_true: Box<Spanned<Expr>>,
    /// Three alternatives if false (bet among these)
    pub if_false: [Box<Spanned<Expr>>; 3],
}

/// Lambda expression
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaExpr {
    pub params: Vec<Spanned<Pattern>>,
    pub body: Box<Spanned<Expr>>,
}

/// Let expression
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LetExpr {
    pub pattern: Spanned<Pattern>,
    pub type_ann: Option<Spanned<Type>>,
    pub value: Box<Spanned<Expr>>,
    pub body: Box<Spanned<Expr>>,
    pub is_rec: bool,
}

/// Do-notation (monadic sequencing)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DoExpr {
    pub statements: Vec<Spanned<DoStatement>>,
}

/// A statement in do-notation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DoStatement {
    /// Bind: `x <- e`
    Bind(Spanned<Pattern>, Spanned<Expr>),
    /// Expression (no binding): `e`
    Expr(Spanned<Expr>),
    /// Let in do-block: `let x = e`
    Let(Spanned<Pattern>, Spanned<Expr>),
}

/// If expression
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfExpr {
    pub condition: Box<Spanned<Expr>>,
    pub then_branch: Box<Spanned<Expr>>,
    pub else_branch: Box<Spanned<Expr>>,
}

/// Match expression (ternary: exactly 3 arms encouraged)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatchExpr {
    pub scrutinee: Box<Spanned<Expr>>,
    pub arms: Vec<MatchArm>,
}

/// A single match arm
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatchArm {
    pub pattern: Spanned<Pattern>,
    pub guard: Option<Spanned<Expr>>,
    pub body: Spanned<Expr>,
}

/// Inference expression
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InferExpr {
    pub method: InferMethod,
    pub params: Vec<(Spanned<Symbol>, Spanned<Expr>)>,
    pub model: Box<Spanned<Expr>>,
}

/// Inference methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InferMethod {
    /// Markov Chain Monte Carlo
    MCMC,
    /// Hamiltonian Monte Carlo
    HMC,
    /// Sequential Monte Carlo
    SMC,
    /// Variational Inference
    VI,
    /// Rejection sampling
    Rejection,
    /// Importance sampling
    Importance,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BinOp {
    // Arithmetic
    Add, Sub, Mul, Div, Mod, Pow,
    // Comparison
    Eq, Ne, Lt, Le, Gt, Ge,
    // Logical (ternary-aware)
    And, Or, Xor,
    // String
    Concat,
    // List
    Cons, Append,
    // Probabilistic
    Compose, // f >> g (Kleisli composition)
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnOp {
    Neg,    // -x
    Not,    // not x (ternary negation)
    Sample, // sample shorthand
}

/// Ternary logic values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TernaryValue {
    True,
    False,
    Unknown,
}

/// Patterns for matching and binding
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Pattern {
    /// Wildcard: `_`
    Wildcard,
    /// Variable binding: `x`
    Var(Symbol),
    /// Literal pattern: `42`, `"hello"`, `true`
    Literal(Literal),
    /// Tuple pattern: `(a, b, c)`
    Tuple(Vec<Spanned<Pattern>>),
    /// List pattern: `[a, b, c]` or `[h | t]`
    List(Vec<Spanned<Pattern>>, Option<Box<Spanned<Pattern>>>),
    /// Constructor pattern: `Some x`
    Constructor(Symbol, Vec<Spanned<Pattern>>),
    /// Record pattern: `{ x, y }`
    Record(Vec<(Spanned<Symbol>, Option<Spanned<Pattern>>)>),
    /// As-pattern: `p as x`
    As(Box<Spanned<Pattern>>, Symbol),
    /// Or-pattern: `p1 | p2 | p3` (ternary!)
    Or(Box<Spanned<Pattern>>, Box<Spanned<Pattern>>, Box<Spanned<Pattern>>),
    /// Type-annotated pattern
    Annotate(Box<Spanned<Pattern>>, Spanned<Type>),
}

/// Literal values (subset of expressions)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Ternary(TernaryValue),
    Unit,
}

/// Types
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// Named type: `Int`, `String`, `MyType`
    Named(Symbol),

    /// Type variable: `'a`
    Var(Symbol),

    /// Type application: `List Int`, `Dist Float`
    App(Box<Spanned<Type>>, Vec<Spanned<Type>>),

    /// Function type: `a -> b`
    Arrow(Box<Spanned<Type>>, Box<Spanned<Type>>),

    /// Tuple type: `(a, b, c)`
    Tuple(Vec<Spanned<Type>>),

    /// Record type: `{ x : Int, y : Float }`
    Record(Vec<(Symbol, Spanned<Type>)>),

    /// Distribution type: `Dist a` (first-class!)
    Dist(Box<Spanned<Type>>),

    /// Probability-indexed type: `Prob p a` (dependent)
    Prob(Box<Spanned<Expr>>, Box<Spanned<Type>>),

    /// Ternary type: `Ternary`
    Ternary,

    /// Inference hole: `_`
    Hole,

    /// Error recovery
    Error,
}

/// Built-in types
pub mod builtin_types {
    use super::*;

    pub fn int() -> Type { Type::Named(Symbol::intern("Int")) }
    pub fn float() -> Type { Type::Named(Symbol::intern("Float")) }
    pub fn bool() -> Type { Type::Named(Symbol::intern("Bool")) }
    pub fn string() -> Type { Type::Named(Symbol::intern("String")) }
    pub fn unit() -> Type { Type::Named(Symbol::intern("Unit")) }
    pub fn ternary() -> Type { Type::Ternary }

    pub fn dist(inner: Type) -> Type {
        Type::Dist(Box::new(Spanned::dummy(inner)))
    }

    pub fn list(inner: Type) -> Type {
        Type::App(
            Box::new(Spanned::dummy(Type::Named(Symbol::intern("List")))),
            vec![Spanned::dummy(inner)],
        )
    }
}

impl Expr {
    /// Check if this is a value (no further evaluation needed)
    pub fn is_value(&self) -> bool {
        matches!(
            self,
            Expr::Int(_)
                | Expr::Float(_)
                | Expr::String(_)
                | Expr::Bool(_)
                | Expr::Ternary(_)
                | Expr::Unit
                | Expr::Lambda(_)
        )
    }

    /// Check if this expression involves randomness
    pub fn is_probabilistic(&self) -> bool {
        match self {
            Expr::Bet(_) | Expr::WeightedBet(_) | Expr::ConditionalBet(_) => true,
            Expr::Sample(_) | Expr::Infer(_) | Expr::Parallel(_, _) => true,
            Expr::App(f, args) => {
                f.node.is_probabilistic() || args.iter().any(|a| a.node.is_probabilistic())
            }
            Expr::Let(LetExpr { value, body, .. }) => {
                value.node.is_probabilistic() || body.node.is_probabilistic()
            }
            Expr::Do(DoExpr { statements }) => {
                statements.iter().any(|s| match &s.node {
                    DoStatement::Bind(_, e) | DoStatement::Expr(e) | DoStatement::Let(_, e) => {
                        e.node.is_probabilistic()
                    }
                })
            }
            Expr::If(IfExpr { condition, then_branch, else_branch }) => {
                condition.node.is_probabilistic()
                    || then_branch.node.is_probabilistic()
                    || else_branch.node.is_probabilistic()
            }
            _ => false,
        }
    }
}

impl TernaryValue {
    /// Convert to numeric representation (Kleene logic)
    pub fn to_f64(self) -> f64 {
        match self {
            TernaryValue::False => 0.0,
            TernaryValue::Unknown => 0.5,
            TernaryValue::True => 1.0,
        }
    }

    /// Ternary AND (minimum)
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (TernaryValue::False, _) | (_, TernaryValue::False) => TernaryValue::False,
            (TernaryValue::True, x) | (x, TernaryValue::True) => x,
            _ => TernaryValue::Unknown,
        }
    }

    /// Ternary OR (maximum)
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (TernaryValue::True, _) | (_, TernaryValue::True) => TernaryValue::True,
            (TernaryValue::False, x) | (x, TernaryValue::False) => x,
            _ => TernaryValue::Unknown,
        }
    }

    /// Ternary NOT
    pub fn not(self) -> Self {
        match self {
            TernaryValue::True => TernaryValue::False,
            TernaryValue::False => TernaryValue::True,
            TernaryValue::Unknown => TernaryValue::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_logic() {
        use TernaryValue::*;

        // AND truth table
        assert_eq!(True.and(True), True);
        assert_eq!(True.and(Unknown), Unknown);
        assert_eq!(True.and(False), False);
        assert_eq!(Unknown.and(Unknown), Unknown);
        assert_eq!(False.and(Unknown), False);

        // OR truth table
        assert_eq!(False.or(False), False);
        assert_eq!(False.or(Unknown), Unknown);
        assert_eq!(False.or(True), True);

        // NOT
        assert_eq!(True.not(), False);
        assert_eq!(False.not(), True);
        assert_eq!(Unknown.not(), Unknown);
    }
}
