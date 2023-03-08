#ifndef H_INPUT
#define H_INPUT

#define INPUT_NUM_TEXTFIELD 16
#define INPUT_FLAGS_ENABLED 1


typedef struct InputTextField {
	char id;
	char* text;
	char cursorPosition;
	char flags;
} InputTextField;


void initInput();

void handleInput();

InputTextField* createInputText(unsigned char length);

#endif
