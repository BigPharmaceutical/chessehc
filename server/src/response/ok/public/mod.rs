use crate::{config::CHALLENGE_LENGTH, response::Responder};

use self::profile::Profile;

pub mod profile;

pub enum Public<'a> {
    Status,
    Profile(Profile<'a>),
    LogInChallenge(&'a [u8; CHALLENGE_LENGTH]),
}

impl<'a> Responder for Public<'a> {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match &self {
            Self::Status => 0,
            Self::Profile(_) => 1,
            Self::LogInChallenge(_) => 3,
        } << 3;

        match self {
            Self::Status => (),
            Self::Profile(res) => res.write(buffer),
            Self::LogInChallenge(challenge) => buffer.extend_from_slice(challenge),
        }
    }
}
