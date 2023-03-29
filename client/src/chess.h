#ifndef H_CHESS
#define H_CHESS

#include "gui.h"

#define CHESSPIECE_TYPE_NULL 0
#define CHESSPIECE_TYPE_PAWN 1
#define CHESSPIECE_TYPE_TOWER 2
#define CHESSPIECE_TYPE_HORSE 3
#define CHESSPIECE_TYPE_BISHOP 4
#define CHESSPIECE_TYPE_QUEEN 5
#define CHESSPIECE_TYPE_KING 6

struct ChessPiece {
	unsigned char type;
	struct ChessGamePlayer* owner;
};

struct ChessBoardRow {
	struct ChessPiece* pieces[8];
};

struct ChessBoard {
	unsigned short height;
	struct ChessBoardRow* rows;
	struct InputField* inputField;
};

void initChess();

struct ChessBoard* createChessBoard(unsigned short height);

void drawChessBoard(struct ChessBoard* board, SDL_Surface* surface);

void disposeChessBoard(struct ChessBoard* board);

#endif
