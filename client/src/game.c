#include "game.h"
#include "graphics.h"
#include "gui.h"
#include "input.h"

void initGame() {


}

void gameGuiDispose(struct GuiProxyData* data) {
}

void gameGuiDraw(struct GuiElement* element, SDL_Surface* surface) {	
	struct ChessGame* game = ((struct GuiDataProxyType*) element->data)->data;
	drawChessBoard(game->board, surface);
}	

struct ChessGame* createGame() {
	struct ChessGame* game = malloc(sizeof(struct ChessGame));
	game->players = 0;
	game->board = createChessBoard(32);

	struct GuiProxyData* proxyData = malloc(sizeof(struct GuiProxyData));
	proxyData->onCreate = 0;
	proxyData->onDispose = &gameGuiDispose;
	proxyData->onDraw = &gameGuiDraw;
	game->guiProxy = createGuiElement(*fullRect, 0, GUI_ELEMENT_TYPE_PROXY, proxyData);
	((struct GuiDataProxyType*) game->guiProxy->data)->data = game;
	game->board->inputField->guiElementFlags = &game->guiProxy->flags;
	
	return game;
}

void startGame(struct ChessGame* game) {
	

}

void gameContainerLink(struct GuiElement* container, struct ChessGame* game) {
	guiContainerLink(container, game->guiProxy);
}

void disposeGame(struct ChessGame* game) {
	free(game);
}

struct ChessGamePlayer* createChessGamePlayer(char* name, struct ChessGame* game) {
	struct ChessGamePlayer* player = malloc(sizeof(struct ChessGamePlayer));
	player->name = name;
	player->status = PLAYER_FLAG_ALIVE;
	player->chessGame = game;
	player->color = 0xFFFF0000;
	game->players = linkedListAppend(game->players, player);
	return player;
}

void disposeChessGamePlayer(struct ChessGamePlayer* player) {
	free(player);
}


