// TODO:
// + what will the api look like?
//   + will it be like the xml and svg crates, i.e. with events?
//   + or more "high-level" than that?
//   + ...or both?
//   + is an api like svg's necessary if xml-rs can already achieve the same thing?
// + how will the data be presented?
//   + what fields will be public?
//   + how will modules be organized/structured?
// 
// 


pub mod parse;
pub mod node;
//mod channel;
//mod context;
//mod brush;

/*
mod elements {
    mod definitions;
    mod context;
    mod trace;
    mod traceGroup;
    mod traceView;
    mod annotation;
    mod annotationXML;
}
*/

use parse::Trace;
use parse::Point;
use parse::Value;
use parse::{wsp, digit, hex};
use parse::{ParseResult, ParseError};
use node::Node;

use std::io::Read;

//use xml::attribute::{Attribute, OwnedAttribute};

struct Ink {
	document_id: Option<String>,
	children: Vec<Box<dyn Node>>,
	/*
	definitions: Vec<Definition>,
	contexts: Vec<Context>,
	traces: Vec<Trace>,
	trace_groups: Vec<TraceGroup>,
	trace_views: Vec<TraceView>,
	annotations: Vec<Annotation>,
	annotations_xml: Vec<AnnotationXML>,
	*/
}

/*
value   ::= difference_order?  wsp* "-"? wsp* number | "T" | "F" | "*" | "?"
number  ::= (decimal | double | hex)
double  ::= decimal ("e"|"E") ("+"|"-")? digit+ 
decimal ::= digit+ ("." digit*)? | "." digit+
difference_order ::= ("!" | "'" | '"')
*/



//enum DifferenceOrder { First, Second, Third }


pub static INKML_URI: &str = "http://www.w3.org/2003/InkML";

static TEST_INKML: &str = include_str!("minimal.ink");

fn open<R: Read>(source: R) -> Ink {
	unimplemented!();
}



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
