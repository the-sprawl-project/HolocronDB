use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ErrorKind {
    ErrorNone,
    ParseError
}

pub struct SocketError {
    pub kind_: ErrorKind,
    pub context_: String,
}

impl Display for SocketError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match write!(
            f,
            "{}, context: {}",
            error_kind_to_str(self.kind_),
            self.context_
        ) {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e)
            }
        };

        Ok(())
    }
}

impl Debug for SocketError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match write!(
            f,
            "Error! {{ kind: {}, context: {} }}",
            error_kind_to_str(self.kind_),
            self.context_
        ) {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e)
            }
        };

        Ok(())
    }
}

fn error_kind_to_str(ek: ErrorKind) -> String {
    let ret: &str;
    match ek {
        ErrorKind::ErrorNone => {
            ret = "";
        }
        ErrorKind::ParseError => ret = "Cannot parse payload",
    }
    return String::from(ret);
}
