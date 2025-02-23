//! JAMAL scope
//!
//! A scope is 

use pest::{Parser, iterators::Pairs};

use crate::{JamalParser, JamalErr, Rule};

pub struct Scope {}

impl Scope {
        pub fn new(_: Pairs<'_, Rule>) -> Self {
                return Self {};
        }
}

mod tests {
        use super::*;

        #[test]
        fn test_scope_basic() -> Result<(), pest::error::Error<Rule>> {
                let a = JamalParser::parse(Rule::file, r#"
                // comment
                1 + 1;
                var balls = "string";
                "#)?;

                println!("{:?}", a);
                assert!(false);

                return Ok(());
        }
}
