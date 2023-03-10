# Requests

The requests are split into an op-code (0th byte) and data (subsequent bytes).

## Op-code:

domain (0-1):
- 0 - public (2-3)
  - 0 - status
    - *todo*
  - 1 - profile
    - *todo*
  - 2 - [create account](#create-account)
  - 3 - log in (4)
    - 0 - [request challenge](#request-challenge)
    - 1 - [challenge response](#challenge-response)
- 1 - account (2-3)
  - 0 - [change username](#change-username)
  - 1 - *unreserved*
  - 2 - [change key](#change-key)
  - 3 - [delete](#delete-account)
- 2 - game (2)
    - 0 - [create](#create-game)
    - 1 - [join](#join-game)
- 3 - in-game (2-3)
    - 0 - game
      - *todo*
    - 1 - board
      - *todo*
    - 2 - manage
      - *todo*
    - 3 - [leave](#leave-game)

### Create Account

Op-code: `00100000`  
Data: username, *null*, public key  
[Response](./response.md#confirmation)  
[Error](./response.md#logged-in)  
[Error](./response.md#invalid-username)  
[Error](./response.md#username-in-use)  
[Error](./response.md#invalid-public-key)

### Request Challenge

Op-code: `00110000`  
Data: username (string)  
[Response](./response.md#log-in-challenge)  
[Error](./response.md#logged-in)  
[Error](./response.md#unknown-username)

### Challenge Response

Op-code: `00111000`  
Data: signed challenge (bytes)  
[Response](./response.md#confirmation)  
[Error](./response.md#no-challenge-request)  
[Error](./response.md#log-in-failed)

### Change Username

Op-code: `01000000`  
Data: new username (string)  
[Response](./response.md#confirmation)  
[Error](./response.md#not-logged-in)  
[Error](./response.md#invalid-username)  
[Error](./response.md#username-in-use)

### Change Key

Op-code: `01100000`  
Data: new public key (bytes)  
[Response](./response.md#confirmation)  
[Error](./response.md#not-logged-in)  
[Error](./response.md#invalid-public-key)

### Delete Account

Op-code: `01110000`  
[Response](./response.md#confirmation)  
[Error](./response.md#not-logged-in)

### Create Game

Op-code: `10000000`  
[Response](./response.md#game-id)  
[Error](./response.md#in-game)  
[Error](./response.md#not-logged-in)

### Join Game

Op-code: `10100000`  
Data: game id (string)  
[Response](./response.md#confirmation)  
[Error](./response.md#in-game)  
[Error](./response.md#not-logged-in)  
[Error](./response.md#invalid-game-id)  
[Error](./response.md#unknown-game-id)

### Leave Game

Op-code: `11110000`  
[Response](./response.md#confirmation)  
[Error](./response.md#not-in-game)
