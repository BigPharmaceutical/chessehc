#include "chess.h"
#include <stdlib.h>

ChessBoard* board;

void initChess() {
	


}

ChessBoard* createChessBoard(short height) {
	ChessBoard* board = malloc(sizeof(ChessBoard));
	board->height = height;
	board->rows = calloc(height, sizeof(ChessBoardRow));
	return board;
}

void drawChessBoard(ChessBoard* board, SDL_Surface* surface, int bottomPosition) {
	SDL_Rect rect;
	rect.w = 48;
	rect.h = 48;

	for (short r = 0; r < 8; r++) {
		rect.y = r * 48 + 10;
		for (char c = (bottomPosition + r) % 2; c < 8; c += 2) {
			int rowIndex = (bottomPosition + r) % board->height;

			rect.x = c * 48 + 10;
			SDL_FillRect(surface, &rect, 0xFFFFFFFF);
		}
	}
}

void disposeChessBoard(ChessBoard* board) {
	free(board);
}
