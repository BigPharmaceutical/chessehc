#include <stdio.h>
#include "font.h"
#include "fontblob.h"

const char brightnessValues[] = {0, 85, 170, 255};
SDL_Surface* charSurface;

void initFont() {
	// This needs to be one bigger for a null char
	unsigned char textureBuffer[288];
	charSurface = SDL_CreateRGBSurface(0, 8 * 128, 12, 24, 0xFF0000, 0x00FF00, 0x0000FF, 0);
	
	for (unsigned char charIndex = 0; charIndex < 128; charIndex++) {
		// It appears that fread(...) dies if there isn't an extra \0 at the end for some systems.
		unsigned int pixel = 0;
		for (int pixel = 0; pixel < 96; pixel++) {
			// Read brightness of pixel, mapped to [0,255] through lookup
			unsigned char brightness = brightnessValues[(FONTBLOB[charIndex * 24 + pixel / 4] >> (6 - 2 * (pixel % 4))) & 0b11];
			// Base offset in pixel array of this pixel
			long byteOffset = 3 * (8 * charIndex + (pixel % 8) + (pixel / 8) * charSurface->w);
			unsigned char* pixels = charSurface->pixels;
			pixels[byteOffset] = brightness;
			pixels[byteOffset + 1] = brightness;
			pixels[byteOffset + 2] = brightness;
		}
	}
}

void drawChar(SDL_Surface* destination, char character, SDL_Rect* target) {
	SDL_Rect source;
	source.x = character * 8;
	source.y = 0;
	source.w = 8;
	source.h = 12;
	SDL_BlitScaled(charSurface, &source, destination, target);
}

void drawString(SDL_Surface* destination, char* string, SDL_Rect* target, char spacing, char padLength) {
	SDL_Rect drawDest = *target;
	char* drawStr = string;
	while (*drawStr) {
		drawChar(destination, *drawStr, &drawDest);
		drawStr++;
		drawDest.x += drawDest.w + spacing;
	}
	if (padLength) {
		for (char i = padLength - (char)(drawStr - string); i > 0; i--) {
			drawChar(destination, ' ', &drawDest);
			drawDest.x += drawDest.w + spacing;
		}
	}
}

void disposeFont() {
	SDL_FreeSurface(charSurface);
}
