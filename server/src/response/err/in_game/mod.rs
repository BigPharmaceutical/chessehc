use crate::response::Responder;

pub enum InGame {}

impl Responder for InGame {
    fn write(self, _buffer: &mut Vec<u8>) {
        todo!()
    }
}
