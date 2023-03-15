#ifndef FONT_H
#define FONT_H

#include <SDL2/SDL.h>

void initFont();

void drawChar(SDL_Surface* destination, char character, SDL_Rect* target);

void drawString(SDL_Surface* destination, char* string, SDL_Rect* target, char spacing, char padLength);

void disposeFont();

#endif
