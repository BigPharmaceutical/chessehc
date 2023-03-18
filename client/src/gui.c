#include "gui.h"
#include "util.h"
#include "font.h"
#include "input.h"
#include "graphics.h"
#include <stdlib.h>
#include <SDL2/SDL.h>

typedef struct GuiDataContainerType {
	LinkedList* children;
	SDL_Surface* surface;
	char invalidated;
} GuiDataContainerType;
GuiDataContainerType* createGuiDataContainer(SDL_Rect* param);
void disposeGuiDataContainer(GuiDataContainerType* data);

void* createGuiDataText(char* param);
void disposeGuiDataText(void* data);
void drawGuiElementText(GuiElement* text, SDL_Surface* surface);

void* createGuiDataTextfield(char param);
void disposeGuiDataTextfield(void* data);
void drawGuiElementTextfield(GuiElement* text, SDL_Surface* surface);

typedef struct GuiDataButtonType {
	char* text;
	InputField* inputField;
} GuiDataButtonType;
GuiDataButtonType* createGuiDataButton(InputButtonData* param);
void disposeGuiDataButton(GuiDataButtonType* data);
void drawGuiElementButton(GuiElement* element, SDL_Surface* surface);

typedef struct GuiDataProxyType {
	GuiProxyData* proxy;
	void* data;
} GuiDataProxyType;
GuiDataProxyType* createGuiDataProxy(GuiProxyData* param);
void disposeGuiDataProxy(GuiDataProxyType* data);
void drawGuiElementProxy(GuiElement* element, SDL_Surface* surface);


//////// General ////////

GuiElement* createGuiElement(SDL_Rect area, char flags, char type, void* param) {
	GuiElement* element = malloc(sizeof(GuiElement));
	element->position = area;
	element->flags = flags;
	element->type = type;

	switch (type) {
		case GUI_ELEMENT_TYPE_CONTAINER:
			element->data = createGuiDataContainer(&area);
			break;	

		case GUI_ELEMENT_TYPE_TEXT:
			element->data = createGuiDataText(param);
			break;

		case GUI_ELEMENT_TYPE_TEXTFIELD:
			element->data = createGuiDataTextfield((long) param);
			break;
		
		case GUI_ELEMENT_TYPE_BUTTON:
			element->data = createGuiDataButton(param);
			break;

		case GUI_ELEMENT_TYPE_PROXY:
			element->data = createGuiDataProxy(param);
			break;
	}
	return element;
}

void disposeGuiElement(GuiElement* element) {
	switch (element->type) {
		case GUI_ELEMENT_TYPE_CONTAINER:
			disposeGuiDataContainer(element->data);
			break;

		case GUI_ELEMENT_TYPE_TEXT:
			disposeGuiDataText(element->data);
			break;

		case GUI_ELEMENT_TYPE_TEXTFIELD:
			disposeGuiDataTextfield(element->data);
			break;

		case GUI_ELEMENT_TYPE_BUTTON:
			disposeGuiDataButton(element->data);
			break;

		case GUI_ELEMENT_TYPE_PROXY:
			disposeGuiDataProxy(element->data);
			break;
	}
	free(element);
}

void drawGuiElement(GuiElement* element, SDL_Surface* surface) {
	switch (element->type) {
		case GUI_ELEMENT_TYPE_TEXT:
			drawGuiElementText(element, surface);
			break;
	
		case GUI_ELEMENT_TYPE_TEXTFIELD:
			drawGuiElementTextfield(element, surface);
			break;

		case GUI_ELEMENT_TYPE_BUTTON:
			drawGuiElementButton(element, surface);
			break;

		case GUI_ELEMENT_TYPE_PROXY:
			drawGuiElementProxy(element, surface);
			break;
	}

}

//////// Container ////////

