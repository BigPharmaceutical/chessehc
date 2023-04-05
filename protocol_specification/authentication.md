# Authentication

The authentication is based on key based signing

## Authentication Mechanism

→  
[`public / log in / request challenge`](./request.md#request-challenge)  
account id (i64)

←  
[`ok / public / log in challenge`](./response.md#log-in-challenge)  
challenge (32 bytes)

Sign the challenge using the private key associated with the account.

→  
[`public / log in / challenge response`](./request.md#challenge-response)  
challenge signature (64 bytes)

←  
[`ok / confirmation`](./response.md#confirmation)
