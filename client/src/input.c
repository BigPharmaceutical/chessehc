#include "input.h"
#include "gui.h"
#include "util.h"
#include "main.h"
#include "content.h"
#include <stdlib.h>

LinkedList* inputFocused = 0;
char inputNextid = 0;

LinkedList* inputFieldsHead = 0;
LinkedList* inputFieldsTail = 0;


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
		}
	} else {
		if (nextIndex < text->length) {
			text->chars[nextIndex] = key;
		}
	}
	if (field->guiElementFlags) {
		*field->guiElementFlags |= GUI_ELEMENT_FLAG_INVALIDATED;
	}
}

void handleInputButton(InputField* field, char key) {
	if (key != SDLK_SPACE && key != SDLK_RETURN) {
		return;
	}
	void (*func)(InputField*) = field->data;
	(*func)(field);
}

void handleInputProxy(InputField* field, char key) {
	InputProxyData* pData = field->data;
	pData->onKeyPress(field, key);
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
			InputField* focusedField = inputFocused->value;
			if (!focusedField) {
				break;
			}

			switch (focusedField->type) {
				case (INPUT_TYPE_TEXT): {
					handleInputText(focusedField, key);
				} break;
				
				case (INPUT_TYPE_BUTTON): {
					handleInputButton(focusedField, key);
				} break;

				case (INPUT_TYPE_PROXY): {
					handleInputProxy(focusedField, key);
				} break;
			}

			if (focusedField->guiElementFlags) {
				*focusedField->guiElementFlags |= GUI_ELEMENT_FLAG_INVALIDATED;
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

void inputLinkFlags(InputField* field, char* flagPtr) {
	field->guiElementFlags = flagPtr;
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


InputField* createInputProxy(void (*onKeyPress)(InputField* field, char key), void (*onDispose)(InputField* field), void* data, char flags) {
	InputField* field = createInputOfType(flags, INPUT_TYPE_PROXY);
	InputProxyData* pData = malloc(sizeof(InputProxyData));
	pData->onKeyPress = onKeyPress;
	pData->onDispose = onDispose;
	pData->data = data;
	field->data = pData;
	return field;
}	


void disposeInputProxy(InputField* target) {
	InputProxyData* pData = target->data;
	pData->onDispose(target);
}

void disposeInputField(InputField* target) {
	switch (target->type) {
		case (INPUT_TYPE_TEXT): {
			InputTextfieldData* data = target->data;
			free(data->chars);
			free(data);
		} break;

		case (INPUT_TYPE_BUTTON): {	
		} break;

		case (INPUT_TYPE_PROXY): {
			disposeInputProxy(target);
		} break;

	}

	inputFocused = inputFieldsHead;
	free(target);
}

void disposeOneInput(LinkedList* target) {
	if (inputFieldsHead == target) {
		// Don't need to change parent of first element
		inputFieldsHead = inputFieldsHead->next;
		if (!inputFieldsHead) {
			inputFieldsTail = 0;
		}
	} else {
		// We do for further ones though
		LinkedList* search = inputFieldsHead;
		while (search && search->next != target) {
			search = search->next;
		}
		if (search) {
			search->next = target->next;
			if (!target->next) {
				inputFieldsTail = search;
			}
		}
	}
	disposeInputField(target->value);
	free(target);
}

void disposeOneInputByField(InputField* field) {
	LinkedList* targetEntry;
	if (inputFieldsHead && inputFieldsHead->value == field) {
		targetEntry = inputFieldsHead;
		inputFieldsHead = targetEntry->next;
		if (!inputFieldsHead) {
			inputFieldsTail = 0;
		}
	} else {
		LinkedList* search = inputFieldsHead;
		while (search && search->next->value != field) {
			search = search -> next;
		}
		if (search) {
			targetEntry = search->next;
			search->next = targetEntry->next;
			if (!targetEntry->next) {
				inputFieldsTail = search;
			}
		}
	}
	free(targetEntry);
	disposeInputField(field);
}

void disposeInput() {
	LinkedList* target = inputFieldsHead;
	while (target) {
		LinkedList* next = target->next;
		disposeInputField(target->value);
		free(target);
		target = next;
	}
}
