#ifndef H_CHESS
#define H_CHESS

#include "gui.h"


#define CHESSPIECE_TYPE_NULL 1
#define CHESSPIECE_TYPE_PAWN 1
#define CHESSPIECE_TYPE_TOWER 2
#define CHESSPIECE_TYPE_HORSE 3
#define CHESSPIECE_TYPE_BISHOP 4
#define CHESSPIECE_TYPE_QUEEN 5
#define CHESSPIECE_TYPE_KING 6

typedef struct ChessPiece {
	char type;
	struct ChessGamePlayer* owner;
} ChessPiece;

typedef struct ChessBoardRow {
	ChessPiece* pieces[8];
} ChessBoardRow;

typedef struct ChessBoard {
	short height;
	ChessBoardRow* rows;
	InputField* inputField;
} ChessBoard;

void initChess();

ChessBoard* createChessBoard(short height);

void drawChessBoard(ChessBoard* board, SDL_Surface* surface);

void disposeChessBoard(ChessBoard* board);


#endif
