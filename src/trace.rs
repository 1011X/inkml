//use xsd;
use super::{Point, ParseError, ParseResult, wsp}; 

//mod channel;

#[derive(Debug, PartialEq)]
enum TraceType { PenDown, PenUp, Indeterminate }

impl Default for TraceType {
    fn default() -> Self { TraceType::PenDown }
}

#[derive(Debug, PartialEq)]
pub enum TraceContinuation {
    Begin,
    //Middle(xsd::AnyUri),
    Middle(String),
    //End(xsd::AnyUri)
    End(String),
}

#[derive(Debug, PartialEq)]
pub struct Trace {
    
    /// List of data points that describe this trace.
    pub content: Vec<Point>,
    
	/* ATTRIBUTES */
	
	/// The identifier for this trace.
	/// Required: no, Default: none
	//id: Option<xsd::ID>,
	id: Option<String>,
	
	/// The type of this trace.
	/// Required: no, Default: "penDown"
	kind: Option<TraceType>,
	
	/// This attribute indicates whether this trace is a trace fragment, and if
	/// so, where this trace is located in the set of continuation traces.
	/// Required: no, Default: none
	continuation: Option<TraceContinuation>,
	
	/// The context for this trace. Any values in this context over-ride the
	/// values in the inherited context.
	/// Required: no, Default: "#DefaultContext," unless this <trace> is
	/// contained within a <traceGroup>, then inherit from the <traceGroup>.
	//context_ref: Option<xsd::AnyUri>,
	context_ref: Option<String>,
	
	/// The brush for this trace.
	/// Required: no, Default: Inherited from context.
	//brush_ref: Option<xsd::AnyUri>,
	brush_ref: Option<String>,
	
	/// The duration of this trace, in milliseconds.
	/// Required: no, Default: none
	//duration: Option<xsd::Decimal>,
	duration: Option<f64>,
	
	/// The relative timestamp or time-of-day for the start of this trace, in
	/// milliseconds.
	/// Required: no, Default: none
	//time_offset: Option<xsd::Decimal>,
	time_offset: Option<f64>,
}

impl Trace {
    pub fn new(points: Vec<Point>) -> Trace {
        Trace {
            content: points,
            id: None,
            kind: None,
            continuation: None,
            context_ref: None,
            brush_ref: None,
            duration: None,
            time_offset: None,
        }
    }
    // trace ::= point ("," point)* ","? wsp*
    //       ::= <0> point <1> ("," point)* <2> ","? <3> wsp* <4>
    // 0 -> {point -> 1}
    // 1 -> {"," -> point -> 1, "" -> 2}
    // 2 -> {"," -> 3, "" -> 3}
    // 3 -> {wsp -> 3, "" -> 4}
    // 4 -> {}
    pub fn parse(input: &str) -> ParseResult<(&str, Self)> {
        let mut points = Vec::new();
        
        // <0>
        // point
        let (mut input, point) = Point::parse(input)?;
        points.push(point);
        
        // <1>
        // ("," point)*
        loop {
            let backtrack = input;
            
            // <1> "" -> 2
            if input.is_empty() || input.starts_with(wsp) { break }
            
            // ","
            if !input.starts_with(',') {
                return Err(ParseError::UnexpectedValue(input));
            }
            input = &input[1..];
            
            // point
            match Point::parse(input) {
                Ok((i, point)) => {
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
        
        // <2>
        // ","?
        if input.starts_with(',') {
            input = &input[1..];
        }
        
        // <3>
        // wsp*
        input = input.trim_left_matches(wsp);
        
        // <4>
        Ok((input, Trace::new(points)))
    }
}

#[cfg(test)]
mod test {
    use super::Trace;
    use super::super::{Point, Value};
    
    #[test]
    #[should_panic]
    fn empty_string() {
        Trace::parse("").unwrap();
    }
    
    #[test]
    fn single() {
        let expect = ("", Trace::new(vec![Point(vec![Value::Inferred])]));
        assert_eq!(expect, Trace::parse("*").unwrap());
        assert_eq!(expect, Trace::parse("*,").unwrap());
        assert_eq!(expect, Trace::parse("* ").unwrap());
        assert_eq!(expect, Trace::parse("*, ").unwrap());
    }
    
    #[test]
    fn many() {
        let expect = ("", Trace::new(vec![
            Point(vec![Value::Inferred]),
            Point(vec![Value::Inferred])
        ]));
        assert_eq!(expect, Trace::parse("*,*").unwrap());
        assert_eq!(expect, Trace::parse("*,*,").unwrap());
        assert_eq!(expect, Trace::parse("*,* ").unwrap());
        assert_eq!(expect, Trace::parse("*,*, ").unwrap());
    }
}




/*
pub struct TraceFormat {
	/// The unique identifier for this trace format.
	/// Required: no, Default: none
	//id: Option<xsd::ID>,
	id: Option<String>,
	
	channels: Vec<Channel>,
	intermittent_channels: Vec<IntermittentChannel>,
}
*/
