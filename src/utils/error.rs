use std::result;
use std::io;
use std::fmt;
use std::error;
use std::num;

use utils::jwt;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    CodedError(ErrorCode),
    TokenError(jwt::Error),
    ParseIntError(num::ParseIntError),
    Message(String)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<ErrorCode> for Error {
    fn from(err: ErrorCode) -> Error {
        Error::CodedError(err)
    }
}

impl From<jwt::Error> for Error {
    fn from(err: jwt::Error) -> Error {
        Error::TokenError(err)
    }
}


impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParseIntError(err)
    }
}


impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref inner) => inner.fmt(fmt),
            Error::CodedError(ref inner) => inner.fmt(fmt),
            Error::TokenError(ref inner) => inner.fmt(fmt),
            Error::ParseIntError(ref inner) => inner.fmt(fmt),
            Error::Message(ref inner) => inner.fmt(fmt)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref err) => err.description(),
            Error::CodedError(ref err) => err.to_str(),
            Error::TokenError(ref err) => err.description(),
            Error::ParseIntError(ref err) => err.description(),
            Error::Message(ref err) => err
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::CodedError(_) => None,
            Error::TokenError(ref err) => Some(err),
            Error::ParseIntError(ref err) => Some(err),
            Error::Message(_) => None
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Ord, PartialOrd)]
pub struct ErrorCode(pub u16);

impl ErrorCode {
    pub fn to_str(&self) -> &str {
        match self.0 {
            10004 => "资源不存在",
            10005 => "权限不足",
            20001 => "登录信息已过期",
            20002 => "用户名或密码错误",
            20003 => "用户已存在",
            30001 => "文章不存在",
            _ => "未知错误"
        }
    }

    pub fn to_code(&self) -> u16 {
        self.0
    }
}

impl From<i16> for ErrorCode {
    fn from(in_code: i16) -> ErrorCode {
        ErrorCode(in_code as u16)
    }
}

impl From<u16> for ErrorCode {
    fn from(in_code: u16) -> ErrorCode {
        ErrorCode(in_code)
    }
}

impl From<i32> for ErrorCode {
    fn from(in_code: i32) -> ErrorCode {
        ErrorCode(in_code as u16)
    }
}

impl From<u32> for ErrorCode {
    fn from(in_code: u32) -> ErrorCode {
        ErrorCode(in_code as u16)
    }

}

impl fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.to_str())
    }
}
