use super::Responder;

pub enum Ok {}

impl Responder for Ok {
    fn write(self, _buffer: &mut Vec<u8>) {
        todo!()
    }
}
