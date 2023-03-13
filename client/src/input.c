#include "input.h"
#include "util.h"
#include "main.h"
#include <stdlib.h>

LinkedList* inputFocused;
char inputNextid = 0;

LinkedList* inputFieldsHead;
LinkedList* inputFieldsTail;

void initInput() {

}

void handleInputSelected(InputField* field) {

}

void handleInput(SDL_Keysym key) {
	putc(key.sym, stdout);

	switch (key.sym) {
		case (SDLK_TAB): {
			LinkedList* dest;
 			if (!inputFocused) {
				if (inputFieldsHead) {
					dest = inputFieldsHead;
				} else {
					break;
				}
			} else {
				dest = inputFocused->next;
			}
			while (dest != inputFocused) {
				dest = dest->next;
				if (!dest) {
					dest = inputFieldsHead;
				}
				if (((InputField*)dest->value)->flags & INPUT_FLAGS_SELECTABLE) {
					break;
				}
			}
			handleInputSelected(dest->value);
			inputFocused = dest;
		} break;

		case (SDLK_ESCAPE): {
			doMainExit();
		}
	}
}

InputField* createInputText(unsigned char length, char flags) {
	InputField* field = malloc(sizeof(InputField));
	field->id = inputNextid++;
	field->flags = flags;
	field->type = INPUT_TYPE_TEXT;
	
	field->data = calloc(length + 1, sizeof(char));
	((char*) (field->data))[length] = '\0';
	
	inputFieldsTail = linkedListAppend(inputFieldsTail, field);
	if (!inputFieldsHead) {
		inputFieldsHead = inputFieldsTail;
		inputFocused = inputFieldsTail;
	}
	return field;
}


void disposeInput() {
	LinkedList* target = inputFieldsHead;
	while (target) {
		LinkedList* next = target->next;
		InputField* value = target->value;
		free(value->data);
		free(value);
		free(target);
		target = next;
	}
}
