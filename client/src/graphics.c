#include "graphics.h"
#include "gui.h"
#include "font.h"

SDL_Window* window;
SDL_Surface* windowSurface;

SDL_Surface* drawSurface;
SDL_Rect* fullRect;

int initGraphics() {
    if (SDL_Init(SDL_INIT_VIDEO) != 0) return 0;
	
	window = SDL_CreateWindow("Chessehc", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, WINDOW_WIDTH, WINDOW_HEIGHT, SDL_WINDOW_SHOWN);
	if (!window) return 0;

	windowSurface = SDL_GetWindowSurface(window);

	fullRect = malloc(sizeof(SDL_Rect));
	fullRect->x = 0;
	fullRect->y = 0;
	fullRect->w = windowSurface->w;
	fullRect->h = windowSurface->h;

	return 1;
}
   
void graphicsRender() {
	// Swap buffers and clear
	SDL_UpdateWindowSurface(window);
	SDL_FillRect(windowSurface, 0, 0);
}

void disposeGraphics() {
	free(fullRect);
	SDL_FreeSurface(drawSurface);

    SDL_DestroyWindow(window);
    SDL_Quit();
}
