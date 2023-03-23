use crate::server::response::Responder;

pub enum MalformedBinary {
    Op,
    Data,
}

impl Responder for MalformedBinary {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match self {
            Self::Op => 0,
            Self::Data => 1,
        } << 3;
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
        test_type_malformed_op_code,
        Response::Err(Error::MalReq(MalformedRequest::Bin(
            super::MalformedBinary::Op
        ))),
        0b1110_0000
    );

    test_type!(
        test_type_malformed_data,
        Response::Err(Error::MalReq(MalformedRequest::Bin(
            super::MalformedBinary::Data
        ))),
        0b1110_1000
    );
}
