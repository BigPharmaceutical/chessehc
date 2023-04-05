use crate::response::Responder;

pub enum Account<'a> {
    Username(&'a str),
    AccountId(i64),
}

impl<'a> Responder for Account<'a> {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match &self {
            Self::Username(_) => 0,
            Self::AccountId(_) => 1,
        } << 1;

        match self {
            Self::Username(username) => buffer.extend_from_slice(username.as_bytes()),
            Self::AccountId(account_id) => buffer.extend_from_slice(&account_id.to_be_bytes()),
        }
    }
}
