#ifndef H_CHESS
#define H_CHESS

#include "gui.h"

typedef struct ChessBoardRow {
	unsigned char pieces[8];
} ChessBoardRow;

typedef struct ChessBoard {
	short height;
	GuiElement* guiElement;
	ChessBoardRow* rows;
} ChessBoard;

void initChess();

ChessBoard* createChessBoard(short height);

void drawChessBoard(ChessBoard* board, SDL_Surface* surface, int bottomPosition);

void disposeChessBoard(ChessBoard* board);


#endif
