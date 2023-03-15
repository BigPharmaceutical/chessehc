#ifndef H_INPUT
#define H_INPUT

#include <SDL2/SDL.h>

#define INPUT_FLAGS_ENABLED 1
#define INPUT_FLAGS_SELECTABLE 2

#define INPUT_TYPE_TEXT 1



typedef struct InputField {
	char id;
	char flags;
	char type;
	void* data;
} InputField;

void initInput();

void handleInput(char key);

InputField* createInputText(unsigned char length, char flags);

void disposeInput();

void disposeOneInput(InputField* target);

#endif
