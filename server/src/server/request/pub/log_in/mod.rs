use async_trait::async_trait;
use ed25519_dalek::{VerifyingKey, PUBLIC_KEY_LENGTH, Verifier, Signature, SIGNATURE_LENGTH};
use rand::{thread_rng, RngCore};
use tokio::time::Instant;

use crate::{
    config::LOGIN_TIMEOUT_SECS,
    db::account::{self, get_username},
    server::{
        handler::Client,
        request::Requester,
        response::{
            err::{
                inval_req::{
                    auth::{
                        challenge::Challenge,
                        id::Identity,
                        Authentication,
                    },
                    InvalidRequest,
                },
                mal_req::{mal_bin::MalformedBinary, MalformedRequest},
                Error,
            },
            Result,
        },
    },
};

pub enum LogIn<'a> {
    RequestChallenge(i64),
    ChallengeResponse(&'a [u8]),
}

#[async_trait]
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

    async fn run<'b>(self, client: &'a mut Client<'b>) -> Result<()>
    where
        'b: 'a,
    {
        match self {
            Self::RequestChallenge(account_id) => make_challenge(account_id, client).await,
            Self::ChallengeResponse(signature_attempt) => attempt_challenge(signature_attempt, client).await,
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

    let mut challenge = [0; 32];
    thread_rng().fill_bytes(&mut challenge);

    client.log_in_challenge = Some((account_id, challenge, Instant::now()));

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

    if public_key.len() != 32 {
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
        return Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Challenge(Challenge::LogInFailed))));
    }

    client.log_in = Some(challenge.0);
    Ok(())
}
