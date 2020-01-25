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

use self::parse::Trace;

use std::io::Read;
use xml::reader::XmlEvent;
use xml::name::OwnedName;
//use xml::attribute::{Attribute, OwnedAttribute};
use xml::reader::Result as XmlResult;

struct Ink {
	//document_id: Option<String>,
	traces: Vec<Trace>,
	/*
	children: Vec<Box<dyn Node>>,
	definitions: Vec<Definition>,
	contexts: Vec<Context>,
	trace_groups: Vec<TraceGroup>,
	trace_views: Vec<TraceView>,
	annotations: Vec<Annotation>,
	annotations_xml: Vec<AnnotationXML>,
	*/
}

impl Ink {
    fn parse<R: Read>(er: &mut xml::reader::EventReader<R>) -> XmlResult<Self> {
        use XmlEvent::*;
        let mut traces = Vec::new();
        
        loop {
            match er.next()? {
                EndElement { name: OwnedName { local_name, .. } }
                if local_name == "ink" => break,
                
                StartElement { name: OwnedName { local_name, .. }, .. }
                if local_name == "trace" => {
                    let trace = Trace::parse(er)?;
                    traces.push(trace);
                }
                
                _ => unimplemented!()
            }
        }
        
        Ok(Ink { traces })
    }
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

fn open<R: Read>(source: R) -> XmlResult<Ink> {
    let mut reader = xml::EventReader::new(source);
    let mut ink = Ink { traces: Vec::new() };
    let mut path = Vec::new();
    
    loop {
        match reader.next()? {
            XmlEvent::StartDocument { .. } => {}
            XmlEvent::Whitespace(_) => {}
            XmlEvent::EndDocument => break,
            
            XmlEvent::StartElement { name: OwnedName { local_name, .. }, .. } => {
                path.push(local_name.clone());
            }
            
            XmlEvent::Characters(data) => {
                println!("got chars: {}", data);
                if path.last().unwrap() == "trace" {
                    match Trace::parse_content(&data) {
                        Ok(points) =>
                            ink.traces.push(Trace { content: points }),
                        Err(e) => panic!("{:?}", e),
                    }
                }
            }
            
            XmlEvent::EndElement { name: OwnedName { local_name, .. } } => {
                assert_eq!(path.pop().unwrap(), local_name);
            }
            
            evt => {
                println!("{:?}", evt);
                unimplemented!()
            }
        }
    }
    
    Ok(ink)
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn read_minimal() {
        let test: &[u8] = include_bytes!("minimal.ink");
        open(test).unwrap();
    }
    
    #[test]
    fn read_hello() {
        let test: &[u8] = include_bytes!("hello.ink");
        open(test).unwrap();
    }
}
