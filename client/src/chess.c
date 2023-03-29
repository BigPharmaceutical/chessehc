#include "chess.h"
#include <stdlib.h>
#include "input.h"
#include "font.h"
#include "SDL2/SDL.h"
#include "graphics.h"


typedef struct ChessBoardInputData {
	ChessBoard* board;
	char selectX;
	unsigned short selectY;
} ChessBoardInputData;

void initChess() {

}

void boardInputKey(InputField* field, char key) {
	ChessBoardInputData* data = ((InputProxyData*)field->data)->data;

	switch (key) {
		case (SDLK_h):
		case (SDLK_a):
			data->selectX = (data->selectX + 7) % 8;
			break;
		case (SDLK_l):
		case (SDLK_d):
			data->selectX = (data->selectX + 9) % 8;
			break;
		case (SDLK_j):
		case (SDLK_s):
			data->selectY = (data->selectY + data->board->height + 1) % data->board->height;
			break;
		case (SDLK_k):
		case (SDLK_w):
			data->selectY = (data->selectY + data->board->height - 1) % data->board->height;
			break;
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

void drawChessBoard(ChessBoard* board, SDL_Surface* surface) {
	ChessBoardInputData* inputData = ((InputProxyData*)board->inputField->data)->data;
	
	SDL_Rect rect;
	rect.w = 48;
	rect.h = 48;

	for (short r = 0; r < 9; r++) {
		rect.y = r * 48 + 10;
		int rowIndex = (inputData->selectY - 4 + r + board->height) % board->height;
		for (unsigned char columnIndex = 0; columnIndex < 8; columnIndex++) {
			rect.x = columnIndex * 48 + 10;
			if ((inputData->selectY - 4 + r + columnIndex) % 2 == 0) {
				SDL_FillRect(surface, &rect, 0xFFFFFFFF);
			} else {
				SDL_FillRect(surface, &rect, 0xFF000000);
			}

			//todo make it not letters
			
			//ChessPiece piece = board->rows[rowIndex].pieces[columnIndex];
			ChessPiece* piece = board->rows[rowIndex].pieces[columnIndex];	
			if (!piece) {
				continue;
			}

			char dChar = piece->type + 96;
			drawChar(surface, dChar, &rect);



		}
		rect.x = 8 * 48 + 20;
		drawChar(surface, rowIndex + 48, &rect);
	}

	rect.x = inputData->selectX * 48 + 10;
	rect.y = 4 * 48 + 10;
	SDL_FillRect(surface, &rect, 0xFF00FF00);

	// Make everything COLORFUL
	graphicsDyeSurface(surface, 255, 0, 125); 
}

void disposeChessBoard(ChessBoard* board) {
	disposeOneInputByField(board->inputField);
	free(board);
}
