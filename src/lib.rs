extern crate xml;

mod trace;
//mod channel;
//mod context;
//mod brush;

pub use trace::Trace;

//use xml::attribute::{Attribute, OwnedAttribute};

/*
type AnyURI = String;
type ID = String;

struct Ink {
	document_id: Option<AnyURI>,
	
	definitions: Vec<Definition>,
	contexts: Vec<Context>,
	traces: Vec<Trace>,
	trace_groups: Vec<TraceGroup>,
	trace_views: Vec<TraceView>,
	annotations: Vec<Annotation>,
	annotations_xml: Vec<AnnotationXML>,
}
*/

pub type ParseResult<'a, T> = Result<T, ParseError<'a>>;

#[derive(Debug)]
pub enum ParseError<'a> {
    EndOfFile,
    UnexpectedValue(&'a str),
}

/*
value   ::= difference_order?  wsp* "-"? wsp* number | "T" | "F" | "*" | "?"
number  ::= (decimal | double | hex)
double  ::= decimal ("e"|"E") ("+"|"-")? digit+ 
decimal ::= digit+ ("." digit*)? | "." digit+
difference_order ::= ("!" | "'" | '"')
*/

// TODO: include Unicode whitespace code points?
// wsp ::= (#x20 | #x9 | #xD | #xA)
pub fn wsp(c: char) -> bool {
    ['\x20', '\x09', '\x0D', '\x0A'].contains(&c)
}

// digit ::= ("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
fn digit(c: char) -> bool {
    c.is_ascii_digit()
}

// hex ::= "#" (digit | "A" | "B" | "C" | "D" | "E" | "F")+
fn hex(mut input: &str) -> ParseResult<(&str, &str)> {
    let mut end = 0;
    let i = input;
    
    // "#"
    if !input.starts_with('#') {
        return Err(ParseError::UnexpectedValue(input));
    }
    input = &input[1..];
    end += 1;
    
    // (digit | "A" | "B" | "C" | "D" | "E" | "F")
    if !input.starts_with(|c: char| c.is_ascii_hexdigit()) {
        return Err(ParseError::UnexpectedValue(input));
    }
    input = &input[1..];
    end += 1;
    
    // (digit | "A" | "B" | "C" | "D" | "E" | "F")*
    while input.starts_with(|c: char| c.is_ascii_hexdigit()) {
        input = &input[1..];
        end += 1;
    }
    
    Ok((input, &i[..end]))
}


#[derive(Debug, PartialEq)]
pub struct Point(Vec<Value>);

impl Point {
    fn parse(mut input: &str) -> ParseResult<(&str, Self)> {
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
    use super::{Point, Value};
    
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



//enum DifferenceOrder { First, Second, Third }

#[derive(Debug, PartialEq)]
pub enum Value {
    Inferred,
    NotGiven,
    Bool(bool),
    //Number {
}

impl Value {
    fn parse(input: &str) -> ParseResult<(&str, Self)> {
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



static TEST_INKML: &str = include_str!("test.inkml");



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn open_simple_inkml() {
        use xml::reader::XmlEvent;
        use xml::name::OwnedName;
        
        let stream = xml::EventReader::from_str(TEST_INKML);
        let mut current_path = Vec::new();
        let mut trace = None;
        
        for event in stream {
            match event.unwrap() {
                XmlEvent::StartElement { name: OwnedName { local_name, .. }, .. } => {
                    current_path.push(local_name.clone());
                }
                XmlEvent::EndElement { name: OwnedName { local_name, .. }, .. } => {
                    assert!(*current_path.last().unwrap() == local_name);
                    current_path.pop();
                }
                XmlEvent::Characters(data) => {
                    if current_path.last().unwrap() == "trace" {
                        trace = Some(Trace::parse(&data).unwrap().1);
                    }
                }
                _ => {}
            }
        }
        
        assert_eq!(trace.unwrap(), Trace::with_points(vec![
            Point(vec![Value::Inferred, Value::Inferred]),
            Point(vec![Value::NotGiven]),
            Point(vec![Value::Bool(false), Value::Bool(true)]),
        ]));
    }
}
