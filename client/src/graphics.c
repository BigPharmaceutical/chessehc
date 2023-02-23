#include "graphics.h"
#include <SDL2/SDL.h>

SDL_Window* window;
SDL_Renderer* renderer;

int initGraphics() {
    if (SDL_Init(SDL_INIT_VIDEO) != 0) return 0;
	
	window = SDL_CreateWindow("Chessehc", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, WINDOW_WIDTH, WINDOW_HEIGHT, SDL_WINDOW_SHOWN);
	if (!window) return 0;

	renderer = SDL_CreateRenderer(window, -1, 0);
	if (!renderer) return 0;
	
	// Make fullscreen scale
	SDL_DisplayMode displayMode;
	SDL_GetCurrentDisplayMode(0, &displayMode);
	SDL_RenderSetLogicalSize(renderer, displayMode.w, displayMode.h);

	// todo remove
	graphicsRender();
	graphicsRender();	
	
	return 1;
}
   
void graphicsRender() {
	// Swap buffers and clear
	SDL_RenderPresent(renderer);
	SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
	SDL_RenderClear(renderer);
}

void disposeGraphics() {
	SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();
}
