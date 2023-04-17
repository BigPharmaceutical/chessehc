use crate::response::Responder;

pub enum Players {
    Join(i64),
    Leave(i64),
    List(Vec<i64>),
}

impl Responder for Players {
    fn write(self, _buffer: &mut Vec<u8>) {
        todo!()
    }
}
