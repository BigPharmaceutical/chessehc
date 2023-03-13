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
#include "input.h"

char mainExit = 0;

int main(int argc, char** args){
	initGraphics();
	initInput();

	while (!mainExit) {
		SDL_Event event;
		while (SDL_PollEvent(&event)) {
			switch (event.type) {
				case (SDL_KEYDOWN):
					handleInput(event.key.keysym);	
					break;
				
				case (SDL_QUIT):
					mainExit = 1;
					break;

				default:
					break;
			}
		}
	}

	disposeGraphics();
	disposeInput();
	return 1;
}

void doMainExit() {
	mainExit = 1;
}
