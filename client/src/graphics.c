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
	SDL_SetSurfaceBlendMode(windowSurface, SDL_BLENDMODE_BLEND);

	fullRect = malloc(sizeof(SDL_Rect));
	fullRect->x = 0;
	fullRect->y = 0;
	fullRect->w = windowSurface->w;
	fullRect->h = windowSurface->h;

	return 1;
}
void graphicsDyeSurface(SDL_Surface* surface, struct PixelRGB* color) {
	unsigned long max = surface->w * surface->h;
	struct PixelARGB* pixels = surface->pixels;
	for (unsigned long i = 0; i < max; i++) {
		// average bit manipulation
		pixels[i].r += (pixels[i].r & color->r) + ((pixels[i].r ^ color->r) >> 1);
		pixels[i].g += (pixels[i].g & color->g) + ((pixels[i].g ^ color->g) >> 1);
		pixels[i].b += (pixels[i].b & color->b) + ((pixels[i].b ^ color->b) >> 1);
	}
}

void graphicsDrawRectOutline(SDL_Surface* surface, SDL_Rect* innerArea, short thickness, int color) {
	SDL_Rect outer;
	outer.x = innerArea->x - thickness;
	outer.y = innerArea->y - thickness;
	outer.w = innerArea->w + 2 * thickness;
	outer.h = innerArea->h + 2 * thickness;
	SDL_FillRect(surface, &outer, color);
	SDL_FillRect(surface, innerArea, 0x00000000);
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
