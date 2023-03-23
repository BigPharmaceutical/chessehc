use crate::server::response::Responder;

pub mod log_in;
pub mod uname_use;

use self::{log_in::LogIn, uname_use::UsernameUse};

pub enum Authentication {
    LI(LogIn),
    InvalidPublicKey,
    InvalidUsername,
    UnameUse(UsernameUse),
}

impl Responder for Authentication {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match &self {
            Self::LI(_) => 0,
            Self::InvalidPublicKey => 1,
            Self::InvalidUsername => 2,
            Self::UnameUse(_) => 3,
        } << 1;

        match self {
            Self::LI(err) => err.write(buffer),
            Self::UnameUse(err) => err.write(buffer),
            _ => (),
        }
    }
}
