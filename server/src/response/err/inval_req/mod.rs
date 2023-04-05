use crate::response::Responder;

pub mod auth;
pub mod game;
pub mod perms;

use self::{auth::Authentication, game::Game, perms::Permissions};

pub enum InvalidRequest {
    Perm(Permissions),
    Auth(Authentication),
    Game(Game),
}

impl Responder for InvalidRequest {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match &self {
            Self::Perm(_) => 0,
            Self::Auth(_) => 1,
            Self::Game(_) => 2,
        } << 3;

        match self {
            Self::Perm(err) => err.write(buffer),
            Self::Auth(err) => err.write(buffer),
            Self::Game(err) => err.write(buffer),
        }
    }
}
