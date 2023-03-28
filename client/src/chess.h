#ifndef H_CHESS
#define H_CHESS

#include "gui.h"

typedef struct ChessBoardRow {
	unsigned char pieces[8];
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
