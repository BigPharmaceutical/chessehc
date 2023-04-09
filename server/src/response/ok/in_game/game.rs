use crate::response::Responder;

pub enum Game {
    GameToken(String),
}

impl Responder for Game {
    fn write(self, _buffer: &mut Vec<u8>) {
        todo!()
    }
}
