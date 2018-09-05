use super::{
	ParseResult,
	ParseError,
	wsp,
};
use super::super::Value;

#[derive(Debug, PartialEq)]
pub struct Point(pub Vec<Value>);

impl Point {
    pub fn parse(mut input: &str) -> ParseResult<(&str, Self)> {
        // point ::= (wsp* value)+ wsp*
        //       ::= wsp* value wsp* (value wsp*)*
        let mut values = Vec::new();
        
        // wsp*
        input = input.trim_left_matches(wsp);
        
        // value
        let (mut input, value) = Value::parse(input)?;
        values.push(value);
        
        // wsp*
        input = input.trim_left_matches(wsp);
        
        // (value wsp*)*
        loop {
            // value
            match Value::parse(input) {
                Ok((i, value)) => {
                    input = i;
                    values.push(value);
                }
                Err(_) => break
            }
            
            // wsp*
            input = input.trim_left_matches(wsp);
        }
        
        Ok((input, Point(values)))
    }
}

#[cfg(test)]
mod point {
    use super::Point;
    use super::super::Value;
    
    #[test]
    #[should_panic]
    fn empty_string() {
        Point::parse("").unwrap();
    }
    
    #[test]
    fn single() {
        let expect = ("", Point(vec![Value::Inferred]));
        assert_eq!(expect, Point::parse("*").unwrap());
        assert_eq!(expect, Point::parse(" *").unwrap());
        assert_eq!(expect, Point::parse(" \t*\r\n").unwrap());
    }
    
    #[test]
    fn many() {
        let expect = ("", Point(vec![Value::Inferred, Value::Inferred]));
        assert_eq!(expect, Point::parse("**").unwrap());
        assert_eq!(expect, Point::parse("* *").unwrap());
        assert_eq!(expect, Point::parse(" * *").unwrap());
        assert_eq!(expect, Point::parse(" * * ").unwrap());
    }
}
