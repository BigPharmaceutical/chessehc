#ifndef H_GAME
#define H_GAME

#define PLAYER_FLAG_ALIVE 1

#include "chess.h"
#include "util.h"

struct ChessGamePlayer {
	char* name;
	unsigned char status;
	struct ChessGame* chessGame;
	unsigned int color;
};

struct ChessGame {
	struct LinkedList* players;
	struct GuiElement* guiProxy;
	struct ChessBoard* board;
};

void initGame();

struct ChessGame* createGame();
void disposeGame(struct ChessGame* game);

struct ChessGamePlayer* createChessGamePlayer(char* name, struct ChessGame* game);
void gameContainerLink(struct GuiElement* container, struct ChessGame* game);

void disposeChessGamePlayer(struct ChessGamePlayer* player);

#endif
