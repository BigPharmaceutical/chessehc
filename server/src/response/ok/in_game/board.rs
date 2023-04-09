use crate::response::Responder;

pub enum Board {}

impl Responder for Board {
    fn write(self, _buffer: &mut Vec<u8>) {
        todo!()
    }
}
