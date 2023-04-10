#include "input.h"
#include "gui.h"
#include "util.h"
#include "main.h"
#include "content.h"
#include <stdlib.h>

struct LinkedList* inputFocused = 0;
char inputNextid = 0;

struct LinkedList* inputFieldsHead = 0;
struct LinkedList* inputFieldsTail = 0;


void initInput() {
}

struct InputField* getInputFocused() {
	return inputFocused->value;
}

void handleInputSelected(struct InputField* field) {
}

void handleInputUnselected(struct InputField* field) {
}

void handleInputText(struct InputField* field, char key) {
	struct InputDataTextfield* text = field->data;
	unsigned char nextIndex = 0;
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

void handleInputButton(struct InputField* field, char key) {
	if (key != SDLK_SPACE && key != SDLK_RETURN) {
		return;
	}
	void (*func)(struct InputField*) = field->data;
	(*func)(field);
}

void handleInputProxy(struct InputField* field, char key) {
	struct InputDataProxy* pData = field->data;
	pData->onKeyPress(field, key);
}

void handleInput(char key) {
	switch (key) {
		case (SDLK_TAB): {
			struct LinkedList* dest;
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
				if (((struct InputField*)dest->value)->flags & INPUT_FLAGS_SELECTABLE) {
					break;
				}
				dest = dest->next;
			}
	
			struct InputField* fieldPrev = inputFocused->value;
			struct InputField* fieldNext = dest->value;

			handleInputUnselected(fieldPrev);
			handleInputSelected(fieldNext);

			if (fieldPrev->guiElementFlags) {
				*fieldPrev->guiElementFlags |= GUI_ELEMENT_FLAG_INVALIDATED;
			}
			if (fieldNext->guiElementFlags) {
				*fieldNext->guiElementFlags |= GUI_ELEMENT_FLAG_INVALIDATED;
			}

			inputFocused = dest;
		} break;

		case (SDLK_ESCAPE): {
			doMainExit();
		} break;

		default: {
			if (!inputFocused) {
				break;
			}
			struct InputField* focusedField = inputFocused->value;

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

struct InputField* createInputOfType(char flags, char type) {
	struct InputField* field = malloc(sizeof(struct InputField));
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

void inputLinkFlags(struct InputField* field, char* flagPtr) {
	field->guiElementFlags = flagPtr;
}

struct InputField* createInputText(unsigned char length, char flags) {
	struct InputField* field = createInputOfType(flags, INPUT_TYPE_TEXT);
	struct InputDataTextfield* data = malloc(sizeof(struct InputDataTextfield));
	data->length = length;	
	data->chars = calloc(length + 1, sizeof(char));
	field->data = data;
	
	return field;
}

struct InputField* createInputButton(void (*onPress)(struct InputField*), char flags) {
	struct InputField* field = createInputOfType(flags, INPUT_TYPE_BUTTON);
	field->data = onPress;
	return field;
}


struct InputField* createInputProxy(void (*onKeyPress)(struct InputField* field, char key), void (*onDispose)(struct InputField* field), void* data, char flags) {
	struct InputField* field = createInputOfType(flags, INPUT_TYPE_PROXY);
	struct InputDataProxy* pData = malloc(sizeof(struct InputDataProxy));
	pData->onKeyPress = onKeyPress;
	pData->onDispose = onDispose;
	pData->data = data;
	field->data = pData;
	return field;
}	


void disposeInputProxy(struct InputField* target) {
	struct InputDataProxy* pData = target->data;
	pData->onDispose(target);
}

void disposeInputField(struct InputField* target) {
	switch (target->type) {
		case (INPUT_TYPE_TEXT): {
			struct InputDataTextfield* data = target->data;
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

void disposeOneInput(struct LinkedList* target) {
	if (inputFieldsHead == target) {
		// Don't need to change parent of first element
		inputFieldsHead = inputFieldsHead->next;
		if (!inputFieldsHead) {
			inputFieldsTail = 0;
		}
	} else {
		// We do for further ones though
		struct LinkedList* search = inputFieldsHead;
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

void disposeOneInputByField(struct InputField* field) {
	struct LinkedList* targetEntry = 0;
	if (inputFieldsHead && inputFieldsHead->value == field) {
		targetEntry = inputFieldsHead;
		inputFieldsHead = targetEntry->next;
		if (!inputFieldsHead) {
			inputFieldsTail = 0;
		}
	} else {
		struct LinkedList* search = inputFieldsHead;
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
	if (targetEntry) {
		free(targetEntry);
	}
	disposeInputField(field);
}

void disposeInput() {
	struct LinkedList* target = inputFieldsHead;
	while (target) {
		struct LinkedList* next = target->next;
		disposeInputField(target->value);
		free(target);
		target = next;
	}
}
