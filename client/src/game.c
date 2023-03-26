#include "game.h"
#include "graphics.h"
#include "gui.h"

void initGame() {


}

void gameGuiDispose(GuiProxyData* data) {
}

void gameGuiDraw(GuiElement* element, SDL_Surface* surface) {	
	ChessGame* game = ((GuiDataProxyType*) element->data)->data;
	drawChessBoard(game->board, surface, 0);
}	

ChessGame* createGame() {
	ChessGame* game = malloc(sizeof(ChessGame));
	game->players = 0;
	game->board = createChessBoard(32);

	GuiProxyData* proxyData = malloc(sizeof(GuiProxyData));
	proxyData->onCreate = 0;
	proxyData->onDispose = &gameGuiDispose;
	proxyData->onDraw = &gameGuiDraw;
	game->guiProxy = createGuiElement(*fullRect, 0, GUI_ELEMENT_TYPE_PROXY, proxyData);
	((GuiDataProxyType*) game->guiProxy->data)->data = game;

	return game;
}

void startGame(ChessGame* game) {
	

}

void gameContainerLink(GuiElement* container, ChessGame* game) {
	guiContainerLink(container, game->guiProxy);
}

void disposeGame(ChessGame* game) {
	free(game);
}

ChessGamePlayer* createChessGamePlayer(char* name, ChessGame* game) {
	ChessGamePlayer* player = malloc(sizeof(ChessGamePlayer));
	player->name = name;
	player->status = PLAYER_FLAG_ALIVE;
	player->chessGame = game;
	player->color = 0xFFFF0000;
	game->players = linkedListAppend(game->players, player);
	return player;
}

void disposeChessGamePlayer(ChessGamePlayer* player) {
	free(player);
}


