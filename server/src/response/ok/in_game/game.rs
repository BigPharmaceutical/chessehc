use crate::response::Responder;

pub enum Game {}

impl Responder for Game {
    fn write(self, buffer: &mut Vec<u8>) {
        todo!()
    }
}
