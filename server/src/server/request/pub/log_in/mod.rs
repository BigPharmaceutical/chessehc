use crate::server::{
    request::Requester,
    response::err::{
        mal_req::{mal_bin::MalformedBinary, MalformedRequest},
        Error,
    },
};

pub enum LogIn<'a> {
    RequestChallenge(&'a str),
    ChallengeResponse(&'a [u8]),
}

impl<'a> Requester<'a> for LogIn<'a> {
    fn parse(buffer: &'a [u8]) -> Result<Self, Error> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 3) & 0b1 {
            0 => {
                let username = std::str::from_utf8(&buffer[1..])
                    .map_err(|_| Error::MalReq(MalformedRequest::Bin(MalformedBinary::Data)))?;
                Self::RequestChallenge(username)
            }
            1 => {
                let signature = &buffer[1..];
                Self::ChallengeResponse(signature)
            }
            _ => return Err(MalformedRequest::op_err()),
        })
    }

    fn run(self, _client: &mut crate::server::handler::Client) {
        todo!()
    }
}
