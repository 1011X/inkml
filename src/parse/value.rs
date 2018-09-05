use super::{ParseResult, ParseError};

#[derive(Debug, PartialEq)]
pub enum Value {
    Inferred,
    NotGiven,
    Bool(bool),
    //Number {
}

impl Value {
    pub fn parse(input: &str) -> ParseResult<(&str, Self)> {
        // value ::= difference_order?  wsp* "-"? wsp* number | "T" | "F" | "*" | "?"
        
        if input.is_empty() {
            return Err(ParseError::EndOfFile);
        }
        
        // {! ' " wsp - number T F * ?}
        /*
        if input.starts_with('*') {
            return Ok((&input[1..], Value::Inferred));
        }
        
        if input.starts_with('?') {
            return Ok((&input[1..], Value::NotGiven));
        }
        
        if input.starts_with('T') {
            return Ok((&input[1..], Value::Bool(true)));
        }
        
        if input.starts_with('F') {
            return Ok((&input[1..], Value::Bool(false)));
        }
        
        fn number(mut i: &str) -> ParseResult<(&str, i64)> {
            // TODO
            // difference_order?
            
            // wsp*
            i = i.trim_left_matches(wsp);
            
            // "-"?
            let mut negated = false;
            if i.starts_with('-') {
                negated = true;
                i = &i[1..];
            }
            
            // wsp*
            i = i.trim_left_matches(wsp);
            
            // number
            let num = i.matches(char::is_ascii_digit)
                .nth(0)
                .ok_or(ParseError::UnexpectedValue(i))?;
            
            Ok((&i
        }
        
        // difference_order? wsp* "-"? wsp* number
        if let Ok((i, num)) = number(input)? {
            return Ok((i, Value::Number(order, num)));
        }
        */
        let value = match &input[..1] {
            // "*"
            "*" => Value::Inferred,
            // "?"
            "?" => Value::NotGiven,
            // "T"
            "T" => Value::Bool(true),
            // "F"
            "F" => Value::Bool(false),
            // number
            //"!" | "'" | "\"" | "-" | x if 
            // TODO
            // difference_order?  wsp* "-"? wsp* number
            _ => return Err(ParseError::UnexpectedValue(input))
        };
        
        Ok((&input[1..], value))
    }
}

#[cfg(test)]
mod value {
    use super::Value;
    
    #[test]
    fn inferred() {
        assert_eq!(("", Value::Inferred), Value::parse("*").unwrap());
    }
    
    #[test]
    fn not_given() {
        assert_eq!(("", Value::NotGiven), Value::parse("?").unwrap());
    }
    
    #[test]
    fn boolean() {
        assert_eq!(("", Value::Bool(true)), Value::parse("T").unwrap());
        assert_eq!(("", Value::Bool(false)), Value::parse("F").unwrap());
    }
}
