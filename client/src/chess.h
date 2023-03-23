#ifndef H_CHESS
#define H_CHESS

#include "gui.h"

typedef struct ChessBoard {
	short height;
	GuiElement* guiElement;
} ChessBoard;

void initChess();

ChessBoard* createChessBoard(short height);

void disposeChessBoard(ChessBoard* board);


#endif
