use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "jamal/jamal.pest"]
struct JamalParser;

pub struct Jamal {

}

impl Jamal {
//        pub fn load_file()

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

                // comment
                JamalParser::parse(Rule::COMMENT, "// this is a comment")?;
                JamalParser::parse(Rule::COMMENT, "/* this is a block comment */")?;
                JamalParser::parse(Rule::COMMENT, "/* this is a multiline
                                                      block comment */")?;
                JamalParser::parse(Rule::COMMENT, "").err().unwrap();

                // integer
                JamalParser::parse(Rule::integer, "0123456789")?;
                JamalParser::parse(Rule::integer, "+1234")?;
                JamalParser::parse(Rule::integer, "-1234")?;
                JamalParser::parse(Rule::integer, "").err().unwrap();
                JamalParser::parse(Rule::integer, "+").err().unwrap();
                JamalParser::parse(Rule::integer, "-").err().unwrap();

                // float
                // - left side
                JamalParser::parse(Rule::float, "0123456789")?;
                JamalParser::parse(Rule::float, "+1234")?;
                JamalParser::parse(Rule::float, "-1234")?;
                // - right side
                JamalParser::parse(Rule::float, ".0123456789")?;
                JamalParser::parse(Rule::float, "+.1234")?;
                JamalParser::parse(Rule::float, "-.1234")?;
                // - both
                JamalParser::parse(Rule::float, "514.0123456789")?;
                JamalParser::parse(Rule::float, "+43421.1234")?;
                JamalParser::parse(Rule::float, "-54.1234")?;
                // - neither
                JamalParser::parse(Rule::float, "").err().unwrap();
                JamalParser::parse(Rule::float, ".").err().unwrap();
                JamalParser::parse(Rule::float, "+").err().unwrap();
                JamalParser::parse(Rule::float, "-").err().unwrap();
                JamalParser::parse(Rule::float, "+.").err().unwrap();
                JamalParser::parse(Rule::float, "-.").err().unwrap();

                // string
                JamalParser::parse(Rule::string, r#""this is a string with quotes""#)?;
                JamalParser::parse(Rule::string, r#"'this is a string with apostrophes'"#)?;
                JamalParser::parse(Rule::string, r#""this is a
                                                    multi-line
                                                    string""#)?;
                JamalParser::parse(Rule::string, r#""これわ日本語のstringですよ!""#)?;

                return Ok(());
        }

}
