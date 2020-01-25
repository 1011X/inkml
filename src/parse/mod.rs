pub mod trace;

pub use self::trace::Trace;


pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    EndOfFile,
    UnexpectedValue,
}


// TODO: include Unicode whitespace code points?
// wsp ::= (#x20 | #x9 | #xD | #xA)
pub(crate) fn wsp(c: char) -> bool {
    ['\x20', '\x09', '\x0D', '\x0A'].contains(&c)
}

// digit ::= ("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
pub(crate) fn digit(c: char) -> bool {
    c.is_ascii_digit()
}

// hex ::= "#" (digit | "A" | "B" | "C" | "D" | "E" | "F")+
pub(crate) fn hex(mut input: &str) -> ParseResult<(&str, &str)> {
    let mut end = 0;
    let i = input;
    
    if input.is_empty() {
        return Err(ParseError::EndOfFile);
    }
    
    // "#"
    if !input.starts_with('#') {
        return Err(ParseError::UnexpectedValue);
    }
    input = &input[1..];
    end += 1;
    
    // (digit | "A" | "B" | "C" | "D" | "E" | "F")
    if !input.starts_with(|c: char| c.is_ascii_hexdigit()) {
        return Err(ParseError::UnexpectedValue);
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
