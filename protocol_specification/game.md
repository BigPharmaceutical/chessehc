# Game Data

## Players

The list of players is a list of account Ids.

The game identifies players by their index (unsigned 8-bit integer) in this list.

## Pieces

Each piece is represented by two bytes: a player index and a piece Id.

### Piece Ids:
- 1 - Pawn
- 2 - Bishop
- 3 - Knight
- 4 - Rook
- 5 - Queen
- 6 - King

(0 is an empty spot, in this case, ignore player index byte)

## Board

The board will be 8 spots wide and 7 &times; [the number of players] tall.

The board is represented by two bytes for each spot. The spots will be ordered starting at `(0, 0)` and going through each row.

<details>

<summary>Example</summary>

|   | 0 | 1 | 2 |
|:-:|:-:|:-:|:-:|
| 0 | a | b | c |
| 1 | d | e | f |
| 2 | g | h | i |
| 3 | j | k | l |

This board would be listed in alphabetical order.

</details>
