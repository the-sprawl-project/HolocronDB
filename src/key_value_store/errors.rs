use std::fmt::{Display, Formatter, Result, Debug};

#[derive(Copy, Clone)]
pub enum ErrorKind {
    ErrorNone,
    FileOpenError,
    FileWriteError,
    FileReadError,
    DataDecodeError
}


pub struct RWError {
    pub kind_: ErrorKind,
    pub context_: String,
}

impl Display for RWError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match write!(
            f,
            "{}, context: {}",
            error_kind_to_str(self.kind_),
            self.context_
        ) {
            Ok(_) => {},
            Err(e) => { panic!("{}", e)}
        };

        Ok(())
    }
}

impl Debug for RWError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match write!(
            f,
            "Error! {{ kind: {}, context: {} }}",
            error_kind_to_str(self.kind_),
            self.context_
        ) {
            Ok(_) => {},
            Err(e) => { panic!("{}", e)}
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
        ErrorKind::FileOpenError => {
            ret = "Cannot open file"
        }
        ErrorKind::FileReadError => {
            ret = "Cannot read file"
        }
        ErrorKind::FileWriteError => {
            ret = "Cannot write to file"
        }
        ErrorKind::DataDecodeError => {
            ret = "Data decode error"
        }
    }
    return String::from(ret);
}
