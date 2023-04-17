use crate::response::Responder;

pub enum Game {
    InvalidGameId,
    UnknownGameId,
    NotIn,
    In,
    Started,
    Full,
    InThis,
    TooFewPlayers,
}

impl Responder for Game {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match self {
            Self::InvalidGameId => 0,
            Self::UnknownGameId => 1,
            Self::NotIn => 2,
            Self::In => 3,
            Self::Started => 4,
            Self::Full => 5,
            Self::InThis => 6,
            Self::TooFewPlayers => 7,
        };
    }
}
