use crate::response::{err::Error, Responder, Response};

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
    #[must_use]
    pub const fn op<'a>() -> Response<'a> {
        Response::Err(Error::MalReq(Self::Bin(MalformedBinary::Op)))
    }

    #[must_use]
    pub const fn op_err() -> Error {
        Error::MalReq(Self::Bin(MalformedBinary::Op))
    }

    #[must_use]
    pub const fn data<'a>() -> Response<'a> {
        Response::Err(Error::MalReq(Self::Bin(MalformedBinary::Data)))
    }

    #[must_use]
    pub const fn data_err() -> Error {
        Error::MalReq(Self::Bin(MalformedBinary::Data))
    }

    #[must_use]
    pub const fn b64<'a>() -> Response<'a> {
        Response::Err(Error::MalReq(Self::B64))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        response::{
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
