#ifndef H_INPUT
#define H_INPUT

#include <SDL2/SDL.h>

#define INPUT_FLAGS_ENABLED 1
#define INPUT_FLAGS_SELECTABLE 2
#define INPUT_FLAGS_SELECTABLE_WHEN_VISIBLE 4

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

struct InputDataProxy {
	void (*onKeyPress)(struct InputField* field, char key);
	void (*onDispose)(struct InputField* field);
	void* data;
};

struct InputDataTextfield {
	char* chars;
	unsigned char length;
};

void initInput();

struct InputField* getInputFocused();

void handleInput(char key);

void inputFixInvalidSelection();

struct InputField* createInputText(unsigned char length, char flags);

struct InputField* createInputButton(void (*onPress)(struct InputField*), char flags);

struct InputField* createInputProxy(void (*onKeyPress)(struct InputField* field, char key), void (*onDispose)(struct InputField* field), void* data, char flags);

void disposeInput();

void disposeInputField(struct InputField* target);

void disposeOneInputByField(struct InputField* target);

#endif
