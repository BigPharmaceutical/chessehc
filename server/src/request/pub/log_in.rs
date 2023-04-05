use ed25519_dalek::{Signature, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};

use rand::{thread_rng, RngCore};
use tokio::time::Instant;

use crate::{
    config::{CHALLENGE_LENGTH, LOGIN_TIMEOUT_SECS},
    db::account::{self, get_username},
    request::{Requester, RequesterRunResult},
    response::{
        err::{
            inval_req::{
                auth::{challenge::Challenge, id::Identity, Authentication},
                InvalidRequest,
            },
            mal_req::{mal_bin::MalformedBinary, MalformedRequest},
            Error,
        },
        ok::{public::Public, Ok},
        Response, Result,
    },
    server::handler::Client,
};

const CHALLENGE_RESPONSE_OP_CODE: u8 = 0b0011_1000;

pub enum LogIn<'a> {
    RequestChallenge(i64),
    ChallengeResponse(&'a [u8]),
}

impl<'a> Requester<'a> for LogIn<'a> {
    fn parse(buffer: &'a [u8]) -> Result<Self> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 3) & 0b1 {
            0 => {
                if buffer.len() != 9 {
                    return Err(Error::MalReq(MalformedRequest::Bin(MalformedBinary::Data)));
                }

                let mut account_id_bytes = [0; 8];
                account_id_bytes.copy_from_slice(&buffer[1..]);
                let account_id = i64::from_be_bytes(account_id_bytes);

                Self::RequestChallenge(account_id)
            }
            1 => {
                let signature = &buffer[1..];
                Self::ChallengeResponse(signature)
            }
            _ => return Err(MalformedRequest::op_err()),
        })
    }

    fn run<'b>(self, client: &'a mut Client<'b>) -> RequesterRunResult<'a>
    where
        'b: 'a,
    {
        match self {
            Self::RequestChallenge(account_id) => Box::pin(make_challenge(account_id, client)),
            Self::ChallengeResponse(signature_attempt) => {
                Box::pin(attempt_challenge(signature_attempt, client))
            }
        }
    }
}

async fn make_challenge<'a>(account_id: i64, client: &mut Client<'a>) -> Result<()> {
    match get_username(account_id).await {
        Ok(Some(_)) => (),
        Ok(None) => {
            return Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Id(
                Identity::UnknownId,
            ))))
        }
        Err(_) => return Err(Error::Server),
    }

    let mut challenge = [0; CHALLENGE_LENGTH];
    thread_rng().fill_bytes(&mut challenge);

    client.log_in_challenge = Some((account_id, challenge, Instant::now()));

    client
        .send(Response::Ok(Ok::Public(Public::LogInChallenge(&challenge))).into())
        .await
        .ok();

    Ok(())
}

async fn attempt_challenge<'a>(signature_attempt: &[u8], client: &mut Client<'a>) -> Result<()> {
    let Some(challenge) = client.log_in_challenge.take() else {
        return Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Challenge(Challenge::NoRequest))));
    };

    if challenge.2.elapsed().as_secs() >= LOGIN_TIMEOUT_SECS {
        return Err(Error::InvalReq(InvalidRequest::Auth(
            Authentication::Challenge(Challenge::TimedOut),
        )));
    }

    if signature_attempt.len() != SIGNATURE_LENGTH {
        return Err(Error::MalReq(MalformedRequest::Bin(MalformedBinary::Data)));
    }

    let public_key = match account::get_public_key(challenge.0).await {
        Ok(Some(public_key)) => public_key,
        Ok(None) => {
            return Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Id(
                Identity::UnknownId,
            ))))
        }
        Err(err) => {
            eprintln!("Database Error: {err:?}");
            return Err(Error::Server);
        }
    };

    if public_key.len() != PUBLIC_KEY_LENGTH {
        eprintln!(
            "Error: User {} has a public key of the wrong length!",
            challenge.0
        );
        return Err(Error::Server);
    }

    let mut signature_bytes = [0; SIGNATURE_LENGTH];
    signature_bytes.copy_from_slice(signature_attempt);
    let signature = Signature::from_bytes(&signature_bytes);

    let mut verifying_key_bytes = [0; PUBLIC_KEY_LENGTH];
    verifying_key_bytes.copy_from_slice(&public_key);

    let Ok(verifying_key) = VerifyingKey::from_bytes(&verifying_key_bytes) else {
        eprintln!(
            "Error: User {} has an invalid public key!",
            challenge.0
        );
        return Err(Error::Server);
    };

    if verifying_key.verify(&challenge.1, &signature).is_err() {
        return Err(Error::InvalReq(InvalidRequest::Auth(
            Authentication::Challenge(Challenge::LogInFailed),
        )));
    }

    client.log_in = Some(challenge.0);

    client
        .send(Response::Ok(Ok::Confirmation(CHALLENGE_RESPONSE_OP_CODE)).into())
        .await
        .ok();

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::request::{
        r#pub::{
            log_in::{LogIn, CHALLENGE_RESPONSE_OP_CODE},
            Public,
        },
        Request, Requester,
    };

    #[test]
    fn test_challenge_response_op_code() {
        let request = &[
            CHALLENGE_RESPONSE_OP_CODE,
            0x49,
            0xd9,
            0x0a,
            0x67,
            0x83,
            0xd1,
            0x6c,
            0xe5,
            0xde,
            0x52,
            0x15,
            0x3c,
            0x47,
            0x4c,
            0x80,
            0x5f,
            0x13,
            0xf5,
            0x39,
            0xcf,
            0x98,
            0x43,
            0x05,
            0x55,
            0x2e,
            0xee,
            0x8c,
            0x09,
            0x1a,
            0x1a,
            0x44,
            0xf9,
            0x9b,
            0x15,
            0x5c,
            0x5f,
            0x2a,
            0x58,
            0x83,
            0xc5,
            0xfc,
            0x14,
            0x03,
            0xc1,
            0xcf,
            0xad,
            0xd6,
            0x31,
            0xc3,
            0xfe,
            0x20,
            0x4f,
            0x87,
            0x20,
            0x8a,
            0xf2,
            0xc8,
            0xbf,
            0x65,
            0x69,
            0x93,
            0x24,
            0x0f,
            0x0c,
        ];

        let challenge_response = Request::parse(request);
        assert!(
            matches!(
                challenge_response,
                Ok(Request::Pub(Public::LI(LogIn::ChallengeResponse(_))))
            ),
            "op-code {CHALLENGE_RESPONSE_OP_CODE:0>8b} is not the challenge response op-code"
        );
    }
}
