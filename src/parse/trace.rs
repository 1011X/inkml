use std::io::Read;

//use xsd;
use super::{ParseError, ParseResult, wsp};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TraceType { PenDown, PenUp, Indeterminate }

impl Default for TraceType {
    fn default() -> Self { TraceType::PenDown }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TraceContinuation {
    Begin,
    Middle(String),
    End(String),
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Trace {
    
    /// List of data points that describe this trace.
    pub content: Vec<Point>,
    
	/* ATTRIBUTES */
	
	// The identifier for this trace.
	// Required: no, Default: none
	//id: Option<String>,
	
	// The type of this trace.
	// Required: no, Default: "penDown"
	//kind: TraceType,
	
	// This attribute indicates whether this trace is a trace fragment, and if
	// so, where this trace is located in the set of continuation traces.
	// Required: no, Default: none
	//continuation: Option<TraceContinuation>,
	
	// The context for this trace. Any values in this context over-ride the
	// values in the inherited context.
	// Required: no, Default: "#DefaultContext," unless this <trace> is
	// contained within a <traceGroup>, then inherit from the <traceGroup>.
	//context_ref: String,
	
	// The brush for this trace.
	// Required: no, Default: Inherited from context.
	//brush_ref: String,
	
	// The duration of this trace, in milliseconds.
	// Required: no, Default: none
	//duration: Option<f64>,
	
	// The relative timestamp or time-of-day for the start of this trace, in
	// milliseconds.
	// Required: no, Default: none
	//time_offset: Option<f64>,
}

impl Trace {
    
    pub(crate) fn parse<R: Read>(er: &xml::reader::EventReader<R>) -> xml::reader::Result<Trace> {
        unimplemented!()
    }
    
    /// Parses `<trace>` data, which is just a sequence of points.
    /// 
    /// Here, there's a mandatory minimum of 1 point. More points can follow
    /// when preceded by a comma. There's an optional ending comma, then maybe
    /// some whitespace characters.
    /// 
    /// In Backus-Naur form, it looks like this:
    /// 
    /// ```bnf
    /// trace ::= point ("," point)* ","? wsp*
    /// ```
    pub fn parse_content(input: &str) -> ParseResult<Vec<Point>> {
        // To help with understanding, the Backus-Naur form has been expanded a
        // bit and augmented with possible states (in angle brackets):
        // 
        //     trace ::= <0> point <1> ("," point)* <2> ","? <3> wsp* <4>
        // 
        // Possible paths each state can take:
        // 0 -> {point -> 1}
        // 1 -> {"," point -> 1, "" -> 2}
        // 2 -> {"," -> 3, "" -> 3}
        // 3 -> {wsp -> 3, "" -> 4}
        // 4 -> {}
        
        let mut points = Vec::new();
        
        // <0> point
        let (point, mut input) = Point::parse(input)?;
        points.push(point);
        
        // <1> ("," point)*
        loop {
            // Because states 1 and 2 can both take a comma, some backtracking
            // is done in case the `point` in this state fails.
            let backtrack = input;
            
            // <1> "" -> 2
            // This is a special case for no input or whitespace because this is
            // the top-level parser and both of these cases are valid at the end
            // of the data string.
            // TODO: double-check this.
            if input.is_empty() || input.starts_with(wsp) { break }
            
            // ","
            if !input.starts_with(',') {
                // anything else is not valid
                // TODO double-check this
                return Err(ParseError::UnexpectedValue);
            }
            input = &input[1..];
            
            // point
            match Point::parse(input) {
                Ok((point, i)) => {
                    input = i;
                    points.push(point);
                }
                // <1> "" -> "," -> 3
                Err(_) => {
                    input = backtrack;
                    break;
                }
            }
        }
        
        // <2> ","?
        if input.starts_with(',') {
            input = &input[1..];
        }
        
        // <3> wsp*
        input = input.trim_start_matches(wsp);
        
        // ensure there's no extra data
        if !input.is_empty() {
            return Err(ParseError::UnexpectedValue);
        }
        
        // <4>
        Ok(points)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Point(pub Vec<Value>);

impl Point {
    pub(crate) fn parse(mut input: &str) -> ParseResult<(Self, &str)> {
        // point ::= (wsp* value)+ wsp*
        //       ::= wsp* value (wsp* value)* wsp*
        //       ::= wsp* value wsp* (value wsp*)*
        let mut values = Vec::new();
        
        // wsp*
        input = input.trim_start_matches(wsp);
        
        // value
        let (value, mut input) = Value::parse(input)?;
        values.push(value);
        
        // wsp*
        input = input.trim_start_matches(wsp);
        
        // (value wsp*)*
        loop {
            // value
            match Value::parse(input) {
                Ok((value, i)) => {
                    input = i;
                    values.push(value);
                }
                Err(_) => break
            }
            
            // wsp*
            input = input.trim_start_matches(wsp);
        }
        
        Ok((Point(values), input))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Inferred,
    NotGiven,
    Bool(bool),
    //Number(Option<DiffOrder>, Number),
}

impl Value {
    pub fn parse(mut input: &str) -> ParseResult<(Self, &str)> {
        // value ::= difference_order? wsp* "-"? wsp* number | "T" | "F" | "*" | "?"
        
        if input.is_empty() {
            return Err(ParseError::EndOfFile);
        }
        
        // "*" | "?" | "T" | "F"
        match &input[..1] {
            "*" => return Ok((Value::Inferred, &input[1..])),
            "?" => return Ok((Value::NotGiven, &input[1..])),
            "T" => return Ok((Value::Bool(true), &input[1..])),
            "F" => return Ok((Value::Bool(false), &input[1..])),
            _ => {}
        }
        
        // difference_order? wsp* "-"? wsp* number
        
        // difference_order ::= "!" | "'" | '"'
        let diff_order = match &input[..1] {
            "!" => {
                input = &input[1..];
                Some(DiffOrder::Explicit)
            }
            "'" => {
                input = &input[1..];
                Some(DiffOrder::First)
            }
            "\"" => {
                input = &input[1..];
                Some(DiffOrder::Second)
            }
            _ => None
        };
        
        // wsp*
        input = input.trim_start_matches(wsp);
        
        // "-"?
        
        
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DiffOrder { Explicit, First, Second }



#[cfg(test)]
mod tests {
    use super::{Point, Trace, Value};
    
    #[test]
    #[should_panic]
    fn empty_string() {
        Trace::parse_content("").unwrap();
    }
    
    #[test]
    fn single() {
        let expect = vec![Point(vec![Value::Inferred])];
        assert_eq!(expect, Trace::parse_content("*").unwrap());
        //assert_eq!(expect, Trace::parse_content("*,").unwrap());
        assert_eq!(expect, Trace::parse_content("* ").unwrap());
        //assert_eq!(expect, Trace::parse_content("*, ").unwrap());
    }
    
    #[test]
    fn many() {
        let expect = vec![
            Point(vec![Value::Inferred]),
            Point(vec![Value::Inferred])
        ];
        assert_eq!(expect, Trace::parse_content("*,*").unwrap());
        assert_eq!(expect, Trace::parse_content("*,*,").unwrap());
        assert_eq!(expect, Trace::parse_content("*,* ").unwrap());
        assert_eq!(expect, Trace::parse_content("*,*, ").unwrap());
    }
    
    mod point {
        use super::super::{Point, Value};
        
        #[test]
        #[should_panic]
        fn empty_string() {
            Point::parse("").unwrap();
        }
        
        #[test]
        fn single() {
            let expect = (Point(vec![Value::Inferred]), "");
            assert_eq!(expect, Point::parse("*").unwrap());
            assert_eq!(expect, Point::parse(" *").unwrap());
            assert_eq!(expect, Point::parse("* ").unwrap());
            assert_eq!(expect, Point::parse(" \t*\r\n").unwrap());
        }
        
        #[test]
        fn many() {
            let expect = (Point(vec![Value::Inferred, Value::Inferred]), "");
            assert_eq!(expect, Point::parse("**").unwrap());
            assert_eq!(expect, Point::parse("* *").unwrap());
            assert_eq!(expect, Point::parse(" * *").unwrap());
            assert_eq!(expect, Point::parse(" * * ").unwrap());
        }
    }
    
    mod value {
        use super::super::Value;
        
        #[test]
        fn inferred() {
            assert_eq!((Value::Inferred, ""), Value::parse("*").unwrap());
        }
        
        #[test]
        fn notgiven() {
            assert_eq!((Value::NotGiven, ""), Value::parse("?").unwrap());
        }
        
        #[test]
        fn boolean() {
            assert_eq!((Value::Bool(true), ""), Value::parse("T").unwrap());
            assert_eq!((Value::Bool(false), ""), Value::parse("F").unwrap());
        }
    }
}
