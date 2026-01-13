//! JAMAL scope
//!
//! TODO

use crate::{JamalErr, JamalParser, Rule};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use std::{collections::HashMap, rc::Rc};

/// Primitive types of JAMAL
/// Sorted by priority low to high
#[derive(Debug, Clone, PartialEq)]
enum Primitive {
    Null,
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
}

impl Primitive {
    /// Get a number representing the priority of a primitive, bigger number == more priority
    fn get_priority(&self) -> i8 {
        match self {
            Primitive::Null => 0,
            Primitive::Bool(_) => 1,
            Primitive::Int(_) => 2,
            Primitive::Float(_) => 3,
            Primitive::String(_) => 4,
        }
    }

    /// Determine if other is of higher priority, true if equal
    fn higher_priority_than(&self, other: &Primitive) -> bool {
        return (self.get_priority() - other.get_priority()) >= 0;
    }
}

/// Operators of JAMAL
#[derive(PartialEq)]
enum Operator {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Pow, // ^
    Mod, // %
    And, // &&
    Or,  // ||
    Eq,  // ==
    Ne,  // !=
}

impl Operator {
    fn from_str(s: &str) -> Self {
        match s {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            "^" => Operator::Pow,
            "%" => Operator::Mod,
            "&&" => Operator::And,
            "||" => Operator::Or,
            "==" => Operator::Eq,
            "!=" => Operator::Ne,
            _ => unreachable!("Invalid operator for string conversion passed: {s}"),
        }
    }
}

/// A JAMAL Scope containing references to its parent and children
pub struct Scope {
    /// Single parent scope to inherit variables and such from
    parent: Option<Rc<Scope>>,
    /// Ordered list of children to be executed.
    children: Vec<Rc<Scope>>,
    /// Locally-scoped variables
    vars: HashMap<String, Primitive>,
}

impl Scope {
    /// Create a new, empty scope
    pub fn new() -> Self {
        return Self {
            parent: None,
            children: Vec::new(),
            vars: HashMap::new(),
        };
    }

    /// Create a new, empty scope and give it a parent
    pub fn new_with_parent(parent: Rc<Self>) -> Self {
        let mut scope = Self::new();
        scope.parent = Some(parent);
        return scope;
    }

    /// Run a parsed JAMAL file
    pub fn run(&mut self, parsed: Pairs<'_, Rule>) -> Result<(), JamalErr> {
        for pair in parsed {
            match pair.as_rule() {
                Rule::file => {
                    self.run(pair.into_inner())?;
                }
                Rule::expression => {
                    self.handle_expression(pair)?;
                }
                Rule::assignment => {
                    self.handle_assignment(pair)?;
                }
                Rule::WHITESPACE | Rule::COMMENT | Rule::EOI => continue,
                _ => unreachable!("Missing directive for parsing given pair: '{:?}'", pair),
            }
        }
        return Ok(());
    }

    /// Handle an expression. Expressions will always evaluate to a primitive.
    /// Evaluating an expression will decide which primitive it becomes based on its contents
    /// i.e. (1+1) -> Integer, (1+1.0) -> Float, (1.0 + "1") -> String
    fn handle_expression(&mut self, expr: Pair<'_, Rule>) -> Result<Primitive, JamalErr> {
        debug_assert!(expr.as_rule() == Rule::expression);
        let mut output = Vec::<Primitive>::with_capacity(2);
        let mut stack = Vec::<Operator>::with_capacity(1);

        for pair in expr.into_inner() {
            println!("{:?}", pair);
            match pair.as_rule() {
                // int
                Rule::integer => {
                    output.push(Primitive::Int(
                        pair.as_span().as_str().parse::<i32>().unwrap(),
                    ));
                }
                // bool
                Rule::boolean => {
                    output.push(Primitive::Bool(
                        pair.as_span().as_str().parse::<bool>().unwrap(),
                    ));
                }
                // op
                Rule::operator => {
                    stack.push(Operator::from_str(pair.as_span().as_str()));
                }
                // other
                Rule::WHITESPACE | Rule::COMMENT => {}
                _ => unreachable!(
                    "Missing directive for evaluating given pair in expression: '{:?}'",
                    pair
                ),
            }
        }

        // determine what type it should be
        let mut t = Primitive::Null;
        output.into_iter().for_each(|prim| {
            if !t.higher_priority_than(&prim) {
                t = prim.clone();
            }
        });

        // evaluate to new decided type

        return Ok(Primitive::Bool(true));
    }

