#include "chess.h"
#include <stdlib.h>

ChessBoard* board;

void initChess() {
	


}

ChessBoard* createChessBoard(short height) {
	ChessBoard* board = malloc(sizeof(ChessBoard));
	board->height = height;
	return board;
}

void disposeChessBoard(ChessBoard* board) {
	free(board);
}
