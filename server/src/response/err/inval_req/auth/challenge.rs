use crate::response::Responder;

pub enum Challenge {
    NoRequest,
    TimedOut,
    LogInFailed,
    InvalidPublicKey,
}

impl Responder for Challenge {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match &self {
            Self::NoRequest => 0,
            Self::TimedOut => 1,
            Self::LogInFailed => 2,
            Self::InvalidPublicKey => 3,
        };
    }
}
