#ifndef H_INPUT
#define H_INPUT

#include <SDL2/SDL.h>

#define INPUT_FLAGS_ENABLED 1
#define INPUT_FLAGS_SELECTABLE 2

#define INPUT_TYPE_TEXT 1
#define INPUT_TYPE_BUTTON 2
#define INPUT_TYPE_PROXY 3

typedef struct InputField {
	char id;
	char flags;
	char type;
	void* data;
	char* guiElementFlags;
} InputField;

typedef struct InputProxyData {
	void (*onKeyPress)(InputField* field, char key);
	void (*onDispose)(InputField* field);
	void* data;
} InputProxyData;

void initInput();

void handleInput(char key);

InputField* createInputText(unsigned char length, char flags);

InputField* createInputButton(void (*onPress)(InputField*), char flags);

InputField* createInputProxy(void (*onKeyPress)(InputField* field, char key), void (*onDispose)(InputField* field), void* data, char flags);

void disposeInput();

void disposeInputField(InputField* target);

void disposeOneInputByField(InputField* target);

#endif
