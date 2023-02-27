#include "graphics.h"
#include <SDL2/SDL.h>
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
	drawSurface = SDL_CreateRGBSurface(0, windowSurface->w, windowSurface->h, 24, 0xFF0000, 0x00FF00, 0x0000FF, 0);

	fullRect = malloc(sizeof(SDL_Rect));
	fullRect->x = 0;
	fullRect->y = 0;
	fullRect->w = windowSurface->w;
	fullRect->h = windowSurface->h;

	//SDL_Renderer* renderer = SDL_CreateRenderer(window, -1, 0);
	//if (!renderer) return 0;
	
	// Make fullscreen scale
	SDL_DisplayMode displayMode;
	SDL_GetCurrentDisplayMode(0, &displayMode);
	//SDL_RenderSetLogicalSize(renderer, displayMode.w, displayMode.h);

	fontLoad();

	//SDL_Texture* texture = SDL_CreateTexture(renderer, SDL_PIXELFORMAT_RGB888, SDL_TEXTUREACCESS_STATIC, 8, 12);



	drawChar(drawSurface, 49, 10, 10);
	graphicsRender();	

	return 1;
}
   
void graphicsRender() {
	// Swap buffers and clear
	SDL_BlitSurface(drawSurface, fullRect, windowSurface, fullRect); 
	SDL_UpdateWindowSurface(window);

	//SDL_RenderPresent(renderer);
	//SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
	//SDL_RenderClear(renderer);
}

void disposeGraphics() {
	//SDL_DestroyRenderer(renderer);
	free(fullRect);
    SDL_DestroyWindow(window);
    SDL_Quit();
}
