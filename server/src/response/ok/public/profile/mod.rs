use crate::response::Responder;

use self::account::Account;

pub mod account;

pub enum Profile<'a> {
    Account(Account<'a>),
}

impl<'a> Responder for Profile<'a> {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match &self {
            Self::Account(_) => 0,
        } << 2;

        match self {
            Self::Account(res) => res.write(buffer),
        }
    }
}
