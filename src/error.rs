use thiserror::Error;


#[derive(Error, Debug)]
pub enum IloxError{
    Syntax(SyntaxError)
}

impl std::fmt::Display for IloxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Syntax(s) => {
                writeln!(f, "Syntax error: {0} on line: {1}", s.msg, s.line)
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct SyntaxError{
    msg: String,
    line: u64
}



impl SyntaxError {
    pub(crate) fn new(msg: String, line: u64) -> Self {
        Self { msg, line }
    }
}
