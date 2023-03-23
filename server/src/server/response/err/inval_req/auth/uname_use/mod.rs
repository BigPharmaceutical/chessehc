use crate::server::response::Responder;

pub enum UsernameUse {
    UnknownUsername,
    UsernameInUse,
}

impl Responder for UsernameUse {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match self {
            Self::UnknownUsername => 0,
            Self::UsernameInUse => 1,
        }
    }
}
