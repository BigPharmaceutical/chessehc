#include <stdio.h>
#include "font.h"
#include "fontblob.h"
#include "util.h"

const unsigned char brightnessValues[] = {0, 85, 170, 255};
SDL_Surface* charSurface;

void initFont() {
	// This needs to be one bigger for a null char
	charSurface = SDL_CreateRGBSurface(0, 8 * 128, 12, 32, 0xFF000000, 0x00FF0000, 0x0000FF00, 0x000000FF);
	//SDL_SetSurfaceBlendMode(charSurface, SDL_BLENDMODE_BLEND);

	for (unsigned char charIndex = 0; charIndex < 128; charIndex++) {
		for (unsigned int pixel = 0; pixel < 96; pixel++) {
			// Read brightness of pixel, mapped to [0,255] through lookup
			unsigned char brightness = brightnessValues[(FONTBLOB[charIndex * 24 + pixel / 4] >> (6 - 2 * (pixel % 4))) & 0b11];
			unsigned long pixelIndex = (8 * charIndex + (pixel % 8) + (pixel / 8) * charSurface->w);
			struct PixelARGB* pixels = charSurface->pixels;
			pixels[pixelIndex].r = brightness;
			pixels[pixelIndex].g = brightness;
			pixels[pixelIndex].b = brightness;
			pixels[pixelIndex].a = (brightness > 0) * 255;
		}
	}
}


void drawChar(SDL_Surface* destination, char character, SDL_Rect* target) {
	SDL_Rect charSource;
	charSource.x = character * 8;
	charSource.y = 0;
	charSource.w = 8;
	charSource.h = 12;
	SDL_BlitScaled(charSurface, &charSource, destination, target);
}

void drawString(SDL_Surface* destination, char* string, SDL_Rect* target, char spacing, unsigned char padLength) {
	SDL_Rect drawDest = *target;
	char* drawStr = string;
	while (*drawStr) {
		drawChar(destination, *drawStr, &drawDest);
		drawStr++;
		drawDest.x += drawDest.w + spacing;
	}
	if (padLength) {
		for (unsigned char i = padLength - (char)(drawStr - string); i > 0; i--) {
			drawChar(destination, ' ', &drawDest);
			drawDest.x += drawDest.w + spacing;
		}
	}
}

void disposeFont() {
	SDL_FreeSurface(charSurface);
}
