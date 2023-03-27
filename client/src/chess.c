#include "chess.h"
#include <stdlib.h>
#include "input.h"

typedef struct ChessBoardInputData {
	ChessBoard* board;
	char selectX;
	unsigned short selectY;
} ChessBoardInputData;


void initChess() {
	


}

void boardInputKey(InputField* field, char key) {
	ChessBoardInputData* data = ((InputProxyData*)field->data)->data;
	data->selectX += 1;
	if (data->selectX == 8) {
		data->selectX = 0;
		data->selectY = (data->selectY + 1) % data->board->height;
	}
}

void boardInputDispose(InputField* field) {

}

ChessBoard* createChessBoard(short height) {
	ChessBoard* board = malloc(sizeof(ChessBoard));
	board->height = height;
	board->rows = calloc(height, sizeof(ChessBoardRow));

	ChessBoardInputData* bData = malloc(sizeof(ChessBoardInputData));
	bData->board = board;
	bData->selectX = 0;
	bData->selectY = 0;

	board->inputField = createInputProxy(*boardInputKey, *boardInputDispose, bData, INPUT_FLAGS_ENABLED | INPUT_FLAGS_SELECTABLE);



	return board;
}

void drawChessBoard(ChessBoard* board, SDL_Surface* surface, int bottomPosition) {
	SDL_Rect rect;
	rect.w = 48;
	rect.h = 48;

	for (short r = 0; r < 8; r++) {
		rect.y = r * 48 + 10;
		int rowIndex = (bottomPosition + r) % board->height;
		for (char c = 0; c < 8; c++) {
			rect.x = c * 48 + 10;
			if ((bottomPosition + r + c) % 2 == 1) {
				SDL_FillRect(surface, &rect, 0xFFFFFFFF);
			} else {
				SDL_FillRect(surface, &rect, 0xFF000000);
			}
		}
	}

	ChessBoardInputData* inputData = ((InputProxyData*)board->inputField->data)->data;
	rect.x = inputData->selectX * 48 + 10;
	rect.y = inputData->selectY * 48 + 10;
	// note to self: it's ARGB, with 255 being opaque
	SDL_FillRect(surface, &rect, 0xFF00FF00);
}

void disposeChessBoard(ChessBoard* board) {
	disposeOneInputByField(board->inputField);
	free(board);
}
