use crate::server::response::{err::Error, Responder, Response};

pub mod mal_bin;

use mal_bin::MalformedBinary;

pub enum MalformedRequest {
    Bin(MalformedBinary),
    B64,
}

impl Responder for MalformedRequest {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match self {
            Self::Bin(_) => 0,
            Self::B64 => 1,
        } << 4;

        if let Self::Bin(value) = self {
            value.write(buffer);
        }
    }
}

impl MalformedRequest {
    pub const fn op() -> Response {
        Response::Err(Error::MalReq(Self::Bin(MalformedBinary::Op)))
    }

    pub const fn op_err() -> Error {
        Error::MalReq(Self::Bin(MalformedBinary::Op))
    }

    pub const fn data() -> Response {
        Response::Err(Error::MalReq(Self::Bin(MalformedBinary::Data)))
    }

    pub const fn data_err() -> Error {
        Error::MalReq(Self::Bin(MalformedBinary::Data))
    }

    pub const fn b64() -> Response {
        Response::Err(Error::MalReq(Self::B64))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        server::response::{
            err::{mal_req::MalformedRequest, Error},
            Response,
        },
        test_type,
    };

    test_type!(
        test_type_malformed_b64,
        Response::Err(Error::MalReq(MalformedRequest::B64)),
        0b1111_0000
    );
}
