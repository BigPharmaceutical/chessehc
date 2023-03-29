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

struct PixelRGB {
	unsigned char r;
	unsigned char g;
	unsigned char b;
};

/* Only use with 24-bit pixel surfaces (without alpha) */
void graphicsDyeSurface(SDL_Surface* surface, unsigned char r, unsigned char g, unsigned char b) {
	unsigned long max = surface->w * surface->h;
	struct PixelRGB* pixels = surface->pixels;
	for (unsigned long i = 0; i < max; i++) {
		pixels[i].r += (pixels[i].r & r) + ((pixels[i].r ^ r) >> 1);
		pixels[i].g += (pixels[i].g & g) + ((pixels[i].g ^ g) >> 1);
		pixels[i].b += (pixels[i].b & b) + ((pixels[i].b ^ b) >> 1);
	}
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
