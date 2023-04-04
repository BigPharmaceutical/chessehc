-- Database: chessehc


CREATE TABLE accounts (
    account_id bigint GENERATED ALWAYS AS IDENTITY,
    username VARCHAR ( 15 ) NOT NULL UNIQUE,
    public_key BYTEA NOT NULL,

    PRIMARY KEY ( account_id )
);


CREATE TABLE games (
    game_id bigint GENERATED ALWAYS AS IDENTITY,

    PRIMARY KEY ( game_id )
);


CREATE TABLE players (
    game_id bigint NOT NULL,
    player_id bigint NOT NULL,
    points int NOT NULL,

    PRIMARY KEY ( game_id, player_id ),

    CONSTRAINT fk_game
        FOREIGN KEY ( game_id ) REFERENCES games ( game_id ),
    -- player_id does not have to be a foreign key, as accounts can be deleted
);