GuiDataContainerType* createGuiDataContainer(SDL_Rect* param) {
	GuiDataContainerType* new = malloc(sizeof(GuiDataContainerType));
	new->surface = SDL_CreateRGBSurface(0, param->w, param->h, 32, 0x00FF0000, 0x0000FF00, 0x000000FF, 0xFF000000);
	new->children = 0;
	new->invalidated = 1;
	return new;
}

void disposeGuiDataContainer(struct GuiDataContainerType* data) {
	SDL_FreeSurface(data->surface);
	free(data->surface);
	linkedListDispose(data->children);
	free(data);
}

void guiContainerLink(GuiElement* container, GuiElement* child) {
	GuiDataContainerType* data = container->data;
	data->children = linkedListPrepend(data->children, child);
}

void guiContainerUnlink(GuiElement* container, GuiElement* child) {
	GuiDataContainerType* data = container->data;
	LinkedList* previous = 0;
	LinkedList* current = data->children;
	
	while (current) {
		if (current->value == child) {
			if (previous) {
				linkedListRemove(previous);
				current = previous->next;
			} else {
				data->children = current->next;
				free(current);
				current = data->children;
			}
		} else {
			previous = current;
			current = previous->next;
		}
	}
}

SDL_Surface* guiContainerSurface(GuiElement* container) {
	GuiDataContainerType* data = container->data;
	if (data->invalidated) {
		LinkedList* target = data->children;
		while (target) {
			drawGuiElement(target->value, data->surface);
			target = target->next;
		}
		data->invalidated = 0;
	}
	return data->surface;
}

void guiContainerInvalidate(GuiElement* container) {
	((GuiDataContainerType*) container->data)->invalidated = 1;
}

//////// Text ////////
void* createGuiDataText(char* param) {
	char length = 0;
	while (param[length++] != '\0') {}
	char* data = malloc(length);
	memcpy(data, param, length); 
	return data;
}

void disposeGuiDataText(void* data) {
	free(data);
}

void drawGuiElementText(GuiElement* element, SDL_Surface* surface) {
	drawString(surface, element->data, &(element->position), 1, 0);
}


//////// Text Field ////////

void* createGuiDataTextfield(char param) {
	return createInputText(param, INPUT_FLAGS_ENABLED | INPUT_FLAGS_SELECTABLE);
}

void disposeGuiDataTextfield(void* data) {
	disposeOneInput(data);
}

void drawGuiElementTextfield(GuiElement* element, SDL_Surface* surface) {
	InputTextfieldData* field = ((InputField*)element->data)->data;
	drawString(surface, field->chars, &(element->position), 1, field->length);
}


//////// Button ////////

GuiDataButtonType* createGuiDataButton(InputButtonData* param) {
	GuiDataButtonType* data = malloc(sizeof(GuiDataButtonType));
	data->inputField = createInputButton(param->onPress, INPUT_FLAGS_ENABLED | INPUT_FLAGS_SELECTABLE);

	char length = 0;
	while (param->text[length++] != '\0') {}
	data->text = malloc(length);
	memcpy(data->text, param->text, length); 
	return data;
}

void disposeGuiDataButton(GuiDataButtonType* data) {
	disposeOneInput(data->inputField);
	free(data);
}

void drawGuiElementButton(GuiElement* element, SDL_Surface* surface) {
	InputButtonData* data = element->data;
	drawString(surface, data->text, &(element->position), 1, 0);
}

//////// Proxy ////////

GuiDataProxyType* createGuiDataProxy(GuiProxyData* param) {
	GuiDataProxyType* data = malloc(sizeof(GuiDataProxyType));
	data->proxy = param;
	data->data = param->onCreate(param);
	return data;
}

void disposeGuiDataProxy(GuiDataProxyType* data) {
	data->proxy->onDispose(data->data);
	free(data->proxy);
	free(data);
}

void drawGuiElementProxy(GuiElement* element, SDL_Surface* surface) {
	GuiDataProxyType* data = element->data;
	data->proxy->onDraw(element, surface);
}
