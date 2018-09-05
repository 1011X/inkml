extern crate xml;

pub mod parse;
//mod channel;
//mod context;
//mod brush;

use parse::Trace;
use parse::Point;
use parse::Value;
use parse::{wsp, digit, hex};
use parse::{ParseResult, ParseError};

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

/*
value   ::= difference_order?  wsp* "-"? wsp* number | "T" | "F" | "*" | "?"
number  ::= (decimal | double | hex)
double  ::= decimal ("e"|"E") ("+"|"-")? digit+ 
decimal ::= digit+ ("." digit*)? | "." digit+
difference_order ::= ("!" | "'" | '"')
*/



//enum DifferenceOrder { First, Second, Third }



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
