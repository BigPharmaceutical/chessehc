use crate::response::Responder;

pub enum Status<'a> {
    Start(&'a [i64], &'a [(u8, u8)]),
    End,
}

impl<'a> Responder for Status<'a> {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match &self {
            Self::Start(..) => 0,
            Self::End => 1,
        } << 1;

        match self {
            Self::Start(players, board) => {
                buffer.reserve(1 + players.len() * 8 + 2 * 8 * players.len() * 7);

                buffer.push(u8::try_from(players.len()).expect("too many players in game"));
                buffer.extend(players.iter().flat_map(|id| id.to_be_bytes()));
                buffer.extend(board.iter().flat_map(|(player, id)| [player, id]));
            }
            Self::End => todo!(),
        }
    }
}
