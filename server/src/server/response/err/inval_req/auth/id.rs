use crate::server::response::Responder;

pub enum Identity {
    UnknownId,
    InvalidUsername,
    UnknownUsername,
    UsernameInUse,
}

impl Responder for Identity {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match self {
            Self::UnknownId => 0,
            Self::InvalidUsername => 1,
            Self::UnknownUsername => 2,
            Self::UsernameInUse => 3,
        }
    }
}
