#include "input.h"
#include "util.h"
#include "main.h"
#include "content.h"
#include <stdlib.h>

LinkedList* inputFocused;
char inputNextid = 0;

LinkedList* inputFieldsHead;
LinkedList* inputFieldsTail;


void initInput() {
}

void handleInputSelected(InputField* field) {
	switch (field->type) {
		case (INPUT_TYPE_TEXT): {
		}
	}
}

void handleInputText(InputField* field, char key) {
	InputTextData* text = field->data;
	char nextIndex = 0;
	while (text->chars[nextIndex]) {
		nextIndex++;
	}
	if (key == SDLK_BACKSPACE) {
		if (nextIndex > 0) {
			text->chars[nextIndex - 1] = 0;
			guiContainerInvalidate(currentContainer);
		}
	} else {
		if (nextIndex < text->length) {
			text->chars[nextIndex] = key;
			guiContainerInvalidate(currentContainer);
		}
	}
}

void handleInput(char key) {
	switch (key) {
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
				if (!dest) {
					dest = inputFieldsHead;
					if (dest == inputFocused) {
						break;
					}
				}
				if (((InputField*)dest->value)->flags & INPUT_FLAGS_SELECTABLE) {
					break;
				}
				dest = dest->next;
			}
			handleInputSelected(dest->value);
			inputFocused = dest;
		} break;

		case (SDLK_ESCAPE): {
			doMainExit();
		} break;

		default: {
			if (!inputFocused) {
				break;
			}

			switch (((InputField*)inputFocused->value)->type) {
				case (INPUT_TYPE_TEXT): {
					handleInputText(inputFocused->value, key);
				} break;

			}
		} break;
	}
}

InputField* createInputText(unsigned char length, char flags) {
	InputField* field = malloc(sizeof(InputField));
	field->id = inputNextid++;
	field->flags = flags;
	field->type = INPUT_TYPE_TEXT;

	InputTextData* data = malloc(sizeof(InputTextData));
	data->length = length;
	
	data->chars = calloc(length + 1, sizeof(char));
	field->data = data;
	
	inputFieldsTail = linkedListAppend(inputFieldsTail, field);
	if (!inputFieldsHead) {
		inputFieldsHead = inputFieldsTail;
		inputFocused = inputFieldsTail;
	}
	return field;
}

void disposeOneInput(InputField* target) {
	switch (target->type) {
		case (INPUT_TYPE_TEXT): {
			InputTextData* data = target->data;
			free(data->chars);
			free(data);
		} break;



	}
	if (inputFieldsHead->value == target) {
		inputFieldsHead = inputFieldsHead->next;
	}



	inputFocused = inputFieldsHead;
	free(target);
}

void disposeInput() {
	LinkedList* target = inputFieldsHead;
	while (target) {
		LinkedList* next = target->next;
		disposeOneInput(target->value);
		free(target);
		target = next;
	}
}
