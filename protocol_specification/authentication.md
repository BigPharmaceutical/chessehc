# Authentication

The authentication is based on key based signing

## Authentication Mechanism

→  
[`public / log in / request challenge`](./request.md#request-challenge)  
username (string)

←  
[`ok / public / log in challenge`](./response.md#log-in-challenge)  
challenge (bytes)

Sign the challenge using the private key associated with the username.

→  
[`public / log in / challenge response`](./request.md#challenge-response)  
signed challenge (bytes)

←  
[`ok / confirmation`](./response.md#confirmation)
