pub mod xas;

#[macro_use]
mod macros;

pub use self::error::Error;
pub use self::reader::Reader;

#[derive(Debug)]
pub enum Mode {
    Xas,
    Xmcd,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Xas => write!(f, "xas"),
            Self::Xmcd => write!(f, "xmcd"),
        }
    }
}

impl std::str::FromStr for Mode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = match s {
            "xas" | "XAS" | "Xas" => Mode::Xas,
            "xmcd" | "XMCD" | "Xmcd" => Mode::Xmcd,
            _ => bail!("Incorrect mode"),
        };
        Ok(s)
    }
}

mod error {
    use std::{error, fmt, io};

    #[derive(Debug)]
    pub enum Error {
        Custom(String),
        Io(io::Error),
        Parse(std::num::ParseIntError),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Custom(s) => write!(f, "{}", s),
                Self::Io(s) => write!(f, "{}", s),
                Self::Parse(s) => write!(f, "{}", s),
            }
        }
    }

    impl From<std::io::Error> for Error {
        fn from(e: std::io::Error) -> Self {
            Self::Io(e)
        }
    }

    impl From<std::num::ParseIntError> for Error {
        fn from(e: std::num::ParseIntError) -> Self {
            Self::Parse(e)
        }
    }

    impl error::Error for Error {}
}
mod reader {
    use std::fs;
    use std::io;
    pub enum Reader<'a> {
        File(io::BufReader<fs::File>),
        Stdin(io::StdinLock<'a>),
    }

    impl<'a> io::Read for Reader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            match self {
                Self::File(file) => file.read(buf),
                Self::Stdin(guard) => guard.read(buf),
            }
        }
    }

    impl<'a> io::BufRead for Reader<'a> {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            match self {
                Self::File(reader) => reader.fill_buf(),
                Self::Stdin(guard) => guard.fill_buf(),
            }
        }
        fn consume(&mut self, amt: usize) {
            match self {
                Self::File(reader) => reader.consume(amt),
                Self::Stdin(guard) => guard.consume(amt),
            }
        }
    }
}
