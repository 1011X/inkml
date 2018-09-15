mod trace;
mod point;
mod value;
mod number;

pub use self::trace::Trace;
pub use self::point::Point;
pub use self::value::Value;


pub type ParseResult<'a, T> = Result<T, ParseError<'a>>;

#[derive(Debug)]
pub enum ParseError<'a> {
    EndOfFile,
    UnexpectedValue(&'a str),
}


// TODO: include Unicode whitespace code points?
// wsp ::= (#x20 | #x9 | #xD | #xA)
pub fn wsp(c: char) -> bool {
    ['\x20', '\x09', '\x0D', '\x0A'].contains(&c)
}

// digit ::= ("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
pub fn digit(c: char) -> bool {
    c.is_ascii_digit()
}

// hex ::= "#" (digit | "A" | "B" | "C" | "D" | "E" | "F")+
pub fn hex(mut input: &str) -> ParseResult<(&str, &str)> {
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
