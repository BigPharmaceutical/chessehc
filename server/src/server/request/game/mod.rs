use async_trait::async_trait;

use crate::server::{
    handler::Client,
    response::{
        err::mal_req::MalformedRequest,
        Result,
    },
};

use super::Requester;

pub enum Game {
    Create,
    Join,
}

#[async_trait]
impl<'a> Requester<'a> for Game {
    fn parse(buffer: &[u8]) -> Result<Self> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 5) & 0b1 {
            0 => Self::Create,
            1 => Self::Join,
            _ => return Err(MalformedRequest::op_err()),
        })
    }

    async fn run<'b>(self, _client: &'a mut Client<'b>) -> Result<()>
    where
        'b: 'a,
    {
        todo!()
    }
}
