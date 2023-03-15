#ifndef GRAPHICS_H
#define GRAPHICS_H

#include <SDL2/SDL.h>

#define WINDOW_WIDTH 640
#define WINDOW_HEIGHT 480

extern SDL_Surface* drawSurface;
extern SDL_Rect* fullRect;

int initGraphics();
void disposeGraphics();

void graphicsRender();
#endif
