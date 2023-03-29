#ifndef GRAPHICS_H
#define GRAPHICS_H

#include <SDL2/SDL.h>

#define WINDOW_WIDTH 640
#define WINDOW_HEIGHT 480

extern SDL_Surface* windowSurface;
extern SDL_Rect* fullRect;

int initGraphics();
void disposeGraphics();

void graphicsRender();

void graphicsDyeSurface(SDL_Surface* surface, unsigned char r, unsigned char g, unsigned char b);

#endif
