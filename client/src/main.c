#include <SDL2/SDL.h>
#include <stdio.h>

#include "graphics.h"

int main( int argc, char** args){
	initGraphics();
	SDL_Delay(10000);
    
	disposeGraphics();
	return 1;
}
