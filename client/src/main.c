/*
	Chessehc Client
	Copyright (C) 2023  BigPharmaceutical

	This program is free software: you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation, either version 3 of the License, or
	(at your option) any later version.

	This program is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

#include "main.h"
#include <SDL2/SDL.h>
#include <stdio.h>

#include "graphics.h"
#include "font.h"
#include "input.h"
#include "content.h"
#include "gui.h"

char mainExit = 0;

void querty(){
	mainExit = mainExit | 0;
}

int main(int argc, char** args){
	initGraphics();
	initFont();
	initInput();
	initContent();
	querty();
	while (!mainExit) {
		SDL_Event event;
		while (SDL_PollEvent(&event)) {
			switch (event.type) {
				case (SDL_KEYDOWN):
					// We still want to handle control characters
					if (event.key.keysym.sym < 32) {
						handleInput(event.key.keysym.sym);
					}	
					break;
				
				case (SDL_QUIT):
					mainExit = 1;
					break;

				case (SDL_TEXTINPUT):
					handleInput(event.text.text[0]);	
					break;

				default:
					break;
			}
		}
		drawGuiElement(currentContainer, windowSurface);
		//SDL_BlitSurface(guiContainerSurface(currentContainer), fullRect, drawSurface, fullRect); 
		graphicsRender();
		querty();
	}

	disposeGraphics();
	disposeFont();
	disposeInput();
	disposeContent();
	return 1;
}

void doMainExit() {
	mainExit = 1;
}
