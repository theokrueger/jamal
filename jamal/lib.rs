//! JAMAL language
//!
//! JAMAL is a string maniuplation-focused language
//! aimed at making formatting text as easy as possible

use pest::Parser;
use pest_derive::Parser;
use std::fs;

mod scope;
use scope::Scope;

#[derive(Parser)]
#[grammar = "jamal/jamal.pest"]
pub struct JamalParser;

#[derive(Debug)]
pub enum JamalErr {
        FileReadErr,
        FileParseErr,
}

impl From<pest::error::Error<Rule>> for JamalErr {
    fn from(item: pest::error::Error<Rule>) -> Self {
        return JamalErr::FileParseErr;
    }
}

pub struct Jamal {}

impl Jamal {
        pub fn parse_file(path: String) -> Result<(), JamalErr> {
                let unparsed = match fs::read_to_string(path) {
                        Ok(s) => s,
                        Err(_) => return Err(JamalErr::FileReadErr),
                };

                let parsed = match JamalParser::parse(Rule::file, &unparsed) {
                        Ok(f) => f,
                        Err(e) => return Err(JamalErr::FileParseErr),
                };

            let root_scope = Scope::new().run(parsed);

                return Ok(());
        }
}

mod tests {
        use super::*;

        #[test]
        fn test_atomics() -> Result<(), pest::error::Error<Rule>> {
                // whitespace
                JamalParser::parse(Rule::WHITESPACE, " ")?;
                JamalParser::parse(Rule::WHITESPACE, "\t")?;
                JamalParser::parse(Rule::WHITESPACE, "   \t   \t")?;
                JamalParser::parse(Rule::WHITESPACE, "").err().unwrap();
                JamalParser::parse(Rule::WHITESPACE, "asdfaf").err().unwrap();

                // comment
                JamalParser::parse(Rule::COMMENT, "// this is a comment")?;
                JamalParser::parse(Rule::COMMENT, "/* this is a block comment */")?;
                JamalParser::parse(Rule::COMMENT, "/* this is a multiline
                                                      block comment */")?;
                JamalParser::parse(Rule::COMMENT, "").err().unwrap();
                JamalParser::parse(Rule::COMMENT, "not a comment").err().unwrap();

                // integer
                JamalParser::parse(Rule::integer, "0123456789")?;
                JamalParser::parse(Rule::integer, "+1234")?;
                JamalParser::parse(Rule::integer, "-1234")?;
                JamalParser::parse(Rule::integer, "0123456789e123")?;
                JamalParser::parse(Rule::integer, "+1234e151")?;
                JamalParser::parse(Rule::integer, "-1234e5123")?;
                JamalParser::parse(Rule::integer, "0123456789e+123")?;
                JamalParser::parse(Rule::integer, "+1234e+151")?;
                JamalParser::parse(Rule::integer, "-1234e+5123")?;
                JamalParser::parse(Rule::integer, "0123456789e-123")?;
                JamalParser::parse(Rule::integer, "+1234e-151")?;
                JamalParser::parse(Rule::integer, "-1234e-5123")?;
                JamalParser::parse(Rule::integer, "").err().unwrap();
                JamalParser::parse(Rule::integer, "+").err().unwrap();
                JamalParser::parse(Rule::integer, "-").err().unwrap();
                JamalParser::parse(Rule::integer, "object").err().unwrap();

                // float
                // - left side
                JamalParser::parse(Rule::float, "0123456789")?;
                JamalParser::parse(Rule::float, "+1234")?;
                JamalParser::parse(Rule::float, "-1234")?;
                JamalParser::parse(Rule::float, "0123456789e99")?;
                JamalParser::parse(Rule::float, "+1234e421")?;
                JamalParser::parse(Rule::float, "-1234e344")?;
                JamalParser::parse(Rule::float, "0123456789e+99")?;
                JamalParser::parse(Rule::float, "+1234e+421")?;
                JamalParser::parse(Rule::float, "-1234e+344")?;
                JamalParser::parse(Rule::float, "0123456789e-99")?;
                JamalParser::parse(Rule::float, "+1234e-421")?;
                JamalParser::parse(Rule::float, "-1234e-344")?;
                // - right side
                JamalParser::parse(Rule::float, ".0123456789")?;
                JamalParser::parse(Rule::float, "+.1234")?;
                JamalParser::parse(Rule::float, "-.1234")?;
                JamalParser::parse(Rule::float, ".0123456789e69")?;
                JamalParser::parse(Rule::float, "+.1234e99")?;
                JamalParser::parse(Rule::float, "-.1234e39")?;
                JamalParser::parse(Rule::float, ".0123456789e+69")?;
                JamalParser::parse(Rule::float, "+.1234e+99")?;
                JamalParser::parse(Rule::float, "-.1234e+39")?;
                JamalParser::parse(Rule::float, ".0123456789e-69")?;
                JamalParser::parse(Rule::float, "+.1234e-99")?;
                JamalParser::parse(Rule::float, "-.1234e-39")?;
                // - both
                JamalParser::parse(Rule::float, "514.0123456789")?;
                JamalParser::parse(Rule::float, "+43421.1234")?;
                JamalParser::parse(Rule::float, "-54.1234")?;
                JamalParser::parse(Rule::float, "514.0123456789e99")?;
                JamalParser::parse(Rule::float, "+43421.1234e12")?;
                JamalParser::parse(Rule::float, "-54.1234e51")?;
                JamalParser::parse(Rule::float, "514.0123456789e+99")?;
                JamalParser::parse(Rule::float, "+43421.1234e+12")?;
                JamalParser::parse(Rule::float, "-54.1234e+51")?;
                JamalParser::parse(Rule::float, "514.0123456789e-99")?;
                JamalParser::parse(Rule::float, "+43421.1234e-12")?;
                JamalParser::parse(Rule::float, "-54.1234e-51")?;
                // - neither
                JamalParser::parse(Rule::float, "").err().unwrap();
                JamalParser::parse(Rule::float, ".").err().unwrap();
                JamalParser::parse(Rule::float, "+").err().unwrap();
                JamalParser::parse(Rule::float, "-").err().unwrap();
                JamalParser::parse(Rule::float, "e99").err().unwrap();
                JamalParser::parse(Rule::float, "+.").err().unwrap();
                JamalParser::parse(Rule::float, "-.").err().unwrap();
                JamalParser::parse(Rule::float, "object").err().unwrap();

                // string
                JamalParser::parse(Rule::string, r#""this is a string with quotes""#)?;
                JamalParser::parse(Rule::string, r#"'this is a string with apostrophes'"#)?;
                JamalParser::parse(Rule::string, r#""this is a
                                                     multi-line
                                                     string""#)?;
                JamalParser::parse(Rule::string, r#""これわ日本語のstringですよ!""#)?;
                JamalParser::parse(Rule::string, "object").err().unwrap();

                // boolean
                JamalParser::parse(Rule::boolean, "true")?;
                JamalParser::parse(Rule::boolean, "false")?;
                JamalParser::parse(Rule::boolean, "").err().unwrap();
                JamalParser::parse(Rule::boolean, "True").err().unwrap();
                JamalParser::parse(Rule::boolean, "False").err().unwrap();
                JamalParser::parse(Rule::boolean, "object").err().unwrap();

                // operators
                JamalParser::parse(Rule::operator, "+")?;
                JamalParser::parse(Rule::operator, "-")?;
                JamalParser::parse(Rule::operator, "/")?;
                JamalParser::parse(Rule::operator, "*")?;
                JamalParser::parse(Rule::operator, "^")?;
                JamalParser::parse(Rule::operator, "%")?;
                JamalParser::parse(Rule::operator, "&&")?;
                JamalParser::parse(Rule::operator, "||")?;
                JamalParser::parse(Rule::operator, "==")?;
                JamalParser::parse(Rule::operator, "!=")?;
                JamalParser::parse(Rule::operator, "").err().unwrap();
                JamalParser::parse(Rule::operator, "&").err().unwrap();
                JamalParser::parse(Rule::operator, "|").err().unwrap();
                JamalParser::parse(Rule::operator, "~=").err().unwrap();
                JamalParser::parse(Rule::operator, "1 + 1").err().unwrap();
                JamalParser::parse(Rule::operator, "1+1").err().unwrap();
                JamalParser::parse(Rule::operator, "object").err().unwrap();

                // - objects
                JamalParser::parse(Rule::object, "objects_are_ascii_only_1234")?;
                JamalParser::parse(Rule::object, "ThEy_Are_CASE_sensitive")?;
                JamalParser::parse(Rule::object, "1they_cannot_start_with_numbers").err().unwrap();
                JamalParser::parse(Rule::object, "#$%^*&").err().unwrap();
                JamalParser::parse(Rule::object, "_____").err().unwrap();
                JamalParser::parse(Rule::object, "true").err().unwrap();
                JamalParser::parse(Rule::object, "false").err().unwrap();
                JamalParser::parse(Rule::object, "let").err().unwrap();

                return Ok(());
        }


        #[test]
        fn test_expression() -> Result<(), pest::error::Error<Rule>> {
                JamalParser::parse(Rule::expression, "1 + 1")?;
                JamalParser::parse(Rule::expression, r#""string" + 1"#)?;
                JamalParser::parse(Rule::expression, "1^2")?;
                JamalParser::parse(Rule::expression, "true || false")?;
                JamalParser::parse(Rule::expression, "a")?;
                JamalParser::parse(Rule::expression, "false && (1 || name)")?;
                JamalParser::parse(Rule::expression, "fortnite && ((19 || card) + (balls)) / (balls^2)")?;
                JamalParser::parse(Rule::expression, "|| b").err().unwrap();

                return Ok(());
        }

        #[test]
        fn test_assignment_declaration() -> Result<(), pest::error::Error<Rule>> {
                JamalParser::parse(Rule::assignment, "let a = fortnite && ((19 || card) + (balls)) / (balls^2)")?;
                JamalParser::parse(Rule::assignment, r#"const a ="string""#)?;
                JamalParser::parse(Rule::assignment, r#"var a= "string""#)?;
                JamalParser::parse(Rule::assignment, r#"a="string""#)?;
                JamalParser::parse(Rule::assignment, r#"a = "string""#)?;
                JamalParser::parse(Rule::assignment, "coa=").err().unwrap();
                JamalParser::parse(Rule::assignment, "=").err().unwrap();
                JamalParser::parse(Rule::assignment, "=12").err().unwrap();

                return Ok(());
        }
}
