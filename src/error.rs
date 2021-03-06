use std::fmt;
use std::error::Error as StdError;
use std::io::Error as IoError;
use mammut::Error as MammutError;
use toml::de::Error as TomlDeError;
use toml::ser::Error as TomlSerError;

pub type Result<T> =  std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	Io(IoError),
	Mammut(MammutError),
	TomlDe(TomlDeError),
	TomlSer(TomlSerError),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl StdError for Error {
	fn description(&self) -> &str {
		match *self {
			Error::Io(ref e) => e.description(),
			Error::Mammut(ref e) => e.description(),
			Error::TomlDe(ref e) => e.description(),
			Error::TomlSer(ref e) => e.description(),
		}
	}
}

impl From<IoError> for Error {
	fn from(error: IoError) -> Self {
		Error::Io(error)
	}
}

impl From<MammutError> for Error {
	fn from(error: MammutError) -> Self {
		Error::Mammut(error)
	}
}

impl From<TomlDeError> for Error {
	fn from(error: TomlDeError) -> Self {
		Error::TomlDe(error)
	}
}

impl From<TomlSerError> for Error {
	fn from(error: TomlSerError) -> Self {
		Error::TomlSer(error)
	}
}

