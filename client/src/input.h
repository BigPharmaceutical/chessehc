#ifndef H_INPUT
#define H_INPUT

#include <SDL2/SDL.h>

#define INPUT_FLAGS_ENABLED 1
#define INPUT_FLAGS_SELECTABLE 2

#define INPUT_TYPE_TEXT 1
#define INPUT_TYPE_BUTTON 2
#define INPUT_TYPE_PROXY 3

struct InputField {
	unsigned char id;
	char flags;
	char type;
	void* data;
	char* guiElementFlags;
};

struct InputProxyData {
	void (*onKeyPress)(struct InputField* field, char key);
	void (*onDispose)(struct InputField* field);
	void* data;
};

void initInput();

void handleInput(char key);

struct InputField* createInputText(unsigned char length, char flags);

struct InputField* createInputButton(void (*onPress)(struct InputField*), char flags);

struct InputField* createInputProxy(void (*onKeyPress)(struct InputField* field, char key), void (*onDispose)(struct InputField* field), void* data, char flags);

void disposeInput();

void disposeInputField(struct InputField* target);

void disposeOneInputByField(struct InputField* target);

#endif
