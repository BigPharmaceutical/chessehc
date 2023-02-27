#include <stdio.h>
#include "font.h"


SDL_Surface* charSurface;

void fontLoad() {
	FILE* file = fopen("font.bin", "r");
	unsigned char rawBuffer[24];
	unsigned char textureBuffer[288];
	charSurface = SDL_CreateRGBSurface(0, 8 * 128, 12, 24, 0xFF0000, 0x00FF00, 0x0000FF, 0);
	for (unsigned char charIndex = 0; charIndex < 128; charIndex++) {
		fread(rawBuffer, 24, 1, file);
		int pixel = 0;
		for (int pixel = 0; pixel < 96; pixel++) {
			char bufferValue = (rawBuffer[pixel / 4] >> (6 - 2 * (pixel % 4))) & 0b11;

			unsigned char brightness = 0;
			if (bufferValue == 1) {
				brightness = 85;
			} else if (bufferValue == 2) {
				brightness = 170;
			} else if (bufferValue == 3) {
				brightness = 255;
			}

			long byteOffset = (8 * charIndex + pixel % 8 + (pixel / 8) * 128) * 3;
			((char*) charSurface->pixels)[byteOffset] = brightness;
			((char*) charSurface->pixels)[byteOffset + 1] = brightness;
			((char*) charSurface->pixels)[byteOffset + 2] = brightness;
		}
	}
}


void drawChar(SDL_Surface* destination, char character, int x, int y) {
	SDL_Rect source;
	source.x = character * 8;
	source.y = 0;
	source.w = 64*8;
	source.h = 12;

	SDL_Rect target;
	target.x = x;
	target.y = y;
	target.w = 640;
	target.h = 60;
	SDL_BlitScaled(charSurface, &source, destination, &target);
}