    /// Handle an assignment
    fn handle_assignment(&mut self, expr: Pair<'_, Rule>) -> Result<(), JamalErr> {
        debug_assert!(expr.as_rule() == Rule::assignment);
        println!("{:?}", expr);
        return Ok(());
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_operator_enum() {
        assert!(Operator::from_str("+") == Operator::Add);
        assert!(Operator::from_str("-") == Operator::Sub);
        assert!(Operator::from_str("*") == Operator::Mul);
        assert!(Operator::from_str("/") == Operator::Div);
        assert!(Operator::from_str("^") == Operator::Pow);
        assert!(Operator::from_str("%") == Operator::Mod);
        assert!(Operator::from_str("&&") == Operator::And);
        assert!(Operator::from_str("||") == Operator::Or);
        assert!(Operator::from_str("==") == Operator::Eq);
        assert!(Operator::from_str("!=") == Operator::Ne);
    }

    #[test]
    fn test_primitive_enum() {
        let n = Primitive::Null;
        let b = Primitive::Bool(true);
        let i = Primitive::Int(1);
        let f = Primitive::Float(0.1);
        let s = Primitive::String("asdf".into());

        assert!(s.higher_priority_than(&n));
        assert!(s.higher_priority_than(&b));
        assert!(s.higher_priority_than(&i));
        assert!(s.higher_priority_than(&f));
        assert!(s.higher_priority_than(&Primitive::String("ghjk".into())));

        assert!(f.higher_priority_than(&n));
        assert!(f.higher_priority_than(&b));
        assert!(f.higher_priority_than(&i));
        assert!(f.higher_priority_than(&Primitive::Float(0.2)));
        assert!(!f.higher_priority_than(&s));

        assert!(i.higher_priority_than(&n));
        assert!(i.higher_priority_than(&b));
        assert!(i.higher_priority_than(&Primitive::Int(2)));
        assert!(!i.higher_priority_than(&f));
        assert!(!i.higher_priority_than(&s));

        assert!(b.higher_priority_than(&n));
        assert!(b.higher_priority_than(&Primitive::Bool(true)));
        assert!(!b.higher_priority_than(&i));
        assert!(!b.higher_priority_than(&f));
        assert!(!b.higher_priority_than(&s));

        assert!(n.higher_priority_than(&Primitive::Null));
        assert!(!n.higher_priority_than(&b));
        assert!(!n.higher_priority_than(&i));
        assert!(!n.higher_priority_than(&f));
        assert!(!n.higher_priority_than(&s));
    }

    #[test]
    fn test_scope_basic() -> Result<(), pest::error::Error<Rule>> {
        let parsed = JamalParser::parse(
            Rule::file,
            r#"
                // comment
                1 + 1;
                1 + (1 + 1);
                var balls = "string";
                "#,
        )?;
        let mut scope = Scope::new();
        scope.run(parsed);
        assert_eq!(
            scope.vars.get("balls").unwrap(),
            &Primitive::String("string".to_string())
        );

        return Ok(());
    }

    #[test]
    fn test_expression() -> Result<(), JamalErr> {
        let tests = vec![
            ("true", Primitive::Bool(true)),
            ("1 + 1", Primitive::Int(2)),
        ];
        let mut scope = Scope::new();
        for (inp, outp) in tests {
            let parsed = JamalParser::parse(Rule::expression, inp)?;
            for pair in parsed {
                if pair.as_rule() != Rule::expression {
                    continue;
                }
                let res = scope.handle_expression(pair)?;
                assert!(res == outp);
            }
        }
        return Ok(());
    }
}
