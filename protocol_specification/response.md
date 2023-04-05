# Responses

The requests are split into an type (0th byte) and data (subsequent bytes).

## Type:

result (0):
- 0 - ok (1-2)
  - 0 - public (3-4)
    - 0 - status
      - *todo*
    - 1 - profile (5)
      - 0 - account (6)
        - 0 - [username](#username)
        - 1 - [account id](#account-id)
      - 1 - *todo*
    - 2 - *unreserved*
    - 3 - [log in challenge](#log-in-challenge)
  - 1 - [Confirmation](#confirmation)
  - 2 - [account]
  - 3 - in-game (3)
    - 0 - game (4-5)
      - 0 - [game token](#game-token)
      - *todo*
    - 1 - board
      - *todo*
- 1 - error (1-2)
  - 0 - [server](#server)
  - 1 - in-game
    - *todo*
  - 2 - invalid (3-4)
    - 0 - permissions (5)
      - 0 - log in
        - 0 - [not logged in](#not-logged-in)
        - 1 - [logged in](#logged-in)
      - 1 - [not game host](#not-game-host)
    - 1 - authentication (5)
      - 0 - challenge (6-7)
        - 0 - [no challenge request](#no-challenge-request)
        - 1 - [challenge timed out](#challenge-timed-out)
        - 2 - [log in failed](#log-in-failed)
        - 3 - [invalid public key](#invalid-public-key)
      - 1 - identity (6-7)
        - 0 - [unknown id](#unknown-id)
        - 1 - [invalid username](#invalid-username)
        - 2 - [unknown username](#unknown-username)
        - 3 - [username in use](#username-in-use)
    - 2 - game (5-6)
      - 0 - [invalid game id](#invalid-game-id)
      - 1 - [unknown game id](#unknown-game-id)
      - 2 - [not in game](#not-in-game)
      - 3 - [in game](#in-game)
    - 3 - *unreserved*
  - 3 - malformed (3)
    - 0 - binary (4)
      - 0 - [op-code](#malformed-op-code)
      - 1 - [data](#malformed-data)
    - 1 - [base64](#malformed-base64)

### Username

Type: `00001000`  
Data: username (string)

### Account Id

Type: `00001010`  
Data: account id (i64)

### Log in Challenge

Type: `00011000`  
Data: challenge (bytes)

### Confirmation

Type: `00100000`  
Data: op-code (1 byte)

### Game Token

Type: `01000000`  
Data: game token (string)

### Server

Type: `10000000`  

### Not Logged In

Type: `11000000`

### Logged In

Type: `11000010`

### Not Game Host

Type: `11000100`

### No Challenge Request

Type: `11001000`

### Challenge Timed Out

Type: `11001001`

### Log In Failed

Type: `11001010`

### Invalid Public Key

Type: `11001011`

### Unknown Id

Type: `11001100`

### Invalid Username

Type: `11001101`

### Unknown Username

Type: `11001110`

### Username in Use

Type: `11001111`

### Invalid Game Id

Type: `11010000`

### Unknown Game Id

Type: `11010010`

### Not in Game

Type: `11010100`

### In Game

Type: `11010110`

### Malformed Op-code

Type: `11100000`

### Malformed Data

Type: `11101000`

### Malformed Base64

Type: `11110000`
