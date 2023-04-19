use std::borrow::Cow;

use crate::{
    game,
    response::{
        err::{
            inval_req::{self, perms::Permissions, InvalidRequest},
            mal_req::MalformedRequest,
            Error,
        },
        ok::{in_game, Ok},
        Response, Result,
    },
    server::handler::Client,
};

use super::{Requester, RequesterRunResult};

const JOIN_GAME_OP_CODE: u8 = 0b1010_0000;

pub enum Game<'a> {
    Create,
    Join(Cow<'a, str>),
}

impl<'a> Requester<'a> for Game<'a> {
    fn parse(buffer: &'a [u8]) -> Result<Self> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 5) & 0b1 {
            0 => Self::Create,
            1 => {
                let code = String::from_utf8_lossy(&buffer[1..]);
                Self::Join(code)
            }
            _ => unreachable!(),
        })
    }

    fn run<'b>(self, client: &'a mut Client<'b>) -> RequesterRunResult<'a>
    where
        'b: 'a,
    {
        match self {
            Self::Create => Box::pin(create_game(client)),
            Self::Join(code) => Box::pin(join_game(client, code)),
        }
    }
}

async fn create_game<'a>(client: &mut Client<'a>) -> Result<()> {
    let Some(account_id) = client.log_in.take() else {
        return Err(Error::InvalReq(InvalidRequest::Perm(Permissions::NotLoggedIn)));
    };

    let Ok(game) = game::create(account_id, client.game_handle.0.clone()) else {
        return Err(Error::Server);
    };

    client.game = (Some(game.1), Some(game.2));

    client
        .send(
            Response::Ok(Ok::InGame(in_game::InGame::Game(
                in_game::game::Game::Code(game.0),
            )))
            .into(),
        )
        .await
        .ok();

    Ok(())
}

async fn join_game<'a, 'b>(client: &mut Client<'a>, code: Cow<'b, str>) -> Result<()> {
    let Some(account_id) = client.log_in.take() else {
        return Err(Error::InvalReq(InvalidRequest::Perm(Permissions::NotLoggedIn)));
    };

    let token = game::code_to_token(code.as_bytes())
        .map_err(|_| Error::InvalReq(InvalidRequest::Game(inval_req::game::Game::InvalidGameId)))?;

    let game_sender = game::get(token).ok_or(Error::InvalReq(inval_req::InvalidRequest::Game(
        inval_req::game::Game::UnknownGameId,
    )))?;

    game_sender
        .send(game::PlayerMessage::Join(
            account_id,
            client.game_handle.0.clone(),
        ))
        .await
        .map_err(|_| Error::Server)?;

    client.game.1 = Some(game_sender);

    client
        .send(Response::Ok(Ok::Confirmation(JOIN_GAME_OP_CODE)).into())
        .await
        .ok();

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::request::{game::Game, Request, Requester};

    use super::JOIN_GAME_OP_CODE;

    #[test]
    fn test_join_game_op_code() {
        let request = [JOIN_GAME_OP_CODE];

        let join_game = Request::parse(&request);

        assert!(
            matches!(join_game, Ok(Request::Game(Game::Join(_)))),
            "op-code {JOIN_GAME_OP_CODE:0>8b} is not the join game op-code"
        );
    }
}
