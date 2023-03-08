#include "input.h"
#include <stdlib.h>

void initInput() {

}

void handleInput() {



}

char inputFocused = 0;
char inputNextid = 1;
InputTextField* inputTextFields[INPUT_NUM_TEXTFIELD] = {0};

InputTextField* createInputText(unsigned char length) {
	InputTextField* field = malloc(sizeof(InputTextField));
	field->id = inputNextid++;
	field->text = calloc(length + 1, sizeof(char));
	inputTextFields[field->id] = field;
	return field;
}




void disposeInput() {
	for (char i = 0; i < INPUT_NUM_TEXTFIELD; i++) {
		if (!inputTextFields[i]) continue;
		free(inputTextFields[i]->text);
		free(inputTextFields[i]);
		inputTextFields[i] = 0;
	}
	inputFocused = 0;
	inputNextid = 1;
}
