use tokio_tungstenite::tungstenite::Message;

pub mod err;
pub mod ok;

use self::{err::Error, ok::Ok};

pub type Result<T> = std::result::Result<T, Error>;

pub trait Responder {
    fn write(self, buffer: &mut Vec<u8>);
}

pub enum Response {
    Ok(Ok),
    Err(Error),
}

impl Responder for Response {
    fn write(self, buffer: &mut Vec<u8>) {
        buffer.push(
            match &self {
                Self::Ok(_) => 0,
                Self::Err(_) => 1,
            } << 7,
        );

        match self {
            Self::Ok(value) => value.write(buffer),
            Self::Err(err) => err.write(buffer),
        }
    }
}

impl From<Response> for Vec<u8> {
    fn from(value: Response) -> Self {
        let mut buffer = Self::with_capacity(1);
        value.write(&mut buffer);
        buffer
    }
}

impl From<Response> for Message {
    fn from(value: Response) -> Self {
        Self::Binary(value.into())
    }
}

#[cfg(test)]
mod test {
    #[allow(clippy::module_name_repetitions)]
    #[macro_export]
    macro_rules! test_type {
        ( $name:ident, $value:expr, $expected:expr ) => {
            #[test]
            fn $name() {
                let value = $value;
                let bytes: Vec<u8> = value.into();
                let op = bytes.first().expect("empty response buffer");
                assert_eq!(*op, $expected);
            }
        };
    }
}
