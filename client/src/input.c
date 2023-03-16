#include "input.h"
#include "gui.h"
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
}

void handleInputText(InputField* field, char key) {
	InputTextfieldData* text = field->data;
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

void handleInputButton(InputField* field, char key) {
	if (key != SDLK_SPACE && key != SDLK_RETURN) {
		return;
	}
	void (*func)(InputField*) = field->data;
	(*func)(field);
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
				
				case (INPUT_TYPE_BUTTON): {
					handleInputButton(inputFocused->value, key);
				}
			}
		} break;
	}
}

InputField* createInputOfType(char flags, char type) {
	InputField* field = malloc(sizeof(InputField));
	field->id = inputNextid++;
	field->flags = flags;
	field->type = type;
	
	inputFieldsTail = linkedListAppend(inputFieldsTail, field);
	if (!inputFieldsHead) {
		inputFieldsHead = inputFieldsTail;
		inputFocused = inputFieldsTail;
	}
	
	return field;
}

InputField* createInputText(unsigned char length, char flags) {
	InputField* field = createInputOfType(flags, INPUT_TYPE_TEXT);
	InputTextfieldData* data = malloc(sizeof(InputTextfieldData));
	data->length = length;	
	data->chars = calloc(length + 1, sizeof(char));
	field->data = data;
	
	return field;
}

InputField* createInputButton(void (*onPress)(InputField*), char flags) {
	InputField* field = createInputOfType(flags, INPUT_TYPE_BUTTON);
	field->data = onPress;
	return field;
}

void disposeOneInput(InputField* target) {
	switch (target->type) {
		case (INPUT_TYPE_TEXT): {
			InputTextfieldData* data = target->data;
			free(data->chars);
			free(data);
		} break;

		case (INPUT_TYPE_BUTTON): {	
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
