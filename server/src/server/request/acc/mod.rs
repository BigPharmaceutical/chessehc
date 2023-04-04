use async_trait::async_trait;

use crate::server::{
    handler::Client,
    response::{err::mal_req::MalformedRequest, Result},
};

use super::Requester;

pub enum Account {
    ChangeUsername,
    ChangeKey,
    Delete,
}

#[async_trait]
impl<'a> Requester<'a> for Account {
    fn parse(buffer: &[u8]) -> Result<Self> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 4) & 0b11 {
            0 => Self::ChangeUsername,
            2 => Self::ChangeKey,
            3 => Self::Delete,
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
