use super::{ParseResult, ParseError};

#[derive(Debug, PartialEq)]
pub enum Value {
    Inferred,
    NotGiven,
    Bool(bool),
    //Number(Number),
}

impl Value {
    pub fn parse(input: &str) -> ParseResult<(Self, &str)> {
        // value ::= difference_order?  wsp* "-"? wsp* number | "T" | "F" | "*" | "?"
        let mut tokens = input.chars();
        
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
        Ok((match tokens.next() {
        	None => return Err(ParseError::EndOfFile),
            Some('*') => Value::Inferred,
            Some('?') => Value::NotGiven,
            Some('T') => Value::Bool(true),
            Some('F') => Value::Bool(false),
            // TODO number
            // difference_order?  wsp* "-"? wsp* number
            // number ::= decimal | double | hex
            /*
            Some(diff @ '!' | '\'' | '"') => {
            	let diff = DiffOrder::from_char(diff);
            	
            	let tokens = tokens.skip_while(|&c| wsp(c));
            	let negative = tokens.next();
            }
            */
            _ => return Err(ParseError::UnexpectedValue)
        }, &input[1..]))
        
        //Ok((&input[1..], value))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DiffOrder { Explicit, Single, Second }

impl DiffOrder {
	fn from_char(c: char) -> DiffOrder {
		match c {
			'!'  => DiffOrder::Explicit,
			'\'' => DiffOrder::Single,
			'"'  => DiffOrder::Second,
			_ => unreachable!()
		}
	}
}



#[cfg(test)]
mod tests {
    use super::Value;
    
    #[test]
    fn inferred() {
        assert_eq!((Value::Inferred, ""), Value::parse("*").unwrap());
    }
    
    #[test]
    fn not_given() {
        assert_eq!((Value::NotGiven, ""), Value::parse("?").unwrap());
    }
    
    #[test]
    fn boolean() {
        assert_eq!((Value::Bool(true), ""), Value::parse("T").unwrap());
        assert_eq!((Value::Bool(false), ""), Value::parse("F").unwrap());
    }
}
