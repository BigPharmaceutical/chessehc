use crate::response::Responder;

pub enum Game {
    InvalidGameId,
    UnknownGameId,
    NotIn,
    In,
}

impl Responder for Game {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match self {
            Self::InvalidGameId => 0,
            Self::UnknownGameId => 1,
            Self::NotIn => 2,
            Self::In => 3,
        } << 1;
    }
}
