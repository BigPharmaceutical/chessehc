use crate::server::response::Responder;

pub enum LogIn {
    NoChallengeRequest,
    LogInFailed,
}

impl Responder for LogIn {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match self {
            Self::NoChallengeRequest => 0,
            Self::LogInFailed => 1,
        }
    }
}
