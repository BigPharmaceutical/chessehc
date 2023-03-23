#include "game.h"
#include "graphics.h"
#include "gui.h"

void initGame() {


}

void gameGuiDispose(GuiProxyData* data) {
}

ChessGame* gameGuiPassDataData;
void* gameGuiPassData(GuiProxyData* data) {
	return gameGuiPassDataData;
}

void gameGuiDraw(GuiElement* element, SDL_Surface* surface) {
	SDL_Rect target;
	target.x = 10;
	target.y = 10;
	target.w = 20;
	target.h = 400;
	SDL_FillRect(surface, &target, 0xFFFFFF00);
}	

ChessGame* createGame(short numPlayers) {
	ChessGame* game = malloc(sizeof(ChessGame));
	game->players = 0;

	GuiProxyData* proxyData = malloc(sizeof(GuiProxyData));
	proxyData->onCreate = &gameGuiPassData;
	proxyData->onDispose = &gameGuiDispose;
	proxyData->onDraw = &gameGuiDraw;
	gameGuiPassDataData = game;
	game->guiProxy = createGuiElement(*fullRect, 0, GUI_ELEMENT_TYPE_PROXY, proxyData);


	return game;
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
	player->color = 0xFF;
	game->players = linkedListAppend(game->players, player);
	return player;
}

void disposeChessGamePlayer(ChessGamePlayer* player) {
	free(player);
}


