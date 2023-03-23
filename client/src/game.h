#ifndef H_GAME
#define H_GAME

#define PLAYER_FLAG_ALIVE 1

#include "chess.h"
#include "util.h"

struct ChessGame;

typedef struct ChessGamePlayer {
	char* name;
	char status;
	struct ChessGame* chessGame;
	char color;
} ChessGamePlayer;

typedef struct ChessGame {
	LinkedList* players;
	GuiElement* guiProxy;
} ChessGame;

void initGame();

ChessGame* createGame(short numPlayers);
void disposeGame(ChessGame* game);

ChessGamePlayer* createChessGamePlayer(char* name, ChessGame* game);
void gameContainerLink(GuiElement* container, ChessGame* game);

void disposeChessGamePlayer(ChessGamePlayer* player);

#endif
