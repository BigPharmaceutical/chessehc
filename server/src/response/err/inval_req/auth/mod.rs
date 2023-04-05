use crate::response::Responder;

pub mod challenge;
pub mod id;

use self::{challenge::Challenge, id::Identity};

pub enum Authentication {
    Challenge(Challenge),
    Id(Identity),
}

impl Responder for Authentication {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match &self {
            Self::Challenge(_) => 0,
            Self::Id(_) => 1,
        } << 2;

        match self {
            Self::Challenge(err) => err.write(buffer),
            Self::Id(err) => err.write(buffer),
        }
    }
}
