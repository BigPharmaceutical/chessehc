# Responses

The requests are split into an type (0th byte) and data (subsequent bytes).

## Type:

result (0):
- 0 - ok (1-2)
  - 0 - public (3-4)
    - 0 - status
      - *todo*
    - 1 - profile
      - *todo*
    - 2 - *unreserved*
    - 3 - [log in challenge](#log-in-challenge)
  - 1 - [Confirmation](#confirmation)
  - 2 - [game id](#game-id)
  - 3 - in-game (3)
    - 0 - game
      - *todo*
    - 1 - board
      - *todo*
- 1 - error (1-2)
  - 0 - *unreserved*
  - 1 - in-game
    - *todo*
  - 2 - invalid (3-4)
    - 0 - permissions (5)
      - 0 - log in
        - 0 - [not logged in](#not-logged-in)
        - 1 - [logged in](#logged-in)
      - 1 - [not game host](#not-game-host)
    - 1 - authentication (5-6)
      - 0 - log in (7)
        - 0 - [no challenge request](#no-challenge-request)
        - 1 - [log in failed](#log-in-failed)
      - 1 - [invalid public key](#invalid-public-key)
      - 2 - [invalid username](#invalid-username)
      - 3 - username use (7)
        - 0 - [unknown username](#unknown-username)
        - 1 - [username in use](#username-in-use)
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

### Confirmation

Type: `00100000`  
Data: op-code (1 byte)

### Game Id

Type: `01000000`  
Data: game id (bytes)

### Log in Challenge

Type: `00011000`  
Data: challenge (bytes)

### Not Logged In

Type: `11000000`

### Logged In

Type: `11000010`

### Not Game Host

Type: `11000100`

### No Challenge Request

Type: `11001000`

### Log In Failed

Type: `11001001`

### Invalid Public Key

Type: `11001010`

### Invalid Username

Type: `11001100`

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
