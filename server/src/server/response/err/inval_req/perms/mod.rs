use crate::server::response::Responder;

pub enum Permissions {
    NotLoggedIn,
    NotGameHost,
}

impl Responder for Permissions {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match self {
            Self::NotLoggedIn => 0,
            Self::NotGameHost => 1,
        } << 2;
    }
}

#[cfg(test)]
mod test {
    use crate::{
        server::response::{
            err::{inval_req::InvalidRequest, Error},
            Response,
        },
        test_type,
    };

    test_type!(
        test_type_not_logged_in,
        Response::Err(Error::InvalReq(InvalidRequest::Perm(
            super::Permissions::NotLoggedIn
        ))),
        0b1100_0000
    );

    test_type!(
        test_type_not_game_host,
        Response::Err(Error::InvalReq(InvalidRequest::Perm(
            super::Permissions::NotGameHost
        ))),
        0b1100_0100
    );
}
